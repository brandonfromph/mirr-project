#![forbid(unsafe_code)]
// Integration tests for the MIRR diagnostic rendering engine.
//
// These tests exercise the public API of `nasa_rust_project::diagnostic`
// end-to-end, verifying that rendered output matches the expected
// rustc-style format with headers, source snippets, carets, and labels.

use nasa_rust_project::diagnostic::{render_diagnostic, Diagnostic, Severity};
use nasa_rust_project::span::Span;

// ---------------------------------------------------------------------------
// Helper: multi-line MIRR sample sources
// ---------------------------------------------------------------------------

/// Three-line source used by most tests (lines 0, 1, 2 in 0-based indexing).
const SAMPLE_SOURCE: &str = "\
module Sensor {
    signal temperature: in u16;
    signal pressure: in u8;
}";

/// Source with a duplicate signal on lines 1 and 2 (0-based).
const DUP_SOURCE: &str = "\
module Dup {
    signal temperature: in u8;
    signal temperature: in u16;
}";

// ---------------------------------------------------------------------------
// Test: parse error with span
// ---------------------------------------------------------------------------

#[test]
fn render_parse_error_with_span() {
    // Error on 0-based line 2 ("    signal pressure: in u8;"), columns 11..19
    // which covers "pressure". Display line = 3, display col = 12.
    let diag = Diagnostic::error("unexpected token `u8`")
        .with_code("E100")
        .with_span(Some(Span::single_line(2, 11, 19)));

    let rendered = render_diagnostic(&diag, SAMPLE_SOURCE, "test.mirr");

    // Header: "error[E100]: unexpected token `u8`"
    assert!(rendered.contains("error[E100]:"), "error code header missing:\n{rendered}");
    assert!(rendered.contains("unexpected token `u8`"), "error message missing:\n{rendered}");

    // Location: "--> test.mirr:3:12"
    assert!(rendered.contains("--> test.mirr:3:12"), "location line missing or wrong:\n{rendered}");

    // Source line present
    assert!(rendered.contains("signal pressure: in u8;"), "source line missing:\n{rendered}");

    // Carets: 19 - 11 = 8 carets
    assert!(rendered.contains("^^^^^^^^"), "carets missing:\n{rendered}");
}

// ---------------------------------------------------------------------------
// Test: semantic error with help
// ---------------------------------------------------------------------------

#[test]
fn render_semantic_error_with_help() {
    let diag = Diagnostic::error("duplicate signal name `temperature`")
        .with_code("E201")
        .with_span(Some(Span::single_line(2, 11, 22)))
        .with_help("each signal name must be unique within a module");

    let rendered = render_diagnostic(&diag, DUP_SOURCE, "dup.mirr");

    // Header
    assert!(rendered.contains("error[E201]:"), "error code header missing:\n{rendered}");
    assert!(
        rendered.contains("duplicate signal name `temperature`"),
        "error message missing:\n{rendered}"
    );

    // Help label (without span renders as "= help: ...")
    assert!(
        rendered.contains("= help: each signal name must be unique within a module"),
        "help label missing:\n{rendered}"
    );
}

// ---------------------------------------------------------------------------
// Test: error without span
// ---------------------------------------------------------------------------

#[test]
fn render_error_without_span() {
    let diag = Diagnostic::error("compilation aborted due to previous errors").with_code("E000");

    let rendered = render_diagnostic(&diag, "", "");

    // Header present
    assert!(
        rendered.contains("error[E000]: compilation aborted due to previous errors"),
        "header missing:\n{rendered}"
    );

    // No location arrow
    assert!(!rendered.contains("-->"), "unexpected location line when span is None:\n{rendered}");
}

// ---------------------------------------------------------------------------
// Test: warning severity
// ---------------------------------------------------------------------------

#[test]
fn render_warning_severity() {
    let diag = Diagnostic::warning("unused signal `pressure`")
        .with_code("W001")
        .with_span(Some(Span::single_line(2, 11, 19)));

    let rendered = render_diagnostic(&diag, SAMPLE_SOURCE, "test.mirr");

    // Prefix must be "warning", not "error"
    assert!(rendered.starts_with("warning[W001]:"), "warning prefix missing or wrong:\n{rendered}");
    assert!(!rendered.starts_with("error"), "should not start with 'error':\n{rendered}");
    assert!(rendered.contains("unused signal `pressure`"), "message missing:\n{rendered}");
}

// ---------------------------------------------------------------------------
// Test: note span (secondary label with its own source snippet)
// ---------------------------------------------------------------------------

#[test]
fn render_with_note_span() {
    // Primary error on line 2, note pointing back to line 1 (first definition).
    let diag = Diagnostic::error("duplicate signal name `temperature`")
        .with_code("E201")
        .with_span(Some(Span::single_line(2, 11, 22)))
        .with_note_span(Some(Span::single_line(1, 11, 22)), "first defined here");

    let rendered = render_diagnostic(&diag, DUP_SOURCE, "dup.mirr");

    // Note label text
    assert!(rendered.contains("note: first defined here"), "note label missing:\n{rendered}");

    // Note span should produce its own location line pointing to line 2
    // (0-based line 1 => display line 2)
    assert!(rendered.contains("--> dup.mirr:2:12"), "note span location missing:\n{rendered}");

    // Primary span location (0-based line 2 => display line 3)
    assert!(rendered.contains("--> dup.mirr:3:12"), "primary span location missing:\n{rendered}");
}

// ---------------------------------------------------------------------------
// Test: multiple labels (both note and help)
// ---------------------------------------------------------------------------

#[test]
fn render_multi_label() {
    let diag = Diagnostic::error("duplicate signal name `temperature`")
        .with_code("E201")
        .with_span(Some(Span::single_line(2, 11, 22)))
        .with_note_span(Some(Span::single_line(1, 11, 22)), "first defined here")
        .with_help("each signal name must be unique within a module");

    let rendered = render_diagnostic(&diag, DUP_SOURCE, "dup.mirr");

    // Both note and help should appear
    assert!(rendered.contains("note: first defined here"), "note label missing:\n{rendered}");
    assert!(
        rendered.contains("= help: each signal name must be unique within a module"),
        "help label missing:\n{rendered}"
    );
}

// ---------------------------------------------------------------------------
// Test: full_line span underlines entire content
// ---------------------------------------------------------------------------

#[test]
fn render_full_line_span_underlines_content() {
    // Span::full_line(1) covers 0-based line 1 entirely.
    // Line 1 of SAMPLE_SOURCE is "    signal temperature: in u16;"
    // The trimmed length is 31 characters, so we expect 31 carets.
    let diag = Diagnostic::warning("entire line flagged").with_span(Some(Span::full_line(1)));

    let rendered = render_diagnostic(&diag, SAMPLE_SOURCE, "test.mirr");

    assert!(rendered.contains("warning: entire line flagged"), "header missing:\n{rendered}");

    // full_line => start_col=0, end_col=u32::MAX => underlines up to
    // trimmed trailing whitespace length.
    let line_1 = "    signal temperature: in u16;";
    let expected_caret_count = line_1.trim_end().len(); // 30
    let expected_carets = "^".repeat(expected_caret_count);
    assert!(
        rendered.contains(&expected_carets),
        "full-line carets (expected {expected_caret_count}) missing:\n{rendered}"
    );
}

// ---------------------------------------------------------------------------
// Additional: severity Display trait coverage
// ---------------------------------------------------------------------------

#[test]
fn severity_display_variants() {
    assert_eq!(Severity::Error.to_string(), "error");
    assert_eq!(Severity::Warning.to_string(), "warning");
    assert_eq!(Severity::Info.to_string(), "info");
    assert_eq!(Severity::Help.to_string(), "help");
}
