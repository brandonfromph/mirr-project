//! Width constraint representation and generation.
//!
//! Encodes the relationship between expression node widths as constraints
//! that the solver propagates to fixpoint.

#![forbid(unsafe_code)]

use crate::ast::types::BinaryOp;
use crate::width::types::{FlatNode, WidthDiag};
use serde::Serialize;
use std::borrow::Borrow;
use std::hash::Hash;

// ---------------------------------------------------------------------------
// Constraint types
// ---------------------------------------------------------------------------

/// A width constraint relating one or more flat nodes.
#[derive(Debug, Clone, Serialize)]
pub enum WidthConstraint {
    /// Node must be exactly `width` bits (literal or declared signal).
    Fixed { node: u32, width: u32 },

    /// Node width = max(left, right) + 1  (for Add).
    MaxPlusOne { node: u32, left: u32, right: u32 },

    /// Node width = max(left, right)  (for Sub, And, Or, Xor).
    MaxOf { node: u32, left: u32, right: u32 },

    /// Node width = left + right  (for Mul).
    SumOf { node: u32, left: u32, right: u32 },

    /// Node width = left_width + shift_const  (for Shl with constant shift).
    LeftPlusConst { node: u32, left: u32, shift_amount: u32 },

    /// Node width = left_width + 63  (for Shl with variable shift — worst case).
    LeftPlusMaxShift { node: u32, left: u32 },

    /// Node width = max(1, left_width - shift_const)  (for Shr with constant shift).
    /// Right shift by a known amount narrows the result; minimum 1 bit guaranteed.
    LeftMinusConst { node: u32, left: u32, shift_amount: u32 },

    /// Node width = left_width  (for Shr with variable shift, Unary Not).
    SameAs { node: u32, source: u32 },

    /// Node width = source_width + 1  (for unsigned-to-signed negate).
    SameAsPlusOne { node: u32, source: u32 },

    /// Node width = sw.min(narrow_width) (for BitwiseAnd with literal).
    Narrowed { node: u32, source: u32, narrow_width: u32 },

    /// Node width = 1  (for comparison operators and boolean literals).
    Boolean { node: u32 },

    /// Node width = sum of all element widths (for array/struct literals).
    SumAll { node: u32, elements: Vec<u32> },
}

// ---------------------------------------------------------------------------
// Constraint generation
// ---------------------------------------------------------------------------

/// Result of constraint generation.
pub struct ConstraintSet {
    /// Generated width constraints for the solver.
    pub constraints: Vec<WidthConstraint>,
    /// Diagnostics emitted during constraint generation (e.g. undeclared signals).
    pub diagnostics: Vec<WidthDiag>,
}

/// Generate width constraints for a flattened expression tree.
pub fn generate_constraints(
    nodes: &[FlatNode],
    signals: &std::collections::HashMap<String, u32>,
) -> ConstraintSet {
    generate_constraints_with_index(nodes, signals)
}

pub(crate) fn generate_constraints_with_index<K>(
    nodes: &[FlatNode],
    signals: &std::collections::HashMap<K, u32>,
) -> ConstraintSet
where
    K: Eq + Hash + Borrow<str>,
{
    let mut constraints = Vec::new();
    let mut diagnostics = Vec::new();

    for (i, node) in nodes.iter().enumerate() {
        let node_id = i as u32;
        match node {
            FlatNode::Literal { value } => {
                let width = min_bits_for(*value);
                constraints.push(WidthConstraint::Fixed { node: node_id, width });
            }
            FlatNode::Signal { name, .. } => {
                if let Some(&width) = signals.get(name) {
                    constraints.push(WidthConstraint::Fixed { node: node_id, width });
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
                    // Fallback to width 1 to allow solver to continue.
                    constraints.push(WidthConstraint::Fixed { node: node_id, width: 1 });
                }
            }
            FlatNode::Unary { op, operand } => {
                match op {
                    crate::ast::types::UnaryOp::Not => {
                        constraints
                            .push(WidthConstraint::SameAs { node: node_id, source: *operand });
                    }
                    crate::ast::types::UnaryOp::Negate => {
                        // Check signedness of operand to decide SameAs vs SameAsPlusOne
                        let is_signed = match nodes.get(*operand as usize) {
                            Some(FlatNode::Signal { signed, .. }) => *signed,
                            Some(FlatNode::Prev { signed, .. }) => *signed,
                            _ => false,
                        };
                        if is_signed {
                            constraints
                                .push(WidthConstraint::SameAs { node: node_id, source: *operand });
                        } else {
                            constraints.push(WidthConstraint::SameAsPlusOne {
                                node: node_id,
                                source: *operand,
                            });
                        }
                    }
                }
            }
            FlatNode::Binary { op, left, right } => {
                if *op == BinaryOp::Sub {
                    // Unsigned subtraction may underflow due modular/wrapping semantics.
                    // Emit info diagnostics when both operands are unsigned or literals.
                    let left_node = nodes.get(*left as usize);
                    let right_node = nodes.get(*right as usize);
                    if let (Some(ln), Some(rn)) = (left_node, right_node) {
                        if is_unsigned_node(ln) && is_unsigned_node(rn) {
                            let left_val = literal_value(ln);
                            let right_val = literal_value(rn);
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
                }

                match op {
                    BinaryOp::Add => {
                        constraints.push(WidthConstraint::MaxPlusOne {
                            node: node_id,
                            left: *left,
                            right: *right,
                        });
                    }
                    BinaryOp::Mul => {
                        constraints.push(WidthConstraint::SumOf {
                            node: node_id,
                            left: *left,
                            right: *right,
                        });
                    }
                    BinaryOp::BitwiseAnd => {
                        // Intelligent width narrowing: if either side is a literal,
                        // we constrain the result to the minimum of the source and the literal.
                        let left_node = nodes.get(*left as usize);
                        let right_node = nodes.get(*right as usize);

                        if let Some(FlatNode::Literal { value }) = left_node {
                            constraints.push(WidthConstraint::Narrowed {
                                node: node_id,
                                source: *right,
                                narrow_width: min_bits_for(*value),
                            });
                        } else if let Some(FlatNode::Literal { value }) = right_node {
                            constraints.push(WidthConstraint::Narrowed {
                                node: node_id,
                                source: *left,
                                narrow_width: min_bits_for(*value),
                            });
                        } else {
                            constraints.push(WidthConstraint::MaxOf {
                                node: node_id,
                                left: *left,
                                right: *right,
                            });
                        }
                    }
                    BinaryOp::Sub
                    | BinaryOp::And
                    | BinaryOp::Or
                    | BinaryOp::BitwiseOr
                    | BinaryOp::Xor => {
                        constraints.push(WidthConstraint::MaxOf {
                            node: node_id,
                            left: *left,
                            right: *right,
                        });
                    }
                    BinaryOp::Shl => {
                        // Check if right is a literal for precision
                        if let Some(FlatNode::Literal { value }) = nodes.get(*right as usize) {
                            constraints.push(WidthConstraint::LeftPlusConst {
                                node: node_id,
                                left: *left,
                                shift_amount: *value as u32,
                            });
                        } else {
                            constraints.push(WidthConstraint::LeftPlusMaxShift {
                                node: node_id,
                                left: *left,
                            });
                        }
                    }
                    BinaryOp::Shr => {
                        if let Some(FlatNode::Literal { value }) = nodes.get(*right as usize) {
                            constraints.push(WidthConstraint::LeftMinusConst {
                                node: node_id,
                                left: *left,
                                shift_amount: *value as u32,
                            });
                        } else {
                            constraints
                                .push(WidthConstraint::SameAs { node: node_id, source: *left });
                        }
                    }
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => {
                        constraints.push(WidthConstraint::Boolean { node: node_id });
                    }
                }
            }
            FlatNode::Prev { signal, .. } => {
                if let Some(&width) = signals.get(signal) {
                    constraints.push(WidthConstraint::Fixed { node: node_id, width });
                } else {
                    constraints.push(WidthConstraint::Fixed { node: node_id, width: 1 });
                }
            }
            FlatNode::ArrayIndex { width, .. } => {
                // ArrayIndex width is resolved by flattening from the array element type.
                constraints.push(WidthConstraint::Fixed { node: node_id, width: *width });
            }
            FlatNode::FieldAccess { .. } => {
                // FieldAccess width is determined during flattening via typeck.
                // We use Fixed constraint here because flattening already resolved it.
                if let FlatNode::FieldAccess { width, .. } = node {
                    constraints.push(WidthConstraint::Fixed { node: node_id, width: *width });
                }
            }
            FlatNode::ArrayLiteral { elements, .. } => {
                constraints
                    .push(WidthConstraint::SumAll { node: node_id, elements: elements.clone() });
            }
            FlatNode::StructLiteral { fields, .. } => {
                let elements: Vec<u32> = fields.iter().map(|(_, id)| *id).collect();
                constraints.push(WidthConstraint::SumAll { node: node_id, elements });
            }
            FlatNode::UnfoldIndex { name } => {
                // UnfoldIndex should have been turned into a concrete signal by scope expansion.
                // If it reaches constraint generation, emit a semantic-width diagnostic.
                diagnostics.push(
                    WidthDiag::error(format!(
                        "{} unresolved UnfoldIndex '{}' reached width constraints",
                        crate::error_codes::ec(506),
                        name
                    ))
                    .with_code("E506"),
                );
                constraints.push(WidthConstraint::Fixed { node: node_id, width: 32 });
            }
        }
    }

    ConstraintSet { constraints, diagnostics }
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
fn is_unsigned_node(node: &FlatNode) -> bool {
    match node {
        FlatNode::Literal { .. } => true,
        FlatNode::Signal { signed, .. } => !*signed,
        FlatNode::Prev { signed, .. } => !*signed,
        FlatNode::ArrayIndex { signed, .. } => !*signed,
        FlatNode::FieldAccess { signed, .. } => !*signed,
        FlatNode::ArrayLiteral { .. } => true,
        FlatNode::StructLiteral { .. } => true,
        _ => false,
    }
}

/// Get literal value if this node is a literal constant.
fn literal_value(node: &FlatNode) -> Option<u64> {
    match node {
        FlatNode::Literal { value } => Some(*value),
        _ => None,
    }
}
