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
//!
//! See `docs/error_codes.md` for the full catalogue.
// ---------------------------------------------------------------------------

use std::error::Error;
use std::fmt;

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
}

impl MirrError {
    /// Convenience constructor for parse errors (no span).
    pub fn new(message: impl Into<String>) -> Self {
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
            | Self::RspuError { span, .. } => *span,
        }
    }
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
                write!(f, "Semantic error: {}", message)?;
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
                write!(f, "Type error: {}", message)?;
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
        }
    }
}

impl Error for MirrError {}
