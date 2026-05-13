#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, LiteralValue};
use crate::ecs::components::*;
use crate::ecs::registry::Registry;
use rayon::prelude::*;

/// ECS System: Parallel Constant Folding.
///
/// Sweeps across all expression entities in parallel using all available CPU cores.
/// Adheres to NASA P10 Rule #1 (Simple control flow).
pub fn parallel_constant_folding_system(registry: &mut Registry) {
    // 1. Identify candidate entities (Those with binary ops)
    let next_id = registry.next_id as usize;

    // 2. Compute reductions in parallel
    // We use rayon to split the workload across the thread pool.
    let reductions: Vec<(EntityId, LiteralValue)> = (0..next_id)
        .into_par_iter()
        .filter_map(|idx| {
            // Check if this entity has a binary operation
            let binary = registry.binary_ops[idx].as_ref()?;

            // Check if operands are literals
            let left_lit = registry.literals[binary.left.0 as usize].as_ref()?;
            let right_lit = registry.literals[binary.right.0 as usize].as_ref()?;

            // Attempt to fold
            fold_binary(binary.op, &left_lit.0, &right_lit.0).map(|val| (EntityId(idx as u32), val))
        })
        .collect();

    // 3. Apply reductions (Atomic/Serial commit)
    for (id, value) in reductions {
        let idx = id.0 as usize;
        registry.binary_ops[idx] = None;
        registry.literals[idx] = Some(LiteralComponent(value));
    }
}

fn fold_binary(op: BinaryOp, left: &LiteralValue, right: &LiteralValue) -> Option<LiteralValue> {
    match (op, left, right) {
        (BinaryOp::And, LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
            Some(LiteralValue::Bool(*l && *r))
        }
        (BinaryOp::Or, LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
            Some(LiteralValue::Bool(*l || *r))
        }
        (BinaryOp::Add, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Integer(l.wrapping_add(*r)))
        }
        (BinaryOp::Eq, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Bool(l == r))
        }
        _ => None,
    }
}
