use super::*;

// Section 27: Shift Right Operator
// ===========================================================================

#[test]
fn shr_operator_renders() {
    let module = Module {
        name: "shr_test".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Unsigned(16)),
            signal_decl("out", SignalKind::Output, SignalType::Unsigned(16)),
        ],
        guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
        reflexes: vec![make_reflex(
            "r",
            vec!["g"],
            vec![make_assignment(
                "out",
                Expr::Binary {
                    op: BinaryOp::Shr,
                    left: Box::new(sig("a")),
                    right: Box::new(lit_int(2)),
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

    assert!(sv.contains("(a >> 2)"), "SHR operator must render as (a >> 2)");
}
