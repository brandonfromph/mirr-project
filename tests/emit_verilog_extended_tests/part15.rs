use super::*;

// Section 15: SVA Disable Iff (rst_n handling)
// ===========================================================================

#[test]
fn sva_disable_iff_with_rst_n_input() {
    // When module has rst_n as input, properties should have disable iff (!rst_n)
    let module = Module {
        name: "rst_mod".to_string(),
        signals: vec![
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Assert,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("disable iff (!rst_n)"),
        "SVA must have 'disable iff (!rst_n)' when rst_n is an input signal"
    );
}

#[test]
fn sva_no_disable_iff_without_rst_n() {
    // Without rst_n input, no disable clause
    let module = Module {
        name: "no_rst".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Assert,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("disable iff"),
        "SVA must NOT have 'disable iff' without rst_n input signal"
    );
}

// ===========================================================================
