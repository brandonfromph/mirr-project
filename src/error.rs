// ---------------------------------------------------------------------------
// Centralized error authority
// ---------------------------------------------------------------------------
// NASA/JPL Rule: All error variants for the subsystem are catalogued in one
// place — critical for mission-critical diagnostics.
// ---------------------------------------------------------------------------

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum MirrError {
    /// General parsing error
    ParseError { message: String },
    /// Lexical analysis error
    LexicalError { message: String },
    /// Semantic analysis error
    SemanticError { message: String },
    /// Temporal causality violation error
    TemporalCausalityViolation { cause: String, effect: String, constraint_type: String },
    /// Temporal compilation error
    TemporalCompilationError { message: String },
}

impl MirrError {
    pub fn new(message: impl Into<String>) -> Self {
        Self::ParseError { message: message.into() }
    }
}

impl fmt::Display for MirrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MirrError::ParseError { message } => write!(f, "Parse error: {}", message),
            MirrError::LexicalError { message } => write!(f, "Lexical error: {}", message),
            MirrError::SemanticError { message } => write!(f, "Semantic error: {}", message),
            MirrError::TemporalCausalityViolation { cause, effect, constraint_type } => {
                write!(
                    f,
                    "Temporal causality violation: {} cannot cause {} ({})",
                    cause, effect, constraint_type
                )
            }
            MirrError::TemporalCompilationError { message } => {
                write!(f, "Temporal compilation error: {}", message)
            }
        }
    }
}

impl Error for MirrError {}
