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

use nasa_rust_project::ast::program::{MirrProgram, Module, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{ExtendedType, SignalKind, SignalType};
use nasa_rust_project::ast::Expr;
use nasa_rust_project::mape_k::bridge::{
    bridge_from_pipeline, BridgeError, MAX_BRIDGE_PROPERTIES, MAX_BRIDGE_SIGNALS,
};
use nasa_rust_project::mape_k::{
    AdaptationAction, SignalPredicate, TemporalProperty, TriggerCondition,
};
use nasa_rust_project::pipeline::PipelineResult;

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
/// All optional pipeline stages are `None` (only the program AST matters
/// for bridge conversion).
fn stub_pipeline(signals: Vec<SignalDecl>, properties: Vec<PropertyDecl>) -> PipelineResult {
    let module = Module {
        name: "test_mod".to_string(),
        signals,
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties,
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    PipelineResult {
        program: MirrProgram { patterns: Vec::new(), module },
        simplify_stats: None,
        width_result: None,
        temporal_netlist: None,
        rspu_program: None,
        type_map: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        sat_stats: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
    }
}

/// Create an input signal declaration with the given name and type.
fn input_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

/// Create an output signal declaration with the given name and type.
fn output_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Output,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

/// Create an Assert property declaration with the given name and formula.
fn assert_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Assert,
        formula,
        origin: None,
        span: None,
    }
}

// =========================================================================
// Test 1: SimConfig from a single safety property
// =========================================================================

#[test]
fn bridge_generates_sim_config_from_safety_property() {
    // Build a PipelineResult with one Assert(Always(IsTrue("alarm"))) property.
    let props = vec![assert_property(
        "alarm_always_on",
        PropertyFormula::Always(Expr::Signal("alarm".to_string())),
    )];
    let result = stub_pipeline(Vec::new(), props);

    let config = bridge_from_pipeline(&result).expect("bridge should succeed");

    // Exactly one temporal property, lowered to Always(IsTrue("alarm")).
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
    // Mimic neonatal_respirator.mirr: 4 input sensors + 2 outputs + 3 properties.
    let signals = vec![
        input_signal("pressure", SignalType::Unsigned(8)),
        input_signal("flow_rate", SignalType::Unsigned(16)),
        input_signal("spo2", SignalType::Unsigned(8)),
        input_signal("heartbeat", SignalType::Bool),
        output_signal("alarm", SignalType::Bool),
        output_signal("valve_pos", SignalType::Unsigned(8)),
    ];
    let props = vec![
        assert_property(
            "pressure_safe",
            PropertyFormula::Always(Expr::Signal("pressure".to_string())),
        ),
        assert_property(
            "spo2_recovery",
            PropertyFormula::EventuallyWithin {
                expr: Expr::Signal("spo2".to_string()),
                cycles: 20,
            },
        ),
        assert_property(
            "heartbeat_present",
            PropertyFormula::Always(Expr::Signal("heartbeat".to_string())),
        ),
    ];
    let result = stub_pipeline(signals, props);

    let config = bridge_from_pipeline(&result).expect("bridge should succeed");

    // All 6 signals become sensors (MEGA-14: all signals, not just inputs).
    assert_eq!(config.sensors.len(), 6, "all signals become sensors");
    assert_eq!(config.sensors[0].name, "pressure");
    assert_eq!(config.sensors[1].name, "flow_rate");
    assert_eq!(config.sensors[2].name, "spo2");
    assert_eq!(config.sensors[3].name, "heartbeat");
    assert_eq!(config.sensors[4].name, "alarm");
    assert_eq!(config.sensors[5].name, "valve_pos");

    // All 3 assert properties are lowered.
    assert_eq!(config.properties.len(), 3, "all assert properties should be lowered");

    // Action table matches properties 1:1.
    assert_eq!(config.action_table.len(), 3, "action table should match property count");
}

// =========================================================================
// Test 3: Reject too many signals
// =========================================================================

#[test]
fn bridge_rejects_too_many_signals() {
    // Build PipelineResult with MAX_BRIDGE_SIGNALS + 1 input signals (257).
    let count = (MAX_BRIDGE_SIGNALS + 1).min(MAX_TEST_SIGNALS);
    let signals: Vec<SignalDecl> =
        (0..count).map(|i| input_signal(&format!("s{i}"), SignalType::Unsigned(8))).collect();

    let result = stub_pipeline(signals, Vec::new());
    let err = bridge_from_pipeline(&result).expect_err("should fail with too many signals");

    assert!(
        err.iter().any(|e| matches!(e, BridgeError::TooManySignals { .. })),
        "expected TooManySignals error, got: {err:?}"
    );
}

// =========================================================================
// Test 4: Reject too many properties
// =========================================================================

#[test]
fn bridge_rejects_too_many_properties() {
    // Build PipelineResult with MAX_BRIDGE_PROPERTIES + 1 assert properties (65).
    let count = (MAX_BRIDGE_PROPERTIES + 1).min(MAX_TEST_PROPERTIES);
    let props: Vec<PropertyDecl> = (0..count)
        .map(|i| {
            assert_property(
                &format!("p{i}"),
                PropertyFormula::Always(Expr::Signal(format!("sig{i}"))),
            )
        })
        .collect();

    let result = stub_pipeline(Vec::new(), props);
    let err = bridge_from_pipeline(&result).expect_err("should fail with too many properties");

    assert!(
        err.iter().any(|e| matches!(e, BridgeError::TooManyProperties { .. })),
        "expected TooManyProperties error, got: {err:?}"
    );
}

// =========================================================================
// Test 5: Always formula extraction with signal-predicate verification
// =========================================================================

#[test]
fn bridge_extracts_always_property() {
    // Assert(Always(Signal("engine_running"))) should lower to
    // TemporalProperty::Always(SignalPredicate::IsTrue("engine_running")).
    let props = vec![assert_property(
        "p_always",
        PropertyFormula::Always(Expr::Signal("engine_running".to_string())),
    )];
    let result = stub_pipeline(Vec::new(), props);

    let config = bridge_from_pipeline(&result).expect("bridge should succeed");

    assert_eq!(config.properties.len(), 1);
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("engine_running".to_string()))
    );

    // Verify signal name is accessible through the TemporalProperty API.
    assert_eq!(config.properties[0].signal_name(), "engine_running");
}

// =========================================================================
// Test 6: EventuallyWithin formula extraction with deadline verification
// =========================================================================

#[test]
fn bridge_extracts_eventually_property() {
    // Assert(EventuallyWithin { expr: Signal("ready"), cycles: 10 }) should
    // lower to TemporalProperty::EventuallyWithin(IsTrue("ready"), 10).
    let props = vec![assert_property(
        "p_eventually",
        PropertyFormula::EventuallyWithin { expr: Expr::Signal("ready".to_string()), cycles: 10 },
    )];
    let result = stub_pipeline(Vec::new(), props);

    let config = bridge_from_pipeline(&result).expect("bridge should succeed");

    assert_eq!(config.properties.len(), 1);
    assert_eq!(
        config.properties[0],
        TemporalProperty::EventuallyWithin(SignalPredicate::IsTrue("ready".to_string()), 10)
    );

    // The deadline should be preserved exactly (u64 from u32 input).
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
    // Build 3 properties and verify each gets an action entry with graduated
    // priorities (MEGA-14): Always=200, EventuallyWithin=128.
    let props = vec![
        assert_property("p1", PropertyFormula::Always(Expr::Signal("a".to_string()))),
        assert_property("p2", PropertyFormula::Always(Expr::Signal("b".to_string()))),
        assert_property(
            "p3",
            PropertyFormula::EventuallyWithin { expr: Expr::Signal("c".to_string()), cycles: 5 },
        ),
    ];
    let result = stub_pipeline(Vec::new(), props);

    let config = bridge_from_pipeline(&result).expect("bridge should succeed");

    assert_eq!(
        config.action_table.len(),
        config.properties.len(),
        "action table must have one entry per property"
    );

    // Check graduated priorities: Always=200, EventuallyWithin=128
    let expected_priorities = [200_u8, 200, 128];
    for (i, entry) in config.action_table.iter().enumerate() {
        assert_eq!(entry.trigger_property_idx, i, "entry {i} should reference property {i}");
        assert_eq!(
            entry.priority, expected_priorities[i],
            "entry {i} should have graduated priority"
        );
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
    // Build a mix of Cover, Assume, and Assert properties.
    // Only the Assert property should appear in the lowered SimConfig.
    let props = vec![
        PropertyDecl {
            name: "cover_prop".to_string(),
            directive: PropertyDirective::Cover,
            formula: PropertyFormula::Always(Expr::Signal("x".to_string())),
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "assume_prop".to_string(),
            directive: PropertyDirective::Assume,
            formula: PropertyFormula::Always(Expr::Signal("y".to_string())),
            origin: None,
            span: None,
        },
        assert_property("assert_prop", PropertyFormula::Always(Expr::Signal("z".to_string()))),
    ];
    let result = stub_pipeline(Vec::new(), props);

    let config = bridge_from_pipeline(&result).expect("bridge should succeed");

    // Only the Assert property should be lowered.
    assert_eq!(config.properties.len(), 1, "only Assert properties should be lowered");
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("z".to_string()))
    );

    // Action table should match: exactly 1 entry for the 1 Assert property.
    assert_eq!(config.action_table.len(), 1, "action table should match property count");
    assert_eq!(config.action_table[0].trigger_property_idx, 0);
    assert_eq!(config.action_table[0].action, AdaptationAction::EmergencyStop);
}
