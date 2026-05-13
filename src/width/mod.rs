//! Phase 4: Bit-width inference for MIRR expressions.
//!
//! Analyzes expression trees to assign minimum safe bit-widths to every node,
//! detects unsafe truncations, and emits clear diagnostics.
//!
//! Pipeline position: runs after Phase 3 simplification, before Verilog emit.

#![forbid(unsafe_code)]

pub mod constraint;
pub mod display;
pub mod flatten;
pub mod graph;
pub mod scc;
pub mod scc_solver;
pub mod solver;
pub mod types;
pub mod verify;

use crate::ast::expr::Expr;
use crate::ast::program::{Assignment, SignalDecl};

use serde::Serialize;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use types::{DiagSeverity, WidthDiag, WidthExpr, WidthStats};

// ---------------------------------------------------------------------------
// Public API: single expression
// ---------------------------------------------------------------------------

/// Result of width inference on a single MIRR expression or signal.
#[derive(Debug, Clone, Serialize)]
pub struct WidthInferenceResult {

    /// Width-annotated expression tree (None if flattening failed).
    pub expr: Option<WidthExpr>,
    /// Diagnostics emitted during inference.
    pub diagnostics: Vec<WidthDiag>,
    /// Statistics from this inference run.
    pub stats: WidthStats,
}

impl WidthInferenceResult {
    /// Returns true if any diagnostic is an error.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == DiagSeverity::Error)
    }
}

/// Infer bit-widths for a single expression tree.
///
/// `signals` provides the declared widths for Signal nodes.
///
/// Pipeline: flatten -> generate constraints -> solve -> reconstruct.
/// All steps are iterative with bounded loops.
pub fn infer_widths(expr: &Expr, signals: &[SignalDecl]) -> WidthInferenceResult {
    let signal_widths = signal_width_map(signals);
    infer_widths_with_signal_widths(expr, signals, &signal_widths)
}

fn infer_widths_with_signal_widths<K>(
    expr: &Expr,
    signals: &[SignalDecl],
    signal_widths: &HashMap<K, u32>,
) -> WidthInferenceResult
where
    K: Eq + Hash + Borrow<str>,
{
    // Step 1: Flatten.
    let flat_nodes = match flatten::flatten_expr(expr, signals) {
        Some(nodes) => nodes,
        None => {
            return WidthInferenceResult {
                expr: None,
                diagnostics: vec![WidthDiag::error(
                    "[E500] expression tree exceeds maximum node count (512)".to_string(),
                )
                .with_code("E500")],
                stats: WidthStats {
                    nodes_analyzed: 0,
                    propagation_rounds: 0,
                    diagnostics_count: 1,
                    scc_count: 0,
                    expansive_count: 0,
                    nonexpansive_count: 0,
                },
            };
        }
    };

    let node_count = flat_nodes.len();

    // Step 2: Generate constraints.
    // Convert signal declarations to a name->width map for the constraint generator API.
    let cset = constraint::generate_constraints_with_index(&flat_nodes, signal_widths);
    let mut all_diags = cset.diagnostics;

    // Step 3: Solve. solver::validate_widths already emits hard errors for
    // any node whose inferred width exceeds 64 — no second pass needed.
    let solve_result = solver::solve(&flat_nodes, &cset.constraints);
    all_diags.extend(solve_result.diagnostics);

    // Step 4: Reconstruct.
    let width_expr = flatten::reconstruct_width_expr(&flat_nodes, &solve_result.widths);

    let stats = WidthStats {
        nodes_analyzed: node_count,
        propagation_rounds: solve_result.rounds,
        diagnostics_count: all_diags.len(),
        scc_count: 0,
        expansive_count: 0,
        nonexpansive_count: 0,
    };

    WidthInferenceResult { expr: width_expr, diagnostics: all_diags, stats }
}

// ---------------------------------------------------------------------------
// Public API: single-assignment truncation check (test + diagnostic use)
// ---------------------------------------------------------------------------

/// Check a single assignment for unsafe truncation.
///
/// Runs width inference on the RHS expression, then compares the inferred
/// width against the target signal's declared width.
///
/// The main pipeline uses [`infer_program_widths`] instead, which inlines this
/// logic to accumulate per-node statistics. This entry point exists for
/// integration tests and external tooling that need to check one assignment
/// in isolation without constructing a full `MirrProgram`.
pub fn check_assignment(assignment: &Assignment, signals: &[SignalDecl]) -> Vec<WidthDiag> {
    let result = infer_widths(&assignment.value, signals);
    let mut diags = result.diagnostics;

    // Look up target width and signedness.
    let target_info = signal_info_map(signals).get(assignment.target.as_str()).copied();

    if let (Some(we), Some((tw, ts))) = (&result.expr, target_info) {
        let expr_w = we.width();
        let trunc_diags = solver::check_truncation(&assignment.target, tw, expr_w, ts);
        diags.extend(trunc_diags);
    }

    diags
}

// ---------------------------------------------------------------------------
// Public API: full program
// ---------------------------------------------------------------------------

/// Result of width inference on an entire module.
#[derive(Debug, Clone, Serialize)]
pub struct ProgramWidthResult {
    /// Per-guard condition inference results (guard_name, result).
    pub guard_results: Vec<(String, WidthInferenceResult)>,
    /// Per-assignment inference results (reflex_name.target, result + truncation diags).
    pub assignment_results: Vec<(String, Vec<WidthDiag>)>,
    /// Aggregate statistics.
    pub stats: WidthStats,
}

impl ProgramWidthResult {
    /// Returns true if any diagnostic across the whole program is an error.
    pub fn has_errors(&self) -> bool {
        for (_, r) in &self.guard_results {
            if r.has_errors() {
                return true;
            }
        }
        for (_, diags) in &self.assignment_results {
            if diags.iter().any(|d| d.severity == DiagSeverity::Error) {
                return true;
            }
        }
        false
    }

    /// Collect all diagnostics across the entire program.
    pub fn all_diagnostics(&self) -> Vec<&WidthDiag> {
        let mut all: Vec<&WidthDiag> = Vec::new();
        for (_, r) in &self.guard_results {
            for d in &r.diagnostics {
                all.push(d);
            }
        }
        for (_, diags) in &self.assignment_results {
            for d in diags {
                all.push(d);
            }
        }
        all
    }
}

/// Run width inference over an entire MIRR module.
///
/// Infers widths for all guard conditions and all reflex assignment RHS,
/// checking for truncations at every assignment site.
///
/// Bounded: iterates over guards + reflexes (finite, from parsed program).
pub fn infer_program_widths(program: &crate::ast::MirrProgram) -> ProgramWidthResult {
    let signals = &program.module.signals;
    let signal_widths = signal_width_map(signals);
    let signal_info = signal_info_map(signals);
    let mut guard_results: Vec<(String, WidthInferenceResult)> = Vec::new();
    let mut assignment_results: Vec<(String, Vec<WidthDiag>)> = Vec::new();
    let mut total_stats = WidthStats {
        nodes_analyzed: 0,
        propagation_rounds: 0,
        diagnostics_count: 0,
        scc_count: 0,
        expansive_count: 0,
        nonexpansive_count: 0,
    };

    // Infer widths for guard conditions.
    for g in &program.module.guards {
        let result = infer_widths_with_signal_widths(&g.condition, signals, &signal_widths);
        total_stats.nodes_analyzed += result.stats.nodes_analyzed;
        total_stats.propagation_rounds += result.stats.propagation_rounds;
        total_stats.diagnostics_count += result.stats.diagnostics_count;
        guard_results.push((g.name.clone(), result));
    }

    // Infer widths for reflex assignment RHS and check truncation.
    // Inlined (not via check_assignment) so that nodes_analyzed and
    // propagation_rounds are accumulated alongside diagnostics_count.
    for r in &program.module.reflexes {
        for a in &r.assignments {
            let rhs_result = infer_widths_with_signal_widths(&a.value, signals, &signal_widths);
            total_stats.nodes_analyzed += rhs_result.stats.nodes_analyzed;
            total_stats.propagation_rounds += rhs_result.stats.propagation_rounds;

            let mut diags = rhs_result.diagnostics;

            // Perform the truncation check inline.
            let target_info = signal_info.get(a.target.as_str()).copied();
            if let (Some(we), Some((tw, ts))) = (&rhs_result.expr, target_info) {
                diags.extend(solver::check_truncation(&a.target, tw, we.width(), ts));
            }

            total_stats.diagnostics_count += diags.len();
            let label = format!("{}.{}", r.name, a.target);
            assignment_results.push((label, diags));
        }
    }

    ProgramWidthResult { guard_results, assignment_results, stats: total_stats }
}

// ---------------------------------------------------------------------------
// Public API: Phase 4b — SCC-based width inference
// ---------------------------------------------------------------------------

/// Result of SCC-based width analysis on an entire module.
#[derive(Debug, Clone, Serialize)]
pub struct SccWidthResult {
    /// Phase 4a results (per-expression inference).
    pub phase4a: ProgramWidthResult,
    /// Detected non-trivial SCCs with classification.
    pub sccs: Vec<types::SccInfo>,
    /// Per-SCC solve results (parallel to `sccs`).
    pub scc_solves: Vec<(types::SccInfo, scc_solver::SccSolveResult)>,
    /// Least-solution verification result.
    pub verification: verify::VerifyResult,
    /// Aggregate statistics including SCC info.
    pub stats: WidthStats,
    /// All diagnostics from SCC analysis.
    pub scc_diagnostics: Vec<WidthDiag>,
    /// Signal names that belong to any detected SCC.
    /// Phase 4a truncation errors for these targets are suppressed
    /// because Phase 4b owns their width determination.
    pub scc_member_names: std::collections::HashSet<String>,
}

impl SccWidthResult {
    /// Returns true if any non-suppressed diagnostic is an error.
    ///
    /// Phase 4a truncation errors for signals in SCCs are suppressed:
    /// Phase 4b owns the width of SCC member signals, so Phase 4a's
    /// acyclic truncation analysis does not apply to them.
    pub fn has_errors(&self) -> bool {
        // Guard errors always count (guards are never SCC members).
        for (_, r) in &self.phase4a.guard_results {
            if r.has_errors() {
                return true;
            }
        }
        // Assignment errors: suppress truncation errors for SCC member targets.
        for (label, diags) in &self.phase4a.assignment_results {
            let target = label.split('.').nth(1).unwrap_or(label.as_str());
            let is_scc_member = self.scc_member_names.contains(target);
            for d in diags {
                if d.severity == DiagSeverity::Error {
                    if is_scc_member && d.message.contains("truncates") {
                        continue;
                    }
                    return true;
                }
            }
        }
        // SCC-level diagnostics.
        if self.scc_diagnostics.iter().any(|d| d.severity == DiagSeverity::Error) {
            return true;
        }
        // Verification diagnostics.
        if self.verification.diagnostics.iter().any(|d| d.severity == DiagSeverity::Error) {
            return true;
        }
        false
    }
}

/// Run full Phase 4b SCC-based width inference on a MIRR module.
///
/// Pipeline: Phase 4a per-expression -> build graph -> find SCCs ->
/// solve SCCs -> verify minimality.
///
/// Bounded: all steps are individually bounded.
pub fn infer_program_widths_with_scc(
    program: &crate::ast::MirrProgram,
    _type_map: Option<&crate::typeck::TypeMap>,
) -> SccWidthResult {
    // Step 1: Run Phase 4a (per-expression inference).
    let phase4a = infer_program_widths(program);

    // Step 2: Build width dependency graph.
    let dep_graph = graph::build_graph(program);

    // Step 3: Find SCCs.
    let scc_result = scc::find_sccs(&dep_graph);
    let mut scc_diags = scc_result.diagnostics;

    // Step 4: Solve each SCC.
    let signals = &program.module.signals;
    let guards = &program.module.guards;
    let mut scc_solves: Vec<(types::SccInfo, scc_solver::SccSolveResult)> =
        Vec::with_capacity(scc_result.sccs.len());

    for scc_info in scc_result.sccs {
        let solve_result = scc_solver::solve_scc(&scc_info, signals, guards, program);
        scc_diags.extend(solve_result.diagnostics.iter().cloned());
        scc_solves.push((scc_info, solve_result));
    }

    // Step 5: Verify least solution.
    let verification = verify::verify_least_solution(&scc_solves, signals);
    scc_diags.extend(verification.diagnostics.iter().cloned());

    // Aggregate stats.
    let scc_count = scc_solves.len();
    let expansive_count =
        scc_solves.iter().filter(|(s, _)| s.kind == types::SccKind::Expansive).count();
    let nonexpansive_count = scc_count - expansive_count;

    let stats = WidthStats {
        nodes_analyzed: phase4a.stats.nodes_analyzed,
        propagation_rounds: phase4a.stats.propagation_rounds,
        diagnostics_count: phase4a.stats.diagnostics_count + scc_diags.len(),
        scc_count,
        expansive_count,
        nonexpansive_count,
    };

    let sccs: Vec<types::SccInfo> = scc_solves.iter().map(|(s, _)| s.clone()).collect();

    // Collect SCC member signal names for truncation suppression.
    let mut scc_member_names = std::collections::HashSet::new();
    for (scc_info, _) in &scc_solves {
        for &idx in &scc_info.signal_indices {
            if let Some(s) = signals.get(idx) {
                scc_member_names.insert(s.name.clone());
            }
        }
    }

    SccWidthResult {
        phase4a,
        sccs,
        scc_solves,
        verification,
        stats,
        scc_diagnostics: scc_diags,
        scc_member_names,
    }
}

fn signal_width_map(signals: &[SignalDecl]) -> HashMap<String, u32> {
    signals.iter().map(|signal| (signal.name.clone(), signal.ty.signal_type().width())).collect()
}

fn signal_info_map(signals: &[SignalDecl]) -> HashMap<&str, (u32, bool)> {
    signals
        .iter()
        .map(|signal| {
            let ty = signal.ty.signal_type();
            (signal.name.as_str(), ty.width_and_signed())
        })
        .collect()
}
