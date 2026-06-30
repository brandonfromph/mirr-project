#![forbid(unsafe_code)]

use mirrc::ast::program::{Guard, Module};
use mirrc::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use mirrc::ast::Expr;
use mirrc::ecs::{EntityId, Registry};
use mirrc::temporal::compiler::TemporalCompiler;

#[test]
fn test_temporal_compiler_module_and_expressions() {
    let mut compiler = TemporalCompiler::new();

    // Create guards with various expressions
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
        // Unsupported or complex forms that should fail ConditionKind::try_from_ecs
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
    ];

    let ok_guards = ["guard_literal_bool", "guard_unary", "guard_binary", "guard_prev"];

    for guard in guards {
        let name = guard.name.clone();
        let mut registry = Registry::new();
        let module = Module {
            name: "test".to_string(),
            clock_domains: vec![],
            signals: vec![
                mirrc::ast::program::SignalDecl {
                    name: "sig".to_string(),
                    kind: mirrc::ast::types::SignalKind::Input,
                    ty: mirrc::ast::types::SignalType::Bool.into(),
                    span: None,
                    origin: None,
                },
                mirrc::ast::program::SignalDecl {
                    name: "a".to_string(),
                    kind: mirrc::ast::types::SignalKind::Input,
                    ty: mirrc::ast::types::SignalType::Bool.into(),
                    span: None,
                    origin: None,
                },
                mirrc::ast::program::SignalDecl {
                    name: "b".to_string(),
                    kind: mirrc::ast::types::SignalKind::Input,
                    ty: mirrc::ast::types::SignalType::Bool.into(),
                    span: None,
                    origin: None,
                },
            ],
            guards: vec![guard],
            reflexes: vec![],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        };
        registry.ingest_module(&module).unwrap();

        let mut guard_entities = Vec::new();
        for i in 0..registry.next_id().0 as usize {
            if registry.cycles[i].is_some() && registry.kinds[i].is_some() {
                guard_entities.push(EntityId(i as u32));
            }
        }

        let res = compiler.compile_module(&registry, &guard_entities);
        if ok_guards.contains(&name.as_str()) {
            assert!(
                res.is_ok(),
                "Guard '{}' should have succeeded but failed with {:?}",
                name,
                res
            );
        } else {
            assert!(
                res.is_err(),
                "Guard '{}' should have failed but succeeded with {:?}",
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
        clock_domains: vec![],
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
