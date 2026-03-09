//! Central diagnostic rendering engine for the MIRR compiler.
//!
//! Provides the [`Diagnostic`] struct and [`render_diagnostic`] function that
//! produce rustc-style terminal output with source snippets, carets, and
//! help/note labels.
//!
//! Design constraints (NASA Power-of-10):
//! - No `unsafe` code (`#![forbid(unsafe_code)]`).
//! - No recursion.
//! - All iteration is bounded by named constants.
//! - Zero new external dependencies (only `std` and `crate::span::Span`).

#![forbid(unsafe_code)]

use crate::span::Span;
use std::fmt;

/// Maximum width of a source line in the rendered output.
const MAX_DIAG_LINE_WIDTH: usize = 200;

/// Maximum number of secondary labels.
const MAX_LABELS: usize = 8;

/// Severity level for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Help,
}

/// A secondary label attached to a diagnostic.
#[derive(Debug, Clone)]
pub struct Label {
    pub span: Option<Span>,
    pub message: String,
    pub kind: LabelKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelKind {
    Note,
    Help,
}

/// A structured diagnostic that can render to rustc-style terminal output.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<String>,
    pub message: String,
    pub span: Option<Span>,
    pub labels: Vec<Label>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            code: None,
            span: None,
            labels: Vec::new(),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            code: None,
            span: None,
            labels: Vec::new(),
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_span(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }

    pub fn with_label(mut self, label: Label) -> Self {
        if self.labels.len() < MAX_LABELS {
            self.labels.push(label);
        }
        self
    }

    pub fn with_help(self, message: impl Into<String>) -> Self {
        self.with_label(Label { span: None, message: message.into(), kind: LabelKind::Help })
    }

    pub fn with_note(self, message: impl Into<String>) -> Self {
        self.with_label(Label { span: None, message: message.into(), kind: LabelKind::Note })
    }

    pub fn with_note_span(self, span: Option<Span>, message: impl Into<String>) -> Self {
        self.with_label(Label { span, message: message.into(), kind: LabelKind::Note })
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
            Severity::Help => write!(f, "help"),
        }
    }
}

// ---------------------------------------------------------------------------
// Source-line helpers (no recursion, bounded iteration)
// ---------------------------------------------------------------------------

/// Retrieve the `n`th line (0-based) from `source`, or `None` if out of bounds.
///
/// Uses `lines()` with an explicit counter bounded by the total number of
/// lines (which is itself bounded by source length).
fn get_source_line(source: &str, line_idx: u32) -> Option<&str> {
    source.lines().nth(line_idx as usize)
}

/// Truncate a source line to at most `MAX_DIAG_LINE_WIDTH` characters.
fn truncate_line(line: &str) -> &str {
    if line.len() <= MAX_DIAG_LINE_WIDTH {
        line
    } else {
        // Find a char boundary at or before MAX_DIAG_LINE_WIDTH.
        let mut end = MAX_DIAG_LINE_WIDTH;
        // Bounded: at most 4 iterations (UTF-8 char max width).
        let mut guard = 0;
        while end > 0 && !line.is_char_boundary(end) && guard < 4 {
            end -= 1;
            guard += 1;
        }
        &line[..end]
    }
}

/// Calculate the display width (number of decimal digits) of a line number.
fn line_number_width(line_num: usize) -> usize {
    if line_num == 0 {
        return 1;
    }
    let mut n = line_num;
    let mut digits: usize = 0;
    // Bounded: at most 10 iterations (u32::MAX has 10 digits).
    while n > 0 && digits < 10 {
        n /= 10;
        digits += 1;
    }
    digits
}

/// Build a caret line (`^^^`) for the span within the given source line.
///
/// Returns `None` if carets cannot be placed (e.g., start_col past end of line).
fn build_caret_line(line_text: &str, start_col: u32, end_col: u32) -> Option<String> {
    let line_len = line_text.len();

    // Determine the effective start and end columns.
    let effective_start = start_col as usize;

    let effective_end = if end_col == u32::MAX {
        // Full-line span: underline up to trimmed trailing whitespace.
        line_text.trim_end().len()
    } else {
        let e = end_col as usize;
        if e > MAX_DIAG_LINE_WIDTH {
            MAX_DIAG_LINE_WIDTH
        } else {
            e
        }
    };

    // Skip carets if start is past the line content.
    if effective_start >= line_len && line_len > 0 {
        return None;
    }
    if effective_end <= effective_start {
        return None;
    }

    let caret_count = effective_end - effective_start;
    if caret_count == 0 {
        return None;
    }

    let mut carets = String::with_capacity(effective_start + caret_count);
    // Bounded: at most MAX_DIAG_LINE_WIDTH iterations.
    let mut i: usize = 0;
    while i < effective_start && i < MAX_DIAG_LINE_WIDTH {
        carets.push(' ');
        i += 1;
    }
    i = 0;
    while i < caret_count && i < MAX_DIAG_LINE_WIDTH {
        carets.push('^');
        i += 1;
    }
    Some(carets)
}

// ---------------------------------------------------------------------------
// Snippet rendering helpers
// ---------------------------------------------------------------------------

/// Render a source snippet block for a given span.
///
/// Writes to `out`:
/// ```text
///  --> file.mirr:5:5
///   |
/// 5 |     signal temperature: in u16;
///   |            ^^^^^^^^^^^ optional message
/// ```
///
/// `gutter_width` is the number of characters reserved for the line-number
/// gutter (determined by the caller to keep multiple snippets aligned).
fn render_snippet(
    out: &mut String,
    source: &str,
    file_path: &str,
    span: &Span,
    gutter_width: usize,
    message: &str,
) {
    let display_line = span.start_line as usize + 1; // 1-based for display
    let display_col = span.start_col as usize + 1; // 1-based for display

    // Location line: " --> file:line:col"
    // Pad the arrow to align with the gutter.
    let arrow_pad = gutter_width; // spaces before "-->"
    let mut pad_str = String::with_capacity(arrow_pad);
    let mut p: usize = 0;
    while p < arrow_pad && p < MAX_DIAG_LINE_WIDTH {
        pad_str.push(' ');
        p += 1;
    }
    out.push_str(&pad_str);
    out.push_str("--> ");
    out.push_str(file_path);
    out.push(':');
    out.push_str(&display_line.to_string());
    out.push(':');
    out.push_str(&display_col.to_string());
    out.push('\n');

    // Fetch the source line.
    let raw_line = match get_source_line(source, span.start_line) {
        Some(l) => l,
        None => return, // Line out of bounds — skip snippet.
    };
    let line_text = truncate_line(raw_line);

    // Blank gutter separator: "   |"
    let mut blank_gutter = String::with_capacity(gutter_width + 2);
    p = 0;
    while p < gutter_width + 1 && p < MAX_DIAG_LINE_WIDTH {
        blank_gutter.push(' ');
        p += 1;
    }
    blank_gutter.push('|');

    out.push_str(&blank_gutter);
    out.push('\n');

    // Source line: "5 |     signal temperature: in u16;"
    // Right-align the line number within gutter_width characters.
    let num_str = display_line.to_string();
    let num_pad = if gutter_width > num_str.len() { gutter_width - num_str.len() } else { 0 };
    p = 0;
    while p < num_pad && p < MAX_DIAG_LINE_WIDTH {
        out.push(' ');
        p += 1;
    }
    out.push_str(&num_str);
    out.push_str(" | ");
    out.push_str(line_text);
    out.push('\n');

    // Caret line: "   |            ^^^^^^^^^^^ message"
    if let Some(carets) = build_caret_line(line_text, span.start_col, span.end_col) {
        out.push_str(&blank_gutter);
        out.push(' ');
        out.push_str(&carets);
        if !message.is_empty() {
            out.push(' ');
            out.push_str(message);
        }
        out.push('\n');
    }
}

/// Determine the maximum line number that will appear in the rendered output.
///
/// This drives the gutter width so all snippets align consistently.
fn compute_max_line_number(diag: &Diagnostic) -> usize {
    let mut max_line: usize = 0;

    if let Some(ref span) = diag.span {
        let display = span.start_line as usize + 1;
        if display > max_line {
            max_line = display;
        }
    }

    // Bounded: at most MAX_LABELS iterations.
    let mut i: usize = 0;
    while i < diag.labels.len() && i < MAX_LABELS {
        if let Some(ref span) = diag.labels[i].span {
            let display = span.start_line as usize + 1;
            if display > max_line {
                max_line = display;
            }
        }
        i += 1;
    }

    max_line
}

// ---------------------------------------------------------------------------
// Public rendering API
// ---------------------------------------------------------------------------

/// Render a diagnostic with source snippet in rustc style.
///
/// Output format:
/// ```text
/// error[E201]: duplicate signal name `temperature`
///  --> file.mirr:5:5
///   |
/// 5 |     signal temperature: in u16;
///   |            ^^^^^^^^^^^ `temperature` is already declared
///   |
/// note: first defined here
///  --> file.mirr:3:5
///   |
/// 3 |     signal temperature: in u8;
///   |            ^^^^^^^^^^^
///   = help: each signal name must be unique within a module
/// ```
pub fn render_diagnostic(diag: &Diagnostic, source: &str, file_path: &str) -> String {
    let mut out = String::with_capacity(256);

    // 1. Header line: "error[E201]: message"
    out.push_str(&diag.severity.to_string());
    if let Some(ref code) = diag.code {
        out.push('[');
        out.push_str(code);
        out.push(']');
    }
    out.push_str(": ");
    out.push_str(&diag.message);
    out.push('\n');

    // Compute gutter width from the largest line number that will be shown.
    let max_line = compute_max_line_number(diag);
    let gutter_width = if max_line > 0 { line_number_width(max_line) } else { 1 };

    // 2. Primary span snippet.
    if let Some(ref span) = diag.span {
        render_snippet(&mut out, source, file_path, span, gutter_width, "");
    }

    // 3. Secondary labels (bounded by MAX_LABELS).
    let mut i: usize = 0;
    while i < diag.labels.len() && i < MAX_LABELS {
        let label = &diag.labels[i];

        let prefix = match label.kind {
            LabelKind::Note => "note",
            LabelKind::Help => "help",
        };

        if let Some(ref span) = label.span {
            // Label with span: render a full snippet block with a prefix header.

            // Blank gutter separator before the label header.
            let mut blank_gutter = String::with_capacity(gutter_width + 2);
            let mut p: usize = 0;
            while p < gutter_width + 1 && p < MAX_DIAG_LINE_WIDTH {
                blank_gutter.push(' ');
                p += 1;
            }
            blank_gutter.push('|');
            out.push_str(&blank_gutter);
            out.push('\n');

            // Label kind header: "note: message" or "help: message"
            out.push_str(prefix);
            out.push_str(": ");
            out.push_str(&label.message);
            out.push('\n');

            render_snippet(&mut out, source, file_path, span, gutter_width, "");
        } else {
            // Label without span: " = help: message"
            let mut eq_pad = String::with_capacity(gutter_width + 2);
            let mut p: usize = 0;
            while p < gutter_width + 1 && p < MAX_DIAG_LINE_WIDTH {
                eq_pad.push(' ');
                p += 1;
            }
            out.push_str(&eq_pad);
            out.push_str("= ");
            out.push_str(prefix);
            out.push_str(": ");
            out.push_str(&label.message);
            out.push('\n');
        }

        i += 1;
    }

    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::Span;

    #[test]
    fn test_render_error_with_span() {
        let source = "module Sensor {\n    signal temperature: in u16;\n}\n";
        let diag = Diagnostic::error("unexpected token")
            .with_code("E101")
            .with_span(Some(Span::single_line(1, 11, 22)));

        let rendered = render_diagnostic(&diag, source, "sensor.mirr");

        // Header present.
        assert!(rendered.contains("error[E101]: unexpected token"), "header missing: {rendered}");
        // Location line (1-based: line 2, col 12).
        assert!(rendered.contains("--> sensor.mirr:2:12"), "location missing: {rendered}");
        // Source line displayed.
        assert!(
            rendered.contains("signal temperature: in u16;"),
            "source line missing: {rendered}"
        );
        // Carets present: 22 - 11 = 11 carets.
        assert!(rendered.contains("^^^^^^^^^^^"), "carets missing: {rendered}");
    }

    #[test]
    fn test_render_error_no_span() {
        let diag =
            Diagnostic::error("compilation aborted due to previous errors").with_code("E000");

        let rendered = render_diagnostic(&diag, "", "");

        assert!(
            rendered.contains("error[E000]: compilation aborted due to previous errors"),
            "header missing: {rendered}"
        );
        // No arrow / location line.
        assert!(!rendered.contains("-->"), "unexpected location line: {rendered}");
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

        // Header.
        assert!(
            rendered.contains("error[E201]: duplicate signal name `x`"),
            "header missing: {rendered}"
        );
        // Primary span points to line 3 (0-based line 2).
        assert!(rendered.contains("--> dup.mirr:3:12"), "primary location missing: {rendered}");
        // Note label with span.
        assert!(rendered.contains("note: first defined here"), "note label missing: {rendered}");
        // Note span points to line 2 (0-based line 1).
        assert!(rendered.contains("--> dup.mirr:2:12"), "note location missing: {rendered}");
        // Help label without span.
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
        // full_line(1) => start_col=0, end_col=u32::MAX
        // The trimmed line "    signal bad_signal: in u8;" has length 29.
        // So we expect 29 carets.
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
        // 11 spaces then 11 carets.
        assert_eq!(&carets[..11], "           ");
        assert_eq!(&carets[11..], "^^^^^^^^^^^");
    }

    #[test]
    fn test_build_caret_line_full() {
        let line = "    signal temperature: in u8;";
        let carets = build_caret_line(line, 0, u32::MAX).unwrap();
        // Full line trimmed is 29 chars, so 29 carets.
        let expected_count = line.trim_end().len();
        let caret_count = carets.chars().filter(|c| *c == '^').count();
        assert_eq!(caret_count, expected_count);
    }

    #[test]
    fn test_max_labels_bounded() {
        let mut diag = Diagnostic::error("too many labels");
        // Try to add more than MAX_LABELS.
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
}
