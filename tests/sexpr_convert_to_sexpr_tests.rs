#![forbid(unsafe_code)]
//! AST → S-expression conversion tests for `registry_to_sexpr`.
//!
//! NASA Power-of-10: bounded iteration, no recursion.

use mirrc::ecs::EntityId;
use mirrc::ecs::Registry;
use mirrc::sexpr::convert::registry_to_sexpr;
use mirrc::sexpr::print_sexpr;

fn get_sexpr(src: &str) -> mirrc::sexpr::types::SExpr {
    let mut registry = Registry::new();
    let _ = mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut registry, src, None)
        .expect("failed to parse");

    let mut mod_id = EntityId(0);
    for i in 0..registry.active_entities() {
        if let Some(kind) = registry.kinds[i].as_ref() {
            if std::mem::discriminant(&kind.0)
                == std::mem::discriminant(&mirrc::ecs::components::EntityKind::MODULE)
            {
                mod_id = EntityId(i as u32);
                break;
            }
        }
    }
    registry_to_sexpr(&registry, mod_id)
}

#[test]
fn integer_literal_appears() {
    let sexpr = get_sexpr(
        "
    module test { 
        signal x: in bool; 
        signal v: out u8; 
        guard g { 
            when true 
        } 
        reflex r { 
            on g {
                v = 99; 
            }
        } 
    }",
    );
    let text = print_sexpr(&sexpr);
    println!("SEXPR OUTPUT: {}", text);
    assert!(text.contains("99"), "integer literal 99 must appear");
}
