//! MAPE-K system error definitions (E12xx category).
//!
//! Integrates MAPE-K failures into the centralized MIRR Diagnostic Engine.

#![forbid(unsafe_code)]

use std::fmt;

#[derive(Debug, Clone)]
pub enum MapeKError {
    BridgeConfigError(String),
    MonitorCapacityExceeded(usize, usize),
    LoweringError(String),
    ExecutionPanic(String),
}

impl fmt::Display for MapeKError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BridgeConfigError(s) => write!(f, "[E1201] Bridge configuration error: {}", s),
            Self::MonitorCapacityExceeded(c, m) => {
                write!(f, "[E1202] Monitor capacity exceeded: {} > {}", c, m)
            }
            Self::LoweringError(s) => write!(f, "[E1203] Property lowering failure: {}", s),
            Self::ExecutionPanic(s) => write!(f, "[E1204] Simulator execution panic: {}", s),
        }
    }
}

impl std::error::Error for MapeKError {}

impl MapeKError {
    pub fn to_diagnostic(&self) -> crate::diagnostic::Diagnostic {
        let msg = self.to_string();
        crate::diagnostic::Diagnostic::error(msg)
    }
}
