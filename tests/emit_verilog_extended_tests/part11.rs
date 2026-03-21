use super::*;

// Section 11: Expression Rendering
// ===========================================================================

#[test]
fn expr_literal_bool_true() {
    let module = Module {
        name: "lit_true".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Bool),
            signal_decl("y", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", sig("x"), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", lit_bool(true))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("1'b1"), "true literal must render as 1'b1");
}

#[test]
fn expr_literal_bool_false() {
    let module = Module {
        name: "lit_false".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Bool),
            signal_decl("y", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", sig("x"), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", lit_bool(false))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("1'b0"), "false literal must render as 1'b0");
}

#[test]
fn expr_literal_integer() {
    let module = Module {
        name: "lit_int".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("y", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("x"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", lit_int(42))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("42"), "integer literal 42 must appear in output");
}

#[test]
fn expr_not_operator() {
    let module = Module {
        name: "not_op".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Bool),
            signal_decl("y", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", sig("x"), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("y", not_expr(sig("x")))])],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(!x)"), "NOT operator must render as (!x)");
}

#[test]
fn expr_multiply_operator() {
    let module = Module {
        name: "mul_op".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("b", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("out", SignalKind::Output, SignalType::Unsigned(16)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment("out", mul_expr(sig("a"), sig("b")))],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(a * b)"), "multiply must render as (a * b)");
}

// ===========================================================================