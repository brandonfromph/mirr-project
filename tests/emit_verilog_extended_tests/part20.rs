use super::*;

// Section 20: Pattern Origin Annotations
// ===========================================================================

#[test]
fn pattern_origin_comment_in_output() {
    let module = Module {
        name: "pat_mod".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("b", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![make_assignment("b", sig("a"))],
            origin: Some("watchdog(10, threshold)".to_string()),
            span: None,
        }],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![mirrc::ast::pattern::PatternOrigin {
            pattern_name: "watchdog".to_string(),
            call_args_summary: "10, threshold".to_string(),
        }],
        span: None,
    };

    let result = result_from_module(module);
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("// Pattern: watchdog(10, threshold)"),
        "must emit pattern expansion annotation"
    );
}

#[test]
fn no_pattern_section_without_origins() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// ── Pattern Expansions ──"),
        "must NOT have pattern section when no pattern origins"
    );
}

// ===========================================================================
