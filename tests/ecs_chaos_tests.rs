#![forbid(unsafe_code)]

use mirrc::ecs::components::*;
use mirrc::ecs::registry::Registry;

#[test]
fn test_chaos_missing_kind_no_panic() {
    let mut registry = Registry::new();
    let ent1 = registry.next_id();
    registry.names[ent1.0 as usize] = Some(NameComponent("duplicate".to_string()));
    registry.kinds[ent1.0 as usize] = Some(KindComponent(EntityKind::GUARD));

    let ent2 = registry.next_id();
    registry.names[ent2.0 as usize] = Some(NameComponent("duplicate".to_string()));
    // CRITICAL: Missing KindComponent on ent2!

    // This should no longer panic, but return Err with a descriptive message.
    let result = registry.semantic_validate();
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("missing a KindComponent"));
}

#[test]
fn test_chaos_circular_expression_audit() {
    let mut registry = Registry::new();
    let ent_a = registry.next_id();
    let ent_b = EntityId(ent_a.0 + 1);

    // A -> B
    registry.unary_ops[ent_a.0 as usize] =
        Some(UnaryComponent { op: mirrc::ast::types::UnaryOp::Not, operand: ent_b });
    // B -> A
    registry.unary_ops[ent_b.0 as usize] =
        Some(UnaryComponent { op: mirrc::ast::types::UnaryOp::Not, operand: ent_a });

    let g = registry.create_entity("circular_g", KindComponent(EntityKind::GUARD));
    registry.cycles[g.0 as usize] = Some(CyclesComponent(10));
    registry.conditions[g.0 as usize] = Some(ConditionComponent(ent_a));

    // This should terminate quickly and return a depth limit error.
    let result = registry.semantic_validate();
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(err_str.contains("Expression depth limit exceeded"));
}
