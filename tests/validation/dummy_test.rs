#[test]
fn scratch_test() {
    use nasa_rust_project::ecs::registry::Registry;
    use nasa_rust_project::ecs::components::*;
    use nasa_rust_project::ecs::systems::temporal_synthesis_system;
    use nasa_rust_project::ast::types::*;
    let mut reg = Registry::new();
    let cond_id = reg.create_entity("cond_1", KindComponent::SIGNAL);
    reg.signals[cond_id.0 as usize] = Some(SignalComponent { kind: SignalKind::In, typ: Type::Bool, width: 1, signed: false });
    
    let guard_id = reg.create_entity("guard_1", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(4));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(cond_id));
    
    let res = temporal_synthesis_system(&mut reg);
    println!("Error: {:?}", res);
}