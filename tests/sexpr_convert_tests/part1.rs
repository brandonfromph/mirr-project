use super::*;

#[test]
fn ast_to_sexpr_empty_program_has_program_head() {
    let program = empty_program();
    let sexpr = ast_to_sexpr(&program);
    let items = sexpr.as_list().expect("top-level must be a list");
    assert!(
        items.len() >= 3,
        "program list must have at least 3 elements (head, patterns, module)"
    );
    assert_eq!(items[0].as_symbol(), Some("program"), "first element must be 'program' symbol");
}

#[test]
fn ast_to_sexpr_empty_patterns_section() {
    let program = empty_program();
    let sexpr = ast_to_sexpr(&program);
    let items = sexpr.as_list().unwrap();
    let patterns = items[1].as_list().expect("patterns must be a list");
    assert_eq!(
        patterns[0].as_symbol(),
        Some("patterns"),
        "patterns section must start with 'patterns' symbol"
    );
    assert_eq!(patterns.len(), 1, "empty patterns section should only contain the head symbol");
}

#[test]
fn ast_to_sexpr_module_name_preserved() {
    let program = MirrProgram {
        target: None,
        patterns: Vec::new(),
        imports: Vec::new(),
        module: empty_module("my_mod"),
    };
    let sexpr = ast_to_sexpr(&program);
    let items = sexpr.as_list().unwrap();
    let module_list = items[2].as_list().expect("module must be a list");
    assert_eq!(module_list[0].as_symbol(), Some("module"), "module section head must be 'module'");
    assert_eq!(module_list[1].as_str_val(), Some("my_mod"), "module name must match input");
}

// =========================================================================
// 2. Signal declarations
// =========================================================================

fn make_signal(name: &str, kind: SignalKind, core: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(core),
        origin: None,
        span: None,
    }
}

#[test]
fn ast_to_sexpr_signal_input_bool() {
    let mut program = empty_program();
    program.module.signals.push(make_signal("enable", SignalKind::Input, SignalType::Bool));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    // signals section is the first section after module name
    let signals_section = module_list[2].as_list().expect("signals section must be a list");
    assert_eq!(signals_section[0].as_symbol(), Some("signals"), "head must be 'signals'");
    let sig = signals_section[1].as_list().expect("signal entry must be a list");
    assert_eq!(sig[0].as_symbol(), Some("signal"), "signal entry head must be 'signal'");
    assert_eq!(sig[1].as_str_val(), Some("enable"), "signal name must be 'enable'");
    assert_eq!(sig[2].as_symbol(), Some("input"), "signal kind must be 'input'");
    assert_eq!(sig[3].as_symbol(), Some("bool"), "signal type must be 'bool'");
}

#[test]
fn ast_to_sexpr_signal_output_unsigned() {
    let mut program = empty_program();
    program.module.signals.push(make_signal(
        "data_out",
        SignalKind::Output,
        SignalType::Unsigned(16),
    ));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let signals_section = module_list[2].as_list().unwrap();
    let sig = signals_section[1].as_list().unwrap();
    assert_eq!(sig[1].as_str_val(), Some("data_out"), "signal name must be 'data_out'");
    assert_eq!(sig[2].as_symbol(), Some("output"), "signal kind must be 'output'");
    let ty = sig[3].as_list().expect("unsigned type must be a list");
    assert_eq!(ty[0].as_symbol(), Some("unsigned"), "type head must be 'unsigned'");
    assert_eq!(ty[1].as_integer(), Some(16), "width must be 16");
}

#[test]
fn ast_to_sexpr_signal_internal_signed() {
    let mut program = empty_program();
    program.module.signals.push(make_signal(
        "counter",
        SignalKind::Internal,
        SignalType::Signed(32),
    ));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let signals_section = module_list[2].as_list().unwrap();
    let sig = signals_section[1].as_list().unwrap();
    assert_eq!(sig[2].as_symbol(), Some("internal"), "signal kind must be 'internal'");
    let ty = sig[3].as_list().expect("signed type must be a list");
    assert_eq!(ty[0].as_symbol(), Some("signed"), "type head must be 'signed'");
    assert_eq!(ty[1].as_integer(), Some(32), "width must be 32");
}

#[test]
fn ast_to_sexpr_signal_all_three_kinds() {
    let mut program = empty_program();
    let kinds = [
        ("a", SignalKind::Input, "input"),
        ("b", SignalKind::Output, "output"),
        ("c", SignalKind::Internal, "internal"),
    ];
    for i in 0..3 {
        let (name, kind, _) = kinds[i];
        program.module.signals.push(make_signal(name, kind, SignalType::Bool));
    }
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let signals_section = module_list[2].as_list().unwrap();
    for i in 0..3 {
        let (_, _, expected_sym) = kinds[i];
        let sig = signals_section[i + 1].as_list().unwrap();
        assert_eq!(sig[2].as_symbol(), Some(expected_sym), "signal kind mismatch at index {i}");
    }
}

// =========================================================================
// 3. Type annotations
// =========================================================================

fn make_signal_with_annotations(name: &str, ann: TypeAnnotations) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::new(SignalType::Unsigned(8), ann),
        origin: None,
        span: None,
    }
}

#[test]
fn ast_to_sexpr_annotations_linearity_linear() {
    let mut ann = default_annotations();
    ann.linearity = Linearity::Linear;
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("lin_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    // signal, name, kind, type, annotations
    assert!(sig.len() >= 5, "signal with annotations must have at least 5 elements");
    let annotations = sig[4].as_list().expect("annotations must be a list");
    assert_eq!(annotations[0].as_symbol(), Some("annotations"), "head must be 'annotations'");
    let linearity = annotations[1].as_list().expect("linearity annotation must be a list");
    assert_eq!(linearity[0].as_symbol(), Some("linearity"), "annotation head must be 'linearity'");
    assert_eq!(linearity[1].as_symbol(), Some("linear"), "linearity value must be 'linear'");
}

#[test]
fn ast_to_sexpr_annotations_effect_stateful() {
    let mut ann = default_annotations();
    ann.effect = EffectQualifier::Stateful;
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("stateful_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let effect = annotations[1].as_list().expect("effect annotation must be a list");
    assert_eq!(effect[0].as_symbol(), Some("effect"), "annotation head must be 'effect'");
    assert_eq!(effect[1].as_symbol(), Some("stateful"), "effect value must be 'stateful'");
}

#[test]
fn ast_to_sexpr_annotations_effect_pure() {
    let mut ann = default_annotations();
    ann.effect = EffectQualifier::Pure;
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("pure_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let effect = annotations[1].as_list().unwrap();
    assert_eq!(effect[1].as_symbol(), Some("pure"), "effect value must be 'pure'");
}

#[test]
fn ast_to_sexpr_annotations_refinement_range() {
    let mut ann = default_annotations();
    ann.refinement = Some(Refinement::Range { lo: 10, hi: 200 });
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("ranged", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let refinement = annotations[1].as_list().expect("refinement annotation must be a list");
    assert_eq!(refinement[0].as_symbol(), Some("refinement"), "head must be 'refinement'");
    let range = refinement[1].as_list().expect("range must be a list");
    assert_eq!(range[0].as_symbol(), Some("range"), "range head must be 'range'");
    assert_eq!(range[1].as_integer(), Some(10), "range lo must be 10");
    assert_eq!(range[2].as_integer(), Some(200), "range hi must be 200");
}

#[test]
fn ast_to_sexpr_annotations_refinement_predicate() {
    let mut ann = default_annotations();
    ann.refinement = Some(Refinement::Predicate("value < 1024".to_string()));
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("pred_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let refinement = annotations[1].as_list().unwrap();
    let predicate = refinement[1].as_list().expect("predicate must be a list");
    assert_eq!(predicate[0].as_symbol(), Some("predicate"), "head must be 'predicate'");
    assert_eq!(predicate[1].as_str_val(), Some("value < 1024"), "predicate expression must match");
}

#[test]
fn ast_to_sexpr_annotations_clock_domain() {
    let mut ann = default_annotations();
    ann.clock_domain = Some("fast_clk".to_string());
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("clk_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let clock = annotations[1].as_list().expect("clock-domain annotation must be a list");
    assert_eq!(clock[0].as_symbol(), Some("clock-domain"), "head must be 'clock-domain'");
    assert_eq!(clock[1].as_str_val(), Some("fast_clk"), "clock domain must be 'fast_clk'");
}

#[test]
fn ast_to_sexpr_annotations_phantom_tag() {
    let mut ann = default_annotations();
    ann.phantom_tag = Some("Celsius".to_string());
    let mut program = empty_program();
    program.module.signals.push(make_signal_with_annotations("temp_sig", ann));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    let annotations = sig[4].as_list().unwrap();
    let phantom = annotations[1].as_list().expect("phantom-tag annotation must be a list");
    assert_eq!(phantom[0].as_symbol(), Some("phantom-tag"), "head must be 'phantom-tag'");
    assert_eq!(phantom[1].as_str_val(), Some("Celsius"), "phantom tag must be 'Celsius'");
}

#[test]
fn ast_to_sexpr_default_annotations_omitted() {
    let mut program = empty_program();
    program.module.signals.push(make_signal("plain", SignalKind::Input, SignalType::Bool));
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let sig = module_list[2].as_list().unwrap()[1].as_list().unwrap();
    // No annotations element should be present (only signal, name, kind, type = 4 elements)
    assert_eq!(sig.len(), 4, "signal with default annotations must have exactly 4 elements");
}

// =========================================================================
// 4. Expression conversion
// =========================================================================

#[test]
fn ast_to_sexpr_guard_with_bool_literal() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_true".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    // guards section is after signals
    let guards_section = module_list[3].as_list().unwrap();
    assert_eq!(guards_section[0].as_symbol(), Some("guards"), "head must be 'guards'");
    let guard = guards_section[1].as_list().unwrap();
    assert_eq!(guard[0].as_symbol(), Some("guard"), "guard head must be 'guard'");
    assert_eq!(guard[1].as_str_val(), Some("g_true"), "guard name must be 'g_true'");
    assert_eq!(guard[2].as_bool(), Some(true), "condition must be bool true");
    assert_eq!(guard[3].as_integer(), Some(1), "cycles must be 1");
}

#[test]
fn ast_to_sexpr_guard_with_integer_literal() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_num".to_string(),
        condition: Expr::Literal(LiteralValue::Integer(42)),
        cycles: 5,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    assert_eq!(guard[2].as_integer(), Some(42), "condition must be integer 42");
    assert_eq!(guard[3].as_integer(), Some(5), "cycles must be 5");
}

#[test]
fn ast_to_sexpr_guard_with_signal_expr() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_sig".to_string(),
        condition: Expr::Signal("enable".to_string()),
        cycles: 3,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let sig_expr = guard[2].as_list().expect("signal expr must be a list");
    assert_eq!(sig_expr[0].as_symbol(), Some("signal"), "signal expr head must be 'signal'");
    assert_eq!(sig_expr[1].as_str_val(), Some("enable"), "signal name must be 'enable'");
}

#[test]
fn ast_to_sexpr_guard_with_prev_expr() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_prev".to_string(),
        condition: Expr::Prev { signal: "temp".to_string(), delay: 2 },
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let prev_expr = guard[2].as_list().expect("prev expr must be a list");
    assert_eq!(prev_expr[0].as_symbol(), Some("prev"), "prev expr head must be 'prev'");
    assert_eq!(prev_expr[1].as_str_val(), Some("temp"), "prev signal name must be 'temp'");
    assert_eq!(prev_expr[2].as_integer(), Some(2), "prev delay must be 2");
}

#[test]
fn ast_to_sexpr_guard_with_unary_not() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_not".to_string(),
        condition: Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Literal(LiteralValue::Bool(false))),
        },
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let not_expr = guard[2].as_list().expect("not expr must be a list");
    assert_eq!(not_expr[0].as_symbol(), Some("not"), "unary expr head must be 'not'");
    assert_eq!(not_expr[1].as_bool(), Some(false), "operand must be false");
}

#[test]
fn ast_to_sexpr_guard_with_unary_negate() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_neg".to_string(),
        condition: Expr::Unary {
            op: UnaryOp::Negate,
            operand: Box::new(Expr::Literal(LiteralValue::Integer(7))),
        },
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let neg_expr = guard[2].as_list().expect("negate expr must be a list");
    assert_eq!(neg_expr[0].as_symbol(), Some("negate"), "unary expr head must be 'negate'");
    assert_eq!(neg_expr[1].as_integer(), Some(7), "operand must be 7");
}

#[test]
fn ast_to_sexpr_all_binary_operators() {
    let ops: [(BinaryOp, &str); 13] = [
        (BinaryOp::And, "and"),
        (BinaryOp::Or, "or"),
        (BinaryOp::Xor, "xor"),
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
    for i in 0..13 {
        let (op, expected_sym) = ops[i];
        let mut program = empty_program();
        program.module.guards.push(Guard {
            name: format!("g_{i}"),
            condition: Expr::Binary {
                op,
                left: Box::new(Expr::Literal(LiteralValue::Integer(1))),
                right: Box::new(Expr::Literal(LiteralValue::Integer(2))),
            },
            cycles: 1,
            template_cycles: None,
            origin: None,
            span: None,
        });
        let sexpr = ast_to_sexpr(&program);
        let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
        let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
        let bin_expr = guard[2].as_list().expect("binary expr must be a list");
        assert_eq!(
            bin_expr[0].as_symbol(),
            Some(expected_sym),
            "binary op symbol mismatch for op index {i}"
        );
        assert_eq!(bin_expr[1].as_integer(), Some(1), "left operand must be 1 for op index {i}");
        assert_eq!(bin_expr[2].as_integer(), Some(2), "right operand must be 2 for op index {i}");
    }
}

#[test]
fn ast_to_sexpr_binary_shr_operator() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_shr".to_string(),
        condition: Expr::Binary {
            op: BinaryOp::Shr,
            left: Box::new(Expr::Literal(LiteralValue::Integer(8))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
        },
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let bin_expr = guard[2].as_list().unwrap();
    assert_eq!(bin_expr[0].as_symbol(), Some(">>"), "shr operator must be '>>'");
}

#[test]
fn ast_to_sexpr_nested_binary_expression() {
    // (a + b) > 10
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_nested".to_string(),
        condition: Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Signal("b".to_string())),
            }),
            right: Box::new(Expr::Literal(LiteralValue::Integer(10))),
        },
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let guard = module_list[3].as_list().unwrap()[1].as_list().unwrap();
    let gt_expr = guard[2].as_list().unwrap();
    assert_eq!(gt_expr[0].as_symbol(), Some(">"), "outer op must be '>'");
    let add_expr = gt_expr[1].as_list().expect("left operand of '>' must be add-expression list");
    assert_eq!(add_expr[0].as_symbol(), Some("+"), "inner op must be '+'");
    assert_eq!(gt_expr[2].as_integer(), Some(10), "right operand of '>' must be 10");
}

// =========================================================================
// 5. Reflex conversion
// =========================================================================
