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
// Constraint types
// ---------------------------------------------------------------------------

// #[allow(dead_code, deprecated)]
// #[deprecated]
// /// A width constraint relating one or more flat nodes.
// #[derive(Debug, Clone, Serialize)]
// pub enum WidthConstraint {
//     /// Node must be exactly `width` bits (literal or declared signal).
//     Fixed { node: u32, width: u32 },
//
//     /// Node width = max(left, right) + 1  (for Add).
//     MaxPlusOne { node: u32, left: u32, right: u32 },
//
//     /// Node width = max(left, right)  (for Sub, And, Or, Xor).
//     MaxOf { node: u32, left: u32, right: u32 },
//
//     /// Node width = left + right  (for Mul).
//     SumOf { node: u32, left: u32, right: u32 },
//
//     /// Node width = left_width + shift_const  (for Shl with constant shift).
//     LeftPlusConst { node: u32, left: u32, shift_amount: u32 },
//
//     /// Node width = left_width + 63  (for Shl with variable shift — worst case).
//     LeftPlusMaxShift { node: u32, left: u32 },
//
//     /// Node width = max(1, left_width - shift_const)  (for Shr with constant shift).
//     /// Right shift by a known amount narrows the result; minimum 1 bit guaranteed.
//     LeftMinusConst { node: u32, left: u32, shift_amount: u32 },
//
//     /// Node width = left_width  (for Shr with variable shift, Unary Not).
//     SameAs { node: u32, source: u32 },
//
//     /// Node width = source_width + 1  (for unsigned-to-signed negate).
//     SameAsPlusOne { node: u32, source: u32 },
//
//     /// Node width = sw.min(narrow_width) (for BitwiseAnd with literal).
//     Narrowed { node: u32, source: u32, narrow_width: u32 },
//
//     /// Node width = 1  (for comparison operators and boolean literals).
//     Boolean { node: u32 },
//
//     /// Node width = sum of all element widths (for array/struct literals).
//     SumAll { node: u32, elements: Vec<u32> },
// }
//
// // ---------------------------------------------------------------------------
// // Constraint generation
// // ---------------------------------------------------------------------------
//
// /// Result of constraint generation.
// pub struct ConstraintSet {
//     /// Generated width constraints for the solver.
//     pub constraints: Vec<WidthConstraint>,
//     /// Diagnostics emitted during constraint generation (e.g. undeclared signals).
//     pub diagnostics: Vec<WidthDiag>,
// }
//
// /// Generate width constraints for a flattened expression tree.
// pub fn generate_constraints(
//     nodes: &[FlatNode],
//     signals: &std::collections::HashMap<String, u32>,
// ) -> ConstraintSet {
//     generate_constraints_with_index(nodes, signals)
// }
//
// pub(crate) fn generate_constraints_with_index<K>(
//     nodes: &[FlatNode],
//     signals: &std::collections::HashMap<K, u32>,
// ) -> ConstraintSet
// where
//     K: Eq + Hash + Borrow<str>,
// {
//     let mut constraints = Vec::new();
//     let mut diagnostics = Vec::new();
//
//     for (i, node) in nodes.iter().enumerate() {
//         let node_id = i as u32;
//         match node {
//             FlatNode::Literal { value } => {
//                 let width = min_bits_for(*value);
//                 constraints.push(WidthConstraint::Fixed { node: node_id, width });
//             }
//             FlatNode::Signal { name, .. } => {
//                 if let Some(&width) = signals.get(name) {
//                     constraints.push(WidthConstraint::Fixed { node: node_id, width });
//                 } else {
//                     diagnostics.push(
//                         WidthDiag::error(format!(
//                             "{} undeclared signal reference: '{}'",
//                             crate::error_codes::ec(501),
//                             name
//                         ))
//                         .with_code("E501")
//                         .with_signal(name),
//                     );
//                     // Fallback to width 1 to allow solver to continue.
//                     constraints.push(WidthConstraint::Fixed { node: node_id, width: 1 });
//                 }
//             }
//             FlatNode::Unary { op, operand } => {
//                 match op {
//                     crate::ast::types::UnaryOp::Not => {
//                         constraints
//                             .push(WidthConstraint::SameAs { node: node_id, source: *operand });
//                     }
//                     crate::ast::types::UnaryOp::ReductionOr => {
//                         constraints.push(WidthConstraint::Fixed { node: node_id, width: 1 });
//                     }
//                     crate::ast::types::UnaryOp::Negate => {
//                         // Check signedness of operand to decide SameAs vs SameAsPlusOne
//                         let is_signed = match nodes.get(*operand as usize) {
//                             Some(FlatNode::Signal { signed, .. }) => *signed,
//                             Some(FlatNode::Prev { signed, .. }) => *signed,
//                             _ => false,
//                         };
//                         if is_signed {
//                             constraints
//                                 .push(WidthConstraint::SameAs { node: node_id, source: *operand });
//                         } else {
//                             constraints.push(WidthConstraint::SameAsPlusOne {
//                                 node: node_id,
//                                 source: *operand,
//                             });
//                         }
//                     }
//                 }
//             }
//             FlatNode::Binary { op, left, right } => {
//                 if *op == BinaryOp::Sub {
//                     // Unsigned subtraction may underflow due modular/wrapping semantics.
//                     // Emit info diagnostics when both operands are unsigned or literals.
//                     let left_node = nodes.get(*left as usize);
//                     let right_node = nodes.get(*right as usize);
//                     if let (Some(ln), Some(rn)) = (left_node, right_node) {
//                         if is_unsigned_node(ln) && is_unsigned_node(rn) {
//                             let left_val = literal_value(ln);
//                             let right_val = literal_value(rn);
//                             let should_emit = match (left_val, right_val) {
//                                 (Some(l), Some(r)) => l < r,
//                                 _ => true,
//                             };
//                             if should_emit {
//                                 diagnostics.push(WidthDiag::info(
//                                     "unsigned subtraction may underflow (wrapping semantics)",
//                                 ));
//                             }
//                         }
//                     }
//                 }
//
//                 match op {
//                     BinaryOp::Add => {
//                         constraints.push(WidthConstraint::MaxPlusOne {
//                             node: node_id,
//                             left: *left,
//                             right: *right,
//                         });
//                     }
//                     BinaryOp::Mul => {
//                         constraints.push(WidthConstraint::SumOf {
//                             node: node_id,
//                             left: *left,
//                             right: *right,
//                         });
//                     }
//                     BinaryOp::BitwiseAnd => {
//                         // Intelligent width narrowing: if either side is a literal,
//                         // we constrain the result to the minimum of the source and the literal.
//                         let left_node = nodes.get(*left as usize);
//                         let right_node = nodes.get(*right as usize);
//
//                         if let Some(FlatNode::Literal { value }) = left_node {
//                             constraints.push(WidthConstraint::Narrowed {
//                                 node: node_id,
//                                 source: *right,
//                                 narrow_width: min_bits_for(*value),
//                             });
//                         } else if let Some(FlatNode::Literal { value }) = right_node {
//                             constraints.push(WidthConstraint::Narrowed {
//                                 node: node_id,
//                                 source: *left,
//                                 narrow_width: min_bits_for(*value),
//                             });
//                         } else {
//                             constraints.push(WidthConstraint::MaxOf {
//                                 node: node_id,
//                                 left: *left,
//                                 right: *right,
//                             });
//                         }
//                     }
//                     BinaryOp::Sub
//                     | BinaryOp::And
//                     | BinaryOp::Or
//                     | BinaryOp::BitwiseOr
//                     | BinaryOp::Xor => {
//                         constraints.push(WidthConstraint::MaxOf {
//                             node: node_id,
//                             left: *left,
//                             right: *right,
//                         });
//                     }
//                     BinaryOp::Shl => {
//                         // Check if right is a literal for precision
//                         if let Some(FlatNode::Literal { value }) = nodes.get(*right as usize) {
//                             constraints.push(WidthConstraint::LeftPlusConst {
//                                 node: node_id,
//                                 left: *left,
//                                 shift_amount: *value as u32,
//                             });
//                         } else {
//                             constraints.push(WidthConstraint::LeftPlusMaxShift {
//                                 node: node_id,
//                                 left: *left,
//                             });
//                         }
//                     }
//                     BinaryOp::Shr => {
//                         if let Some(FlatNode::Literal { value }) = nodes.get(*right as usize) {
//                             constraints.push(WidthConstraint::LeftMinusConst {
//                                 node: node_id,
//                                 left: *left,
//                                 shift_amount: *value as u32,
//                             });
//                         } else {
//                             constraints
//                                 .push(WidthConstraint::SameAs { node: node_id, source: *left });
//                         }
//                     }
//                     BinaryOp::Eq
//                     | BinaryOp::Ne
//                     | BinaryOp::Lt
//                     | BinaryOp::Le
//                     | BinaryOp::Gt
//                     | BinaryOp::Ge => {
//                         constraints.push(WidthConstraint::Boolean { node: node_id });
//                     }
//                 }
//             }
//             FlatNode::Prev { signal, .. } => {
//                 if let Some(&width) = signals.get(signal) {
//                     constraints.push(WidthConstraint::Fixed { node: node_id, width });
//                 } else {
//                     constraints.push(WidthConstraint::Fixed { node: node_id, width: 1 });
//                 }
//             }
//             FlatNode::ArrayIndex { width, .. } => {
//                 // ArrayIndex width is resolved by flattening from the array element type.
//                 constraints.push(WidthConstraint::Fixed { node: node_id, width: *width });
//             }
//             FlatNode::FieldAccess { .. } => {
//                 // FieldAccess width is determined during flattening via typeck.
//                 // We use Fixed constraint here because flattening already resolved it.
//                 if let FlatNode::FieldAccess { width, .. } = node {
//                     constraints.push(WidthConstraint::Fixed { node: node_id, width: *width });
//                 }
//             }
//             FlatNode::ArrayLiteral { elements, .. } => {
//                 constraints
//                     .push(WidthConstraint::SumAll { node: node_id, elements: elements.clone() });
//             }
//             FlatNode::StructLiteral { fields, .. } => {
//                 let elements: Vec<u32> = fields.iter().map(|(_, id)| *id).collect();
//                 constraints.push(WidthConstraint::SumAll { node: node_id, elements });
//             }
//             FlatNode::UnfoldIndex { name } => {
//                 // UnfoldIndex should have been turned into a concrete signal by scope expansion.
//                 // If it reaches constraint generation, emit a semantic-width diagnostic.
//                 diagnostics.push(
//                     WidthDiag::error(format!(
//                         "{} unresolved UnfoldIndex '{}' reached width constraints",
//                         crate::error_codes::ec(506),
//                         name
//                     ))
//                     .with_code("E506"),
//                 );
//                 constraints.push(WidthConstraint::Fixed { node: node_id, width: 32 });
//             }
//         }
//     }
//
//     ConstraintSet { constraints, diagnostics }
// }

/// Generate WidthConstraintComponent for all ECS Entities in the Registry
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
