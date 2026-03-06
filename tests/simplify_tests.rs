//! Unit tests for MIRR logic simplification

use nasa_rust_project::ast::Expr;
use nasa_rust_project::ast::{BinaryOp, LiteralValue, UnaryOp};
use nasa_rust_project::simplify::simplify_expr;

#[test]
fn test_and_true() {
    let expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("a".into())),
        right: Box::new(Expr::Literal(LiteralValue::Bool(true))),
    };
    assert_eq!(simplify_expr(expr), Expr::Signal("a".into()));
}

#[test]
fn test_and_false() {
    let expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("a".into())),
        right: Box::new(Expr::Literal(LiteralValue::Bool(false))),
    };
    assert_eq!(simplify_expr(expr), Expr::Literal(LiteralValue::Bool(false)));
}

#[test]
fn test_or_true() {
    let expr = Expr::Binary {
        op: BinaryOp::Or,
        left: Box::new(Expr::Signal("a".into())),
        right: Box::new(Expr::Literal(LiteralValue::Bool(true))),
    };
    assert_eq!(simplify_expr(expr), Expr::Literal(LiteralValue::Bool(true)));
}

#[test]
fn test_xor_false() {
    let expr = Expr::Binary {
        op: BinaryOp::Xor,
        left: Box::new(Expr::Signal("a".into())),
        right: Box::new(Expr::Literal(LiteralValue::Bool(false))),
    };
    assert_eq!(simplify_expr(expr), Expr::Signal("a".into()));
}

#[test]
fn test_double_negation() {
    let expr = Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Signal("a".into())),
        }),
    };
    assert_eq!(simplify_expr(expr), Expr::Signal("a".into()));
}
