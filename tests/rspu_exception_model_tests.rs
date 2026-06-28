//! R-SPU exception model unit tests — exercises `src/emit/rspu_exceptions.rs`
//! branches not covered by inline tests:
//!   - Display impls for all ExceptionCode / ExceptionAction variants
//!   - default_action for every ExceptionCode
//!   - ExceptionState: configure_handler extending with defaults
//!   - raise_exception with custom handlers at non-zero indices

#![forbid(unsafe_code)]

use mirrc::emit::rspu_exceptions::{
    ExceptionState, ExceptionCode, ExceptionAction, ExecMode,
};

// -----------------------------------------------------------------------
// Display coverage
// -----------------------------------------------------------------------
#[test]
fn exception_code_display_all_variants() {
    assert_eq!(format!("{}", ExceptionCode::TagViolation), "TagViolation (E708)");
    assert_eq!(format!("{}", ExceptionCode::DeadlineMiss), "DeadlineMiss (E715)");
    assert_eq!(format!("{}", ExceptionCode::PropertyFail), "PropertyFail");
    assert_eq!(format!("{}", ExceptionCode::DivisionByZero), "DivisionByZero");
    assert_eq!(format!("{}", ExceptionCode::RegisterOverflow), "RegisterOverflow");
    assert_eq!(format!("{}", ExceptionCode::SoftwareTrap), "SoftwareTrap");
    assert_eq!(format!("{}", ExceptionCode::InvalidMode), "InvalidMode (E714)");
    assert_eq!(format!("{}", ExceptionCode::IntervalViolation), "IntervalViolation (MEGA-5)");
}

#[test]
fn exception_action_display_all_variants() {
    assert_eq!(format!("{}", ExceptionAction::Halt), "Halt");
    assert_eq!(format!("{}", ExceptionAction::EmergencyStop), "EmergencyStop");
    assert_eq!(format!("{}", ExceptionAction::IgnoreAndContinue), "IgnoreAndContinue");
    assert_eq!(format!("{}", ExceptionAction::TrapToHost), "TrapToHost");
}

#[test]
fn exec_mode_display() {
    assert_eq!(format!("{}", ExecMode::Reflex), "Reflex");
    assert_eq!(format!("{}", ExecMode::Host), "Host");
}

// -----------------------------------------------------------------------
// Default action coverage per ExceptionCode
// -----------------------------------------------------------------------
#[test]
fn default_action_tag_violation_is_emergency_stop() {
    let mut state = ExceptionState::new();
    let action = state.raise_exception(ExceptionCode::TagViolation).unwrap();
    assert_eq!(action, ExceptionAction::EmergencyStop);
}

#[test]
fn default_action_deadline_miss_is_halt() {
    let mut state = ExceptionState::new();
    let action = state.raise_exception(ExceptionCode::DeadlineMiss).unwrap();
    assert_eq!(action, ExceptionAction::Halt);
}

#[test]
fn default_action_property_fail_is_halt() {
    let mut state = ExceptionState::new();
    let action = state.raise_exception(ExceptionCode::PropertyFail).unwrap();
    assert_eq!(action, ExceptionAction::Halt);
}

#[test]
fn default_action_register_overflow_is_emergency_stop() {
    let mut state = ExceptionState::new();
    let action = state.raise_exception(ExceptionCode::RegisterOverflow).unwrap();
    assert_eq!(action, ExceptionAction::EmergencyStop);
}

#[test]
fn default_action_software_trap_is_trap_to_host() {
    let mut state = ExceptionState::new();
    let action = state.raise_exception(ExceptionCode::SoftwareTrap).unwrap();
    assert_eq!(action, ExceptionAction::TrapToHost);
}

#[test]
fn default_action_invalid_mode_is_halt() {
    let mut state = ExceptionState::new();
    let action = state.raise_exception(ExceptionCode::InvalidMode).unwrap();
    assert_eq!(action, ExceptionAction::Halt);
}

#[test]
fn default_action_interval_violation_is_halt() {
    let mut state = ExceptionState::new();
    let action = state.raise_exception(ExceptionCode::IntervalViolation).unwrap();
    assert_eq!(action, ExceptionAction::Halt);
}

// -----------------------------------------------------------------------
// Handler table gap-filling: configure handler at index 7 fills 0..7 with Halt
// -----------------------------------------------------------------------
#[test]
fn configure_handler_at_high_index_fills_gap_with_halt() {
    let mut state = ExceptionState::new();
    // IntervalViolation = 7 → should fill indices 0..6 with Halt
    state
        .configure_handler(ExceptionCode::IntervalViolation, ExceptionAction::IgnoreAndContinue)
        .unwrap();
    assert!(state.handler_table.len() >= 8);

    // Verify gap entries are Halt
    for i in 0..7 {
        assert_eq!(state.handler_table[i], ExceptionAction::Halt);
    }
    // Verify configured entry
    assert_eq!(state.handler_table[7], ExceptionAction::IgnoreAndContinue);
}

// -----------------------------------------------------------------------
// Clear pending underflow safety
// -----------------------------------------------------------------------
#[test]
fn clear_pending_on_fresh_state_does_not_underflow() {
    let mut state = ExceptionState::new();
    // depth is 0 — clear_pending should not panic or underflow
    state.clear_pending();
    assert_eq!(state.depth, 0);
    assert!(state.pending.is_none());
}

// -----------------------------------------------------------------------
// Mode switching from Host back to Reflex and double-same error
// -----------------------------------------------------------------------
#[test]
fn switch_from_host_to_reflex() {
    let mut state = ExceptionState::new();
    state.switch_mode(ExecMode::Host).unwrap();
    assert_eq!(state.mode, ExecMode::Host);
    state.switch_mode(ExecMode::Reflex).unwrap();
    assert_eq!(state.mode, ExecMode::Reflex);
}

#[test]
fn double_host_switch_fails() {
    let mut state = ExceptionState::new();
    state.switch_mode(ExecMode::Host).unwrap();
    let err = state.switch_mode(ExecMode::Host).unwrap_err();
    assert!(err.to_string().contains("E714"));
}

// -----------------------------------------------------------------------
// raise_exception with configured handler overrides default
// -----------------------------------------------------------------------
#[test]
fn configured_handler_overrides_default_for_division_by_zero() {
    let mut state = ExceptionState::new();
    state
        .configure_handler(ExceptionCode::DivisionByZero, ExceptionAction::IgnoreAndContinue)
        .unwrap();
    let action = state.raise_exception(ExceptionCode::DivisionByZero).unwrap();
    assert_eq!(action, ExceptionAction::IgnoreAndContinue);
}

// -----------------------------------------------------------------------
// Serde round-trip for ExceptionCode and ExecMode
// -----------------------------------------------------------------------
#[test]
fn exception_code_serde_roundtrip() {
    let codes = [
        ExceptionCode::TagViolation,
        ExceptionCode::DeadlineMiss,
        ExceptionCode::PropertyFail,
        ExceptionCode::DivisionByZero,
        ExceptionCode::RegisterOverflow,
        ExceptionCode::SoftwareTrap,
        ExceptionCode::InvalidMode,
        ExceptionCode::IntervalViolation,
    ];
    for code in &codes {
        let json = serde_json::to_string(code).unwrap();
        let restored: ExceptionCode = serde_json::from_str(&json).unwrap();
        assert_eq!(*code, restored);
    }
}

#[test]
fn exec_mode_serde_roundtrip() {
    for mode in [ExecMode::Reflex, ExecMode::Host] {
        let json = serde_json::to_string(&mode).unwrap();
        let restored: ExecMode = serde_json::from_str(&json).unwrap();
        assert_eq!(mode, restored);
    }
}
