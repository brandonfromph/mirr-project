//! Property extraction and formula lowering.

#![forbid(unsafe_code)]

use super::MAX_BRIDGE_PROPERTIES;
use crate::ast::property::{PropertyDirective, PropertyFormula};

use crate::mape_k::error::MapeKError;
use crate::mape_k::ltl::{SignalPredicate, TemporalProperty};
use crate::pipeline::PipelineResult;

/// Maximum expression nodes to visit when extracting a signal name.
const MAX_EXPR_VISIT: usize = 64;

/// Walk the module's property declarations. For each `Assert` property,
/// attempt to lower the formula to a `TemporalProperty`. `Cover` and
/// `Assume` directives are skipped.
/// Walk the module's property declarations. For each `Assert` property,
/// attempt to lower the formula to a `TemporalProperty`. `Cover` and
/// `Assume` directives are skipped.
pub(super) fn extract_properties(
    result: &PipelineResult,
    errors: &mut Vec<MapeKError>,
) -> Vec<TemporalProperty> {
    let registry = result.ecs_registry.as_ref().expect("ECS registry required");

    let mut assert_count = 0;
    for prop_comp in registry.property_comps.iter().flatten() {
        if prop_comp.directive == PropertyDirective::Assert {
            assert_count += 1;
        }
    }

    if assert_count > MAX_BRIDGE_PROPERTIES {
        errors.push(MapeKError::BridgeConfigError(format!(
            "too many properties: {} > {}",
            assert_count, MAX_BRIDGE_PROPERTIES
        )));
        return Vec::new();
    }

    let mut temporal_props = Vec::with_capacity(assert_count);

    for prop in registry.property_comps.iter().flatten() {
        if prop.directive != PropertyDirective::Assert {
            continue;
        }

        match lower_formula_ecs(prop, registry) {
            Ok(tp) => temporal_props.push(tp),
            Err(err) => errors.push(err),
        }

        if temporal_props.len() >= MAX_BRIDGE_PROPERTIES {
            break;
        }
    }

    temporal_props
}

/// Attempt to lower a `PropertyComponent` into a `TemporalProperty` using ECS traversal.
fn lower_formula_ecs(
    prop: &crate::ecs::components::PropertyComponent,
    registry: &crate::ecs::Registry,
) -> Result<TemporalProperty, MapeKError> {
    match &prop.formula {
        PropertyFormula::Always(_) => {
            let pred = lower_ecs_expr_to_predicate(prop.formula_exprs[0], registry)?;
            Ok(TemporalProperty::Always(pred))
        }
        PropertyFormula::Never(_) => {
            let signal = extract_signal_name_ecs(prop.formula_exprs[0], registry)?;
            let pred = SignalPredicate::LessThan(signal, 1);
            Ok(TemporalProperty::Always(pred))
        }
        PropertyFormula::EventuallyWithin { cycles, .. } => {
            let pred = lower_ecs_expr_to_predicate(prop.formula_exprs[0], registry)?;
            Ok(TemporalProperty::EventuallyWithin(pred, u64::from(*cycles)))
        }
        PropertyFormula::AlwaysImplies { .. } => {
            let a = lower_ecs_expr_to_predicate(prop.formula_exprs[0], registry)?;
            let b = lower_ecs_expr_to_predicate(prop.formula_exprs[1], registry)?;
            Ok(TemporalProperty::AlwaysImplies(a, b))
        }
        PropertyFormula::NeverImplies { .. } => {
            let a = lower_ecs_expr_to_predicate(prop.formula_exprs[0], registry)?;
            let b = lower_ecs_expr_to_predicate(prop.formula_exprs[1], registry)?;
            Ok(TemporalProperty::NeverImplies(a, b))
        }
        PropertyFormula::AlwaysFollowedBy { delay_cycles, .. } => {
            let t = lower_ecs_expr_to_predicate(prop.formula_exprs[0], registry)?;
            let r = lower_ecs_expr_to_predicate(prop.formula_exprs[1], registry)?;
            Ok(TemporalProperty::AlwaysFollowedBy(t, u64::from(*delay_cycles), r))
        }
    }
}

use crate::ecs::components;

/// Lower an ECS expression to a `SignalPredicate`.
fn lower_ecs_expr_to_predicate(
    root: crate::ecs::EntityId,
    registry: &crate::ecs::Registry,
) -> Result<SignalPredicate, MapeKError> {
    let i = root.0 as usize;
    if i >= registry.names.len() {
        return Err(MapeKError::LoweringError("invalid entity id".to_string()));
    }

    if let Some(components::SignalRefComponent(sig_ent)) = &registry.signal_refs[i] {
        if let Some(name) = &registry.names[sig_ent.0 as usize] {
            return Ok(SignalPredicate::IsTrue(name.0.clone()));
        }
    }

    if let Some(components::PendingSignalRef(name)) = &registry.pending_signal_refs[i] {
        return Ok(SignalPredicate::IsTrue(name.clone()));
    }

    if let Some(bin) = &registry.binary_ops[i] {
        return lower_binary_predicate_ecs(bin, registry);
    }

    if registry.unary_ops[i].is_some() {
        let name = extract_signal_name_ecs(root, registry)?;
        return Ok(SignalPredicate::IsTrue(name));
    }

    if registry.prev_ops[i].is_some() {
        let name = extract_signal_name_ecs(root, registry)?;
        return Ok(SignalPredicate::IsTrue(name));
    }

    if registry.literals[i].is_some() {
        return Err(MapeKError::LoweringError(
            "bare literal cannot be a signal predicate".to_string(),
        ));
    }

    let name =
        extract_signal_name_ecs(root, registry).unwrap_or_else(|_| "__composite__".to_string());
    Ok(SignalPredicate::IsTrue(name))
}

/// Lower a binary ECS expression to a `SignalPredicate`.
fn lower_binary_predicate_ecs(
    bin: &crate::ecs::components::BinaryComponent,
    registry: &crate::ecs::Registry,
) -> Result<SignalPredicate, MapeKError> {
    use crate::ast::types::BinaryOp;

    let left_idx = bin.left.0 as usize;

    let sig_name = if let Some(components::SignalRefComponent(sig_ent)) =
        &registry.signal_refs[left_idx]
    {
        registry.names[sig_ent.0 as usize].as_ref().map(|n| n.0.clone())
    } else if let Some(components::PendingSignalRef(name)) = &registry.pending_signal_refs[left_idx]
    {
        Some(name.clone())
    } else {
        None
    };

    if let (Some(name), Some(threshold)) = (sig_name, literal_u64_ecs(bin.right, registry)) {
        return match bin.op {
            BinaryOp::Lt => Ok(SignalPredicate::LessThan(name, threshold)),
            BinaryOp::Le => Ok(SignalPredicate::LessThan(name, threshold.saturating_add(1))),
            BinaryOp::Gt => Ok(SignalPredicate::GreaterThan(name, threshold)),
            BinaryOp::Ge => Ok(SignalPredicate::GreaterThan(name, threshold.saturating_sub(1))),
            _ => Ok(SignalPredicate::IsTrue(name)),
        };
    }

    let name = extract_signal_name_ecs(bin.left, registry)
        .or_else(|_| extract_signal_name_ecs(bin.right, registry))?;
    Ok(SignalPredicate::IsTrue(name))
}

/// Extract a `u64` from an ECS literal, if it is one.
fn literal_u64_ecs(ent: crate::ecs::EntityId, registry: &crate::ecs::Registry) -> Option<u64> {
    if let Some(lit) = &registry.literals[ent.0 as usize] {
        match &lit.0 {
            crate::ast::types::LiteralValue::Integer(n) => Some(*n),
            crate::ast::types::LiteralValue::Bool(b) => Some(u64::from(*b)),
        }
    } else {
        None
    }
}

/// Walk an ECS expression tree (bounded, iterative) to find the first `Signal` name.
pub(super) fn extract_signal_name_ecs(
    root: crate::ecs::EntityId,
    registry: &crate::ecs::Registry,
) -> Result<String, MapeKError> {
    let mut stack: Vec<crate::ecs::EntityId> = Vec::with_capacity(MAX_EXPR_VISIT);
    stack.push(root);

    let mut visited: usize = 0;
    while let Some(current) = stack.pop() {
        visited = visited.saturating_add(1);
        if visited > MAX_EXPR_VISIT {
            break;
        }

        let i = current.0 as usize;
        if i >= registry.names.len() {
            continue;
        }

        if let Some(components::SignalRefComponent(sig_ent)) = &registry.signal_refs[i] {
            if let Some(name) = &registry.names[sig_ent.0 as usize] {
                return Ok(name.0.clone());
            }
        }
        if let Some(components::PendingSignalRef(name)) = &registry.pending_signal_refs[i] {
            return Ok(name.clone());
        }
        if let Some(p) = &registry.prev_ops[i] {
            stack.push(p.signal);
            continue;
        }
        if let Some(u) = &registry.unary_ops[i] {
            stack.push(u.operand);
        }
        if let Some(b) = &registry.binary_ops[i] {
            stack.push(b.right);
            stack.push(b.left);
        }
        if let Some(ai) = &registry.array_indices[i] {
            stack.push(ai.array);
        }
        if let Some(fa) = &registry.field_accesses[i] {
            stack.push(fa.object);
        }
        if let Some(al) = &registry.array_literals[i] {
            if let Some(&first) = al.0.first() {
                stack.push(first);
            }
        }
        if let Some(sl) = &registry.struct_literals[i] {
            if let Some(first_field) = sl.fields.first() {
                stack.push(first_field.1);
            }
        }
    }

    Err(MapeKError::LoweringError("no signal reference found in ECS expression".to_string()))
}
