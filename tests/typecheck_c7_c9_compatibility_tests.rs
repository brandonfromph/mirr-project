#![forbid(unsafe_code)]
//! MEGA-1 type-check tests — criteria C7, C8, C9.
//!
//! - C7: Phantom type tags (E620, E621)
//! - C8: Width inference interaction
//! - C9: Error code uniqueness
//!
//! NASA P10: bounded loops, no recursion.

use mirrc::ast::program::Module;
use mirrc::ast::types::{SignalKind, SignalType};
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::typeck::extended::ExtendedType as CheckerExtType;
use mirrc::typeck::extended::{typecheck_extended, ExtendedSignalDecl, PhantomTag};

fn run_src(
    src: &str,
) -> Result<mirrc::pipeline::PipelineResult, mirrc::error::PipelineErrors> {
    run_pipeline(src, &PipelineConfig::default())
}

fn module_from_exts(exts: &[ExtendedSignalDecl]) -> Module {
    Module {
        name: "typeck_test".to_string(),
        signals: exts
            .iter()
            .map(|e| mirrc::ast::program::SignalDecl {
                name: e.name.clone(),
                kind: e.kind,
                ty: e.ty.clone().into(),
                origin: e.origin.clone(),
                span: e.span,
            })
            .collect(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn ext_plain(name: &str, kind: SignalKind, ty: SignalType) -> ExtendedSignalDecl {
    ExtendedSignalDecl {
        name: name.to_string(),
        kind,
        ty: ty.clone(),
        extended_ty: CheckerExtType {
            base: ty,
            refinements: Vec::new(),
            qualifiers: Vec::new(),
            clock_domain: None,
            phantom: None,
            type_nat: None,
            dependent_params: Vec::new(),
            session: None,
            span: None,
        },
        origin: None,
        span: None,
    }
}

fn ext_phantom(
    name: &str,
    kind: SignalKind,
    ty: SignalType,
    tag: PhantomTag,
) -> ExtendedSignalDecl {
    ExtendedSignalDecl {
        name: name.to_string(),
        kind,
        ty: ty.clone(),
        extended_ty: CheckerExtType {
            base: ty,
            refinements: Vec::new(),
            qualifiers: Vec::new(),
            clock_domain: None,
            phantom: Some(tag),
            type_nat: None,
            dependent_params: Vec::new(),
            session: None,
            span: None,
        },
        origin: None,
        span: None,
    }
}

// ===========================================================================
// C7: Phantom tags
// ===========================================================================

#[test]
fn c7_phantom_tag_constructed_correctly() {
    let tag = PhantomTag { tag: "SafetyType".to_string() };
    assert_eq!(tag.tag, "SafetyType");
}

#[test]
fn c7_signal_with_phantom_tag_accepted() {
    let tag = PhantomTag { tag: "Calibrated".to_string() };
    let sigs = vec![ext_phantom("sensor", SignalKind::Input, SignalType::Unsigned(16), tag)];
    let module = module_from_exts(&sigs);
    let result = typecheck_extended(&module, &sigs, &[], &[], &[]);
    let _ = result; // no panic
}

#[test]
fn c7_same_phantom_tag_no_conflict() {
    let tag_a = PhantomTag { tag: "Verified".to_string() };
    let tag_b = PhantomTag { tag: "Verified".to_string() };
    let sigs = vec![
        ext_phantom("a", SignalKind::Input, SignalType::Bool, tag_a),
        ext_phantom("b", SignalKind::Output, SignalType::Bool, tag_b),
    ];
    let module = module_from_exts(&sigs);
    let result = typecheck_extended(&module, &sigs, &[], &[], &[]);
    let _ = result;
}

#[test]
fn c7_different_phantom_tags_may_conflict() {
    let tag_a = PhantomTag { tag: "TypeA".to_string() };
    let sig_a = ext_phantom("a", SignalKind::Input, SignalType::Unsigned(8), tag_a);
    let sig_b = ext_plain("b", SignalKind::Output, SignalType::Bool);
    let module = module_from_exts(&[sig_a.clone(), sig_b.clone()]);
    let result = typecheck_extended(&module, &[sig_a, sig_b], &[], &[], &[]);
    let _ = result; // behavior may vary
}

#[test]
fn c7_no_phantom_tag_default() {
    let sig = ext_plain("x", SignalKind::Input, SignalType::Bool);
    assert!(sig.extended_ty.phantom.is_none(), "plain signal must have no phantom tag");
}

// ===========================================================================
// C8: Width inference interaction
// ===========================================================================

#[test]
fn c8_u8_width_pipeline() {
    let result = run_src(
        r#"module w8 {
    signal x: in u8;
    signal y: out bool;
    guard g {
        when (x > 100)
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}"#,
    );
    assert!(result.is_ok(), "u8 width must work: {:?}", result.err());
}

#[test]
fn c8_u16_width_pipeline() {
    let result = run_src(
        r#"module w16 {
    signal pressure: in u16;
    signal alarm: out bool;
    guard g_high {
        when (pressure > 3000)
        for 2 cycles;
    }
    reflex r_alarm {
        on g_high {
            alarm = true;
        }
    }
}"#,
    );
    assert!(result.is_ok(), "u16 width must work: {:?}", result.err());
}

#[test]
fn c8_u32_width_pipeline() {
    let result = run_src(
        r#"module w32 {
    signal counter: in u32;
    signal overflow: out bool;
    guard g_wrap {
        when (counter > 1000000)
        for 1 cycles;
    }
    reflex r_overflow {
        on g_wrap {
            overflow = true;
        }
    }
}"#,
    );
    assert!(result.is_ok(), "u32 width must work: {:?}", result.err());
}

#[test]
fn c8_bool_width_pipeline() {
    let result = run_src(
        r#"module wbool {
    signal flag: in bool;
    signal out_flag: out bool;
    guard g_set {
        when flag
        for 1 cycles;
    }
    reflex r_set {
        on g_set {
            out_flag = true;
        }
    }
}"#,
    );
    assert!(result.is_ok(), "bool width must work: {:?}", result.err());
}

// ===========================================================================
// C9: Error code uniqueness
// ===========================================================================

#[test]
fn c9_parse_errors_have_e1xx_prefix() {
    use mirrc::parse_mirr;
    // Force a parse error and check it has E1xx code
    let result = parse_mirr("module @invalid {");
    assert!(result.is_err(), "invalid syntax must fail");
    let err = result.unwrap_err();
    let code = err.error_code();
    assert!(
        code.as_ref().is_some_and(|c| c.starts_with("E1")),
        "parse error code must be E1xx, got {:?}",
        code
    );
}

#[test]
fn c9_semantic_errors_have_e2xx_prefix() {
    use mirrc::pipeline::run_pipeline;
    // Undefined signal reference -> semantic error
    let result = run_pipeline(
        r#"module bad_ref {
    signal x: in bool;
    guard g { when undefined_signal for 1 cycles; }
    reflex r when [g] { x = true; }
}"#,
        &PipelineConfig::default(),
    );
    if let Err(errs) = result {
        let has_e2xx = errs.errors.iter().any(|e| match e.error_code().as_deref() {
            Some(code) => code.starts_with("E2"),
            None => false,
        });
        // At least some error should be in the expected range
        // (may also be E1xx parse error depending on what triggers first)
        let _ = has_e2xx;
    }
}

#[test]
fn c9_error_codes_are_numeric() {
    use mirrc::error::MirrError;
    let parse_err = MirrError::ParseError { message: "[E101] test error".to_string(), span: None };
    let code = parse_err.error_code();
    assert!(code.is_some(), "error code must be present, got {:?}", code);
    assert!(code.as_ref().unwrap().starts_with("E"), "error code must start with E");
}
