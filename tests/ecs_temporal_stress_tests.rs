#![forbid(unsafe_code)]

use mirrc::ast::types::BinaryOp;
use mirrc::ecs::components::*;
use mirrc::ecs::registry::Registry;
use mirrc::temporal::compiler::TemporalCompiler;
use mirrc::temporal::low_level_ir::CompiledGuard;

#[test]
fn test_stress_nesting_depth_limit() {
    let mut registry = Registry::new();

    // Create a deeply nested AND tree: A && (A && (A && ...))
    let _sig_a = registry.create_entity("sig_a", KindComponent::SIGNAL);
    let expr_a =
        registry.ingest_expr(&mirrc::ast::Expr::Signal("sig_a".to_string())).unwrap();

    let mut current_expr = expr_a;
    // Nest 70 times (exceeds 64 limit)
    for i in 0..70 {
        let parent = registry.next_id();
        registry.binary_ops[parent.0 as usize] =
            Some(BinaryComponent { op: BinaryOp::And, left: expr_a, right: current_expr });
        registry.names[parent.0 as usize] = Some(NameComponent(format!("nest_{}", i)));
        current_expr = parent;
    }

    let guard_ent = registry.create_entity("deep_guard", KindComponent::GUARD);
    registry.cycles[guard_ent.0 as usize] = Some(CyclesComponent(10));
    registry.conditions[guard_ent.0 as usize] = Some(ConditionComponent(current_expr));

    let mut compiler = TemporalCompiler::new();
    let result = compiler.lower_guard_to_ecs(&registry, guard_ent);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exceeds maximum nesting depth"));
}

#[test]
fn test_stress_adaptive_strategy_selection() {
    let mut registry = Registry::new();
    let sig_a = registry.create_entity("sig_a", KindComponent::SIGNAL);
    let expr_a =
        registry.ingest_expr(&mirrc::ast::Expr::Signal("sig_a".to_string())).unwrap();

    let mut compiler = TemporalCompiler::new();

    // 1. Threshold Boundary: 16 cycles (Shift Register)
    let g16 = registry.create_entity("g16", KindComponent::GUARD);
    registry.cycles[g16.0 as usize] = Some(CyclesComponent(16));
    registry.conditions[g16.0 as usize] = Some(ConditionComponent(expr_a));

    let res16 = compiler.lower_guard_to_ecs(&registry, g16).unwrap();
    assert!(matches!(res16, CompiledGuard::ShiftRegister(_)));

    // 2. Threshold Boundary: 17 cycles (Counter)
    let g17 = registry.create_entity("g17", KindComponent::GUARD);
    registry.cycles[g17.0 as usize] = Some(CyclesComponent(17));
    registry.conditions[g17.0 as usize] = Some(ConditionComponent(expr_a));

    let res17 = compiler.lower_guard_to_ecs(&registry, g17).unwrap();
    assert!(matches!(res17, CompiledGuard::Counter(_)));

    // 3. Composite delay: prev(5) + 12 cycles = 17 (Counter)
    let prev_ent = registry.next_id();
    registry.prev_ops[prev_ent.0 as usize] = Some(PrevComponent { signal: sig_a, delay: 5 });
    let g_prev = registry.create_entity("g_prev", KindComponent::GUARD);
    registry.cycles[g_prev.0 as usize] = Some(CyclesComponent(12));
    registry.conditions[g_prev.0 as usize] = Some(ConditionComponent(prev_ent));

    let res_prev = compiler.lower_guard_to_ecs(&registry, g_prev).unwrap();
    assert!(matches!(res_prev, CompiledGuard::Counter(_)));
}

#[test]
fn test_stress_registry_robustness_missing_components() {
    let mut registry = Registry::new();
    let ent = registry.next_id();
    registry.names[ent.0 as usize] = Some(NameComponent("broken".to_string()));
    // Missing Cycles and Condition

    let mut compiler = TemporalCompiler::new();
    let result = compiler.lower_guard_to_ecs(&registry, ent);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("missing CyclesComponent"));

    registry.cycles[ent.0 as usize] = Some(CyclesComponent(10));
    // Missing Condition
    let result2 = compiler.lower_guard_to_ecs(&registry, ent);
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("missing ConditionComponent"));
}

#[test]
fn test_stress_large_breadth_synthesis() {
    let mut registry = Registry::new();
    let _sig_a = registry.create_entity("sig_a", KindComponent::SIGNAL);
    let expr_a =
        registry.ingest_expr(&mirrc::ast::Expr::Signal("sig_a".to_string())).unwrap();

    let mut compiler = TemporalCompiler::new();

    // Synthesize 500 independent guards
    for i in 0..500 {
        let g = registry.create_entity(&format!("g_{}", i), KindComponent::GUARD);
        registry.cycles[g.0 as usize] = Some(CyclesComponent(10));
        registry.conditions[g.0 as usize] = Some(ConditionComponent(expr_a));

        let res = compiler.lower_guard_to_ecs(&registry, g);
        assert!(res.is_ok());
    }
}

#[test]
fn test_stress_circular_reference_prevention() {
    let mut registry = Registry::new();

    let ent_a = registry.next_id();
    let ent_b = EntityId(ent_a.0 + 1);

    // A = B && B
    registry.binary_ops[ent_a.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::And, left: ent_b, right: ent_b });
    // B = A && A
    registry.binary_ops[ent_b.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::And, left: ent_a, right: ent_a });

    let guard = registry.create_entity("circular", KindComponent::GUARD);
    registry.cycles[guard.0 as usize] = Some(CyclesComponent(10));
    registry.conditions[guard.0 as usize] = Some(ConditionComponent(ent_a));

    let mut compiler = TemporalCompiler::new();
    let result = compiler.lower_guard_to_ecs(&registry, guard);

    // Should hit depth limit and return Err, not crash
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("exceeds maximum nesting depth"));
}
