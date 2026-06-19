#![cfg(any())]
#![forbid(unsafe_code)]
//! ERR-002: Multi-error accumulation tests.
//!
//! Verifies that the compiler reports multiple errors per compilation
//! instead of stopping at the first.

use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::ast::SignalDecl;
use mirrc::error::{MirrError, PipelineErrors, MAX_ACCUMULATED_ERRORS};
use mirrc::parser::parse_mirr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::validation::validate_module;

fn default_config() -> PipelineConfig {
    PipelineConfig {
        typecheck: true,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    }
}

/// Helper: run pipeline and expect errors.
fn pipeline_errs(src: &str) -> PipelineErrors {
    match run_pipeline(src, &default_config()) {
        Err(e) => e,
        Ok(_) => panic!("expected pipeline to fail"),
    }
}

// ---------------------------------------------------------------------------
// Semantic error accumulation
// ---------------------------------------------------------------------------

#[test]
fn two_duplicate_signals_reported() {
    let src = "\
module m {
    signal x: in bool;
    signal x: in bool;
    signal y: out u8;
    signal y: out u8;
}
";
    let program = parse_mirr(src).unwrap();
    let errs = validate_module(&program.module).unwrap_err();
    assert!(errs.errors.len() >= 2, "expected at least 2 errors, got {}", errs.errors.len());
    for e in &errs.errors {
        assert!(e.to_string().contains("E201"), "expected E201, got: {e}");
    }
}

#[test]
fn duplicate_signal_and_undeclared_ref() {
    let src = "\
module m {
    signal x: in bool;
    signal x: in bool;
    guard g {
        when zzzz
        for 1 cycles;
    }
}
";
    let program = parse_mirr(src).unwrap();
    let errs = validate_module(&program.module).unwrap_err();
    assert!(errs.errors.len() >= 2, "expected at least 2 errors, got {}", errs.errors.len());
    let codes: Vec<String> = errs.errors.iter().filter_map(|e| e.error_code()).collect();
    assert!(codes.contains(&"E201".to_string()), "missing E201 in {codes:?}");
    assert!(codes.contains(&"E204".to_string()), "missing E204 in {codes:?}");
}

#[test]
fn undeclared_signal_dedup() {
    // Same undeclared signal "xyz" referenced in 3 different guards.
    // Should only report 1 E204 error, not 3.
    let src = "\
module m {
    signal a: in bool;
    guard g1 {
        when xyz
        for 1 cycles;
    }
    guard g2 {
        when xyz
        for 2 cycles;
    }
    guard g3 {
        when xyz
        for 3 cycles;
    }
}
";
    let program = parse_mirr(src).unwrap();
    let errs = validate_module(&program.module).unwrap_err();
    let e204_count = errs
        .errors
        .iter()
        .filter(|e| e.error_code().as_deref() == Some("E204") && e.to_string().contains("'xyz'"))
        .count();
    assert_eq!(e204_count, 1, "undeclared signal 'xyz' should be reported once, got {e204_count}");
}

#[test]
fn max_errors_bounded() {
    // Create a module with 25 duplicate signals — should cap at MAX_ACCUMULATED_ERRORS.
    let mut signals = String::new();
    for i in 0..25 {
        signals.push_str("    signal dup: in u8;\n");
        if i == 0 {
            // First one is the original; rest are duplicates.
            continue;
        }
    }
    let src = format!("module m {{\n{}}}\n", signals);
    let program = parse_mirr(&src).unwrap();
    let errs = validate_module(&program.module).unwrap_err();
    assert_eq!(
        errs.errors.len(),
        MAX_ACCUMULATED_ERRORS,
        "expected exactly {} errors (capped), got {}",
        MAX_ACCUMULATED_ERRORS,
        errs.errors.len()
    );
}

#[test]
fn valid_module_no_errors() {
    let src = "\
module m {
    signal x: in bool;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}
";
    let result = run_pipeline(src, &default_config());
    assert!(result.is_ok(), "valid module should not produce errors: {:?}", result.err());
}

#[test]
fn single_error_still_works() {
    let src = "\
module m {
    signal x: in bool;
    signal x: in bool;
}
";
    let program = parse_mirr(src).unwrap();
    let errs = validate_module(&program.module).unwrap_err();
    assert_eq!(errs.errors.len(), 1, "expected 1 error, got {}", errs.errors.len());
    assert!(errs.errors[0].to_string().contains("E201"));
}

#[test]
fn parse_error_wraps_to_pipeline_errors() {
    let errs = pipeline_errs("this is not valid mirr at all");
    assert_eq!(errs.errors.len(), 1, "parse errors produce exactly 1 error");
    assert!(matches!(errs.errors[0], MirrError::ParseError { .. }));
}

// ---------------------------------------------------------------------------
// Type error accumulation
// ---------------------------------------------------------------------------

#[test]
fn type_errors_accumulate() {
    // Two guards with non-bool conditions — both should be reported.
    let src = "\
module m {
    signal a: in u8;
    signal b: in u8;
    signal out: out bool;
    guard g1 {
        when a
        for 1 cycles;
    }
    guard g2 {
        when b
        for 1 cycles;
    }
    reflex r {
        on g1 {
            out = true;
        }
    }
}
";
    let errs = pipeline_errs(src);
    let e601_count =
        errs.errors.iter().filter(|e| e.error_code().as_deref() == Some("E601")).count();
    assert!(e601_count >= 2, "expected at least 2 E601 errors, got {e601_count}");
}

// ---------------------------------------------------------------------------
// Pass gating
// ---------------------------------------------------------------------------

#[test]
fn semantic_errors_gate_typeck() {
    // Undeclared signal is a semantic error — typeck should be skipped,
    // so no type errors should appear.
    let src = "\
module m {
    signal x: in bool;
    guard g {
        when nonexistent
        for 1 cycles;
    }
}
";
    let errs = pipeline_errs(src);
    // Should only have semantic errors (E2xx), no type errors (E6xx).
    for e in &errs.errors {
        let code = e.error_code().unwrap_or_default();
        assert!(code.starts_with("E2"), "expected only semantic errors, got {code}: {e}");
    }
}

// ---------------------------------------------------------------------------
// Multi-writer ownership accumulation
// ---------------------------------------------------------------------------

#[test]
fn multi_writer_signals_reported() {
    let src = "\
module m {
    signal x: in bool;
    signal out1: out u8;
    signal out2: out u8;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r1 {
        on g {
            out1 = 1;
            out2 = 1;
        }
    }
    reflex r2 {
        on g {
            out1 = 2;
            out2 = 2;
        }
    }
}
";
    let program = parse_mirr(src).unwrap();
    let errs = validate_module(&program.module).unwrap_err();
    let e216_count = errs.errors.iter().filter(|e| e.to_string().contains("E216")).count();
    assert!(e216_count >= 2, "expected at least 2 E216 errors, got {e216_count}");
}

// ---------------------------------------------------------------------------
// Property error accumulation
// ---------------------------------------------------------------------------

#[test]
fn property_errors_accumulated() {
    let src = "\
module m {
    signal x: in bool;
    property p1 {
        always (nonexistent1);
    }
    property p2 {
        always (nonexistent2);
    }
}
";
    let program = parse_mirr(src).unwrap();
    let errs = validate_module(&program.module).unwrap_err();
    let e211_count = errs.errors.iter().filter(|e| e.to_string().contains("E211")).count();
    assert!(e211_count >= 2, "expected at least 2 E211 errors, got {e211_count}");
}

// ---------------------------------------------------------------------------
// PipelineErrors container
// ---------------------------------------------------------------------------

#[test]
fn pipeline_errors_display() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::SemanticError {
        message: "[E201] Duplicate signal name: 'x'.".to_string(),
        span: None,
    });
    pe.push(MirrError::SemanticError {
        message: "[E204] Guard 'g' references undeclared signal 'y'.".to_string(),
        span: None,
    });
    let display = pe.to_string();
    assert!(display.contains("E201"), "should contain first error");
    assert!(display.contains("E204"), "should contain second error");
    assert!(display.contains("aborting due to 2 previous errors"), "should have footer");
}

#[test]
fn pipeline_errors_from_single() {
    let e = MirrError::ParseError { message: "test".to_string(), span: None };
    let pe = PipelineErrors::from(e);
    assert_eq!(pe.len(), 1);
    assert!(!pe.is_empty());
    assert!(pe.first().is_some());
}

// ---------------------------------------------------------------------------
// Pattern error accumulation
// ---------------------------------------------------------------------------

#[test]
fn pattern_errors_accumulate() {
    use mirrc::ast::macro_nodes::ModuleMacroStmt;
    use mirrc::ast::pattern::{PatternDef, ReflectBlock};
    use mirrc::validation::validate_pattern_defs;

    let defs = vec![
        PatternDef {
            name: "dup".to_string(),
            params: vec![],
            body: ReflectBlock {
                statements: vec![ModuleMacroStmt::Signal(SignalDecl {
                    name: "dummy".to_string(),
                    kind: SignalKind::Internal,
                    ty: ExtendedType::new(SignalType::Bool, Default::default()),
                    origin: None,
                    span: None,
                })],
            },
            span: None,
        },
        PatternDef {
            name: "dup".to_string(),
            params: vec![],
            body: ReflectBlock {
                statements: vec![ModuleMacroStmt::Signal(SignalDecl {
                    name: "dummy".to_string(),
                    kind: SignalKind::Internal,
                    ty: ExtendedType::new(SignalType::Bool, Default::default()),
                    origin: None,
                    span: None,
                })],
            },
            span: None,
        },
    ];
    let errs = validate_pattern_defs(&defs).unwrap_err();
    assert!(!errs.errors.is_empty(), "expected pattern duplicate error");
    assert!(errs.errors[0].to_string().contains("Duplicate pattern"));
}
