#![forbid(unsafe_code)]

use nasa_rust_project::ast::program::Guard;
use nasa_rust_project::ast::{BinaryOp, Expr};
use nasa_rust_project::ecs::components::KindComponent;
use nasa_rust_project::ecs::registry::Registry;
use nasa_rust_project::temporal::compiler::TemporalCompiler;

#[test]
fn test_temporal_lowering_parity_complex_guard() {
    let mut registry = Registry::new();

    // Define a complex guard: (sig_a && sig_b) for 10 cycles
    let _sig_a = registry.create_entity(
        "sig_a",
        KindComponent(nasa_rust_project::ecs::components::EntityKind::SIGNAL(
            nasa_rust_project::ast::types::SignalKind::Input,
        )),
    );
    let _sig_b = registry.create_entity(
        "sig_b",
        KindComponent(nasa_rust_project::ecs::components::EntityKind::SIGNAL(
            nasa_rust_project::ast::types::SignalKind::Input,
        )),
    );

    let expr_a = registry.ingest_expr(&Expr::Signal("sig_a".to_string())).unwrap();
    let expr_b = registry.ingest_expr(&Expr::Signal("sig_b".to_string())).unwrap();

    let cond_ent = registry.next_id();
    registry.binary_ops[cond_ent.0 as usize] =
        Some(nasa_rust_project::ecs::components::BinaryComponent {
            op: BinaryOp::And,
            left: expr_a,
            right: expr_b,
        });

    let guard_ent = registry.next_id();
    registry.names[guard_ent.0 as usize] =
        Some(nasa_rust_project::ecs::components::NameComponent("complex_g".to_string()));
    registry.cycles[guard_ent.0 as usize] =
        Some(nasa_rust_project::ecs::components::CyclesComponent(10));
    registry.conditions[guard_ent.0 as usize] =
        Some(nasa_rust_project::ecs::components::ConditionComponent(cond_ent));

    // Path A: Legacy AST Path (Reify -> Compile)
    let ast_guard = Guard {
        name: "complex_g".to_string(),
        condition: Expr::Binary {
            left: Box::new(Expr::Signal("sig_a".to_string())),
            op: BinaryOp::And,
            right: Box::new(Expr::Signal("sig_b".to_string())),
        },
        cycles: 10,
        origin: None,
        span: None,
    };

    let mut compiler_legacy = TemporalCompiler::new();
    let netlist_legacy = compiler_legacy.compile_module(&[ast_guard]).unwrap();

    // Path B: Modern ECS Path (Direct Synthesis)
    let mut compiler_ecs = TemporalCompiler::new();
    let compiled_ecs = compiler_ecs.lower_guard_to_ecs(&registry, guard_ent).unwrap();

    // Verification: Compare outputs
    let legacy_top = netlist_legacy.guards.first().unwrap();

    // Check names and basic properties
    assert_eq!(legacy_top.name(), compiled_ecs.name());

    // Deep Check: Both should be ComplexGuards with identical structures
    match (legacy_top, &compiled_ecs) {
        (
            nasa_rust_project::temporal::low_level_ir::CompiledGuard::Complex(l),
            nasa_rust_project::temporal::low_level_ir::CompiledGuard::Complex(r),
        ) => {
            assert_eq!(l.sub_guards.len(), r.sub_guards.len());
            // Since sub-guards are generated with unique names based on a counter,
            // and we used fresh compilers for both, they should match exactly.
            for (ls, rs) in l.sub_guards.iter().zip(r.sub_guards.iter()) {
                assert_eq!(ls.name(), rs.name());
                assert_eq!(ls.output_signal(), rs.output_signal());
            }
        }
        _ => panic!("Expected both paths to produce ComplexGuard for compound condition"),
    }
}
