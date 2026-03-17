// ---------------------------------------------------------------------------
//! MEGA-4: Totality Engine — Static totality analysis for MIRR modules.
//!
//! Verifies that a MIRR module is a total function: bounded resource usage,
//! every output driven, all guards coverable, bounded temporal latency, and
//! no combinational cycles. Each analysis is a named function with a clear
//! input/output contract, suitable for 1:1 Rocq formalization in MEGA-7.
//!
//! ## Analyses
//!
//! 1. `check_resource_bounds` — resource usage within hardware limits
//! 2. `check_output_completeness` — every output has a driving reflex
//! 3. `check_guard_coverage` — at least one guard can fire per output
//! 4. `check_temporal_bound` — worst-case latency is finite
//! 5. `check_dependency_acyclicity` — no combinational feedback loops
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

mod checks;
mod types;

pub use checks::{
    check_guard_coverage, check_output_completeness, check_resource_bounds, check_temporal_bound,
};
pub use types::*;

use crate::ast::program::Module;

/// Maximum signals in a module (NASA P10 iteration bound).
pub(super) const MAX_SIGNALS: usize = 4096;

/// Maximum reflexes in a module (NASA P10 iteration bound).
pub(super) const MAX_REFLEXES: usize = 4096;

/// Maximum expression nodes to traverse during dependency analysis.
pub(super) const MAX_DEP_NODES: usize = 8192;

/// Maximum stack depth for dependency graph traversal (bounded DFS).
const MAX_DFS_STACK: usize = 4096;

// ---------------------------------------------------------------------------
// Top-level entry point
// ---------------------------------------------------------------------------

/// Run all five totality analyses on a parsed MIRR module.
///
/// Bounded: each sub-analysis has its own MAX_* iteration bounds.
pub fn run_totality_check(module: &Module) -> TotalityResult {
    let resource_bound = check_resource_bounds(module);
    let output_completeness = check_output_completeness(module);
    let guard_coverage = check_guard_coverage(module);
    let temporal_bound = check_temporal_bound(module);
    let acyclicity = check_dependency_acyclicity(module);

    let property_summary = build_property_summary(module);

    let is_total = resource_bound.pass
        && output_completeness.pass
        && guard_coverage.pass
        && temporal_bound.pass
        && acyclicity.pass;

    TotalityResult {
        resource_bound,
        output_completeness,
        guard_coverage,
        temporal_bound,
        acyclicity,
        property_summary,
        is_total,
    }
}

// ---------------------------------------------------------------------------
// Analysis 5: Dependency acyclicity
// ---------------------------------------------------------------------------

/// Verify that the signal dependency graph has no combinational cycles.
/// A combinational cycle occurs when a reflex assignment depends on its own
/// target signal without going through a `prev` (temporal barrier).
///
/// Uses bounded DFS on the signal dependency graph. Prev edges are NOT
/// followed (they break cycles via temporal delay).
///
/// Bounded: MAX_DFS_STACK for the work stack. MAX_SIGNALS for signal count.
pub fn check_dependency_acyclicity(module: &Module) -> AcyclicityResult {
    let mut names: Vec<&str> = Vec::new();
    let mut ni = 0;
    while ni < module.signals.len() && ni < MAX_SIGNALS {
        names.push(&module.signals[ni].name);
        ni += 1;
    }
    let n = names.len();
    if n == 0 {
        return AcyclicityResult { pass: true, cycle_witness: None };
    }

    let mut adj: Vec<Vec<usize>> = Vec::new();
    let mut ai = 0;
    while ai < n {
        adj.push(Vec::new());
        ai += 1;
    }

    let mut ri = 0;
    while ri < module.reflexes.len() && ri < MAX_REFLEXES {
        let mut asgn_i = 0;
        while asgn_i < module.reflexes[ri].assignments.len() && asgn_i < MAX_REFLEXES {
            let target = &module.reflexes[ri].assignments[asgn_i].target;
            let target_idx = find_signal_index(&names, target);
            if let Some(ti) = target_idx {
                let deps = collect_signal_deps(&module.reflexes[ri].assignments[asgn_i].value);
                let mut di = 0;
                while di < deps.len() && di < MAX_DEP_NODES {
                    if let Some(dep_idx) = find_signal_index(&names, &deps[di]) {
                        adj[ti].push(dep_idx);
                    }
                    di += 1;
                }
            }
            asgn_i += 1;
        }
        ri += 1;
    }

    let mut color: Vec<u8> = Vec::new();
    let mut ci = 0;
    while ci < n {
        color.push(0);
        ci += 1;
    }

    let mut si = 0;
    while si < n {
        if color[si] == 0 {
            let mut stack: Vec<(usize, usize)> = Vec::new();
            stack.push((si, 0));
            color[si] = 1;

            let mut iterations = 0usize;
            while let Some((node, edge_idx)) = stack.last_mut() {
                iterations += 1;
                if iterations > MAX_DFS_STACK {
                    break;
                }

                if *edge_idx < adj[*node].len() {
                    let neighbor = adj[*node][*edge_idx];
                    *edge_idx += 1;

                    if color[neighbor] == 1 {
                        return AcyclicityResult {
                            pass: false,
                            cycle_witness: Some(names[neighbor].to_string()),
                        };
                    } else if color[neighbor] == 0 {
                        color[neighbor] = 1;
                        stack.push((neighbor, 0));
                    }
                } else {
                    color[*node] = 2;
                    stack.pop();
                }
            }
        }
        si += 1;
    }

    AcyclicityResult { pass: true, cycle_witness: None }
}

// ---------------------------------------------------------------------------
// Property summary
// ---------------------------------------------------------------------------

/// Build a summary of all declared properties for the certificate.
///
/// Bounded: iterates over properties (≤ MAX_SIGNALS).
fn build_property_summary(module: &Module) -> Vec<PropertySummary> {
    let mut summaries: Vec<PropertySummary> = Vec::new();
    let mut pi = 0;
    while pi < module.properties.len() && pi < MAX_SIGNALS {
        let p = &module.properties[pi];
        let kind = match &p.formula {
            crate::ast::PropertyFormula::Always(_) => "always",
            crate::ast::PropertyFormula::Never(_) => "never",
            crate::ast::PropertyFormula::AlwaysImplies { .. } => "always_implies",
            crate::ast::PropertyFormula::NeverImplies { .. } => "never_implies",
            crate::ast::PropertyFormula::EventuallyWithin { .. } => "eventually_within",
            crate::ast::PropertyFormula::AlwaysFollowedBy { .. } => "always_followed_by",
        };
        summaries.push(PropertySummary { name: p.name.clone(), kind: kind.to_string() });
        pi += 1;
    }
    summaries
}

/// Find the index of a signal name in the names array (bounded linear scan).
fn find_signal_index(names: &[&str], target: &str) -> Option<usize> {
    let mut i = 0;
    while i < names.len() && i < MAX_SIGNALS {
        if names[i] == target {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Collect signal names referenced in an expression (excluding Prev references).
///
/// Bounded: explicit stack with MAX_DEP_NODES limit.
fn collect_signal_deps(expr: &crate::ast::Expr) -> Vec<String> {
    let mut deps: Vec<String> = Vec::new();
    let mut stack: Vec<&crate::ast::Expr> = Vec::new();
    stack.push(expr);

    let mut visited = 0usize;
    while let Some(e) = stack.pop() {
        visited += 1;
        if visited > MAX_DEP_NODES {
            break;
        }
        match e {
            crate::ast::Expr::Signal(name) => {
                deps.push(name.clone());
            }
            crate::ast::Expr::Unary { operand, .. } => {
                stack.push(operand);
            }
            crate::ast::Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            crate::ast::Expr::Prev { .. } | crate::ast::Expr::Literal(_) => {}
        }
    }
    deps
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
    use crate::ast::types::LiteralValue;
    use crate::ast::types::{ExtendedType, SignalKind, SignalType};
    use crate::ast::Expr;

    fn make_signal(name: &str, kind: SignalKind) -> SignalDecl {
        SignalDecl {
            name: name.to_string(),
            kind,
            ty: ExtendedType::from_core(SignalType::Bool),
            origin: None,
            span: None,
        }
    }

    fn make_guard(name: &str, cycles: u64) -> Guard {
        Guard {
            name: name.to_string(),
            condition: Expr::Literal(LiteralValue::Bool(true)),
            cycles,
            origin: None,
            span: None,
        }
    }

    fn make_reflex(name: &str, guard: &str, target: &str) -> Reflex {
        Reflex {
            name: name.to_string(),
            guard_names: vec![guard.to_string()],
            assignments: vec![Assignment {
                target: target.to_string(),
                value: Expr::Signal("input_a".to_string()),
                span: None,
            }],
            origin: None,
            span: None,
        }
    }

    fn make_module(signals: Vec<SignalDecl>, guards: Vec<Guard>, reflexes: Vec<Reflex>) -> Module {
        Module {
            name: "test".to_string(),
            signals,
            guards,
            reflexes,
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        }
    }

    #[test]
    fn test_total_module_passes_all() {
        let m = make_module(
            vec![
                make_signal("input_a", SignalKind::Input),
                make_signal("output_b", SignalKind::Output),
            ],
            vec![make_guard("g1", 3)],
            vec![make_reflex("r1", "g1", "output_b")],
        );
        let result = run_totality_check(&m);
        assert!(result.is_total);
        assert!(result.resource_bound.pass);
        assert!(result.output_completeness.pass);
        assert!(result.guard_coverage.pass);
        assert!(result.temporal_bound.pass);
        assert!(result.acyclicity.pass);
    }

    #[test]
    fn test_undriven_output_fails_completeness() {
        let m = make_module(
            vec![
                make_signal("input_a", SignalKind::Input),
                make_signal("output_b", SignalKind::Output),
                make_signal("output_c", SignalKind::Output),
            ],
            vec![make_guard("g1", 3)],
            vec![make_reflex("r1", "g1", "output_b")],
        );
        let result = run_totality_check(&m);
        assert!(!result.is_total);
        assert!(!result.output_completeness.pass);
        assert_eq!(result.output_completeness.undriven_outputs, vec!["output_c"]);
    }

    #[test]
    fn test_temporal_bound() {
        let m = make_module(
            vec![
                make_signal("input_a", SignalKind::Input),
                make_signal("output_b", SignalKind::Output),
            ],
            vec![make_guard("g1", 10), make_guard("g2", 25)],
            vec![make_reflex("r1", "g1", "output_b")],
        );
        let result = check_temporal_bound(&m);
        assert_eq!(result.max_guard_cycles, 25);
        assert!(result.pass);
    }

    #[test]
    fn test_combinational_cycle_detected() {
        let m = make_module(
            vec![make_signal("output_b", SignalKind::Output)],
            vec![make_guard("g1", 1)],
            vec![Reflex {
                name: "r_cycle".to_string(),
                guard_names: vec!["g1".to_string()],
                assignments: vec![Assignment {
                    target: "output_b".to_string(),
                    value: Expr::Signal("output_b".to_string()),
                    span: None,
                }],
                origin: None,
                span: None,
            }],
        );
        let result = check_dependency_acyclicity(&m);
        assert!(!result.pass);
        assert_eq!(result.cycle_witness.as_deref(), Some("output_b"));
    }

    #[test]
    fn test_prev_breaks_cycle() {
        let m = make_module(
            vec![make_signal("output_b", SignalKind::Output)],
            vec![make_guard("g1", 1)],
            vec![Reflex {
                name: "r_prev".to_string(),
                guard_names: vec!["g1".to_string()],
                assignments: vec![Assignment {
                    target: "output_b".to_string(),
                    value: Expr::Prev { signal: "output_b".to_string(), delay: 1 },
                    span: None,
                }],
                origin: None,
                span: None,
            }],
        );
        let result = check_dependency_acyclicity(&m);
        assert!(result.pass);
    }
}
