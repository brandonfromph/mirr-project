#![forbid(unsafe_code)]

fn get_registry_and_scc(
    source: &str,
    signal_name: &str,
) -> (mirrc::ecs::Registry, mirrc::width::types::SccInfo) {
    let program = mirrc::parser::parse_mirr(source).unwrap();
    let mut registry = mirrc::ecs::Registry::new();
    mirrc::ecs::adapter::ingest_program(&mut registry, program, None).unwrap();

    let mut sig_id = None;
    for (id, name) in registry.names.iter().enumerate() {
        if let Some(n) = name {
            if n.0 == signal_name {
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
fn test_solve_expansive_simple_accumulator() {
    let source = r#"
target profile {
    name: "test";
    word_size: 64;
    reg_width: 10;
    op_width: 6;
}

module top {
    signal a: internal u1;

    guard g { when true for 10 cycles; }

    reflex r {
        on g {
            a = prev(a, 1) + 5;
        }
    }
}
"#;
    let (registry, scc) = get_registry_and_scc(source, "a");
    let result = mirrc::width::scc_solver::solve_expansive(&scc, &registry);

    assert!(
        result.diagnostics.iter().all(|d| d.severity != mirrc::width::types::DiagSeverity::Error),
        "Expected no error diagnostics: {:?}",
        result.diagnostics
    );
    assert_eq!(result.widths.len(), 1);
    // 10 cycles * 5 increment = 50 max value. min_bits_for(50) = 6 bits
    assert_eq!(result.widths[0], 6);
}

#[test]
fn test_solve_expansive_reversed_operand_accumulator() {
    let source = r#"
target profile {
    name: "test";
    word_size: 64;
    reg_width: 10;
    op_width: 6;
}

module top {
    signal a: internal u1;

    guard g { when true for 100 cycles; }

    reflex r {
        on g {
            a = 3 + prev(a, 1);
        }
    }
}
"#;
    let (registry, scc) = get_registry_and_scc(source, "a");
    let result = mirrc::width::scc_solver::solve_expansive(&scc, &registry);

    assert!(
        result.diagnostics.iter().all(|d| d.severity != mirrc::width::types::DiagSeverity::Error),
        "Expected no error diagnostics: {:?}",
        result.diagnostics
    );
    assert_eq!(result.widths.len(), 1);
    // 100 cycles * 3 increment = 300 max value. min_bits_for(300) = 9 bits
    assert_eq!(result.widths[0], 9);
}

#[test]
fn test_solve_expansive_multiple_accumulators_takes_first_valid() {
    let source = r#"
target profile {
    name: "test";
    word_size: 64;
    reg_width: 10;
    op_width: 6;
}

module top {
    signal a: internal u1;

    guard g1 { when true for 5 cycles; }
    guard g2 { when true for 10 cycles; }

    reflex r1 {
        on g1 {
            a = prev(a, 1) + 2;
        }
    }

    reflex r2 {
        on g2 {
            a = prev(a, 1) + 8;
        }
    }
}
"#;
    let (registry, scc) = get_registry_and_scc(source, "a");
    let result = mirrc::width::scc_solver::solve_expansive(&scc, &registry);

    assert!(
        result.diagnostics.iter().all(|d| d.severity != mirrc::width::types::DiagSeverity::Error),
        "Expected no error diagnostics: {:?}",
        result.diagnostics
    );
    assert_eq!(result.widths.len(), 1);
    // Note: The loop in `infer_bound_from_guards` iterates through reflexes in module order.
    // The first one it finds matching the accumulator will be returned.
    // reflex1 is first, so 2 * 5 = 10 max value => min_bits_for(10) = 4.
    assert_eq!(result.widths[0], 4, "Expected first valid matching reflex to determine width");
}
