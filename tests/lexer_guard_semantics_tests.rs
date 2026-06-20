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
    let source = r#"
        module test {
            signal input_byte_is_digit: in bool;
            signal emit_push_integer: out bool;
            
            guard digit_guard {
                when input_byte_is_digit
                for 3 cycles;
            }
            
            reflex on_digit {
                on digit_guard {
                    emit_push_integer = true;
                }
            }
            
            reflex clear_tick {
                on digit_guard {
                    emit_push_integer = false;
                }
            }
        }
    "#;

    // Input: a digit followed by two identifier tokens to drive total 3 ticks.
    // The guard has cycles = 3, so we expect 3 emit_push_integer events.
    let input = b"4 a b";
    let mut reg = mirrc::ecs::Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut reg, source, None)
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
