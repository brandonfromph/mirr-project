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

/// Maximum number of instructions per cycle (program execution limit).
pub const MAX_PROGRAM_ITERATIONS: usize = 10_000;

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
// ---------------------------------------------------------------------------
// Guard and Property state
// ---------------------------------------------------------------------------

/// Internal state of a hardware Guard Unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardUnit {
    /// A shift register of a specific length, storing up to 64 bits of history.
    ShiftRegister {
        /// The bit history (bit 0 is most recent).
        data: u64,
        /// Number of cycles to track.
        length: u32,
        /// The register being monitored for input.
        input_reg: crate::emit::rspu_isa::RegId,
    },
    /// A hardware counter.
    Counter {
        /// Current count value.
        current: u64,
        /// Target count value.
        target: u64,
        /// The register being monitored for the condition.
        input_reg: crate::emit::rspu_isa::RegId,
    },
    /// A simple combinatorial guard (direct signal pass-through).
    Combinatorial(bool),
    /// An uninitialized or inactive guard unit.
    Uninitialized,
}

impl Default for GuardUnit {
    fn default() -> Self {
        Self::Uninitialized
    }
}

/// A double-buffered guard unit for cycle-accurate simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DoubleBufferedGuard {
    /// The state visible to the current cycle's `Query` instructions.
    pub current: GuardUnit,
    /// The state being updated by the current cycle's `Tick` instructions.
    pub next: GuardUnit,
}

impl DoubleBufferedGuard {
    /// Commit the next state to the current state at the end of a cycle.
    pub fn commit(&mut self) {
        self.current = self.next;
    }
}

impl std::ops::Not for DoubleBufferedGuard {
    type Output = bool;

    fn not(self) -> Self::Output {
        match self.current {
            GuardUnit::ShiftRegister { data, length, .. } => {
                let mask = 1u64 << (length - 1);
                (data & mask) == 0
            }
            GuardUnit::Counter { current, target, .. } => current < target,
            GuardUnit::Combinatorial(b) => !b,
            GuardUnit::Uninitialized => true,
        }
    }
}

/// Simulation status for an LTL property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyStatus {
    /// Property is active and currently satisfied.
    Satisfied,
    /// Property has been violated in the current or a past cycle.
    Violated,
    /// Property is inactive (not yet triggered).
    Inactive,
}

#[derive(Debug, Clone)]
pub struct PropertyState {
    /// Mapping from property ID to its current status.
    pub statuses: std::collections::HashMap<crate::emit::rspu_isa::PropertyId, PropertyStatus>,
    /// Legacy violation list for compatibility with existing tests.
    pub violations: Vec<crate::emit::rspu_isa::PropertyId>,
}

impl PropertyState {
    pub fn new() -> Self {
        Self { statuses: std::collections::HashMap::new(), violations: Vec::new() }
    }

    pub(crate) fn record_violation(&mut self, id: PropertyId) {
        if self.statuses.len() < MAX_PROPERTY_VIOLATIONS {
            self.statuses.insert(id, PropertyStatus::Violated);
            if !self.violations.contains(&id) {
                self.violations.push(id);
            }
        }
    }

    pub(crate) fn record_satisfaction(&mut self, id: PropertyId) {
        if self.statuses.len() < MAX_PROPERTY_VIOLATIONS {
            // Only upgrade to Satisfied if not already Violated.
            let entry = self.statuses.entry(id).or_insert(PropertyStatus::Satisfied);
            if *entry != PropertyStatus::Violated {
                *entry = PropertyStatus::Satisfied;
            }
        }
    }

    pub fn get_violations(&self) -> Vec<PropertyId> {
        self.statuses
            .iter()
            .filter(|(_, status)| **status == PropertyStatus::Violated)
            .map(|(id, _)| *id)
            .collect()
    }
}

impl Default for PropertyState {
    fn default() -> Self {
        Self::new()
    }
}
