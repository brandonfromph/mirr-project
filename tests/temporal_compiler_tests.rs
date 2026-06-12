#![forbid(unsafe_code)]

use mirrc::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use mirrc::ast::{program::Guard, Expr};
use mirrc::temporal::compiler::TemporalCompiler;

#[test]
fn test_temporal_compiler_module_and_expressions() {
    let mut compiler = TemporalCompiler::new();

    // Create a guard with various expressions to test `hash_expr_stable` and `format_expr_short`
    let guards = vec![
        Guard {
            name: "guard_literal_bool".to_string(),
            condition: Expr::Literal(LiteralValue::Bool(true)),
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_literal_int".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Literal(LiteralValue::Integer(42))),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_unary".to_string(),
            condition: Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_binary".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Signal("b".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_prev".to_string(),
            condition: Expr::Prev { signal: "sig".to_string(), delay: 2 },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_array_index".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::ArrayIndex {
                    array: Box::new(Expr::Signal("arr".to_string())),
                    index: Box::new(Expr::Literal(LiteralValue::Integer(0))),
                }),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_field_access".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(Expr::FieldAccess {
                    object: Box::new(Expr::Signal("obj".to_string())),
                    field: "fld".to_string(),
                }),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_array_literal".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::ArrayLiteral(vec![
                    Expr::Literal(LiteralValue::Bool(true)),
                    Expr::Literal(LiteralValue::Bool(false)),
                ])),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_struct_literal".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(Expr::StructLiteral {
                    name: "MyStruct".to_string(),
                    fields: vec![("f1".to_string(), Expr::Literal(LiteralValue::Bool(true)))],
                }),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_unfold_index".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::UnfoldIndex("i".to_string())),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        // Deep expression to trigger depth > 3 in format_expr_short
        Guard {
            name: "guard_deep".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(Expr::Unary {
                        op: UnaryOp::Not,
                        operand: Box::new(Expr::Unary {
                            op: UnaryOp::Not,
                            operand: Box::new(Expr::Unary {
                                op: UnaryOp::Not,
                                operand: Box::new(Expr::Signal("sig".to_string())),
                            }),
                        }),
                    }),
                }),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        // Empty guard name
        Guard {
            name: "".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Signal("empty_name_sig".to_string())),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_long".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Signal(
                    "this_is_a_very_long_signal_name_that_exceeds_twenty_four_chars".to_string(),
                )),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_unary_reduction".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Unary {
                    op: UnaryOp::ReductionOr,
                    operand: Box::new(Expr::Signal("sig".to_string())),
                }),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
        Guard {
            name: "guard_unary_neg".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(Expr::Signal("sig".to_string())),
                }),
                right: Box::new(Expr::Signal("sig".to_string())),
            },
            cycles: 5,
            span: None,
            origin: None,
            template_cycles: None,
        },
    ];

    let ok_guards = [
        "guard_literal_bool",
        "guard_unary",
        "guard_binary",
        "",
        "guard_long",
        "guard_counter",
        "guard_prev",
        "guard_array_index",
    ];

    for (i, guard) in guards.into_iter().enumerate() {
        let name = guard.name.clone();
        let res = compiler.compile_module(&[guard]);
        if ok_guards.contains(&name.as_str()) {
            assert!(
                res.is_ok(),
                "Guard {} ({}) should have succeeded but failed with {:?}",
                i,
                name,
                res
            );
        } else {
            assert!(
                res.is_err(),
                "Guard {} ({}) should have failed but succeeded with {:?}",
                i,
                name,
                res
            );
        }
    }
}

#[test]
fn test_temporal_compiler_ecs_integration() {
    use mirrc::ast::program::{Module, SignalDecl};
    use mirrc::ast::types::{SignalKind, SignalType};
    use mirrc::ecs::Registry;

    let mut registry = Registry::new();
    let module = Module {
        name: "test_module".to_string(),
        signals: vec![
            SignalDecl {
                name: "sig_a".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Bool.into(),
                span: None,
                origin: None,
            },
            SignalDecl {
                name: "sig_b".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Bool.into(),
                span: None,
                origin: None,
            },
        ],
        guards: vec![
            Guard {
                name: "test_guard".to_string(),
                condition: Expr::Binary {
                    op: BinaryOp::And,
                    left: Box::new(Expr::Signal("sig_a".to_string())),
                    right: Box::new(Expr::Signal("sig_b".to_string())),
                },
                cycles: 10,
                span: None,
                origin: None,
                template_cycles: None,
            },
            Guard {
                name: "counter_guard".to_string(),
                condition: Expr::Signal("sig_a".to_string()),
                cycles: 20, // > 16 to trigger counter hardware
                span: None,
                origin: None,
                template_cycles: None,
            },
        ],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let _mod_id = registry.ingest_module(&module).expect("Failed to ingest module");

    let guard_id = registry
        .get_entity_by_name("test_module::test_guard")
        .or_else(|| registry.get_entity_by_name("test_guard"))
        .expect("Guard not found in registry");

    let counter_guard_id = registry
        .get_entity_by_name("test_module::counter_guard")
        .or_else(|| registry.get_entity_by_name("counter_guard"))
        .expect("Counter guard not found in registry");

    let mut compiler = TemporalCompiler::new();

    // Test shift register synthesis (<= 16 cycles)
    let res = compiler.lower_guard_to_ecs(&registry, guard_id);
    assert!(res.is_ok(), "Temporal ECS compilation failed for shift register: {:?}", res);

    // Test counter synthesis (> 16 cycles)
    let res2 = compiler.lower_guard_to_ecs(&registry, counter_guard_id);
    assert!(res2.is_ok(), "Temporal ECS compilation failed for counter: {:?}", res2);
}
