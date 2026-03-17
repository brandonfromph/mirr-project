//! R-SPU simulator data types: StepResult, SimResult, PropertyState.

#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::emit::rspu_exceptions::ExceptionCode;
use crate::emit::rspu_isa::{PortId, PropertyId};
use crate::emit::rspu_tagged::TaggedWord;
use serde::Serialize;

// ---------------------------------------------------------------------------
// Constants (NASA P10 bounded-resource model)
// ---------------------------------------------------------------------------

/// Maximum number of property violations tracked before saturation.
const MAX_PROPERTY_VIOLATIONS: usize = 1024;

// ---------------------------------------------------------------------------
// StepResult
// ---------------------------------------------------------------------------

/// Result of executing a single instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepResult {
    /// Execution should continue to the next instruction.
    Continue,
    /// The processor has been halted (graceful stop).
    Halted,
    /// Emergency stop (immediate abort, safety-critical).
    EmergencyStop,
    /// An exception was raised with the given code.
    Exception(ExceptionCode),
}

// ---------------------------------------------------------------------------
// SimResult
// ---------------------------------------------------------------------------

/// Result of a complete simulation run.
#[derive(Debug, Clone, Serialize)]
pub struct SimResult {
    /// Total cycles executed.
    pub cycles: u64,
    /// Output port values (scanned from the output register partition).
    pub outputs: HashMap<PortId, TaggedWord>,
    /// Property IDs that were violated during execution.
    pub property_violations: Vec<PropertyId>,
    /// Exception that terminated execution, if any.
    pub exception: Option<ExceptionCode>,
    /// Whether the simulator halted normally.
    pub halted: bool,
}

// ---------------------------------------------------------------------------
// PropertyState
// ---------------------------------------------------------------------------

/// Tracks property assertion violations during simulation.
#[derive(Debug, Clone)]
pub struct PropertyState {
    /// List of property IDs that have been violated.
    pub violations: Vec<PropertyId>,
}

impl PropertyState {
    /// Create a new property state with no violations.
    pub fn new() -> Self {
        Self { violations: Vec::new() }
    }

    /// Record a property violation, respecting the saturation bound.
    pub(crate) fn record_violation(&mut self, id: PropertyId) {
        if self.violations.len() < MAX_PROPERTY_VIOLATIONS {
            self.violations.push(id);
        }
    }
}

impl Default for PropertyState {
    fn default() -> Self {
        Self::new()
    }
}
