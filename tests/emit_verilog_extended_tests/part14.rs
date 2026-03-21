use super::*;

// Section 14: SVA Directives (Assert, Cover, Assume)
// ===========================================================================

#[test]
fn sva_directive_assert_keyword() {
    let module = Module {
        name: "dir_assert".to_string(),
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

    assert!(sv.contains("assert property"), "Assert directive must produce 'assert property'");
}

#[test]
fn sva_directive_cover_keyword() {
    let module = Module {
        name: "dir_cover".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Cover,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("cover property"), "Cover directive must produce 'cover property'");
}

#[test]
fn sva_directive_assume_keyword() {
    let module = Module {
        name: "dir_assume".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![make_property(
            "p",
            PropertyDirective::Assume,
            PropertyFormula::Always(gt_expr(sig("s"), 0)),
        )],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("assume property"), "Assume directive must produce 'assume property'");
}

// ===========================================================================