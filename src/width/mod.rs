//! Phase 4: Bit-width inference for MIRR expressions.
//!
//! Analyzes expression trees to assign minimum safe bit-widths to every node,
//! detects unsafe truncations, and emits clear diagnostics.
//!
//! Pipeline position: runs after Phase 3 simplification, before Verilog emit.

#![forbid(unsafe_code)]

pub mod types;
pub mod flatten;
pub mod constraint;
pub mod solver;
pub mod display;

use crate::ast::expr::Expr;
use crate::ast::program::{Assignment, SignalDecl};
use crate::ast::types::SignalType;
use types::{DiagSeverity, WidthDiag, WidthExpr, WidthStats};

// ---------------------------------------------------------------------------
// Public API: single expression
// ---------------------------------------------------------------------------

/// Result of width inference on a single expression.
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
    // Step 1: Flatten.
    let flat_nodes = match flatten::flatten_expr(expr) {
        Some(nodes) => nodes,
        None => {
            return WidthInferenceResult {
                expr: None,
                diagnostics: vec![WidthDiag::error(
                    "expression tree exceeds maximum node count (512)".to_string(),
                )],
                stats: WidthStats {
                    nodes_analyzed: 0,
                    propagation_rounds: 0,
                    diagnostics_count: 1,
                },
            };
        }
    };

    let node_count = flat_nodes.len();

    // Step 2: Generate constraints.
    let cset = constraint::generate_constraints(&flat_nodes, signals);
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
    };

    WidthInferenceResult {
        expr: width_expr,
        diagnostics: all_diags,
        stats,
    }
}

// ---------------------------------------------------------------------------
// Public API: assignment truncation check
// ---------------------------------------------------------------------------

/// Check a single assignment for unsafe truncation.
///
/// Runs width inference on the RHS expression, then compares the inferred
/// width against the target signal's declared width.
pub fn check_assignment(
    assignment: &Assignment,
    signals: &[SignalDecl],
) -> Vec<WidthDiag> {
    let result = infer_widths(&assignment.value, signals);
    let mut diags = result.diagnostics;

    // Look up target width.
    let target_width = signals.iter()
        .find(|s| s.name == assignment.target)
        .map(|s| match s.ty {
            SignalType::Bool => 1u32,
            SignalType::Unsigned(w) => w,
        });

    if let (Some(we), Some(tw)) = (&result.expr, target_width) {
        let expr_w = we.width();
        let trunc_diags = solver::check_truncation(&assignment.target, tw, expr_w);
        diags.extend(trunc_diags);
    }

    diags
}

// ---------------------------------------------------------------------------
// Public API: full program
// ---------------------------------------------------------------------------

/// Result of width inference on an entire module.
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
pub fn infer_program_widths(
    program: &crate::ast::MirrProgram,
) -> ProgramWidthResult {
    let signals = &program.module.signals;
    let mut guard_results: Vec<(String, WidthInferenceResult)> = Vec::new();
    let mut assignment_results: Vec<(String, Vec<WidthDiag>)> = Vec::new();
    let mut total_stats = WidthStats {
        nodes_analyzed: 0,
        propagation_rounds: 0,
        diagnostics_count: 0,
    };

    // Infer widths for guard conditions.
    for g in &program.module.guards {
        let result = infer_widths(&g.condition, signals);
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
            let rhs_result = infer_widths(&a.value, signals);
            total_stats.nodes_analyzed += rhs_result.stats.nodes_analyzed;
            total_stats.propagation_rounds += rhs_result.stats.propagation_rounds;

            let mut diags = rhs_result.diagnostics;

            // Perform the truncation check inline.
            let target_width = signals.iter()
                .find(|s| s.name == a.target)
                .map(|s| match s.ty {
                    SignalType::Bool => 1u32,
                    SignalType::Unsigned(w) => w,
                });
            if let (Some(we), Some(tw)) = (&rhs_result.expr, target_width) {
                diags.extend(solver::check_truncation(&a.target, tw, we.width()));
            }

            total_stats.diagnostics_count += diags.len();
            let label = format!("{}.{}", r.name, a.target);
            assignment_results.push((label, diags));
        }
    }

    ProgramWidthResult {
        guard_results,
        assignment_results,
        stats: total_stats,
    }
}

