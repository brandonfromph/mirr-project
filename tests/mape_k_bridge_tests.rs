#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

//! Integration tests for the MAPE-K bridge module (`src/mape_k/bridge.rs`).
//!
//! Validates signal/property lowering from `PipelineResult` to `SimConfig`,
//! including sensor extraction, property lowering, action table generation,
//! and error handling for unsupported formulas and resource limits.

use nasa_rust_project::ast::program::{MirrProgram, Module};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::ast::{Expr, SignalDecl};
use nasa_rust_project::mape_k::bridge::{
    bridge_from_pipeline, BridgeError, DEFAULT_KNOWLEDGE_CAPACITY, DEFAULT_WINDOW_SIZE,
    MAX_BRIDGE_PROPERTIES, MAX_BRIDGE_SIGNALS,
};
use nasa_rust_project::mape_k::planner::{AdaptationAction, TriggerCondition};
use nasa_rust_project::mape_k::{SignalPredicate, TemporalProperty};
use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::pipeline::PipelineResult;

// ---------------------------------------------------------------------------
// Constants — bounded iteration limits (NASA P10)
// ---------------------------------------------------------------------------

const MAX_TEST_SENSORS: usize = 64;
const MAX_TEST_PROPERTIES: usize = 64;
const MAX_TEST_ACTION_ENTRIES: usize = 64;

/// PRNG seed base used by the bridge (mirrors bridge.rs constant).
const SEED_BASE: u64 = 1000;

// ---------------------------------------------------------------------------
// Helpers — no recursion, bounded iteration
// ---------------------------------------------------------------------------

/// Build a minimal `PipelineResult` with the given signals and properties.
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
    }
}

/// Parse a MIRR source string into a `PipelineResult` suitable for bridge testing.
fn parse_to_pipeline(source: &str) -> PipelineResult {
    let program = parse_mirr(source).expect("MIRR parse should succeed");
    PipelineResult {
        program,
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
    }
}

fn input_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn output_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Output,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn internal_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn assert_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Assert,
        formula,
        origin: None,
        span: None,
    }
}

fn cover_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Cover,
        formula,
        origin: None,
        span: None,
    }
}

fn assume_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Assume,
        formula,
        origin: None,
        span: None,
    }
}

// ---------------------------------------------------------------------------
// 1. Sensor extraction — through parser
// ---------------------------------------------------------------------------

#[test]
fn bridge_basic_module_produces_valid_config() {
    let source = "\
module m {
    signal pressure: in u8;
    signal alarm: out bool;
    guard g {
        when pressure
        for 1 cycles;
    }
    reflex r {
        on g {
            alarm = true;
        }
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for a basic module");

    assert_eq!(config.sensors.len(), 1, "basic module with one input should produce one sensor");
    assert_eq!(config.sensors[0].name, "pressure", "sensor name should match input signal name");
    assert_eq!(config.window_size, DEFAULT_WINDOW_SIZE, "window_size should be the default");
    assert_eq!(
        config.knowledge_capacity, DEFAULT_KNOWLEDGE_CAPACITY,
        "knowledge_capacity should be the default"
    );
}

#[test]
fn bridge_bool_input_sensor_defaults_from_source() {
    let source = "\
module m {
    signal flag: in bool;
    signal out_sig: out bool;
    guard g {
        when flag
        for 1 cycles;
    }
    reflex r {
        on g {
            out_sig = true;
        }
    }
}";
    let result = parse_to_pipeline(source);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for bool input module");

    assert_eq!(config.sensors.len(), 1, "should have exactly one sensor for one bool input");
    assert_eq!(config.sensors[0].base_value, 1, "bool sensor base_value should be 1");
    assert_eq!(
        config.sensors[0].noise_amplitude, 0,
        "bool sensor noise_amplitude should be 0 (deterministic toggle)"
    );
}

#[test]
fn bridge_unsigned_input_sensor_midpoint_from_source() {
    let source = "\
module m {
    signal data: in u8;
    signal out_sig: out bool;
    guard g {
        when data
        for 1 cycles;
    }
    reflex r {
        on g {
            out_sig = true;
        }
    }
}";
    let result = parse_to_pipeline(source);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for unsigned input module");

    // u8 max = 255, midpoint = 127
    assert_eq!(config.sensors[0].base_value, 127, "u8 sensor base_value should be midpoint 127");
    assert_eq!(
        config.sensors[0].noise_amplitude, 2,
        "u8 sensor noise_amplitude should be DEFAULT_NOISE_AMPLITUDE (2)"
    );
}

#[test]
fn bridge_output_signals_excluded_from_sensors_parsed() {
    let source = "\
module m {
    signal inp: in bool;
    signal alarm: out bool;
    signal status: out u8;
    guard g {
        when inp
        for 1 cycles;
    }
    reflex r {
        on g {
            alarm = true;
        }
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed when outputs present");

    assert_eq!(
        config.sensors.len(),
        1,
        "only input signals should become sensors; outputs excluded"
    );
    assert_eq!(config.sensors[0].name, "inp", "the sole sensor should be the input signal");
}

#[test]
fn bridge_multiple_inputs_produces_sensors_in_order() {
    let source = "\
module m {
    signal alpha: in u8;
    signal beta: in u16;
    signal gamma: in bool;
    signal out_sig: out bool;
    guard g {
        when alpha
        for 1 cycles;
    }
    reflex r {
        on g {
            out_sig = true;
        }
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for multiple inputs");

    assert_eq!(config.sensors.len(), 3, "three input signals should produce three sensors");
    assert_eq!(config.sensors[0].name, "alpha", "first sensor should be 'alpha'");
    assert_eq!(config.sensors[1].name, "beta", "second sensor should be 'beta'");
    assert_eq!(config.sensors[2].name, "gamma", "third sensor should be 'gamma'");
}

// ---------------------------------------------------------------------------
// 2. Sensor extraction — direct AST construction
// ---------------------------------------------------------------------------

#[test]
fn bridge_internal_signals_excluded_from_sensors() {
    let signals = vec![
        input_signal("inp", SignalType::Bool),
        output_signal("out_sig", SignalType::Bool),
        internal_signal("state", SignalType::Unsigned(8)),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed when internal signals present");

    assert_eq!(
        config.sensors.len(),
        1,
        "only input signals should become sensors; internals excluded"
    );
    assert_eq!(config.sensors[0].name, "inp", "the sole sensor should be the input signal");
}

#[test]
fn bridge_signed_input_sensor_centered_at_zero() {
    let signals = vec![
        input_signal("temp", SignalType::Signed(16)),
        output_signal("alarm", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for signed input");

    assert_eq!(config.sensors[0].base_value, 0, "signed sensor base_value should be centered at 0");
    assert_eq!(
        config.sensors[0].noise_amplitude, 2,
        "signed sensor noise_amplitude should be DEFAULT_NOISE_AMPLITUDE (2)"
    );
}

#[test]
fn bridge_sensor_seeds_are_sequential_from_seed_base() {
    let signals = vec![
        input_signal("s0", SignalType::Bool),
        input_signal("s1", SignalType::Bool),
        input_signal("s2", SignalType::Bool),
        output_signal("out_sig", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for sequential seed test");

    for i in 0..MAX_TEST_SENSORS.min(config.sensors.len()) {
        assert_eq!(
            config.sensors[i].seed,
            SEED_BASE.wrapping_add(i as u64),
            "sensor {} should have seed SEED_BASE + {}",
            config.sensors[i].name,
            i
        );
    }
}

#[test]
fn bridge_sensor_fault_fields_default_to_none() {
    let signals = vec![
        input_signal("pressure", SignalType::Unsigned(8)),
        output_signal("alarm", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for fault fields test");

    let sensor = &config.sensors[0];
    assert!(sensor.fault_at_tick.is_none(), "bridge-generated sensor fault_at_tick should be None");
    assert_eq!(sensor.fault_value, 0, "bridge-generated sensor fault_value should be 0");
    assert!(
        sensor.fault_end_tick.is_none(),
        "bridge-generated sensor fault_end_tick should be None"
    );
}

#[test]
fn bridge_zero_width_unsigned_sensor() {
    let signals = vec![
        input_signal("zero_w", SignalType::Unsigned(0)),
        output_signal("out_sig", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should handle unsigned(0)");

    // max_unsigned_value(0) = 0, midpoint = 0
    assert_eq!(config.sensors[0].base_value, 0, "unsigned(0) sensor base_value should be 0");
    assert_eq!(
        config.sensors[0].noise_amplitude, 0,
        "unsigned(0) sensor noise_amplitude should be 0"
    );
}

#[test]
fn bridge_wide_unsigned_sensor_64bit() {
    let signals = vec![
        input_signal("wide", SignalType::Unsigned(64)),
        output_signal("out_sig", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should handle unsigned(64)");

    // max_unsigned_value(64) = u64::MAX, midpoint = u64::MAX / 2
    let expected_midpoint = u64::MAX / 2;
    assert_eq!(
        config.sensors[0].base_value, expected_midpoint,
        "unsigned(64) sensor base_value should be u64::MAX/2"
    );
    assert_eq!(
        config.sensors[0].noise_amplitude, 2,
        "unsigned(64) sensor noise_amplitude should be 2"
    );
}

#[test]
fn bridge_narrow_unsigned_sensor_1bit() {
    let signals = vec![
        input_signal("bit", SignalType::Unsigned(1)),
        output_signal("out_sig", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should handle unsigned(1)");

    // max_unsigned_value(1) = 1, midpoint = 0
    // noise = min(2, 0) = 0
    assert_eq!(
        config.sensors[0].base_value, 0,
        "unsigned(1) sensor base_value should be 0 (midpoint of [0,1])"
    );
    assert_eq!(
        config.sensors[0].noise_amplitude, 0,
        "unsigned(1) sensor noise_amplitude should be 0 (min(2,0))"
    );
}

#[test]
fn bridge_u16_sensor_midpoint() {
    let signals = vec![
        input_signal("data16", SignalType::Unsigned(16)),
        output_signal("out_sig", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for u16 input");

    // u16 max = 65535, midpoint = 32767
    assert_eq!(
        config.sensors[0].base_value, 32767,
        "u16 sensor base_value should be midpoint 32767"
    );
}

#[test]
fn bridge_signed_1bit_sensor() {
    let signals = vec![
        input_signal("narrow_signed", SignalType::Signed(1)),
        output_signal("out_sig", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should handle signed(1)");

    // Signed(1): half = max_unsigned_value(0) = 0
    // base_value = 0, noise = min(2, 0) = 0
    assert_eq!(config.sensors[0].base_value, 0, "signed(1) sensor base_value should be 0");
    assert_eq!(
        config.sensors[0].noise_amplitude, 0,
        "signed(1) sensor noise_amplitude should be 0"
    );
}

// ---------------------------------------------------------------------------
// 3. Property lowering — through parser
// ---------------------------------------------------------------------------

#[test]
fn bridge_always_property_from_parsed_source() {
    let source = "\
module m {
    signal alive: in bool;
    signal out_sig: out bool;
    guard g {
        when alive
        for 1 cycles;
    }
    reflex r {
        on g {
            out_sig = true;
        }
    }
    property p1 {
        always (alive);
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for always property");

    assert_eq!(
        config.properties.len(),
        1,
        "one assert property should produce one temporal property"
    );
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("alive".to_string())),
        "always(signal) should lower to Always(IsTrue(signal))"
    );
}

#[test]
fn bridge_never_property_from_parsed_source() {
    let source = "\
module m {
    signal fault: in bool;
    signal out_sig: out bool;
    guard g {
        when fault
        for 1 cycles;
    }
    reflex r {
        on g {
            out_sig = true;
        }
    }
    property p_never {
        never (fault);
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for never property");

    assert_eq!(
        config.properties.len(),
        1,
        "one never-assert property should produce one temporal property"
    );
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::LessThan("fault".to_string(), 1)),
        "never(signal) should lower to Always(LessThan(signal, 1))"
    );
}

#[test]
fn bridge_eventually_within_from_parsed_source() {
    let source = "\
module m {
    signal ready: in bool;
    signal out_sig: out bool;
    guard g {
        when ready
        for 1 cycles;
    }
    reflex r {
        on g {
            out_sig = true;
        }
    }
    property p_ev {
        eventually within 10 (ready);
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed for eventually_within property");

    assert_eq!(
        config.properties.len(),
        1,
        "one eventually_within property should produce one temporal property"
    );
    assert_eq!(
        config.properties[0],
        TemporalProperty::EventuallyWithin(SignalPredicate::IsTrue("ready".to_string()), 10),
        "eventually within 10 (signal) should lower to EventuallyWithin(IsTrue(signal), 10)"
    );
}

#[test]
fn bridge_cover_property_skipped_from_source() {
    let source = "\
module m {
    signal x: in bool;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
    property p_cover {
        cover always (x);
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed when only cover properties present");

    assert!(
        config.properties.is_empty(),
        "cover properties should be skipped; no temporal properties expected"
    );
    assert!(
        config.action_table.is_empty(),
        "action table should be empty when no assert properties lowered"
    );
}

#[test]
fn bridge_assume_property_skipped_from_source() {
    let source = "\
module m {
    signal x: in bool;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
    property p_assume {
        assume always (x);
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed when only assume properties present");

    assert!(
        config.properties.is_empty(),
        "assume properties should be skipped; no temporal properties expected"
    );
}

#[test]
fn bridge_multiple_properties_from_source() {
    let source = "\
module m {
    signal alive: in bool;
    signal ready: in bool;
    signal alarm: out bool;
    guard g {
        when alive
        for 1 cycles;
    }
    reflex r {
        on g {
            alarm = true;
        }
    }
    property p_always {
        always (alive);
    }
    property p_never {
        never (ready);
    }
}";
    let result = parse_to_pipeline(source);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for multiple properties");

    assert_eq!(
        config.properties.len(),
        2,
        "two assert properties should produce two temporal properties"
    );
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("alive".to_string())),
        "first property should be Always(IsTrue(alive))"
    );
    assert_eq!(
        config.properties[1],
        TemporalProperty::Always(SignalPredicate::LessThan("ready".to_string(), 1)),
        "second property should be Always(LessThan(ready, 1)) from never()"
    );
}

#[test]
fn bridge_mixed_directives_only_assert_lowered() {
    let props = vec![
        cover_property("c1", PropertyFormula::Always(Expr::Signal("x".to_string()))),
        assert_property("a1", PropertyFormula::Always(Expr::Signal("alive".to_string()))),
        assume_property("u1", PropertyFormula::Always(Expr::Signal("y".to_string()))),
        assert_property("a2", PropertyFormula::Never(Expr::Signal("fault".to_string()))),
    ];
    let signals = vec![
        input_signal("x", SignalType::Bool),
        input_signal("alive", SignalType::Bool),
        output_signal("y", SignalType::Bool),
        input_signal("fault", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, props);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for mixed directives");

    assert_eq!(
        config.properties.len(),
        2,
        "only assert directives should be lowered; cover and assume skipped"
    );
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("alive".to_string())),
        "first lowered property should be from assert a1"
    );
    assert_eq!(
        config.properties[1],
        TemporalProperty::Always(SignalPredicate::LessThan("fault".to_string(), 1)),
        "second lowered property should be from assert a2 (never)"
    );
}

// ---------------------------------------------------------------------------
// 4. Property lowering — direct AST (binary predicates)
// ---------------------------------------------------------------------------

#[test]
fn bridge_binary_lt_expression_lowers_to_less_than() {
    let props = vec![assert_property(
        "p_lt",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Lt,
            left: Box::new(Expr::Signal("pressure".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(100))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for Lt binary expression");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::LessThan("pressure".to_string(), 100)),
        "signal < 100 should lower to LessThan(signal, 100)"
    );
}

#[test]
fn bridge_binary_gt_expression_lowers_to_greater_than() {
    let props = vec![assert_property(
        "p_gt",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("rate".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(50))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for Gt binary expression");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::GreaterThan("rate".to_string(), 50)),
        "signal > 50 should lower to GreaterThan(signal, 50)"
    );
}

#[test]
fn bridge_binary_le_expression_lowers_to_less_than_plus_one() {
    let props = vec![assert_property(
        "p_le",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Le,
            left: Box::new(Expr::Signal("level".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(200))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for Le binary expression");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::LessThan("level".to_string(), 201)),
        "signal <= 200 should lower to LessThan(signal, 201)"
    );
}

#[test]
fn bridge_binary_ge_expression_lowers_to_greater_than_minus_one() {
    let props = vec![assert_property(
        "p_ge",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Ge,
            left: Box::new(Expr::Signal("temp".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(10))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for Ge binary expression");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::GreaterThan("temp".to_string(), 9)),
        "signal >= 10 should lower to GreaterThan(signal, 9)"
    );
}

#[test]
fn bridge_binary_and_falls_back_to_is_true() {
    let props = vec![assert_property(
        "p_and",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(Expr::Signal("flag".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Bool(true))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for And binary expression");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("flag".to_string())),
        "And expression with Signal left should fall back to IsTrue(signal)"
    );
}

#[test]
fn bridge_binary_eq_falls_back_to_is_true() {
    let props = vec![assert_property(
        "p_eq",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Eq,
            left: Box::new(Expr::Signal("status".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for Eq binary expression");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("status".to_string())),
        "Eq expression with Signal left should fall back to IsTrue(signal)"
    );
}

#[test]
fn bridge_prev_expression_treated_as_signal_check() {
    let props = vec![assert_property(
        "p_prev",
        PropertyFormula::Always(Expr::Prev { signal: "prev_val".to_string(), delay: 1 }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for Prev expression");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("prev_val".to_string())),
        "Prev expression should be treated as IsTrue on the signal name"
    );
}

#[test]
fn bridge_unary_not_extracts_signal_name() {
    use nasa_rust_project::ast::types::UnaryOp;

    let props = vec![assert_property(
        "p_not",
        PropertyFormula::Always(Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Signal("active".to_string())),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for unary Not expression");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("active".to_string())),
        "unary Not wrapping a signal should extract the signal name as IsTrue"
    );
}

#[test]
fn bridge_bool_literal_in_never_formula() {
    // never(true) is equivalent to never(Literal(Bool(true)))
    // The bridge extracts signal name from Never(expr) -- Bool literal has no signal.
    let props = vec![assert_property(
        "p_lit",
        PropertyFormula::Never(Expr::Literal(LiteralValue::Bool(true))),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let err = bridge_from_pipeline(&result)
        .expect_err("bridge should fail for never(literal) with no signal");

    assert!(
        err.iter().any(|e| matches!(e, BridgeError::UnsupportedFormula { .. })),
        "should produce UnsupportedFormula error for never(literal)"
    );
}

#[test]
fn bridge_literal_only_formula_produces_error() {
    let props = vec![assert_property(
        "p_bare_lit",
        PropertyFormula::Always(Expr::Literal(LiteralValue::Integer(42))),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let err =
        bridge_from_pipeline(&result).expect_err("bridge should fail for always(bare_literal)");

    assert!(
        err.iter().any(|e| matches!(e, BridgeError::UnsupportedFormula { .. })),
        "should produce UnsupportedFormula error for bare literal in always()"
    );
}

#[test]
fn bridge_binary_with_no_signal_falls_back_to_error() {
    // Binary expression with Literal on both sides: no signal to extract.
    let props = vec![assert_property(
        "p_lit_lit",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Lt,
            left: Box::new(Expr::Literal(LiteralValue::Integer(1))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(2))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let err = bridge_from_pipeline(&result)
        .expect_err("bridge should fail for binary with no signal reference");

    assert!(
        err.iter().any(|e| matches!(e, BridgeError::UnsupportedFormula { .. })),
        "should produce UnsupportedFormula when no signal found"
    );
}

// ---------------------------------------------------------------------------
// 5. Action table generation
// ---------------------------------------------------------------------------

#[test]
fn bridge_action_table_one_entry_per_property() {
    let props = vec![
        assert_property("p1", PropertyFormula::Always(Expr::Signal("a".to_string()))),
        assert_property("p2", PropertyFormula::Always(Expr::Signal("b".to_string()))),
        assert_property("p3", PropertyFormula::Never(Expr::Signal("c".to_string()))),
    ];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for multiple properties");

    assert_eq!(
        config.action_table.len(),
        3,
        "action table should have one entry per lowered property"
    );

    for i in 0..MAX_TEST_ACTION_ENTRIES.min(config.action_table.len()) {
        let entry = &config.action_table[i];
        assert_eq!(
            entry.trigger_property_idx, i,
            "action entry {} trigger_property_idx should be {}",
            i, i
        );
        assert_eq!(
            entry.action,
            AdaptationAction::EmergencyStop,
            "action entry {} should be EmergencyStop",
            i
        );
        assert_eq!(entry.priority, 255, "action entry {} priority should be 255 (maximum)", i);
        assert_eq!(
            entry.trigger_on,
            TriggerCondition::OnViolation,
            "action entry {} should trigger OnViolation",
            i
        );
    }
}

#[test]
fn bridge_action_table_empty_when_no_properties() {
    let signals = vec![input_signal("x", SignalType::Bool), output_signal("y", SignalType::Bool)];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should succeed with no properties");

    assert!(
        config.action_table.is_empty(),
        "action table should be empty when there are no properties"
    );
}

#[test]
fn bridge_action_table_empty_when_only_cover_assume() {
    let props = vec![
        cover_property("c1", PropertyFormula::Always(Expr::Signal("x".to_string()))),
        assume_property("a1", PropertyFormula::Always(Expr::Signal("y".to_string()))),
    ];
    let result = stub_pipeline(Vec::new(), props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed with only cover/assume");

    assert!(
        config.action_table.is_empty(),
        "action table should be empty when only non-assert directives present"
    );
}

// ---------------------------------------------------------------------------
// 6. Error handling and bounds
// ---------------------------------------------------------------------------

#[test]
fn bridge_too_many_signals_produces_error() {
    let mut signals: Vec<SignalDecl> = Vec::with_capacity(MAX_BRIDGE_SIGNALS + 2);
    for i in 0..(MAX_BRIDGE_SIGNALS + 1) {
        signals.push(input_signal(&format!("s{i}"), SignalType::Unsigned(8)));
    }
    let result = stub_pipeline(signals, Vec::new());
    let err = bridge_from_pipeline(&result).expect_err("bridge should fail with too many signals");

    let has_too_many = err
        .iter()
        .any(|e| matches!(e, BridgeError::TooManySignals { count } if *count > MAX_BRIDGE_SIGNALS));
    assert!(
        has_too_many,
        "error list should contain TooManySignals with count > MAX_BRIDGE_SIGNALS"
    );
}

#[test]
fn bridge_too_many_properties_produces_error() {
    let mut props: Vec<PropertyDecl> = Vec::with_capacity(MAX_BRIDGE_PROPERTIES + 2);
    for i in 0..(MAX_BRIDGE_PROPERTIES + 1) {
        props.push(assert_property(
            &format!("p{i}"),
            PropertyFormula::Always(Expr::Signal(format!("sig{i}"))),
        ));
    }
    let result = stub_pipeline(Vec::new(), props);
    let err =
        bridge_from_pipeline(&result).expect_err("bridge should fail with too many properties");

    let has_too_many = err.iter().any(
        |e| matches!(e, BridgeError::TooManyProperties { count } if *count > MAX_BRIDGE_PROPERTIES),
    );
    assert!(
        has_too_many,
        "error list should contain TooManyProperties with count > MAX_BRIDGE_PROPERTIES"
    );
}

#[test]
fn bridge_unsupported_always_implies_produces_error() {
    let props = vec![assert_property(
        "p_impl",
        PropertyFormula::AlwaysImplies {
            antecedent: Expr::Signal("a".to_string()),
            consequent: Expr::Signal("b".to_string()),
        },
    )];
    let result = stub_pipeline(Vec::new(), props);
    let err = bridge_from_pipeline(&result).expect_err("bridge should fail for AlwaysImplies");

    assert_eq!(err.len(), 1, "should produce exactly one error");
    match &err[0] {
        BridgeError::UnsupportedFormula { description } => {
            assert!(
                description.contains("AlwaysImplies"),
                "error description should mention AlwaysImplies, got: {description}"
            );
        }
        other => panic!("expected UnsupportedFormula error, got: {other:?}"),
    }
}

#[test]
fn bridge_unsupported_never_implies_produces_error() {
    let props = vec![assert_property(
        "p_nimpl",
        PropertyFormula::NeverImplies {
            antecedent: Expr::Signal("a".to_string()),
            consequent: Expr::Signal("b".to_string()),
        },
    )];
    let result = stub_pipeline(Vec::new(), props);
    let err = bridge_from_pipeline(&result).expect_err("bridge should fail for NeverImplies");

    assert_eq!(err.len(), 1, "should produce exactly one error");
    match &err[0] {
        BridgeError::UnsupportedFormula { description } => {
            assert!(
                description.contains("NeverImplies"),
                "error description should mention NeverImplies, got: {description}"
            );
        }
        other => panic!("expected UnsupportedFormula error, got: {other:?}"),
    }
}

#[test]
fn bridge_unsupported_always_followed_by_produces_error() {
    let props = vec![assert_property(
        "p_afb",
        PropertyFormula::AlwaysFollowedBy {
            trigger: Expr::Signal("req".to_string()),
            response: Expr::Signal("ack".to_string()),
            delay_cycles: 5,
        },
    )];
    let result = stub_pipeline(Vec::new(), props);
    let err = bridge_from_pipeline(&result).expect_err("bridge should fail for AlwaysFollowedBy");

    assert_eq!(err.len(), 1, "should produce exactly one error");
    match &err[0] {
        BridgeError::UnsupportedFormula { description } => {
            assert!(
                description.contains("AlwaysFollowedBy"),
                "error description should mention AlwaysFollowedBy, got: {description}"
            );
        }
        other => panic!("expected UnsupportedFormula error, got: {other:?}"),
    }
}

#[test]
fn bridge_multiple_unsupported_formulas_collect_all_errors() {
    let props = vec![
        assert_property(
            "p1",
            PropertyFormula::AlwaysImplies {
                antecedent: Expr::Signal("a".to_string()),
                consequent: Expr::Signal("b".to_string()),
            },
        ),
        assert_property(
            "p2",
            PropertyFormula::NeverImplies {
                antecedent: Expr::Signal("c".to_string()),
                consequent: Expr::Signal("d".to_string()),
            },
        ),
        assert_property(
            "p3",
            PropertyFormula::AlwaysFollowedBy {
                trigger: Expr::Signal("e".to_string()),
                response: Expr::Signal("f".to_string()),
                delay_cycles: 3,
            },
        ),
    ];
    let result = stub_pipeline(Vec::new(), props);
    let err = bridge_from_pipeline(&result)
        .expect_err("bridge should fail for multiple unsupported formulas");

    assert_eq!(err.len(), 3, "should collect all three unsupported formula errors");
    for i in 0..MAX_TEST_PROPERTIES.min(err.len()) {
        assert!(
            matches!(&err[i], BridgeError::UnsupportedFormula { .. }),
            "error {} should be UnsupportedFormula",
            i
        );
    }
}

#[test]
fn bridge_error_display_too_many_signals() {
    let err = BridgeError::TooManySignals { count: 300 };
    let msg = format!("{err}");
    assert!(msg.contains("300"), "TooManySignals Display should include the count");
    assert!(msg.contains("256"), "TooManySignals Display should include the limit");
}

#[test]
fn bridge_error_display_too_many_properties() {
    let err = BridgeError::TooManyProperties { count: 100 };
    let msg = format!("{err}");
    assert!(msg.contains("100"), "TooManyProperties Display should include the count");
    assert!(msg.contains("64"), "TooManyProperties Display should include the limit");
}

#[test]
fn bridge_error_display_unsupported_formula() {
    let err = BridgeError::UnsupportedFormula { description: "test formula error".to_string() };
    let msg = format!("{err}");
    assert!(
        msg.contains("test formula error"),
        "UnsupportedFormula Display should include the description"
    );
}

// ---------------------------------------------------------------------------
// 7. Config defaults
// ---------------------------------------------------------------------------

#[test]
fn bridge_window_size_is_64() {
    assert_eq!(DEFAULT_WINDOW_SIZE, 64, "DEFAULT_WINDOW_SIZE should be 64");

    let result = stub_pipeline(Vec::new(), Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for empty module");
    assert_eq!(config.window_size, 64, "config window_size should be 64");
}

#[test]
fn bridge_knowledge_capacity_is_4096() {
    assert_eq!(DEFAULT_KNOWLEDGE_CAPACITY, 4096, "DEFAULT_KNOWLEDGE_CAPACITY should be 4096");

    let result = stub_pipeline(Vec::new(), Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for empty module");
    assert_eq!(config.knowledge_capacity, 4096, "config knowledge_capacity should be 4096");
}

#[test]
fn bridge_max_signals_constant_is_256() {
    assert_eq!(MAX_BRIDGE_SIGNALS, 256, "MAX_BRIDGE_SIGNALS should be 256");
}

#[test]
fn bridge_max_properties_constant_is_64() {
    assert_eq!(MAX_BRIDGE_PROPERTIES, 64, "MAX_BRIDGE_PROPERTIES should be 64");
}

// ---------------------------------------------------------------------------
// 8. Empty and edge cases
// ---------------------------------------------------------------------------

#[test]
fn bridge_empty_module_produces_empty_config() {
    let result = stub_pipeline(Vec::new(), Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for empty module");

    assert!(config.sensors.is_empty(), "empty module should produce no sensors");
    assert!(config.properties.is_empty(), "empty module should produce no properties");
    assert!(config.action_table.is_empty(), "empty module should produce no action table entries");
}

#[test]
fn bridge_signals_only_no_properties() {
    let signals = vec![
        input_signal("pressure", SignalType::Unsigned(8)),
        input_signal("temp", SignalType::Unsigned(16)),
        output_signal("alarm", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for signals only");

    assert_eq!(config.sensors.len(), 2, "two input signals should produce two sensors");
    assert!(config.properties.is_empty(), "no properties should produce empty properties");
    assert!(config.action_table.is_empty(), "no properties should produce empty action table");
}

#[test]
fn bridge_properties_only_no_signals() {
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(Vec::new(), props);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for properties only");

    assert!(config.sensors.is_empty(), "no signals should produce no sensors");
    assert_eq!(
        config.properties.len(),
        1,
        "one assert property should produce one temporal property"
    );
    assert_eq!(config.action_table.len(), 1, "one property should produce one action entry");
}

#[test]
fn bridge_exactly_max_signals_succeeds() {
    let mut signals: Vec<SignalDecl> = Vec::with_capacity(MAX_BRIDGE_SIGNALS + 1);
    for i in 0..MAX_BRIDGE_SIGNALS {
        signals.push(input_signal(&format!("s{i}"), SignalType::Bool));
    }
    // Add one output to verify it does not count towards the input limit.
    signals.push(output_signal("out_sig", SignalType::Bool));

    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed with exactly MAX_BRIDGE_SIGNALS inputs");

    assert_eq!(
        config.sensors.len(),
        MAX_BRIDGE_SIGNALS,
        "exactly MAX_BRIDGE_SIGNALS inputs should produce MAX_BRIDGE_SIGNALS sensors"
    );
}

#[test]
fn bridge_exactly_max_properties_succeeds() {
    let mut props: Vec<PropertyDecl> = Vec::with_capacity(MAX_BRIDGE_PROPERTIES);
    for i in 0..MAX_BRIDGE_PROPERTIES {
        props.push(assert_property(
            &format!("p{i}"),
            PropertyFormula::Always(Expr::Signal(format!("sig{i}"))),
        ));
    }
    let result = stub_pipeline(Vec::new(), props);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed with exactly MAX_BRIDGE_PROPERTIES");

    assert_eq!(
        config.properties.len(),
        MAX_BRIDGE_PROPERTIES,
        "exactly MAX_BRIDGE_PROPERTIES asserts should produce that many temporal properties"
    );
    assert_eq!(
        config.action_table.len(),
        MAX_BRIDGE_PROPERTIES,
        "action table should match property count at the max"
    );
}

// ---------------------------------------------------------------------------
// 9. Full round-trip through parser — complex scenarios
// ---------------------------------------------------------------------------

#[test]
fn bridge_neonatal_respirator_scenario() {
    let source = "\
module respirator {
    signal airway_pressure: in u8;
    signal flow_rate: in u8;
    signal alarm: out bool;
    signal valve: out bool;
    guard overpressure {
        when airway_pressure
        for 3 cycles;
    }
    reflex safety_clamp {
        on overpressure {
            alarm = true;
            valve = false;
        }
    }
    property p_always_alive {
        always (airway_pressure);
    }
    property p_never_zero_flow {
        never (flow_rate);
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed for neonatal respirator scenario");

    // Sensors: only inputs (airway_pressure, flow_rate).
    assert_eq!(config.sensors.len(), 2, "respirator should have 2 sensors (one per input)");
    assert_eq!(config.sensors[0].name, "airway_pressure", "first sensor should be airway_pressure");
    assert_eq!(config.sensors[1].name, "flow_rate", "second sensor should be flow_rate");

    // Both are u8: midpoint = 127, noise = 2.
    for i in 0..2 {
        assert_eq!(
            config.sensors[i].base_value, 127,
            "sensor {} base_value should be u8 midpoint 127",
            config.sensors[i].name
        );
        assert_eq!(
            config.sensors[i].noise_amplitude, 2,
            "sensor {} noise_amplitude should be 2",
            config.sensors[i].name
        );
    }

    // Properties.
    assert_eq!(config.properties.len(), 2, "respirator should have 2 temporal properties");
    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("airway_pressure".to_string())),
        "first property should be Always(IsTrue(airway_pressure))"
    );
    assert_eq!(
        config.properties[1],
        TemporalProperty::Always(SignalPredicate::LessThan("flow_rate".to_string(), 1)),
        "second property should be Always(LessThan(flow_rate, 1)) from never()"
    );

    // Action table.
    assert_eq!(config.action_table.len(), 2, "action table should have one entry per property");
}

#[test]
fn bridge_eventually_within_parsed_with_large_cycle_count() {
    let source = "\
module m {
    signal ready: in bool;
    signal done: out bool;
    guard g {
        when ready
        for 1 cycles;
    }
    reflex r {
        on g {
            done = true;
        }
    }
    property p_ev_large {
        eventually within 999 (ready);
    }
}";
    let result = parse_to_pipeline(source);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed for large cycle count eventually_within");

    assert_eq!(
        config.properties[0],
        TemporalProperty::EventuallyWithin(SignalPredicate::IsTrue("ready".to_string()), 999),
        "eventually within 999 should lower with cycle count 999"
    );
}

#[test]
fn bridge_always_implies_from_parsed_source_produces_error() {
    let source = "\
module m {
    signal a: in bool;
    signal b: in bool;
    signal y: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
    property p_impl {
        always (a -> b);
    }
}";
    let result = parse_to_pipeline(source);
    let err = bridge_from_pipeline(&result)
        .expect_err("bridge should fail for always implies from parsed source");

    assert!(
        err.iter().any(|e| matches!(e, BridgeError::UnsupportedFormula { .. })),
        "should produce UnsupportedFormula error for AlwaysImplies"
    );
}

#[test]
fn bridge_mixed_valid_and_invalid_properties() {
    let props = vec![
        assert_property("p_ok", PropertyFormula::Always(Expr::Signal("alive".to_string()))),
        assert_property(
            "p_bad",
            PropertyFormula::AlwaysImplies {
                antecedent: Expr::Signal("a".to_string()),
                consequent: Expr::Signal("b".to_string()),
            },
        ),
    ];
    let result = stub_pipeline(Vec::new(), props);
    let err = bridge_from_pipeline(&result)
        .expect_err("bridge should fail when any property is unsupported");

    // Even though p_ok is valid, the bridge still reports errors because
    // errors are collected and returned if any exist.
    assert_eq!(err.len(), 1, "should have exactly one error for the unsupported formula");
    assert!(
        matches!(&err[0], BridgeError::UnsupportedFormula { .. }),
        "the error should be UnsupportedFormula"
    );
}

#[test]
fn bridge_binary_expression_with_signal_on_right_fallback() {
    // When the left side is not a Signal but the right is, the bridge
    // should fall back to extracting signal from right.
    let props = vec![assert_property(
        "p_right",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            right: Box::new(Expr::Signal("sensor".to_string())),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed extracting signal from right side");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::IsTrue("sensor".to_string())),
        "should fall back to IsTrue on signal found on right side"
    );
}

#[test]
fn bridge_le_at_u64_max_saturates() {
    // signal <= u64::MAX should produce LessThan(signal, u64::MAX.saturating_add(1))
    // which saturates to u64::MAX.
    let props = vec![assert_property(
        "p_sat",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Le,
            left: Box::new(Expr::Signal("val".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(u64::MAX))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for Le at u64::MAX");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::LessThan("val".to_string(), u64::MAX)),
        "Le at u64::MAX should saturate to LessThan(signal, u64::MAX)"
    );
}

#[test]
fn bridge_ge_at_zero_saturates() {
    // signal >= 0 should produce GreaterThan(signal, 0u64.saturating_sub(1))
    // which saturates to 0.
    let props = vec![assert_property(
        "p_ge0",
        PropertyFormula::Always(Expr::Binary {
            op: BinaryOp::Ge,
            left: Box::new(Expr::Signal("val".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
        }),
    )];
    let result = stub_pipeline(Vec::new(), props);
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for Ge at 0");

    assert_eq!(
        config.properties[0],
        TemporalProperty::Always(SignalPredicate::GreaterThan("val".to_string(), 0)),
        "Ge at 0 should saturate to GreaterThan(signal, 0)"
    );
}

// ---------------------------------------------------------------------------
// 10. SimConfig structure verification
// ---------------------------------------------------------------------------

#[test]
fn bridge_config_sensors_properties_and_actions_are_consistent() {
    let source = "\
module m {
    signal inp1: in u8;
    signal inp2: in bool;
    signal out_sig: out bool;
    guard g {
        when inp1
        for 2 cycles;
    }
    reflex r {
        on g {
            out_sig = true;
        }
    }
    property p1 {
        always (inp1);
    }
    property p2 {
        never (inp2);
    }
    property p3 {
        eventually within 5 (inp1);
    }
}";
    let result = parse_to_pipeline(source);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for consistent config test");

    // Sensors match inputs.
    assert_eq!(config.sensors.len(), 2, "should have 2 sensors for 2 inputs");

    // Properties match assert count.
    assert_eq!(config.properties.len(), 3, "should have 3 temporal properties");

    // Action table matches properties.
    assert_eq!(
        config.action_table.len(),
        config.properties.len(),
        "action table length should equal property count"
    );

    // Each action entry index is sequential.
    for i in 0..MAX_TEST_ACTION_ENTRIES.min(config.action_table.len()) {
        assert_eq!(
            config.action_table[i].trigger_property_idx, i,
            "action entry {i} should reference property index {i}"
        );
    }
}

#[test]
fn bridge_config_from_module_with_all_signal_types() {
    let signals = vec![
        input_signal("bool_in", SignalType::Bool),
        input_signal("u8_in", SignalType::Unsigned(8)),
        input_signal("u16_in", SignalType::Unsigned(16)),
        input_signal("u32_in", SignalType::Unsigned(32)),
        input_signal("i8_in", SignalType::Signed(8)),
        input_signal("i16_in", SignalType::Signed(16)),
        output_signal("out_sig", SignalType::Bool),
    ];
    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result).expect("bridge should succeed for all signal types");

    assert_eq!(config.sensors.len(), 6, "should have 6 sensors (all inputs, no output)");

    // Verify type-specific heuristics.
    // Bool: base=1, noise=0
    assert_eq!(config.sensors[0].base_value, 1, "bool sensor base should be 1");
    assert_eq!(config.sensors[0].noise_amplitude, 0, "bool sensor noise should be 0");

    // u8: midpoint=127, noise=2
    assert_eq!(config.sensors[1].base_value, 127, "u8 sensor base should be 127");

    // u16: midpoint=32767, noise=2
    assert_eq!(config.sensors[2].base_value, 32767, "u16 sensor base should be 32767");

    // u32: midpoint = (2^32 - 1)/2 = 2147483647
    assert_eq!(config.sensors[3].base_value, 2_147_483_647, "u32 sensor base should be 2147483647");

    // i8: base=0, noise=min(2, max_unsigned_value(7))=min(2,127)=2
    assert_eq!(config.sensors[4].base_value, 0, "i8 sensor base should be 0");
    assert_eq!(config.sensors[4].noise_amplitude, 2, "i8 sensor noise should be 2");

    // i16: base=0, noise=min(2, max_unsigned_value(15))=min(2,32767)=2
    assert_eq!(config.sensors[5].base_value, 0, "i16 sensor base should be 0");
    assert_eq!(config.sensors[5].noise_amplitude, 2, "i16 sensor noise should be 2");
}

// ---------------------------------------------------------------------------
// 11. Full pipeline integration with MAPE-K stage
// ---------------------------------------------------------------------------

#[test]
fn bridge_full_pipeline_with_mape_k_enabled() {
    use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

    let source = "\
module m {
    signal x: in bool;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
    property p1 {
        always (x);
    }
}";
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: true,
        ..PipelineConfig::default()
    };
    let result =
        run_pipeline(source, &config).expect("full pipeline should succeed for simple module");

    // The pipeline itself runs bridge_from_pipeline internally and stores
    // the result in mape_k_result.
    assert!(result.mape_k_result.is_some(), "MAPE-K result should be present when mape_k=true");

    let mk = result.mape_k_result.as_ref().unwrap();
    assert!(mk.total_ticks > 0, "MAPE-K simulation should have run at least one tick");
}

#[test]
fn bridge_full_pipeline_without_mape_k_disabled() {
    use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

    let source = "\
module m {
    signal x: in bool;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}";
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };
    let result =
        run_pipeline(source, &config).expect("full pipeline should succeed for simple module");

    assert!(result.mape_k_result.is_none(), "MAPE-K result should be None when mape_k=false");
}
