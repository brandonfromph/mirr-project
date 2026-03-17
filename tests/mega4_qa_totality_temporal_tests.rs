//! MEGA-4 QA — Totality Engine Part 2: Temporal bounds, acyclicity, property
//! formula coverage. 20 edge-case tests. All loops bounded. No unsafe code.

#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{
    BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType, UnaryOp,
};
use nasa_rust_project::totality::{
    check_dependency_acyclicity, check_guard_coverage, check_output_completeness,
    check_resource_bounds, check_temporal_bound, run_totality_check,
};

const MAX_TEST_SIGNALS: usize = 512;
const MAX_TEST_REFLEXES: usize = 512;

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

fn make_guard_with_condition(name: &str, cycles: u64, condition: Expr) -> Guard {
    Guard { name: name.to_string(), condition, cycles, origin: None, span: None }
}

fn make_reflex(name: &str, guard: &str, target: &str, value: Expr) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec![guard.to_string()],
        assignments: vec![Assignment { target: target.to_string(), value, span: None }],
        origin: None,
        span: None,
    }
}

fn make_reflex_no_guard(name: &str, target: &str, value: Expr) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec![],
        assignments: vec![Assignment { target: target.to_string(), value, span: None }],
        origin: None,
        span: None,
    }
}

fn make_module(signals: Vec<SignalDecl>, guards: Vec<Guard>, reflexes: Vec<Reflex>) -> Module {
    Module {
        name: "qa_test".to_string(),
        signals,
        guards,
        reflexes,
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

fn make_module_with_props(
    signals: Vec<SignalDecl>,
    guards: Vec<Guard>,
    reflexes: Vec<Reflex>,
    properties: Vec<PropertyDecl>,
) -> Module {
    Module {
        name: "qa_test".to_string(),
        signals,
        guards,
        reflexes,
        properties,
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

// Test 11: Guard with 0 cycles => max_guard_cycles=0
#[test]
fn qa_temporal_bound_guard_zero_cycles() {
    let signals =
        vec![make_signal("in", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard("g_zero", 0)];
    let reflexes = vec![make_reflex("r1", "g_zero", "out", Expr::Signal("in".to_string()))];
    let m = make_module(signals, guards, reflexes);
    let tb = check_temporal_bound(&m);
    assert_eq!(tb.max_guard_cycles, 0);
    assert_eq!(tb.max_prev_delay, 0);
    assert_eq!(tb.worst_case_latency, 0);
    assert!(tb.pass);
}

// Test 12: Guard with u64::MAX cycles — saturating_add caps at u64::MAX
#[test]
fn qa_temporal_bound_max_u64_cycles_saturating() {
    let signals =
        vec![make_signal("in", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard("g_max", u64::MAX)];
    let reflexes =
        vec![make_reflex("r1", "g_max", "out", Expr::Prev { signal: "in".to_string(), delay: 10 })];
    let m = make_module(signals, guards, reflexes);
    let tb = check_temporal_bound(&m);
    assert_eq!(tb.max_guard_cycles, u64::MAX);
    assert_eq!(tb.max_prev_delay, 10);
    assert_eq!(tb.worst_case_latency, u64::MAX);
    assert!(tb.pass);
}

// Test 13: Prev in guard condition contributes to max_prev_delay
#[test]
fn qa_temporal_bound_prev_in_guard_condition() {
    let signals =
        vec![make_signal("sensor", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard_with_condition(
        "g_prev_cond",
        5,
        Expr::Prev { signal: "sensor".to_string(), delay: 42 },
    )];
    let reflexes =
        vec![make_reflex("r1", "g_prev_cond", "out", Expr::Signal("sensor".to_string()))];
    let m = make_module(signals, guards, reflexes);
    let tb = check_temporal_bound(&m);
    assert_eq!(tb.max_guard_cycles, 5);
    assert_eq!(
        tb.max_prev_delay, 42,
        "Prev in guard condition should contribute to max_prev_delay"
    );
    assert_eq!(tb.worst_case_latency, 47);
}

// Test 14: Deeply nested Expr with multiple prev delays — picks the max
#[test]
fn qa_temporal_bound_nested_binary_prev_delays() {
    let signals = vec![
        make_signal("a", SignalKind::Input),
        make_signal("b", SignalKind::Input),
        make_signal("out", SignalKind::Output),
    ];
    let guards = vec![make_guard("g1", 3)];
    let nested_expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Prev { signal: "a".to_string(), delay: 7 }),
        right: Box::new(Expr::Binary {
            op: BinaryOp::Or,
            left: Box::new(Expr::Prev { signal: "b".to_string(), delay: 15 }),
            right: Box::new(Expr::Literal(LiteralValue::Bool(true))),
        }),
    };
    let reflexes = vec![make_reflex("r1", "g1", "out", nested_expr)];
    let m = make_module(signals, guards, reflexes);
    let tb = check_temporal_bound(&m);
    assert_eq!(tb.max_prev_delay, 15, "Should pick the max delay from all nested prev nodes");
    assert_eq!(tb.worst_case_latency, 3 + 15);
}

// Test 15: Diamond dependency (DAG, not cycle) should pass
#[test]
fn qa_acyclicity_diamond_dependency() {
    let signals = vec![
        make_signal("a", SignalKind::Input),
        make_signal("b", SignalKind::Internal),
        make_signal("c", SignalKind::Internal),
        make_signal("d", SignalKind::Output),
    ];
    let guards = vec![make_guard("g1", 1)];
    let reflexes = vec![
        make_reflex("r_ab", "g1", "b", Expr::Signal("a".to_string())),
        make_reflex("r_ac", "g1", "c", Expr::Signal("a".to_string())),
        make_reflex(
            "r_bd",
            "g1",
            "d",
            Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Signal("b".to_string())),
                right: Box::new(Expr::Signal("c".to_string())),
            },
        ),
    ];
    let m = make_module(signals, guards, reflexes);
    let ac = check_dependency_acyclicity(&m);
    assert!(ac.pass, "Diamond dependency (DAG) should not be reported as a cycle");
    assert!(ac.cycle_witness.is_none());
}

// Test 16: Two independent cycles co-existing — both detected
#[test]
fn qa_acyclicity_multiple_independent_cycles() {
    let signals = vec![
        make_signal("x", SignalKind::Output),
        make_signal("y", SignalKind::Internal),
        make_signal("z", SignalKind::Internal),
    ];
    let guards = vec![make_guard("g1", 1)];
    let reflexes = vec![
        make_reflex("r_xx", "g1", "x", Expr::Signal("x".to_string())),
        make_reflex("r_yz", "g1", "y", Expr::Signal("z".to_string())),
        make_reflex("r_zy", "g1", "z", Expr::Signal("y".to_string())),
    ];
    let m = make_module(signals, guards, reflexes);
    let ac = check_dependency_acyclicity(&m);
    assert!(!ac.pass, "Module with cycles should fail acyclicity check");
    assert!(ac.cycle_witness.is_some());
}

// Test 17: Indirect cycle through internal signal
#[test]
fn qa_acyclicity_indirect_cycle_through_internal() {
    let signals =
        vec![make_signal("out", SignalKind::Output), make_signal("internal", SignalKind::Internal)];
    let guards = vec![make_guard("g1", 1)];
    let reflexes = vec![
        make_reflex("r1", "g1", "out", Expr::Signal("internal".to_string())),
        make_reflex("r2", "g1", "internal", Expr::Signal("out".to_string())),
    ];
    let m = make_module(signals, guards, reflexes);
    let ac = check_dependency_acyclicity(&m);
    assert!(!ac.pass, "Indirect cycle through internal signal should be detected");
    assert!(ac.cycle_witness.is_some());
}

// Test 18: Prev(delay=1) is a temporal barrier — breaks the cycle
#[test]
fn qa_acyclicity_prev_delay_breaks_cycle() {
    let signals = vec![make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard("g1", 1)];
    let reflexes =
        vec![make_reflex("r1", "g1", "out", Expr::Prev { signal: "out".to_string(), delay: 1 })];
    let m = make_module(signals, guards, reflexes);
    let ac = check_dependency_acyclicity(&m);
    assert!(ac.pass, "Prev(delay=1) should break the cycle as a temporal barrier");
    assert!(ac.cycle_witness.is_none());
}

// Test 19: All 6 PropertyFormula kinds appear in property summary
#[test]
fn qa_totality_all_property_formula_kinds_in_summary() {
    let sig_x = Expr::Signal("x".to_string());
    let sig_y = Expr::Signal("y".to_string());
    let properties = vec![
        PropertyDecl {
            name: "p_always".to_string(),
            directive: PropertyDirective::Assert,
            formula: PropertyFormula::Always(sig_x.clone()),
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "p_never".to_string(),
            directive: PropertyDirective::Assert,
            formula: PropertyFormula::Never(sig_x.clone()),
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "p_always_implies".to_string(),
            directive: PropertyDirective::Assert,
            formula: PropertyFormula::AlwaysImplies {
                antecedent: sig_x.clone(),
                consequent: sig_y.clone(),
            },
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "p_never_implies".to_string(),
            directive: PropertyDirective::Assert,
            formula: PropertyFormula::NeverImplies {
                antecedent: sig_x.clone(),
                consequent: sig_y.clone(),
            },
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "p_eventually_within".to_string(),
            directive: PropertyDirective::Assert,
            formula: PropertyFormula::EventuallyWithin { expr: sig_x.clone(), cycles: 100 },
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "p_always_followed_by".to_string(),
            directive: PropertyDirective::Assert,
            formula: PropertyFormula::AlwaysFollowedBy {
                trigger: sig_x.clone(),
                response: sig_y.clone(),
                delay_cycles: 50,
            },
            origin: None,
            span: None,
        },
    ];
    let signals = vec![make_signal("x", SignalKind::Input), make_signal("y", SignalKind::Input)];
    let m = make_module_with_props(signals, vec![], vec![], properties);
    let result = run_totality_check(&m);
    assert_eq!(result.property_summary.len(), 6);
    let expected_kinds = [
        "always",
        "never",
        "always_implies",
        "never_implies",
        "eventually_within",
        "always_followed_by",
    ];
    let mut ki = 0;
    while ki < expected_kinds.len() && ki < 8 {
        assert_eq!(
            result.property_summary[ki].kind, expected_kinds[ki],
            "Property {} should have kind '{}'",
            ki, expected_kinds[ki]
        );
        ki += 1;
    }
}

// Test 20: Empty module is total
#[test]
fn qa_totality_empty_module_is_total() {
    let m = make_module(vec![], vec![], vec![]);
    let result = run_totality_check(&m);
    assert!(
        result.is_total,
        "Empty module should be total: no outputs to drive, no cycles, no resources"
    );
    assert!(result.resource_bound.pass);
    assert!(result.output_completeness.pass);
    assert!(result.guard_coverage.pass);
    assert!(result.temporal_bound.pass);
    assert!(result.acyclicity.pass);
    assert_eq!(result.resource_bound.registers, 0);
    assert_eq!(result.resource_bound.guards, 0);
    assert_eq!(result.resource_bound.instructions_estimate, 0);
    assert_eq!(result.temporal_bound.worst_case_latency, 0);
    assert!(result.property_summary.is_empty());
}

// Test 21: No guards + no prev => worst_case_latency=0
#[test]
fn qa_temporal_bound_no_guards_no_prev() {
    let signals =
        vec![make_signal("in", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let reflexes = vec![make_reflex_no_guard("r1", "out", Expr::Signal("in".to_string()))];
    let m = make_module(signals, vec![], reflexes);
    let tb = check_temporal_bound(&m);
    assert_eq!(tb.max_guard_cycles, 0, "No guards means max_guard_cycles=0");
    assert_eq!(tb.max_prev_delay, 0, "No prev refs means max_prev_delay=0");
    assert_eq!(tb.worst_case_latency, 0, "No guards + no prev => worst_case_latency=0");
    assert!(tb.pass);
}

// Test 22: 5 guards with varying cycles — max_guard_cycles picks the max
#[test]
fn qa_temporal_bound_multiple_guards_max_wins() {
    let signals =
        vec![make_signal("in", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let guards = vec![
        make_guard("g_1cycle", 1),
        make_guard("g_3cycle", 3),
        make_guard("g_7cycle", 7),
        make_guard("g_2cycle", 2),
        make_guard("g_5cycle", 5),
    ];
    let reflexes = vec![make_reflex("r1", "g_7cycle", "out", Expr::Signal("in".to_string()))];
    let m = make_module(signals, guards, reflexes);
    let tb = check_temporal_bound(&m);
    assert_eq!(
        tb.max_guard_cycles, 7,
        "max_guard_cycles should be 7 (the maximum among [1,3,7,2,5])"
    );
    assert_eq!(tb.worst_case_latency, 7);
    assert!(tb.pass);
}

// Test 23: Linear chain a->b->c->d->e (no cycle) => pass
#[test]
fn qa_acyclicity_long_chain_no_cycle() {
    let signals = vec![
        make_signal("a", SignalKind::Input),
        make_signal("b", SignalKind::Internal),
        make_signal("c", SignalKind::Internal),
        make_signal("d", SignalKind::Internal),
        make_signal("e", SignalKind::Output),
    ];
    let guards = vec![make_guard("g1", 1)];
    let reflexes = vec![
        make_reflex("r_ab", "g1", "b", Expr::Signal("a".to_string())),
        make_reflex("r_bc", "g1", "c", Expr::Signal("b".to_string())),
        make_reflex("r_cd", "g1", "d", Expr::Signal("c".to_string())),
        make_reflex("r_de", "g1", "e", Expr::Signal("d".to_string())),
    ];
    let m = make_module(signals, guards, reflexes);
    let ac = check_dependency_acyclicity(&m);
    assert!(ac.pass, "Linear chain a->b->c->d->e (no cycle) should pass acyclicity");
    assert!(ac.cycle_witness.is_none());
}

// Test 24: out = Not(Signal("out")) — self-loop via unary operator
#[test]
fn qa_acyclicity_self_loop_via_unary() {
    let signals = vec![make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard("g1", 1)];
    let reflexes = vec![make_reflex(
        "r_self",
        "g1",
        "out",
        Expr::Unary { op: UnaryOp::Not, operand: Box::new(Expr::Signal("out".to_string())) },
    )];
    let m = make_module(signals, guards, reflexes);
    let ac = check_dependency_acyclicity(&m);
    assert!(!ac.pass, "out = Not(Signal(out)) is a self-loop and should be detected as a cycle");
    assert!(ac.cycle_witness.is_some());
}

// Test 25: 100 outputs all driven by reflexes => completeness pass
#[test]
fn qa_output_completeness_100_outputs_all_driven() {
    const MAX_OUTPUTS: usize = 100;
    let mut signals: Vec<SignalDecl> = Vec::new();
    signals.push(make_signal("in", SignalKind::Input));
    let sig_bound = MAX_TEST_SIGNALS.min(MAX_OUTPUTS);
    let mut i = 0;
    while i < sig_bound {
        signals.push(make_signal(&format!("out_{}", i), SignalKind::Output));
        i += 1;
    }
    let guards = vec![make_guard("g1", 1)];
    let mut reflexes: Vec<Reflex> = Vec::new();
    let reflex_bound = MAX_TEST_REFLEXES.min(MAX_OUTPUTS);
    let mut j = 0;
    while j < reflex_bound {
        let rname = format!("r_{}", j);
        let tgt = format!("out_{}", j);
        reflexes.push(make_reflex(&rname, "g1", &tgt, Expr::Signal("in".to_string())));
        j += 1;
    }
    let m = make_module(signals, guards, reflexes);
    let oc = check_output_completeness(&m);
    assert!(oc.pass, "All 100 outputs are driven by reflexes, completeness should pass");
    assert!(oc.undriven_outputs.is_empty());
}

// Test 26: 2 outputs, 1 guardless reflex => guard coverage fails
#[test]
fn qa_guard_coverage_two_outputs_one_uncovered() {
    let signals = vec![
        make_signal("in", SignalKind::Input),
        make_signal("out_a", SignalKind::Output),
        make_signal("out_b", SignalKind::Output),
    ];
    let guards = vec![make_guard("g1", 1)];
    let reflexes = vec![
        make_reflex("r_a", "g1", "out_a", Expr::Signal("in".to_string())),
        make_reflex_no_guard("r_b", "out_b", Expr::Signal("in".to_string())),
    ];
    let m = make_module(signals, guards, reflexes);
    let gc = check_guard_coverage(&m);
    assert!(!gc.pass, "out_b is driven by a guardless reflex, guard coverage should fail");
    assert_eq!(gc.covered_outputs, 1, "Only out_a is guard-covered");
    assert_eq!(gc.total_outputs, 2);
}

// Test 27: Zero signals => resource bounds all zero, pass
#[test]
fn qa_resource_bounds_zero_signals() {
    let m = make_module(vec![], vec![], vec![]);
    let rb = check_resource_bounds(&m);
    assert_eq!(rb.registers, 0);
    assert_eq!(rb.guards, 0);
    assert_eq!(rb.instructions_estimate, 0);
    assert!(rb.pass, "Empty module should pass resource bounds with all zeros");
}

// Test 28: Prev inside unary operand => max_prev_delay picks it up
#[test]
fn qa_temporal_bound_prev_in_unary_operand() {
    let signals = vec![make_signal("x", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard("g1", 2)];
    let reflexes = vec![make_reflex(
        "r1",
        "g1",
        "out",
        Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Prev { signal: "x".to_string(), delay: 20 }),
        },
    )];
    let m = make_module(signals, guards, reflexes);
    let tb = check_temporal_bound(&m);
    assert_eq!(tb.max_prev_delay, 20, "Prev inside Not() unary operand should contribute delay=20");
    assert_eq!(tb.worst_case_latency, 2 + 20);
    assert!(tb.pass);
}

// Test 29: Binary(And, Signal("out"), Prev("out",1)) — Signal edge creates cycle
#[test]
fn qa_acyclicity_prev_in_binary_breaks_one_dep() {
    let signals = vec![make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard("g1", 1)];
    let reflexes = vec![make_reflex(
        "r1",
        "g1",
        "out",
        Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(Expr::Signal("out".to_string())),
            right: Box::new(Expr::Prev { signal: "out".to_string(), delay: 1 }),
        },
    )];
    let m = make_module(signals, guards, reflexes);
    let ac = check_dependency_acyclicity(&m);
    assert!(
        !ac.pass,
        "Signal(out) in binary expr creates a combinational self-loop even though Prev(out,1) is also present"
    );
    assert!(ac.cycle_witness.is_some());
}

// Test 30: run_totality_check — all 5 sub-results accessible and consistent with is_total
#[test]
fn qa_totality_aggregate_all_five_individual_results_accessible() {
    let signals =
        vec![make_signal("sensor", SignalKind::Input), make_signal("actuator", SignalKind::Output)];
    let guards = vec![make_guard("watchdog", 10)];
    let reflexes =
        vec![make_reflex("r_drive", "watchdog", "actuator", Expr::Signal("sensor".to_string()))];
    let properties = vec![PropertyDecl {
        name: "safety_always".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::Always(Expr::Signal("actuator".to_string())),
        origin: None,
        span: None,
    }];
    let m = make_module_with_props(signals, guards, reflexes, properties);
    let result = run_totality_check(&m);
    assert!(result.resource_bound.pass);
    assert_eq!(result.resource_bound.registers, 2);
    assert_eq!(result.resource_bound.guards, 1);
    assert!(result.output_completeness.pass);
    assert!(result.output_completeness.undriven_outputs.is_empty());
    assert!(result.guard_coverage.pass);
    assert_eq!(result.guard_coverage.covered_outputs, 1);
    assert_eq!(result.guard_coverage.total_outputs, 1);
    assert!(result.temporal_bound.pass);
    assert_eq!(result.temporal_bound.max_guard_cycles, 10);
    assert_eq!(result.temporal_bound.max_prev_delay, 0);
    assert_eq!(result.temporal_bound.worst_case_latency, 10);
    assert!(result.acyclicity.pass);
    assert!(result.acyclicity.cycle_witness.is_none());
    assert!(result.is_total, "Well-formed module with all sub-checks passing should be total");
    assert_eq!(result.property_summary.len(), 1);
    assert_eq!(result.property_summary[0].kind, "always");
    assert_eq!(result.property_summary[0].name, "safety_always");
}
