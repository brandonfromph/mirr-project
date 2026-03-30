use super::*;

// Section 21: Property with Origin Tag
// ===========================================================================

#[test]
fn property_origin_comment_in_sva() {
    let module = Module {
        name: "prop_origin".to_string(),
        signals: vec![
            signal_decl("s", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("o", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![make_guard("g", gt_expr(sig("s"), 0), 1)],
        reflexes: vec![make_reflex("r", vec!["g"], vec![make_assignment("o", lit_bool(true))])],
        properties: vec![PropertyDecl {
            name: "traceability_prop".to_string(),
            directive: PropertyDirective::Assert,
            formula: PropertyFormula::Always(gt_expr(sig("s"), 0)),
            origin: Some("safety_watchdog".to_string()),
            span: None,
        }],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("// Pattern: safety_watchdog"),
        "property origin tag must appear as comment in SVA output"
    );
}

// ===========================================================================
