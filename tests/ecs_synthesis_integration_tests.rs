//! Contract: ECS-Native Temporal Synthesis Integration
//! Validates the end-to-end synthesis pipeline:
//! AST -> ECS Registry -> SPU Resource Allocation -> Temporal IR (CompiledGuard)

use mirrc::ast::program::SignalDecl;
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::ecs::{adapter::register_signal_to_ecs, components::*, Registry};
use mirrc::temporal::compiler::TemporalCompiler;
use mirrc::temporal::low_level_ir::CompiledGuard;

#[test]
fn test_synthesis_pipeline() {
    let mut registry = Registry::new();
    let mod_entity = registry.create_entity("test_module", KindComponent::MODULE);
    let mut compiler = TemporalCompiler::new();

    // 1. Register Signal Entity
    let sig = SignalDecl {
        name: "sensor".to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(SignalType::Bool),
        origin: None,
        span: None,
    };
    let _sig_ent = register_signal_to_ecs(&mut registry, mod_entity, sig);

    // 2. Register Guard Entity with expression hydration
    let cond_expr_id = registry
        .ingest_expr(&mirrc::ast::Expr::Signal("sensor".to_string()))
        .expect("failed to ingest");

    let guard_entity = registry.create_entity("guard1", KindComponent::GUARD);
    registry.conditions[guard_entity.0 as usize] = Some(ConditionComponent(cond_expr_id));
    registry.cycles[guard_entity.0 as usize] = Some(CyclesComponent(8));

    // 3. Physical Synthesis (Core logic)
    let guard = compiler.lower_guard_to_ecs(&registry, guard_entity).expect("Synthesis failed");

    match guard {
        CompiledGuard::ShiftRegister(sr) => {
            assert_eq!(sr.input_signal, "sensor");
            assert_eq!(sr.delay_cycles, 8);
        }
        _ => panic!("Expected shift register guard"),
    }
}
