// ---------------------------------------------------------------------------
// Centralized error authority
// ---------------------------------------------------------------------------
// NASA/JPL Rule: All error variants for the subsystem are catalogued in one
// place — critical for mission-critical diagnostics.
// ---------------------------------------------------------------------------

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct MirrError {
    message: String,
}

impl MirrError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for MirrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MirrError {}