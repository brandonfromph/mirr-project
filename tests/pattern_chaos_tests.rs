#![forbid(unsafe_code)]

use mirrc::ast::macro_nodes::ModuleMacroStmt;
use mirrc::ast::pattern::{PatternDef, ReflectBlock};
use mirrc::ast::program::{MirrProgram, Module};
use mirrc::ecs::Registry;
use mirrc::expand::expand_patterns;

#[test]
fn test_pattern_chaos_recursion_depth_limit() {
    // Define a self-recursive pattern: A calls A.
    let patterns = vec![PatternDef {
        name: "A".to_string(),
        params: vec![],
        body: ReflectBlock {
            statements: vec![ModuleMacroStmt::PatternCall(mirrc::ast::pattern::PatternCall {
                pattern_name: "A".to_string(),
                arguments: vec![],
                span: None,
            })],
        },
        span: None,
    }];

    let mut program = MirrProgram {
        imports: vec![],
        patterns: patterns.clone(),
        module: Module {
            name: "top".to_string(),
            signals: vec![],
            guards: vec![],
            reflexes: vec![],
            properties: vec![],
            pattern_calls: vec![mirrc::ast::pattern::PatternCall {
                pattern_name: "A".to_string(),
                arguments: vec![],
                span: None,
            }],
            pattern_origins: vec![],
            span: None,
        },
    };

    let mut registry = Registry::new();
    for pat in &patterns {
        let ent = registry.create_entity(&pat.name, mirrc::ecs::components::KindComponent::PATTERN);
        registry.pattern_defs[ent.0 as usize] =
            Some(mirrc::ecs::components::PatternDefComponent(pat.clone()));
    }

    // This should fail with a PatternError (Circular reference detected)
    let result = expand_patterns(&mut program, &registry);
    assert!(result.is_err());
    let err_str = format!("{:?}", result.err());
    assert!(
        err_str.contains("Circular pattern reference"),
        "Expected circular reference error, got: {}",
        err_str
    );
}

#[test]
fn test_pattern_chaos_exponential_expansion_stress() {
    // Pattern A calls Pattern B twice.
    // ...
    // Pattern C calls Pattern D twice.
    // This creates 1 + 2 + 4 + 8 = 15 expansions.

    let mut patterns = Vec::new();
    let pat_names = ["A", "B", "C", "D"];
    for i in 0..3 {
        patterns.push(PatternDef {
            name: pat_names[i].to_string(),
            params: vec![],
            body: ReflectBlock {
                statements: vec![
                    ModuleMacroStmt::PatternCall(mirrc::ast::pattern::PatternCall {
                        pattern_name: pat_names[i + 1].to_string(),
                        arguments: vec![],
                        span: None,
                    }),
                    ModuleMacroStmt::PatternCall(mirrc::ast::pattern::PatternCall {
                        pattern_name: pat_names[i + 1].to_string(),
                        arguments: vec![],
                        span: None,
                    }),
                ],
            },
            span: None,
        });
    }
    patterns.push(PatternDef {
        name: "D".to_string(),
        params: vec![],
        body: ReflectBlock {
            statements: vec![
                ModuleMacroStmt::Signal(mirrc::ast::program::SignalDecl {
                    name: "s".to_string(),
                    kind: mirrc::ast::types::SignalKind::Internal,
                    ty: mirrc::ast::types::ExtendedType::from_core(
                        mirrc::ast::types::SignalType::Bool,
                    ),
                    origin: None,
                    span: None,
                }),
                ModuleMacroStmt::Reflex(mirrc::ast::macro_nodes::UnexpandedReflex {
                    name: "dummy".to_string(),
                    guard_names: vec![],
                    statements: vec![mirrc::ast::macro_nodes::ReflexMacroStmt::Assignment(
                        mirrc::ast::program::Assignment {
                            target: "s".to_string(),
                            value: mirrc::ast::expr::Expr::Literal(
                                mirrc::ast::types::LiteralValue::Bool(true),
                            ),
                            span: None,
                        },
                    )],
                    span: None,
                }),
            ],
        },
        span: None,
    });

    let mut program = MirrProgram {
        imports: vec![],
        patterns: patterns.clone(),
        module: Module {
            name: "top".to_string(),
            signals: vec![],
            guards: vec![],
            reflexes: vec![],
            properties: vec![],
            pattern_calls: vec![mirrc::ast::pattern::PatternCall {
                pattern_name: "A".to_string(),
                arguments: vec![],
                span: None,
            }],
            pattern_origins: vec![],
            span: None,
        },
    };

    let mut registry = Registry::new();
    for pat in &patterns {
        let ent = registry.create_entity(&pat.name, mirrc::ecs::components::KindComponent::PATTERN);
        registry.pattern_defs[ent.0 as usize] =
            Some(mirrc::ecs::components::PatternDefComponent(pat.clone()));
    }

    // Run the expansion.
    let result = expand_patterns(&mut program, &registry);
    assert!(result.is_ok(), "Expansion failed: {:?}", result.err());

    // There should be 8 signals named 's' (from 8 calls to D)
    assert_eq!(program.module.signals.len(), 8);

    let mut names: Vec<String> = program.module.signals.iter().map(|s| s.name.clone()).collect();
    names.sort();
    let old_len = names.len();
    names.dedup();
    assert_eq!(names.len(), old_len, "Generated signals are not unique!");

    for name in names {
        // Names should contain pattern name and index
        assert!(name.contains("D_"), "Name {} does not contain pattern D namespace", name);
    }
}
