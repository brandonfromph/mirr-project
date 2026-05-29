//! MEGA-4 QA Edge-Case Tests — Totality Engine Part 1: Resource bounds +
//! output completeness + guard coverage.
//!
//! 10 focused edge-case tests targeting resource-bound boundaries, output
//! completeness corner cases, and guard coverage semantics.
//!
//! Every loop is bounded by a MAX_* constant. No recursion. No unsafe code.

#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::emit::rspu_isa::{MAX_GUARDS, MAX_REGISTERS};
use nasa_rust_project::totality::{
    check_guard_coverage, check_output_completeness, check_resource_bounds,
};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA P10)
// ---------------------------------------------------------------------------

const MAX_TEST_SIGNALS: usize = 512;
const MAX_TEST_GUARDS: usize = 128;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

/// Build N signals with format "sig_<index>" of given kind.
fn build_n_signals(n: usize, kind: SignalKind) -> Vec<SignalDecl> {
    let mut signals: Vec<SignalDecl> = Vec::new();
    let mut i = 0;
    while i < n && i < MAX_TEST_SIGNALS {
        let name = format!("sig_{}", i);
        signals.push(make_signal(&name, kind));
        i += 1;
    }
    signals
}

/// Build N guards with format "g_<index>" all with given cycles.
fn build_n_guards(n: usize, cycles: u64) -> Vec<Guard> {
    let mut guards: Vec<Guard> = Vec::new();
    let mut i = 0;
    while i < n && i < MAX_TEST_GUARDS {
        let name = format!("g_{}", i);
        guards.push(make_guard(&name, cycles));
        i += 1;
    }
    guards
}

// ---------------------------------------------------------------------------
// Test 1: Resource bounds exactly at MAX_REGISTERS (256 signals)
// ---------------------------------------------------------------------------
#[test]
fn qa_resource_bounds_exactly_at_max_registers() {
    let signals = build_n_signals(MAX_REGISTERS, SignalKind::Internal);
    let m = make_module(signals, vec![], vec![]);
    let rb = check_resource_bounds(&m);
    assert_eq!(rb.registers as usize, MAX_REGISTERS);
    assert!(rb.pass, "Module with exactly MAX_REGISTERS signals should pass resource bounds");
}

// ---------------------------------------------------------------------------
// Test 2: Resource bounds exceeds MAX_REGISTERS (257 signals)
// ---------------------------------------------------------------------------
#[test]
fn qa_resource_bounds_exceeds_max_registers() {
    let signals = build_n_signals(MAX_REGISTERS + 1, SignalKind::Internal);
    let m = make_module(signals, vec![], vec![]);
    let rb = check_resource_bounds(&m);
    assert!(
        !rb.pass,
        "Module with {} signals should exceed MAX_REGISTERS={}",
        MAX_REGISTERS + 1,
        MAX_REGISTERS
    );
}

// ---------------------------------------------------------------------------
// Test 3: Resource bounds exactly at MAX_GUARDS (64 guards)
// ---------------------------------------------------------------------------
#[test]
fn qa_resource_bounds_exactly_at_max_guards() {
    let signals =
        vec![make_signal("in", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let guards = build_n_guards(MAX_GUARDS, 1);
    let reflexes = vec![make_reflex("r0", "g_0", "out", Expr::Signal("in".to_string()))];
    let m = make_module(signals, guards, reflexes);
    let rb = check_resource_bounds(&m);
    assert_eq!(rb.guards as usize, MAX_GUARDS);
    assert!(rb.pass, "Module with exactly MAX_GUARDS guards should pass");
}

// ---------------------------------------------------------------------------
// Test 4: Resource bounds exceeds MAX_GUARDS (65 guards)
// ---------------------------------------------------------------------------
#[test]
fn qa_resource_bounds_exceeds_max_guards() {
    let signals = vec![make_signal("in", SignalKind::Input)];
    let guards = build_n_guards(MAX_GUARDS + 1, 1);
    let m = make_module(signals, guards, vec![]);
    let rb = check_resource_bounds(&m);
    assert!(
        !rb.pass,
        "Module with {} guards should exceed MAX_GUARDS={}",
        MAX_GUARDS + 1,
        MAX_GUARDS
    );
}

// ---------------------------------------------------------------------------
// Test 5: Resource bounds instruction estimate accuracy
// ---------------------------------------------------------------------------
#[test]
fn qa_resource_bounds_instruction_estimate_accuracy() {
    // Formula: signals + guards*3 + reflex_assignments + properties
    let signals = vec![
        make_signal("in1", SignalKind::Input),
        make_signal("in2", SignalKind::Input),
        make_signal("out1", SignalKind::Output),
        make_signal("out2", SignalKind::Output),
    ];
    let guards = vec![make_guard("g1", 5), make_guard("g2", 10)];
    let reflexes = vec![
        make_reflex("r1", "g1", "out1", Expr::Signal("in1".to_string())),
        make_reflex("r2", "g2", "out2", Expr::Signal("in2".to_string())),
    ];
    let properties = vec![PropertyDecl {
        name: "p1".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::Always(Expr::Signal("out1".to_string())),
        origin: None,
        span: None,
    }];
    let m = make_module_with_props(signals, guards, reflexes, properties);
    let rb = check_resource_bounds(&m);

    // 4 signals + 2*3 guards + 2 reflex assignments + 1 property = 13
    let expected: u32 = 4 + 2 * 3 + 2 + 1;
    assert_eq!(
        rb.instructions_estimate, expected,
        "Instruction estimate should be signals({}) + guards*3({}) + reflex_asgns({}) + props({}) = {}",
        4, 6, 2, 1, expected
    );
}

// ---------------------------------------------------------------------------
// Test 6: Output driven by multiple reflexes still passes
// ---------------------------------------------------------------------------
#[test]
fn qa_output_completeness_output_driven_by_multiple_reflexes() {
    let signals =
        vec![make_signal("in", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard("g1", 1), make_guard("g2", 2)];
    let reflexes = vec![
        make_reflex("r1", "g1", "out", Expr::Signal("in".to_string())),
        make_reflex("r2", "g2", "out", Expr::Literal(LiteralValue::Bool(false))),
    ];
    let m = make_module(signals, guards, reflexes);
    let oc = check_output_completeness(&m);
    assert!(oc.pass, "Output driven by multiple reflexes should still pass completeness");
    assert!(oc.undriven_outputs.is_empty());
}

// ---------------------------------------------------------------------------
// Test 7: Module with only input signals is trivially total
// ---------------------------------------------------------------------------
#[test]
fn qa_output_completeness_only_inputs_is_trivially_total() {
    let signals = vec![
        make_signal("in1", SignalKind::Input),
        make_signal("in2", SignalKind::Input),
        make_signal("in3", SignalKind::Input),
    ];
    let m = make_module(signals, vec![], vec![]);
    let oc = check_output_completeness(&m);
    assert!(
        oc.pass,
        "Module with only inputs has no outputs to drive, so completeness should pass"
    );
    assert!(oc.undriven_outputs.is_empty());
}

// ---------------------------------------------------------------------------
// Test 8: Internal signals are not checked for drivers
// ---------------------------------------------------------------------------
#[test]
fn qa_output_completeness_internal_signals_ignored() {
    let signals = vec![
        make_signal("in", SignalKind::Input),
        make_signal("internal_x", SignalKind::Internal),
        make_signal("out", SignalKind::Output),
    ];
    let guards = vec![make_guard("g1", 1)];
    let reflexes = vec![make_reflex("r1", "g1", "out", Expr::Signal("in".to_string()))];
    // internal_x is NOT driven by any reflex, but completeness should still pass
    let m = make_module(signals, guards, reflexes);
    let oc = check_output_completeness(&m);
    assert!(oc.pass, "Internal signals should not be checked for drivers");
    assert!(oc.undriven_outputs.is_empty());
}

// ---------------------------------------------------------------------------
// Test 9: Reflex with empty guard_names means output NOT covered by guard
// ---------------------------------------------------------------------------
#[test]
fn qa_guard_coverage_reflex_with_empty_guard_names() {
    let signals =
        vec![make_signal("in", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let reflexes = vec![make_reflex_no_guard("r_no_guard", "out", Expr::Signal("in".to_string()))];
    let m = make_module(signals, vec![], reflexes);
    let gc = check_guard_coverage(&m);
    assert!(!gc.pass, "Reflex with empty guard_names should not contribute to guard coverage");
    assert_eq!(gc.covered_outputs, 0);
    assert_eq!(gc.total_outputs, 1);
}

// ---------------------------------------------------------------------------
// Test 10: Multiple guards covering the same output from different reflexes
// ---------------------------------------------------------------------------
#[test]
fn qa_guard_coverage_multiple_guards_covering_same_output() {
    let signals =
        vec![make_signal("in", SignalKind::Input), make_signal("out", SignalKind::Output)];
    let guards = vec![make_guard("g1", 1), make_guard("g2", 2)];
    let reflexes = vec![
        make_reflex("r1", "g1", "out", Expr::Signal("in".to_string())),
        make_reflex("r2", "g2", "out", Expr::Literal(LiteralValue::Bool(true))),
    ];
    let m = make_module(signals, guards, reflexes);
    let gc = check_guard_coverage(&m);
    assert!(gc.pass);
    // Only 1 output, covered_outputs should be 1 (not 2)
    assert_eq!(gc.covered_outputs, 1);
    assert_eq!(gc.total_outputs, 1);
}
