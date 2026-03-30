use super::*;

// Section 10: Prev Register Handling
// ===========================================================================

#[test]
fn prev_delay_1_renders_as_d1() {
    let module = Module {
        name: "prev_test".to_string(),
        signals: vec![
            signal_decl("sensor", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("delta", SignalKind::Output, SignalType::Unsigned(16)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("sensor"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment("delta", sub_expr(sig("sensor"), prev_expr("sensor", 1)))],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("sensor_d1"), "prev(sensor, 1) must render as sensor_d1");
}

#[test]
fn prev_delay_3_renders_as_d3() {
    let module = Module {
        name: "prev3_test".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("y", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("x"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", prev_expr("x", 3))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("x_d3"), "prev(x, 3) must render as x_d3");
}

#[test]
fn prev_in_binary_expression() {
    let module = Module {
        name: "prev_bin".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("out", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment("out", add_expr(sig("a"), prev_expr("a", 2)))],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(a + a_d2)"), "prev in binary expr must render as (a + a_d2)");
}

// ===========================================================================
