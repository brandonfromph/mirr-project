#![forbid(unsafe_code)]
//! Comprehensive tests for MIRR logic simplification (Phase 3).
//!
//! Covers: boolean identity/annihilation, idempotence/absorption, comparison
//! constant folding, arithmetic identity/annihilation, arithmetic constant
//! folding, cascading simplification, fixpoint idempotence, base cases, and
//! integration with the temporal pipeline.

use nasa_rust_project::ast::Expr;
use nasa_rust_project::ast::{BinaryOp, LiteralValue, UnaryOp};
use nasa_rust_project::simplify::{simplify_expr, simplify_expr_with_stats};

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn sig(name: &str) -> Expr {
    Expr::Signal(name.into())
}
fn bool_lit(b: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(b))
}
fn int_lit(n: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(n))
}
fn not(e: Expr) -> Expr {
    Expr::Unary { op: UnaryOp::Not, operand: Box::new(e) }
}
fn bin(op: BinaryOp, l: Expr, r: Expr) -> Expr {
    Expr::Binary { op, left: Box::new(l), right: Box::new(r) }
}

// ---------------------------------------------------------------------------
// Boolean identity / annihilation (8 rules)
// ---------------------------------------------------------------------------

#[test]
fn and_true_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::And, sig("a"), bool_lit(true))), sig("a"));
}

#[test]
fn and_true_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::And, bool_lit(true), sig("a"))), sig("a"));
}

#[test]
fn and_false_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::And, sig("a"), bool_lit(false))), bool_lit(false));
}

#[test]
fn and_false_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::And, bool_lit(false), sig("a"))), bool_lit(false));
}

#[test]
fn or_false_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::Or, sig("a"), bool_lit(false))), sig("a"));
}

#[test]
fn or_false_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::Or, bool_lit(false), sig("a"))), sig("a"));
}

#[test]
fn or_true_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::Or, sig("a"), bool_lit(true))), bool_lit(true));
}

#[test]
fn or_true_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::Or, bool_lit(true), sig("a"))), bool_lit(true));
}

#[test]
fn xor_false_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::Xor, sig("a"), bool_lit(false))), sig("a"));
}

#[test]
fn xor_false_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::Xor, bool_lit(false), sig("a"))), sig("a"));
}

#[test]
fn xor_true_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::Xor, sig("a"), bool_lit(true))), not(sig("a")));
}

#[test]
fn xor_true_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::Xor, bool_lit(true), sig("a"))), not(sig("a")));
}

// ---------------------------------------------------------------------------
// Unary rules
// ---------------------------------------------------------------------------

#[test]
fn double_negation() {
    assert_eq!(simplify_expr(not(not(sig("a")))), sig("a"));
}

#[test]
fn not_true() {
    assert_eq!(simplify_expr(not(bool_lit(true))), bool_lit(false));
}

#[test]
fn not_false() {
    assert_eq!(simplify_expr(not(bool_lit(false))), bool_lit(true));
}

// ---------------------------------------------------------------------------
// Boolean idempotence / absorption (5 rules)
// ---------------------------------------------------------------------------

#[test]
fn and_idempotent() {
    assert_eq!(simplify_expr(bin(BinaryOp::And, sig("x"), sig("x"))), sig("x"));
}

#[test]
fn or_idempotent() {
    assert_eq!(simplify_expr(bin(BinaryOp::Or, sig("x"), sig("x"))), sig("x"));
}

#[test]
fn xor_self_cancel() {
    assert_eq!(simplify_expr(bin(BinaryOp::Xor, sig("x"), sig("x"))), bool_lit(false));
}

#[test]
fn and_contradiction() {
    // a && !a => false
    assert_eq!(simplify_expr(bin(BinaryOp::And, sig("a"), not(sig("a")))), bool_lit(false));
}

#[test]
fn and_contradiction_reversed() {
    // !a && a => false
    assert_eq!(simplify_expr(bin(BinaryOp::And, not(sig("a")), sig("a"))), bool_lit(false));
}

#[test]
fn or_tautology() {
    // a || !a => true
    assert_eq!(simplify_expr(bin(BinaryOp::Or, sig("a"), not(sig("a")))), bool_lit(true));
}

#[test]
fn or_tautology_reversed() {
    // !a || a => true
    assert_eq!(simplify_expr(bin(BinaryOp::Or, not(sig("a")), sig("a"))), bool_lit(true));
}

// ---------------------------------------------------------------------------
// Comparison constant folding
// ---------------------------------------------------------------------------

#[test]
fn cmp_lt_true() {
    assert_eq!(simplify_expr(bin(BinaryOp::Lt, int_lit(3), int_lit(5))), bool_lit(true));
}

#[test]
fn cmp_lt_false() {
    assert_eq!(simplify_expr(bin(BinaryOp::Lt, int_lit(5), int_lit(3))), bool_lit(false));
}

#[test]
fn cmp_le_true() {
    assert_eq!(simplify_expr(bin(BinaryOp::Le, int_lit(5), int_lit(5))), bool_lit(true));
}

#[test]
fn cmp_gt_true() {
    assert_eq!(simplify_expr(bin(BinaryOp::Gt, int_lit(7), int_lit(3))), bool_lit(true));
}

#[test]
fn cmp_ge_true() {
    assert_eq!(simplify_expr(bin(BinaryOp::Ge, int_lit(3), int_lit(3))), bool_lit(true));
}

#[test]
fn cmp_eq_true() {
    assert_eq!(simplify_expr(bin(BinaryOp::Eq, int_lit(42), int_lit(42))), bool_lit(true));
}

#[test]
fn cmp_eq_false() {
    assert_eq!(simplify_expr(bin(BinaryOp::Eq, int_lit(1), int_lit(2))), bool_lit(false));
}

#[test]
fn cmp_ne_true() {
    assert_eq!(simplify_expr(bin(BinaryOp::Ne, int_lit(1), int_lit(2))), bool_lit(true));
}

#[test]
fn cmp_ne_false() {
    assert_eq!(simplify_expr(bin(BinaryOp::Ne, int_lit(5), int_lit(5))), bool_lit(false));
}

// ---------------------------------------------------------------------------
// Arithmetic identity / annihilation
// ---------------------------------------------------------------------------

#[test]
fn add_zero_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::Add, sig("x"), int_lit(0))), sig("x"));
}

#[test]
fn add_zero_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::Add, int_lit(0), sig("x"))), sig("x"));
}

#[test]
fn sub_zero() {
    assert_eq!(simplify_expr(bin(BinaryOp::Sub, sig("x"), int_lit(0))), sig("x"));
}

#[test]
fn mul_one_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::Mul, sig("x"), int_lit(1))), sig("x"));
}

#[test]
fn mul_one_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::Mul, int_lit(1), sig("x"))), sig("x"));
}

#[test]
fn mul_zero_right() {
    assert_eq!(simplify_expr(bin(BinaryOp::Mul, sig("x"), int_lit(0))), int_lit(0));
}

#[test]
fn mul_zero_left() {
    assert_eq!(simplify_expr(bin(BinaryOp::Mul, int_lit(0), sig("x"))), int_lit(0));
}

#[test]
fn shl_zero() {
    assert_eq!(simplify_expr(bin(BinaryOp::Shl, sig("x"), int_lit(0))), sig("x"));
}

#[test]
fn shr_zero() {
    assert_eq!(simplify_expr(bin(BinaryOp::Shr, sig("x"), int_lit(0))), sig("x"));
}

// ---------------------------------------------------------------------------
// Arithmetic constant folding
// ---------------------------------------------------------------------------

#[test]
fn add_constants() {
    assert_eq!(simplify_expr(bin(BinaryOp::Add, int_lit(3), int_lit(5))), int_lit(8));
}

#[test]
fn sub_constants() {
    assert_eq!(simplify_expr(bin(BinaryOp::Sub, int_lit(10), int_lit(3))), int_lit(7));
}

#[test]
fn mul_constants() {
    assert_eq!(simplify_expr(bin(BinaryOp::Mul, int_lit(4), int_lit(8))), int_lit(32));
}

#[test]
fn shl_constants() {
    assert_eq!(simplify_expr(bin(BinaryOp::Shl, int_lit(1), int_lit(4))), int_lit(16));
}

#[test]
fn shr_constants() {
    assert_eq!(simplify_expr(bin(BinaryOp::Shr, int_lit(16), int_lit(2))), int_lit(4));
}

#[test]
fn shl_clamped_to_63() {
    // Shift by 64 should clamp to 63, not panic.
    let result = simplify_expr(bin(BinaryOp::Shl, int_lit(1), int_lit(64)));
    assert_eq!(result, int_lit(1u64 << 63));
}

#[test]
fn sub_wrapping() {
    // 0 - 1 wraps to u64::MAX.
    assert_eq!(simplify_expr(bin(BinaryOp::Sub, int_lit(0), int_lit(1))), int_lit(u64::MAX));
}

// ---------------------------------------------------------------------------
// Cascading / nested simplification
// ---------------------------------------------------------------------------

#[test]
fn cascading_and_true_or_false() {
    // (a && true) || false  =>  a || false  =>  a
    let inner = bin(BinaryOp::And, sig("a"), bool_lit(true));
    let outer = bin(BinaryOp::Or, inner, bool_lit(false));
    assert_eq!(simplify_expr(outer), sig("a"));
}

#[test]
fn cascading_double_neg_and_true() {
    // !!a && true  =>  a && true  =>  a
    let inner = not(not(sig("a")));
    let outer = bin(BinaryOp::And, inner, bool_lit(true));
    assert_eq!(simplify_expr(outer), sig("a"));
}

#[test]
fn nested_arithmetic() {
    // (3 + 5) * 2  =>  8 * 2  =>  16
    let add = bin(BinaryOp::Add, int_lit(3), int_lit(5));
    let mul = bin(BinaryOp::Mul, add, int_lit(2));
    assert_eq!(simplify_expr(mul), int_lit(16));
}

// ---------------------------------------------------------------------------
// Base cases (passthrough)
// ---------------------------------------------------------------------------

#[test]
fn signal_passthrough() {
    assert_eq!(simplify_expr(sig("x")), sig("x"));
}

#[test]
fn bool_literal_passthrough() {
    assert_eq!(simplify_expr(bool_lit(true)), bool_lit(true));
}

#[test]
fn int_literal_passthrough() {
    assert_eq!(simplify_expr(int_lit(42)), int_lit(42));
}

#[test]
fn non_simplifiable_comparison_passthrough() {
    // signal < 50  (not two literals, can't fold)
    let expr = bin(BinaryOp::Lt, sig("pressure"), int_lit(50));
    assert_eq!(simplify_expr(expr.clone()), expr);
}

// ---------------------------------------------------------------------------
// Fixpoint / idempotence
// ---------------------------------------------------------------------------

#[test]
fn fixpoint_idempotent() {
    let expr = bin(BinaryOp::And, sig("a"), bool_lit(true));
    let once = simplify_expr(expr.clone());
    let twice = simplify_expr(once.clone());
    assert_eq!(once, twice, "simplify must be idempotent");
}

// ---------------------------------------------------------------------------
// Stats API
// ---------------------------------------------------------------------------

#[test]
fn stats_reports_rule_application() {
    let expr = bin(BinaryOp::And, sig("a"), bool_lit(true));
    let (_result, stats) = simplify_expr_with_stats(expr);
    assert!(stats.rules_applied >= 1, "at least one rule should fire");
    assert!(stats.nodes_before > stats.nodes_after, "node count should decrease");
}

#[test]
fn stats_zero_for_no_simplification() {
    let expr = bin(BinaryOp::Lt, sig("pressure"), int_lit(50));
    let (_result, stats) = simplify_expr_with_stats(expr);
    assert_eq!(stats.rules_applied, 0, "no rules should fire");
    assert_eq!(stats.nodes_before, stats.nodes_after);
}

// ---------------------------------------------------------------------------
// Integration: simplification before temporal lowering
// ---------------------------------------------------------------------------

#[test]
fn temporal_pipeline_simplifies_guard_conditions() {
    // Parse a module with a guard whose condition is `airway_pressure < 50 && true`.
    // After simplification, this should reduce to `airway_pressure < 50` and
    // compile normally as a Comparison ConditionKind.
    let src = r#"
module test_simplify {
    signal airway_pressure: in u16;
    signal alarm: out bool;

    guard g1 {
        when airway_pressure < 50 && true
        for 10 cycles;
    }

    reflex r1 {
        on g1 {
            alarm = true;
        }
    }
}
"#;
    let program = nasa_rust_project::parse_mirr(src).expect("parse should succeed");
    let mut compiler = nasa_rust_project::TemporalGuardCompiler::new();
    let netlist = compiler
        .compile_temporal_guards(&program.module)
        .expect("temporal lowering should succeed after simplification");
    assert_eq!(netlist.guards.len(), 1, "should have 1 compiled guard");
}
