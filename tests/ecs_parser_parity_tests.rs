use mirrc::ecs::adapter::ingest_program;
use mirrc::ecs::Registry;
use mirrc::parser::ecs_parser::parse_mirr_ecs;
use mirrc::parser::module_parser::parse_mirr;

/// Compares two registries for semantic parity.
fn assert_registry_parity(legacy: &Registry, ecs: &Registry) {
    assert_eq!(legacy.active_entities(), ecs.active_entities(), "Entity count mismatch");

    for i in 0..legacy.active_entities() {
        assert_eq!(legacy.names[i], ecs.names[i], "Name mismatch at entity {}", i);
        assert_eq!(legacy.kinds[i], ecs.kinds[i], "Kind mismatch at entity {}", i);

        // Deep compare TypeComponent if present
        if let (Some(l_ty), Some(e_ty)) = (&legacy.types[i], &ecs.types[i]) {
            assert_eq!(l_ty.0.core, e_ty.0.core, "Type core mismatch at entity {}", i);
            assert_eq!(
                l_ty.0.annotations.linearity, e_ty.0.annotations.linearity,
                "Linearity mismatch at entity {}",
                i
            );
            assert_eq!(
                l_ty.0.annotations.effect, e_ty.0.annotations.effect,
                "Effect mismatch at entity {}",
                i
            );
            assert_eq!(
                l_ty.0.annotations.refinement, e_ty.0.annotations.refinement,
                "Refinement mismatch at entity {}",
                i
            );
            assert_eq!(
                l_ty.0.annotations.clock_domain, e_ty.0.annotations.clock_domain,
                "Clock domain mismatch at entity {}",
                i
            );
            assert_eq!(
                l_ty.0.annotations.phantom_tag, e_ty.0.annotations.phantom_tag,
                "Phantom tag mismatch at entity {}",
                i
            );
        } else {
            assert_eq!(
                legacy.types[i].is_some(),
                ecs.types[i].is_some(),
                "Type presence mismatch at entity {}",
                i
            );
        }

        assert_eq!(legacy.modules[i], ecs.modules[i], "Parent/Module mismatch at entity {}", i);

        // Guard components
        assert_eq!(legacy.conditions[i], ecs.conditions[i], "Condition mismatch at entity {}", i);
        assert_eq!(legacy.cycles[i], ecs.cycles[i], "Cycles mismatch at entity {}", i);

        // Reflex/Assignment components
        assert_eq!(legacy.reflex_comps[i], ecs.reflex_comps[i], "Reflex mismatch at entity {}", i);
        assert_eq!(
            legacy.assignment_comps[i], ecs.assignment_comps[i],
            "Assignment mismatch at entity {}",
            i
        );

        // Expression components
        assert_eq!(legacy.literals[i], ecs.literals[i], "Literal mismatch at entity {}", i);
        assert_eq!(legacy.unary_ops[i], ecs.unary_ops[i], "Unary mismatch at entity {}", i);
        assert_eq!(legacy.binary_ops[i], ecs.binary_ops[i], "Binary mismatch at entity {}", i);
        assert_eq!(legacy.prev_ops[i], ecs.prev_ops[i], "Prev mismatch at entity {}", i);
        assert_eq!(legacy.signal_refs[i], ecs.signal_refs[i], "SignalRef mismatch at entity {}", i);
        assert_eq!(
            legacy.pending_signal_refs[i], ecs.pending_signal_refs[i],
            "PendingSignalRef mismatch at entity {}",
            i
        );
    }

    // Check symbol table parity
    for (name, leg_id) in legacy.get_symbol_table() {
        let ecs_id =
            ecs.get_symbol_table().get(name).expect(&format!("Symbol {} missing in ECS", name));
        assert_eq!(leg_id, ecs_id, "Symbol {} points to different entity", name);
    }
}

#[test]
fn test_parser_parity_empty_module() {
    let source = "module empty {}";

    // Legacy pipeline
    let mut legacy_registry = Registry::new();
    let legacy_ast = parse_mirr(source).expect("Legacy parse failed");
    ingest_program(&mut legacy_registry, legacy_ast, None).expect("Legacy ingest failed");

    // ECS pipeline
    let mut ecs_registry = Registry::new();
    parse_mirr_ecs(&mut ecs_registry, source).expect("ECS parse failed");

    // Verify
    assert_registry_parity(&legacy_registry, &ecs_registry);
}

#[test]
fn test_parser_parity_basic_signals() {
    let source = "
module top {
    signal clk: in bool;
    signal rst_n: in bool;
    signal data_out: out u16;
    signal internal_reg: u32;
}
";

    // Legacy pipeline
    let mut legacy_registry = Registry::new();
    let legacy_ast = parse_mirr(source).expect("Legacy parse failed");
    ingest_program(&mut legacy_registry, legacy_ast, None).expect("Legacy ingest failed");

    // ECS pipeline
    let mut ecs_registry = Registry::new();
    parse_mirr_ecs(&mut ecs_registry, source).expect("ECS parse failed");

    // Verify
    assert_registry_parity(&legacy_registry, &ecs_registry);
}

#[test]
fn test_parser_parity_mega1_signals() {
    let source = "
module top {
    signal s1: in linear bool;
    signal s2: stateful u32 where 0..100;
    signal s3: pure u16 @fast_clk;
    signal s4: out u64 #Important;
}
";

    // Legacy pipeline
    let mut legacy_registry = Registry::new();
    let legacy_ast = parse_mirr(source).expect("Legacy parse failed");
    ingest_program(&mut legacy_registry, legacy_ast, None).expect("Legacy ingest failed");

    // ECS pipeline
    let mut ecs_registry = Registry::new();
    parse_mirr_ecs(&mut ecs_registry, source).expect("ECS parse failed");

    // Verify
    assert_registry_parity(&legacy_registry, &ecs_registry);
}

#[test]
fn test_parser_parity_expressions() {
    let expr_str = "a + b * (123 - 1) == d || !e && prev(f, 1)";

    // Legacy pipeline
    let mut legacy_registry = Registry::new();
    let legacy_ast_expr =
        mirrc::parser::expr_parser::parse_expression(expr_str).expect("Legacy parse failed");
    legacy_registry.ingest_expr(&legacy_ast_expr).expect("Legacy ingest failed");

    // ECS pipeline
    let mut ecs_registry = Registry::new();
    mirrc::parser::ecs_parser::parse_expression_ecs(&mut ecs_registry, expr_str)
        .expect("ECS parse failed");

    // Verify
    assert_registry_parity(&legacy_registry, &ecs_registry);
}

#[test]
fn test_parser_parity_guards() {
    let source = "
module top {
    signal a: in bool;
    guard g1 {
        when a == true;
        for 5 cycles;
    }
}
";

    // Legacy pipeline
    let mut legacy_registry = Registry::new();
    let legacy_ast = parse_mirr(source).expect("Legacy parse failed");
    ingest_program(&mut legacy_registry, legacy_ast, None).expect("Legacy ingest failed");

    // ECS pipeline
    let mut ecs_registry = Registry::new();
    parse_mirr_ecs(&mut ecs_registry, source).expect("ECS parse failed");

    // Verify
    assert_registry_parity(&legacy_registry, &ecs_registry);
}

#[test]
fn test_parser_parity_reflexes() {
    let source = "
module top {
    signal a: in bool;
    signal b: out u32;
    guard g1 { when a == true; for 1 cycles; }
    reflex r1 {
        on g1 {
            b = 42;
        }
    }
    reflex r2 {
        on always {
            b = b + 1;
        }
    }
}
";

    // Legacy pipeline
    let mut legacy_registry = Registry::new();
    let legacy_ast = parse_mirr(source).expect("Legacy parse failed");
    ingest_program(&mut legacy_registry, legacy_ast, None).expect("Legacy ingest failed");

    // ECS pipeline
    let mut ecs_registry = Registry::new();
    parse_mirr_ecs(&mut ecs_registry, source).expect("ECS parse failed");

    // Verify
    assert_registry_parity(&legacy_registry, &ecs_registry);
}
