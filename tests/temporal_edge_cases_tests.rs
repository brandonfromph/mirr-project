//! Temporal compiler edge-case tests.
//!
//! Covers complex guard with OR, ConditionKind::try_from_expr for Literal
//! and Prev guard conditions, comparison literal-on-left error,
//! TemporalNetlist::summary() text, DOT emit for complex guard cluster,
//! and emit_dot for empty netlist.

use nasa_rust_project::ast::program::{Guard, Module};
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue};
use nasa_rust_project::ast::Expr;
use nasa_rust_project::temporal::emit as temporal_emit;
use nasa_rust_project::temporal::low_level_ir::{
    CompiledGuard, ComplexGuard, ConditionKind, GeneratedSignal, ShiftRegisterGuard,
    TemporalNetlist,
};
use nasa_rust_project::temporal::TemporalGuardCompiler;

// ---------------------------------------------------------------------------
// Complex guard with OR
// ---------------------------------------------------------------------------

#[test]
fn complex_guard_or_combination_compiles() {
    // guard g { when (a > 5) || (b > 10) for 4 cycles; }
    let condition = Expr::Binary {
        op: BinaryOp::Or,
        left: Box::new(Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("a".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
        }),
        right: Box::new(Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("b".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(10))),
        }),
    };

    let guard = Guard { name: "or_guard".to_string(), condition, cycles: 4 };

    let module = Module {
        name: "or_test".to_string(),
        signals: vec![],
        guards: vec![guard],
        reflexes: vec![],
    };

    let mut compiler = TemporalGuardCompiler::new();
    let netlist =
        compiler.compile_temporal_guards(&module).expect("OR complex guard should compile");

    // Should produce a Complex guard
    let complex = netlist.guards.iter().find(|g| matches!(g, CompiledGuard::Complex(_)));
    assert!(complex.is_some(), "should produce a ComplexGuard");

    if let Some(CompiledGuard::Complex(cx)) = complex {
        assert_eq!(cx.name, "or_guard");
        assert_eq!(cx.sub_guards.len(), 2);
    }
}

// ---------------------------------------------------------------------------
// ConditionKind::try_from_expr edge cases
// ---------------------------------------------------------------------------

#[test]
fn condition_kind_rejects_bare_literal() {
    let expr = Expr::Literal(LiteralValue::Bool(true));
    let result = ConditionKind::try_from_expr(&expr);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "unsupported condition expression form");
}

#[test]
fn condition_kind_rejects_prev_as_guard() {
    let expr = Expr::Prev { signal: "x".to_string(), delay: 1 };
    let result = ConditionKind::try_from_expr(&expr);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "unsupported condition expression form");
}

#[test]
fn condition_kind_rejects_literal_on_left_comparison() {
    // `42 < sensor` should fail because comparisons require <signal> <op> <literal>
    let expr = Expr::Binary {
        op: BinaryOp::Lt,
        left: Box::new(Expr::Literal(LiteralValue::Integer(42))),
        right: Box::new(Expr::Signal("sensor".to_string())),
    };
    let result = ConditionKind::try_from_expr(&expr);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "comparisons must be of the form <signal> <op> <literal>");
}

#[test]
fn condition_kind_rejects_negation_of_non_signal() {
    // `!(a > 5)` — negation of a complex expression
    let expr = Expr::Unary {
        op: nasa_rust_project::ast::types::UnaryOp::Not,
        operand: Box::new(Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("a".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
        }),
    };
    let result = ConditionKind::try_from_expr(&expr);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "negation of non-signal expressions is not supported");
}

// ---------------------------------------------------------------------------
// TemporalNetlist::summary()
// ---------------------------------------------------------------------------

#[test]
fn temporal_netlist_summary_contains_expected_lines() {
    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::SimpleSignal("sig".to_string());
    let sr = ShiftRegisterGuard::new("g".to_string(), "sig".to_string(), 4, ck);
    for (i, stage_name) in sr.stages.iter().enumerate() {
        netlist.add_signal(GeneratedSignal::shift_register_stage(stage_name.clone(), i as u32));
    }
    netlist.add_guard(CompiledGuard::ShiftRegister(sr));

    let summary = netlist.summary();
    assert!(summary.contains("Guards: 1"), "summary should mention 1 guard");
    assert!(summary.contains("Shift Registers: 4"), "summary should mention 4 SR stages");
    assert!(summary.contains("Max Delay: 4 cycles"), "summary should mention max delay");
}

// ---------------------------------------------------------------------------
// DOT emit for complex guard cluster
// ---------------------------------------------------------------------------

#[test]
fn temporal_dot_emit_complex_guard_cluster() {
    let mut netlist = TemporalNetlist::new();
    let complex = ComplexGuard::new("combo".to_string(), vec![], Expr::Signal("dummy".to_string()));
    netlist.add_guard(CompiledGuard::Complex(complex));

    let dot = temporal_emit::emit_dot(&netlist).unwrap();
    assert!(dot.contains("cluster_combo"), "DOT should have complex guard cluster");
    assert!(dot.contains("Complex: combo"), "DOT label should contain 'Complex: combo'");
    assert!(dot.contains("style=dashed"), "complex guard output should be dashed");
}

// ---------------------------------------------------------------------------
// DOT emit for empty netlist
// ---------------------------------------------------------------------------

#[test]
fn temporal_dot_emit_empty_netlist() {
    let netlist = TemporalNetlist::new();
    let dot = temporal_emit::emit_dot(&netlist).unwrap();

    assert!(dot.starts_with("digraph TemporalNetlist {"));
    assert!(dot.ends_with("}\n"));
    // No cluster subgraphs
    assert!(!dot.contains("cluster_"));
}
