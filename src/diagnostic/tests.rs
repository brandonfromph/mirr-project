use super::*;
use crate::span::Span;

#[test]
fn test_render_error_with_span() {
    let source = "module Sensor {\n    signal temperature: in u16;\n}\n";
    let diag = Diagnostic::error("unexpected token")
        .with_code("E101")
        .with_span(Some(Span::single_line(1, 11, 22)));

    let rendered = render_diagnostic(&diag, source, "sensor.mirr");

    assert!(rendered.contains("error[E101]: unexpected token"), "header missing: {rendered}");
    assert!(rendered.contains("@> sensor.mirr:2:12"), "location missing: {rendered}");
    assert!(rendered.contains("2 │     signal temperature: in u16;"), "source line missing: {rendered}");
    assert!(rendered.contains("  │            ^^^^^^^^^^^"), "carets missing: {rendered}");
}

#[test]
fn test_render_error_no_span() {
    let diag = Diagnostic::error("compilation aborted due to previous errors").with_code("E000");

    let rendered = render_diagnostic(&diag, "", "");

    assert!(
        rendered.contains("error[E000]: compilation aborted due to previous errors"),
        "header missing: {rendered}"
    );
    assert!(!rendered.contains("@>"), "unexpected location line: {rendered}");
}

#[test]
fn test_render_with_note_and_help() {
    let source = "module Dup {\n    signal x: in u8;\n    signal x: in u16;\n}\n";

    let diag = Diagnostic::error("duplicate signal name `x`")
        .with_code("E201")
        .with_span(Some(Span::single_line(2, 11, 12)))
        .with_note_span(Some(Span::single_line(1, 11, 12)), "first defined here")
        .with_help("each signal name must be unique within a module");

    let rendered = render_diagnostic(&diag, source, "dup.mirr");

    assert!(
        rendered.contains("error[E201]: duplicate signal name `x`"),
        "header missing: {rendered}"
    );
    assert!(rendered.contains("@> dup.mirr:3:12"), "primary location missing: {rendered}");
    assert!(rendered.contains("note: first defined here"), "note label missing: {rendered}");
    assert!(rendered.contains("@> dup.mirr:2:12"), "note location missing: {rendered}");
    assert!(
        rendered.contains("= help: each signal name must be unique within a module"),
        "help label missing: {rendered}"
    );
}

#[test]
fn test_render_full_line_span() {
    let source = "module Test {\n    signal bad_signal: in u8;\n}\n";

    let diag = Diagnostic::warning("entire line flagged").with_span(Some(Span::full_line(1)));

    let rendered = render_diagnostic(&diag, source, "test.mirr");

    assert!(rendered.contains("warning: entire line flagged"), "header missing: {rendered}");
    let expected_carets = "^".repeat("    signal bad_signal: in u8;".trim_end().len());
    assert!(rendered.contains(&expected_carets), "full-line carets missing: {rendered}");
}

#[test]
fn test_severity_display() {
    assert_eq!(Severity::Error.to_string(), "error");
    assert_eq!(Severity::Warning.to_string(), "warning");
    assert_eq!(Severity::Info.to_string(), "info");
    assert_eq!(Severity::Help.to_string(), "help");
}

#[test]
fn test_line_number_width() {
    assert_eq!(line_number_width(0), 1);
    assert_eq!(line_number_width(1), 1);
    assert_eq!(line_number_width(9), 1);
    assert_eq!(line_number_width(10), 2);
    assert_eq!(line_number_width(99), 2);
    assert_eq!(line_number_width(100), 3);
    assert_eq!(line_number_width(1000), 4);
}

#[test]
fn test_truncate_line() {
    let short = "hello world";
    assert_eq!(truncate_line(short), short);

    let long: String = "x".repeat(MAX_DIAG_LINE_WIDTH + 50);
    let truncated = truncate_line(&long);
    assert_eq!(truncated.len(), MAX_DIAG_LINE_WIDTH);
}

#[test]
fn test_get_source_line() {
    let src = "line0\nline1\nline2\n";
    assert_eq!(get_source_line(src, 0), Some("line0"));
    assert_eq!(get_source_line(src, 1), Some("line1"));
    assert_eq!(get_source_line(src, 2), Some("line2"));
    assert_eq!(get_source_line(src, 3), None);
}

#[test]
fn test_build_caret_line_single() {
    let line = "    signal temperature: in u8;";
    let carets = build_caret_line(line, 11, 22).unwrap();
    assert_eq!(&carets[..11], "           ");
    assert_eq!(&carets[11..], "^^^^^^^^^^^");
}

#[test]
fn test_build_caret_line_full() {
    let line = "    signal temperature: in u8;";
    let carets = build_caret_line(line, 0, u32::MAX).unwrap();
    let expected_count = line.trim_end().len();
    let caret_count = carets.chars().filter(|c| *c == '^').count();
    assert_eq!(caret_count, expected_count);
}

#[test]
fn test_max_labels_bounded() {
    let mut diag = Diagnostic::error("too many labels");
    let mut i: usize = 0;
    while i < MAX_LABELS + 5 {
        diag = diag.with_note(format!("note {i}"));
        i += 1;
    }
    assert_eq!(diag.labels.len(), MAX_LABELS);
}

#[test]
fn test_warning_builder() {
    let diag = Diagnostic::warning("unused signal `x`")
        .with_code("W001")
        .with_span(Some(Span::single_line(0, 0, 5)));

    assert_eq!(diag.severity, Severity::Warning);
    assert_eq!(diag.code.as_deref(), Some("W001"));
    assert_eq!(diag.message, "unused signal `x`");
    assert!(diag.span.is_some());
}
