use super::*;

#[test]
fn ast_to_sexpr_reflex_with_assignments() {
    let mut program = empty_program();
    program.module.reflexes.push(Reflex {
        name: "r_drive".to_string(),
        guard_names: vec!["g_hot".to_string(), "g_cold".to_string()],
        assignments: vec![
            Assignment {
                target: "alarm".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
                span: None,
            },
            Assignment {
                target: "count".to_string(),
                value: Expr::Literal(LiteralValue::Integer(0)),
                span: None,
            },
        ],
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let reflexes_section = module_list[4].as_list().unwrap();
    assert_eq!(reflexes_section[0].as_symbol(), Some("reflexes"), "head must be 'reflexes'");
    let reflex = reflexes_section[1].as_list().unwrap();
    assert_eq!(reflex[0].as_symbol(), Some("reflex"), "reflex head must be 'reflex'");
    assert_eq!(reflex[1].as_str_val(), Some("r_drive"), "reflex name must be 'r_drive'");

    // on-clause
    let on_clause = reflex[2].as_list().expect("on clause must be a list");
    assert_eq!(on_clause[0].as_symbol(), Some("on"), "on-clause head must be 'on'");
    assert_eq!(on_clause[1].as_str_val(), Some("g_hot"), "first guard must be 'g_hot'");
    assert_eq!(on_clause[2].as_str_val(), Some("g_cold"), "second guard must be 'g_cold'");

    // assignments
    let assign1 = reflex[3].as_list().expect("first assignment must be a list");
    assert_eq!(assign1[0].as_symbol(), Some("assign"), "assign head must be 'assign'");
    assert_eq!(assign1[1].as_str_val(), Some("alarm"), "assign target must be 'alarm'");
    assert_eq!(assign1[2].as_bool(), Some(true), "assign value must be true");

    let assign2 = reflex[4].as_list().expect("second assignment must be a list");
    assert_eq!(assign2[1].as_str_val(), Some("count"), "second assign target must be 'count'");
    assert_eq!(assign2[2].as_integer(), Some(0), "second assign value must be 0");
}

// =========================================================================
// 6. Property conversion
// =========================================================================

#[test]
fn ast_to_sexpr_property_always() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_safe".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::Always(Expr::Signal("ok".to_string())),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let props_section = module_list[5].as_list().unwrap();
    assert_eq!(props_section[0].as_symbol(), Some("properties"), "head must be 'properties'");
    let prop = props_section[1].as_list().unwrap();
    assert_eq!(prop[1].as_str_val(), Some("p_safe"), "property name must be 'p_safe'");
    assert_eq!(prop[2].as_symbol(), Some("assert"), "directive must be 'assert'");
    let formula = prop[3].as_list().expect("formula must be a list");
    assert_eq!(formula[0].as_symbol(), Some("always"), "formula head must be 'always'");
}

#[test]
fn ast_to_sexpr_property_never() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_never".to_string(),
        directive: PropertyDirective::Cover,
        formula: PropertyFormula::Never(Expr::Literal(LiteralValue::Bool(false))),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    assert_eq!(prop[2].as_symbol(), Some("cover"), "directive must be 'cover'");
    let formula = prop[3].as_list().unwrap();
    assert_eq!(formula[0].as_symbol(), Some("never"), "formula head must be 'never'");
}

#[test]
fn ast_to_sexpr_property_always_implies() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_impl".to_string(),
        directive: PropertyDirective::Assume,
        formula: PropertyFormula::AlwaysImplies {
            antecedent: Expr::Signal("a".to_string()),
            consequent: Expr::Signal("b".to_string()),
        },
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    assert_eq!(prop[2].as_symbol(), Some("assume"), "directive must be 'assume'");
    let formula = prop[3].as_list().unwrap();
    assert_eq!(
        formula[0].as_symbol(),
        Some("always-implies"),
        "formula head must be 'always-implies'"
    );
    assert_eq!(formula.len(), 3, "always-implies must have head + antecedent + consequent");
}

#[test]
fn ast_to_sexpr_property_never_implies() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_nimpl".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::NeverImplies {
            antecedent: Expr::Signal("x".to_string()),
            consequent: Expr::Signal("y".to_string()),
        },
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    let formula = prop[3].as_list().unwrap();
    assert_eq!(
        formula[0].as_symbol(),
        Some("never-implies"),
        "formula head must be 'never-implies'"
    );
}

#[test]
fn ast_to_sexpr_property_eventually_within() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_even".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::EventuallyWithin {
            expr: Expr::Signal("done".to_string()),
            cycles: 10,
        },
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    let formula = prop[3].as_list().unwrap();
    assert_eq!(
        formula[0].as_symbol(),
        Some("eventually-within"),
        "formula head must be 'eventually-within'"
    );
    assert_eq!(formula[2].as_integer(), Some(10), "cycles must be 10");
}

#[test]
fn ast_to_sexpr_property_always_followed_by() {
    let mut program = empty_program();
    program.module.properties.push(PropertyDecl {
        name: "p_follow".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::AlwaysFollowedBy {
            trigger: Expr::Signal("req".to_string()),
            response: Expr::Signal("ack".to_string()),
            delay_cycles: 5,
        },
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let prop = module_list[5].as_list().unwrap()[1].as_list().unwrap();
    let formula = prop[3].as_list().unwrap();
    assert_eq!(
        formula[0].as_symbol(),
        Some("always-followed-by"),
        "formula head must be 'always-followed-by'"
    );
    assert_eq!(formula.len(), 4, "always-followed-by must have head + trigger + response + delay");
    assert_eq!(formula[3].as_integer(), Some(5), "delay must be 5");
}

// =========================================================================
// 7. Pattern definitions
// =========================================================================

#[test]
fn ast_to_sexpr_pattern_def_with_params() {
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "monitor".to_string(),
        params: vec![
            PatternParam {
                name: "sensor".to_string(),
                kind: PatternParamKind::Signal {
                    kind: SignalKind::Input,
                    ty: SignalType::Unsigned(16),
                    annotations: default_annotations(),
                },
            },
            PatternParam {
                name: "threshold".to_string(),
                kind: PatternParamKind::Constant {
                    ty: SignalType::Unsigned(16),
                    annotations: default_annotations(),
                },
            },
            PatternParam { name: "handler".to_string(), kind: PatternParamKind::Pattern },
        ],
        body: ReflectBlock {
            raw_lines: vec!["guard g_${sensor} when ${sensor} > ${threshold} for 3;".to_string()],
        },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let items = sexpr.as_list().unwrap();
    let patterns_section = items[1].as_list().unwrap();
    let pattern_def = patterns_section[1].as_list().expect("pattern-def must be a list");
    assert_eq!(pattern_def[0].as_symbol(), Some("pattern-def"), "head must be 'pattern-def'");
    assert_eq!(pattern_def[1].as_str_val(), Some("monitor"), "pattern name must be 'monitor'");

    // params section
    let params = pattern_def[2].as_list().expect("params must be a list");
    assert_eq!(params[0].as_symbol(), Some("params"), "params head must be 'params'");
    assert_eq!(params.len(), 4, "must have head + 3 params");

    // First param: signal
    let p0 = params[1].as_list().unwrap();
    assert_eq!(p0[1].as_str_val(), Some("sensor"), "first param name must be 'sensor'");
    assert_eq!(p0[2].as_symbol(), Some("signal"), "first param kind must be 'signal'");
    assert_eq!(p0[3].as_symbol(), Some("input"), "signal kind must be 'input'");

    // Second param: constant
    let p1 = params[2].as_list().unwrap();
    assert_eq!(p1[1].as_str_val(), Some("threshold"), "second param name must be 'threshold'");
    assert_eq!(p1[2].as_symbol(), Some("constant"), "second param kind must be 'constant'");

    // Third param: pattern
    let p2 = params[3].as_list().unwrap();
    assert_eq!(p2[1].as_str_val(), Some("handler"), "third param name must be 'handler'");
    assert_eq!(p2[2].as_symbol(), Some("pattern"), "third param kind must be 'pattern'");

    // Reflect body
    let reflect = pattern_def[3].as_list().expect("reflect must be a list");
    assert_eq!(reflect[0].as_symbol(), Some("reflect"), "reflect head must be 'reflect'");
    assert_eq!(reflect.len(), 2, "reflect must have head + 1 line");
    assert!(
        reflect[1].as_str_val().unwrap().contains("${sensor}"),
        "reflect line must contain template markers"
    );
}

// =========================================================================
// 8. Pattern calls and origins
// =========================================================================

#[test]
fn ast_to_sexpr_pattern_calls() {
    let mut program = empty_program();
    program.module.pattern_calls.push(PatternCall {
        pattern_name: "monitor".to_string(),
        arguments: vec![
            PatternArg::SignalRef("pressure".to_string()),
            PatternArg::ConstInt(100),
            PatternArg::ConstBool(true),
            PatternArg::PatternRef("alert".to_string()),
        ],
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let calls_section = module_list[6].as_list().unwrap();
    assert_eq!(calls_section[0].as_symbol(), Some("pattern-calls"), "head must be 'pattern-calls'");
    let call = calls_section[1].as_list().unwrap();
    assert_eq!(call[0].as_symbol(), Some("pattern-call"), "call head must be 'pattern-call'");
    assert_eq!(call[1].as_str_val(), Some("monitor"), "call pattern name must be 'monitor'");

    // Arguments
    let arg0 = call[2].as_list().unwrap();
    assert_eq!(arg0[0].as_symbol(), Some("signal-ref"), "arg0 head must be 'signal-ref'");
    assert_eq!(arg0[1].as_str_val(), Some("pressure"), "arg0 value must be 'pressure'");

    let arg1 = call[3].as_list().unwrap();
    assert_eq!(arg1[0].as_symbol(), Some("const-int"), "arg1 head must be 'const-int'");
    assert_eq!(arg1[1].as_integer(), Some(100), "arg1 value must be 100");

    let arg2 = call[4].as_list().unwrap();
    assert_eq!(arg2[0].as_symbol(), Some("const-bool"), "arg2 head must be 'const-bool'");
    assert_eq!(arg2[1].as_bool(), Some(true), "arg2 value must be true");

    let arg3 = call[5].as_list().unwrap();
    assert_eq!(arg3[0].as_symbol(), Some("pattern-ref"), "arg3 head must be 'pattern-ref'");
    assert_eq!(arg3[1].as_str_val(), Some("alert"), "arg3 value must be 'alert'");
}

#[test]
fn ast_to_sexpr_pattern_origins() {
    let mut program = empty_program();
    program.module.pattern_origins.push(PatternOrigin {
        pattern_name: "monitor".to_string(),
        call_args_summary: "pressure, 100, true, alert".to_string(),
    });
    let sexpr = ast_to_sexpr(&program);
    let module_list = sexpr.as_list().unwrap()[2].as_list().unwrap();
    let origins_section = module_list[7].as_list().unwrap();
    assert_eq!(
        origins_section[0].as_symbol(),
        Some("pattern-origins"),
        "head must be 'pattern-origins'"
    );
    let origin = origins_section[1].as_list().unwrap();
    assert_eq!(
        origin[0].as_symbol(),
        Some("pattern-origin"),
        "origin head must be 'pattern-origin'"
    );
    assert_eq!(origin[1].as_str_val(), Some("monitor"), "origin pattern name must be 'monitor'");
    assert_eq!(
        origin[2].as_str_val(),
        Some("pressure, 100, true, alert"),
        "origin summary must match"
    );
}

// =========================================================================
// 9. S-Expr -> AST: full round-trip
// =========================================================================

#[test]
fn sexpr_to_ast_roundtrip_empty_program() {
    let program = empty_program();
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for empty program");
    assert_eq!(restored.module.name, "test_module", "module name must survive round-trip");
    assert!(restored.patterns.is_empty(), "patterns must be empty");
    assert!(restored.module.signals.is_empty(), "signals must be empty");
    assert!(restored.module.guards.is_empty(), "guards must be empty");
    assert!(restored.module.reflexes.is_empty(), "reflexes must be empty");
    assert!(restored.module.properties.is_empty(), "properties must be empty");
}

#[test]
fn sexpr_to_ast_roundtrip_signals() {
    let mut program = empty_program();
    program.module.signals.push(make_signal("clk", SignalKind::Input, SignalType::Bool));
    program.module.signals.push(make_signal("data", SignalKind::Output, SignalType::Unsigned(8)));
    program.module.signals.push(make_signal("acc", SignalKind::Internal, SignalType::Signed(16)));
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed");
    assert_eq!(restored.module.signals.len(), 3, "must have 3 signals");
    for i in 0..3 {
        assert_eq!(
            restored.module.signals[i].name, program.module.signals[i].name,
            "signal name mismatch at index {i}"
        );
        assert_eq!(
            restored.module.signals[i].kind, program.module.signals[i].kind,
            "signal kind mismatch at index {i}"
        );
        assert_eq!(
            restored.module.signals[i].ty.core, program.module.signals[i].ty.core,
            "signal type mismatch at index {i}"
        );
    }
}

#[test]
fn sexpr_to_ast_roundtrip_annotations() {
    let mut ann = default_annotations();
    ann.linearity = Linearity::Linear;
    ann.effect = EffectQualifier::Stateful;
    ann.refinement = Some(Refinement::Range { lo: 0, hi: 255 });
    ann.clock_domain = Some("sys_clk".to_string());
    ann.phantom_tag = Some("Voltage".to_string());

    let mut program = empty_program();
    program.module.signals.push(SignalDecl {
        name: "full_ann".to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::new(SignalType::Unsigned(8), ann.clone()),
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for annotated signal");
    let restored_ann = &restored.module.signals[0].ty.annotations;
    assert_eq!(restored_ann.linearity, Linearity::Linear, "linearity must round-trip");
    assert_eq!(restored_ann.effect, EffectQualifier::Stateful, "effect must round-trip");
    assert_eq!(
        restored_ann.refinement,
        Some(Refinement::Range { lo: 0, hi: 255 }),
        "refinement must round-trip"
    );
    assert_eq!(
        restored_ann.clock_domain,
        Some("sys_clk".to_string()),
        "clock_domain must round-trip"
    );
    assert_eq!(
        restored_ann.phantom_tag,
        Some("Voltage".to_string()),
        "phantom_tag must round-trip"
    );
}

#[test]
fn sexpr_to_ast_roundtrip_guards() {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g1".to_string(),
        condition: Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("temp".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(100))),
        },
        cycles: 3,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for guards");
    assert_eq!(restored.module.guards.len(), 1, "must have 1 guard");
    let g = &restored.module.guards[0];
    assert_eq!(g.name, "g1", "guard name must be 'g1'");
    assert_eq!(g.cycles, 3, "guard cycles must be 3");
    // Verify the condition structure
    match &g.condition {
        Expr::Binary { op, left, right } => {
            assert_eq!(*op, BinaryOp::Gt, "condition op must be Gt");
            match left.as_ref() {
                Expr::Signal(name) => assert_eq!(name, "temp", "left must be signal 'temp'"),
                other => panic!("expected Signal, got {:?}", other),
            }
            match right.as_ref() {
                Expr::Literal(LiteralValue::Integer(n)) => {
                    assert_eq!(*n, 100, "right must be integer 100");
                }
                other => panic!("expected Integer literal, got {:?}", other),
            }
        }
        other => panic!("expected Binary expression, got {:?}", other),
    }
}

#[test]
fn sexpr_to_ast_roundtrip_reflexes() {
    let mut program = empty_program();
    program.module.reflexes.push(Reflex {
        name: "r1".to_string(),
        guard_names: vec!["g1".to_string()],
        assignments: vec![Assignment {
            target: "alarm".to_string(),
            value: Expr::Literal(LiteralValue::Bool(true)),
            span: None,
        }],
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for reflexes");
    assert_eq!(restored.module.reflexes.len(), 1, "must have 1 reflex");
    let r = &restored.module.reflexes[0];
    assert_eq!(r.name, "r1", "reflex name must be 'r1'");
    assert_eq!(r.guard_names, vec!["g1".to_string()], "guard_names must match");
    assert_eq!(r.assignments.len(), 1, "must have 1 assignment");
    assert_eq!(r.assignments[0].target, "alarm", "assignment target must be 'alarm'");
}

#[test]
fn sexpr_to_ast_roundtrip_properties_all_formulas() {
    let mut program = empty_program();
    let sig_a = Expr::Signal("a".to_string());
    let sig_b = Expr::Signal("b".to_string());
    program.module.properties.push(PropertyDecl {
        name: "p_always".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::Always(sig_a.clone()),
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_never".to_string(),
        directive: PropertyDirective::Cover,
        formula: PropertyFormula::Never(sig_a.clone()),
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_ai".to_string(),
        directive: PropertyDirective::Assume,
        formula: PropertyFormula::AlwaysImplies {
            antecedent: sig_a.clone(),
            consequent: sig_b.clone(),
        },
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_ni".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::NeverImplies {
            antecedent: sig_a.clone(),
            consequent: sig_b.clone(),
        },
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_ew".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::EventuallyWithin { expr: sig_a.clone(), cycles: 10 },
        origin: None,
        span: None,
    });
    program.module.properties.push(PropertyDecl {
        name: "p_afb".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::AlwaysFollowedBy {
            trigger: sig_a.clone(),
            response: sig_b.clone(),
            delay_cycles: 5,
        },
        origin: None,
        span: None,
    });

    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for all formula types");
    assert_eq!(restored.module.properties.len(), 6, "must have 6 properties");

    for i in 0..6 {
        assert_eq!(
            restored.module.properties[i].name, program.module.properties[i].name,
            "property name mismatch at index {i}"
        );
        assert_eq!(
            restored.module.properties[i].directive, program.module.properties[i].directive,
            "property directive mismatch at index {i}"
        );
    }

    // Verify specific formula types
    assert!(
        matches!(restored.module.properties[0].formula, PropertyFormula::Always(_)),
        "property 0 must be Always"
    );
    assert!(
        matches!(restored.module.properties[1].formula, PropertyFormula::Never(_)),
        "property 1 must be Never"
    );
    assert!(
        matches!(restored.module.properties[2].formula, PropertyFormula::AlwaysImplies { .. }),
        "property 2 must be AlwaysImplies"
    );
    assert!(
        matches!(restored.module.properties[3].formula, PropertyFormula::NeverImplies { .. }),
        "property 3 must be NeverImplies"
    );
    match &restored.module.properties[4].formula {
        PropertyFormula::EventuallyWithin { cycles, .. } => {
            assert_eq!(*cycles, 10, "EventuallyWithin cycles must be 10");
        }
        other => panic!("property 4 must be EventuallyWithin, got {:?}", other),
    }
    match &restored.module.properties[5].formula {
        PropertyFormula::AlwaysFollowedBy { delay_cycles, .. } => {
            assert_eq!(*delay_cycles, 5, "AlwaysFollowedBy delay must be 5");
        }
        other => panic!("property 5 must be AlwaysFollowedBy, got {:?}", other),
    }
}

#[test]
fn sexpr_to_ast_roundtrip_pattern_def() {
    let mut program = empty_program();
    program.patterns.push(PatternDef {
        name: "watchdog".to_string(),
        params: vec![
            PatternParam {
                name: "sig".to_string(),
                kind: PatternParamKind::Signal {
                    kind: SignalKind::Output,
                    ty: SignalType::Bool,
                    annotations: default_annotations(),
                },
            },
            PatternParam {
                name: "limit".to_string(),
                kind: PatternParamKind::Constant {
                    ty: SignalType::Unsigned(32),
                    annotations: default_annotations(),
                },
            },
        ],
        body: ReflectBlock { raw_lines: vec!["line1".to_string(), "line2".to_string()] },
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for pattern defs");
    assert_eq!(restored.patterns.len(), 1, "must have 1 pattern");
    let p = &restored.patterns[0];
    assert_eq!(p.name, "watchdog", "pattern name must be 'watchdog'");
    assert_eq!(p.params.len(), 2, "must have 2 params");
    assert_eq!(p.params[0].name, "sig", "first param name must be 'sig'");
    assert_eq!(p.params[1].name, "limit", "second param name must be 'limit'");
    assert_eq!(p.body.raw_lines.len(), 2, "reflect body must have 2 lines");
    assert_eq!(p.body.raw_lines[0], "line1", "first reflect line must be 'line1'");
}

#[test]
fn sexpr_to_ast_roundtrip_pattern_calls() {
    let mut program = empty_program();
    program.module.pattern_calls.push(PatternCall {
        pattern_name: "test_pattern".to_string(),
        arguments: vec![
            PatternArg::SignalRef("sig1".to_string()),
            PatternArg::ConstInt(42),
            PatternArg::ConstBool(false),
            PatternArg::PatternRef("other_pat".to_string()),
        ],
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for pattern calls");
    assert_eq!(restored.module.pattern_calls.len(), 1, "must have 1 pattern call");
    let c = &restored.module.pattern_calls[0];
    assert_eq!(c.pattern_name, "test_pattern", "pattern_name must be 'test_pattern'");
    assert_eq!(c.arguments.len(), 4, "must have 4 arguments");
    assert_eq!(
        c.arguments[0],
        PatternArg::SignalRef("sig1".to_string()),
        "arg 0 must be SignalRef"
    );
    assert_eq!(c.arguments[1], PatternArg::ConstInt(42), "arg 1 must be ConstInt(42)");
    assert_eq!(c.arguments[2], PatternArg::ConstBool(false), "arg 2 must be ConstBool(false)");
    assert_eq!(
        c.arguments[3],
        PatternArg::PatternRef("other_pat".to_string()),
        "arg 3 must be PatternRef"
    );
}

#[test]
fn sexpr_to_ast_roundtrip_pattern_origins() {
    let mut program = empty_program();
    program.module.pattern_origins.push(PatternOrigin {
        pattern_name: "monitor_sensor".to_string(),
        call_args_summary: "airway_pressure, 10, 200, 500, alarm".to_string(),
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("sexpr_to_ast must succeed for pattern origins");
    assert_eq!(restored.module.pattern_origins.len(), 1, "must have 1 pattern origin");
    let o = &restored.module.pattern_origins[0];
    assert_eq!(o.pattern_name, "monitor_sensor", "pattern_name must be 'monitor_sensor'");
    assert_eq!(
        o.call_args_summary, "airway_pressure, 10, 200, 500, alarm",
        "call_args_summary must match"
    );
}

// =========================================================================
// 10. Expression round-trip: all variants
// =========================================================================

/// Helper: put an expression in a guard, round-trip, extract the condition.
fn roundtrip_expr(expr: Expr) -> Expr {
    let mut program = empty_program();
    program.module.guards.push(Guard {
        name: "g_test".to_string(),
        condition: expr,
        cycles: 1,
        origin: None,
        span: None,
    });
    let sexpr = ast_to_sexpr(&program);
    let restored = sexpr_to_ast(&sexpr).expect("expression round-trip must succeed");
    restored.module.guards[0].condition.clone()
}

#[test]
fn roundtrip_expr_bool_true() {
    let result = roundtrip_expr(Expr::Literal(LiteralValue::Bool(true)));
    assert_eq!(result, Expr::Literal(LiteralValue::Bool(true)), "bool true must round-trip");
}

#[test]
fn roundtrip_expr_bool_false() {
    let result = roundtrip_expr(Expr::Literal(LiteralValue::Bool(false)));
    assert_eq!(result, Expr::Literal(LiteralValue::Bool(false)), "bool false must round-trip");
}

#[test]
fn roundtrip_expr_integer_zero() {
    let result = roundtrip_expr(Expr::Literal(LiteralValue::Integer(0)));
    assert_eq!(result, Expr::Literal(LiteralValue::Integer(0)), "integer 0 must round-trip");
}

#[test]
fn roundtrip_expr_integer_large() {
    let result = roundtrip_expr(Expr::Literal(LiteralValue::Integer(0xFFFF_FFFF)));
    assert_eq!(
        result,
        Expr::Literal(LiteralValue::Integer(0xFFFF_FFFF)),
        "large integer must round-trip"
    );
}

#[test]
fn roundtrip_expr_signal_ref() {
    let result = roundtrip_expr(Expr::Signal("sensor_value".to_string()));
    assert_eq!(result, Expr::Signal("sensor_value".to_string()), "signal ref must round-trip");
}

