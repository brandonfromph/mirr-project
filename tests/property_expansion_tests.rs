//! Tests for the property system hardening campaign:
//! - Bug fix: prev() delay validation now applies to properties
//! - Refactor: PropertyFormula::exprs() / exprs_mut() centralize variant dispatch

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyFormula};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::{run_pipeline, validate_module, PipelineConfig};

// ---------------------------------------------------------------------------
// Helpers: build AST nodes directly
// ---------------------------------------------------------------------------

fn sig(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn prev(name: &str, delay: u64) -> Expr {
    Expr::Prev { signal: name.to_string(), delay }
}

fn gt(lhs: Expr, rhs: u64) -> Expr {
    Expr::Binary {
        op: BinaryOp::Gt,
        left: Box::new(lhs),
        right: Box::new(Expr::Literal(LiteralValue::Integer(rhs))),
    }
}

/// Build a minimal valid module with the given properties.
fn module_with_properties(properties: Vec<PropertyDecl>) -> Module {
    Module {
        name: "m".to_string(),
        signals: vec![
            SignalDecl {
                name: "sensor".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "alarm".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: gt(sig("sensor"), 100),
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "alarm".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties,
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

fn prop(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: nasa_rust_project::ast::property::PropertyDirective::Assert,
        formula,
        origin: None,
        span: None,
    }
}

// ---------------------------------------------------------------------------
// Bug fix: prev(sig, 0) in property formulas must be rejected
// ---------------------------------------------------------------------------

#[test]
fn prev_zero_delay_in_always_property_is_rejected() {
    let module = module_with_properties(vec![prop(
        "bad",
        PropertyFormula::Always(gt(prev("sensor", 0), 50)),
    )]);
    let errs = validate_module(&module).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    let msg = err.to_string();
    assert!(msg.contains("prev") && msg.contains("delay"), "Expected prev delay error, got: {msg}");
}

#[test]
fn prev_zero_delay_in_never_property_is_rejected() {
    let module =
        module_with_properties(vec![prop("bad", PropertyFormula::Never(prev("alarm", 0)))]);
    let errs = validate_module(&module).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    let msg = err.to_string();
    assert!(msg.contains("prev") && msg.contains("delay"), "Expected prev delay error, got: {msg}");
}

#[test]
fn prev_zero_delay_in_implies_antecedent_is_rejected() {
    let module = module_with_properties(vec![prop(
        "bad",
        PropertyFormula::AlwaysImplies {
            antecedent: gt(prev("sensor", 0), 100),
            consequent: sig("alarm"),
        },
    )]);
    let errs = validate_module(&module).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    let msg = err.to_string();
    assert!(msg.contains("prev") && msg.contains("delay"), "Expected prev delay error, got: {msg}");
}

#[test]
fn prev_zero_delay_in_implies_consequent_is_rejected() {
    let module = module_with_properties(vec![prop(
        "bad",
        PropertyFormula::AlwaysImplies {
            antecedent: gt(sig("sensor"), 100),
            consequent: prev("alarm", 0),
        },
    )]);
    let errs = validate_module(&module).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    let msg = err.to_string();
    assert!(msg.contains("prev") && msg.contains("delay"), "Expected prev delay error, got: {msg}");
}

#[test]
fn prev_valid_delay_in_property_passes() {
    let module = module_with_properties(vec![prop(
        "ok",
        PropertyFormula::Always(gt(prev("sensor", 1), 50)),
    )]);
    validate_module(&module).expect("prev(sensor, 1) should pass validation");
}

// ---------------------------------------------------------------------------
// Refactor: PropertyFormula::exprs() returns correct references
// ---------------------------------------------------------------------------

#[test]
fn exprs_always_returns_one() {
    let f = PropertyFormula::Always(sig("x"));
    assert_eq!(f.exprs().len(), 1);
}

#[test]
fn exprs_never_returns_one() {
    let f = PropertyFormula::Never(sig("x"));
    assert_eq!(f.exprs().len(), 1);
}

#[test]
fn exprs_implies_returns_two() {
    let f = PropertyFormula::AlwaysImplies { antecedent: sig("x"), consequent: sig("y") };
    assert_eq!(f.exprs().len(), 2);
}

#[test]
fn exprs_mut_allows_modification() {
    let mut f = PropertyFormula::Always(sig("x"));
    let mut exprs = f.exprs_mut();
    assert_eq!(exprs.len(), 1);
    // Verify we can mutate through the reference
    *exprs[0] = sig("y");
    assert_eq!(f.exprs()[0], &sig("y"));
}

// ---------------------------------------------------------------------------
// Backward compatibility: existing properties still work through full pipeline
// ---------------------------------------------------------------------------

#[test]
fn full_pipeline_with_all_three_property_forms() {
    let src = r#"
module m {
    signal sensor: in u16;
    signal alarm: out bool;

    guard g {
        when sensor > 100
        for 3 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }

    property bounded {
        always (sensor < 1000);
    }

    property no_false {
        never (alarm && sensor < 50);
    }

    property trigger {
        always (sensor > 100 -> alarm);
    }
}
"#;
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
    };
    let result = run_pipeline(src, &config).expect("Pipeline should succeed");
    assert_eq!(result.program.module.properties.len(), 3);

    assert!(matches!(result.program.module.properties[0].formula, PropertyFormula::Always(_)));
    assert!(matches!(result.program.module.properties[1].formula, PropertyFormula::Never(_)));
    assert!(matches!(
        result.program.module.properties[2].formula,
        PropertyFormula::AlwaysImplies { .. }
    ));
}

#[test]
fn existing_property_examples_compile() {
    let src = std::fs::read_to_string("examples/safety_property.mirr")
        .expect("safety_property.mirr should exist");
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
    };
    run_pipeline(&src, &config).expect("safety_property.mirr should compile");
}
