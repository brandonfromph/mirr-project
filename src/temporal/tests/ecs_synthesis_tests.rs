//! TDD Suite: ECS-Native Guard Synthesis.
//! Ensures that ECS Registry Guard Entities lower correctly to Temporal IR.

use nasa_rust_project::ast::program::SignalDecl;
use nasa_rust_project::ast::types::{ExtendedType, SignalKind, SignalType};
use nasa_rust_project::ecs::{adapter::register_signal_to_ecs, components::*, Registry};
use nasa_rust_project::temporal::low_level_ir::{CompiledGuard, ConditionKind};

#[test]
fn test_lower_ecs_guard_to_shift_register() {
    // 1. Setup minimal registry
    let mut registry = Registry::new(1024);
    let mod_entity = registry.create_entity("test_module", KindComponent::MODULE);

    // 2. Register Input Signal
    let input_sig = SignalDecl {
        name: "lidar_in".to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(SignalType::Bool),
        origin: None,
        span: None,
    };
    let input_entity = register_signal_to_ecs(&mut registry, mod_entity, input_sig);

    // 3. Register Guard Entity
    let guard_entity = registry.create_entity("lidar_close", KindComponent::GUARD);
    registry.set_component(
        guard_entity,
        ConditionComponent(ConditionKind::SimpleSignal("lidar_in".to_string())),
    );
    registry.set_component(guard_entity, DelayComponent(4)); // 4 cycles

    // 4. Run Synthesis
    let compiled = lower_guard_to_ecs(&registry, guard_entity).expect("lowering failed");

    match compiled {
        CompiledGuard::ShiftRegister(sr) => {
            assert_eq!(sr.delay_cycles, 4);
            assert_eq!(sr.input_signal, "lidar_in");
        }
        _ => panic!("Expected ShiftRegister"),
    }
}

// Function signature stub to drive development
use nasa_rust_project::ecs::EntityId;
fn lower_guard_to_ecs(_reg: &Registry, _guard: EntityId) -> Result<CompiledGuard, nasa_rust_project::error::MirrError> {
    Err(nasa_rust_project::error::MirrError::InternalError("Not implemented".to_string()))
}
