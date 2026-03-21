use super::*;

// Section 24: All Binary Operators Render Correctly
// ===========================================================================

#[test]
fn all_binary_ops_in_expression() {
    // Test that all 13 binary operators render with correct SV syntax.
    let ops_and_symbols: [(BinaryOp, &str); 13] = [
        (BinaryOp::And, "&"),
        (BinaryOp::Or, "|"),
        (BinaryOp::Xor, "^"),
        (BinaryOp::Lt, "<"),
        (BinaryOp::Le, "<="),
        (BinaryOp::Gt, ">"),
        (BinaryOp::Ge, ">="),
        (BinaryOp::Eq, "=="),
        (BinaryOp::Ne, "!="),
        (BinaryOp::Add, "+"),
        (BinaryOp::Sub, "-"),
        (BinaryOp::Mul, "*"),
        (BinaryOp::Shl, "<<"),
    ];

    for i in 0..MAX_PROPERTY_VARIANTS {
        if i >= ops_and_symbols.len() {
            break;
        }
        let (op, expected_sym) = ops_and_symbols[i];
        let module = Module {
            name: format!("op_test_{i}"),
            signals: vec![
                signal_decl("a", SignalKind::Input, SignalType::Unsigned(8)),
                signal_decl("b", SignalKind::Input, SignalType::Unsigned(8)),
                signal_decl("out", SignalKind::Output, SignalType::Unsigned(8)),
            ],
            guards: vec![make_guard("g", gt_expr(sig("a"), 0), 1)],
            reflexes: vec![make_reflex(
                "r",
                vec!["g"],
                vec![make_assignment(
                    "out",
                    Expr::Binary { op, left: Box::new(sig("a")), right: Box::new(sig("b")) },
                )],
            )],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        };

        let result = result_from_module(module);
        let sv = verilog::emit_sv(&result);

        let expected_expr = format!("(a {expected_sym} b)");
        assert!(
            sv.contains(&expected_expr),
            "binary op {:?} must render as '{}' in SV output, got:\n{}",
            op,
            expected_expr,
            sv
        );
    }
}

// ===========================================================================