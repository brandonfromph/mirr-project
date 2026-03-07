//! Temporal Guard Resource Estimation Tests
//!
//! Requirement coverage: P2-REQ-009 through P2-REQ-012
//! Ref: MIRR-PHASE2-001 §6 (traceability table)

use nasa_rust_project::temporal::{
    compiler::{ImplementationStrategy, ResourceEstimator},
    low_level_ir::ConditionKind,
};

// ---------------------------------------------------------------------------
// P2-REQ-009: Shift-register resource estimate is correct
// ---------------------------------------------------------------------------
#[test]
fn test_shift_register_resource_est() {
    let est = ResourceEstimator::estimate_shift_register_resources(4);
    assert_eq!(est.shift_registers, 4, "4 cycles → 4 SR flip-flops");
    assert_eq!(est.counters, 0, "no counters for SR implementation");
    assert_eq!(est.logic_gates, 1, "one AND gate at the output");
    assert_eq!(est.total_signals, 5, "4 stages + 1 output");
}

#[test]
fn test_shift_register_resource_est_single_cycle() {
    let est = ResourceEstimator::estimate_shift_register_resources(1);
    assert_eq!(est.shift_registers, 1);
    assert_eq!(est.total_signals, 2); // 1 stage + 1 output
}

// ---------------------------------------------------------------------------
// P2-REQ-010: Counter resource estimate is correct
// ---------------------------------------------------------------------------
#[test]
fn test_counter_resource_est() {
    // 100-cycle delay: counter width = ceil(log2(100)) + 1 = 8 bits
    let est = ResourceEstimator::estimate_counter_resources(100);
    assert_eq!(est.shift_registers, 0, "no SR stages for counter impl");
    assert_eq!(est.counters, 1, "one counter register");
    assert_eq!(est.logic_gates, 2, "increment logic + comparator");
    // total_signals = counter_width + comparator + output = 8 + 2 = 10
    assert_eq!(est.total_signals, 10);
}

#[test]
fn test_counter_resource_est_large_delay() {
    // 1000-cycle delay: ceil(log2(1000))+1 = 10+1 = 11 bits
    let est = ResourceEstimator::estimate_counter_resources(1000);
    assert_eq!(est.counters, 1);
    assert!(est.total_signals > 10, "larger delay needs wider counter → more signals");
}

// ---------------------------------------------------------------------------
// P2-REQ-011: Strategy selection respects SHIFT_REGISTER_THRESHOLD (= 16)
// ---------------------------------------------------------------------------
#[test]
fn test_strategy_selection_threshold() {
    // At exactly the threshold → shift register
    let at_threshold = ResourceEstimator::choose_optimal_strategy(16);
    assert!(
        matches!(at_threshold, ImplementationStrategy::ShiftRegister(_)),
        "N=16 must select ShiftRegister"
    );

    // One above the threshold → counter
    let above_threshold = ResourceEstimator::choose_optimal_strategy(17);
    assert!(
        matches!(above_threshold, ImplementationStrategy::Counter(_)),
        "N=17 must select Counter"
    );

    // Clearly short → shift register
    let short = ResourceEstimator::choose_optimal_strategy(8);
    assert!(
        matches!(short, ImplementationStrategy::ShiftRegister(_)),
        "N=8 must select ShiftRegister"
    );

    // Clearly long → counter
    let long = ResourceEstimator::choose_optimal_strategy(100);
    assert!(matches!(long, ImplementationStrategy::Counter(_)), "N=100 must select Counter");
}

// ---------------------------------------------------------------------------
// P2-REQ-012: Counter width is ceil(log2(N)) + 1 bits
// ---------------------------------------------------------------------------
#[test]
fn test_counter_width_calculation() {
    use nasa_rust_project::temporal::low_level_ir::CounterGuard;

    struct Case {
        cycles: u64,
        expected_width: u32,
    }

    let cases = [
        Case { cycles: 1, expected_width: 1 },
        Case { cycles: 2, expected_width: 2 },
        Case { cycles: 4, expected_width: 3 },
        Case { cycles: 16, expected_width: 5 },
        Case { cycles: 100, expected_width: 8 },
        Case { cycles: 1000, expected_width: 11 },
    ];

    for case in &cases {
        let ck = ConditionKind::SimpleSignal("in".to_string());
        let guard = CounterGuard::new("w".to_string(), "in".to_string(), case.cycles, ck);
        assert_eq!(
            guard.counter_width(),
            case.expected_width,
            "cycles={} expected_width={}",
            case.cycles,
            case.expected_width
        );
    }
}
