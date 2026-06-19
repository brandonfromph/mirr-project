#![forbid(unsafe_code)]

fn get_registry_and_scc(
    source: &str,
    signal_name: &str,
) -> (mirrc::ecs::Registry, mirrc::width::types::SccInfo) {
    let program = mirrc::parser::parse_mirr(source).unwrap();
    let mut registry = mirrc::ecs::Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut registry, source, None).unwrap();

    let mut sig_id = None;
    for (id, name) in registry.names.iter().enumerate() {
        if let Some(n) = name {
            if registry.resolve_name(n.0) == signal_name {
                sig_id = Some(mirrc::ecs::components::EntityId(id as u32));
                break;
            }
        }
    }
    let sig_id = sig_id.expect("Signal not found in registry");

    // Force signal to behave as unbounded (u0) to bypass typechecker early-fail
    // and correctly trigger the width inference engine
    let tc = mirrc::ecs::components::TypeComponent(mirrc::ast::types::ExtendedType::from_core(
        mirrc::ast::types::SignalType::Unsigned(0),
    ));
    registry.set_type(sig_id, tc);

    let scc = mirrc::width::types::SccInfo {
        signals: vec![sig_id],
        kind: mirrc::width::types::SccKind::Expansive,
    };

    (registry, scc)
}

#[test]
fn test_width_chaos_unbounded_expansive_loop() {
    let source = r#"
target profile {
    name: "test";
    word_size: 64;
    reg_width: 10;
    op_width: 6;
}

module top {
    signal a: internal u1;

    reflex r {
        on always {
            a = a + 1;
        }
    }
}
"#;
    let (registry, scc) = get_registry_and_scc(source, "a");
    let result = mirrc::width::scc_solver::solve_expansive(&scc, &registry);

    let err_str = format!("{:?}", result.diagnostics);
    assert!(err_str.contains("E510"), "Expected E510, got: {}", err_str);
}

#[test]
fn test_width_chaos_overflowing_inference() {
    let source = r#"
target profile {
    name: "test";
    word_size: 64;
    reg_width: 10;
    op_width: 6;
}

module top {
    signal a: internal u1;

    guard g { when true for 1152921504606846976 cycles; }

    reflex r {
        on g {
            a = prev(a, 1) + 1024;
        }
    }
}
"#;
    let (registry, scc) = get_registry_and_scc(source, "a");
    let result = mirrc::width::scc_solver::solve_expansive(&scc, &registry);

    let err_str = format!("{:?}", result.diagnostics);
    assert!(err_str.contains("E510"), "Expected E510, got: {}", err_str);
}
