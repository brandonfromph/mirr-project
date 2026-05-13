#![forbid(unsafe_code)]

use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::symbolic::{SymState, SymValue, sym_eval_expr};
use std::fs;

#[test]
fn test_majority_gate_behavioral_logic() {
    let src = fs::read_to_string("stdlib/safety/majority.mirr").expect("failed to read majority.mirr");
    let prog = parse_mirr(&src).expect("failed to parse majority.mirr");
    
    // The majority gate logic: (A && B) || (A && C) || (B && C)
    // We check every combination of (A, B, C)
    for a in 0..=1 {
        for b in 0..=1 {
            for c in 0..=1 {
                let mut state = SymState::new();
                state.signals.push(("input_a".to_string(), SymValue::Concrete(a)));
                state.signals.push(("input_b".to_string(), SymValue::Concrete(b)));
                state.signals.push(("input_c".to_string(), SymValue::Concrete(c)));

                // Evaluate each guard manually (they are the components of the majority logic)
                let a_and_b = sym_eval_expr(&prog.module.guards.iter().find(|g| g.name == "a_and_b").unwrap().condition, &state);
                let a_and_c = sym_eval_expr(&prog.module.guards.iter().find(|g| g.name == "a_and_c").unwrap().condition, &state);
                let b_and_c = sym_eval_expr(&prog.module.guards.iter().find(|g| g.name == "b_and_c").unwrap().condition, &state);

                let expected = (a != 0 && b != 0) || (a != 0 && c != 0) || (b != 0 && c != 0);
                let actual = a_and_b.as_bool() || a_and_c.as_bool() || b_and_c.as_bool();

                assert_eq!(actual, expected, "Majority logic mismatch for A={}, B={}, C={}", a, b, c);
            }
        }
    }
}

#[test]
fn test_priority_encoder_behavioral_logic() {
    let src = fs::read_to_string("stdlib/safety/priority_enc.mirr").expect("failed to read priority_enc.mirr");
    let prog = parse_mirr(&src).expect("failed to parse priority_enc.mirr");
    
    // Test priority encoding logic
    for i in 0..16 {
        let irq0 = (i & 1) != 0;
        let irq1 = (i & 2) != 0;
        let irq2 = (i & 4) != 0;
        let irq3 = (i & 8) != 0;

        let mut state = SymState::new();
        state.signals.push(("irq_0".to_string(), SymValue::Concrete(irq0 as u64)));
        state.signals.push(("irq_1".to_string(), SymValue::Concrete(irq1 as u64)));
        state.signals.push(("irq_2".to_string(), SymValue::Concrete(irq2 as u64)));
        state.signals.push(("irq_3".to_string(), SymValue::Concrete(irq3 as u64)));

        let any_irq = sym_eval_expr(&prog.module.guards.iter().find(|g| g.name == "any_irq").unwrap().condition, &state);
        let expected_any = irq0 || irq1 || irq2 || irq3;
        assert_eq!(any_irq.as_bool(), expected_any, "any_irq logic mismatch for i={}", i);
    }
}

#[test]
fn test_sensor_validator_behavioral_logic() {
    let src = fs::read_to_string("stdlib/safety/sensor_valid.mirr").expect("failed to read sensor_valid.mirr");
    let prog = parse_mirr(&src).expect("failed to parse sensor_valid.mirr");
    
    // Check range logic
    let cases = vec![
        (100, false), // too low
        (250, true),  // in range
        (420, true),  // boundary
        (500, false), // too high
    ];

    for (temp, expected_ok) in cases {
        let mut state = SymState::new();
        state.signals.push(("raw_temp".to_string(), SymValue::Concrete(temp)));
        
        let guard = prog.module.guards.iter().find(|g| g.name == "temp_in_range").unwrap();
        let res = sym_eval_expr(&guard.condition, &state);
        assert_eq!(res.as_bool(), expected_ok, "temp_in_range logic mismatch for temp={}", temp);
    }
}

#[test]
fn test_industrial_safety_plc_behavioral_logic() {
    let src = fs::read_to_string("examples/industrial_safety_plc.mirr").expect("failed to read industrial_safety_plc.mirr");
    let prog = parse_mirr(&src).expect("failed to parse industrial_safety_plc.mirr");
    
    // Check e_stop logic
    let mut state = SymState::new();
    state.signals.push(("e_stop".to_string(), SymValue::Concrete(1)));
    let guard = prog.module.guards.iter().find(|g| g.name == "e_stop_pressed").unwrap();
    assert!(sym_eval_expr(&guard.condition, &state).as_bool());

    state.signals.clear();
    state.signals.push(("e_stop".to_string(), SymValue::Concrete(0)));
    assert!(!sym_eval_expr(&guard.condition, &state).as_bool());
}

#[test]
fn test_power_supply_monitor_behavioral_logic() {
    let src = fs::read_to_string("examples/power_supply_monitor.mirr").expect("failed to read power_supply_monitor.mirr");
    let prog = parse_mirr(&src).expect("failed to parse power_supply_monitor.mirr");
    
    // Check overvoltage guard logic
    let mut state = SymState::new();
    state.signals.push(("voltage".to_string(), SymValue::Concrete(1450)));
    let guard = prog.module.guards.iter().find(|g| g.name == "over_v").unwrap();
    assert!(sym_eval_expr(&guard.condition, &state).as_bool());

    state.signals.clear();
    state.signals.push(("voltage".to_string(), SymValue::Concrete(1200)));
    assert!(!sym_eval_expr(&guard.condition, &state).as_bool());
}

#[test]
fn test_automotive_brake_behavioral_logic() {
    let src = fs::read_to_string("examples/automotive_brake.mirr").expect("failed to read automotive_brake.mirr");
    let prog = parse_mirr(&src).expect("failed to parse automotive_brake.mirr");
    
    // Check wheel lock guard logic
    let mut state = SymState::new();
    state.signals.push(("wheel_speed_fl".to_string(), SymValue::Concrete(5)));
    let guard = prog.module.guards.iter().find(|g| g.name == "wheel_lock_fl").unwrap();
    assert!(sym_eval_expr(&guard.condition, &state).as_bool());

    state.signals.clear();
    state.signals.push(("wheel_speed_fl".to_string(), SymValue::Concrete(50)));
    assert!(!sym_eval_expr(&guard.condition, &state).as_bool());
}

#[test]
fn test_tmr_voting_system_behavioral_logic() {
    let src = fs::read_to_string("examples/tmr_voting_system.mirr").expect("failed to read tmr_voting_system.mirr");
    let prog = parse_mirr(&src).expect("failed to parse tmr_voting_system.mirr");
    
    // Check sensor failure guard logic
    let mut state = SymState::new();
    state.signals.push(("sensor_a".to_string(), SymValue::Concrete(2)));
    let guard = prog.module.guards.iter().find(|g| g.name == "sensor_a_failed").unwrap();
    assert!(sym_eval_expr(&guard.condition, &state).as_bool());

    state.signals.clear();
    state.signals.push(("sensor_a".to_string(), SymValue::Concrete(10)));
    assert!(!sym_eval_expr(&guard.condition, &state).as_bool());
}

#[test]
fn test_signal_debouncer_behavioral_logic() {
    let src = fs::read_to_string("stdlib/safety/debouncer.mirr").expect("failed to read debouncer.mirr");
    let prog = parse_mirr(&src).expect("failed to parse debouncer.mirr");
    
    // Check guard logic (combinatorial part)
    let mut state = SymState::new();
    state.signals.push(("raw_input".to_string(), SymValue::Concrete(1)));
    let guard = prog.module.guards.iter().find(|g| g.name == "input_high_stable").unwrap();
    assert!(sym_eval_expr(&guard.condition, &state).as_bool());

    state.signals.clear();
    state.signals.push(("raw_input".to_string(), SymValue::Concrete(0)));
    assert!(!sym_eval_expr(&guard.condition, &state).as_bool());
}

#[test]
fn test_heartbeat_monitor_behavioral_logic() {
    let src = fs::read_to_string("stdlib/safety/heartbeat.mirr").expect("failed to read heartbeat.mirr");
    let prog = parse_mirr(&src).expect("failed to parse heartbeat.mirr");
    
    // Check guard logic (combinatorial part)
    let mut state = SymState::new();
    state.signals.push(("heartbeat".to_string(), SymValue::Concrete(0)));
    let guard = prog.module.guards.iter().find(|g| g.name == "heartbeat_missing").unwrap();
    assert!(sym_eval_expr(&guard.condition, &state).as_bool());

    state.signals.clear();
    state.signals.push(("heartbeat".to_string(), SymValue::Concrete(1)));
    assert!(!sym_eval_expr(&guard.condition, &state).as_bool());
}

#[test]
fn test_crc8_checksum_behavioral_logic() {
    let src = fs::read_to_string("stdlib/safety/crc8.mirr").expect("failed to read crc8.mirr");
    let prog = parse_mirr(&src).expect("failed to parse crc8.mirr");
    
    // Check data_received guard
    let mut state = SymState::new();
    state.signals.push(("data_valid".to_string(), SymValue::Concrete(1)));
    let guard = prog.module.guards.iter().find(|g| g.name == "data_received").unwrap();
    assert!(sym_eval_expr(&guard.condition, &state).as_bool());

    state.signals.clear();
    state.signals.push(("data_valid".to_string(), SymValue::Concrete(0)));
    assert!(!sym_eval_expr(&guard.condition, &state).as_bool());
}

/// Helper to convert SymValue to bool for easier assertions.
trait SymValueExt {
    fn as_bool(&self) -> bool;
}

impl SymValueExt for SymValue {
    fn as_bool(&self) -> bool {
        match self {
            SymValue::Concrete(v) => (*v & 1) != 0,
            _ => false,
        }
    }
}
