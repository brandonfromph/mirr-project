#![forbid(unsafe_code)]
//! Type/width shift boundary contract tests.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::width;
use nasa_rust_project::width::types::{DiagSeverity, WidthDiag};

fn sig(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn root_width(expr: &Expr, signals: &[SignalDecl]) -> u32 {
    let inferred = width::infer_widths(expr, signals);
    inferred.expr.as_ref().expect("inference should produce WidthExpr").width().0
}

#[test]
fn shl_literal_expands_width() {
    let signals = [sig("x", SignalType::Unsigned(8))];
    let expr = Expr::Binary {
        op: BinaryOp::Shl,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(3))),
    };
    assert_eq!(root_width(&expr, &signals), 11);
}

#[test]
fn shr_literal_narrows_with_min_one_bit() {
    let signals = [sig("x", SignalType::Unsigned(8))];
    let expr = Expr::Binary {
        op: BinaryOp::Shr,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(20))),
    };
    assert_eq!(root_width(&expr, &signals), 1);
}

#[test]
fn shl_variable_uses_max_shift_budget() {
    let signals = [sig("x", SignalType::Unsigned(8)), sig("amt", SignalType::Unsigned(8))];
    let expr = Expr::Binary {
        op: BinaryOp::Shl,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Signal("amt".to_string())),
    };
    assert_eq!(root_width(&expr, &signals), 71);
}

#[test]
fn shift_result_truncation_reports_e505() {
    let signals = [sig("wide", SignalType::Unsigned(32)), sig("narrow", SignalType::Unsigned(8))];
    let assignment = Assignment {
        target: "narrow".to_string(),
        value: Expr::Binary {
            op: BinaryOp::Shl,
            left: Box::new(Expr::Signal("wide".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(8))),
        },
        span: None,
    };

    let diags = width::check_assignment(&assignment, &signals);
    let errors: Vec<&WidthDiag> =
        diags.iter().filter(|d| d.severity == DiagSeverity::Error).collect();

    assert_eq!(errors.len(), 1, "expected one truncation error");
    assert!(errors[0].message.contains("[E505]"), "expected E505, got: {}", errors[0].message);
}
