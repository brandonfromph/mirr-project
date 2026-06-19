#![forbid(unsafe_code)]
//! Roundtrip verification: MIRR Source -> ECS Registry A -> S-Expr -> ECS Registry B -> S-Expr
//! Validates the full bidirectional fidelity of `from_sexpr.rs`.

use mirrc::ecs::EntityId;
use mirrc::ecs::Registry;
use mirrc::sexpr::convert::{registry_to_sexpr, sexpr_to_registry};
use mirrc::sexpr::print_sexpr;

fn get_module_entity(registry: &Registry) -> EntityId {
    for i in 0..registry.active_entities() {
        if let Some(kind) = registry.kinds[i].as_ref() {
            if std::mem::discriminant(&kind.0)
                == std::mem::discriminant(&mirrc::ecs::components::EntityKind::MODULE)
            {
                return EntityId(i as u32);
            }
        }
    }
    EntityId(0)
}

#[test]
fn roundtrip_module_signals_and_guards() {
    let src = r#"
        module my_core {
            signal clk: in bool;
            signal counter: internal u32;
            signal trigger: out bool;

            guard g_trigger {
                when counter > 100 for 5 cycles;
            }

            reflex handle_trigger on g_trigger {
                trigger = true;
                counter = 0;
            }
        }
    "#;

    // 1. Parse Native to Registry A
    let mut reg_a = Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut reg_a, src, None)
        .expect("Failed to parse native source");

    let mod_a = get_module_entity(&reg_a);
    let sexpr_a = registry_to_sexpr(&reg_a, mod_a);
    let text_a = print_sexpr(&sexpr_a);

    // 2. Ingest S-Expr to Registry B
    let mut reg_b = Registry::new();
    let mod_b = sexpr_to_registry(&mut reg_b, &sexpr_a).expect("Failed to parse S-Expr into Registry");

    // 3. Dump Registry B to S-Expr
    let sexpr_b = registry_to_sexpr(&reg_b, mod_b);
    let text_b = print_sexpr(&sexpr_b);

    // 4. Assert Equivalence
    assert_eq!(
        text_a, text_b,
        "Structural equivalence failed. The re-ingested S-Expression graph diverges."
    );
}
