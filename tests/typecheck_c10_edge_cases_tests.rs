#![allow(clippy::field_reassign_with_default)]
#![forbid(unsafe_code)]
//! MEGA-1 type-check tests — criterion C10 and edge cases.
//!
//! - C10: All property forms typecheck
//! - Edge: border conditions in type checking
//!
//! NASA P10: bounded loops, no recursion.

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Module, SignalDecl};
use mirrc::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use mirrc::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::validate_module;

fn typecheck_module(module: &Module) -> Result<(), mirrc::error::PipelineErrors> {
    let mut registry = mirrc::ecs::Registry::new();
    registry.ingest_module(module).map_err(|e| mirrc::error::PipelineErrors { errors: vec![e] })?;
    registry.semantic_validate()?;
    registry.typecheck(false)
}

fn run_src(src: &str) -> Result<mirrc::pipeline::PipelineResult, mirrc::error::PipelineErrors> {
    run_pipeline(src, &PipelineConfig::default())
}

fn simple_module_with_prop(props: Vec<PropertyDecl>) -> Module {
    Module {
        name: "prop_test".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "y".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "n".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
        ],
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: props,
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn prop_assert(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Assert,
        formula,
        origin: None,
        span: None,
    }
}

#[test]
fn c10_always_signal_typechecks() {
    let prop = prop_assert("p_always", PropertyFormula::Always(Expr::Signal("x".to_string())));
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "Always(signal) must typecheck");
}

#[test]
fn c10_always_comparison_typechecks() {
    let prop = prop_assert(
        "p_cmp",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("n".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(10))),
        }),
    );
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "Always(n > 10) must typecheck");
}

#[test]
fn c10_never_signal_typechecks() {
    let prop = prop_assert("p_never", PropertyFormula::Never(Expr::Signal("x".to_string())));
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "Never(signal) must typecheck");
}

#[test]
fn c10_eventually_within_typechecks() {
    let prop = prop_assert(
        "p_ev",
        PropertyFormula::EventuallyWithin { expr: Expr::Signal("x".to_string()), cycles: 10 },
    );
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "EventuallyWithin must typecheck");
}

#[test]
fn c10_never_comparison_typechecks() {
    use mirrc::ast::types::BinaryOp;
    let prop = prop_assert(
        "p_never_cmp",
        PropertyFormula::Never(Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("n".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(200))),
        }),
    );
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "Never(n > 200) must typecheck");
}

#[test]
fn c10_always_implies_typechecks() {
    let prop = prop_assert(
        "p_imp",
        PropertyFormula::AlwaysImplies {
            antecedent: Expr::Signal("x".to_string()),
            consequent: Expr::Signal("y".to_string()),
        },
    );
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "AlwaysImplies must typecheck");
}

#[test]
fn c10_never_implies_typechecks() {
    let prop = prop_assert(
        "p_nimp",
        PropertyFormula::NeverImplies {
            antecedent: Expr::Signal("x".to_string()),
            consequent: Expr::Signal("y".to_string()),
        },
    );
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "NeverImplies must typecheck");
}

#[test]
fn c10_always_followed_by_typechecks() {
    let prop = prop_assert(
        "p_fb",
        PropertyFormula::AlwaysFollowedBy {
            trigger: Expr::Signal("x".to_string()),
            response: Expr::Signal("y".to_string()),
            delay_cycles: 5,
        },
    );
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "AlwaysFollowedBy must typecheck");
}

#[test]
fn c10_cover_directive_typechecks() {
    let prop = PropertyDecl {
        name: "coverage_point".to_string(),
        directive: PropertyDirective::Cover,
        formula: PropertyFormula::Always(Expr::Signal("x".to_string())),
        origin: None,
        span: None,
    };
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "Cover directive must typecheck");
}

#[test]
fn c10_assume_directive_typechecks() {
    let prop = PropertyDecl {
        name: "assumption".to_string(),
        directive: PropertyDirective::Assume,
        formula: PropertyFormula::Always(Expr::Signal("x".to_string())),
        origin: None,
        span: None,
    };
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "Assume directive must typecheck");
}

#[test]
fn c10_multiple_properties_typecheck() {
    let props = vec![
        prop_assert("p1", PropertyFormula::Always(Expr::Signal("x".to_string()))),
        prop_assert("p2", PropertyFormula::Never(Expr::Signal("y".to_string()))),
        prop_assert(
            "p3",
            PropertyFormula::EventuallyWithin { expr: Expr::Signal("x".to_string()), cycles: 20 },
        ),
    ];
    let m = simple_module_with_prop(props);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "multiple properties must typecheck");
}

// Edge: property with literal condition
#[test]
fn edge_always_literal_bool_typechecks() {
    let prop =
        prop_assert("p_lit", PropertyFormula::Always(Expr::Literal(LiteralValue::Bool(true))));
    let m = simple_module_with_prop(vec![prop]);
    validate_module(&m).expect("must validate");
    assert!(typecheck_module(&m).is_ok(), "Always(true) must typecheck");
}

// Edge: source-level always property
#[test]
fn edge_source_always_property() {
    let result = run_src(
        r#"module src_prop {
    signal x: in bool;
    property liveness {
        always (x);
    }
}"#,
    );
    assert!(result.is_ok(), "source-level always property: {:?}", result.err());
}
