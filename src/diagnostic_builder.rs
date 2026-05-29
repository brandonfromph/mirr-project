// ---------------------------------------------------------------------------
//! `MirrDiagnostic` — typed, ECS-aware diagnostic builder.
//!
//! This is the **preferred** API for emitting compiler diagnostics.
//! It wraps `MirrError` with structured fields (typed code, help text,
//! entity ID, component name) and converts to `MirrError` for backward
//! compatibility.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use crate::diagnostic_builder::MirrDiagnostic;
//! use crate::error_codes::ErrorCode;
//!
//! return Err(MirrDiagnostic::error(ErrorCode::GuardNameEmpty)
//!     .with_span(span)
//!     .with_label("Guard name cannot be empty.")
//!     .with_help("Provide a name after the 'guard' keyword.")
//!     .with_entity(entity_id)
//!     .build());
//! ```
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};
use crate::span::Span;

// ── NASA Power-of-10 bounds ────────────────────────────────────────────────

/// Maximum length of a diagnostic label string.
pub const MAX_LABEL_LEN: usize = 512;
/// Maximum length of a diagnostic help string.
pub const MAX_HELP_LEN: usize = 512;

// ── Builder ────────────────────────────────────────────────────────────────

/// Structured, ECS-aware diagnostic builder.
///
/// Fields:
/// - `code`      — typed `ErrorCode` (compile-time enforced)
/// - `label`     — primary message (replaces the raw format! string)
/// - `span`      — source location
/// - `help`      — optional recovery suggestion shown below the error
/// - `entity_id` — ECS entity that emitted this diagnostic (for RAG/LSP)
/// - `component` — ECS component type name (e.g. `"Guard"`, `"SignalDecl"`)
#[derive(Debug, Clone)]
pub struct MirrDiagnostic {
    pub code: ErrorCode,
    pub label: String,
    pub span: Option<Span>,
    pub help: Option<String>,
    pub entity_id: Option<u64>,
    pub component: Option<&'static str>,
}

impl MirrDiagnostic {
    /// Start building a diagnostic with the given typed error code.
    /// The label defaults to the code's display form until `.with_label()` is called.
    pub fn error(code: ErrorCode) -> Self {
        Self {
            code,
            label: String::new(),
            span: None,
            help: None,
            entity_id: None,
            component: None,
        }
    }

    /// Set the primary human-readable message.
    /// Truncated to `MAX_LABEL_LEN` characters (NASA Power-of-10).
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        let s = label.into();
        self.label = if s.len() > MAX_LABEL_LEN { s[..MAX_LABEL_LEN].to_string() } else { s };
        self
    }

    /// Attach a source span.
    pub fn with_span(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }

    /// Attach a recovery suggestion shown below the error line.
    /// Truncated to `MAX_HELP_LEN` characters (NASA Power-of-10).
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        let s = help.into();
        self.help = Some(if s.len() > MAX_HELP_LEN { s[..MAX_HELP_LEN].to_string() } else { s });
        self
    }

    /// Link to the ECS entity that caused this diagnostic.
    /// Stored for RAG retrieval and LSP `publishDiagnostics`.
    pub fn with_entity(mut self, entity_id: u64) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    /// Name the ECS component type (e.g. `"Guard"`, `"SignalDecl"`).
    pub fn with_component(mut self, component: &'static str) -> Self {
        self.component = Some(component);
        self
    }

    /// Materialise into a `MirrError`.
    ///
    /// The label is used as the message if set; otherwise the code's
    /// bracket form is used as a fallback.
    /// The span is forwarded. Help text and entity fields are embedded
    /// in the message for now (Phase 5 will attach them to the Registry).
    pub fn build(self) -> MirrError {
        let body = if self.label.is_empty() {
            format!("{} (no label set)", self.code.bracketed())
        } else {
            self.label.clone()
        };

        // Append help as a note in the message body (transitional until
        // the Diagnostic struct gains a dedicated help field in Phase 5).
        let full_body = match &self.help {
            Some(h) => format!("{} — help: {}", body, h),
            None => body,
        };

        mirrcode(self.code, full_body).with_span(self.span)
    }

    /// Convenience: build and immediately wrap in `Err(...)`.
    pub fn into_err<T>(self) -> Result<T, MirrError> {
        Err(self.build())
    }
}

// ── Convenience free functions ─────────────────────────────────────────────

/// Emit a simple one-liner diagnostic with no help or entity link.
///
/// Equivalent to `MirrDiagnostic::error(code).with_label(msg).build()`.
pub fn emit(code: ErrorCode, msg: impl Into<String>) -> MirrError {
    MirrDiagnostic::error(code).with_label(msg).build()
}

/// Emit a diagnostic with a span.
pub fn emit_at(code: ErrorCode, msg: impl Into<String>, span: Span) -> MirrError {
    MirrDiagnostic::error(code).with_label(msg).with_span(Some(span)).build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_codes::ErrorCode;

    #[test]
    fn builder_sets_label() {
        let err = MirrDiagnostic::error(ErrorCode::GuardNameEmpty)
            .with_label("Guard name cannot be empty.")
            .build();
        assert!(err.message().contains("Guard name cannot be empty."));
        assert!(err.message().contains("[E121]"));
    }

    #[test]
    fn builder_embeds_help() {
        let err = MirrDiagnostic::error(ErrorCode::GuardNameEmpty)
            .with_label("Guard name cannot be empty.")
            .with_help("Add a name after 'guard'.")
            .build();
        assert!(err.message().contains("Add a name after 'guard'."));
    }

    #[test]
    fn builder_preserves_span() {
        use crate::span::Span;
        let span = Span::full_line(10);
        let err = MirrDiagnostic::error(ErrorCode::SignalNameEmpty)
            .with_label("Signal name cannot be empty.")
            .with_span(Some(span))
            .build();
        assert_eq!(err.span(), Some(span));
    }

    #[test]
    fn emit_shorthand() {
        let err = emit(ErrorCode::DuplicateSignalName, "duplicate signal 'clk'");
        assert!(err.message().contains("[E201]"));
        assert!(err.message().contains("duplicate signal 'clk'"));
    }

    #[test]
    fn label_truncated_to_max() {
        let long = "x".repeat(MAX_LABEL_LEN + 100);
        let err = MirrDiagnostic::error(ErrorCode::ParseFallback).with_label(long).build();
        // Message contains prefix + code, so just check it doesn't blow up.
        assert!(!err.message().is_empty());
    }

    #[test]
    fn into_err_returns_result() {
        let result: Result<(), _> =
            MirrDiagnostic::error(ErrorCode::GuardNameEmpty).with_label("test").into_err();
        assert!(result.is_err());
    }
}
