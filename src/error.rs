// ---------------------------------------------------------------------------
//! Centralized error authority for the MIRR compiler.
//!
//! All error variants are catalogued here — NASA/JPL rule for
//! mission-critical diagnostics traceability.
//!
//! ## Error Code Scheme
//!
//! | Prefix | Range     | Category                |
//! |--------|-----------|-------------------------|
//! | E1xx   | 100–199   | Parse errors            |
//! | E2xx   | 200–299   | Semantic errors         |
//! | E3xx   | 300–399   | Temporal errors         |
//! | E4xx   | 400–499   | Pattern errors          |
//! | E5xx   | 500–599   | Width inference errors   |
//! | E6xx   | 600–699   | Type errors              |
//! | E7xx   | 700–799   | R-SPU emission errors    |
//! | E8xx   | 800–899   | S-expression errors      |
//! | E9xx   | 900–999   | SAT solver errors        |
//! | E10xx  | 1000–1099 | Symbolic analysis errors |
//! | E11xx  | 1100–1199 | Totality errors          |
//!
//! See `docs/error_codes.md` for the full catalogue.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;

use crate::diagnostic::{Diagnostic, Severity};
use crate::span::Span;

#[derive(Debug, Clone)]
pub enum MirrError {
    /// Parse/lexical error (E1xx).
    ParseError { message: String, span: Option<Span> },
    /// Semantic analysis error (E2xx).
    SemanticError { message: String, span: Option<Span> },
    /// Temporal compilation error (E3xx).
    TemporalCompilationError { message: String, span: Option<Span> },
    /// Pattern expansion error (E4xx).
    PatternError { message: String, span: Option<Span> },
    /// Type checking error (E6xx).
    TypeError { message: String, span: Option<Span> },
    /// R-SPU emission error (E7xx).
    RspuError { message: String, span: Option<Span> },
    /// S-expression error (E8xx).
    SExprError { message: String, span: Option<Span> },
    /// SAT solver error (E9xx).
    SatError { message: String, span: Option<Span> },
    /// Symbolic analysis error (E10xx).
    SymbolicError { message: String, span: Option<Span> },
    /// Totality error (E11xx).
    TotalityError { message: String, span: Option<Span> },
}

impl MirrError {
    /// Convenience constructor for spanless parse errors.
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::ParseError { message: message.into(), span: None }
    }

    /// Attach a source span to this error (builder pattern).
    pub fn with_span(self, span: Option<Span>) -> Self {
        match self {
            Self::ParseError { message, .. } => Self::ParseError { message, span },
            Self::SemanticError { message, .. } => Self::SemanticError { message, span },
            Self::TemporalCompilationError { message, .. } => {
                Self::TemporalCompilationError { message, span }
            }
            Self::PatternError { message, .. } => Self::PatternError { message, span },
            Self::TypeError { message, .. } => Self::TypeError { message, span },
            Self::RspuError { message, .. } => Self::RspuError { message, span },
            Self::SExprError { message, .. } => Self::SExprError { message, span },
            Self::SatError { message, .. } => Self::SatError { message, span },
            Self::SymbolicError { message, .. } => Self::SymbolicError { message, span },
            Self::TotalityError { message, .. } => Self::TotalityError { message, span },
        }
    }

    /// Extract the source span, if any.
    pub fn span(&self) -> Option<Span> {
        match self {
            Self::ParseError { span, .. }
            | Self::SemanticError { span, .. }
            | Self::TemporalCompilationError { span, .. }
            | Self::PatternError { span, .. }
            | Self::TypeError { span, .. }
            | Self::RspuError { span, .. }
            | Self::SExprError { span, .. }
            | Self::SatError { span, .. }
            | Self::SymbolicError { span, .. }
            | Self::TotalityError { span, .. } => *span,
        }
    }

    /// Extract the error message body (without variant prefix).
    pub fn message(&self) -> &str {
        match self {
            Self::ParseError { message, .. }
            | Self::SemanticError { message, .. }
            | Self::TemporalCompilationError { message, .. }
            | Self::PatternError { message, .. }
            | Self::TypeError { message, .. }
            | Self::RspuError { message, .. }
            | Self::SExprError { message, .. }
            | Self::SatError { message, .. }
            | Self::SymbolicError { message, .. }
            | Self::TotalityError { message, .. } => message,
        }
    }

    /// Extract the error code (e.g. "E201") from the message if present.
    ///
    /// Looks for `[Ennn]` patterns in the message string.  Falls back to
    /// the variant's generic code if no embedded code is found.
    pub fn error_code(&self) -> Option<String> {
        // First: try to extract an embedded [Ennn] from the message.
        if let Some(code) = extract_embedded_code(self.message()) {
            return Some(code);
        }
        // Fallback: generic code per variant.
        match self {
            Self::ParseError { .. } => Some("E100".to_string()),
            Self::TemporalCompilationError { .. } => Some("E300".to_string()),
            Self::PatternError { .. } => Some("E400".to_string()),
            Self::RspuError { .. } => Some("E700".to_string()),
            Self::SExprError { .. } => Some("E800".to_string()),
            Self::SatError { .. } => Some("E900".to_string()),
            Self::SymbolicError { .. } => Some("E1000".to_string()),
            Self::TotalityError { .. } => Some("E1100".to_string()),
            // SemanticError and TypeError fall back to category codes.
            Self::SemanticError { .. } => Some("E200".to_string()),
            Self::TypeError { .. } => Some("E600".to_string()),
        }
    }

    /// Convert this error to a structured `Diagnostic` for rich rendering.
    pub fn to_diagnostic(&self) -> Diagnostic {
        let code = self.error_code();
        let clean_msg = strip_embedded_code(self.message());

        let severity = Severity::Error;

        let mut diag = Diagnostic::error(clean_msg).with_span(self.span());
        diag.severity = severity;
        if let Some(c) = code {
            diag = diag.with_code(c);
        }
        diag
    }
}

/// Extract an `[Ennn]` error code from the beginning of a message.
fn extract_embedded_code(msg: &str) -> Option<String> {
    if msg.len() < 5 {
        return None;
    }
    let bytes = msg.as_bytes();
    if bytes[0] != b'[' {
        return None;
    }
    // Look for closing bracket within first 7 chars: [E100] or [E1234]
    // Bounded: at most 7 iterations.
    let mut i: usize = 1;
    while i < msg.len().min(8) {
        if bytes[i] == b']' {
            let code = &msg[1..i];
            // Validate: must start with 'E' followed by digits.
            if code.len() >= 2
                && code.as_bytes()[0] == b'E'
                && code[1..].bytes().all(|b| b.is_ascii_digit())
            {
                return Some(code.to_string());
            }
            return None;
        }
        i += 1;
    }
    None
}

/// Strip a leading `[Ennn] ` prefix from a message, returning the clean body.
fn strip_embedded_code(msg: &str) -> String {
    if msg.len() < 5 {
        return msg.to_string();
    }
    let bytes = msg.as_bytes();
    if bytes[0] != b'[' {
        return msg.to_string();
    }
    // Bounded: at most 8 iterations.
    let mut i: usize = 1;
    while i < msg.len().min(8) {
        if bytes[i] == b']' {
            let rest = &msg[i + 1..];
            // Skip optional space after bracket.
            return if let Some(stripped) = rest.strip_prefix(' ') {
                stripped.to_string()
            } else {
                rest.to_string()
            };
        }
        i += 1;
    }
    msg.to_string()
}

impl fmt::Display for MirrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MirrError::ParseError { message, span } => {
                write!(f, "[E100] Parse error: {}", message)?;
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::SemanticError { message, span } => {
                if let Some(code) = extract_embedded_code(message) {
                    write!(f, "[{}] Semantic error: {}", code, strip_embedded_code(message))?;
                } else {
                    write!(f, "[E200] Semantic error: {}", message)?;
                }
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::TemporalCompilationError { message, span } => {
                write!(f, "[E300] Temporal compilation error: {}", message)?;
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::PatternError { message, span } => {
                write!(f, "[E400] Pattern error: {}", message)?;
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::TypeError { message, span } => {
                if let Some(code) = extract_embedded_code(message) {
                    write!(f, "[{}] Type error: {}", code, strip_embedded_code(message))?;
                } else {
                    write!(f, "[E600] Type error: {}", message)?;
                }
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::RspuError { message, span } => {
                write!(f, "[E700] R-SPU error: {}", message)?;
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::SExprError { message, span } => {
                write!(f, "[E800] S-expression error: {}", message)?;
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::SatError { message, span } => {
                write!(f, "[E900] SAT error: {}", message)?;
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::SymbolicError { message, span } => {
                write!(f, "[E1000] Symbolic error: {}", message)?;
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
            MirrError::TotalityError { message, span } => {
                write!(f, "[E1100] Totality error: {}", message)?;
                if let Some(s) = span {
                    write!(f, " (line {})", s.start_line + 1)?;
                }
                Ok(())
            }
        }
    }
}

impl Error for MirrError {}

// ---------------------------------------------------------------------------
// Multi-error accumulation (ERR-002)
// ---------------------------------------------------------------------------

/// Maximum errors accumulated before a pass stops scanning.
/// NASA Power-of-10: all collections bounded.
pub const MAX_ACCUMULATED_ERRORS: usize = 20;

/// Container for multiple compiler errors accumulated within a single pass.
///
/// The pipeline uses this to report all recoverable errors at once instead
/// of stopping at the first.  Bounded by [`MAX_ACCUMULATED_ERRORS`].
#[derive(Debug, Clone)]
pub struct PipelineErrors {
    pub errors: Vec<MirrError>,
}

impl PipelineErrors {
    /// Create an empty error accumulator.
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Push an error, respecting the accumulation cap.
    /// No-op if already at [`MAX_ACCUMULATED_ERRORS`].
    pub fn push(&mut self, e: MirrError) {
        if self.errors.len() < MAX_ACCUMULATED_ERRORS {
            self.errors.push(e);
        }
    }

    /// True when no errors have been accumulated.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Number of accumulated errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Reference to the first error, if any.
    pub fn first(&self) -> Option<&MirrError> {
        self.errors.first()
    }

    /// Convert all contained errors to `Diagnostic` structs.
    pub fn to_diagnostics(&self) -> Vec<Diagnostic> {
        self.errors.iter().map(MirrError::to_diagnostic).collect()
    }
}

impl Default for PipelineErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MirrError> for PipelineErrors {
    fn from(e: MirrError) -> Self {
        Self { errors: vec![e] }
    }
}

impl From<Vec<MirrError>> for PipelineErrors {
    fn from(errors: Vec<MirrError>) -> Self {
        Self { errors }
    }
}

impl fmt::Display for PipelineErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.errors {
            writeln!(f, "{e}")?;
        }
        let n = self.errors.len();
        if n == 1 {
            write!(f, "error: aborting due to previous error")
        } else {
            write!(f, "error: aborting due to {n} previous errors")
        }
    }
}

impl Error for PipelineErrors {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_code_from_semantic_message() {
        assert_eq!(
            extract_embedded_code("[E201] duplicate signal name 'x'"),
            Some("E201".to_string())
        );
    }

    #[test]
    fn extract_code_from_plain_message() {
        assert_eq!(extract_embedded_code("Guard name is empty"), None);
    }

    #[test]
    fn strip_code_preserves_body() {
        assert_eq!(
            strip_embedded_code("[E201] duplicate signal name 'x'"),
            "duplicate signal name 'x'"
        );
    }

    #[test]
    fn strip_code_no_code_passes_through() {
        assert_eq!(strip_embedded_code("Guard name is empty"), "Guard name is empty");
    }

    #[test]
    fn to_diagnostic_semantic_with_code() {
        let err = MirrError::SemanticError {
            message: "[E201] duplicate signal name 'x'".to_string(),
            span: Some(Span::full_line(4)),
        };
        let diag = err.to_diagnostic();
        assert_eq!(diag.code.as_deref(), Some("E201"));
        assert_eq!(diag.message, "duplicate signal name 'x'");
        assert!(diag.span.is_some());
    }

    #[test]
    fn to_diagnostic_parse_fallback_code() {
        let err = MirrError::ParseError { message: "Guard name is empty".to_string(), span: None };
        let diag = err.to_diagnostic();
        assert_eq!(diag.code.as_deref(), Some("E100"));
        assert_eq!(diag.message, "Guard name is empty");
    }

    #[test]
    fn error_code_rspu_embedded() {
        let err = MirrError::RspuError {
            message: "[E701] register allocation failed".to_string(),
            span: None,
        };
        assert_eq!(err.error_code().as_deref(), Some("E701"));
    }

    #[test]
    fn display_backward_compat() {
        let err = MirrError::ParseError {
            message: "Guard name is empty".to_string(),
            span: Some(Span::full_line(4)),
        };
        let s = err.to_string();
        assert!(s.contains("[E100]"));
        assert!(s.contains("Parse error:"));
        assert!(s.contains("(line 5)"));
    }
}
