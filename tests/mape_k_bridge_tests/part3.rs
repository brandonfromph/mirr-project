use super::*;

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

    assert_eq!(config.sensors.len(), 3, "three signals should produce three sensors");
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
    for i in 0..(MAX_BRIDGE_SIGNALS - 1) {
        signals.push(input_signal(&format!("s{i}"), SignalType::Bool));
    }
    // Add one output - now total is exactly MAX_BRIDGE_SIGNALS
    signals.push(output_signal("out_sig", SignalType::Bool));

    let result = stub_pipeline(signals, Vec::new());
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed with exactly MAX_BRIDGE_SIGNALS total signals");

    assert_eq!(
        config.sensors.len(),
        MAX_BRIDGE_SIGNALS,
        "exactly MAX_BRIDGE_SIGNALS signals should produce MAX_BRIDGE_SIGNALS sensors"
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

    // Sensors: all signals (airway_pressure, flow_rate, alarm, valve).
    assert_eq!(config.sensors.len(), 4, "respirator should have 4 sensors (all signals)");
    assert_eq!(config.sensors[0].name, "airway_pressure", "first sensor should be airway_pressure");
    assert_eq!(config.sensors[1].name, "flow_rate", "second sensor should be flow_rate");
    assert_eq!(config.sensors[2].name, "alarm", "third sensor should be alarm");
    assert_eq!(config.sensors[3].name, "valve", "fourth sensor should be valve");

    // Inputs are u16: midpoint = 32767, noise = 2.
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
fn bridge_always_implies_from_parsed_source_now_supported() {
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
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed for always implies from parsed source (MEGA-14)");

    assert_eq!(config.properties.len(), 1, "should have one property");
    assert_eq!(config.action_table.len(), 1, "should have one action entry");
    assert_eq!(config.action_table[0].priority, 100, "AlwaysImplies priority should be 100");
}

#[test]
fn bridge_mixed_property_types_all_supported() {
    let signals = vec![
        input_signal("alive", SignalType::Bool),
        input_signal("a", SignalType::Bool),
        input_signal("b", SignalType::Bool),
    ];
    let props = vec![
        assert_property("p_always", PropertyFormula::Always(Expr::Signal("alive".to_string()))),
        assert_property(
            "p_implies",
            PropertyFormula::AlwaysImplies {
                antecedent: Expr::Signal("a".to_string()),
                consequent: Expr::Signal("b".to_string()),
            },
        ),
    ];
    let result = stub_pipeline(signals, props);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed for mixed property types (MEGA-14)");

    // Both properties should be lowered successfully
    assert_eq!(config.properties.len(), 2, "should have two properties");
    assert_eq!(config.action_table.len(), 2, "should have two action entries");
    // Always has priority 200, AlwaysImplies has priority 100
    assert_eq!(config.action_table[0].priority, 200, "Always priority should be 200");
    assert_eq!(config.action_table[1].priority, 100, "AlwaysImplies priority should be 100");
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

    // Sensors match all signals.
    assert_eq!(config.sensors.len(), 3, "should have 3 sensors for all signals");

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

    assert_eq!(config.sensors.len(), 7, "should have 7 sensors (all signals)");

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
    use mirrc::pipeline::{run_pipeline, PipelineConfig};

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
    use mirrc::pipeline::{run_pipeline, PipelineConfig};

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
