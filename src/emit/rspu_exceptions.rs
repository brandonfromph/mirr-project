//! R-SPU exception model for ISA v2.
//!
//! Defines the exception handling primitives for the Reflex Signal Processing
//! Unit.  The R-SPU operates in two execution modes:
//!
//! - **Reflex** (default): deterministic, cycle-accurate, no interrupts.
//! - **Host**: interrupt-driven, variable latency.
//!
//! Exception codes map to MIRR error codes in the E7xx range.
//! All collections are bounded by `MAX_*` constants (NASA Power-of-10).

#![forbid(unsafe_code)]

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::{MAX_EXCEPTION_DEPTH, MAX_TRAP_HANDLERS};
use crate::error::MirrError;

// ---------------------------------------------------------------------------
// Execution mode
// ---------------------------------------------------------------------------

/// R-SPU execution mode.
///
/// The processor starts in `Reflex` mode (deterministic, cycle-accurate).
/// A `TrapToHost` action transitions to `Host` mode (interrupt-driven).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecMode {
    /// Deterministic, cycle-accurate execution — no interrupts.
    Reflex,
    /// Interrupt-driven execution with variable latency.
    Host,
}

impl fmt::Display for ExecMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reflex => write!(f, "Reflex"),
            Self::Host => write!(f, "Host"),
        }
    }
}

// ---------------------------------------------------------------------------
// Exception codes
// ---------------------------------------------------------------------------

/// Exception codes for the R-SPU runtime.
///
/// Each variant maps to an error code in the E7xx range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ExceptionCode {
    /// Tag-type violation at runtime (E708).
    TagViolation = 0,
    /// Hard real-time deadline missed (E715).
    DeadlineMiss = 1,
    /// `AssertAlways` or `AssertNever` property violation.
    PropertyFail = 2,
    /// Integer division by zero.
    DivisionByZero = 3,
    /// Register file overflow.
    RegisterOverflow = 4,
    /// Software-initiated trap (`TRAP` instruction).
    SoftwareTrap = 5,
    /// Invalid execution-mode transition (E714).
    InvalidMode = 6,
    /// Interval bound violation (MEGA-5 symbolic reasoning).
    IntervalViolation = 7,
}

impl fmt::Display for ExceptionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TagViolation => write!(f, "TagViolation (E708)"),
            Self::DeadlineMiss => write!(f, "DeadlineMiss (E715)"),
            Self::PropertyFail => write!(f, "PropertyFail"),
            Self::DivisionByZero => write!(f, "DivisionByZero"),
            Self::RegisterOverflow => write!(f, "RegisterOverflow"),
            Self::SoftwareTrap => write!(f, "SoftwareTrap"),
            Self::InvalidMode => write!(f, "InvalidMode (E714)"),
            Self::IntervalViolation => write!(f, "IntervalViolation (MEGA-5)"),
        }
    }
}

// ---------------------------------------------------------------------------
// Exception actions
// ---------------------------------------------------------------------------

/// Action taken by the R-SPU when an exception is raised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExceptionAction {
    /// Graceful halt — complete the current cycle then stop.
    Halt,
    /// Immediate stop — abort mid-cycle (safety-critical).
    EmergencyStop,
    /// Skip the faulting operation and continue execution.
    IgnoreAndContinue,
    /// Switch to Host mode for software handling.
    TrapToHost,
}

impl fmt::Display for ExceptionAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => write!(f, "Halt"),
            Self::EmergencyStop => write!(f, "EmergencyStop"),
            Self::IgnoreAndContinue => write!(f, "IgnoreAndContinue"),
            Self::TrapToHost => write!(f, "TrapToHost"),
        }
    }
}

// ---------------------------------------------------------------------------
// Exception state machine
// ---------------------------------------------------------------------------

/// Runtime exception state for a single R-SPU core.
///
/// Tracks the current execution mode, pending exceptions, handler
/// configuration, and nesting depth.  All vectors are bounded by
/// `MAX_TRAP_HANDLERS`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionState {
    /// Currently pending exception, if any.
    pub pending: Option<ExceptionCode>,
    /// Handler table indexed by `ExceptionCode as u8`.
    /// Bounded by [`MAX_TRAP_HANDLERS`].
    pub handler_table: Vec<ExceptionAction>,
    /// Current execution mode.
    pub mode: ExecMode,
    /// Whether the processor has been halted.
    pub halted: bool,
    /// Current exception nesting depth.
    /// Bounded by [`MAX_EXCEPTION_DEPTH`].
    pub depth: usize,
}

impl ExceptionState {
    /// Create a new exception state in the default configuration.
    ///
    /// Starts in `Reflex` mode with no pending exception and an empty
    /// handler table.
    pub fn new() -> Self {
        Self {
            pending: None,
            handler_table: Vec::new(),
            mode: ExecMode::Reflex,
            halted: false,
            depth: 0,
        }
    }

    /// Configure a handler action for a given exception code.
    ///
    /// Uses the code's `repr(u8)` value as the table index.  Grows the
    /// table with `Halt` defaults if necessary.  Returns `Err(E712)` if
    /// the table would exceed [`MAX_TRAP_HANDLERS`].
    pub fn configure_handler(
        &mut self,
        code: ExceptionCode,
        action: ExceptionAction,
    ) -> Result<(), MirrError> {
        let idx = code as u8 as usize;
        if idx >= MAX_TRAP_HANDLERS {
            return Err(rspu_err(format!(
                "{} trap handler table full: index {} >= {}",
                crate::error_codes::ec(712),
                idx,
                MAX_TRAP_HANDLERS,
            )));
        }
        // Extend table to cover the requested index.
        while self.handler_table.len() <= idx {
            if self.handler_table.len() >= MAX_TRAP_HANDLERS {
                return Err(rspu_err(format!(
                    "{} trap handler table full: {} >= {}",
                    crate::error_codes::ec(712),
                    self.handler_table.len(),
                    MAX_TRAP_HANDLERS,
                )));
            }
            self.handler_table.push(ExceptionAction::Halt);
        }
        self.handler_table[idx] = action;
        Ok(())
    }

    /// Raise an exception and return the configured (or default) action.
    ///
    /// Returns `Err(E711)` if the nesting depth would exceed
    /// [`MAX_EXCEPTION_DEPTH`], or `Err(E713)` if the processor is
    /// already halted.
    pub fn raise_exception(&mut self, code: ExceptionCode) -> Result<ExceptionAction, MirrError> {
        if self.halted {
            return Err(rspu_err(format!("{} program halted", crate::error_codes::ec(713))));
        }
        if self.depth >= MAX_EXCEPTION_DEPTH {
            return Err(rspu_err(format!(
                "{} unhandled nested exception: depth {} >= {}",
                crate::error_codes::ec(711),
                self.depth,
                MAX_EXCEPTION_DEPTH,
            )));
        }
        self.depth += 1;
        self.pending = Some(code);

        let idx = code as u8 as usize;
        if idx < self.handler_table.len() {
            Ok(self.handler_table[idx])
        } else {
            Ok(Self::default_action(code))
        }
    }

    /// Clear the pending exception and decrement the nesting depth.
    pub fn clear_pending(&mut self) {
        self.pending = None;
        self.depth = self.depth.saturating_sub(1);
    }

    /// Halt the processor.
    pub fn halt(&mut self) {
        self.halted = true;
    }

    /// Switch execution mode.
    ///
    /// Returns `Err(E714)` if the source and target mode are the same.
    pub fn switch_mode(&mut self, new_mode: ExecMode) -> Result<(), MirrError> {
        if self.mode == new_mode {
            return Err(rspu_err(format!(
                "{} invalid mode transition: already in {} mode",
                crate::error_codes::ec(714),
                self.mode,
            )));
        }
        self.mode = new_mode;
        Ok(())
    }

    /// Default exception action when no handler has been configured.
    fn default_action(code: ExceptionCode) -> ExceptionAction {
        match code {
            ExceptionCode::TagViolation => ExceptionAction::EmergencyStop,
            ExceptionCode::DeadlineMiss => ExceptionAction::Halt,
            ExceptionCode::PropertyFail => ExceptionAction::Halt,
            ExceptionCode::DivisionByZero => ExceptionAction::Halt,
            ExceptionCode::RegisterOverflow => ExceptionAction::EmergencyStop,
            ExceptionCode::SoftwareTrap => ExceptionAction::TrapToHost,
            ExceptionCode::InvalidMode => ExceptionAction::Halt,
            ExceptionCode::IntervalViolation => ExceptionAction::Halt,
        }
    }
}

impl Default for ExceptionState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exception_state_new() {
        let state = ExceptionState::new();
        assert_eq!(state.mode, ExecMode::Reflex);
        assert!(state.pending.is_none());
        assert!(state.handler_table.is_empty());
        assert!(!state.halted);
        assert_eq!(state.depth, 0);
    }

    #[test]
    fn test_raise_and_clear_exception() {
        let mut state = ExceptionState::new();
        let action = state.raise_exception(ExceptionCode::DivisionByZero).unwrap();
        // No handler configured — should return default (Halt).
        assert_eq!(action, ExceptionAction::Halt);
        assert_eq!(state.pending, Some(ExceptionCode::DivisionByZero));
        assert_eq!(state.depth, 1);

        state.clear_pending();
        assert!(state.pending.is_none());
        assert_eq!(state.depth, 0);
    }

    #[test]
    fn test_configure_handler() {
        let mut state = ExceptionState::new();
        state.configure_handler(ExceptionCode::TagViolation, ExceptionAction::TrapToHost).unwrap();
        let action = state.raise_exception(ExceptionCode::TagViolation).unwrap();
        assert_eq!(action, ExceptionAction::TrapToHost);
    }

    #[test]
    fn test_nested_exception_depth_limit() {
        let mut state = ExceptionState::new();
        // Fill up to MAX_EXCEPTION_DEPTH.
        for _ in 0..MAX_EXCEPTION_DEPTH {
            state.raise_exception(ExceptionCode::SoftwareTrap).unwrap();
        }
        assert_eq!(state.depth, MAX_EXCEPTION_DEPTH);
        // Next raise must fail with E711.
        let err = state.raise_exception(ExceptionCode::SoftwareTrap).unwrap_err();
        assert!(err.to_string().contains("E711"));
    }

    #[test]
    fn test_mode_switch() {
        let mut state = ExceptionState::new();
        assert_eq!(state.mode, ExecMode::Reflex);

        state.switch_mode(ExecMode::Host).unwrap();
        assert_eq!(state.mode, ExecMode::Host);

        state.switch_mode(ExecMode::Reflex).unwrap();
        assert_eq!(state.mode, ExecMode::Reflex);

        // Same-mode transition must fail with E714.
        let err = state.switch_mode(ExecMode::Reflex).unwrap_err();
        assert!(err.to_string().contains("E714"));
    }

    #[test]
    fn test_halt_prevents_further_exceptions() {
        let mut state = ExceptionState::new();
        state.halt();
        assert!(state.halted);

        let err = state.raise_exception(ExceptionCode::DivisionByZero).unwrap_err();
        assert!(err.to_string().contains("E713"));
    }
}
