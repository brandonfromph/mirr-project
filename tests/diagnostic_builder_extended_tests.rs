#![forbid(unsafe_code)]

use mirrc::diagnostic_builder::MirrDiagnostic;
use mirrc::error::MirrError;
use mirrc::error_codes::{mirrcode, ErrorCode};

#[test]
fn diagnostic_with_entity_and_component() {
    let diag = MirrDiagnostic::error(ErrorCode::GuardNameEmpty)
        .with_entity(42)
        .with_component("Guard");
    
    assert_eq!(diag.entity_id, Some(42));
    assert_eq!(diag.component, Some("Guard"));
}

#[test]
fn diagnostic_build_empty_label() {
    let diag = MirrDiagnostic::error(ErrorCode::GuardNameEmpty)
        .with_entity(42)
        .with_component("Guard");
    let err = diag.build();
    assert!(err.message().contains("[E121] (no label set)"));
}

#[test]
fn error_code_pattern_fallback() {
    let err = mirrcode(ErrorCode::PatternFallback, "pattern error");
    assert!(matches!(err, MirrError::PatternError { .. }));
}

#[test]
fn error_code_width_fallback() {
    let err = mirrcode(ErrorCode::WidthFallback, "width error");
    assert!(matches!(err, MirrError::WidthError { .. }));
}

#[test]
fn error_code_type_fallback() {
    let err = mirrcode(ErrorCode::TypeFallback, "type error");
    assert!(matches!(err, MirrError::TypeError { .. }));
}

#[test]
fn error_code_sexpr_fallback() {
    let err = mirrcode(ErrorCode::SExprFallback, "sexpr error");
    assert!(matches!(err, MirrError::SExprError { .. }));
}

#[test]
fn error_code_symbolic_fallback() {
    let err = mirrcode(ErrorCode::SymbolicFallback, "symbolic error");
    assert!(matches!(err, MirrError::SymbolicError { .. }));
}

#[test]
fn error_code_totality_fallback() {
    let err = mirrcode(ErrorCode::TotalityFallback, "totality error");
    assert!(matches!(err, MirrError::TotalityError { .. }));
}
