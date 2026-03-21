use super::*;

// Section 26: Edge Cases
// ===========================================================================

#[test]
fn empty_module_no_crash() {
    // Module with only IO signals, no guards or reflexes
    let result = run_pipeline(NO_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("module bare"), "must contain module name");
    assert!(sv.contains("endmodule"), "must contain endmodule");
    assert!(!sv.contains("always_ff"), "must not contain always_ff");
    assert!(!sv.contains("always_comb"), "must not contain always_comb");
}

#[test]
fn multiple_assignments_in_single_reflex() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // The reflex has two assignments: accumulator = sensor and result = accumulator
    assert!(sv.contains("accumulator"), "must contain accumulator assignment");
    assert!(sv.contains("result"), "must contain result assignment");
}

#[test]
fn condition_boolean_literal_in_comparison() {
    // Build a netlist with a bool-valued comparison condition
    let module = Module {
        name: "bool_cmp".to_string(),
        signals: vec![
            signal_decl("flag", SignalKind::Input, SignalType::Bool),
            signal_decl("out", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::Comparison {
        signal: "flag".to_string(),
        op: BinaryOp::Eq,
        value: LiteralValue::Bool(true),
    };
    let sr = ShiftRegisterGuard::new("bool_guard".to_string(), "flag".to_string(), 2, ck);
    netlist.add_guard(CompiledGuard::ShiftRegister(sr));

    let result = result_with_netlist(module, netlist);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("1'b1"), "bool comparison value true must render as 1'b1");
}

#[test]
fn condition_boolean_false_literal() {
    let module = Module {
        name: "bool_false_cmp".to_string(),
        signals: vec![
            signal_decl("flag", SignalKind::Input, SignalType::Bool),
            signal_decl("out", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::Comparison {
        signal: "flag".to_string(),
        op: BinaryOp::Eq,
        value: LiteralValue::Bool(false),
    };
    let sr = ShiftRegisterGuard::new("bool_false_g".to_string(), "flag".to_string(), 2, ck);
    netlist.add_guard(CompiledGuard::ShiftRegister(sr));

    let result = result_with_netlist(module, netlist);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("1'b0"), "bool comparison value false must render as 1'b0");
}

#[test]
fn negate_expr_renders_correctly() {
    let module = Module {
        name: "negate_test".to_string(),
        signals: vec![
            signal_decl("x", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("y", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("x"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment(
                "y",
                Expr::Unary {
                    op: nasa_rust_project::ast::types::UnaryOp::Negate,
                    operand: Box::new(sig("x")),
                },
            )],
        )],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("(-x)"), "negate operator must render as (-x)");
}

// ===========================================================================