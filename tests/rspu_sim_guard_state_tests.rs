//! R-SPU simulator guard and property state tests.
//!
//! Exercises `src/emit/rspu_sim/sim_types.rs` branches:
//!   - DoubleBufferedGuard: commit, Not impl for all GuardUnit variants
//!   - PropertyState: record_violation, record_satisfaction, get_violations
//!   - SimResult construction
//!   - StepResult variants

#![forbid(unsafe_code)]
#![allow(clippy::bool_assert_comparison)]

use mirrc::emit::rspu_exceptions::ExceptionCode;
use mirrc::emit::rspu_sim::{
    DoubleBufferedGuard, GuardUnit, PropertyState, PropertyStatus, SimResult, StepResult,
};
use mirrc::emit::rspu_tagged::TaggedWord;

// -----------------------------------------------------------------------
// StepResult variant coverage
// -----------------------------------------------------------------------
#[test]
fn step_result_variants_are_distinguishable() {
    assert_eq!(StepResult::Continue, StepResult::Continue);
    assert_eq!(StepResult::Halted, StepResult::Halted);
    assert_eq!(StepResult::EmergencyStop, StepResult::EmergencyStop);
    assert_eq!(
        StepResult::Exception(ExceptionCode::DivisionByZero),
        StepResult::Exception(ExceptionCode::DivisionByZero)
    );
    assert_ne!(StepResult::Continue, StepResult::Halted);
}

// -----------------------------------------------------------------------
// DoubleBufferedGuard: commit propagates next → current
// -----------------------------------------------------------------------
#[test]
fn double_buffered_guard_commit_propagates_next_to_current() {
    let mut guard = DoubleBufferedGuard::default();
    assert_eq!(guard.current, GuardUnit::Uninitialized);
    assert_eq!(guard.next, GuardUnit::Uninitialized);

    guard.next = GuardUnit::Combinatorial(true);
    guard.commit();
    assert_eq!(guard.current, GuardUnit::Combinatorial(true));
}

// -----------------------------------------------------------------------
// DoubleBufferedGuard: Not impl for each GuardUnit variant
// -----------------------------------------------------------------------
#[test]
fn not_uninitialized_guard_is_true() {
    let guard =
        DoubleBufferedGuard { current: GuardUnit::Uninitialized, next: GuardUnit::Uninitialized };
    assert_eq!(!guard, true);
}

#[test]
fn not_combinatorial_true_is_false() {
    let guard = DoubleBufferedGuard {
        current: GuardUnit::Combinatorial(true),
        next: GuardUnit::Uninitialized,
    };
    assert_eq!(!guard, false);
}

#[test]
fn not_combinatorial_false_is_true() {
    let guard = DoubleBufferedGuard {
        current: GuardUnit::Combinatorial(false),
        next: GuardUnit::Uninitialized,
    };
    assert_eq!(!guard, true);
}

#[test]
fn not_shift_register_with_bit_set_is_false() {
    // data = 0b1000, length = 4 → mask = 1 << 3 = 8 → (data & mask) != 0 → !guard = false
    let guard = DoubleBufferedGuard {
        current: GuardUnit::ShiftRegister { data: 0b1000, length: 4, input_reg: 0 },
        next: GuardUnit::Uninitialized,
    };
    assert_eq!(!guard, false);
}

#[test]
fn not_shift_register_with_bit_clear_is_true() {
    // data = 0b0000, length = 4 → mask = 1 << 3 = 8 → (data & mask) == 0 → !guard = true
    let guard = DoubleBufferedGuard {
        current: GuardUnit::ShiftRegister { data: 0, length: 4, input_reg: 0 },
        next: GuardUnit::Uninitialized,
    };
    assert_eq!(!guard, true);
}

#[test]
fn not_counter_below_target_is_true() {
    let guard = DoubleBufferedGuard {
        current: GuardUnit::Counter { current: 5, target: 10, input_reg: 0 },
        next: GuardUnit::Uninitialized,
    };
    // current < target → !guard = true
    assert_eq!(!guard, true);
}

#[test]
fn not_counter_at_target_is_false() {
    let guard = DoubleBufferedGuard {
        current: GuardUnit::Counter { current: 10, target: 10, input_reg: 0 },
        next: GuardUnit::Uninitialized,
    };
    // current == target → current < target is false → !guard = false
    assert_eq!(!guard, false);
}

#[test]
fn not_counter_above_target_is_false() {
    let guard = DoubleBufferedGuard {
        current: GuardUnit::Counter { current: 15, target: 10, input_reg: 0 },
        next: GuardUnit::Uninitialized,
    };
    assert_eq!(!guard, false);
}

// -----------------------------------------------------------------------
// PropertyState: record_violation and record_satisfaction
// -----------------------------------------------------------------------
#[test]
fn property_state_new_is_empty() {
    let ps = PropertyState::new();
    assert!(ps.statuses.is_empty());
    assert!(ps.violations.is_empty());
    assert!(ps.get_violations().is_empty());
}

#[test]
fn property_state_default_is_same_as_new() {
    let ps = PropertyState::default();
    assert!(ps.statuses.is_empty());
    assert!(ps.violations.is_empty());
}

#[test]
fn record_violation_adds_to_statuses_and_violations() {
    let mut ps = PropertyState::new();
    ps.record_violation(42);
    assert_eq!(*ps.statuses.get(&42).unwrap(), PropertyStatus::Violated);
    assert_eq!(ps.violations, vec![42]);
}

#[test]
fn record_violation_deduplicates_violations_list() {
    let mut ps = PropertyState::new();
    ps.record_violation(7);
    ps.record_violation(7);
    ps.record_violation(7);
    assert_eq!(ps.violations, vec![7]);
}

#[test]
fn record_satisfaction_sets_satisfied_status() {
    let mut ps = PropertyState::new();
    ps.record_satisfaction(99);
    assert_eq!(*ps.statuses.get(&99).unwrap(), PropertyStatus::Satisfied);
}

#[test]
fn record_satisfaction_does_not_overwrite_violated() {
    let mut ps = PropertyState::new();
    ps.record_violation(5);
    ps.record_satisfaction(5);
    // Violated must NOT be downgraded to Satisfied
    assert_eq!(*ps.statuses.get(&5).unwrap(), PropertyStatus::Violated);
}

#[test]
fn get_violations_returns_only_violated_properties() {
    let mut ps = PropertyState::new();
    ps.record_violation(1);
    ps.record_satisfaction(2);
    ps.record_violation(3);
    ps.record_satisfaction(4);
    let violations = ps.get_violations();
    assert!(violations.contains(&1));
    assert!(violations.contains(&3));
    assert!(!violations.contains(&2));
    assert!(!violations.contains(&4));
}

// -----------------------------------------------------------------------
// SimResult construction
// -----------------------------------------------------------------------
#[test]
fn sim_result_can_be_constructed_and_serialized() {
    use mirrc::emit::rspu_tagged::TypeTag;
    use std::collections::HashMap;

    let mut outputs = HashMap::new();
    outputs.insert(0u16, TaggedWord::from_literal(42, TypeTag::Unsigned { width: 8 }));

    let result = SimResult {
        cycles: 100,
        outputs,
        property_violations: vec![1, 2],
        exception: Some(ExceptionCode::DeadlineMiss),
        halted: true,
    };

    assert_eq!(result.cycles, 100);
    assert!(result.halted);
    assert_eq!(result.property_violations.len(), 2);
    assert_eq!(result.exception, Some(ExceptionCode::DeadlineMiss));
    assert_eq!(result.outputs.get(&0).unwrap().value, 42);

    // Verify serde works
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("100"));
}
