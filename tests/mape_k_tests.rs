//! Integration tests for Phase 5 — MAPE-K Simulation Harness.
//!
//! Tests the full MAPE-K feedback loop end-to-end, covering:
//! 1. Sensor determinism
//! 2. Monitor windowing
//! 3. LTL property evaluation (Always, EventuallyWithin, Persists)
//! 4. Planner action selection
//! 5. Executor action application
//! 6. Knowledge base audit logging
//! 7. End-to-end scenarios (neonatal respirator, multi-property)
//! 8. Edge cases and boundary conditions

#![forbid(unsafe_code)]

use nasa_rust_project::mape_k::{
    ActionEntry, AdaptationAction, Analyzer, Executor, KnowledgeBase, MapeKSimulator, Monitor,
    Planner, PropertyResult, RingBuffer, SensorConfig, SensorModel, SignalPredicate, SimConfig,
    TemporalProperty, TriggerCondition,
};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// 1. Sensor determinism & fault injection
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn sensor_deterministic_across_runs() {
    let cfg = SensorConfig {
        name: "p".into(),
        base_value: 100,
        noise_amplitude: 10,
        fault_at_tick: None,
        fault_value: 0,
        fault_end_tick: None,
        seed: 12345,
        is_observable: true,
    };
    let mut s1 = SensorModel::new(cfg.clone());
    let mut s2 = SensorModel::new(cfg);
    let run1: Vec<u64> = (0..200).map(|_| s1.sample()).collect();
    let run2: Vec<u64> = (0..200).map(|_| s2.sample()).collect();
    assert_eq!(run1, run2, "same seed must produce identical sequences");
}

#[test]
fn sensor_fault_window_bounded() {
    let cfg = SensorConfig {
        name: "pressure".into(),
        base_value: 120,
        noise_amplitude: 0,
        fault_at_tick: Some(10),
        fault_value: 0,
        fault_end_tick: Some(20),
        seed: 1,
        is_observable: true,
    };
    let mut s = SensorModel::new(cfg);
    // Ticks 0-9: normal (120).
    for tick in 0..10 {
        assert_eq!(s.sample(), 120, "tick {tick} should be normal");
    }
    // Ticks 10-19: fault (0).
    for tick in 10..20 {
        assert_eq!(s.sample(), 0, "tick {tick} should be fault");
    }
    // Tick 20+: recovered (120).
    assert_eq!(s.sample(), 120, "tick 20 should be recovered");
}

#[test]
fn sensor_permanent_fault() {
    let cfg = SensorConfig {
        name: "p".into(),
        base_value: 100,
        noise_amplitude: 0,
        fault_at_tick: Some(5),
        fault_value: 999,
        fault_end_tick: None,
        seed: 1,
        is_observable: true,
    };
    let mut s = SensorModel::new(cfg);
    for _ in 0..5 {
        assert_eq!(s.sample(), 100);
    }
    // Permanent fault: should be 999 forever.
    for _ in 5..100 {
        assert_eq!(s.sample(), 999);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. Monitor windowing
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn ring_buffer_overwrites_oldest() {
    let mut rb = RingBuffer::new(4);
    for i in 0..10 {
        rb.push(i);
    }
    assert_eq!(rb.len(), 4);
    let vals: Vec<u64> = rb.iter().collect();
    assert_eq!(vals, vec![6, 7, 8, 9]);
}

#[test]
fn monitor_window_tracks_signal() {
    let mut mon = Monitor::new(8, &["hr", "bp"]);
    mon.record_sample("hr", 72);
    mon.record_sample("bp", 120);
    mon.advance_tick();
    mon.record_sample("hr", 75);
    mon.record_sample("bp", 118);
    mon.advance_tick();

    let hr_win = mon.window("hr").unwrap();
    assert_eq!(hr_win.len(), 2);
    assert_eq!(hr_win.get(0), Some(72));
    assert_eq!(hr_win.get(1), Some(75));
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. LTL property evaluation
// ═══════════════════════════════════════════════════════════════════════════

fn build_monitor(signal: &str, values: &[u64]) -> Monitor {
    let mut mon = Monitor::new(64, &[signal]);
    for &v in values {
        mon.record_sample(signal, v);
        mon.advance_tick();
    }
    mon
}

#[test]
fn always_satisfied_all_above_threshold() {
    let mon = build_monitor("p", &[80, 90, 100, 110]);
    let a =
        Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan("p".into(), 50))]);
    let results = a.evaluate(&mon);
    assert!(results[0].satisfied);
}

#[test]
fn always_violated_single_dip() {
    let mon = build_monitor("p", &[80, 90, 30, 110]);
    let a =
        Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan("p".into(), 50))]);
    let results = a.evaluate(&mon);
    assert!(!results[0].satisfied);
    assert_eq!(results[0].evidence_tick, Some(2));
}

#[test]
fn eventually_within_found_in_window() {
    let mon = build_monitor("flag", &[0, 0, 1, 0, 0]);
    let a = Analyzer::new(vec![TemporalProperty::EventuallyWithin(
        SignalPredicate::IsTrue("flag".into()),
        5,
    )]);
    let results = a.evaluate(&mon);
    assert!(results[0].satisfied);
}

#[test]
fn eventually_within_outside_deadline() {
    // True at tick 0, deadline is 2, only checks last 2 ticks.
    let mon = build_monitor("f", &[1, 0, 0, 0]);
    let a = Analyzer::new(vec![TemporalProperty::EventuallyWithin(
        SignalPredicate::IsTrue("f".into()),
        2,
    )]);
    let results = a.evaluate(&mon);
    assert!(!results[0].satisfied);
}

#[test]
fn persists_three_consecutive() {
    let mon = build_monitor("x", &[0, 50, 60, 70, 0]);
    let a = Analyzer::new(vec![TemporalProperty::Persists(
        SignalPredicate::GreaterThan("x".into(), 40),
        3,
    )]);
    let results = a.evaluate(&mon);
    assert!(results[0].satisfied);
}

#[test]
fn persists_interrupted_fails() {
    let mon = build_monitor("x", &[50, 60, 10, 70, 80]);
    let a = Analyzer::new(vec![TemporalProperty::Persists(
        SignalPredicate::GreaterThan("x".into(), 40),
        3,
    )]);
    let results = a.evaluate(&mon);
    assert!(!results[0].satisfied);
}

#[test]
fn in_range_predicate_works() {
    let mon = build_monitor("temp", &[35, 36, 37, 38, 39]);
    let a = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::InRange(
        "temp".into(),
        36,
        38,
    ))]);
    let results = a.evaluate(&mon);
    assert!(!results[0].satisfied); // 35 and 39 are out of range
    assert_eq!(results[0].evidence_tick, Some(0));
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Planner action selection
// ═══════════════════════════════════════════════════════════════════════════

fn violation(idx: usize) -> PropertyResult {
    PropertyResult { property_idx: idx, satisfied: false, evidence_tick: Some(0) }
}

#[test]
fn planner_selects_highest_priority() {
    let planner = Planner::new(vec![
        ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "a".into(), value: 1 },
            priority: 5,
            trigger_on: TriggerCondition::OnViolation,
        },
        ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 100,
            trigger_on: TriggerCondition::OnViolation,
        },
    ]);
    let result = planner.select(&[violation(0)]);
    assert_eq!(result.action, Some(AdaptationAction::EmergencyStop));
}

#[test]
fn planner_no_match_returns_none() {
    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 5,
        action: AdaptationAction::EmergencyStop,
        priority: 10,
        trigger_on: TriggerCondition::OnViolation,
    }]);
    let result = planner.select(&[violation(0)]);
    assert_eq!(result.action, None);
}

#[test]
fn planner_multiple_violations_picks_best() {
    let planner = Planner::new(vec![
        ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "a".into(), value: 1 },
            priority: 10,
            trigger_on: TriggerCondition::OnViolation,
        },
        ActionEntry {
            trigger_property_idx: 1,
            action: AdaptationAction::EmergencyStop,
            priority: 50,
            trigger_on: TriggerCondition::OnViolation,
        },
    ]);
    let result = planner.select(&[violation(0), violation(1)]);
    assert_eq!(result.action, Some(AdaptationAction::EmergencyStop));
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. Executor action application
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn executor_set_signal() {
    let mut exec = Executor::new(vec!["alarm".into(), "valve".into()]);
    let mut env = HashMap::from([("alarm".into(), 0u64), ("valve".into(), 0u64)]);
    let rec = exec.apply(&AdaptationAction::SetSignal { name: "alarm".into(), value: 1 }, &mut env);
    assert!(rec.success);
    assert_eq!(env["alarm"], 1);
}

#[test]
fn executor_emergency_stop_zeros_all_signals() {
    let mut exec = Executor::new(vec!["a".into(), "b".into()]);
    let mut env = HashMap::from([("a".into(), 42u64), ("b".into(), 99u64)]);
    let rec = exec.apply(&AdaptationAction::EmergencyStop, &mut env);
    assert!(rec.success);
    assert!(exec.is_emergency_active());
    assert_eq!(env["a"], 0);
    assert_eq!(env["b"], 0);
}

#[test]
fn executor_unknown_signal_fails_gracefully() {
    let mut exec = Executor::new(vec!["known".into()]);
    let mut env = HashMap::from([("known".into(), 0u64)]);
    let rec =
        exec.apply(&AdaptationAction::SetSignal { name: "unknown".into(), value: 1 }, &mut env);
    assert!(!rec.success);
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Knowledge base audit logging
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn knowledge_base_records_and_retrieves() {
    let mut kb = KnowledgeBase::new(100);
    let rec = nasa_rust_project::mape_k::knowledge::AdaptationRecord {
        tick: 42,
        trigger_property_idx: 0,
        trigger_description: "test".into(),
        action: AdaptationAction::EmergencyStop,
        success: true,
        pre_state: vec![("p".into(), 120)],
        post_state: vec![("p".into(), 0)],
    };
    kb.record(rec);
    assert_eq!(kb.len(), 1);
    assert_eq!(kb.records()[0].tick, 42);
}

#[test]
fn knowledge_base_evicts_oldest_at_capacity() {
    let mut kb = KnowledgeBase::new(3);
    for i in 0..5 {
        let rec = nasa_rust_project::mape_k::knowledge::AdaptationRecord {
            tick: i,
            trigger_property_idx: 0,
            trigger_description: "test".into(),
            action: AdaptationAction::EmergencyStop,
            success: true,
            pre_state: vec![],
            post_state: vec![],
        };
        kb.record(rec);
    }
    assert_eq!(kb.len(), 3);
    assert_eq!(kb.records()[0].tick, 2); // oldest surviving
    assert_eq!(kb.total_recorded(), 5);
}

#[test]
fn knowledge_base_serializes_to_json() {
    let mut kb = KnowledgeBase::new(10);
    let rec = nasa_rust_project::mape_k::knowledge::AdaptationRecord {
        tick: 7,
        trigger_property_idx: 0,
        trigger_description: "pressure drop".into(),
        action: AdaptationAction::SetSignal { name: "valve".into(), value: 1 },
        success: true,
        pre_state: vec![("valve".into(), 0)],
        post_state: vec![("valve".into(), 1)],
    };
    kb.record(rec);
    let json = kb.to_json().expect("serialization should succeed");
    assert!(json.contains("\"tick\": 7"));
    assert!(json.contains("pressure drop"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 7. End-to-end simulation scenarios
// ═══════════════════════════════════════════════════════════════════════════

fn neonatal_config() -> SimConfig {
    SimConfig {
        sensors: vec![SensorConfig {
            name: "airway_pressure".into(),
            base_value: 120,
            noise_amplitude: 5,
            fault_at_tick: None,
            fault_value: 0,
            fault_end_tick: None,
            seed: 42,
            is_observable: true,
        }],
        properties: vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "airway_pressure".into(),
            50,
        ))],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "airway_pressure".into(), value: 1 },
            priority: 10,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 64,
        knowledge_capacity: 100,
    }
}

#[test]
fn neonatal_normal_no_violations() {
    let config = neonatal_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(1000);
    assert_eq!(result.total_ticks, 1000);
    assert_eq!(result.total_violations, 0);
    assert_eq!(result.total_adaptations, 0);
    assert!(!result.emergency_triggered);
}

#[test]
fn neonatal_fault_triggers_adaptation() {
    let mut config = neonatal_config();
    config.sensors[0].fault_at_tick = Some(100);
    config.sensors[0].fault_value = 10; // below threshold of 50

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(200);

    // Should detect violations starting at tick 100.
    assert!(result.total_violations > 0, "violations expected after fault");
    assert!(result.total_adaptations > 0, "adaptations expected");
    assert!(!result.adaptation_log.is_empty(), "audit log should have entries");
}

#[test]
fn neonatal_fault_and_recovery() {
    let mut config = neonatal_config();
    config.sensors[0].fault_at_tick = Some(50);
    config.sensors[0].fault_value = 10;
    config.sensors[0].fault_end_tick = Some(70); // fault clears at tick 70

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(200);

    // Violations happen during fault window, adaptations triggered.
    assert!(result.total_violations > 0);
    assert!(result.total_adaptations > 0);
    // Simulation runs to completion (no emergency stop in this config).
    assert_eq!(result.total_ticks, 200);
}

#[test]
fn emergency_stop_halts_simulation() {
    let config = SimConfig {
        sensors: vec![SensorConfig {
            name: "pressure".into(),
            base_value: 120,
            noise_amplitude: 0,
            fault_at_tick: Some(10),
            fault_value: 0,
            fault_end_tick: None,
            seed: 1,
            is_observable: true,
        }],
        properties: vec![TemporalProperty::Persists(
            SignalPredicate::LessThan("pressure".into(), 50),
            5,
        )],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 100,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 32,
        knowledge_capacity: 100,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(1000);

    // Emergency stop should halt before 1000 ticks.
    assert!(result.emergency_triggered);
    assert!(result.total_ticks < 1000);
    assert!(result.emergency_tick.is_some());
}

#[test]
fn multi_property_multi_action_scenario() {
    let config = SimConfig {
        sensors: vec![
            SensorConfig {
                name: "heart_rate".into(),
                base_value: 72,
                noise_amplitude: 3,
                fault_at_tick: Some(50),
                fault_value: 30, // bradycardia
                fault_end_tick: Some(80),
                seed: 10,
                is_observable: true,
            },
            SensorConfig {
                name: "blood_pressure".into(),
                base_value: 120,
                noise_amplitude: 5,
                fault_at_tick: None,
                fault_value: 0,
                fault_end_tick: None,
                seed: 20,
                is_observable: true,
            },
        ],
        properties: vec![
            TemporalProperty::Always(SignalPredicate::GreaterThan("heart_rate".into(), 50)),
            TemporalProperty::Always(SignalPredicate::GreaterThan("blood_pressure".into(), 80)),
        ],
        action_table: vec![
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::SetSignal { name: "heart_rate".into(), value: 1 },
                priority: 50,
                trigger_on: TriggerCondition::OnViolation,
            },
            ActionEntry {
                trigger_property_idx: 1,
                action: AdaptationAction::EmergencyStop,
                priority: 100,
                trigger_on: TriggerCondition::OnViolation,
            },
        ],
        window_size: 32,
        knowledge_capacity: 200,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(200);

    // Heart rate fault at ticks 50-80 should trigger adaptations.
    assert!(result.total_violations > 0);
    assert!(result.total_adaptations > 0);
}

// ═══════════════════════════════════════════════════════════════════════════
// 8. Edge cases and boundary conditions
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn zero_tick_simulation() {
    let config = neonatal_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(0);
    assert_eq!(result.total_ticks, 0);
    assert_eq!(result.total_violations, 0);
}

#[test]
fn single_tick_simulation() {
    let config = neonatal_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(1);
    assert_eq!(result.total_ticks, 1);
}

#[test]
fn empty_action_table_no_panic() {
    let config = SimConfig {
        sensors: vec![SensorConfig {
            name: "s".into(),
            base_value: 10,
            noise_amplitude: 0,
            fault_at_tick: Some(0),
            fault_value: 0,
            fault_end_tick: None,
            seed: 1,
            is_observable: true,
        }],
        properties: vec![TemporalProperty::Always(SignalPredicate::IsTrue("s".into()))],
        action_table: vec![], // no actions defined
        window_size: 8,
        knowledge_capacity: 10,
    };
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(100);
    // Violations detected, but no adaptations (no matching actions).
    assert!(result.total_violations > 0);
    assert_eq!(result.total_adaptations, 0);
}

#[test]
fn sim_result_summary_format() {
    let mut config = neonatal_config();
    config.sensors[0].fault_at_tick = Some(5);
    config.sensors[0].fault_value = 0;
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(50);
    let summary = result.summary();
    assert!(summary.contains("MAPE-K Simulation:"));
    assert!(summary.contains("Violations detected:"));
    assert!(summary.contains("Adaptations applied:"));
}

#[test]
fn adaptation_log_serializable_to_json() {
    let mut config = neonatal_config();
    config.sensors[0].fault_at_tick = Some(5);
    config.sensors[0].fault_value = 0;
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(20);
    let json = serde_json::to_string_pretty(&result.adaptation_log);
    assert!(json.is_ok(), "adaptation log should serialize to JSON");
}

#[test]
fn switch_mode_action_records_correctly() {
    let mut exec = Executor::new(vec!["s".into()]);
    let mut env = HashMap::from([("s".into(), 42u64)]);
    let rec =
        exec.apply(&AdaptationAction::SwitchMode { mode_name: "high_precision".into() }, &mut env);
    assert!(rec.success);
    // Signal state should not change on mode switch.
    assert_eq!(env["s"], 42);
}
