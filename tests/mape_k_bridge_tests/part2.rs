use super::*;

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
        assert_eq!(entry.priority, 200, "action entry {} priority should be 200 (Always/Never)", i);
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
fn bridge_always_implies_now_supported() {
    let signals = vec![input_signal("a", SignalType::Bool), input_signal("b", SignalType::Bool)];
    let props = vec![assert_property(
        "p_impl",
        PropertyFormula::AlwaysImplies {
            antecedent: Expr::Signal("a".to_string()),
            consequent: Expr::Signal("b".to_string()),
        },
    )];
    let result = stub_pipeline(signals, props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for AlwaysImplies (MEGA-14)");

    assert_eq!(config.properties.len(), 1, "should have one property");
    assert_eq!(config.action_table.len(), 1, "should have one action entry");
    assert_eq!(config.action_table[0].priority, 100, "AlwaysImplies priority should be 100");
}

#[test]
fn bridge_never_implies_now_supported() {
    let signals = vec![input_signal("a", SignalType::Bool), input_signal("b", SignalType::Bool)];
    let props = vec![assert_property(
        "p_nimpl",
        PropertyFormula::NeverImplies {
            antecedent: Expr::Signal("a".to_string()),
            consequent: Expr::Signal("b".to_string()),
        },
    )];
    let result = stub_pipeline(signals, props);
    let config =
        bridge_from_pipeline(&result).expect("bridge should succeed for NeverImplies (MEGA-14)");

    assert_eq!(config.properties.len(), 1, "should have one property");
    assert_eq!(config.action_table.len(), 1, "should have one action entry");
    assert_eq!(config.action_table[0].priority, 100, "NeverImplies priority should be 100");
}

#[test]
fn bridge_always_followed_by_now_supported() {
    let signals =
        vec![input_signal("req", SignalType::Bool), input_signal("ack", SignalType::Bool)];
    let props = vec![assert_property(
        "p_afb",
        PropertyFormula::AlwaysFollowedBy {
            trigger: Expr::Signal("req".to_string()),
            response: Expr::Signal("ack".to_string()),
            delay_cycles: 5,
        },
    )];
    let result = stub_pipeline(signals, props);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed for AlwaysFollowedBy (MEGA-14)");

    assert_eq!(config.properties.len(), 1, "should have one property");
    assert_eq!(config.action_table.len(), 1, "should have one action entry");
    assert_eq!(config.action_table[0].priority, 64, "AlwaysFollowedBy priority should be 64");
}

#[test]
fn bridge_multiple_advanced_formulas_now_supported() {
    let signals = vec![
        input_signal("a", SignalType::Bool),
        input_signal("b", SignalType::Bool),
        input_signal("c", SignalType::Bool),
        input_signal("d", SignalType::Bool),
        input_signal("e", SignalType::Bool),
        input_signal("f", SignalType::Bool),
    ];
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
    let result = stub_pipeline(signals, props);
    let config = bridge_from_pipeline(&result)
        .expect("bridge should succeed for multiple advanced formulas (MEGA-14)");

    assert_eq!(config.properties.len(), 3, "should lower all three properties");
    assert_eq!(config.action_table.len(), 3, "should have three action entries");
    // Priorities: AlwaysImplies=100, NeverImplies=100, AlwaysFollowedBy=64
    assert_eq!(config.action_table[0].priority, 100, "AlwaysImplies priority");
    assert_eq!(config.action_table[1].priority, 100, "NeverImplies priority");
    assert_eq!(config.action_table[2].priority, 64, "AlwaysFollowedBy priority");
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

