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

#[derive(Debug, Clone)]
pub enum MirrError {
    /// Parse/lexical error (E1xx).
    ParseError { message: String },
    /// Semantic analysis error (E2xx).
    SemanticError { message: String },
    /// Temporal compilation error (E3xx).
    TemporalCompilationError { message: String },
    /// Pattern expansion error (E4xx).
    PatternError { message: String },
    /// Type checking error (E6xx).
    TypeError { message: String },
    /// R-SPU emission error (E7xx).
    RspuError { message: String },
}

impl MirrError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::ParseError { message: message.into() }
    }
}

impl fmt::Display for MirrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MirrError::ParseError { message } => write!(f, "[E100] Parse error: {}", message),
            MirrError::SemanticError { message } => {
                write!(f, "Semantic error: {}", message)
            }
            MirrError::TemporalCompilationError { message } => {
                write!(f, "[E300] Temporal compilation error: {}", message)
            }
            MirrError::PatternError { message } => {
                write!(f, "[E400] Pattern error: {}", message)
            }
            MirrError::TypeError { message } => {
                write!(f, "Type error: {}", message)
            }
            MirrError::RspuError { message } => {
                write!(f, "[E700] R-SPU error: {}", message)
            }
        }
    }
}

impl Error for MirrError {}
