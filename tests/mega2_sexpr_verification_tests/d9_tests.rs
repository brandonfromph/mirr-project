use super::*;

// ===========================================================================
// D9: convert_all_ast_nodes (20 tests)
// ===========================================================================

#[test]
fn test_d9_convert_signal_input_bool() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("input"), "Should contain input kind");
    assert!(s.contains("bool"), "Should contain bool type");
}

#[test]
fn test_d9_convert_signal_output_u16() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out u16;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = 42;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("output"), "Should contain output kind");
}

#[test]
fn test_d9_convert_signal_internal() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;
    signal z: internal u8;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = x;
            z = 1;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("internal"), "Should contain internal kind");
}

#[test]
fn test_d9_convert_guard() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 5 cycles;
    }

    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("guard"), "Should contain guard");
    assert!(s.contains("5"), "Should contain cycle count");
}

#[test]
fn test_d9_convert_reflex() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("reflex"), "Should contain reflex");
}

#[test]
fn test_d9_convert_binary_add() {
    let src = r#"module m {
    signal a: in u16;
    signal b: in u16;
    signal c: out u16;

    guard g {
        when a > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            c = a + b;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("+"), "Should contain + operator");
}

#[test]
fn test_d9_convert_binary_lt() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out bool;

    guard g {
        when x < 100
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("<"), "Should contain < operator");
}

#[test]
fn test_d9_convert_unary_not() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when !x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("not") || s.contains("!"), "Should contain not/!");
}

#[test]
fn test_d9_convert_literal_bool_true() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("true"), "Should contain literal true");
}

#[test]
fn test_d9_convert_literal_integer() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out u16;

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            y = 42;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("42"), "Should contain literal 42");
}

#[test]
fn test_d9_convert_prev() {
    // Prev is an AST-level construct (no parser syntax); construct AST directly.
    use nasa_rust_project::ast::expr::Expr;
    use nasa_rust_project::ast::program::*;
    use nasa_rust_project::ast::types::*;
    let m = Module {
        name: "m".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "y".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("x".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "y".to_string(),
                value: Expr::Prev { signal: "x".to_string(), delay: 1 },
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };
    let prog = nasa_rust_project::MirrProgram { module: m, imports: vec![], patterns: vec![] };
    let sexpr = ast_to_sexpr(&prog);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("prev"), "Should contain prev reference");
}

#[test]
fn test_d9_convert_module_name() {
    let src = r#"module my_mod {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("my_mod"), "Should contain module name");
}

#[test]
fn test_d9_convert_multi_assignment() {
    let src = r#"module m {
    signal x: in bool;
    signal a: out bool;
    signal b: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            a = true;
            b = false;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let rebuilt = sexpr_to_ast(&sexpr).unwrap();
    assert_eq!(rebuilt.module.reflexes[0].assignments.len(), 2);
}

#[test]
fn test_d9_convert_multi_guard_reflex() {
    let src = r#"module m {
    signal x: in bool;
    signal y: in bool;
    signal z: out bool;

    guard ga {
        when x
        for 1 cycles;
    }

    guard gb {
        when y
        for 1 cycles;
    }

    reflex r {
        on ga and gb {
            z = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let rebuilt = sexpr_to_ast(&sexpr).unwrap();
    assert_eq!(rebuilt.module.reflexes[0].guard_names.len(), 2);
}

#[test]
fn test_d9_convert_property_always() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out bool;

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property p {
        always (x > 0);
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("always"), "Should contain always property");
}

#[test]
fn test_d9_convert_property_never() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out bool;

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property p {
        never (y && x < 0);
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("never"), "Should contain never property");
}

#[test]
fn test_d9_convert_property_eventually() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property p {
        eventually within 10 (y);
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("eventually"), "Should contain eventually property");
}

#[test]
fn test_d9_convert_multiply_op() {
    let src = r#"module m {
    signal a: in u16;
    signal b: in u16;
    signal c: out u32;

    guard g {
        when a > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            c = a * b;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("*"), "Should contain * operator");
}

#[test]
fn test_d9_convert_comparison_ops() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out bool;

    guard g {
        when x >= 10
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains(">="), "Should contain >= operator");
}

#[test]
fn test_d9_convert_signed_type() {
    let src = r#"module m {
    signal x: in i16;
    signal y: out bool;

    guard g {
        when x < 0
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("i16") || s.contains("signed"), "Should contain signed type: {s}");
}
