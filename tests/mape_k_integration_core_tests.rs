#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

//! Core integration tests for the MAPE-K bridge module.
//!
//! Covers 8 scenarios for `bridge_from_pipeline()`:
//! 1. SimConfig generation from a single safety property
//! 2. Neonatal-respirator scenario (multi-signal, multi-property)
//! 3. Rejection when signal count exceeds MAX_BRIDGE_SIGNALS
//! 4. Rejection when property count exceeds MAX_BRIDGE_PROPERTIES
//! 5. Always formula extraction with signal-predicate verification
//! 6. EventuallyWithin formula extraction with deadline verification
//! 7. Emergency-stop action table generation (priority 255, OnViolation)
//! 8. Cover and Assume properties are filtered out

use mirrc::ast::program::SignalDecl;
use mirrc::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};

use mirrc::mape_k::bridge::{bridge_from_pipeline, MAX_BRIDGE_PROPERTIES, MAX_BRIDGE_SIGNALS};
use mirrc::mape_k::error::MapeKError;
use mirrc::mape_k::{AdaptationAction, SignalPredicate, TemporalProperty, TriggerCondition};
use mirrc::pipeline::PipelineResult;

// =========================================================================
// Constants — bounded iteration limits (NASA P10)
// =========================================================================

/// Upper bound for signal generation loops in stress tests.
const MAX_TEST_SIGNALS: usize = 512;

/// Upper bound for property generation loops in stress tests.
const MAX_TEST_PROPERTIES: usize = 128;

// =========================================================================
// Test helpers — construct PipelineResult stubs
// =========================================================================

/// Build a minimal `PipelineResult` with the given signals and properties.
fn stub_pipeline(source: &str) -> PipelineResult {
    let mut reg = mirrc::ecs::Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut reg, source, None).unwrap();

    PipelineResult {
        hls_result: None,
        program: None,
        simplify_stats: None,
        width_stats: None,
        width_diagnostics: Vec::new(),
        temporal_netlist: None,
        rspu_program: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        sat_stats: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
        ecs_registry: Some(reg),
        file_table: mirrc::span::FileTable::new(),
    }
}

// =========================================================================
// Test 1: SimConfig from a single safety property
// =========================================================================

#[test]
fn bridge_generates_sim_config_from_safety_property() {
    let src = "module test { signal alarm: in bool; assert alarm_always_on: always alarm; }";
    let result = stub_pipeline(src);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed");
    println!(
        "PROPS: {:?}",
        result.ecs_registry.as_ref().unwrap().property_comps.iter().flatten().collect::<Vec<_>>()
    );

    assert_eq!(config.properties.len(), 1, "expected exactly one temporal property");
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("alarm".to_string()))
    );
}

// =========================================================================
// Test 2: Neonatal respirator scenario (multi-signal, multi-property)
// =========================================================================

#[test]
fn bridge_generates_sim_config_from_neonatal() {
    let src = "module test {\nsignal pressure: in u8;\nsignal flow_rate: in u16;\nsignal spo2: in u32;\nsignal heartbeat: in bool;\nsignal alarm: in bool;\nsignal valve_pos: in bool;\nassert pressure_safe: always pressure;\nassert spo2_recovery: eventually within 20 cycles spo2;\nassert heartbeat_present: always heartbeat;\n}";
    let result = stub_pipeline(src);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed");
    println!(
        "PROPS: {:?}",
        result.ecs_registry.as_ref().unwrap().property_comps.iter().flatten().collect::<Vec<_>>()
    );

    assert_eq!(config.sensors.len(), 6, "all signals become sensors");
    assert_eq!(config.sensors[0].name, "pressure");
    assert_eq!(config.sensors[1].name, "flow_rate");
    assert_eq!(config.sensors[2].name, "spo2");
    assert_eq!(config.sensors[3].name, "heartbeat");
    assert_eq!(config.sensors[4].name, "alarm");
    assert_eq!(config.sensors[5].name, "valve_pos");

    assert_eq!(config.properties.len(), 3, "all assert properties should be lowered");
    assert_eq!(config.action_table.len(), 3, "action table should match property count");
}

// =========================================================================
// Test 3: Reject too many signals
// =========================================================================

#[test]
fn bridge_rejects_too_many_signals() {
    let count = (MAX_BRIDGE_SIGNALS + 1).min(MAX_TEST_SIGNALS);
    let mut src = String::from("module test { ");
    for i in 0..count {
        src.push_str(&format!(
            "signal s{}: in u8;
",
            i
        ));
    }
    src.push_str("}");
    let result = stub_pipeline(&src);
    let err = bridge_from_pipeline(&result).expect_err("should fail with too many signals");

    assert!(
        err.iter().any(|e| matches!(e, MapeKError::BridgeConfigError(_))),
        "expected BridgeConfigError error, got: {err:?}"
    );
}

// =========================================================================
// Test 4: Reject too many properties
// =========================================================================

#[test]
fn bridge_rejects_too_many_properties() {
    let count = MAX_TEST_PROPERTIES.min(MAX_BRIDGE_PROPERTIES + 1);
    let mut src = String::from("module test { ");
    for i in 0..count {
        src.push_str(&format!(
            "signal sig{}: in bool;
",
            i
        ));
        src.push_str(&format!(
            "assert p{}: always sig{};
",
            i, i
        ));
    }
    src.push_str("}");
    let result = stub_pipeline(&src);
    let err = bridge_from_pipeline(&result).expect_err("should fail with too many properties");

    assert!(
        err.iter().any(|e| matches!(e, MapeKError::BridgeConfigError(_))),
        "expected BridgeConfigError error, got: {err:?}"
    );
}

// =========================================================================
// Test 5: Always formula extraction with signal-predicate verification
// =========================================================================

#[test]
fn bridge_extracts_always_property() {
    let src =
        "module test { signal engine_running: in bool; assert p_always: always engine_running; }";
    let result = stub_pipeline(src);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed");
    println!(
        "PROPS: {:?}",
        result.ecs_registry.as_ref().unwrap().property_comps.iter().flatten().collect::<Vec<_>>()
    );

    assert_eq!(config.properties.len(), 1);
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("engine_running".to_string()))
    );

    assert_eq!(config.properties[0].signal_name(), "engine_running");
}

// =========================================================================
// Test 6: EventuallyWithin formula extraction with deadline verification
// =========================================================================

#[test]
fn bridge_extracts_eventually_property() {
    let src = "module test { signal ready: in bool; assert p_eventually: eventually within 10 cycles ready; }";
    let result = stub_pipeline(src);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed");
    println!(
        "PROPS: {:?}",
        result.ecs_registry.as_ref().unwrap().property_comps.iter().flatten().collect::<Vec<_>>()
    );

    assert_eq!(config.properties.len(), 1);
    assert_eq!(
        config.properties[0],
        TemporalProperty::EventuallyWithin(SignalPredicate::IsTrue("ready".to_string()), 10)
    );

    match &config.properties[0] {
        TemporalProperty::EventuallyWithin(_, deadline) => {
            assert_eq!(*deadline, 10, "deadline should be 10 ticks");
        }
        other => panic!("expected EventuallyWithin, got: {other:?}"),
    }
}

// =========================================================================
// Test 7: Emergency-stop action table generation
// =========================================================================

#[test]
fn bridge_generates_emergency_stop_actions() {
    let src = "module test {
signal a: in bool;
signal b: in bool;
signal c: in bool;
assert p1: always a;
assert p2: always b;
assert p3: eventually within 5 cycles c;
}";
    let result = stub_pipeline(src);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed");
    println!(
        "PROPS: {:?}",
        result.ecs_registry.as_ref().unwrap().property_comps.iter().flatten().collect::<Vec<_>>()
    );

    assert_eq!(
        config.action_table.len(),
        config.properties.len(),
        "action table must have one entry per property"
    );

    let priorities: Vec<u8> = config.action_table.iter().map(|e| e.priority).collect();
    assert_eq!(
        priorities.iter().filter(|&&p| p == 200).count(),
        2,
        "expected two 200 priority actions"
    );
    assert_eq!(
        priorities.iter().filter(|&&p| p == 128).count(),
        1,
        "expected one 128 priority action"
    );
    for (i, entry) in config.action_table.iter().enumerate() {
        assert_eq!(
            entry.trigger_on,
            TriggerCondition::OnViolation,
            "entry {i} should trigger on violation"
        );
    }
}

// =========================================================================
// Test 8: Cover and Assume properties are filtered out
// =========================================================================

#[test]
fn bridge_skips_cover_and_assume_properties() {
    let src = "module test { signal x: in bool; signal y: in bool; signal z: in bool; property cover_prop: cover always x;\nproperty assume_prop: assume always y;\nassert assert_prop: always z; }";
    let result = stub_pipeline(src);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed");
    println!(
        "PROPS: {:?}",
        result.ecs_registry.as_ref().unwrap().property_comps.iter().flatten().collect::<Vec<_>>()
    );

    assert_eq!(config.properties.len(), 1, "only Assert properties should be lowered");
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("z".to_string()))
    );

    assert_eq!(config.action_table.len(), 1, "action table should match property count");
    assert_eq!(config.action_table[0].trigger_property_idx, 0);
    assert_eq!(config.action_table[0].action, AdaptationAction::EmergencyStop);
}
