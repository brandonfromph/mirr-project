#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::SignalDecl;
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::symbolic::rewrite::RewriteEngine;
use nasa_rust_project::symbolic::{SymState, SymValue};

fn make_decl(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

#[test]
fn test_rewrite_engine_logic_simplification() {
    let signals = vec![
        make_decl("temperature", SignalType::Unsigned(16)),
        make_decl("override_active", SignalType::Bool),
    ];
    let engine = RewriteEngine::new(&signals);

    // Let's establish that override_active is definitely false,
    // and temperature is in range [0, 50].
    let mut state = SymState::new();
    state.signals.push(("temperature".to_string(), SymValue::Interval { lo: 0, hi: 50 }));
    state.signals.push(("override_active".to_string(), SymValue::Concrete(0)));

    // Expr: (temperature > 100) || override_active
    // Since temperature is <= 50, (temperature > 100) is definitely false.
    // So the expr simplifies to: false || false -> false.
    let expr = Expr::Binary {
        op: BinaryOp::Or,
        left: Box::new(Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("temperature".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(100))),
        }),
        right: Box::new(Expr::Signal("override_active".to_string())),
    };

    let rewritten = engine.rewrite_expr(expr, &state);
    assert_eq!(rewritten, Expr::Literal(LiteralValue::Bool(false)));
}

#[test]
fn test_rewrite_engine_nested_algebraic() {
    let signals =
        vec![make_decl("x", SignalType::Unsigned(8)), make_decl("y", SignalType::Unsigned(8))];
    let engine = RewriteEngine::new(&signals);

    let mut state = SymState::new();
    state.signals.push(("x".to_string(), SymValue::Concrete(0))); // x is concrete 0

    // Expr: (x * y) + 12
    // Since x is 0, x * y simplifies to 0 (arithmetic annihilation).
    // So 0 + 12 -> 12.
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(Expr::Signal("x".to_string())),
            right: Box::new(Expr::Signal("y".to_string())),
        }),
        right: Box::new(Expr::Literal(LiteralValue::Integer(12))),
    };

    let rewritten = engine.rewrite_expr(expr, &state);
    assert_eq!(rewritten, Expr::Literal(LiteralValue::Integer(12)));
}

#[test]
fn test_rewrite_engine_no_simplification_without_info() {
    let signals =
        vec![make_decl("x", SignalType::Unsigned(8)), make_decl("y", SignalType::Unsigned(8))];
    let engine = RewriteEngine::new(&signals);

    // No info in SymState (all are Top).
    let state = SymState::new();

    // Expr: x + y
    // Should remain completely unchanged.
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Signal("y".to_string())),
    };

    let rewritten = engine.rewrite_expr(expr.clone(), &state);
    assert_eq!(rewritten, expr);
}
