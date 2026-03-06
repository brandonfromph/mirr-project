use nasa_rust_project::ast::program::{MirrProgram, Module, SignalDecl, Guard, Assignment, Reflex};
use nasa_rust_project::ast::types::{SignalKind, SignalType};
use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::types::LiteralValue;
use nasa_rust_project::mirr_executor::drive_parsed_module_with_interpreter;

#[test]
fn test_guard_counter_lifetime() {
    // Build a minimal MIRR program in-memory:
    // - a guard `digit_guard` triggered by `input_byte_is_digit` for 3 cycles
    // - a reflex that sets `emit_push_integer = true` while the guard is active
    // - a clear/tick reflex that resets `emit_push_integer = false`
    let prog = MirrProgram {
        module: Module {
            name: "test".to_string(),
            signals: vec![
                SignalDecl {
                    name: "input_byte_is_digit".to_string(),
                    kind: SignalKind::Input,
                    ty: SignalType::Bool,
                },
                SignalDecl {
                    name: "emit_push_integer".to_string(),
                    kind: SignalKind::Internal,
                    ty: SignalType::Bool,
                },
            ],
            guards: vec![
                Guard {
                    name: "digit_guard".to_string(),
                    condition: Expr::Signal("input_byte_is_digit".to_string()),
                    cycles: 3,
                },
            ],
            reflexes: vec![
                Reflex {
                    name: "emit_integer".to_string(),
                    guard_names: vec!["digit_guard".to_string()],
                    assignments: vec![
                        Assignment {
                            target: "emit_push_integer".to_string(),
                            value: Expr::Literal(LiteralValue::Bool(true)),
                        }
                    ],
                },
                Reflex {
                    name: "clear_tick".to_string(),
                    guard_names: vec!["digit_guard".to_string()],
                    assignments: vec![
                        Assignment {
                            target: "emit_push_integer".to_string(),
                            value: Expr::Literal(LiteralValue::Bool(false)),
                        }
                    ],
                },
            ],
        }
    };

    // Input: a digit followed by two identifier tokens to drive total 3 ticks.
    // The guard has cycles = 3, so we expect 3 emit_push_integer events.
    let input = b"4 a b";
    let pushes = drive_parsed_module_with_interpreter(&prog, input);

    // Count integer push events and verify payload
    let int_pushes: Vec<_> = pushes.iter()
        .filter(|p| p.kind == "emit_push_integer")
        .collect();

    assert_eq!(int_pushes.len(), 3, "expected 3 integer pushes while guard active for 3 cycles");
    // The lexer captures the integer value on the digit tick; subsequent
    // guard ticks may not carry the integer payload in this minimal setup.
    assert!(int_pushes.iter().any(|p| p.int_val == Some(4)),
        "expected at least one push to carry the integer payload 4");
}