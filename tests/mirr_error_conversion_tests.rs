#![forbid(unsafe_code)]
//! Unit tests for the central MirrError authority (50 distinct tests).

use nasa_rust_project::error::{MirrError, PipelineErrors, MAX_ACCUMULATED_ERRORS};
use nasa_rust_project::span::Span;

macro_rules! test_err_code {
    ($($name:ident, $err:expr, $expected_code:expr);* $(;)?) => {
        $(
            #[test]
            fn $name() {
                let err = $err;
                assert_eq!(err.error_code().as_deref(), Some($expected_code));
            }
        )*
    };
}

macro_rules! test_err_msg {
    ($($name:ident, $err:expr, $expected_msg:expr);* $(;)?) => {
        $(
            #[test]
            fn $name() {
                let err = $err;
                let diag = err.to_diagnostic();
                assert_eq!(diag.message, $expected_msg);
            }
        )*
    };
}

test_err_code! {
    // --- 15 Error Code Extraction Tests ---
    code_parse, MirrError::ParseError { message: "[E102] bad syntax".to_string(), span: None }, "E102";
    code_semantic, MirrError::SemanticError { message: "[E203] type mismatch".to_string(), span: None }, "E203";
    code_symbol, MirrError::SymbolError { message: "[E204] duplicate".to_string(), span: None }, "E204";
    code_import, MirrError::ImportError { message: "[E802] file not found".to_string(), span: None }, "E802";
    code_temporal, MirrError::TemporalCompilationError { message: "[E302] cycle check".to_string(), span: None }, "E302";
    code_pattern, MirrError::PatternError { message: "[E402] substitution failed".to_string(), span: None }, "E402";
    code_width, MirrError::WidthError { message: "[E502] solver timeout".to_string(), span: None }, "E502";
    code_type, MirrError::TypeError { message: "[E602] generic mismatch".to_string(), span: None }, "E602";
    code_rspu, MirrError::RspuError { message: "[E702] bad instruction".to_string(), span: None }, "E702";
    code_sexpr, MirrError::SExprError { message: "[E803] malformed sexpr".to_string(), span: None }, "E803";
    code_sat, MirrError::SatError { message: "[E902] sat failed".to_string(), span: None }, "E902";
    code_symbolic, MirrError::SymbolicError { message: "[E1002] bad fingerprint".to_string(), span: None }, "E1002";
    code_totality, MirrError::TotalityError { message: "[E1102] partial guard".to_string(), span: None }, "E1102";
    code_tooling, MirrError::ToolingError { message: "[E1202] bad runner".to_string(), span: None }, "E1202";
    code_internal, MirrError::InternalError("[E002] fatal stack crash".to_string()), "E002";
}

test_err_msg! {
    // --- 15 Error Message Stripping Tests ---
    msg_parse, MirrError::ParseError { message: "[E102] bad syntax".to_string(), span: None }, "bad syntax";
    msg_semantic, MirrError::SemanticError { message: "[E203] type mismatch".to_string(), span: None }, "type mismatch";
    msg_symbol, MirrError::SymbolError { message: "[E204] duplicate".to_string(), span: None }, "duplicate";
    msg_import, MirrError::ImportError { message: "[E802] file not found".to_string(), span: None }, "file not found";
    msg_temporal, MirrError::TemporalCompilationError { message: "[E302] cycle check".to_string(), span: None }, "cycle check";
    msg_pattern, MirrError::PatternError { message: "[E402] substitution failed".to_string(), span: None }, "substitution failed";
    msg_width, MirrError::WidthError { message: "[E502] solver timeout".to_string(), span: None }, "solver timeout";
    msg_type, MirrError::TypeError { message: "[E602] generic mismatch".to_string(), span: None }, "generic mismatch";
    msg_rspu, MirrError::RspuError { message: "[E702] bad instruction".to_string(), span: None }, "bad instruction";
    msg_sexpr, MirrError::SExprError { message: "[E803] malformed sexpr".to_string(), span: None }, "malformed sexpr";
    msg_sat, MirrError::SatError { message: "[E902] sat failed".to_string(), span: None }, "sat failed";
    msg_symbolic, MirrError::SymbolicError { message: "[E1002] bad fingerprint".to_string(), span: None }, "bad fingerprint";
    msg_totality, MirrError::TotalityError { message: "[E1102] partial guard".to_string(), span: None }, "partial guard";
    msg_tooling, MirrError::ToolingError { message: "[E1202] bad runner".to_string(), span: None }, "bad runner";
    msg_internal, MirrError::InternalError("[E002] fatal stack crash".to_string()), "fatal stack crash";
}

// --- 10 Edge Cases on Code Extraction & Formatting ---
#[test]
fn test_edge_no_brackets() {
    let err = MirrError::ParseError { message: "plain message".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E100"));
}

#[test]
fn test_edge_short_bracket() {
    let err = MirrError::ParseError { message: "[E] short".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E100"));
}

#[test]
fn test_edge_empty_bracket() {
    let err = MirrError::ParseError { message: "[] empty".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E100"));
}

#[test]
fn test_edge_non_digit_code() {
    let err = MirrError::ParseError { message: "[Eabc] word".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E100"));
}

#[test]
fn test_edge_other_letter_code() {
    let err = MirrError::ParseError { message: "[A100] letter".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E100"));
}

#[test]
fn test_edge_unmatched_bracket() {
    let err = MirrError::ParseError { message: "[E100 unmatched".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E100"));
}

#[test]
fn test_edge_with_span_preserves_variant() {
    let span = Span::full_line(5);
    let err =
        MirrError::ParseError { message: "msg".to_string(), span: None }.with_span(Some(span));
    assert_eq!(err.span(), Some(span));
}

#[test]
fn test_edge_to_string_contains_line() {
    let span = Span::full_line(5);
    let err = MirrError::ParseError { message: "error body".to_string(), span: Some(span) };
    let s = err.to_string();
    assert!(s.contains("line 6"));
}

#[test]
fn test_edge_to_diagnostic_severity() {
    let err = MirrError::ParseError { message: "msg".to_string(), span: None };
    let diag = err.to_diagnostic();
    assert_eq!(diag.severity, nasa_rust_project::diagnostic::Severity::Error);
}

#[test]
fn test_edge_message_accessor() {
    let err = MirrError::ParseError { message: "body".to_string(), span: None };
    assert_eq!(err.message(), "body");
}

// --- 10 PipelineErrors Accumulation & Boundaries ---
#[test]
fn test_pipe_errors_default_empty() {
    let pe = PipelineErrors::default();
    assert!(pe.is_empty());
    assert_eq!(pe.len(), 0);
}

#[test]
fn test_pipe_errors_push_one() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::ParseError { message: "one".to_string(), span: None });
    assert!(!pe.is_empty());
    assert_eq!(pe.len(), 1);
    assert_eq!(pe.first().unwrap().message(), "one");
}

#[test]
fn test_pipe_errors_max_bounds() {
    let mut pe = PipelineErrors::new();
    for i in 0..(MAX_ACCUMULATED_ERRORS + 5) {
        pe.push(MirrError::ParseError { message: i.to_string(), span: None });
    }
    assert_eq!(pe.len(), MAX_ACCUMULATED_ERRORS);
}

#[test]
fn test_pipe_errors_display_singular() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::ParseError { message: "one".to_string(), span: None });
    let s = pe.to_string();
    assert!(s.contains("aborting due to previous error"));
}

#[test]
fn test_pipe_errors_display_plural() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::ParseError { message: "one".to_string(), span: None });
    pe.push(MirrError::ParseError { message: "two".to_string(), span: None });
    let s = pe.to_string();
    assert!(s.contains("aborting due to 2 previous errors"));
}

#[test]
fn test_pipe_errors_from_single() {
    let err = MirrError::ParseError { message: "one".to_string(), span: None };
    let pe = PipelineErrors::from(err);
    assert_eq!(pe.len(), 1);
}

#[test]
fn test_pipe_errors_from_vec() {
    let errs = vec![
        MirrError::ParseError { message: "one".to_string(), span: None },
        MirrError::ParseError { message: "two".to_string(), span: None },
    ];
    let pe = PipelineErrors::from(errs);
    assert_eq!(pe.len(), 2);
}

#[test]
fn test_pipe_errors_to_diagnostics() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::ParseError { message: "one".to_string(), span: None });
    let diags = pe.to_diagnostics();
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].message, "one");
}

#[test]
fn test_pipe_errors_internal_error_code() {
    let err = MirrError::InternalError("fatal".to_string());
    assert_eq!(err.error_code().as_deref(), Some("E000"));
}

#[test]
fn test_pipe_errors_none_span_is_none() {
    let err = MirrError::InternalError("fatal".to_string());
    assert_eq!(err.span(), None);
}
