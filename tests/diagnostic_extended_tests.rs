#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
//! Extended integration tests for `src/diagnostic.rs` — diagnostic creation,
//! formatting, error-code rendering, multi-error accumulation, and edge cases.
//!
//! NASA Power-of-10 compliant: bounded iteration, no recursion, descriptive asserts.

use mirrc::diagnostic::{render_diagnostic, Diagnostic, LabelKind, Severity};
use mirrc::error::{MirrError, PipelineErrors, MAX_ACCUMULATED_ERRORS};
use mirrc::span::Span;
use mirrc::width::types::{DiagSeverity, WidthDiag};

/// Maximum iterations for bounded test loops (NASA Power-of-10 rule #2).
const MAX_TEST_ITEMS: usize = 64;

/// Maximum labels used in label-saturation tests.
const MAX_TEST_LABELS: usize = 16;

/// Maximum errors pushed in accumulation stress tests.
const MAX_TEST_ERRORS: usize = 30;

// ===========================================================================
// 1. DIAGNOSTIC CREATION — builder API
// ===========================================================================

#[test]
fn diagnostic_error_builder_sets_severity() {
    let diag = Diagnostic::error("something went wrong");
    assert_eq!(diag.severity, Severity::Error, "Diagnostic::error() should set severity to Error");
    assert_eq!(
        diag.message, "something went wrong",
        "Diagnostic::error() should store the message verbatim"
    );
    assert!(diag.code.is_none(), "Diagnostic::error() should leave code as None by default");
    assert!(diag.span.is_none(), "Diagnostic::error() should leave span as None by default");
    assert!(diag.labels.is_empty(), "Diagnostic::error() should start with empty labels");
}

#[test]
fn diagnostic_warning_builder_sets_severity() {
    let diag = Diagnostic::warning("unused signal");
    assert_eq!(
        diag.severity,
        Severity::Warning,
        "Diagnostic::warning() should set severity to Warning"
    );
    assert_eq!(
        diag.message, "unused signal",
        "Diagnostic::warning() should store the message verbatim"
    );
}

#[test]
fn diagnostic_with_code_attaches_code() {
    let diag = Diagnostic::error("parse failure").with_code("E101");
    assert_eq!(diag.code.as_deref(), Some("E101"), "with_code should attach the error code string");
}

#[test]
fn diagnostic_with_span_attaches_span() {
    let span = Span::single_line(3, 5, 12);
    let diag = Diagnostic::error("bad token").with_span(Some(span));
    assert!(diag.span.is_some(), "with_span(Some(..)) should attach a span");
    let s = diag.span.unwrap();
    assert_eq!(s.start_line, 3, "span start_line should be preserved");
    assert_eq!(s.start_col, 5, "span start_col should be preserved");
    assert_eq!(s.end_col, 12, "span end_col should be preserved");
}

#[test]
fn diagnostic_with_span_none_leaves_none() {
    let diag = Diagnostic::error("no location").with_span(None);
    assert!(diag.span.is_none(), "with_span(None) should leave span as None");
}

#[test]
fn diagnostic_with_help_adds_help_label() {
    let diag = Diagnostic::error("problem").with_help("try this instead");
    assert_eq!(diag.labels.len(), 1, "with_help should add exactly one label");
    assert_eq!(diag.labels[0].kind, LabelKind::Help, "with_help should produce a Help-kind label");
    assert_eq!(
        diag.labels[0].message, "try this instead",
        "help label should carry the provided message"
    );
    assert!(diag.labels[0].span.is_none(), "with_help should produce a label without a span");
}

#[test]
fn diagnostic_with_note_adds_note_label() {
    let diag = Diagnostic::error("duplicate").with_note("first defined here");
    assert_eq!(diag.labels.len(), 1, "with_note should add exactly one label");
    assert_eq!(diag.labels[0].kind, LabelKind::Note, "with_note should produce a Note-kind label");
    assert_eq!(
        diag.labels[0].message, "first defined here",
        "note label should carry the provided message"
    );
}

#[test]
fn diagnostic_with_note_span_adds_spanned_note() {
    let span = Span::single_line(1, 0, 10);
    let diag = Diagnostic::error("conflict").with_note_span(Some(span), "originally declared here");
    assert_eq!(diag.labels.len(), 1, "with_note_span should add one label");
    assert_eq!(diag.labels[0].kind, LabelKind::Note, "with_note_span should produce a Note label");
    assert!(
        diag.labels[0].span.is_some(),
        "with_note_span(Some(..)) should attach a span to the label"
    );
}

#[test]
fn diagnostic_chained_builders() {
    let diag = Diagnostic::error("duplicate signal `x`")
        .with_code("E201")
        .with_span(Some(Span::single_line(4, 11, 12)))
        .with_note("first defined on line 2")
        .with_help("rename one of the signals");
    assert_eq!(diag.severity, Severity::Error, "chained builder: severity");
    assert_eq!(diag.code.as_deref(), Some("E201"), "chained builder: code");
    assert!(diag.span.is_some(), "chained builder: span present");
    assert_eq!(diag.labels.len(), 2, "chained builder: should have note + help = 2 labels");
    assert_eq!(diag.labels[0].kind, LabelKind::Note, "chained builder: first label is note");
    assert_eq!(diag.labels[1].kind, LabelKind::Help, "chained builder: second label is help");
}

// ===========================================================================
// 2. SEVERITY DISPLAY TRAIT
// ===========================================================================

#[test]
fn severity_display_error() {
    assert_eq!(Severity::Error.to_string(), "error", "Severity::Error should display as 'error'");
}

#[test]
fn severity_display_warning() {
    assert_eq!(
        Severity::Warning.to_string(),
        "warning",
        "Severity::Warning should display as 'warning'"
    );
}

#[test]
fn severity_display_info() {
    assert_eq!(Severity::Info.to_string(), "info", "Severity::Info should display as 'info'");
}

#[test]
fn severity_display_help() {
    assert_eq!(Severity::Help.to_string(), "help", "Severity::Help should display as 'help'");
}

// ===========================================================================
// 3. RENDER DIAGNOSTIC — error code display for all categories
// ===========================================================================

#[test]
fn render_e1xx_parse_error_header() {
    let diag = Diagnostic::error("unexpected token `@`").with_code("E101");
    let rendered = render_diagnostic(&diag, "", "input.mirr");
    assert!(
        rendered.contains("error[E101]: unexpected token `@`"),
        "E1xx parse error header should include code and message: {rendered}"
    );
}

#[test]
fn render_e2xx_semantic_error_header() {
    let diag = Diagnostic::error("duplicate signal name `temperature`").with_code("E201");
    let rendered = render_diagnostic(&diag, "", "sensor.mirr");
    assert!(
        rendered.contains("error[E201]: duplicate signal name `temperature`"),
        "E2xx semantic error header should include code and message: {rendered}"
    );
}

#[test]
fn render_e3xx_temporal_error_header() {
    let diag = Diagnostic::error("cyclic temporal dependency").with_code("E301");
    let rendered = render_diagnostic(&diag, "", "guards.mirr");
    assert!(
        rendered.contains("error[E301]: cyclic temporal dependency"),
        "E3xx temporal error header should include code and message: {rendered}"
    );
}

#[test]
fn render_e4xx_pattern_error_header() {
    let diag = Diagnostic::error("pattern arity mismatch").with_code("E401");
    let rendered = render_diagnostic(&diag, "", "patterns.mirr");
    assert!(
        rendered.contains("error[E401]: pattern arity mismatch"),
        "E4xx pattern error header should include code and message: {rendered}"
    );
}

#[test]
fn render_e5xx_width_error_header() {
    let diag = Diagnostic::error("width overflow exceeds 64 bits").with_code("E504");
    let rendered = render_diagnostic(&diag, "", "arith.mirr");
    assert!(
        rendered.contains("error[E504]: width overflow exceeds 64 bits"),
        "E5xx width error header should include code and message: {rendered}"
    );
}

#[test]
fn render_e6xx_type_error_header() {
    let diag = Diagnostic::error("signedness mismatch in comparison").with_code("E601");
    let rendered = render_diagnostic(&diag, "", "types.mirr");
    assert!(
        rendered.contains("error[E601]: signedness mismatch in comparison"),
        "E6xx type error header should include code and message: {rendered}"
    );
}

#[test]
fn render_e7xx_rspu_error_header() {
    let diag = Diagnostic::error("register allocation failed").with_code("E701");
    let rendered = render_diagnostic(&diag, "", "codegen.mirr");
    assert!(
        rendered.contains("error[E701]: register allocation failed"),
        "E7xx R-SPU error header should include code and message: {rendered}"
    );
}

#[test]
fn render_e8xx_sexpr_error_header() {
    let diag = Diagnostic::error("malformed s-expression").with_code("E801");
    let rendered = render_diagnostic(&diag, "", "ir.sexpr");
    assert!(
        rendered.contains("error[E801]: malformed s-expression"),
        "E8xx S-expression error header should include code and message: {rendered}"
    );
}

// ===========================================================================
// 4. RENDER DIAGNOSTIC — span information
// ===========================================================================

#[test]
fn render_with_span_shows_location_line() {
    let source = "module Test {\n    signal x: in u8;\n}\n";
    let diag = Diagnostic::error("issue here")
        .with_code("E100")
        .with_span(Some(Span::single_line(1, 4, 10)));
    let rendered = render_diagnostic(&diag, source, "test.mirr");
    assert!(
        rendered.contains("--> test.mirr:2:5"),
        "location line should show 1-based line:col (line 2, col 5): {rendered}"
    );
}

#[test]
fn render_with_span_shows_source_line() {
    let source = "module Sensor {\n    signal temperature: in u16;\n}\n";
    let diag = Diagnostic::error("flagged").with_span(Some(Span::single_line(1, 11, 22)));
    let rendered = render_diagnostic(&diag, source, "sensor.mirr");
    assert!(
        rendered.contains("signal temperature: in u16;"),
        "rendered output should contain the source line text: {rendered}"
    );
}

#[test]
fn render_with_span_shows_caret_underline() {
    let source = "module M {\n    signal abc: in u8;\n}\n";
    // Underline "abc" at columns 11..14
    let diag = Diagnostic::error("bad name").with_span(Some(Span::single_line(1, 11, 14)));
    let rendered = render_diagnostic(&diag, source, "m.mirr");
    assert!(
        rendered.contains("^^^"),
        "rendered output should contain caret underlines: {rendered}"
    );
}

#[test]
fn render_full_line_span_underlines_entire_line() {
    let source = "module M {\n    signal x: in u32;\n}\n";
    let diag = Diagnostic::warning("line flagged").with_span(Some(Span::full_line(1)));
    let rendered = render_diagnostic(&diag, source, "m.mirr");
    // full_line uses end_col = u32::MAX, so carets cover the trimmed line length
    let line_text = "    signal x: in u32;";
    let expected_caret_count = line_text.trim_end().len();
    let caret_str: String = "^".repeat(expected_caret_count);
    assert!(
        rendered.contains(&caret_str),
        "full-line span should underline the entire trimmed line ({expected_caret_count} carets): {rendered}"
    );
}

#[test]
fn render_span_at_line_zero() {
    let source = "module First {\n}\n";
    let diag = Diagnostic::error("first line error").with_span(Some(Span::single_line(0, 0, 6)));
    let rendered = render_diagnostic(&diag, source, "first.mirr");
    assert!(
        rendered.contains("--> first.mirr:1:1"),
        "line 0 col 0 should display as 1:1: {rendered}"
    );
    assert!(rendered.contains("module"), "should show the first source line: {rendered}");
}

// ===========================================================================
// 5. RENDER DIAGNOSTIC — labels (note and help)
// ===========================================================================

#[test]
fn render_note_label_without_span() {
    let diag = Diagnostic::error("problem occurred")
        .with_code("E200")
        .with_note("additional context here");
    let rendered = render_diagnostic(&diag, "", "file.mirr");
    assert!(
        rendered.contains("= note: additional context here"),
        "note label without span should render with '= note:' prefix: {rendered}"
    );
}

#[test]
fn render_help_label_without_span() {
    let diag =
        Diagnostic::error("issue").with_code("E200").with_help("consider using a different name");
    let rendered = render_diagnostic(&diag, "", "file.mirr");
    assert!(
        rendered.contains("= help: consider using a different name"),
        "help label without span should render with '= help:' prefix: {rendered}"
    );
}

#[test]
fn render_note_label_with_span() {
    let source = "module Dup {\n    signal a: in u8;\n    signal a: in u16;\n}\n";
    let diag = Diagnostic::error("duplicate signal `a`")
        .with_code("E201")
        .with_span(Some(Span::single_line(2, 11, 12)))
        .with_note_span(Some(Span::single_line(1, 11, 12)), "first defined here");
    let rendered = render_diagnostic(&diag, source, "dup.mirr");
    assert!(
        rendered.contains("note: first defined here"),
        "note label with span should include 'note:' header: {rendered}"
    );
    assert!(
        rendered.contains("--> dup.mirr:2:12"),
        "note span should point to the original definition location: {rendered}"
    );
}

#[test]
fn render_multiple_labels() {
    let diag = Diagnostic::error("multiple issues")
        .with_note("first note")
        .with_note("second note")
        .with_help("a suggestion");
    let rendered = render_diagnostic(&diag, "", "multi.mirr");
    assert!(rendered.contains("= note: first note"), "first note should be present: {rendered}");
    assert!(rendered.contains("= note: second note"), "second note should be present: {rendered}");
    assert!(rendered.contains("= help: a suggestion"), "help label should be present: {rendered}");
}

// ===========================================================================
// 6. LABEL SATURATION — bounded by MAX_LABELS (8)
// ===========================================================================

#[test]
fn labels_bounded_at_max_labels() {
    let mut diag = Diagnostic::error("too many labels");
    let mut i: usize = 0;
    while i < MAX_TEST_LABELS {
        diag = diag.with_note(format!("label {i}"));
        i += 1;
    }
    // MAX_LABELS is 8 (from diagnostic.rs)
    assert_eq!(
        diag.labels.len(),
        8,
        "labels should be capped at MAX_LABELS (8), even after adding {MAX_TEST_LABELS}"
    );
}

#[test]
fn labels_at_exactly_max_labels_all_preserved() {
    let mut diag = Diagnostic::error("exactly max");
    let mut i: usize = 0;
    while i < 8 {
        diag = diag.with_note(format!("note {i}"));
        i += 1;
    }
    assert_eq!(
        diag.labels.len(),
        8,
        "adding exactly MAX_LABELS labels should preserve all of them"
    );
    // Verify first and last
    assert_eq!(diag.labels[0].message, "note 0", "first label should be 'note 0'");
    assert_eq!(diag.labels[7].message, "note 7", "last label should be 'note 7'");
}

// ===========================================================================
// 7. MIRR ERROR — Display trait and error code extraction
// ===========================================================================

#[test]
fn mirr_error_parse_display_includes_e100() {
    let err = MirrError::ParseError { message: "unexpected end of input".to_string(), span: None };
    let display = err.to_string();
    assert!(display.contains("[E100]"), "ParseError Display should include [E100]: {display}");
    assert!(
        display.contains("Parse error:"),
        "ParseError Display should include 'Parse error:': {display}"
    );
    assert!(
        display.contains("unexpected end of input"),
        "ParseError Display should include the message body: {display}"
    );
}

#[test]
fn mirr_error_parse_display_with_span_shows_line() {
    let err = MirrError::ParseError {
        message: "bad token".to_string(),
        span: Some(Span::single_line(4, 0, 5)),
    };
    let display = err.to_string();
    assert!(
        display.contains("(line 5)"),
        "ParseError with span should show 1-based line number: {display}"
    );
}

#[test]
fn mirr_error_semantic_display_no_code_prefix() {
    let err = MirrError::SemanticError { message: "undefined signal `x`".to_string(), span: None };
    let display = err.to_string();
    assert!(
        display.contains("Semantic error:"),
        "SemanticError Display should include 'Semantic error:': {display}"
    );
    // SemanticError without embedded code falls back to [E200]
    assert!(
        display.contains("[E200]"),
        "SemanticError without embedded code should use [E200] fallback: {display}"
    );
}

#[test]
fn mirr_error_temporal_display_includes_e300() {
    let err = MirrError::TemporalCompilationError {
        message: "guard cycle detected".to_string(),
        span: Some(Span::full_line(10)),
    };
    let display = err.to_string();
    assert!(
        display.contains("[E300]"),
        "TemporalCompilationError Display should include [E300]: {display}"
    );
    assert!(display.contains("(line 11)"), "should show 1-based line number: {display}");
}

#[test]
fn mirr_error_pattern_display_includes_e400() {
    let err = MirrError::PatternError { message: "arity mismatch".to_string(), span: None };
    let display = err.to_string();
    assert!(display.contains("[E400]"), "PatternError Display should include [E400]: {display}");
}

#[test]
fn mirr_error_type_display_no_code_prefix() {
    let err = MirrError::TypeError { message: "signedness conflict".to_string(), span: None };
    let display = err.to_string();
    assert!(
        display.contains("Type error:"),
        "TypeError Display should include 'Type error:': {display}"
    );
}

#[test]
fn mirr_error_rspu_display_includes_e700() {
    let err = MirrError::RspuError { message: "register spill".to_string(), span: None };
    let display = err.to_string();
    assert!(display.contains("[E700]"), "RspuError Display should include [E700]: {display}");
}

#[test]
fn mirr_error_sexpr_display_includes_e800() {
    let err = MirrError::SExprError { message: "unbalanced parentheses".to_string(), span: None };
    let display = err.to_string();
    assert!(display.contains("[E800]"), "SExprError Display should include [E800]: {display}");
}

// ===========================================================================
// 8. MIRR ERROR — to_diagnostic conversion
// ===========================================================================

#[test]
fn to_diagnostic_parse_uses_fallback_e100() {
    let err = MirrError::ParseError { message: "unexpected token".to_string(), span: None };
    let diag = err.to_diagnostic();
    assert_eq!(
        diag.code.as_deref(),
        Some("E100"),
        "ParseError without embedded code should get fallback E100"
    );
    assert_eq!(diag.message, "unexpected token", "to_diagnostic should preserve the message body");
    assert_eq!(
        diag.severity,
        Severity::Error,
        "to_diagnostic should always produce Error severity"
    );
}

#[test]
fn to_diagnostic_semantic_with_embedded_code() {
    let err = MirrError::SemanticError {
        message: "[E201] duplicate signal name `x`".to_string(),
        span: Some(Span::full_line(5)),
    };
    let diag = err.to_diagnostic();
    assert_eq!(
        diag.code.as_deref(),
        Some("E201"),
        "SemanticError with embedded [E201] should extract it"
    );
    assert_eq!(
        diag.message, "duplicate signal name `x`",
        "to_diagnostic should strip the [E201] prefix from the message"
    );
    assert!(diag.span.is_some(), "to_diagnostic should preserve the span");
}

#[test]
fn to_diagnostic_semantic_without_embedded_code() {
    let err = MirrError::SemanticError { message: "unknown issue".to_string(), span: None };
    let diag = err.to_diagnostic();
    assert_eq!(
        diag.code.as_deref(),
        Some("E200"),
        "SemanticError without embedded code should fall back to E200"
    );
}

#[test]
fn to_diagnostic_rspu_with_embedded_code() {
    let err = MirrError::RspuError {
        message: "[E701] register allocation failed".to_string(),
        span: None,
    };
    let diag = err.to_diagnostic();
    assert_eq!(
        diag.code.as_deref(),
        Some("E701"),
        "RspuError with [E701] should extract the embedded code"
    );
    assert_eq!(
        diag.message, "register allocation failed",
        "to_diagnostic should strip the code prefix"
    );
}

#[test]
fn to_diagnostic_preserves_span() {
    let span = Span::single_line(7, 3, 20);
    let err = MirrError::ParseError { message: "bad token".to_string(), span: Some(span) };
    let diag = err.to_diagnostic();
    let diag_span = diag.span.expect("diagnostic should have a span");
    assert_eq!(diag_span.start_line, 7, "diagnostic span start_line should match error span");
    assert_eq!(diag_span.start_col, 3, "diagnostic span start_col should match error span");
    assert_eq!(diag_span.end_col, 20, "diagnostic span end_col should match error span");
}

// ===========================================================================
// 9. MIRR ERROR — error_code method
// ===========================================================================

#[test]
fn error_code_parse_fallback() {
    let err = MirrError::ParseError { message: "plain message".to_string(), span: None };
    assert_eq!(
        err.error_code().as_deref(),
        Some("E100"),
        "ParseError without embedded code should fall back to E100"
    );
}

#[test]
fn error_code_temporal_fallback() {
    let err = MirrError::TemporalCompilationError {
        message: "generic temporal issue".to_string(),
        span: None,
    };
    assert_eq!(
        err.error_code().as_deref(),
        Some("E300"),
        "TemporalCompilationError should fall back to E300"
    );
}

#[test]
fn error_code_pattern_fallback() {
    let err = MirrError::PatternError { message: "arity mismatch".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E400"), "PatternError should fall back to E400");
}

#[test]
fn error_code_rspu_fallback() {
    let err = MirrError::RspuError { message: "generic R-SPU issue".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E700"), "RspuError should fall back to E700");
}

#[test]
fn error_code_sexpr_fallback() {
    let err = MirrError::SExprError { message: "malformed".to_string(), span: None };
    assert_eq!(err.error_code().as_deref(), Some("E800"), "SExprError should fall back to E800");
}

#[test]
fn error_code_semantic_no_fallback() {
    let err = MirrError::SemanticError { message: "no embedded code".to_string(), span: None };
    assert_eq!(
        err.error_code().as_deref(),
        Some("E200"),
        "SemanticError without embedded code should fall back to E200"
    );
}

#[test]
fn error_code_type_no_fallback() {
    let err = MirrError::TypeError { message: "no embedded code".to_string(), span: None };
    assert_eq!(
        err.error_code().as_deref(),
        Some("E600"),
        "TypeError without embedded code should fall back to E600"
    );
}

// ===========================================================================
// 10. PIPELINE ERRORS — multi-error accumulation
// ===========================================================================

#[test]
fn pipeline_errors_new_is_empty() {
    let pe = PipelineErrors::new();
    assert!(pe.is_empty(), "PipelineErrors::new() should be empty");
    assert_eq!(pe.len(), 0, "PipelineErrors::new() should have length 0");
    assert!(pe.first().is_none(), "PipelineErrors::new().first() should be None");
}

#[test]
fn pipeline_errors_push_accumulates() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::ParseError { message: "err 1".to_string(), span: None });
    pe.push(MirrError::SemanticError { message: "err 2".to_string(), span: None });
    assert_eq!(pe.len(), 2, "should accumulate two errors");
    assert!(!pe.is_empty(), "should not be empty after pushing");
    assert!(pe.first().is_some(), "first() should return Some after pushing");
}

#[test]
fn pipeline_errors_bounded_at_max() {
    let mut pe = PipelineErrors::new();
    let mut i: usize = 0;
    while i < MAX_TEST_ERRORS {
        pe.push(MirrError::ParseError { message: format!("error {i}"), span: None });
        i += 1;
    }
    assert_eq!(
        pe.len(),
        MAX_ACCUMULATED_ERRORS,
        "PipelineErrors should cap at MAX_ACCUMULATED_ERRORS ({MAX_ACCUMULATED_ERRORS})"
    );
}

#[test]
fn pipeline_errors_from_single_mirr_error() {
    let err = MirrError::ParseError { message: "lone error".to_string(), span: None };
    let pe = PipelineErrors::from(err);
    assert_eq!(pe.len(), 1, "PipelineErrors::from(MirrError) should contain one error");
}

#[test]
fn pipeline_errors_from_vec() {
    let errs = vec![
        MirrError::ParseError { message: "a".to_string(), span: None },
        MirrError::SemanticError { message: "b".to_string(), span: None },
    ];
    let pe = PipelineErrors::from(errs);
    assert_eq!(pe.len(), 2, "PipelineErrors::from(Vec) should contain all errors");
}

#[test]
fn pipeline_errors_default_is_empty() {
    let pe = PipelineErrors::default();
    assert!(pe.is_empty(), "PipelineErrors::default() should be empty");
}

// ===========================================================================
// 11. PIPELINE ERRORS — Display trait
// ===========================================================================

#[test]
fn pipeline_errors_display_single() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::ParseError { message: "unexpected EOF".to_string(), span: None });
    let display = pe.to_string();
    assert!(
        display.contains("[E100] Parse error: unexpected EOF"),
        "single-error display should contain the formatted error: {display}"
    );
    assert!(
        display.contains("aborting due to previous error"),
        "single-error display should use singular 'error': {display}"
    );
}

#[test]
fn pipeline_errors_display_multiple() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::ParseError { message: "err 1".to_string(), span: None });
    pe.push(MirrError::SemanticError { message: "err 2".to_string(), span: None });
    pe.push(MirrError::TypeError { message: "err 3".to_string(), span: None });
    let display = pe.to_string();
    assert!(
        display.contains("aborting due to 3 previous errors"),
        "multi-error display should use plural count: {display}"
    );
}

#[test]
fn pipeline_errors_to_diagnostics() {
    let mut pe = PipelineErrors::new();
    pe.push(MirrError::ParseError { message: "parse fail".to_string(), span: None });
    pe.push(MirrError::SemanticError {
        message: "[E202] redefined module".to_string(),
        span: Some(Span::full_line(3)),
    });
    let diags = pe.to_diagnostics();
    assert_eq!(diags.len(), 2, "to_diagnostics should produce one Diagnostic per error");
    assert_eq!(
        diags[0].code.as_deref(),
        Some("E100"),
        "first diagnostic should have parse fallback code"
    );
    assert_eq!(
        diags[1].code.as_deref(),
        Some("E202"),
        "second diagnostic should extract embedded code"
    );
}

// ===========================================================================
// 12. RENDER DIAGNOSTIC — edge cases
// ===========================================================================

#[test]
fn render_no_span_no_code() {
    let diag = Diagnostic::error("generic error");
    let rendered = render_diagnostic(&diag, "", "");
    assert!(
        rendered.contains("error: generic error"),
        "error without code should render as 'error: message': {rendered}"
    );
    assert!(!rendered.contains("["), "error without code should not have brackets: {rendered}");
    assert!(
        !rendered.contains("-->"),
        "error without span should not have location arrow: {rendered}"
    );
}

#[test]
fn render_warning_without_code() {
    let diag = Diagnostic::warning("something suspicious");
    let rendered = render_diagnostic(&diag, "", "warn.mirr");
    assert!(
        rendered.starts_with("warning: something suspicious"),
        "warning without code should render cleanly: {rendered}"
    );
}

#[test]
fn render_empty_source_with_span_skips_snippet() {
    // Span references line 5 but source is empty — snippet should be skipped gracefully
    let diag = Diagnostic::error("out of bounds").with_span(Some(Span::single_line(5, 0, 10)));
    let rendered = render_diagnostic(&diag, "", "empty.mirr");
    assert!(rendered.contains("error: out of bounds"), "header should still render: {rendered}");
    // The location arrow should still appear
    assert!(
        rendered.contains("--> empty.mirr:6:1"),
        "location line should still render even with empty source: {rendered}"
    );
}

#[test]
fn render_diagnostic_multiline_source() {
    let source = "line one\nline two\nline three\nline four\nline five\n";
    let diag = Diagnostic::error("issue on line four").with_span(Some(Span::single_line(3, 0, 9)));
    let rendered = render_diagnostic(&diag, source, "multi.mirr");
    assert!(rendered.contains("line four"), "should display the correct source line: {rendered}");
    assert!(
        rendered.contains("--> multi.mirr:4:1"),
        "should reference line 4 (1-based): {rendered}"
    );
}

// ===========================================================================
// 13. WIDTH DIAGNOSTIC — WidthDiag formatting
// ===========================================================================

#[test]
fn width_diag_error_builder() {
    let d = WidthDiag::error("overflow detected");
    assert_eq!(d.severity, DiagSeverity::Error, "WidthDiag::error should set severity to Error");
    assert_eq!(d.message, "overflow detected", "WidthDiag::error should store message");
}

#[test]
fn width_diag_error_has_error_severity() {
    let d = WidthDiag::error("possible truncation");
    assert_eq!(d.severity, DiagSeverity::Error, "WidthDiag::error should set severity to Error");
}

#[test]
fn width_diag_info_builder() {
    let d = WidthDiag::info("inferred width u16");
    assert_eq!(d.severity, DiagSeverity::Info, "WidthDiag::info should set severity to Info");
}

#[test]
fn width_diag_with_code_chain() {
    let d = WidthDiag::error("problem").with_code("E503");
    assert_eq!(d.code.as_deref(), Some("E503"), "with_code should attach the error code");
}

#[test]
fn width_diag_with_signal_chain() {
    let d = WidthDiag::error("unresolved").with_signal("my_signal");
    assert_eq!(
        d.signal_name.as_deref(),
        Some("my_signal"),
        "with_signal should attach the signal name"
    );
}

#[test]
fn width_diag_with_help_chain() {
    let d = WidthDiag::info("narrow").with_help("consider widening the type");
    assert_eq!(
        d.help.as_deref(),
        Some("consider widening the type"),
        "with_help should attach the help text"
    );
}

#[test]
fn width_diag_display_with_code() {
    let d = WidthDiag::error("width overflow").with_code("E504");
    let formatted = format!("{d}");
    assert!(
        formatted.contains("[width:error E504]"),
        "display should include severity and code: {formatted}"
    );
    assert!(
        formatted.contains("width overflow"),
        "display should include the message: {formatted}"
    );
}

#[test]
fn width_diag_display_without_code() {
    let d = WidthDiag::info("informational");
    let formatted = format!("{d}");
    assert!(
        formatted.contains("[width:info]"),
        "display without code should have severity tag only: {formatted}"
    );
    assert!(formatted.contains("informational"), "display should include message: {formatted}");
}

// ===========================================================================
// 14. DIAGNOSTIC RENDERING — integration with MirrError::to_diagnostic
// ===========================================================================

#[test]
fn render_diagnostic_from_mirr_error_parse() {
    let source = "module Bad {\n    signal @invalid: in u8;\n}\n";
    let err = MirrError::ParseError {
        message: "unexpected character `@`".to_string(),
        span: Some(Span::single_line(1, 11, 12)),
    };
    let diag = err.to_diagnostic();
    let rendered = render_diagnostic(&diag, source, "bad.mirr");
    assert!(
        rendered.contains("error[E100]"),
        "rendered parse error should include fallback E100: {rendered}"
    );
    assert!(
        rendered.contains("unexpected character `@`"),
        "rendered should include the message: {rendered}"
    );
    assert!(
        rendered.contains("--> bad.mirr:2:12"),
        "rendered should include the location: {rendered}"
    );
}

#[test]
fn render_diagnostic_from_mirr_error_semantic() {
    let source = "module Dup {\n    signal x: in u8;\n    signal x: out u16;\n}\n";
    let err = MirrError::SemanticError {
        message: "[E201] duplicate signal name `x`".to_string(),
        span: Some(Span::single_line(2, 11, 12)),
    };
    let diag = err.to_diagnostic();
    let rendered = render_diagnostic(&diag, source, "dup.mirr");
    assert!(
        rendered.contains("error[E201]"),
        "rendered semantic error should include extracted E201: {rendered}"
    );
    assert!(
        rendered.contains("duplicate signal name `x`"),
        "rendered should include the cleaned message: {rendered}"
    );
}

// ===========================================================================
// 15. PARSER-TRIGGERED DIAGNOSTICS — invalid inputs
// ===========================================================================

#[test]
fn parse_empty_input_produces_error() {
    let result = mirrc::parse_mirr("");
    assert!(result.is_err(), "parsing empty input should produce an error");
    let err = result.unwrap_err();
    let display = err.to_string();
    assert!(!display.is_empty(), "error from empty parse should have a non-empty display");
}

#[test]
fn parse_garbage_input_produces_parse_error() {
    let result = mirrc::parse_mirr("@#$%^&*");
    assert!(result.is_err(), "parsing garbage input should produce an error");
    let err = result.unwrap_err();
    let display = err.to_string();
    assert!(
        display.contains("Parse error") || display.contains("[E1"),
        "garbage input should trigger a parse error: {display}"
    );
}

#[test]
fn parse_incomplete_module_produces_error() {
    let result = mirrc::parse_mirr("module Incomplete {");
    assert!(result.is_err(), "parsing incomplete module should produce an error");
}

// ===========================================================================
// 16. MIRR ERROR — with_span builder
// ===========================================================================

#[test]
fn mirr_error_with_span_preserves_variant() {
    let span = Span::single_line(2, 0, 10);
    let err =
        MirrError::SemanticError { message: "test".to_string(), span: None }.with_span(Some(span));
    // After with_span, should still be a SemanticError
    assert_eq!(err.message(), "test", "with_span should preserve the message");
    assert!(err.span().is_some(), "with_span(Some(..)) should attach the span");
    assert_eq!(
        err.span().unwrap().start_line,
        2,
        "attached span should have the correct start_line"
    );
}

#[test]
fn mirr_error_message_accessor() {
    let err = MirrError::PatternError {
        message: "arity mismatch: expected 3, got 2".to_string(),
        span: None,
    };
    assert_eq!(
        err.message(),
        "arity mismatch: expected 3, got 2",
        "message() should return the inner message string"
    );
}

#[test]
fn mirr_error_span_accessor_none() {
    let err = MirrError::TypeError { message: "type issue".to_string(), span: None };
    assert!(err.span().is_none(), "span() should return None when no span is attached");
}

// ===========================================================================
// 17. RENDER — gutter alignment with large line numbers
// ===========================================================================

#[test]
fn render_gutter_alignment_double_digit_lines() {
    // Source with enough lines that the line number has 2 digits
    let mut source = String::new();
    let mut i: usize = 0;
    while i < MAX_TEST_ITEMS.min(15) {
        source.push_str(&format!("line {i}\n"));
        i += 1;
    }
    let diag = Diagnostic::error("issue on line 12").with_span(Some(Span::single_line(11, 0, 7)));
    let rendered = render_diagnostic(&diag, &source, "big.mirr");
    assert!(rendered.contains("--> big.mirr:12:1"), "should reference line 12: {rendered}");
    assert!(rendered.contains("12 |"), "gutter should show line number 12: {rendered}");
}

// ===========================================================================
// 18. MIRR ERROR — std::error::Error impl
// ===========================================================================

#[test]
fn mirr_error_is_std_error() {
    let err = MirrError::ParseError { message: "test".to_string(), span: None };
    // Verify it implements std::error::Error by using it as a trait object
    let boxed: Box<dyn std::error::Error> = Box::new(err);
    assert!(
        !boxed.to_string().is_empty(),
        "MirrError should implement std::error::Error with non-empty display"
    );
}

#[test]
fn pipeline_errors_is_std_error() {
    let pe = PipelineErrors::new();
    let boxed: Box<dyn std::error::Error> = Box::new(pe);
    assert!(
        boxed.to_string().contains("error"),
        "PipelineErrors should implement std::error::Error"
    );
}

// ===========================================================================
// 19. MIRR ERROR — Clone
// ===========================================================================

#[test]
fn mirr_error_clone_preserves_all_fields() {
    let original = MirrError::SemanticError {
        message: "[E201] dup".to_string(),
        span: Some(Span::single_line(3, 5, 10)),
    };
    let cloned = original.clone();
    assert_eq!(cloned.message(), original.message(), "cloned message should match original");
    assert_eq!(cloned.span(), original.span(), "cloned span should match original");
    assert_eq!(
        cloned.error_code(),
        original.error_code(),
        "cloned error_code should match original"
    );
}
