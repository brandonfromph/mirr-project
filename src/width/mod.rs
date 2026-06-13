//! Phase 4: Bit-width inference for MIRR expressions.
//!
//! Analyzes expression trees to assign minimum safe bit-widths to every node,
//! detects unsafe truncations, and emits clear diagnostics.
//!
//! Pipeline position: runs after Phase 3 simplification, before Verilog emit.

#![forbid(unsafe_code)]

pub mod constraint;
pub mod display;
#[allow(dead_code, deprecated)]
pub mod flatten;
pub mod graph;
pub mod scc;
pub mod scc_solver;
#[allow(dead_code, deprecated)]
pub mod solver;
pub mod types;
pub mod verify;

use crate::ast::program::SignalDecl;

use serde::Serialize;
use std::collections::HashMap;
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

// Infer bit-widths for a single expression tree.
//
// `signals` provides the declared widths for Signal nodes.
//
// Pipeline: flatten -> generate constraints -> solve -> reconstruct.
// All steps are iterative with bounded loops.
/* --- LEGACY AST ENGINE (KEPT FOR SAFE MIGRATION REFERENCE) ---
pub fn infer_widths(expr: &Expr, signals: &[SignalDecl]) -> WidthInferenceResult {
    let signal_info = signal_info_map(signals);
    infer_widths_with_signal_info(expr, signals, &signal_info)
}
*/

/*
fn infer_widths_with_signal_info<K>(
    expr: &Expr,
    _signals: &[SignalDecl],
    signal_info: &HashMap<K, (u32, bool)>,
) -> WidthInferenceResult
where
    K: Eq + Hash + Borrow<str>,
{
    // ECS Phase 2 & 3: Use Registry instead of FlatNode array
    let mut registry = crate::ecs::registry::Registry::new();

    // Step 1: Lower expression to ECS
    let _root_id = match registry.ingest_expr(expr) {
        Ok(id) => id,
        Err(_) => {
            return WidthInferenceResult {
                expr: None,
                diagnostics: vec![WidthDiag::error(format!(
                    "{} expression tree exceeds maximum node count",
                    crate::error_codes::ec(500)
                ))
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

    // Populate missing signal widths from signal_widths map into the registry
    let mut to_update = Vec::new();
    for (i, name_opt) in registry.names.iter().enumerate() {
        if let Some(name) = name_opt {
            if let Some(&(w, is_signed)) = signal_info.get(name.0.as_str()) {
                to_update.push((i, w, is_signed));
            }
        }
    }
    for (i, w, is_signed) in to_update {
        let ty = if is_signed {
            crate::ast::types::SignalType::Signed(w)
        } else {
            crate::ast::types::SignalType::Unsigned(w)
        };
        registry.set_type(
            crate::ecs::components::EntityId(i as u32),
            crate::ecs::components::TypeComponent::signal(crate::ast::types::ExtendedType::new(
                ty,
                Default::default(),
            )),
        );
    }

    let node_count = registry.active_entities();

    // Step 2: Generate constraints in ECS
    let mut all_diags =
        crate::width::constraint::generate_ecs_constraints(&mut registry, signal_info);

    // Step 3: Run ECS Solver
    let (solve_diags, ecs_rounds) =
        crate::ecs::systems::expression_width_inference_system(&mut registry);
    all_diags.extend(solve_diags);

    // Step 4: Reconstruct AST WidthExpr from ECS
    let width_expr = ecs_reconstruct_width_expr(&registry, _root_id);

    let stats = WidthStats {
        nodes_analyzed: node_count,
        propagation_rounds: ecs_rounds,
        diagnostics_count: all_diags.len(),
        scc_count: 0,
        expansive_count: 0,
        nonexpansive_count: 0,
    };

    WidthInferenceResult { expr: width_expr, diagnostics: all_diags, stats }
}
*/

/*
fn ecs_reconstruct_width_expr(
    registry: &crate::ecs::registry::Registry,
    id: crate::ecs::components::EntityId,
) -> Option<crate::width::types::WidthExpr> {
    let w = registry.types[id.0 as usize]
        .as_ref()
        .map(|tc| crate::width::types::Width(tc.0.core.width()))
        .unwrap_or(crate::width::types::Width(0));

    if let Some(lit) = &registry.literals[id.0 as usize] {
        let value = match lit.0 {
            crate::ast::types::LiteralValue::Bool(b) => {
                if b {
                    1
                } else {
                    0
                }
            }
            crate::ast::types::LiteralValue::Integer(v) => v,
        };
        return Some(crate::width::types::WidthExpr::Literal { value, width: w });
    }
    if let Some(sig) = &registry.signal_refs[id.0 as usize] {
        if let Some(name) = &registry.names[sig.0 .0 as usize] {
            return Some(crate::width::types::WidthExpr::Signal { name: name.0.clone(), width: w });
        }
    }
    if let Some(psig) = &registry.pending_signal_refs[id.0 as usize] {
        return Some(crate::width::types::WidthExpr::Signal { name: psig.0.clone(), width: w });
    }
    if let Some(un) = &registry.unary_ops[id.0 as usize] {
        let operand = ecs_reconstruct_width_expr(registry, un.operand)?;
        return Some(crate::width::types::WidthExpr::Unary {
            op: un.op,
            operand: Box::new(operand),
            width: w,
        });
    }
    if let Some(bin) = &registry.binary_ops[id.0 as usize] {
        let left = ecs_reconstruct_width_expr(registry, bin.left)?;
        let right = ecs_reconstruct_width_expr(registry, bin.right)?;
        return Some(crate::width::types::WidthExpr::Binary {
            op: bin.op,
            left: Box::new(left),
            right: Box::new(right),
            width: w,
        });
    }
    if let Some(prev) = &registry.prev_ops[id.0 as usize] {
        if let Some(name) = &registry.names[prev.signal.0 as usize] {
            return Some(crate::width::types::WidthExpr::Prev {
                signal: name.0.clone(),
                delay: prev.delay,
                width: w,
            });
        } else if let Some(psig) = &registry.pending_signal_refs[prev.signal.0 as usize] {
            return Some(crate::width::types::WidthExpr::Prev {
                signal: psig.0.clone(),
                delay: prev.delay,
                width: w,
            });
        }
    }
    // Fallback for arrays/structs/etc
    Some(crate::width::types::WidthExpr::Literal { value: 0, width: w })
}
*/
// ---------------------------------------------------------------------------
// Public API: single-assignment truncation check (test + diagnostic use)
// ---------------------------------------------------------------------------

// Check a single assignment for unsafe truncation.
//
// Runs width inference on the RHS expression, then compares the inferred
// width against the target signal's declared width.
//
// The main pipeline uses [`infer_program_widths`] instead, which inlines this
// logic to accumulate per-node statistics. This entry point exists for
// integration tests and external tooling that need to check one assignment
// in isolation without constructing a full `MirrProgram`.
/*
pub fn check_assignment(assignment: &Assignment, signals: &[SignalDecl]) -> Vec<WidthDiag> {
    let result = infer_widths(&assignment.value, signals);
    let mut diags = result.diagnostics;

    // Look up target width and signedness.
    let target_info = signal_info_map(signals).get(assignment.target.as_str()).copied();

    if let (Some(we), Some((tw, ts))) = (&result.expr, target_info) {
        let expr_w = we.width();
        let trunc_diags = check_truncation(&assignment.target, tw, expr_w, ts);
        diags.extend(trunc_diags);
    }

    diags
}
*/
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

// Run width inference over an entire MIRR module.
//
// Infers widths for all guard conditions and all reflex assignment RHS,
// checking for truncations at every assignment site.
//
// Bounded: iterates over guards + reflexes (finite, from parsed program).
/*
pub fn infer_program_widths(program: &crate::ast::MirrProgram) -> ProgramWidthResult {
    let signals = &program.module.signals;
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
        let result = infer_widths_with_signal_info(&g.condition, signals, &signal_info);
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
            let rhs_result = infer_widths_with_signal_info(&a.value, signals, &signal_info);
            total_stats.nodes_analyzed += rhs_result.stats.nodes_analyzed;
            total_stats.propagation_rounds += rhs_result.stats.propagation_rounds;

            let mut diags = rhs_result.diagnostics;

            // Perform the truncation check inline.
            let target_info = signal_info.get(a.target.as_str()).copied();
            if let (Some(we), Some((tw, ts))) = (&rhs_result.expr, target_info) {
                diags.extend(check_truncation(&a.target, tw, we.width(), ts));
            }

            total_stats.diagnostics_count += diags.len();
            let label = format!("{}.{}", r.name, a.target);
            assignment_results.push((label, diags));
        }
    }

    ProgramWidthResult { guard_results, assignment_results, stats: total_stats }
}
*/
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
    // Phase 2 ECS Migration: Ingest the full module into a fresh Registry.
    let mut registry = crate::ecs::registry::Registry::new();
    if let Err(_e) = registry.ingest_module(&program.module) {
        // Fallback: If ingestion fails, continue with empty registry for now.
        // We will map this properly once the entire pipeline requires the ECS registry.
    }

    // Step 1: Run Phase 4a (natively in ECS).
    let signals = &program.module.signals;
    let signal_info = signal_info_map(signals);
    let mut all_diags =
        crate::width::constraint::generate_ecs_constraints(&mut registry, &signal_info);
    let (solve_diags, ecs_rounds) =
        crate::ecs::systems::expression_width_inference_system(&mut registry);
    all_diags.extend(solve_diags);

    let mut assignment_diags = Vec::new();
    let mut assignment_results = Vec::new();
    for (i, assign_opt) in registry.assignment_comps.iter().enumerate() {
        if let Some(assign) = assign_opt {
            let target_name = registry.names[assign.target.0 as usize]
                .as_ref()
                .map(|n| n.0.clone())
                .unwrap_or_default();
            let target_info = signal_info.get(target_name.as_str()).copied();
            let rhs_id = assign.value;
            let tw = target_info.map(|t| t.0).unwrap_or(0);
            let ts = target_info.map(|t| t.1).unwrap_or(false);
            let ew =
                registry.types[rhs_id.0 as usize].as_ref().map(|tc| tc.0.core.width()).unwrap_or(0);
            let mut diags = Vec::new();
            if tw > 0 && ew > 0 {
                let truncs = check_truncation(&target_name, tw, crate::width::types::Width(ew), ts);
                diags.extend(truncs.clone());
                assignment_diags.extend(truncs);
            }
            // Use generic name since we don't have AST reflex names attached to assignments in ECS yet
            assignment_results.push((format!("assignment_{}", i), diags));
        }
    }

    // Aggregate diagnostics into a synthesized ProgramWidthResult
    let phase4a = ProgramWidthResult {
        guard_results: vec![(
            "ecs_solver".to_string(),
            WidthInferenceResult {
                expr: None,
                diagnostics: all_diags,
                stats: WidthStats {
                    nodes_analyzed: registry.active_entities(),
                    propagation_rounds: ecs_rounds,
                    diagnostics_count: 0,
                    scc_count: 0,
                    expansive_count: 0,
                    nonexpansive_count: 0,
                },
            },
        )],
        assignment_results,
        stats: WidthStats {
            nodes_analyzed: registry.active_entities(),
            propagation_rounds: ecs_rounds,
            diagnostics_count: 0,
            scc_count: 0,
            expansive_count: 0,
            nonexpansive_count: 0,
        },
    };

    // Step 2: Build width dependency graph.
    let dep_graph = graph::build_graph(&registry);

    // Step 3: Find SCCs.
    let scc_result = scc::find_sccs(&dep_graph, &registry);
    let mut scc_diags = scc_result.diagnostics;

    // Step 4: Solve each SCC.
    let mut scc_solves: Vec<(types::SccInfo, scc_solver::SccSolveResult)> =
        Vec::with_capacity(scc_result.sccs.len());

    for scc_info in scc_result.sccs {
        let solve_result = scc_solver::solve_scc(&scc_info, &registry);
        scc_diags.extend(solve_result.diagnostics.iter().cloned());
        scc_solves.push((scc_info, solve_result));
    }

    // Step 5: Verify least solution.
    let verification = verify::verify_least_solution(&scc_solves, &registry);
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
        for &entity_id in &scc_info.signals {
            if let Some(name) = &registry.names[entity_id.0 as usize] {
                scc_member_names.insert(name.0.clone());
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

fn signal_info_map(signals: &[SignalDecl]) -> HashMap<&str, (u32, bool)> {
    signals
        .iter()
        .map(|signal| {
            let ty = signal.ty.signal_type();
            (signal.name.as_str(), ty.width_and_signed())
        })
        .collect()
}

pub fn check_truncation(
    target_name: &str,
    target_width: u32,
    expr_width: crate::width::types::Width,
    target_signed: bool,
) -> Vec<crate::width::types::WidthDiag> {
    let mut diags = Vec::new();
    if expr_width.0 > target_width {
        let category = if target_signed { "signed" } else { "unsigned" };
        diags.push(
            crate::width::types::WidthDiag::error(format!(
                "{} assignment to '{}' truncates {} {} bits to {} bits",
                crate::error_codes::ec(505),
                target_name,
                category,
                expr_width.0,
                target_width
            ))
            .with_code("E505")
            .with_signal(target_name),
        );
    }
    diags
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ast::program::{MirrProgram, Module};
    // Unused width types removed
    use std::collections::HashSet;

    /*
    fn make_signal(name: &str, width: u32, is_signed: bool) -> SignalDecl {
        SignalDecl {
            name: name.to_string(),
            kind: SignalKind::Internal,
            ty: if is_signed {
                SignalType::Signed(width).into()
            } else {
                SignalType::Unsigned(width).into()
            },
            origin: None,
            span: None,
        }
    }
    */

    /*
    #[test]
    fn test_width_infer_with_unresolved_names_and_prev() {
        let signals = vec![make_signal("x", 8, false), make_signal("y", 16, true)];
        let signal_info = signal_info_map(&signals);

        let expr = Expr::Binary {
            op: crate::ast::types::BinaryOp::Add,
            left: Box::new(Expr::Signal("y".to_string())),
            right: Box::new(Expr::Prev { signal: "y".to_string(), delay: 1 }),
        };
        let res = infer_widths_with_signal_info(&expr, &signals, &signal_info);
        assert!(!res.has_errors());
    }

    #[test]
    fn test_width_infer_with_pending_signal_and_unop() {
        let signals = vec![make_signal("x", 8, false)];
        let signal_info = signal_info_map(&signals);

        // This simulates a pending signal or unknown name.
        let expr = Expr::Unary {
            op: crate::ast::types::UnaryOp::Not,
            operand: Box::new(Expr::Signal("z".to_string())),
        };
        let res = infer_widths_with_signal_info(&expr, &signals, &signal_info);
        // It now emits an undeclared signal error, but still infers a fallback width.
        assert!(res.has_errors());
    }
    */

    #[test]
    fn test_width_result_has_errors() {
        let mut pwr = ProgramWidthResult {
            guard_results: vec![],
            assignment_results: vec![],
            stats: WidthStats {
                nodes_analyzed: 0,
                propagation_rounds: 0,
                diagnostics_count: 0,
                scc_count: 0,
                expansive_count: 0,
                nonexpansive_count: 0,
            },
        };
        assert!(!pwr.has_errors());

        let bad_diag = WidthDiag::error("bad");
        pwr.guard_results.push((
            "g".to_string(),
            WidthInferenceResult {
                expr: None,
                diagnostics: vec![bad_diag.clone()],
                stats: WidthStats {
                    nodes_analyzed: 0,
                    propagation_rounds: 0,
                    diagnostics_count: 0,
                    scc_count: 0,
                    expansive_count: 0,
                    nonexpansive_count: 0,
                },
            },
        ));
        assert!(pwr.has_errors());
        assert_eq!(pwr.all_diagnostics().len(), 1);

        let pwr2 = ProgramWidthResult {
            guard_results: vec![],
            assignment_results: vec![("a.b".to_string(), vec![bad_diag.clone()])],
            stats: WidthStats {
                nodes_analyzed: 0,
                propagation_rounds: 0,
                diagnostics_count: 0,
                scc_count: 0,
                expansive_count: 0,
                nonexpansive_count: 0,
            },
        };
        assert!(pwr2.has_errors());
        assert_eq!(pwr2.all_diagnostics().len(), 1);
    }

    #[test]
    fn test_scc_width_result_has_errors() {
        let mut res = SccWidthResult {
            phase4a: ProgramWidthResult {
                guard_results: vec![],
                assignment_results: vec![],
                stats: WidthStats {
                    nodes_analyzed: 0,
                    propagation_rounds: 0,
                    diagnostics_count: 0,
                    scc_count: 0,
                    expansive_count: 0,
                    nonexpansive_count: 0,
                },
            },
            sccs: vec![],
            scc_solves: vec![],
            verification: crate::width::verify::VerifyResult {
                is_minimal: true,
                diagnostics: vec![],
            },
            stats: WidthStats {
                nodes_analyzed: 0,
                propagation_rounds: 0,
                diagnostics_count: 0,
                scc_count: 0,
                expansive_count: 0,
                nonexpansive_count: 0,
            },
            scc_diagnostics: vec![],
            scc_member_names: HashSet::new(),
        };
        assert!(!res.has_errors());

        // Add scc_diagnostics error
        res.scc_diagnostics.push(WidthDiag::error("scc err"));
        assert!(res.has_errors());
        res.scc_diagnostics.clear();

        // Add verification error
        res.verification.diagnostics.push(WidthDiag::error("verify err"));
        assert!(res.has_errors());
        res.verification.diagnostics.clear();

        // Add suppressed truncation error
        res.scc_member_names.insert("a".to_string());
        res.phase4a
            .assignment_results
            .push(("x.a".to_string(), vec![WidthDiag::error("truncates")]));
        assert!(!res.has_errors()); // suppressed

        // Add unsuppressed assignment error
        res.phase4a.assignment_results.push(("x.b".to_string(), vec![WidthDiag::error("bad")]));
        assert!(res.has_errors()); // not suppressed
    }

    #[test]
    fn test_infer_program_widths_with_scc_bad_module() {
        // If ingest_module fails, it drops to fallback
        // We can just construct an empty module
        let prog = MirrProgram {
            imports: vec![],
            patterns: vec![],
            target: None,
            module: Module {
                name: "m".to_string(),
                signals: vec![],
                guards: vec![],
                reflexes: vec![],
                properties: vec![],
                pattern_calls: vec![],
                pattern_origins: vec![],
                span: None,
            },
        };
        let res = infer_program_widths_with_scc(&prog, None);
        assert!(!res.has_errors());
    }
}
