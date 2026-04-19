#![forbid(unsafe_code)]
//! Semantic composite expression validation tests.
//!
//! Covers E229/E230 and positive struct/array composite access paths.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::validate_module;

fn bool_assignment_module(value: Expr) -> Module {
    Module {
        name: "composite_bool".to_string(),
        signals: vec![
            SignalDecl {
                name: "trigger".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "pkt".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Struct {
                    name: "Packet".to_string(),
                    fields: vec![("ok".to_string(), SignalType::Bool)],
                }),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "samples".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Array {
                    element: Box::new(SignalType::Unsigned(8)),
                    length: 4,
                }),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "out_flag".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Bool),
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
            assignments: vec![Assignment { target: "out_flag".to_string(), value, span: None }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn u8_assignment_module(value: Expr) -> Module {
    Module {
        name: "composite_u8".to_string(),
        signals: vec![
            SignalDecl {
                name: "trigger".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "samples".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Array {
                    element: Box::new(SignalType::Unsigned(8)),
                    length: 4,
                }),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "out_u8".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
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
            assignments: vec![Assignment { target: "out_u8".to_string(), value, span: None }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn semantic_messages(module: &Module) -> Vec<String> {
    let errs = validate_module(module).expect_err("semantic validation should fail");
    errs.errors.iter().map(ToString::to_string).collect()
}

#[test]
fn missing_struct_field_reports_e229() {
    let value = Expr::FieldAccess {
        object: Box::new(Expr::Signal("pkt".to_string())),
        field: "missing".to_string(),
    };

    let messages = semantic_messages(&bool_assignment_module(value));
    assert!(messages.iter().any(|m| m.contains("[E229]")), "expected E229, got: {messages:?}");
    assert!(
        messages.iter().any(|m| m.contains("No field 'missing' on struct signal 'pkt'.")),
        "expected missing-field detail, got: {messages:?}"
    );
}

#[test]
fn indexing_non_array_signal_reports_e230() {
    let value = Expr::ArrayIndex {
        array: Box::new(Expr::Signal("pkt".to_string())),
        index: Box::new(Expr::Literal(LiteralValue::Integer(0))),
    };

    let messages = semantic_messages(&bool_assignment_module(value));
    assert!(messages.iter().any(|m| m.contains("[E230]")), "expected E230, got: {messages:?}");
    assert!(
        messages.iter().any(|m| m.contains("Signal 'pkt' is not an array type but is indexed.")),
        "expected non-array index detail, got: {messages:?}"
    );
}

#[test]
fn valid_struct_field_access_passes_semantic_validation() {
    let value = Expr::FieldAccess {
        object: Box::new(Expr::Signal("pkt".to_string())),
        field: "ok".to_string(),
    };

    validate_module(&bool_assignment_module(value)).expect("valid field access should pass");
}

#[test]
fn valid_array_index_access_passes_semantic_validation() {
    let value = Expr::ArrayIndex {
        array: Box::new(Expr::Signal("samples".to_string())),
        index: Box::new(Expr::Literal(LiteralValue::Integer(0))),
    };

    validate_module(&u8_assignment_module(value)).expect("valid array index should pass");
}
