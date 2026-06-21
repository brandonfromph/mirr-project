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

pub mod formal_trace;
pub mod vcd_parser;

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

fn get_source_line(source: &str, line_idx: u32) -> Option<&str> {
    source.lines().nth(line_idx as usize)
}

fn truncate_line(line: &str) -> &str {
    if line.len() <= MAX_DIAG_LINE_WIDTH {
        line
    } else {
        let mut end = MAX_DIAG_LINE_WIDTH;
        let mut guard = 0;
        while end > 0 && !line.is_char_boundary(end) && guard < 4 {
            end -= 1;
            guard += 1;
        }
        &line[..end]
    }
}

fn line_number_width(line_num: usize) -> usize {
    if line_num == 0 {
        return 1;
    }
    let mut n = line_num;
    let mut digits: usize = 0;
    while n > 0 && digits < 10 {
        n /= 10;
        digits += 1;
    }
    digits
}

fn build_caret_line(line_text: &str, start_col: u32, end_col: u32) -> Option<String> {
    let line_len = line_text.len();
    let effective_start = start_col as usize;

    let effective_end = if end_col == u32::MAX {
        line_text.trim_end().len()
    } else {
        let e = end_col as usize;
        if e > MAX_DIAG_LINE_WIDTH {
            MAX_DIAG_LINE_WIDTH
        } else {
            e
        }
    };

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

fn render_snippet(
    out: &mut String,
    source: &str,
    file_path: &str,
    span: &Span,
    gutter_width: usize,
    message: &str,
) {
    let display_line = span.start_line as usize + 1;
    let display_col = span.start_col as usize + 1;

    let arrow_pad = gutter_width;
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

    let raw_line = match get_source_line(source, span.start_line) {
        Some(l) => l,
        None => return,
    };
    let line_text = truncate_line(raw_line);

    let mut blank_gutter = String::with_capacity(gutter_width + 2);
    p = 0;
    while p < gutter_width + 1 && p < MAX_DIAG_LINE_WIDTH {
        blank_gutter.push(' ');
        p += 1;
    }
    blank_gutter.push('|');

    out.push_str(&blank_gutter);
    out.push('\n');

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

fn compute_max_line_number(diag: &Diagnostic) -> usize {
    let mut max_line: usize = 0;

    if let Some(ref span) = diag.span {
        let display = span.start_line as usize + 1;
        if display > max_line {
            max_line = display;
        }
    }

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
pub fn render_diagnostic(diag: &Diagnostic, source: &str, file_path: &str) -> String {
    let mut out = String::with_capacity(256);

    out.push_str(&diag.severity.to_string());
    if let Some(ref code) = diag.code {
        out.push('[');
        out.push_str(code);
        out.push(']');
    }
    out.push_str(": ");
    out.push_str(&diag.message);
    out.push('\n');

    let max_line = compute_max_line_number(diag);
    let gutter_width = if max_line > 0 { line_number_width(max_line) } else { 1 };

    if let Some(ref span) = diag.span {
        render_snippet(&mut out, source, file_path, span, gutter_width, "");
    }

    let mut i: usize = 0;
    while i < diag.labels.len() && i < MAX_LABELS {
        let label = &diag.labels[i];

        let prefix = match label.kind {
            LabelKind::Note => "note",
            LabelKind::Help => "help",
        };

        if let Some(ref span) = label.span {
            let mut blank_gutter = String::with_capacity(gutter_width + 2);
            let mut p: usize = 0;
            while p < gutter_width + 1 && p < MAX_DIAG_LINE_WIDTH {
                blank_gutter.push(' ');
                p += 1;
            }
            blank_gutter.push('|');
            out.push_str(&blank_gutter);
            out.push('\n');

            out.push_str(prefix);
            out.push_str(": ");
            out.push_str(&label.message);
            out.push('\n');

            render_snippet(&mut out, source, file_path, span, gutter_width, "");
        } else {
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

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
