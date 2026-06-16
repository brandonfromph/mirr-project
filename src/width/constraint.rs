//! Width constraint representation and generation.
//!
//! Encodes the relationship between expression node widths as constraints
//! that the solver propagates to fixpoint.

#![forbid(unsafe_code)]

use crate::ast::types::BinaryOp;
use crate::width::types::WidthDiag;
// use serde::Serialize;
use std::borrow::Borrow;
use std::hash::Hash;

// ---------------------------------------------------------------------------
pub fn generate_ecs_constraints<K>(
    registry: &mut crate::ecs::registry::Registry,
    signal_info: &std::collections::HashMap<K, (u32, bool)>,
) -> Vec<WidthDiag>
where
    K: Eq + Hash + Borrow<str>,
{
    use crate::ecs::components::{EntityId, PendingSignalRef, WidthConstraintComponent};
    let mut diagnostics = Vec::new();

    for i in 0..registry.names.len() {
        let _node_id = EntityId(i as u32);

        let constraint = if let Some(lit) = &registry.literals[i] {
            let val = match lit.0 {
                crate::ast::types::LiteralValue::Bool(b) => {
                    if b {
                        1
                    } else {
                        0
                    }
                }
                crate::ast::types::LiteralValue::Integer(v) => v,
            };
            WidthConstraintComponent::Fixed(min_bits_for(val))
        } else if let Some(sig_ref) = &registry.signal_refs[i] {
            // Check if signal has a type component assigned
            if let Some(tc) = &registry.types[sig_ref.0 .0 as usize] {
                WidthConstraintComponent::Fixed(tc.0.core.width())
            } else {
                WidthConstraintComponent::Fixed(1)
            }
        } else if let Some(PendingSignalRef(name)) = &registry.pending_signal_refs[i] {
            if let Some(&(width, _is_signed)) = signal_info.get(name.as_str()) {
                WidthConstraintComponent::Fixed(width)
            } else {
                diagnostics.push(
                    WidthDiag::error(format!(
                        "{} undeclared signal reference: '{}'",
                        crate::error_codes::ec(501),
                        name
                    ))
                    .with_code("E501")
                    .with_signal(name),
                );
                WidthConstraintComponent::Fixed(1)
            }
        } else if let Some(un) = &registry.unary_ops[i] {
            match un.op {
                crate::ast::types::UnaryOp::Not => {
                    WidthConstraintComponent::SameAs { source: un.operand }
                }
                crate::ast::types::UnaryOp::ReductionOr => WidthConstraintComponent::Fixed(1),
                crate::ast::types::UnaryOp::Negate => {
                    // ECS migration signedness check fallback
                    WidthConstraintComponent::SameAsPlusOne { source: un.operand }
                }
            }
        } else if let Some(bin) = &registry.binary_ops[i] {
            if bin.op == BinaryOp::Sub {
                let left_is_unsigned = is_unsigned_entity(registry, bin.left, signal_info);
                let right_is_unsigned = is_unsigned_entity(registry, bin.right, signal_info);
                if left_is_unsigned && right_is_unsigned {
                    let left_val = literal_val_entity(registry, bin.left);
                    let right_val = literal_val_entity(registry, bin.right);
                    let should_emit = match (left_val, right_val) {
                        (Some(l), Some(r)) => l < r,
                        _ => true,
                    };
                    if should_emit {
                        diagnostics.push(WidthDiag::info(
                            "unsigned subtraction may underflow (wrapping semantics)",
                        ));
                    }
                }
            }

            match bin.op {
                BinaryOp::Add => {
                    WidthConstraintComponent::MaxPlusOne { left: bin.left, right: bin.right }
                }
                BinaryOp::Mul => {
                    WidthConstraintComponent::SumOf { left: bin.left, right: bin.right }
                }
                BinaryOp::Sub
                | BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::BitwiseOr
                | BinaryOp::Xor => {
                    WidthConstraintComponent::MaxOf { left: bin.left, right: bin.right }
                }
                BinaryOp::BitwiseAnd => {
                    if let Some(l_lit) = registry.literals[bin.left.0 as usize].as_ref() {
                        let l_val = match l_lit.0 {
                            crate::ast::types::LiteralValue::Bool(b) => {
                                if b {
                                    1
                                } else {
                                    0
                                }
                            }
                            crate::ast::types::LiteralValue::Integer(v) => v,
                        };
                        WidthConstraintComponent::Narrowed {
                            source: bin.right,
                            narrow_width: min_bits_for(l_val),
                        }
                    } else if let Some(r_lit) = registry.literals[bin.right.0 as usize].as_ref() {
                        let r_val = match r_lit.0 {
                            crate::ast::types::LiteralValue::Bool(b) => {
                                if b {
                                    1
                                } else {
                                    0
                                }
                            }
                            crate::ast::types::LiteralValue::Integer(v) => v,
                        };
                        WidthConstraintComponent::Narrowed {
                            source: bin.left,
                            narrow_width: min_bits_for(r_val),
                        }
                    } else {
                        WidthConstraintComponent::MaxOf { left: bin.left, right: bin.right }
                    }
                }
                BinaryOp::Shl => {
                    if let Some(r_lit) = registry.literals[bin.right.0 as usize].as_ref() {
                        let r_val = match r_lit.0 {
                            crate::ast::types::LiteralValue::Bool(b) => {
                                if b {
                                    1
                                } else {
                                    0
                                }
                            }
                            crate::ast::types::LiteralValue::Integer(v) => v,
                        };
                        WidthConstraintComponent::LeftPlusConst {
                            left: bin.left,
                            shift_amount: r_val as u32,
                        }
                    } else {
                        WidthConstraintComponent::LeftPlusMaxShift { left: bin.left }
                    }
                }
                BinaryOp::Shr => {
                    if let Some(r_lit) = registry.literals[bin.right.0 as usize].as_ref() {
                        let r_val = match r_lit.0 {
                            crate::ast::types::LiteralValue::Bool(b) => {
                                if b {
                                    1
                                } else {
                                    0
                                }
                            }
                            crate::ast::types::LiteralValue::Integer(v) => v,
                        };
                        WidthConstraintComponent::LeftMinusConst {
                            left: bin.left,
                            shift_amount: r_val as u32,
                        }
                    } else {
                        WidthConstraintComponent::SameAs { source: bin.left }
                    }
                }
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge => WidthConstraintComponent::Boolean,
            }
        } else if let Some(prev) = &registry.prev_ops[i] {
            if let Some(tc) = &registry.types[prev.signal.0 as usize] {
                WidthConstraintComponent::Fixed(tc.0.core.width())
            } else {
                WidthConstraintComponent::Fixed(1)
            }
        } else if let Some(_arr_idx) = &registry.array_indices[i] {
            WidthConstraintComponent::Fixed(32) // Fallback for now
        } else if let Some(_facc) = &registry.field_accesses[i] {
            WidthConstraintComponent::Fixed(32) // Fallback for now
        } else if let Some(arr_lit) = &registry.array_literals[i] {
            WidthConstraintComponent::SumAll { elements: arr_lit.0.clone() }
        } else if let Some(str_lit) = &registry.struct_literals[i] {
            WidthConstraintComponent::SumAll {
                elements: str_lit.fields.iter().map(|f| f.1).collect(),
            }
        } else {
            // Skip non-expression entities
            continue;
        };

        registry.width_constraints[i] = Some(constraint);
    }
    diagnostics
}

/// Calculate minimum bits required to represent an unsigned integer.
fn min_bits_for(v: u64) -> u32 {
    if v == 0 {
        1
    } else {
        64 - v.leading_zeros()
    }
}

/// Returns true if the flattened node is unsigned (either literal or unsigned signal/prev).
// fn is_unsigned_node(node: &FlatNode) -> bool {
//     match node {
//         FlatNode::Literal { .. } => true,
//         FlatNode::Signal { signed, .. } => !*signed,
//         FlatNode::Prev { signed, .. } => !*signed,
//         FlatNode::ArrayIndex { signed, .. } => !*signed,
//         FlatNode::FieldAccess { signed, .. } => !*signed,
//         FlatNode::ArrayLiteral { .. } => true,
//         FlatNode::StructLiteral { .. } => true,
//         _ => false,
//     }
// }
//
// /// Get literal value if this node is a literal constant.
// fn literal_value(node: &FlatNode) -> Option<u64> {
//     match node {
//         FlatNode::Literal { value } => Some(*value),
//         _ => None,
//     }
// }
//
/// Returns true if the ECS entity is unsigned.
fn is_unsigned_entity<K>(
    registry: &crate::ecs::registry::Registry,
    entity: crate::ecs::components::EntityId,
    signal_info: &std::collections::HashMap<K, (u32, bool)>,
) -> bool
where
    K: Eq + Hash + Borrow<str>,
{
    let id = entity.0 as usize;
    if registry.literals[id].is_some() {
        return true;
    }
    if registry.array_literals[id].is_some() || registry.struct_literals[id].is_some() {
        return true;
    }
    if let Some(tc) = &registry.types[id] {
        return !tc.0.core.width_and_signed().1;
    }
    if let Some(sig_ref) = &registry.signal_refs[id] {
        if let Some(tc) = &registry.types[sig_ref.0 .0 as usize] {
            return !tc.0.core.width_and_signed().1;
        }
    }
    if let Some(crate::ecs::components::PendingSignalRef(name)) = &registry.pending_signal_refs[id]
    {
        if let Some(&(_w, is_signed)) = signal_info.get(name.as_str()) {
            return !is_signed;
        }
    }
    if let Some(prev) = &registry.prev_ops[id] {
        if let Some(tc) = &registry.types[prev.signal.0 as usize] {
            return !tc.0.core.width_and_signed().1;
        }
    }
    false
}

/// Get literal value if the ECS entity is a literal constant.
fn literal_val_entity(
    registry: &crate::ecs::registry::Registry,
    entity: crate::ecs::components::EntityId,
) -> Option<u64> {
    if let Some(lit) = &registry.literals[entity.0 as usize] {
        match lit.0 {
            crate::ast::types::LiteralValue::Integer(v) => Some(v),
            crate::ast::types::LiteralValue::Bool(b) => Some(if b { 1 } else { 0 }),
        }
    } else {
        None
    }
}
