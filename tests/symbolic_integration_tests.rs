//! Integration tests for symbolic analysis engine.

#![forbid(unsafe_code)]
#![deny(warnings)]

use nasa_rust_project::symbolic::{
    sym_eval_binary, sym_eval_expr, sym_eval_unary, sym_widen, sym_check_refinement,
    SymState, SymValue,
};
use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, UnaryOp};

#[test]
fn sym_concrete_arithmetic() {
    assert_eq!(sym_eval_binary(BinaryOp::Add, SymValue::Concrete(10), SymValue::Concrete(20)), SymValue::Concrete(30));
    assert_eq!(sym_eval_binary(BinaryOp::Sub, SymValue::Concrete(50), SymValue::Concrete(20)), SymValue::Concrete(30));
    assert_eq!(sym_eval_binary(BinaryOp::Mul, SymValue::Concrete(6), SymValue::Concrete(7)), SymValue::Concrete(42));
}

#[test]
fn sym_concrete_boolean() {
    assert_eq!(sym_eval_binary(BinaryOp::And, SymValue::Concrete(1), SymValue::Concrete(1)), SymValue::Concrete(1));
    assert_eq!(sym_eval_binary(BinaryOp::And, SymValue::Concrete(1), SymValue::Concrete(0)), SymValue::Concrete(0));
    assert_eq!(sym_eval_binary(BinaryOp::Or, SymValue::Concrete(0), SymValue::Concrete(1)), SymValue::Concrete(1));
}

#[test]
fn sym_concrete_comparison() {
    assert_eq!(sym_eval_binary(BinaryOp::Lt, SymValue::Concrete(5), SymValue::Concrete(10)), SymValue::Concrete(1));
    assert_eq!(sym_eval_binary(BinaryOp::Lt, SymValue::Concrete(10), SymValue::Concrete(5)), SymValue::Concrete(0));
    assert_eq!(sym_eval_binary(BinaryOp::Eq, SymValue::Concrete(42), SymValue::Concrete(42)), SymValue::Concrete(1));
    assert_eq!(sym_eval_binary(BinaryOp::Eq, SymValue::Concrete(42), SymValue::Concrete(99)), SymValue::Concrete(0));
}

#[test]
fn sym_top_absorbs() {
    assert_eq!(sym_eval_binary(BinaryOp::Add, SymValue::Top, SymValue::Concrete(1)), SymValue::Top);
    assert_eq!(sym_eval_binary(BinaryOp::Mul, SymValue::Concrete(5), SymValue::Top), SymValue::Top);
}

#[test]
fn sym_unknown_propagation() {
    assert_eq!(sym_eval_binary(BinaryOp::Add, SymValue::Unknown { width: 8 }, SymValue::Concrete(1)), SymValue::Unknown { width: 64 });
    assert_eq!(sym_eval_binary(BinaryOp::Add, SymValue::Unknown { width: 16 }, SymValue::Unknown { width: 32 }), SymValue::Unknown { width: 32 });
}

#[test]
fn sym_refinement_check_pass() {
    assert!(sym_check_refinement(SymValue::Concrete(100), 0, 255));
    assert!(sym_check_refinement(SymValue::Interval { lo: 10, hi: 20 }, 0, 100));
}

#[test]
fn sym_refinement_check_fail() {
    assert!(!sym_check_refinement(SymValue::Concrete(300), 0, 255));
    assert!(!sym_check_refinement(SymValue::Top, 0, u64::MAX));
    assert!(!sym_check_refinement(SymValue::Unknown { width: 8 }, 0, 255));
}

#[test]
fn sym_widen_same_is_stable() {
    let v = SymValue::Concrete(5);
    assert_eq!(sym_widen(v, v), v);
}

#[test]
fn sym_widen_concrete_to_interval() {
    let result = sym_widen(SymValue::Concrete(3), SymValue::Concrete(7));
    assert_eq!(result, SymValue::Interval { lo: 3, hi: 7 });
}

#[test]
fn sym_widen_intervals_to_unknown() {
    let a = SymValue::Interval { lo: 0, hi: 10 };
    let b = SymValue::Interval { lo: 5, hi: 20 };
    assert_eq!(sym_widen(a, b), SymValue::Unknown { width: 64 });
}

#[test]
fn sym_unary_not() {
    assert_eq!(sym_eval_unary(UnaryOp::Not, SymValue::Concrete(0)), SymValue::Concrete(!0u64));
    assert_eq!(sym_eval_unary(UnaryOp::Not, SymValue::Concrete(!0u64)), SymValue::Concrete(0));
}

#[test]
fn sym_unary_negate_widens() {
    assert_eq!(sym_eval_unary(UnaryOp::Negate, SymValue::Concrete(42)), SymValue::Unknown { width: 64 });
}

#[test]
fn sym_unary_on_top() {
    assert_eq!(sym_eval_unary(UnaryOp::Not, SymValue::Top), SymValue::Top);
}

#[test]
fn sym_eval_literal_bool() {
    let state = SymState::new();
    let expr = Expr::Literal(LiteralValue::Bool(true));
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(1));
}

#[test]
fn sym_eval_literal_integer() {
    let state = SymState::new();
    let expr = Expr::Literal(LiteralValue::Integer(42));
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(42));
}

#[test]
fn eval_signal_found() {
    let mut state = SymState::new();
    state.signals.push(("x".to_string(), SymValue::Concrete(99)));
    let expr = Expr::Signal("x".to_string());
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(99));
}

#[test]
fn eval_signal_not_found_is_top() {
    let state = SymState::new();
    let expr = Expr::Signal("missing".to_string());
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Top);
}

#[test]
fn eval_binary_expr() {
    let state = SymState::new();
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Literal(LiteralValue::Integer(10))),
        right: Box::new(Expr::Literal(LiteralValue::Integer(20))),
    };
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(30));
}

#[test]
fn eval_nested_binary() {
    let state = SymState::new();
    let expr = Expr::Binary {
        op: BinaryOp::Mul,
        left: Box::new(Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Literal(LiteralValue::Integer(3))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(4))),
        }),
        right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
    };
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(35));
}

#[test]
fn lookup_returns_top_for_empty() {
    let state = SymState::new();
    assert_eq!(state.lookup("anything"), SymValue::Top);
}