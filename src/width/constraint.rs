//! Width constraint representation and generation.
//!
//! Encodes the relationship between expression node widths as constraints
//! that the solver propagates to fixpoint. Constraint generation is a single
//! bounded pass over the flat node array.

#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, UnaryOp, SignalType};
use crate::ast::SignalDecl;
use super::types::{FlatNode, Width, WidthDiag, MAX_FLAT_NODES};

// ---------------------------------------------------------------------------
// Constraint enum
// ---------------------------------------------------------------------------

/// A width constraint on a node identified by its flat-array index.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    /// Node width = 1  (for comparison operators and boolean literals).
    Boolean { node: u32 },
}

// ---------------------------------------------------------------------------
// Constraint generation
// ---------------------------------------------------------------------------

/// Result of constraint generation.
pub struct ConstraintSet {
    pub constraints: Vec<WidthConstraint>,
    pub diagnostics: Vec<WidthDiag>,
}

/// Generate width constraints from a flat node array.
///
/// `signals` is the module's signal declarations, used to look up declared
/// widths for Signal nodes.
///
/// Bounded: iterates once over `nodes` (len <= MAX_FLAT_NODES).
pub fn generate_constraints(
    nodes: &[FlatNode],
    signals: &[SignalDecl],
) -> ConstraintSet {
    let mut constraints: Vec<WidthConstraint> = Vec::with_capacity(nodes.len());
    let mut diagnostics: Vec<WidthDiag> = Vec::new();

    for (i, node) in nodes.iter().enumerate() {
        if i >= MAX_FLAT_NODES {
            break;
        }
        let id = i as u32;

        match node {
            FlatNode::Literal { value } => {
                // Bool literals (0 or 1) get width 1; integers use min_bits.
                let w = Width::min_bits_for(*value);
                constraints.push(WidthConstraint::Fixed { node: id, width: w.0 });
            }
            FlatNode::Signal { name } => {
                let declared = lookup_signal_width(name, signals);
                match declared {
                    Some(w) => {
                        constraints.push(WidthConstraint::Fixed { node: id, width: w });
                    }
                    None => {
                        diagnostics.push(WidthDiag::error(format!(
                            "signal '{}' has no declared width", name
                        )));
                        // Default to 1 to allow solving to continue.
                        constraints.push(WidthConstraint::Fixed { node: id, width: 1 });
                    }
                }
            }
            FlatNode::Unary { op, operand } => match op {
                UnaryOp::Not => {
                    // Bitwise NOT preserves width.
                    constraints.push(WidthConstraint::SameAs { node: id, source: *operand });
                }
            },
            FlatNode::Binary { op, left, right } => {
                generate_binary_constraint(
                    id, *op, *left, *right, nodes, &mut constraints, &mut diagnostics,
                );
            }
        }
    }

    ConstraintSet { constraints, diagnostics }
}

/// Generate width constraint for a binary operation.
///
/// Split out to keep `generate_constraints` under 60 lines.
fn generate_binary_constraint(
    id: u32,
    op: BinaryOp,
    left: u32,
    right: u32,
    nodes: &[FlatNode],
    constraints: &mut Vec<WidthConstraint>,
    diagnostics: &mut Vec<WidthDiag>,
) {
    match op {
        BinaryOp::Add => {
            constraints.push(WidthConstraint::MaxPlusOne { node: id, left, right });
        }
        BinaryOp::Sub => {
            constraints.push(WidthConstraint::MaxOf { node: id, left, right });
            // Only emit underflow info when we cannot prove safety at compile time.
            // If both operands are literals and left >= right, no underflow is possible.
            let left_val = get_literal_value(left, nodes);
            let right_val = get_literal_value(right, nodes);
            let provably_safe = matches!((left_val, right_val), (Some(l), Some(r)) if l >= r);
            if !provably_safe {
                diagnostics.push(WidthDiag::info(
                    "unsigned subtraction may underflow (wrapping semantics)".to_string(),
                ));
            }
        }
        BinaryOp::Mul => {
            constraints.push(WidthConstraint::SumOf { node: id, left, right });
        }
        BinaryOp::Shl => {
            // If shift amount is a constant literal, use exact width.
            // Otherwise use worst-case (shift by 63).
            let shift_const = get_literal_value(right, nodes);
            match shift_const {
                Some(amt) => {
                    let clamped = amt.min(63) as u32;
                    constraints.push(WidthConstraint::LeftPlusConst {
                        node: id, left, shift_amount: clamped,
                    });
                }
                None => {
                    constraints.push(WidthConstraint::LeftPlusMaxShift { node: id, left });
                }
            }
        }
        BinaryOp::Shr => {
            // If shift amount is a constant, narrow precisely: max(1, w(left) - k).
            // If variable, use SameAs (conservative: shift of 0 needs full width).
            let shift_const = get_literal_value(right, nodes);
            match shift_const {
                Some(amt) => {
                    let clamped = amt.min(63) as u32;
                    constraints.push(WidthConstraint::LeftMinusConst {
                        node: id, left, shift_amount: clamped,
                    });
                }
                None => {
                    constraints.push(WidthConstraint::SameAs { node: id, source: left });
                }
            }
        }
        BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => {
            constraints.push(WidthConstraint::MaxOf { node: id, left, right });
        }
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt
        | BinaryOp::Ge | BinaryOp::Eq | BinaryOp::Ne => {
            constraints.push(WidthConstraint::Boolean { node: id });
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Look up a signal's declared width from the signal declarations.
/// Returns `None` if the signal is not found.
fn lookup_signal_width(name: &str, signals: &[SignalDecl]) -> Option<u32> {
    for s in signals {
        if s.name == name {
            return match s.ty {
                SignalType::Bool => Some(1),
                SignalType::Unsigned(w) => Some(w),
            };
        }
    }
    None
}

/// If the node at `idx` is a Literal, return its value; else None.
fn get_literal_value(idx: u32, nodes: &[FlatNode]) -> Option<u64> {
    nodes.get(idx as usize).and_then(|n| match n {
        FlatNode::Literal { value } => Some(*value),
        _ => None,
    })
}
