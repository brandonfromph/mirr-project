#![forbid(unsafe_code)]
//! Type + width binary operator contract tests.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::typeck::typecheck_module;
use nasa_rust_project::validate_module;
use nasa_rust_project::width;

fn sig(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn infer_binary_width(op: BinaryOp, left_ty: SignalType, right_ty: SignalType) -> u32 {
    let signals = [sig("a", left_ty), sig("b", right_ty)];
    let expr = Expr::Binary {
        op,
        left: Box::new(Expr::Signal("a".to_string())),
        right: Box::new(Expr::Signal("b".to_string())),
    };
    let inferred = width::infer_widths(&expr, &signals);
    inferred.expr.as_ref().expect("inference should produce WidthExpr").width().0
}

fn module_with_guard_condition(condition: Expr) -> Module {
    Module {
        name: "type_width_binary_guard".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "n".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "m".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "out".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition,
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "out".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn typecheck_err(module: &Module) -> String {
    validate_module(module).expect("module should pass semantic validation");
    let errs = typecheck_module(module).expect_err("module should fail type check");
    errs.errors[0].to_string()
}

#[test]
fn add_u8_u8_infers_u9_width() {
    assert_eq!(
        infer_binary_width(BinaryOp::Add, SignalType::Unsigned(8), SignalType::Unsigned(8)),
        9
    );
}

#[test]
fn multiply_u8_u8_infers_u16_width() {
    assert_eq!(
        infer_binary_width(BinaryOp::Mul, SignalType::Unsigned(8), SignalType::Unsigned(8)),
        16
    );
}

#[test]
fn arithmetic_on_bool_reports_e603() {
    let condition = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
    };
    let msg = typecheck_err(&module_with_guard_condition(condition));
    assert!(msg.contains("[E603]"), "expected E603, got: {msg}");
}

#[test]
fn logical_on_unsigned_reports_e604() {
    let condition = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let msg = typecheck_err(&module_with_guard_condition(condition));
    assert!(msg.contains("[E604]"), "expected E604, got: {msg}");
}

#[test]
fn equality_cross_category_reports_e606() {
    let condition = Expr::Binary {
        op: BinaryOp::Eq,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Signal("n".to_string())),
    };
    let msg = typecheck_err(&module_with_guard_condition(condition));
    assert!(msg.contains("[E606]"), "expected E606, got: {msg}");
}
