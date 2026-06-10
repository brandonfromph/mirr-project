#![forbid(unsafe_code)]
//! Tests for the property system expansion campaign:
//!
//! - PropertyDirective: Assert, Cover, Assume
//! - PropertyFormula: NeverImplies, EventuallyWithin, AlwaysFollowedBy
//! - Emitter coverage: Verilog/SVA, FIRRTL, JSON, DOT
//! - Parser round-trip and validation edge cases
//!
//! 51 tests total.

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use mirrc::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use mirrc::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use mirrc::{run_pipeline, validate_module, PipelineConfig};

// ---------------------------------------------------------------------------
// Helpers
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
            template_cycles: None,
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
        directive: PropertyDirective::Assert,
        formula,
        origin: None,
        span: None,
    }
}

fn pipeline_config() -> PipelineConfig {
    PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    }
}

/// Helper: run pipeline expecting an error (avoids needing Debug on PipelineResult).
fn run_pipeline_expect_err(src: &str) -> mirrc::error::MirrError {
    match run_pipeline(src, &pipeline_config()) {
        Err(errs) => errs.errors.into_iter().next().expect("should have at least one error"),
        Ok(_) => panic!("Expected pipeline error, but pipeline succeeded"),
    }
}

/// Build a minimal valid MIRR module source with the given property formula line.
fn mirr_with_property(property_line: &str) -> String {
    format!(
        r#"
module m {{
    signal x: in u16;
    signal y: out bool;

    guard g {{
        when x > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            y = true;
        }}
    }}

    property p {{
        {property_line}
    }}
}}
"#
    )
}

/// Build a minimal valid MIRR module source with multiple property blocks.
fn mirr_with_properties(properties: &[(&str, &str)]) -> String {
    let mut props = String::new();
    for (name, formula_line) in properties {
        props.push_str(&format!("\n    property {name} {{\n        {formula_line}\n    }}\n"));
    }
    format!(
        r#"
module m {{
    signal x: in u16;
    signal y: out bool;

    guard g {{
        when x > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            y = true;
        }}
    }}
{props}}}
"#
    )
}

// ===========================================================================
// Section A: PropertyDirective AST (3 tests)
// ===========================================================================

#[test]
fn directive_default_is_assert() {
    let d = PropertyDirective::default();
    assert_eq!(d, PropertyDirective::Assert);
}

#[test]
fn directive_all_three_distinct() {
    let a = PropertyDirective::Assert;
    let c = PropertyDirective::Cover;
    let s = PropertyDirective::Assume;
    assert_ne!(a, c);
    assert_ne!(a, s);
    assert_ne!(c, s);
}

#[test]
fn directive_clone_eq() {
    let d = PropertyDirective::Cover;
    let d2 = d;
    assert_eq!(d, d2);
}

// ===========================================================================
// Section B: New formula variant AST (6 tests)
// ===========================================================================

#[test]
fn never_implies_fields() {
    let f = PropertyFormula::NeverImplies { antecedent: sig("a"), consequent: sig("b") };
    match f {
        PropertyFormula::NeverImplies { antecedent, consequent } => {
            assert_eq!(antecedent, sig("a"));
            assert_eq!(consequent, sig("b"));
        }
        _ => panic!("Expected NeverImplies"),
    }
}

#[test]
fn eventually_within_fields() {
    let f = PropertyFormula::EventuallyWithin { expr: sig("x"), cycles: 5 };
    match f {
        PropertyFormula::EventuallyWithin { expr, cycles } => {
            assert_eq!(expr, sig("x"));
            assert_eq!(cycles, 5);
        }
        _ => panic!("Expected EventuallyWithin"),
    }
}

#[test]
fn always_followed_by_fields() {
    let f = PropertyFormula::AlwaysFollowedBy {
        trigger: sig("t"),
        response: sig("r"),
        delay_cycles: 3,
    };
    match f {
        PropertyFormula::AlwaysFollowedBy { trigger, response, delay_cycles } => {
            assert_eq!(trigger, sig("t"));
            assert_eq!(response, sig("r"));
            assert_eq!(delay_cycles, 3);
        }
        _ => panic!("Expected AlwaysFollowedBy"),
    }
}

#[test]
fn exprs_never_implies_returns_two() {
    let f = PropertyFormula::NeverImplies { antecedent: sig("a"), consequent: sig("b") };
    assert_eq!(f.exprs().len(), 2);
}

#[test]
fn exprs_eventually_within_returns_one() {
    let f = PropertyFormula::EventuallyWithin { expr: sig("x"), cycles: 10 };
    assert_eq!(f.exprs().len(), 1);
}

#[test]
fn exprs_always_followed_by_returns_two() {
    let f = PropertyFormula::AlwaysFollowedBy {
        trigger: sig("t"),
        response: sig("r"),
        delay_cycles: 5,
    };
    assert_eq!(f.exprs().len(), 2);
}

// ===========================================================================
// Section C: exprs_mut for new variants (3 tests)
// ===========================================================================

#[test]
fn exprs_mut_never_implies_modifiable() {
    let mut f = PropertyFormula::NeverImplies { antecedent: sig("a"), consequent: sig("b") };
    let mut exprs = f.exprs_mut();
    assert_eq!(exprs.len(), 2);
    *exprs[0] = sig("c");
    drop(exprs);
    assert_eq!(f.exprs()[0], &sig("c"));
}

#[test]
fn exprs_mut_eventually_within_modifiable() {
    let mut f = PropertyFormula::EventuallyWithin { expr: sig("x"), cycles: 10 };
    let mut exprs = f.exprs_mut();
    *exprs[0] = sig("y");
    drop(exprs);
    assert_eq!(f.exprs()[0], &sig("y"));
}

#[test]
fn exprs_mut_always_followed_by_modifiable() {
    let mut f = PropertyFormula::AlwaysFollowedBy {
        trigger: sig("t"),
        response: sig("r"),
        delay_cycles: 2,
    };
    let mut exprs = f.exprs_mut();
    *exprs[1] = sig("s");
    drop(exprs);
    assert_eq!(f.exprs()[1], &sig("s"));
}

// ===========================================================================
// Section D: Parser — directive detection (6 tests)
// ===========================================================================

#[test]
fn parse_assert_always_default() {
    let src = mirr_with_property("always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Assert);
}

#[test]
fn parse_cover_always() {
    let src = mirr_with_property("cover always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Cover);
}

#[test]
fn parse_assume_always() {
    let src = mirr_with_property("assume always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Assume);
}

#[test]
fn parse_cover_never() {
    let src = mirr_with_property("cover never (x > 100);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Cover);
    assert!(matches!(result.program.module.properties[0].formula, PropertyFormula::Never(_)));
}

#[test]
fn parse_assume_implies() {
    let src = mirr_with_property("assume always (x > 100 -> y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Assume);
    assert!(matches!(
        result.program.module.properties[0].formula,
        PropertyFormula::AlwaysImplies { .. }
    ));
}

#[test]
fn parse_cover_shorthand_parens() {
    let src = mirr_with_property("cover (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Cover);
    assert!(matches!(result.program.module.properties[0].formula, PropertyFormula::Always(_)));
}

// ===========================================================================
// Section E: Parser — NeverImplies (3 tests)
// ===========================================================================

#[test]
fn parse_never_implies() {
    let src = mirr_with_property("never (x > 100 -> y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert!(matches!(
        result.program.module.properties[0].formula,
        PropertyFormula::NeverImplies { .. }
    ));
}

#[test]
fn parse_cover_never_implies() {
    let src = mirr_with_property("cover never (x > 100 -> y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Cover);
    assert!(matches!(
        result.program.module.properties[0].formula,
        PropertyFormula::NeverImplies { .. }
    ));
}

#[test]
fn never_implies_validation_passes() {
    let module = module_with_properties(vec![prop(
        "ok",
        PropertyFormula::NeverImplies {
            antecedent: gt(sig("sensor"), 100),
            consequent: sig("alarm"),
        },
    )]);
    validate_module(&module).expect("NeverImplies should pass validation");
}

// ===========================================================================
// Section F: Parser — EventuallyWithin (5 tests)
// ===========================================================================

#[test]
fn parse_eventually_within() {
    let src = mirr_with_property("eventually within 10 (y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    match &result.program.module.properties[0].formula {
        PropertyFormula::EventuallyWithin { cycles, .. } => {
            assert_eq!(*cycles, 10);
        }
        other => panic!("Expected EventuallyWithin, got: {:?}", other),
    }
}

#[test]
fn parse_cover_eventually_within() {
    let src = mirr_with_property("cover eventually within 5 (y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Cover);
    assert!(matches!(
        result.program.module.properties[0].formula,
        PropertyFormula::EventuallyWithin { .. }
    ));
}

#[test]
fn eventually_within_zero_cycles_rejected() {
    let src = mirr_with_property("eventually within 0 (y);");
    let err = run_pipeline_expect_err(&src);
    let msg = err.to_string();
    assert!(
        msg.contains("cycles >= 1") || msg.contains("eventually"),
        "Expected cycle >= 1 error, got: {msg}"
    );
}

#[test]
fn eventually_within_missing_within_rejected() {
    let src = mirr_with_property("eventually (y);");
    let err = run_pipeline_expect_err(&src);
    let msg = err.to_string();
    assert!(msg.contains("eventually within"), "Expected 'eventually within' error, got: {msg}");
}

#[test]
fn eventually_within_validation_passes() {
    let module = module_with_properties(vec![prop(
        "ok",
        PropertyFormula::EventuallyWithin { expr: sig("alarm"), cycles: 10 },
    )]);
    validate_module(&module).expect("EventuallyWithin should pass validation");
}

// ===========================================================================
// Section G: Parser — AlwaysFollowedBy (5 tests)
// ===========================================================================

#[test]
fn parse_always_followed_by() {
    let src = mirr_with_property("always (x > 100 followed_by 5 y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    match &result.program.module.properties[0].formula {
        PropertyFormula::AlwaysFollowedBy { delay_cycles, .. } => {
            assert_eq!(*delay_cycles, 5);
        }
        other => panic!("Expected AlwaysFollowedBy, got: {:?}", other),
    }
}

#[test]
fn parse_assume_always_followed_by() {
    let src = mirr_with_property("assume always (x > 100 followed_by 3 y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Assume);
    assert!(matches!(
        result.program.module.properties[0].formula,
        PropertyFormula::AlwaysFollowedBy { .. }
    ));
}

#[test]
fn followed_by_zero_delay_rejected() {
    let src = mirr_with_property("always (x > 100 followed_by 0 y);");
    let err = run_pipeline_expect_err(&src);
    let msg = err.to_string();
    assert!(
        msg.contains("delay >= 1") || msg.contains("followed_by"),
        "Expected delay >= 1 error, got: {msg}"
    );
}

#[test]
fn followed_by_invalid_delay_rejected() {
    let src = mirr_with_property("always (x > 100 followed_by abc y);");
    let err = run_pipeline_expect_err(&src);
    let msg = err.to_string();
    assert!(
        msg.contains("invalid delay") || msg.contains("followed_by"),
        "Expected invalid delay error, got: {msg}"
    );
}

#[test]
fn followed_by_validation_passes() {
    let module = module_with_properties(vec![prop(
        "ok",
        PropertyFormula::AlwaysFollowedBy {
            trigger: gt(sig("sensor"), 100),
            response: sig("alarm"),
            delay_cycles: 5,
        },
    )]);
    validate_module(&module).expect("AlwaysFollowedBy should pass validation");
}

// ===========================================================================
// Section H: Verilog/SVA emission (7 tests)
// ===========================================================================

#[test]
fn sva_assert_always_keyword() {
    let src = mirr_with_property("always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let sv = mirrc::emit::verilog::emit_sv(&result);
    assert!(sv.contains("assert property"), "Expected 'assert property' in SVA: {sv}");
}

#[test]
fn sva_cover_keyword() {
    let src = mirr_with_property("cover always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let sv = mirrc::emit::verilog::emit_sv(&result);
    assert!(sv.contains("cover property"), "Expected 'cover property' in SVA: {sv}");
}

#[test]
fn sva_assume_keyword() {
    let src = mirr_with_property("assume always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let sv = mirrc::emit::verilog::emit_sv(&result);
    assert!(sv.contains("assume property"), "Expected 'assume property' in SVA: {sv}");
}

#[test]
fn sva_never_implies_output() {
    let src = mirr_with_property("never (x > 100 -> y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let sv = mirrc::emit::verilog::emit_sv(&result);
    assert!(sv.contains("|->"), "Expected '|->' in SVA never implies: {sv}");
    assert!(sv.contains("!("), "Expected negation in SVA never implies: {sv}");
}

#[test]
fn sva_eventually_within_output() {
    let src = mirr_with_property("eventually within 10 (y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let sv = mirrc::emit::verilog::emit_sv(&result);
    assert!(sv.contains("prop_p_timer < 10"), "Expected 'prop_p_timer < 10' in SVA: {sv}");
}

#[test]
fn sva_followed_by_output() {
    let src = mirr_with_property("always (x > 100 followed_by 5 y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let sv = mirrc::emit::verilog::emit_sv(&result);
    assert!(
        sv.contains("prop_p_trig_shift[4] |-> y"),
        "Expected shift register logic in SVA: {sv}"
    );
}

#[test]
fn sva_cover_never_implies_combined() {
    let src = mirr_with_property("cover never (x > 100 -> y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let sv = mirrc::emit::verilog::emit_sv(&result);
    assert!(sv.contains("cover property"), "Expected 'cover property': {sv}");
    assert!(sv.contains("|->"), "Expected '|->' in SVA: {sv}");
}

// ===========================================================================
// Section I: JSON netlist emission (4 tests)
// ===========================================================================

#[test]
fn json_directive_field_present() {
    let src = mirr_with_property("cover always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let json = mirrc::emit::json_netlist::emit_json(&result).unwrap();
    assert!(json.contains("\"directive\""), "Expected directive in JSON: {json}");
    assert!(json.contains("\"cover\""), "Expected 'cover' directive: {json}");
}

#[test]
fn json_never_implies_kind() {
    let src = mirr_with_property("never (x > 100 -> y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let json = mirrc::emit::json_netlist::emit_json(&result).unwrap();
    assert!(json.contains("\"never_implies\""), "Expected 'never_implies' kind in JSON: {json}");
}

#[test]
fn json_eventually_within_kind() {
    let src = mirr_with_property("eventually within 7 (y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let json = mirrc::emit::json_netlist::emit_json(&result).unwrap();
    assert!(json.contains("\"eventually_within\""), "Expected 'eventually_within' kind: {json}");
}

#[test]
fn json_always_followed_by_kind() {
    let src = mirr_with_property("always (x > 100 followed_by 3 y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let json = mirrc::emit::json_netlist::emit_json(&result).unwrap();
    assert!(json.contains("\"always_followed_by\""), "Expected 'always_followed_by' kind: {json}");
}

// ===========================================================================
// Section J: FIRRTL emission (3 tests)
// ===========================================================================

#[test]
fn firrtl_property_comment_assert() {
    let src = mirr_with_property("always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let firrtl = mirrc::emit::firrtl::emit_firrtl(&result);
    assert!(firrtl.contains("; property p:"), "Expected FIRRTL property comment: {firrtl}");
}

#[test]
fn firrtl_property_comment_cover_prefix() {
    let src = mirr_with_property("cover always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let firrtl = mirrc::emit::firrtl::emit_firrtl(&result);
    assert!(
        firrtl.contains("; cover property p:"),
        "Expected 'cover property' in FIRRTL comment: {firrtl}"
    );
}

#[test]
fn firrtl_eventually_within_comment() {
    let src = mirr_with_property("eventually within 5 (y);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let firrtl = mirrc::emit::firrtl::emit_firrtl(&result);
    assert!(
        firrtl.contains("eventually within 5"),
        "Expected 'eventually within 5' in FIRRTL: {firrtl}"
    );
}

// ===========================================================================
// Section K: DOT emission (2 tests)
// ===========================================================================

#[test]
fn dot_assert_property_fillcolor() {
    let src = mirr_with_property("always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let dot = mirrc::emit::dot::emit_module_dot(&result);
    assert!(dot.contains("fillcolor=lightblue"), "Expected lightblue for assert property: {dot}");
}

#[test]
fn dot_cover_property_fillcolor() {
    let src = mirr_with_property("cover always (x > 0);");
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    let dot = mirrc::emit::dot::emit_module_dot(&result);
    assert!(
        dot.contains("fillcolor=lightyellow"),
        "Expected lightyellow for cover property: {dot}"
    );
}

// ===========================================================================
// Section L: Full pipeline round-trip (3 tests)
// ===========================================================================

#[test]
fn full_pipeline_all_six_formula_variants() {
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

    property p1 {
        always (sensor < 1000);
    }

    property p2 {
        never (alarm && sensor < 50);
    }

    property p3 {
        always (sensor > 100 -> alarm);
    }

    property p4 {
        never (sensor > 100 -> alarm);
    }

    property p5 {
        eventually within 10 (alarm);
    }

    property p6 {
        always (sensor > 200 followed_by 3 alarm);
    }
}
"#;
    let result = run_pipeline(src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties.len(), 6);

    assert!(matches!(result.program.module.properties[0].formula, PropertyFormula::Always(_)));
    assert!(matches!(result.program.module.properties[1].formula, PropertyFormula::Never(_)));
    assert!(matches!(
        result.program.module.properties[2].formula,
        PropertyFormula::AlwaysImplies { .. }
    ));
    assert!(matches!(
        result.program.module.properties[3].formula,
        PropertyFormula::NeverImplies { .. }
    ));
    assert!(matches!(
        result.program.module.properties[4].formula,
        PropertyFormula::EventuallyWithin { .. }
    ));
    assert!(matches!(
        result.program.module.properties[5].formula,
        PropertyFormula::AlwaysFollowedBy { .. }
    ));
}

#[test]
fn full_pipeline_mixed_directives() {
    let src = mirr_with_properties(&[
        ("p1", "always (x > 0);"),
        ("p2", "cover never (y);"),
        ("p3", "assume always (x < 1000);"),
    ]);
    let result = run_pipeline(&src, &pipeline_config()).unwrap();
    assert_eq!(result.program.module.properties.len(), 3);
    assert_eq!(result.program.module.properties[0].directive, PropertyDirective::Assert);
    assert_eq!(result.program.module.properties[1].directive, PropertyDirective::Cover);
    assert_eq!(result.program.module.properties[2].directive, PropertyDirective::Assume);
}

#[test]
fn full_pipeline_existing_example_still_compiles() {
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
        mape_k: false,
        ..PipelineConfig::default()
    };
    run_pipeline(&src, &config).expect("safety_property.mirr should compile");
}

// ===========================================================================
// Section M: Validation — prev() delay bug fix in new variants (6 tests)
// ===========================================================================

#[test]
fn prev_zero_delay_in_never_implies_antecedent_rejected() {
    let module = module_with_properties(vec![prop(
        "bad",
        PropertyFormula::NeverImplies {
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
fn prev_zero_delay_in_never_implies_consequent_rejected() {
    let module = module_with_properties(vec![prop(
        "bad",
        PropertyFormula::NeverImplies {
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
fn prev_zero_delay_in_eventually_within_rejected() {
    let module = module_with_properties(vec![prop(
        "bad",
        PropertyFormula::EventuallyWithin { expr: gt(prev("sensor", 0), 50), cycles: 10 },
    )]);
    let errs = validate_module(&module).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    let msg = err.to_string();
    assert!(msg.contains("prev") && msg.contains("delay"), "Expected prev delay error, got: {msg}");
}

#[test]
fn prev_zero_delay_in_followed_by_trigger_rejected() {
    let module = module_with_properties(vec![prop(
        "bad",
        PropertyFormula::AlwaysFollowedBy {
            trigger: gt(prev("sensor", 0), 100),
            response: sig("alarm"),
            delay_cycles: 5,
        },
    )]);
    let errs = validate_module(&module).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    let msg = err.to_string();
    assert!(msg.contains("prev") && msg.contains("delay"), "Expected prev delay error, got: {msg}");
}

#[test]
fn prev_zero_delay_in_followed_by_response_rejected() {
    let module = module_with_properties(vec![prop(
        "bad",
        PropertyFormula::AlwaysFollowedBy {
            trigger: gt(sig("sensor"), 100),
            response: prev("alarm", 0),
            delay_cycles: 5,
        },
    )]);
    let errs = validate_module(&module).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    let msg = err.to_string();
    assert!(msg.contains("prev") && msg.contains("delay"), "Expected prev delay error, got: {msg}");
}

#[test]
fn prev_valid_delay_in_new_variants_passes() {
    let module = module_with_properties(vec![
        prop(
            "ok1",
            PropertyFormula::NeverImplies {
                antecedent: gt(prev("sensor", 1), 100),
                consequent: sig("alarm"),
            },
        ),
        prop(
            "ok2",
            PropertyFormula::EventuallyWithin { expr: gt(prev("sensor", 2), 50), cycles: 10 },
        ),
        prop(
            "ok3",
            PropertyFormula::AlwaysFollowedBy {
                trigger: gt(prev("sensor", 3), 100),
                response: sig("alarm"),
                delay_cycles: 5,
            },
        ),
    ]);
    validate_module(&module).expect("Valid prev delays should pass");
}
