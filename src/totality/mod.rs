//! ARCHITECTURAL SUB-ENGINE: TOTALITY ENGINE
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

use crate::ecs::components;

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
pub fn run_totality_check(
    registry: &crate::ecs::Registry,
    target: &crate::emit::rspu_isa::TargetSpec,
) -> TotalityResult {
    let resource_bound = check_resource_bounds(registry, target);
    let output_completeness = check_output_completeness(registry);
    let guard_coverage = check_guard_coverage(registry);
    let temporal_bound = check_temporal_bound(registry);
    let acyclicity = check_dependency_acyclicity(registry);

    let property_summary = build_property_summary(registry);

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

/// Run all five totality analyses on a parsed MIRR module (Compatibility Helper).
pub fn run_totality_check_on_module(
    module: &crate::ast::program::Module,
    target: &crate::emit::rspu_isa::TargetSpec,
) -> TotalityResult {
    let mut reg = crate::ecs::Registry::new();
    if reg.ingest_module(module).is_err() {
        return TotalityResult {
            resource_bound: ResourceBound {
                registers: 0,
                instructions_estimate: 0,
                guards: 0,
                max_cycles: 0,
                pass: false,
            },
            output_completeness: OutputCompletenessResult {
                pass: false,
                undriven_outputs: Vec::new(),
            },
            guard_coverage: GuardCoverageResult {
                pass: false,
                covered_outputs: 0,
                total_outputs: 0,
            },
            temporal_bound: TemporalBoundResult {
                pass: false,
                worst_case_latency: u64::MAX,
                max_guard_cycles: 0,
                max_prev_delay: 0,
            },
            acyclicity: AcyclicityResult { pass: false, cycle_witness: None },
            property_summary: Vec::new(),
            is_total: false,
        };
    }
    run_totality_check(&reg, target)
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
pub fn check_dependency_acyclicity(registry: &crate::ecs::Registry) -> AcyclicityResult {
    let mut names: Vec<&str> = Vec::new();
    let mut ni = 0;

    for i in 0..registry.names.len() {
        if let (Some(name), Some(kind)) = (&registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::components::EntityKind::SIGNAL(_) = kind.0 {
                names.push(registry.resolve_name(name.0));
                ni += 1;
                if ni >= MAX_SIGNALS {
                    break;
                }
            }
        }
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

    for (ri, reflex_opt) in registry.reflex_comps.iter().enumerate() {
        if let Some(reflex) = reflex_opt {
            for (asgn_idx, asgn_ent) in reflex.assignments.iter().enumerate() {
                if asgn_idx >= MAX_REFLEXES {
                    break;
                }
                if let Some(assign) = &registry.assignment_comps[asgn_ent.0 as usize] {
                    if let Some(t_name_comp) = &registry.names[assign.target.0 as usize] {
                        let target_idx =
                            find_signal_index(&names, registry.resolve_name(t_name_comp.0));
                        if let Some(ti) = target_idx {
                            let mut deps = collect_signal_deps_ecs(assign.value, registry);

                            // MEGA-10: Guard dependencies.
                            for guard_ent in &reflex.guards {
                                if let Some(g_name_comp) = &registry.names[guard_ent.0 as usize] {
                                    let g_name = registry.resolve_name(g_name_comp.0);
                                    if g_name == "always" || g_name == "never" {
                                        continue;
                                    }
                                    if let Some(cond_comp) =
                                        &registry.conditions[guard_ent.0 as usize]
                                    {
                                        deps.extend(collect_signal_deps_ecs(cond_comp.0, registry));
                                    }
                                }
                            }

                            let mut di = 0;
                            while di < deps.len() && di < MAX_DEP_NODES {
                                if let Some(dep_idx) = find_signal_index(&names, &deps[di]) {
                                    adj[ti].push(dep_idx);
                                }
                                di += 1;
                            }
                        }
                    }
                }
            }
            if ri >= MAX_REFLEXES {
                break;
            }
        }
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
fn build_property_summary(registry: &crate::ecs::Registry) -> Vec<PropertySummary> {
    let mut summaries: Vec<PropertySummary> = Vec::new();
    let mut pi = 0;

    for i in 0..registry.property_comps.len() {
        if let (Some(name_comp), Some(prop_comp)) =
            (&registry.names[i], &registry.property_comps[i])
        {
            let kind = match &prop_comp.formula {
                crate::ast::property::PropertyFormula::Always(_) => "always",
                crate::ast::property::PropertyFormula::Never(_) => "never",
                crate::ast::property::PropertyFormula::AlwaysImplies { .. } => "always_implies",
                crate::ast::property::PropertyFormula::NeverImplies { .. } => "never_implies",
                crate::ast::property::PropertyFormula::EventuallyWithin { .. } => {
                    "eventually_within"
                }
                crate::ast::property::PropertyFormula::AlwaysFollowedBy { .. } => {
                    "always_followed_by"
                }
            };
            summaries.push(PropertySummary {
                name: registry.resolve_name(name_comp.0).to_string(),
                kind: kind.to_string(),
            });
            pi += 1;
            if pi >= MAX_SIGNALS {
                break;
            }
        }
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

/// Collect signal names referenced in an ECS expression (excluding Prev references).
fn collect_signal_deps_ecs(
    root: crate::ecs::EntityId,
    registry: &crate::ecs::Registry,
) -> Vec<String> {
    let mut deps: Vec<String> = Vec::new();
    let mut stack: Vec<crate::ecs::EntityId> = Vec::new();
    stack.push(root);

    let mut visited = 0usize;
    while let Some(ent) = stack.pop() {
        visited += 1;
        if visited >= MAX_DEP_NODES {
            break;
        }
        let i = ent.0 as usize;
        if i >= registry.names.len() {
            continue;
        }

        // Signal reference
        if let Some(components::SignalRefComponent(sig_ent)) = &registry.signal_refs[i] {
            if let Some(name) = &registry.names[sig_ent.0 as usize] {
                deps.push(registry.resolve_name(name.0).to_string());
            }
            continue;
        }

        // Skip Prev references (temporal boundary)
        if registry.prev_ops[i].is_some() {
            continue;
        }

        // Binary Ops
        if let Some(components::BinaryComponent { left, right, .. }) = &registry.binary_ops[i] {
            stack.push(*left);
            stack.push(*right);
        }

        // Unary Ops
        if let Some(components::UnaryComponent { operand, .. }) = &registry.unary_ops[i] {
            stack.push(*operand);
        }

        // Mux
        if let Some(components::MuxComponent { select, true_val, false_val }) = &registry.muxes[i] {
            stack.push(*select);
            stack.push(*true_val);
            stack.push(*false_val);
        }

        // Struct literals
        if let Some(components::StructLiteralComponent { fields, .. }) =
            &registry.struct_literals[i]
        {
            for field in fields {
                stack.push(field.1);
            }
        }

        // Array literals
        if let Some(components::ArrayLiteralComponent(elements)) = &registry.array_literals[i] {
            for el in elements {
                stack.push(*el);
            }
        }

        // Array index
        if let Some(components::ArrayIndexComponent { array, index }) = &registry.array_indices[i] {
            stack.push(*array);
            stack.push(*index);
        }

        // Field access
        if let Some(components::FieldAccessComponent { object, .. }) = &registry.field_accesses[i] {
            stack.push(*object);
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
            template_cycles: None,
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
            name: "test_module".to_string(),
            clock_domains: vec![],
            signals,
            guards,
            reflexes,
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        }
    }

    fn run_totality_on_module(m: &Module) -> TotalityResult {
        let mut reg = crate::ecs::Registry::new();
        reg.ingest_module(m).unwrap();
        let target = crate::emit::rspu_isa::TargetSpec::from_config(&None);
        run_totality_check(&reg, &target)
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
        let result = run_totality_on_module(&m);
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
        let result = run_totality_on_module(&m);
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
        let mut reg = crate::ecs::Registry::new();
        reg.ingest_module(&m).unwrap();
        let result = check_temporal_bound(&reg);
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
        let mut reg = crate::ecs::Registry::new();
        reg.ingest_module(&m).unwrap();
        let result = check_dependency_acyclicity(&reg);
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
        let mut reg = crate::ecs::Registry::new();
        reg.ingest_module(&m).unwrap();
        let result = check_dependency_acyclicity(&reg);
        assert!(result.pass);
    }
}
