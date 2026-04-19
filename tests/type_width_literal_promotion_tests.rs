#![forbid(unsafe_code)]
//! Type + width literal promotion contract tests.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::typeck::typecheck_module;
use nasa_rust_project::validate_module;
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

fn root_width(expr: &Expr) -> u32 {
    let inferred = width::infer_widths(expr, &[]);
    inferred.expr.as_ref().expect("inference should produce WidthExpr").width().0
}

fn module_with_assignment(target_ty: SignalType, value: Expr) -> Module {
    Module {
        name: "type_width_literal_promotion".to_string(),
        signals: vec![
            SignalDecl {
                name: "trigger".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "out".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(target_ty),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Signal("trigger".to_string()),
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment { target: "out".to_string(), value, span: None }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

#[test]
fn bool_signal_to_u1_assignment_typechecks() {
    let m = module_with_assignment(SignalType::Unsigned(1), Expr::Signal("trigger".to_string()));
    validate_module(&m).expect("module should pass semantic validation");
    typecheck_module(&m).expect("bool to u1 promotion should pass typecheck");
}

#[test]
fn integer_literal_to_u8_assignment_typechecks() {
    let m =
        module_with_assignment(SignalType::Unsigned(8), Expr::Literal(LiteralValue::Integer(42)));
    validate_module(&m).expect("module should pass semantic validation");
    typecheck_module(&m).expect("u8 literal assignment should pass typecheck");
}

#[test]
fn literal_255_infers_u8_width() {
    assert_eq!(root_width(&Expr::Literal(LiteralValue::Integer(255))), 8);
}

#[test]
fn literal_256_infers_u9_width() {
    assert_eq!(root_width(&Expr::Literal(LiteralValue::Integer(256))), 9);
}

#[test]
fn assignment_truncation_reports_e505() {
    let signals = [sig("wide", SignalType::Unsigned(32)), sig("narrow", SignalType::Unsigned(8))];
    let assignment = Assignment {
        target: "narrow".to_string(),
        value: Expr::Signal("wide".to_string()),
        span: None,
    };

    let diags = width::check_assignment(&assignment, &signals);
    let errors: Vec<&WidthDiag> =
        diags.iter().filter(|d| d.severity == DiagSeverity::Error).collect();

    assert_eq!(errors.len(), 1, "expected one truncation error");
    assert!(errors[0].message.contains("[E505]"), "expected E505, got: {}", errors[0].message);
}
