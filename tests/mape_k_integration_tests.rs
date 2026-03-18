//! Comprehensive integration tests for the MAPE-K feedback loop.
//!
//! Tests cross-module interactions spanning ALL files in `src/mape_k/`:
//!   sensor.rs, monitor.rs, analyzer.rs, planner.rs, executor.rs,
//!   knowledge.rs, ltl.rs, mod.rs (MapeKSimulator), bridge.rs
//!
//! Focus areas:
//! 1. Sensor -> Monitor -> Analyzer -> Planner -> Executor -> Knowledge chain
//! 2. Bridge integration (PipelineResult -> SimConfig)
//! 3. Full pipeline -> bridge -> MAPE-K simulator end-to-end
//! 4. Multi-signal multi-property cross-module stress
//! 5. OnSatisfaction vs OnViolation trigger conditions in full loop
//! 6. Knowledge base audit trail correctness through the full chain
//! 7. Edge cases in cross-module boundaries

#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::useless_vec)]

use std::collections::HashMap;

use nasa_rust_project::mape_k::bridge::{
    bridge_from_pipeline, BridgeError, DEFAULT_KNOWLEDGE_CAPACITY, DEFAULT_WINDOW_SIZE,
};
use nasa_rust_project::mape_k::{
    ActionEntry, AdaptationAction, AdaptationRecord, Analyzer, ExecutionRecord, Executor,
    KnowledgeBase, MapeKSimulator, Monitor, Planner, PropertyResult, RingBuffer, SensorConfig,
    SensorModel, SignalPredicate, SimConfig, TemporalProperty, TriggerCondition,
};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

// ═══════════════════════════════════════════════════════════════════════════
// Constants — bounded iteration limits (NASA P10)
// ═══════════════════════════════════════════════════════════════════════════

const MAX_TEST_TICKS: usize = 500;
const MAX_TEST_RECORDS: usize = 256;

// ═══════════════════════════════════════════════════════════════════════════
// Helper: wrap a property body in a valid MIRR module
// ═══════════════════════════════════════════════════════════════════════════

/// Build a minimal valid MIRR module source string containing the given
/// property declaration. Ensures 1 input, 1 output, 1 guard, 1 reflex.
fn mirr_module_with_property(property_body: &str) -> String {
    format!(
        r#"
module mape_test {{
    signal x: in bool;
    signal y: out bool;

    guard g {{
        when x
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            y = true;
        }}
    }}

    {property_body}
}}
"#
    )
}

/// Build a MIRR module with a typed input signal and property.
fn mirr_module_with_typed_input(signal_decl: &str, property_body: &str) -> String {
    format!(
        r#"
module mape_typed {{
    {signal_decl}
    signal out_flag: out bool;

    guard g_typed {{
        when out_flag
        for 1 cycles;
    }}

    reflex r_typed {{
        on g_typed {{
            out_flag = true;
        }}
    }}

    {property_body}
}}
"#
    )
}

/// Build a minimal SensorConfig for testing.
fn test_sensor(name: &str, base: u64, noise: u64, seed: u64) -> SensorConfig {
    SensorConfig {
        name: name.to_string(),
        base_value: base,
        noise_amplitude: noise,
        fault_at_tick: None,
        fault_value: 0,
        fault_end_tick: None,
        seed,
        is_observable: true,
    }
}

/// Build a SensorConfig with fault injection.
fn test_sensor_with_fault(
    name: &str,
    base: u64,
    fault_tick: u64,
    fault_val: u64,
    fault_end: Option<u64>,
    seed: u64,
) -> SensorConfig {
    SensorConfig {
        name: name.to_string(),
        base_value: base,
        noise_amplitude: 0,
        fault_at_tick: Some(fault_tick),
        fault_value: fault_val,
        fault_end_tick: fault_end,
        seed,
        is_observable: true,
    }
}

/// Build a PropertyResult for testing planner integration.
fn make_result(idx: usize, satisfied: bool) -> PropertyResult {
    PropertyResult { property_idx: idx, satisfied, evidence_tick: Some(0) }
}

/// Build an AdaptationRecord for testing knowledge base integration.
fn make_adaptation(tick: u64, prop_idx: usize, action: AdaptationAction) -> AdaptationRecord {
    AdaptationRecord {
        tick,
        trigger_property_idx: prop_idx,
        trigger_description: format!("prop_{prop_idx}"),
        action,
        success: true,
        pre_state: vec![("sig".to_string(), 100)],
        post_state: vec![("sig".to_string(), 0)],
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 1. Sensor -> Monitor cross-module integration
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn sensor_samples_feed_into_monitor_window() {
    let cfg = test_sensor("pressure", 120, 0, 42);
    let mut sensor = SensorModel::new(cfg);
    let mut monitor = Monitor::new(32, &["pressure"]);

    for _tick in 0..MAX_TEST_TICKS.min(50) {
        let value = sensor.sample();
        monitor.record_sample("pressure", value);
        monitor.advance_tick();
    }

    let window = monitor.window("pressure").expect("pressure window should exist in monitor");
    assert_eq!(window.len(), 32, "monitor window should be capped at configured window_size=32");

    // With zero noise, all values should be exactly base_value.
    for i in 0..window.len() {
        let v = window.get(i).expect("index within window length should return Some");
        assert_eq!(v, 120, "zero-noise sensor should produce base_value=120 at every tick");
    }
}

#[test]
fn multiple_sensors_feed_independent_monitor_windows() {
    let sensors_cfg = vec![
        test_sensor("temperature", 37, 0, 10),
        test_sensor("pressure", 120, 0, 20),
        test_sensor("heart_rate", 72, 0, 30),
    ];
    // Build monitor first while we can still borrow sensors_cfg for names.
    // Monitor::new clones the names into its internal HashMap, so the borrow
    // is released before into_iter() consumes sensors_cfg.
    let name_refs: Vec<&str> = sensors_cfg.iter().map(|c| c.name.as_str()).collect();
    let mut monitor = Monitor::new(16, &name_refs);
    let mut sensors: Vec<SensorModel> = sensors_cfg.into_iter().map(SensorModel::new).collect();

    for _tick in 0..MAX_TEST_TICKS.min(20) {
        for sensor in &mut sensors {
            let value = sensor.sample();
            monitor.record_sample(sensor.name(), value);
        }
        monitor.advance_tick();
    }

    // Each window should have exactly 16 entries (window_size).
    let temp_win = monitor.window("temperature").expect("temperature window should exist");
    let pres_win = monitor.window("pressure").expect("pressure window should exist");
    let hr_win = monitor.window("heart_rate").expect("heart_rate window should exist");

    assert_eq!(temp_win.len(), 16, "temperature window should fill to capacity");
    assert_eq!(pres_win.len(), 16, "pressure window should fill to capacity");
    assert_eq!(hr_win.len(), 16, "heart_rate window should fill to capacity");

    // Check distinct base values preserved.
    assert_eq!(temp_win.get(0).unwrap(), 37, "temperature sensor zero-noise should yield 37");
    assert_eq!(pres_win.get(0).unwrap(), 120, "pressure sensor zero-noise should yield 120");
    assert_eq!(hr_win.get(0).unwrap(), 72, "heart_rate sensor zero-noise should yield 72");
}

#[test]
fn sensor_fault_propagates_through_monitor_to_window() {
    let cfg = test_sensor_with_fault("pressure", 120, 5, 0, Some(10), 1);
    let mut sensor = SensorModel::new(cfg);
    let mut monitor = Monitor::new(64, &["pressure"]);

    let mut fault_values_seen = 0u64;
    for _tick in 0..MAX_TEST_TICKS.min(15) {
        let value = sensor.sample();
        monitor.record_sample("pressure", value);
        monitor.advance_tick();
        if value == 0 {
            fault_values_seen += 1;
        }
    }

    assert!(
        fault_values_seen > 0,
        "sensor fault should produce zero values visible in monitor window"
    );

    let window = monitor.window("pressure").unwrap();
    // Window should contain the full 15-tick history.
    assert_eq!(window.len(), 15, "15 ticks should be recorded in the window");

    // Ticks 5-9 (indices 5-9 in window) should be fault value 0.
    for i in 5..10 {
        assert_eq!(window.get(i).unwrap(), 0, "fault window tick {i} should have value 0");
    }
    // Ticks 10-14 should be recovered to base value 120.
    for i in 10..15 {
        assert_eq!(
            window.get(i).unwrap(),
            120,
            "post-fault tick {i} should recover to base value 120"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. Monitor -> Analyzer cross-module integration
// ═══════════════════════════════════════════════════════════════════════════

/// Helper: build a monitor from sensor samples.
fn build_monitor_from_sensor(
    name: &str,
    sensor_cfg: SensorConfig,
    ticks: usize,
    window_size: usize,
) -> Monitor {
    let mut sensor = SensorModel::new(sensor_cfg);
    let mut monitor = Monitor::new(window_size, &[name]);
    let bounded_ticks = ticks.min(MAX_TEST_TICKS);
    for _t in 0..bounded_ticks {
        let v = sensor.sample();
        monitor.record_sample(name, v);
        monitor.advance_tick();
    }
    monitor
}

#[test]
fn analyzer_detects_violation_from_sensor_fault_through_monitor() {
    // Sensor faults at tick 10, producing value 0 (below threshold of 50).
    let cfg = test_sensor_with_fault("pressure", 120, 10, 0, None, 1);
    let monitor = build_monitor_from_sensor("pressure", cfg, 20, 64);

    let analyzer = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
        "pressure".to_string(),
        50,
    ))]);

    let results = analyzer.evaluate(&monitor);
    assert_eq!(results.len(), 1, "should have exactly one property result");
    assert!(
        !results[0].satisfied,
        "Always(pressure > 50) should be violated when fault injects value 0"
    );
}

#[test]
fn analyzer_reports_satisfaction_when_sensor_stays_normal() {
    let cfg = test_sensor("pressure", 120, 0, 42);
    let monitor = build_monitor_from_sensor("pressure", cfg, 50, 32);

    let analyzer = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
        "pressure".to_string(),
        50,
    ))]);

    let results = analyzer.evaluate(&monitor);
    assert!(
        results[0].satisfied,
        "Always(pressure > 50) should be satisfied when sensor stays at 120"
    );
}

#[test]
fn analyzer_eventually_within_detects_transient_fault_recovery() {
    // Sensor faults at tick 5 (value=0), recovers at tick 8 (value=120).
    let cfg = test_sensor_with_fault("signal_a", 120, 5, 0, Some(8), 1);
    let monitor = build_monitor_from_sensor("signal_a", cfg, 15, 64);

    let analyzer = Analyzer::new(vec![TemporalProperty::EventuallyWithin(
        SignalPredicate::GreaterThan("signal_a".to_string(), 100),
        10,
    )]);

    let results = analyzer.evaluate(&monitor);
    assert!(
        results[0].satisfied,
        "EventuallyWithin(signal_a > 100, 10) should be satisfied after fault recovers"
    );
}

#[test]
fn analyzer_persists_detects_sustained_fault_condition() {
    // Sensor faults permanently at tick 3, value drops to 10.
    let cfg = test_sensor_with_fault("temp", 100, 3, 10, None, 1);
    let monitor = build_monitor_from_sensor("temp", cfg, 20, 64);

    // Check if the LOW value persists for 5 ticks.
    let analyzer = Analyzer::new(vec![TemporalProperty::Persists(
        SignalPredicate::LessThan("temp".to_string(), 50),
        5,
    )]);

    let results = analyzer.evaluate(&monitor);
    assert!(
        results[0].satisfied,
        "Persists(temp < 50, 5) should be satisfied when fault value 10 persists beyond 5 ticks"
    );
}

#[test]
fn analyzer_multiple_properties_on_same_monitor() {
    let cfg = test_sensor_with_fault("pressure", 120, 10, 0, None, 1);
    let monitor = build_monitor_from_sensor("pressure", cfg, 25, 64);

    let analyzer = Analyzer::new(vec![
        TemporalProperty::Always(SignalPredicate::GreaterThan("pressure".to_string(), 50)),
        TemporalProperty::EventuallyWithin(
            SignalPredicate::GreaterThan("pressure".to_string(), 100),
            5,
        ),
        TemporalProperty::Persists(SignalPredicate::LessThan("pressure".to_string(), 10), 3),
    ]);

    let results = analyzer.evaluate(&monitor);
    assert_eq!(results.len(), 3, "analyzer should return one result per property");
    assert!(!results[0].satisfied, "Always(pressure > 50) should be violated after fault");
    // EventuallyWithin checks last 5 ticks — all faulted at 0, so not > 100.
    assert!(
        !results[1].satisfied,
        "EventuallyWithin(pressure > 100, 5) should fail when last 5 ticks are faulted"
    );
    // Persists(pressure < 10) — fault value is 0, which is < 10, for 15 consecutive ticks.
    assert!(
        results[2].satisfied,
        "Persists(pressure < 10, 3) should be satisfied with 15 consecutive fault ticks"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. Analyzer -> Planner cross-module integration
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn planner_selects_action_from_analyzer_violation() {
    // Build a monitor with a faulted sensor.
    let cfg = test_sensor_with_fault("p", 120, 0, 0, None, 1);
    let monitor = build_monitor_from_sensor("p", cfg, 10, 32);

    let analyzer = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
        "p".to_string(),
        50,
    ))]);

    let results = analyzer.evaluate(&monitor);
    assert!(!results[0].satisfied, "property should be violated for planner test setup");

    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::EmergencyStop,
        priority: 100,
        trigger_on: TriggerCondition::OnViolation,
    }]);

    let plan = planner.select(&results);
    assert_eq!(
        plan.action,
        Some(AdaptationAction::EmergencyStop),
        "planner should select EmergencyStop when property 0 is violated"
    );
    assert_eq!(
        plan.trigger_property_idx,
        Some(0),
        "planner should report the triggering property index"
    );
}

#[test]
fn planner_selects_on_satisfaction_trigger() {
    // Build a monitor where a dangerous condition IS satisfied.
    let cfg = test_sensor_with_fault("overheat", 200, 0, 999, None, 1);
    let monitor = build_monitor_from_sensor("overheat", cfg, 10, 32);

    // Property: "overheat persists above 500 for 3 ticks" — a dangerous condition.
    let analyzer = Analyzer::new(vec![TemporalProperty::Persists(
        SignalPredicate::GreaterThan("overheat".to_string(), 500),
        3,
    )]);

    let results = analyzer.evaluate(&monitor);
    assert!(
        results[0].satisfied,
        "Persists(overheat > 500, 3) should be satisfied with all values at 999"
    );

    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::EmergencyStop,
        priority: 255,
        trigger_on: TriggerCondition::OnSatisfaction,
    }]);

    let plan = planner.select(&results);
    assert_eq!(
        plan.action,
        Some(AdaptationAction::EmergencyStop),
        "planner should fire on satisfaction when trigger_on is OnSatisfaction"
    );
}

#[test]
fn planner_priority_resolution_across_multiple_analyzer_results() {
    let results = vec![
        make_result(0, false), // violated
        make_result(1, false), // violated
    ];

    let planner = Planner::new(vec![
        ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "valve".to_string(), value: 1 },
            priority: 10,
            trigger_on: TriggerCondition::OnViolation,
        },
        ActionEntry {
            trigger_property_idx: 1,
            action: AdaptationAction::EmergencyStop,
            priority: 200,
            trigger_on: TriggerCondition::OnViolation,
        },
    ]);

    let plan = planner.select(&results);
    assert_eq!(
        plan.action,
        Some(AdaptationAction::EmergencyStop),
        "higher priority action (200) should beat lower priority (10)"
    );
    assert_eq!(
        plan.trigger_property_idx,
        Some(1),
        "winning action should report its trigger property index"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Planner -> Executor cross-module integration
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn executor_applies_planner_set_signal_action() {
    let results = vec![make_result(0, false)];

    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::SetSignal { name: "valve".to_string(), value: 1 },
        priority: 50,
        trigger_on: TriggerCondition::OnViolation,
    }]);

    let plan = planner.select(&results);
    let selected_action = plan.action.as_ref().expect("planner should select an action");

    let mut executor = Executor::new(vec!["valve".to_string(), "alarm".to_string()]);
    let mut env = HashMap::from([("valve".to_string(), 0u64), ("alarm".to_string(), 0u64)]);

    let record = executor.apply(selected_action, &mut env);
    assert!(record.success, "executor should successfully apply SetSignal(valve=1)");
    assert_eq!(env["valve"], 1, "valve signal should be set to 1 after executor applies action");
    assert_eq!(env["alarm"], 0, "alarm signal should remain unchanged when only valve is targeted");
}

#[test]
fn executor_applies_planner_emergency_stop() {
    let results = vec![make_result(0, false)];

    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::EmergencyStop,
        priority: 255,
        trigger_on: TriggerCondition::OnViolation,
    }]);

    let plan = planner.select(&results);
    let selected_action = plan.action.as_ref().expect("planner should select emergency stop");

    let mut executor = Executor::new(vec!["sig_a".to_string(), "sig_b".to_string()]);
    let mut env = HashMap::from([("sig_a".to_string(), 42u64), ("sig_b".to_string(), 99u64)]);

    let record = executor.apply(selected_action, &mut env);
    assert!(record.success, "emergency stop should succeed");
    assert!(executor.is_emergency_active(), "emergency flag should be active after EmergencyStop");
    assert_eq!(env["sig_a"], 0, "all signals should be zeroed after emergency stop");
    assert_eq!(env["sig_b"], 0, "all signals should be zeroed after emergency stop");
}

#[test]
fn executor_switch_mode_preserves_signal_state() {
    let results = vec![make_result(0, false)];

    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::SwitchMode { mode_name: "safe_mode".to_string() },
        priority: 100,
        trigger_on: TriggerCondition::OnViolation,
    }]);

    let plan = planner.select(&results);
    let selected_action = plan.action.as_ref().expect("planner should select switch mode");

    let mut executor = Executor::new(vec!["signal_x".to_string()]);
    let mut env = HashMap::from([("signal_x".to_string(), 42u64)]);

    let record = executor.apply(selected_action, &mut env);
    assert!(record.success, "SwitchMode should succeed");
    assert_eq!(env["signal_x"], 42, "SwitchMode should not change signal values");
    assert!(!executor.is_emergency_active(), "SwitchMode should not trigger emergency state");
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. Executor -> Knowledge cross-module integration
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn execution_record_feeds_into_knowledge_base() {
    let mut executor = Executor::new(vec!["pressure".to_string()]);
    let mut env = HashMap::from([("pressure".to_string(), 120u64)]);

    let action = AdaptationAction::SetSignal { name: "pressure".to_string(), value: 80 };
    let exec_record = executor.apply(&action, &mut env);

    // Build adaptation record from execution record (the bridge between executor and knowledge).
    let adaptation = AdaptationRecord::from_execution(42, 0, "pressure_low", &exec_record);

    let mut kb = KnowledgeBase::new(100);
    kb.record(adaptation);

    assert_eq!(kb.len(), 1, "knowledge base should have one record");
    let stored = &kb.records()[0];
    assert_eq!(stored.tick, 42, "stored record should have tick=42");
    assert_eq!(stored.trigger_property_idx, 0, "stored record should reference property index 0");
    assert!(stored.success, "stored record should reflect successful execution");
    assert_eq!(
        stored.pre_state,
        vec![("pressure".to_string(), 120)],
        "pre_state should show original value"
    );
    assert_eq!(
        stored.post_state,
        vec![("pressure".to_string(), 80)],
        "post_state should show updated value"
    );
}

#[test]
fn knowledge_base_audit_trail_through_multiple_adaptations() {
    let mut executor = Executor::new(vec!["valve".to_string(), "alarm".to_string()]);
    let mut env = HashMap::from([("valve".to_string(), 0u64), ("alarm".to_string(), 0u64)]);
    let mut kb = KnowledgeBase::new(100);

    // Simulate a sequence of adaptations.
    let actions = vec![
        (5u64, AdaptationAction::SetSignal { name: "valve".to_string(), value: 1 }),
        (10, AdaptationAction::SetSignal { name: "alarm".to_string(), value: 1 }),
        (15, AdaptationAction::EmergencyStop),
    ];

    for (tick, action) in actions.iter().take(MAX_TEST_RECORDS.min(3)) {
        let exec_record = executor.apply(action, &mut env);
        let adaptation = AdaptationRecord::from_execution(*tick, 0, &action.label(), &exec_record);
        kb.record(adaptation);
    }

    assert_eq!(kb.len(), 3, "knowledge base should contain all 3 adaptation records");
    assert_eq!(kb.records()[0].tick, 5, "first adaptation should be at tick 5");
    assert_eq!(kb.records()[1].tick, 10, "second adaptation should be at tick 10");
    assert_eq!(kb.records()[2].tick, 15, "third adaptation should be at tick 15");

    // Verify JSON serialization of the audit trail.
    let json = kb.to_json().expect("knowledge base should serialize to JSON");
    assert!(json.contains("\"tick\": 5"), "JSON should contain tick 5 entry");
    assert!(json.contains("EmergencyStop"), "JSON should contain emergency stop action");
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Full chain: Sensor -> Monitor -> Analyzer -> Planner -> Executor -> Knowledge
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn full_mape_chain_manual_orchestration() {
    // Setup: sensor with fault at tick 5.
    let sensor_cfg = test_sensor_with_fault("pressure", 120, 5, 10, None, 42);
    let mut sensor = SensorModel::new(sensor_cfg);
    let mut monitor = Monitor::new(32, &["pressure"]);
    let analyzer = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
        "pressure".to_string(),
        50,
    ))]);
    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::SetSignal { name: "pressure".to_string(), value: 80 },
        priority: 100,
        trigger_on: TriggerCondition::OnViolation,
    }]);
    let mut executor = Executor::new(vec!["pressure".to_string()]);
    let mut signal_env = HashMap::from([("pressure".to_string(), 0u64)]);
    let mut kb = KnowledgeBase::new(100);

    let mut total_violations = 0u64;
    let mut total_adaptations = 0u64;

    // Run the MAPE-K loop manually for 20 ticks.
    for tick in 0..MAX_TEST_TICKS.min(20) {
        // M — Monitor: sample sensor.
        let value = sensor.sample();
        monitor.record_sample("pressure", value);
        signal_env.insert("pressure".to_string(), value);

        // A — Analyze: check properties.
        let results = analyzer.evaluate(&monitor);
        let violations: Vec<&PropertyResult> = results.iter().filter(|r| !r.satisfied).collect();

        if !violations.is_empty() {
            total_violations += 1;
        }

        // P — Plan: select action.
        let plan = planner.select(&results);

        // E — Execute: apply action.
        if let Some(ref action) = plan.action {
            let exec_record = executor.apply(action, &mut signal_env);

            // K — Knowledge: record adaptation.
            let adaptation = AdaptationRecord::from_execution(
                tick as u64,
                plan.trigger_property_idx.unwrap_or(0),
                &action.label(),
                &exec_record,
            );
            kb.record(adaptation);
            total_adaptations += 1;
        }

        monitor.advance_tick();
    }

    assert!(
        total_violations > 0,
        "manual MAPE chain should detect violations after sensor fault at tick 5"
    );
    assert!(
        total_adaptations > 0,
        "manual MAPE chain should produce adaptations in response to violations"
    );
    assert!(!kb.is_empty(), "knowledge base should have audit records after adaptations");
    assert_eq!(
        kb.total_recorded(),
        total_adaptations,
        "knowledge base total_recorded should match total_adaptations count"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 7. MapeKSimulator end-to-end (mod.rs orchestrator)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn simulator_normal_operation_no_violations() {
    let config = SimConfig {
        sensors: vec![test_sensor("sig", 100, 0, 1)],
        properties: vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "sig".to_string(),
            50,
        ))],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 255,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 32,
        knowledge_capacity: 100,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(500);

    assert_eq!(result.total_ticks, 500, "simulator should complete all 500 ticks");
    assert_eq!(result.total_violations, 0, "no violations expected when sensor stays normal");
    assert_eq!(result.total_adaptations, 0, "no adaptations expected when no violations occur");
    assert!(!result.emergency_triggered, "emergency should not trigger during normal operation");
    assert!(result.adaptation_log.is_empty(), "adaptation log should be empty with no violations");
}

#[test]
fn simulator_fault_triggers_emergency_stop() {
    let config = SimConfig {
        sensors: vec![test_sensor_with_fault("sig", 100, 5, 0, None, 1)],
        properties: vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "sig".to_string(),
            50,
        ))],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 255,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 16,
        knowledge_capacity: 50,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(200);

    assert!(result.emergency_triggered, "emergency should trigger after sensor fault");
    assert!(result.total_ticks < 200, "simulation should halt early due to emergency stop");
    assert!(result.emergency_tick.is_some(), "emergency_tick should record when stop occurred");
    assert!(result.total_violations > 0, "violations should be detected before emergency");
    assert!(
        result.total_adaptations > 0,
        "at least one adaptation (the emergency stop) should be recorded"
    );
}

#[test]
fn simulator_transient_fault_with_recovery() {
    let config = SimConfig {
        sensors: vec![test_sensor_with_fault("sig", 100, 20, 10, Some(30), 1)],
        properties: vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "sig".to_string(),
            50,
        ))],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "sig".to_string(), value: 75 },
            priority: 50,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 32,
        knowledge_capacity: 200,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(100);

    assert_eq!(
        result.total_ticks, 100,
        "simulation should complete all ticks (no emergency stop in config)"
    );
    assert!(
        result.total_violations > 0,
        "violations should occur during the transient fault window"
    );
    assert!(result.total_adaptations > 0, "adaptations should be applied during fault window");
    assert!(!result.emergency_triggered, "SetSignal action should not trigger emergency state");
}

#[test]
fn simulator_multi_sensor_multi_property() {
    let config = SimConfig {
        sensors: vec![
            test_sensor_with_fault("temp", 37, 30, 0, Some(40), 10),
            test_sensor("pressure", 120, 0, 20),
        ],
        properties: vec![
            TemporalProperty::Always(SignalPredicate::GreaterThan("temp".to_string(), 10)),
            TemporalProperty::Always(SignalPredicate::GreaterThan("pressure".to_string(), 80)),
        ],
        action_table: vec![
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::SetSignal { name: "temp".to_string(), value: 37 },
                priority: 50,
                trigger_on: TriggerCondition::OnViolation,
            },
            ActionEntry {
                trigger_property_idx: 1,
                action: AdaptationAction::EmergencyStop,
                priority: 200,
                trigger_on: TriggerCondition::OnViolation,
            },
        ],
        window_size: 32,
        knowledge_capacity: 200,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(100);

    // temp faults at ticks 30-39 (value=0, below threshold 10).
    // pressure stays normal at 120 (always > 80).
    assert!(result.total_violations > 0, "temp violation should be detected during fault window");
    assert!(result.total_adaptations > 0, "adaptations should occur for temp violation");
    assert!(
        !result.emergency_triggered,
        "emergency should not trigger since pressure stays normal and temp's action is SetSignal"
    );
}

#[test]
fn simulator_result_summary_contains_all_fields() {
    let config = SimConfig {
        sensors: vec![test_sensor_with_fault("s", 100, 5, 0, None, 1)],
        properties: vec![TemporalProperty::Always(SignalPredicate::IsTrue("s".to_string()))],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "s".to_string(), value: 1 },
            priority: 10,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 16,
        knowledge_capacity: 50,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(20);
    let summary = result.summary();

    assert!(summary.contains("MAPE-K Simulation:"), "summary should contain header");
    assert!(summary.contains("Violations detected:"), "summary should contain violations line");
    assert!(summary.contains("Adaptations applied:"), "summary should contain adaptations line");
    assert!(summary.contains("Adaptation log entries:"), "summary should contain log entry count");
}

#[test]
fn simulator_zero_ticks_produces_empty_result() {
    let config = SimConfig {
        sensors: vec![test_sensor("s", 100, 0, 1)],
        properties: vec![TemporalProperty::Always(SignalPredicate::IsTrue("s".to_string()))],
        action_table: vec![],
        window_size: 8,
        knowledge_capacity: 10,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(0);

    assert_eq!(result.total_ticks, 0, "zero-tick run should yield 0 ticks");
    assert_eq!(result.total_violations, 0, "zero-tick run should have no violations");
    assert_eq!(result.total_adaptations, 0, "zero-tick run should have no adaptations");
}

#[test]
fn simulator_empty_action_table_violations_but_no_adaptations() {
    let config = SimConfig {
        sensors: vec![test_sensor_with_fault("s", 100, 0, 0, None, 1)],
        properties: vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "s".to_string(),
            50,
        ))],
        action_table: vec![], // no actions
        window_size: 16,
        knowledge_capacity: 10,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(50);

    assert!(result.total_violations > 0, "violations should be detected even without action table");
    assert_eq!(result.total_adaptations, 0, "no adaptations should occur with empty action table");
    assert!(result.adaptation_log.is_empty(), "adaptation log should be empty with no actions");
}

// ═══════════════════════════════════════════════════════════════════════════
// 8. Bridge integration (PipelineResult -> SimConfig)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn bridge_converts_always_property_from_mirr_source() {
    let src = mirr_module_with_property(
        r#"property p_safe {
    always (x);
}"#,
    );
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result =
        run_pipeline(&src, &config).expect("pipeline should succeed for valid MIRR source");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should convert Always property");

    assert!(!sim_config.sensors.is_empty(), "bridge should extract input signals as sensors");
    assert_eq!(
        sim_config.properties.len(),
        1,
        "bridge should produce one temporal property from one assert-property"
    );
    assert_eq!(
        sim_config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("x".to_string())),
        "bridge should lower always(x) to Always(IsTrue('x'))"
    );
    assert_eq!(
        sim_config.action_table.len(),
        1,
        "bridge should generate one action entry per property"
    );
    assert_eq!(
        sim_config.action_table[0].action,
        AdaptationAction::EmergencyStop,
        "bridge default action should be EmergencyStop"
    );
    assert_eq!(
        sim_config.action_table[0].priority, 255,
        "bridge default priority should be maximum (255)"
    );
}

#[test]
fn bridge_converts_never_property_from_mirr_source() {
    let src = mirr_module_with_property(
        r#"property p_no_fault {
    never (y);
}"#,
    );
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should convert Never property");

    assert_eq!(sim_config.properties.len(), 1, "bridge should produce one temporal property");
    assert_eq!(
        sim_config.properties[0],
        TemporalProperty::Always(SignalPredicate::LessThan("y".to_string(), 1)),
        "bridge should lower never(y) to Always(LessThan('y', 1))"
    );
}

#[test]
fn bridge_converts_eventually_within_from_mirr_source() {
    let src = mirr_module_with_property(
        r#"property p_recovery {
    eventually within 8 (x);
}"#,
    );
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed");
    let sim_config =
        bridge_from_pipeline(&result).expect("bridge should convert EventuallyWithin property");

    assert_eq!(sim_config.properties.len(), 1, "bridge should produce one temporal property");
    assert_eq!(
        sim_config.properties[0],
        TemporalProperty::EventuallyWithin(SignalPredicate::IsTrue("x".to_string()), 8),
        "bridge should lower eventually within 8 (x) correctly"
    );
}

#[test]
fn bridge_skips_cover_and_assume_properties() {
    let src = mirr_module_with_property(
        r#"property p_cover {
    cover always (x);
}
property p_assume {
    assume always (x);
}"#,
    );
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should succeed");

    assert!(sim_config.properties.is_empty(), "bridge should skip cover and assume properties");
    assert!(
        sim_config.action_table.is_empty(),
        "action table should be empty when no assert-properties exist"
    );
}

#[test]
fn bridge_extracts_input_signals_as_sensors_only() {
    let src = r#"
module sensor_test {
    signal in1: in bool;
    signal in2: in u8;
    signal out1: out bool;
    signal out2: out u16;

    guard g_sensor {
        when in1
        for 1 cycles;
    }

    reflex r_sensor {
        on g_sensor {
            out1 = true;
        }
    }
}
"#;
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(src, &config).expect("pipeline should succeed");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should succeed");

    assert_eq!(
        sim_config.sensors.len(),
        2,
        "bridge should extract exactly 2 input signals (in1, in2)"
    );
    let sensor_names: Vec<&str> = sim_config.sensors.iter().map(|s| s.name.as_str()).collect();
    assert!(sensor_names.contains(&"in1"), "in1 should be extracted as a sensor");
    assert!(sensor_names.contains(&"in2"), "in2 should be extracted as a sensor");
}

#[test]
fn bridge_bool_sensor_has_correct_defaults() {
    let src = mirr_module_with_typed_input("signal flag_in: in bool;", "");
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should succeed");

    let bool_sensor = sim_config
        .sensors
        .iter()
        .find(|s| s.name == "flag_in")
        .expect("flag_in sensor should exist");
    assert_eq!(bool_sensor.base_value, 1, "bool sensor base_value should be 1");
    assert_eq!(bool_sensor.noise_amplitude, 0, "bool sensor noise_amplitude should be 0");
}

#[test]
fn bridge_unsigned_sensor_has_midpoint_base() {
    let src = mirr_module_with_typed_input("signal data_in: in u8;", "");
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should succeed");

    let u8_sensor = sim_config
        .sensors
        .iter()
        .find(|s| s.name == "data_in")
        .expect("data_in sensor should exist");
    // u8 max = 255, midpoint = 127.
    assert_eq!(u8_sensor.base_value, 127, "u8 sensor base_value should be midpoint 127");
    assert_eq!(
        u8_sensor.noise_amplitude, 2,
        "u8 sensor noise_amplitude should be DEFAULT_NOISE_AMPLITUDE=2"
    );
}

#[test]
fn bridge_default_window_and_capacity() {
    let src = mirr_module_with_property("");
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should succeed");

    assert_eq!(
        sim_config.window_size, DEFAULT_WINDOW_SIZE,
        "bridge should use DEFAULT_WINDOW_SIZE"
    );
    assert_eq!(
        sim_config.knowledge_capacity, DEFAULT_KNOWLEDGE_CAPACITY,
        "bridge should use DEFAULT_KNOWLEDGE_CAPACITY"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 9. Full end-to-end: Pipeline -> Bridge -> MapeKSimulator
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn end_to_end_pipeline_bridge_simulator_normal() {
    let src = mirr_module_with_property(
        r#"property p_alive {
    always (x);
}"#,
    );
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should succeed");

    let mut sim = MapeKSimulator::new(sim_config);
    let mape_result = sim.run(200);

    // x is a bool input with base_value=1 and noise=0, so IsTrue should always hold.
    assert_eq!(mape_result.total_ticks, 200, "simulator should complete all 200 ticks");
    // No violations expected (base_value=1 means IsTrue is always satisfied).
    assert_eq!(
        mape_result.total_violations, 0,
        "no violations expected for always-true bool signal"
    );
}

#[test]
fn end_to_end_pipeline_with_mape_k_flag() {
    let src = mirr_module_with_property(
        r#"property p_check {
    always (x);
}"#,
    );
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: true,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed with mape_k=true");
    assert!(
        result.mape_k_result.is_some(),
        "pipeline with mape_k=true should produce a MapeKResult"
    );

    let mape_result = result.mape_k_result.unwrap();
    assert!(
        mape_result.total_ticks > 0,
        "MAPE-K simulation through pipeline should run for some ticks"
    );
}

#[test]
fn end_to_end_pipeline_bridge_simulator_with_multiple_properties() {
    let src = r#"
module multi_prop {
    signal a: in bool;
    signal b: in bool;
    signal z: out bool;

    guard g_multi {
        when a
        for 1 cycles;
    }

    reflex r_multi {
        on g_multi {
            z = true;
        }
    }

    property p1 {
        always (a);
    }

    property p2 {
        always (b);
    }
}
"#;
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(src, &config).expect("pipeline should succeed");
    let sim_config = bridge_from_pipeline(&result).expect("bridge should succeed");

    assert_eq!(sim_config.sensors.len(), 3, "bridge should extract 3 sensors (all signals)");
    assert_eq!(sim_config.properties.len(), 2, "bridge should produce 2 temporal properties");
    assert_eq!(sim_config.action_table.len(), 2, "bridge should generate 2 action entries");

    let mut sim = MapeKSimulator::new(sim_config);
    let mape_result = sim.run(100);
    assert_eq!(mape_result.total_ticks, 100, "multi-property simulation should complete");
}

// ═══════════════════════════════════════════════════════════════════════════
// 10. LTL property evaluation edge cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn ltl_in_range_predicate_boundary_values() {
    let pred = SignalPredicate::InRange("temp".to_string(), 36, 38);
    assert!(pred.evaluate(36), "InRange(36, 38) should include lower bound 36");
    assert!(pred.evaluate(38), "InRange(36, 38) should include upper bound 38");
    assert!(pred.evaluate(37), "InRange(36, 38) should include midpoint 37");
    assert!(!pred.evaluate(35), "InRange(36, 38) should exclude 35");
    assert!(!pred.evaluate(39), "InRange(36, 38) should exclude 39");
    assert!(!pred.evaluate(0), "InRange(36, 38) should exclude 0");
    assert!(!pred.evaluate(u64::MAX), "InRange(36, 38) should exclude u64::MAX");
}

#[test]
fn ltl_signal_predicate_signal_name_extraction() {
    let predicates = vec![
        SignalPredicate::IsTrue("alpha".to_string()),
        SignalPredicate::LessThan("beta".to_string(), 50),
        SignalPredicate::GreaterThan("gamma".to_string(), 100),
        SignalPredicate::InRange("delta".to_string(), 10, 20),
    ];
    let expected_names = ["alpha", "beta", "gamma", "delta"];

    for i in 0..predicates.len() {
        assert_eq!(
            predicates[i].signal_name(),
            expected_names[i],
            "signal_name() for predicate at index {i} should match"
        );
    }
}

#[test]
fn ltl_temporal_property_signal_name_delegation() {
    let props = vec![
        TemporalProperty::Always(SignalPredicate::IsTrue("sig_a".to_string())),
        TemporalProperty::EventuallyWithin(
            SignalPredicate::GreaterThan("sig_b".to_string(), 10),
            5,
        ),
        TemporalProperty::Persists(SignalPredicate::LessThan("sig_c".to_string(), 100), 3),
    ];
    let expected = ["sig_a", "sig_b", "sig_c"];

    for i in 0..props.len() {
        assert_eq!(
            props[i].signal_name(),
            expected[i],
            "TemporalProperty.signal_name() at index {i} should delegate correctly"
        );
    }
}

#[test]
fn analyzer_empty_window_vacuous_truth_for_always() {
    let monitor = Monitor::new(32, &["sig"]);
    // No samples recorded — empty window.
    let analyzer = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
        "sig".to_string(),
        100,
    ))]);

    let results = analyzer.evaluate(&monitor);
    assert!(results[0].satisfied, "Always on empty window should be vacuously true");
}

#[test]
fn analyzer_empty_window_eventually_within_is_false() {
    let monitor = Monitor::new(32, &["sig"]);
    let analyzer = Analyzer::new(vec![TemporalProperty::EventuallyWithin(
        SignalPredicate::IsTrue("sig".to_string()),
        10,
    )]);

    let results = analyzer.evaluate(&monitor);
    assert!(!results[0].satisfied, "EventuallyWithin on empty window should be unsatisfied");
}

#[test]
fn analyzer_persists_zero_n_is_vacuously_satisfied() {
    let mut monitor = Monitor::new(32, &["sig"]);
    monitor.record_sample("sig", 0);
    monitor.advance_tick();

    let analyzer = Analyzer::new(vec![TemporalProperty::Persists(
        SignalPredicate::IsTrue("sig".to_string()),
        0,
    )]);

    let results = analyzer.evaluate(&monitor);
    assert!(results[0].satisfied, "Persists(P, 0) should be vacuously satisfied");
}

#[test]
fn analyzer_unknown_signal_produces_vacuous_result() {
    let mut monitor = Monitor::new(32, &["known"]);
    monitor.record_sample("known", 100);
    monitor.advance_tick();

    // Property references "unknown" signal — not in the monitor.
    let analyzer = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
        "unknown".to_string(),
        50,
    ))]);

    let results = analyzer.evaluate(&monitor);
    assert!(results[0].satisfied, "Always on unknown signal should be vacuously true (no data)");
}

// ═══════════════════════════════════════════════════════════════════════════
// 11. Ring buffer and monitor edge cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn ring_buffer_zero_capacity_ignores_pushes() {
    let mut rb = RingBuffer::new(0);
    rb.push(42);
    assert!(rb.is_empty(), "zero-capacity ring buffer should remain empty after push");
    assert_eq!(rb.get(0), None, "zero-capacity ring buffer should return None for any index");
}

#[test]
fn ring_buffer_single_capacity_overwrites() {
    let mut rb = RingBuffer::new(1);
    rb.push(10);
    assert_eq!(rb.get(0), Some(10), "single-capacity buffer should hold most recent value");
    rb.push(20);
    assert_eq!(rb.get(0), Some(20), "single-capacity buffer should overwrite with new value");
    assert_eq!(rb.len(), 1, "single-capacity buffer length should be 1");
}

#[test]
fn ring_buffer_iter_exact_size() {
    let mut rb = RingBuffer::new(8);
    for i in 0..5 {
        rb.push(i);
    }
    let iter = rb.iter();
    assert_eq!(iter.len(), 5, "RingBufferIter should implement ExactSizeIterator correctly");
    let collected: Vec<u64> = rb.iter().collect();
    assert_eq!(collected, vec![0, 1, 2, 3, 4], "iter should yield values oldest-to-newest");
}

#[test]
fn monitor_reset_clears_all_state() {
    let mut monitor = Monitor::new(16, &["a", "b"]);
    monitor.record_sample("a", 100);
    monitor.record_sample("b", 200);
    monitor.advance_tick();
    monitor.record_sample("a", 101);
    monitor.advance_tick();

    assert_eq!(monitor.tick(), 2, "tick should be 2 before reset");
    monitor.reset();

    assert_eq!(monitor.tick(), 0, "tick should be 0 after reset");
    assert!(monitor.window("a").unwrap().is_empty(), "window 'a' should be empty after reset");
    assert!(monitor.window("b").unwrap().is_empty(), "window 'b' should be empty after reset");
}

#[test]
fn monitor_ignores_unregistered_signals() {
    let mut monitor = Monitor::new(16, &["registered"]);
    monitor.record_sample("registered", 42);
    monitor.record_sample("unregistered", 99); // should not panic

    assert!(monitor.window("unregistered").is_none(), "unregistered signal should have no window");
    assert_eq!(
        monitor.window("registered").unwrap().get(0),
        Some(42),
        "registered signal should have its value"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 12. Executor edge cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn executor_clear_emergency_allows_further_operations() {
    let mut executor = Executor::new(vec!["sig".to_string()]);
    let mut env = HashMap::from([("sig".to_string(), 100u64)]);

    executor.apply(&AdaptationAction::EmergencyStop, &mut env);
    assert!(executor.is_emergency_active(), "emergency should be active after stop");

    executor.clear_emergency();
    assert!(!executor.is_emergency_active(), "emergency should be cleared after clear_emergency()");

    // Apply a normal action after clearing.
    let rec = executor
        .apply(&AdaptationAction::SetSignal { name: "sig".to_string(), value: 50 }, &mut env);
    assert!(rec.success, "SetSignal should succeed after clearing emergency");
    assert_eq!(
        env["sig"], 50,
        "signal should be updated after clearing emergency and applying SetSignal"
    );
}

#[test]
fn executor_emergency_stop_captures_pre_and_post_state() {
    let mut executor = Executor::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    let mut env = HashMap::from([
        ("a".to_string(), 10u64),
        ("b".to_string(), 20u64),
        ("c".to_string(), 30u64),
    ]);

    let rec = executor.apply(&AdaptationAction::EmergencyStop, &mut env);
    assert!(rec.success, "emergency stop should succeed");

    // pre_state should capture all signal values before zeroing.
    assert_eq!(rec.pre_state.len(), 3, "pre_state should capture all 3 signals");
    // post_state should show all zeros.
    for (name, val) in &rec.post_state {
        assert_eq!(*val, 0, "post_state for signal '{name}' should be 0 after emergency stop");
    }
}

#[test]
fn executor_set_signal_pre_post_state_correct() {
    let mut executor = Executor::new(vec!["valve".to_string()]);
    let mut env = HashMap::from([("valve".to_string(), 42u64)]);

    let rec = executor
        .apply(&AdaptationAction::SetSignal { name: "valve".to_string(), value: 100 }, &mut env);

    assert_eq!(
        rec.pre_state,
        vec![("valve".to_string(), 42)],
        "pre_state should capture original value 42"
    );
    assert_eq!(
        rec.post_state,
        vec![("valve".to_string(), 100)],
        "post_state should capture new value 100"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 13. Knowledge base edge cases
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn knowledge_base_eviction_preserves_newest() {
    let mut kb = KnowledgeBase::new(5);

    for tick in 0..MAX_TEST_RECORDS.min(20) {
        kb.record(make_adaptation(
            tick as u64,
            0,
            AdaptationAction::SetSignal { name: "sig".to_string(), value: tick as u64 },
        ));
    }

    assert_eq!(kb.len(), 5, "knowledge base should be capped at capacity=5");
    assert_eq!(
        kb.total_recorded(),
        20,
        "total_recorded should count all 20 records including evicted"
    );
    // Newest 5 records should be ticks 15-19.
    let records = kb.records();
    for i in 0..5 {
        assert_eq!(
            records[i].tick,
            (15 + i) as u64,
            "record at index {i} should have tick {}",
            15 + i
        );
    }
}

#[test]
fn knowledge_base_json_roundtrip_consistency() {
    let mut kb = KnowledgeBase::new(100);
    kb.record(make_adaptation(
        1,
        0,
        AdaptationAction::SetSignal { name: "valve".to_string(), value: 1 },
    ));
    kb.record(make_adaptation(2, 1, AdaptationAction::EmergencyStop));
    kb.record(make_adaptation(
        3,
        0,
        AdaptationAction::SwitchMode { mode_name: "safe".to_string() },
    ));

    let json = kb.to_json().expect("serialization should succeed");

    // Verify all action types are present in JSON.
    assert!(json.contains("SetSignal"), "JSON should contain SetSignal action");
    assert!(json.contains("EmergencyStop"), "JSON should contain EmergencyStop action");
    assert!(json.contains("SwitchMode"), "JSON should contain SwitchMode action");
    assert!(json.contains("\"tick\": 1"), "JSON should contain tick 1");
    assert!(json.contains("\"tick\": 2"), "JSON should contain tick 2");
    assert!(json.contains("\"tick\": 3"), "JSON should contain tick 3");
}

#[test]
fn knowledge_base_clear_resets_everything() {
    let mut kb = KnowledgeBase::new(50);
    for tick in 0..MAX_TEST_RECORDS.min(10) {
        kb.record(make_adaptation(tick as u64, 0, AdaptationAction::EmergencyStop));
    }

    assert_eq!(kb.len(), 10, "should have 10 records before clear");
    kb.clear();

    assert!(kb.is_empty(), "knowledge base should be empty after clear");
    assert_eq!(kb.total_recorded(), 0, "total_recorded should be 0 after clear");
    assert_eq!(kb.records().len(), 0, "records() should return empty slice after clear");
}

// ═══════════════════════════════════════════════════════════════════════════
// 14. Planner edge cases and trigger conditions
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn planner_empty_results_produces_no_action() {
    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::EmergencyStop,
        priority: 100,
        trigger_on: TriggerCondition::OnViolation,
    }]);

    let plan = planner.select(&[]);
    assert_eq!(plan.action, None, "planner should return None when results list is empty");
}

#[test]
fn planner_empty_table_produces_no_action() {
    let planner = Planner::new(vec![]);
    let plan = planner.select(&[make_result(0, false)]);
    assert_eq!(plan.action, None, "planner with empty table should always return None");
}

#[test]
fn planner_on_violation_ignores_satisfied_properties() {
    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::EmergencyStop,
        priority: 100,
        trigger_on: TriggerCondition::OnViolation,
    }]);

    // Property 0 is satisfied — should NOT trigger OnViolation.
    let plan = planner.select(&[make_result(0, true)]);
    assert_eq!(plan.action, None, "OnViolation trigger should not fire when property is satisfied");
}

#[test]
fn planner_on_satisfaction_ignores_violated_properties() {
    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::EmergencyStop,
        priority: 100,
        trigger_on: TriggerCondition::OnSatisfaction,
    }]);

    // Property 0 is violated — should NOT trigger OnSatisfaction.
    let plan = planner.select(&[make_result(0, false)]);
    assert_eq!(
        plan.action, None,
        "OnSatisfaction trigger should not fire when property is violated"
    );
}

#[test]
fn planner_mixed_trigger_conditions() {
    let planner = Planner::new(vec![
        ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "fix".to_string(), value: 1 },
            priority: 50,
            trigger_on: TriggerCondition::OnViolation,
        },
        ActionEntry {
            trigger_property_idx: 1,
            action: AdaptationAction::EmergencyStop,
            priority: 200,
            trigger_on: TriggerCondition::OnSatisfaction,
        },
    ]);

    // Property 0 is violated, property 1 is satisfied.
    let results = vec![make_result(0, false), make_result(1, true)];
    let plan = planner.select(&results);

    assert_eq!(
        plan.action,
        Some(AdaptationAction::EmergencyStop),
        "OnSatisfaction(prop1) should fire since prop1 is satisfied, and it has higher priority"
    );
}

#[test]
fn planner_action_label_formatting() {
    let actions = vec![
        AdaptationAction::SetSignal { name: "valve".to_string(), value: 42 },
        AdaptationAction::SwitchMode { mode_name: "nominal".to_string() },
        AdaptationAction::EmergencyStop,
    ];
    let expected_labels = ["SetSignal(valve=42)", "SwitchMode(nominal)", "EmergencyStop"];

    for i in 0..actions.len() {
        assert_eq!(
            actions[i].label(),
            expected_labels[i],
            "action label at index {i} should match expected format"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 15. Sensor determinism and reset
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn sensor_reset_reproduces_identical_sequence() {
    let cfg = test_sensor("noisy", 100, 10, 777);
    let mut sensor = SensorModel::new(cfg);

    let mut first_run = Vec::with_capacity(100);
    for _i in 0..MAX_TEST_TICKS.min(100) {
        first_run.push(sensor.sample());
    }

    sensor.reset();

    let mut second_run = Vec::with_capacity(100);
    for _i in 0..MAX_TEST_TICKS.min(100) {
        second_run.push(sensor.sample());
    }

    assert_eq!(first_run, second_run, "sensor reset should reproduce identical sequence");
}

#[test]
fn sensor_different_seeds_produce_different_output() {
    let cfg1 = test_sensor("s", 100, 10, 42);
    let cfg2 = test_sensor("s", 100, 10, 99);
    let mut s1 = SensorModel::new(cfg1);
    let mut s2 = SensorModel::new(cfg2);

    let mut any_different = false;
    for _i in 0..MAX_TEST_TICKS.min(50) {
        if s1.sample() != s2.sample() {
            any_different = true;
            break;
        }
    }

    assert!(
        any_different,
        "different seeds should produce at least one different sample in 50 ticks"
    );
}

#[test]
fn sensor_noise_bounded_around_base_value() {
    let cfg = test_sensor("bounded", 100, 5, 42);
    let mut sensor = SensorModel::new(cfg);

    for _i in 0..MAX_TEST_TICKS.min(500) {
        let v = sensor.sample();
        assert!(
            (95..=105).contains(&v),
            "sample {v} should be within [base-noise, base+noise] = [95, 105]"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 16. Adaptation record construction and correctness
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn adaptation_record_from_execution_captures_all_fields() {
    let exec_record = ExecutionRecord {
        action: AdaptationAction::SetSignal { name: "valve".to_string(), value: 1 },
        pre_state: vec![("valve".to_string(), 0)],
        post_state: vec![("valve".to_string(), 1)],
        success: true,
        error: None,
    };

    let adaptation = AdaptationRecord::from_execution(42, 3, "pressure_low", &exec_record);

    assert_eq!(adaptation.tick, 42, "tick should be 42");
    assert_eq!(adaptation.trigger_property_idx, 3, "trigger_property_idx should be 3");
    assert_eq!(adaptation.trigger_description, "pressure_low", "trigger_description should match");
    assert!(adaptation.success, "success should reflect execution record");
    assert_eq!(
        adaptation.pre_state,
        vec![("valve".to_string(), 0)],
        "pre_state should match execution record"
    );
    assert_eq!(
        adaptation.post_state,
        vec![("valve".to_string(), 1)],
        "post_state should match execution record"
    );
}

#[test]
fn adaptation_record_from_failed_execution() {
    let exec_record = ExecutionRecord {
        action: AdaptationAction::SetSignal { name: "unknown".to_string(), value: 1 },
        pre_state: vec![],
        post_state: vec![],
        success: false,
        error: Some("unknown signal 'unknown'".to_string()),
    };

    let adaptation = AdaptationRecord::from_execution(10, 0, "error_test", &exec_record);

    assert!(!adaptation.success, "adaptation should reflect failed execution");
    assert!(adaptation.pre_state.is_empty(), "failed execution should have empty pre_state");
    assert!(adaptation.post_state.is_empty(), "failed execution should have empty post_state");
}

// ═══════════════════════════════════════════════════════════════════════════
// 17. Complex multi-signal simulation scenarios
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn three_sensor_cascade_fault_scenario() {
    let config = SimConfig {
        sensors: vec![
            test_sensor_with_fault("temp", 37, 10, 0, Some(15), 10),
            test_sensor_with_fault("pressure", 120, 20, 0, Some(25), 20),
            test_sensor("heart_rate", 72, 0, 30),
        ],
        properties: vec![
            TemporalProperty::Always(SignalPredicate::GreaterThan("temp".to_string(), 10)),
            TemporalProperty::Always(SignalPredicate::GreaterThan("pressure".to_string(), 50)),
            TemporalProperty::Always(SignalPredicate::GreaterThan("heart_rate".to_string(), 50)),
        ],
        action_table: vec![
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::SetSignal { name: "temp".to_string(), value: 37 },
                priority: 50,
                trigger_on: TriggerCondition::OnViolation,
            },
            ActionEntry {
                trigger_property_idx: 1,
                action: AdaptationAction::SetSignal { name: "pressure".to_string(), value: 120 },
                priority: 100,
                trigger_on: TriggerCondition::OnViolation,
            },
            ActionEntry {
                trigger_property_idx: 2,
                action: AdaptationAction::EmergencyStop,
                priority: 255,
                trigger_on: TriggerCondition::OnViolation,
            },
        ],
        window_size: 32,
        knowledge_capacity: 200,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(100);

    assert!(result.total_violations > 0, "cascading faults should produce violations");
    assert!(result.total_adaptations > 0, "cascading faults should produce adaptations");
    assert!(
        !result.emergency_triggered,
        "heart_rate stays normal (72 > 50), so emergency should not trigger"
    );
    assert!(
        !result.adaptation_log.is_empty(),
        "adaptation log should have entries for the cascade"
    );
}

#[test]
fn persists_trigger_with_on_satisfaction_scenario() {
    // Test scenario: "if the overheat condition persists for 5+ ticks, trigger emergency."
    let config = SimConfig {
        sensors: vec![test_sensor_with_fault("temp", 37, 3, 999, None, 1)],
        properties: vec![TemporalProperty::Persists(
            SignalPredicate::GreaterThan("temp".to_string(), 500),
            5,
        )],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 255,
            trigger_on: TriggerCondition::OnSatisfaction,
        }],
        window_size: 32,
        knowledge_capacity: 50,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(200);

    // Sensor faults at tick 3 to value 999 (> 500). After 5 ticks of persistence
    // (around tick 8), the property becomes satisfied, triggering OnSatisfaction emergency.
    assert!(
        result.emergency_triggered,
        "emergency should trigger when overheat persists for 5+ ticks"
    );
    assert!(result.emergency_tick.is_some(), "emergency_tick should be recorded");
    let e_tick = result.emergency_tick.unwrap();
    assert!(
        e_tick >= 7 && e_tick < 50,
        "emergency should trigger shortly after tick 8 (5 ticks of persistence after fault at tick 3), got {e_tick}"
    );
}

#[test]
fn eventually_within_deadline_miss_triggers_adaptation() {
    let config = SimConfig {
        sensors: vec![test_sensor_with_fault("heartbeat", 1, 5, 0, None, 1)],
        properties: vec![TemporalProperty::EventuallyWithin(
            SignalPredicate::IsTrue("heartbeat".to_string()),
            3,
        )],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "heartbeat".to_string(), value: 1 },
            priority: 100,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 16,
        knowledge_capacity: 100,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(50);

    // heartbeat goes to 0 at tick 5, and stays 0 forever.
    // After 3 ticks without IsTrue being satisfied, EventuallyWithin is violated.
    assert!(result.total_violations > 0, "deadline miss should produce violations");
    assert!(result.total_adaptations > 0, "deadline miss violations should trigger adaptations");
}

// ═══════════════════════════════════════════════════════════════════════════
// 18. SimConfig serialization consistency
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn sim_config_serializes_to_json() {
    let config = SimConfig {
        sensors: vec![test_sensor("test", 100, 5, 42)],
        properties: vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "test".to_string(),
            50,
        ))],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 255,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 32,
        knowledge_capacity: 100,
    };

    let json = serde_json::to_string_pretty(&config).expect("SimConfig should serialize to JSON");
    assert!(json.contains("\"name\": \"test\""), "JSON should contain sensor name");
    assert!(json.contains("EmergencyStop"), "JSON should contain action type");
    assert!(json.contains("\"window_size\": 32"), "JSON should contain window_size");
}

#[test]
fn mape_k_result_serializes_to_json() {
    let config = SimConfig {
        sensors: vec![test_sensor_with_fault("s", 100, 5, 0, None, 1)],
        properties: vec![TemporalProperty::Always(SignalPredicate::IsTrue("s".to_string()))],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 255,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 16,
        knowledge_capacity: 50,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(30);

    let json = serde_json::to_string_pretty(&result).expect("MapeKResult should serialize to JSON");
    assert!(json.contains("\"total_ticks\""), "JSON should contain total_ticks field");
    assert!(json.contains("\"total_violations\""), "JSON should contain total_violations field");
    assert!(
        json.contains("\"emergency_triggered\""),
        "JSON should contain emergency_triggered field"
    );
    assert!(json.contains("\"adaptation_log\""), "JSON should contain adaptation_log field");
}

// ═══════════════════════════════════════════════════════════════════════════
// 19. Bridge error handling
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn bridge_unsupported_always_implies_produces_error() {
    let src = mirr_module_with_property(
        r#"property p_impl {
    always (x -> y);
}"#,
    );
    let config = PipelineConfig {
        typecheck: false,
        simplify: false,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(&src, &config).expect("pipeline should succeed");
    let err = bridge_from_pipeline(&result).expect_err("bridge should fail for AlwaysImplies");

    let has_unsupported = err.iter().any(|e| matches!(e, BridgeError::UnsupportedFormula { .. }));
    assert!(has_unsupported, "bridge errors should include UnsupportedFormula for AlwaysImplies");
}

#[test]
fn bridge_error_display_formatting() {
    let errors = vec![
        BridgeError::TooManySignals { count: 300 },
        BridgeError::TooManyProperties { count: 100 },
        BridgeError::UnsupportedFormula { description: "test formula".to_string() },
    ];

    let s0 = format!("{}", errors[0]);
    assert!(s0.contains("300"), "TooManySignals display should contain the count");
    assert!(s0.contains("too many signals"), "TooManySignals display should contain description");

    let s1 = format!("{}", errors[1]);
    assert!(s1.contains("100"), "TooManyProperties display should contain the count");

    let s2 = format!("{}", errors[2]);
    assert!(
        s2.contains("test formula"),
        "UnsupportedFormula display should contain the description"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 20. Cross-module: PlanResult structure verification
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn plan_result_fields_are_consistent() {
    let planner = Planner::new(vec![
        ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "a".to_string(), value: 1 },
            priority: 10,
            trigger_on: TriggerCondition::OnViolation,
        },
        ActionEntry {
            trigger_property_idx: 1,
            action: AdaptationAction::EmergencyStop,
            priority: 100,
            trigger_on: TriggerCondition::OnViolation,
        },
    ]);

    // Only property 0 violated.
    let plan = planner.select(&[make_result(0, false), make_result(1, true)]);
    assert_eq!(
        plan.action,
        Some(AdaptationAction::SetSignal { name: "a".to_string(), value: 1 }),
        "only entry matching prop 0 should be selected"
    );
    assert_eq!(plan.entry_idx, Some(0), "entry_idx should point to the matching entry");
    assert_eq!(
        plan.trigger_property_idx,
        Some(0),
        "trigger_property_idx should match the violated property"
    );
}

#[test]
fn plan_result_no_action_has_all_none_fields() {
    let planner = Planner::new(vec![ActionEntry {
        trigger_property_idx: 5,
        action: AdaptationAction::EmergencyStop,
        priority: 100,
        trigger_on: TriggerCondition::OnViolation,
    }]);

    let plan = planner.select(&[make_result(0, false)]);
    assert_eq!(plan.action, None, "action should be None when no entry matches");
    assert_eq!(plan.entry_idx, None, "entry_idx should be None when no entry matches");
    assert_eq!(
        plan.trigger_property_idx, None,
        "trigger_property_idx should be None when no entry matches"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 21. Analyzer violations() method integration
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn analyzer_violations_returns_only_failed_properties() {
    let mut monitor = Monitor::new(32, &["a", "b"]);
    for _tick in 0..MAX_TEST_TICKS.min(10) {
        monitor.record_sample("a", 100); // > 50, satisfied
        monitor.record_sample("b", 10); // < 50, violated
        monitor.advance_tick();
    }

    let analyzer = Analyzer::new(vec![
        TemporalProperty::Always(SignalPredicate::GreaterThan("a".to_string(), 50)),
        TemporalProperty::Always(SignalPredicate::GreaterThan("b".to_string(), 50)),
    ]);

    let violations = analyzer.violations(&monitor);
    assert_eq!(violations.len(), 1, "violations() should return only the failed property");
    assert_eq!(violations[0].property_idx, 1, "the violated property should be index 1 (signal b)");
    assert!(!violations[0].satisfied, "violation entry should have satisfied=false");
}

#[test]
fn analyzer_property_count_matches_registered() {
    let analyzer = Analyzer::new(vec![
        TemporalProperty::Always(SignalPredicate::IsTrue("a".to_string())),
        TemporalProperty::EventuallyWithin(SignalPredicate::IsTrue("b".to_string()), 5),
        TemporalProperty::Persists(SignalPredicate::IsTrue("c".to_string()), 3),
    ]);

    assert_eq!(
        analyzer.property_count(),
        3,
        "property_count should match the number of registered properties"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 22. Planner entry_count method
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn planner_entry_count_matches_action_table() {
    let planner = Planner::new(vec![
        ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 100,
            trigger_on: TriggerCondition::OnViolation,
        },
        ActionEntry {
            trigger_property_idx: 1,
            action: AdaptationAction::SetSignal { name: "fix".to_string(), value: 1 },
            priority: 50,
            trigger_on: TriggerCondition::OnViolation,
        },
    ]);

    assert_eq!(
        planner.entry_count(),
        2,
        "entry_count should match the number of action table entries"
    );
}
