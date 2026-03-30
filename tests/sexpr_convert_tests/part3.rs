use super::*;

#[test]
fn roundtrip_expr_prev() {
    let result = roundtrip_expr(Expr::Prev { signal: "temp".to_string(), delay: 5 });
    assert_eq!(
        result,
        Expr::Prev { signal: "temp".to_string(), delay: 5 },
        "prev expression must round-trip"
    );
}

#[test]
fn roundtrip_expr_unary_not() {
    let result = roundtrip_expr(Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Literal(LiteralValue::Bool(true))),
    });
    match &result {
        Expr::Unary { op, operand } => {
            assert_eq!(*op, UnaryOp::Not, "unary op must be Not");
            assert_eq!(
                operand.as_ref(),
                &Expr::Literal(LiteralValue::Bool(true)),
                "operand must be bool true"
            );
        }
        other => panic!("expected Unary, got {:?}", other),
    }
}

#[test]
fn roundtrip_expr_unary_negate() {
    let result = roundtrip_expr(Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Literal(LiteralValue::Integer(42))),
    });
    match &result {
        Expr::Unary { op, operand } => {
            assert_eq!(*op, UnaryOp::Negate, "unary op must be Negate");
            assert_eq!(
                operand.as_ref(),
                &Expr::Literal(LiteralValue::Integer(42)),
                "operand must be integer 42"
            );
        }
        other => panic!("expected Unary, got {:?}", other),
    }
}

#[test]
fn roundtrip_expr_all_binary_ops() {
    let ops: [BinaryOp; 14] = [
        BinaryOp::And,
        BinaryOp::Or,
        BinaryOp::Xor,
        BinaryOp::Lt,
        BinaryOp::Le,
        BinaryOp::Gt,
        BinaryOp::Ge,
        BinaryOp::Eq,
        BinaryOp::Ne,
        BinaryOp::Add,
        BinaryOp::Sub,
        BinaryOp::Mul,
        BinaryOp::Shl,
        BinaryOp::Shr,
    ];
    for i in 0..14 {
        let op = ops[i];
        let expr = Expr::Binary {
            op,
            left: Box::new(Expr::Literal(LiteralValue::Integer(10))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(20))),
        };
        let result = roundtrip_expr(expr);
        match &result {
            Expr::Binary { op: result_op, left, right } => {
                assert_eq!(*result_op, op, "binary op must round-trip at index {i}");
                assert_eq!(
                    left.as_ref(),
                    &Expr::Literal(LiteralValue::Integer(10)),
                    "left operand must be 10 at index {i}"
                );
                assert_eq!(
                    right.as_ref(),
                    &Expr::Literal(LiteralValue::Integer(20)),
                    "right operand must be 20 at index {i}"
                );
            }
            other => panic!("expected Binary at index {i}, got {:?}", other),
        }
    }
}

#[test]
fn roundtrip_expr_nested_binary() {
    // (not (a and (b or c)))
    let inner = Expr::Binary {
        op: BinaryOp::Or,
        left: Box::new(Expr::Signal("b".to_string())),
        right: Box::new(Expr::Signal("c".to_string())),
    };
    let mid = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("a".to_string())),
        right: Box::new(inner),
    };
    let outer = Expr::Unary { op: UnaryOp::Not, operand: Box::new(mid) };
    let result = roundtrip_expr(outer.clone());
    assert_eq!(result, outer, "deeply nested expression must round-trip exactly");
}

// =========================================================================
// 11. Error paths in sexpr_to_ast
// =========================================================================

#[test]
fn sexpr_to_ast_rejects_non_list() {
    let sexpr = SExpr::sym("not-a-program");
    let result = sexpr_to_ast(&sexpr);
    assert!(result.is_err(), "sexpr_to_ast must reject non-list input");
}

#[test]
fn sexpr_to_ast_rejects_empty_list() {
    let sexpr = SExpr::list(Vec::new());
    let result = sexpr_to_ast(&sexpr);
    assert!(result.is_err(), "sexpr_to_ast must reject empty list");
}

#[test]
fn sexpr_to_ast_rejects_wrong_head() {
    let sexpr = SExpr::list(vec![
        SExpr::sym("not-program"),
        SExpr::list(vec![SExpr::sym("patterns")]),
        SExpr::list(vec![SExpr::sym("module"), SExpr::str_val("m")]),
    ]);
    let result = sexpr_to_ast(&sexpr);
    assert!(result.is_err(), "sexpr_to_ast must reject wrong head symbol");
}

#[test]
fn sexpr_to_ast_rejects_too_short_program() {
    let sexpr = SExpr::list(vec![SExpr::sym("program"), SExpr::list(vec![SExpr::sym("patterns")])]);
    let result = sexpr_to_ast(&sexpr);
    assert!(result.is_err(), "sexpr_to_ast must reject program with missing module");
}

// =========================================================================
// 12. Bounded iteration: multiple signals (NASA Power-of-10)
// =========================================================================

#[test]
fn bounded_multiple_signals_roundtrip() {
    let mut program = empty_program();
    let count = 32;
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration guard at index {i}");
        program.module.signals.push(make_signal(
            &format!("sig_{i}"),
            SignalKind::Input,
            SignalType::Unsigned(8),
        ));
    }
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("must succeed for multiple signals");
    assert_eq!(
        restored.module.signals.len(),
        count,
        "signal count must be preserved after round-trip"
    );
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration guard at index {i}");
        assert_eq!(
            restored.module.signals[i].name,
            format!("sig_{i}"),
            "signal name must match at index {i}"
        );
    }
}

#[test]
fn bounded_multiple_guards_roundtrip() {
    let mut program = empty_program();
    let count = 16;
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration guard at index {i}");
        program.module.guards.push(Guard {
            name: format!("g_{i}"),
            condition: Expr::Literal(LiteralValue::Bool(true)),
            cycles: (i as u64) + 1,
            origin: None,
            span: None,
        });
    }
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("must succeed for multiple guards");
    assert_eq!(restored.module.guards.len(), count, "guard count must be preserved");
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration guard at index {i}");
        assert_eq!(
            restored.module.guards[i].name,
            format!("g_{i}"),
            "guard name must match at index {i}"
        );
        assert_eq!(
            restored.module.guards[i].cycles,
            (i as u64) + 1,
            "guard cycles must match at index {i}"
        );
    }
}

// =========================================================================
// 13. Full program round-trip (comprehensive)
// =========================================================================

#[test]
fn full_program_roundtrip() {
    let mut program = empty_program();
    program.module.name = "roundtrip_sanity".to_string();
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("full program round-trip must succeed");

    assert_eq!(restored.module.name, "roundtrip_sanity");
    assert!(restored.module.signals.is_empty());
    assert!(restored.module.guards.is_empty());
    assert!(restored.module.reflexes.is_empty());
    assert!(restored.module.properties.is_empty());
    assert!(restored.module.pattern_calls.is_empty());
    assert!(restored.module.pattern_origins.is_empty());
}

// =========================================================================
// 14. Annotation combinations
// =========================================================================

#[test]
fn roundtrip_annotations_predicate_refinement() {
    let mut ann = default_annotations();
    ann.refinement = Some(Refinement::Predicate("value != 0".to_string()));
    let mut program = empty_program();
    program.module.signals.push(SignalDecl {
        name: "nonzero".to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::new(SignalType::Unsigned(8), ann),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("predicate annotation round-trip must succeed");
    let restored_ann = &restored.module.signals[0].ty.annotations;
    match &restored_ann.refinement {
        Some(Refinement::Predicate(expr)) => {
            assert_eq!(expr, "value != 0", "predicate must match");
        }
        other => panic!("expected Predicate refinement, got {:?}", other),
    }
}

#[test]
fn roundtrip_annotations_pure_effect() {
    let mut ann = default_annotations();
    ann.effect = EffectQualifier::Pure;
    let mut program = empty_program();
    program.module.signals.push(SignalDecl {
        name: "combinational".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::new(SignalType::Bool, ann),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("pure effect annotation round-trip must succeed");
    assert_eq!(
        restored.module.signals[0].ty.annotations.effect,
        EffectQualifier::Pure,
        "pure effect must round-trip"
    );
}

// =========================================================================
// 15. Pattern param with annotations
// =========================================================================

#[test]
fn roundtrip_pattern_param_signal_with_annotations() {
    let mut ann = default_annotations();
    ann.linearity = Linearity::Linear;
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "annotated_pat".to_string(),
        params: vec![PatternParam {
            name: "s".to_string(),
            kind: PatternParamKind::Signal {
                kind: SignalKind::Input,
                ty: SignalType::Unsigned(8),
                annotations: ann,
            },
        }],
        body: ReflectBlock { raw_lines: vec!["body".to_string()] },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("annotated pattern param round-trip must succeed");
    match &restored.patterns[0].params[0].kind {
        PatternParamKind::Signal { annotations, .. } => {
            assert_eq!(
                annotations.linearity,
                Linearity::Linear,
                "linearity must survive pattern param round-trip"
            );
        }
        other => panic!("expected Signal param kind, got {:?}", other),
    }
}

#[test]
fn roundtrip_pattern_param_constant_with_annotations() {
    let mut ann = default_annotations();
    ann.refinement = Some(Refinement::Range { lo: 1, hi: 100 });
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "const_pat".to_string(),
        params: vec![PatternParam {
            name: "n".to_string(),
            kind: PatternParamKind::Constant { ty: SignalType::Unsigned(16), annotations: ann },
        }],
        body: ReflectBlock { raw_lines: vec!["body".to_string()] },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("annotated constant param round-trip must succeed");
    match &restored.patterns[0].params[0].kind {
        PatternParamKind::Constant { annotations, .. } => {
            assert_eq!(
                annotations.refinement,
                Some(Refinement::Range { lo: 1, hi: 100 }),
                "refinement must survive constant param round-trip"
            );
        }
        other => panic!("expected Constant param kind, got {:?}", other),
    }
}

// =========================================================================
// 16. Edge cases
// =========================================================================

#[test]
fn roundtrip_empty_guard_names_in_reflex() {
    // A reflex with no guard names (unusual but representable)
    let mut program = empty_program();
    program.module.reflexes.push(Reflex {
        name: "r_empty".to_string(),
        guard_names: Vec::new(),
        assignments: vec![Assignment {
            target: "out".to_string(),
            value: Expr::Literal(LiteralValue::Bool(false)),
            span: None,
        }],
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("reflex with empty guard names must round-trip");
    assert!(restored.module.reflexes[0].guard_names.is_empty(), "guard_names must remain empty");
}

#[test]
fn roundtrip_pattern_no_params() {
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "noop".to_string(),
        params: Vec::new(),
        body: ReflectBlock { raw_lines: Vec::new() },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("pattern with no params must round-trip");
    assert!(restored.patterns[0].params.is_empty(), "params must remain empty");
    assert!(restored.patterns[0].body.raw_lines.is_empty(), "body must remain empty");
}

#[test]
fn roundtrip_multiple_patterns() {
    let mut program = empty_program();
    let count = 4;
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration at index {i}");
        program.patterns.push(PatternDef {
            name: format!("pat_{i}"),
            params: vec![PatternParam { name: "p".to_string(), kind: PatternParamKind::Pattern }],
            body: ReflectBlock { raw_lines: vec![format!("line_{i}")] },
            span: None,
        });
    }
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("multiple patterns must round-trip");
    assert_eq!(restored.patterns.len(), count, "pattern count must be preserved");
    for i in 0..count {
        assert!(i < MAX_TEST_ITEMS, "bounded iteration at index {i}");
        assert_eq!(
            restored.patterns[i].name,
            format!("pat_{i}"),
            "pattern name must match at index {i}"
        );
    }
}

#[test]
fn roundtrip_prev_delay_one() {
    let result = roundtrip_expr(Expr::Prev { signal: "x".to_string(), delay: 1 });
    assert_eq!(
        result,
        Expr::Prev { signal: "x".to_string(), delay: 1 },
        "prev with delay=1 must round-trip"
    );
}

#[test]
fn roundtrip_zero_width_unsigned() {
    // SignalType::Unsigned(0) is unusual but representable
    let mut program = empty_program();
    program.module.signals.push(make_signal("zero_w", SignalKind::Input, SignalType::Unsigned(0)));
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("zero-width unsigned must round-trip");
    assert_eq!(
        restored.module.signals[0].ty.core,
        SignalType::Unsigned(0),
        "zero-width must survive"
    );
}

#[test]
fn roundtrip_pattern_call_no_args() {
    let mut program = empty_program();
    program.module.pattern_calls.push(PatternCall {
        pattern_name: "no_arg_pat".to_string(),
        arguments: Vec::new(),
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("pattern call with no args must round-trip");
    assert_eq!(restored.module.pattern_calls[0].pattern_name, "no_arg_pat", "name must match");
    assert!(restored.module.pattern_calls[0].arguments.is_empty(), "args must remain empty");
}
