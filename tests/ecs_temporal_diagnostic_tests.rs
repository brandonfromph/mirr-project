#![forbid(unsafe_code)]

use nasa_rust_project::ast::types::BinaryOp;
use nasa_rust_project::ecs::components::*;
use nasa_rust_project::ecs::registry::Registry;
use nasa_rust_project::temporal::compiler::TemporalCompiler;

#[test]
fn test_diagnostic_e301_nesting_depth() {
    let mut registry = Registry::new();
    registry.create_entity("sig_a", KindComponent::SIGNAL);
    let expr_a =
        registry.ingest_expr(&nasa_rust_project::ast::Expr::Signal("sig_a".to_string())).unwrap();

    let mut current = expr_a;
    for _ in 0..70 {
        let p = registry.next_id();
        registry.binary_ops[p.0 as usize] =
            Some(BinaryComponent { op: BinaryOp::And, left: expr_a, right: current });
        current = p;
    }

    let g = registry.create_entity("deep_g", KindComponent::GUARD);
    registry.cycles[g.0 as usize] = Some(CyclesComponent(10));
    registry.conditions[g.0 as usize] = Some(ConditionComponent(current));

    let mut compiler = TemporalCompiler::new();
    let err = compiler.lower_guard_to_ecs(&registry, g).unwrap_err();
    assert!(
        err.to_string().contains("exceeds maximum nesting depth"),
        "Expected E301 depth error, got: {}",
        err
    );
}

#[test]
fn test_diagnostic_e302_unsupported_condition() {
    let mut registry = Registry::new();
    let ent = registry.next_id();
    // Create an entity that exists but has no supported components (e.g. just a name)
    registry.names[ent.0 as usize] = Some(NameComponent("unsupported".to_string()));

    let g = registry.create_entity("fail_g", KindComponent::GUARD);
    registry.cycles[g.0 as usize] = Some(CyclesComponent(10));
    registry.conditions[g.0 as usize] = Some(ConditionComponent(ent));

    let mut compiler = TemporalCompiler::new();
    let err = compiler.lower_guard_to_ecs(&registry, g).unwrap_err();
    assert!(
        err.to_string().contains("Entity is not a valid hardware condition"),
        "Expected E302/E306 form error, got: {}",
        err
    );
}

#[test]
fn test_diagnostic_missing_name_robustness() {
    let mut registry = Registry::new();
    let g = registry.next_id();
    // Entity exists but has no components at all

    let mut compiler = TemporalCompiler::new();
    let err = compiler.lower_guard_to_ecs(&registry, g).unwrap_err();
    assert!(err.to_string().contains("missing NameComponent"), "Expected name error, got: {}", err);
}

#[test]
fn test_diagnostic_prev_on_literal_rejection() {
    let mut registry = Registry::new();
    let lit_ent = registry.next_id();
    registry.literals[lit_ent.0 as usize] =
        Some(LiteralComponent(nasa_rust_project::ast::types::LiteralValue::Bool(true)));

    let prev_ent = registry.next_id();
    registry.prev_ops[prev_ent.0 as usize] = Some(PrevComponent { signal: lit_ent, delay: 5 });

    let g = registry.create_entity("prev_lit_g", KindComponent::GUARD);
    registry.cycles[g.0 as usize] = Some(CyclesComponent(10));
    registry.conditions[g.0 as usize] = Some(ConditionComponent(prev_ent));

    let mut compiler = TemporalCompiler::new();
    let err = compiler.lower_guard_to_ecs(&registry, g).unwrap_err();
    // try_from_ecs fails because the target of prev (a literal) has no NameComponent
    assert!(
        err.to_string().contains("Prev reference to unnamed entity"),
        "Expected naming error for prev on unnamed literal, got: {}",
        err
    );
}
