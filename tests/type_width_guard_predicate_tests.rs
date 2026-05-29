#![forbid(unsafe_code)]
//! Type/width guard predicate contract tests.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::typeck::typecheck_module;
use nasa_rust_project::validate_module;

fn module_with_guard(condition: Expr) -> Module {
    Module {
        name: "guard_predicate_contract".to_string(),
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
    validate_module(module).expect("semantic validation should pass");
    let errs = typecheck_module(module).expect_err("typecheck should fail");
    errs.errors[0].to_string()
}

#[test]
fn bool_guard_predicate_typechecks() {
    let m = module_with_guard(Expr::Signal("x".to_string()));
    validate_module(&m).expect("semantic validation should pass");
    typecheck_module(&m).expect("bool predicate should typecheck");
}

#[test]
fn comparison_guard_predicate_typechecks() {
    let m = module_with_guard(Expr::Binary {
        op: BinaryOp::Gt,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
    });
    validate_module(&m).expect("semantic validation should pass");
    typecheck_module(&m).expect("comparison predicate should typecheck");
}

#[test]
fn non_bool_guard_predicate_reports_e601() {
    let m = module_with_guard(Expr::Signal("n".to_string()));
    let msg = typecheck_err(&m);
    assert!(msg.contains("[E601]"), "expected E601, got: {msg}");
}
