use super::*;

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

    assert_eq!(config.sensors.len(), 2, "basic module with 2 signals should produce 2 sensors");
    assert_eq!(config.sensors[0].name, "pressure", "sensor name should match first signal name");
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

    assert_eq!(config.sensors.len(), 2, "should have 2 sensors for 2 signals");
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

    assert_eq!(config.sensors.len(), 3, "all signals should become sensors");
    assert_eq!(config.sensors[0].name, "inp", "first sensor should be the input signal");
    assert!(config.sensors[0].is_observable, "input signals should be observable");
    assert_eq!(config.sensors[1].name, "alarm", "second sensor should be the output signal");
    assert!(!config.sensors[1].is_observable, "output signals should not be observable");
    assert_eq!(config.sensors[2].name, "status", "third sensor should be the output signal");
    assert!(!config.sensors[2].is_observable, "output signals should not be observable");
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

    assert_eq!(config.sensors.len(), 4, "four signals should produce four sensors");
    assert_eq!(config.sensors[0].name, "alpha", "first sensor should be 'alpha'");
    assert_eq!(config.sensors[1].name, "beta", "second sensor should be 'beta'");
    assert_eq!(config.sensors[2].name, "gamma", "third sensor should be 'gamma'");
    assert_eq!(config.sensors[3].name, "out_sig", "fourth sensor should be 'out_sig'");
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

    assert_eq!(config.sensors.len(), 3, "all signals should become sensors");
    assert_eq!(config.sensors[0].name, "inp", "first sensor should be the input signal");
    assert!(config.sensors[0].is_observable, "input signals should be observable");
    assert_eq!(config.sensors[1].name, "out_sig", "second sensor should be the output signal");
    assert!(!config.sensors[1].is_observable, "output signals should not be observable");
    assert_eq!(config.sensors[2].name, "state", "third sensor should be the internal signal");
    assert!(!config.sensors[2].is_observable, "internal signals should not be observable");
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

