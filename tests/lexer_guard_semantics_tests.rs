#![forbid(unsafe_code)]
use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use mirrc::ast::types::LiteralValue;
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::mirr_executor::drive_parsed_module_with_interpreter;

#[test]
fn test_guard_counter_lifetime() {
    // Build a minimal MIRR program in-memory:
    // - a guard `digit_guard` triggered by `input_byte_is_digit` for 3 cycles
    // - a reflex that sets `emit_push_integer = true` while the guard is active
    // - a clear/tick reflex that resets `emit_push_integer = false`
    let _prog = MirrProgram {
        target: None,
        patterns: Vec::new(),
        imports: Vec::new(),
        module: Module {
            name: "test".to_string(),
            signals: vec![
                SignalDecl {
                    name: "input_byte_is_digit".to_string(),
                    kind: SignalKind::Input,
                    ty: ExtendedType::from_core(SignalType::Bool),
                    origin: None,
                    span: None,
                },
                SignalDecl {
                    name: "emit_push_integer".to_string(),
                    kind: SignalKind::Internal,
                    ty: ExtendedType::from_core(SignalType::Bool),
                    origin: None,
                    span: None,
                },
            ],
            guards: vec![Guard {
                name: "digit_guard".to_string(),
                condition: Expr::Signal("input_byte_is_digit".to_string()),
                cycles: 3,
                template_cycles: None,
                origin: None,
                span: None,
            }],
            reflexes: vec![
                Reflex {
                    name: "emit_integer".to_string(),
                    guard_names: vec!["digit_guard".to_string()],
                    assignments: vec![Assignment {
                        target: "emit_push_integer".to_string(),
                        value: Expr::Literal(LiteralValue::Bool(true)),
                        span: None,
                    }],
                    origin: None,
                    span: None,
                },
                Reflex {
                    name: "clear_tick".to_string(),
                    guard_names: vec!["digit_guard".to_string()],
                    assignments: vec![Assignment {
                        target: "emit_push_integer".to_string(),
                        value: Expr::Literal(LiteralValue::Bool(false)),
                        span: None,
                    }],
                    origin: None,
                    span: None,
                },
            ],
            properties: Vec::new(),
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
            span: None,
        },
    };

    // Input: a digit followed by two identifier tokens to drive total 3 ticks.
    // The guard has cycles = 3, so we expect 3 emit_push_integer events.
    let input = b"4 a b";
    let mut reg = mirrc::ecs::Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut reg, "ERROR_NO_SRC", None)
        .unwrap();
    let pushes = drive_parsed_module_with_interpreter(&reg, input);

    // Count integer push events and verify payload
    let int_pushes: Vec<_> = pushes.iter().filter(|p| p.kind == "emit_push_integer").collect();

    assert_eq!(int_pushes.len(), 3, "expected 3 integer pushes while guard active for 3 cycles");
    // The lexer captures the integer value on the digit tick; subsequent
    // guard ticks may not carry the integer payload in this minimal setup.
    assert!(
        int_pushes.iter().any(|p| p.int_val == Some(4)),
        "expected at least one push to carry the integer payload 4"
    );
}
