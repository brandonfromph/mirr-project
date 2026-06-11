//! ALU execution and type tag conversion helpers.

#![forbid(unsafe_code)]

use crate::emit::rspu_isa::{AluOp, AluUnaryOp};
use crate::emit::rspu_tagged::TypeTag;

// ---------------------------------------------------------------------------
// ALU execution helpers
// ---------------------------------------------------------------------------

/// Execute a binary ALU operation on raw 64-bit values.
///
/// All arithmetic uses wrapping semantics to avoid overflow panics.
pub(super) fn execute_alu(op: AluOp, a: u64, b: u64) -> u64 {
    match op {
        AluOp::Add => a.wrapping_add(b),
        AluOp::Sub => a.wrapping_sub(b),
        AluOp::Mul => a.wrapping_mul(b),
        AluOp::And => a & b,
        AluOp::Or => a | b,
        AluOp::Xor => a ^ b,
        AluOp::Shl => a.wrapping_shl(b as u32),
        AluOp::Shr => a.wrapping_shr(b as u32),
        AluOp::Eq => u64::from(a == b),
        AluOp::Ne => u64::from(a != b),
        AluOp::Lt => u64::from(a < b),
        AluOp::Le => u64::from(a <= b),
        AluOp::Gt => u64::from(a > b),
        AluOp::Ge => u64::from(a >= b),
    }
}

/// Execute a unary ALU operation on a raw 64-bit value.
pub(super) fn execute_alu_unary(op: AluUnaryOp, a: u64, tag: TypeTag) -> u64 {
    match op {
        AluUnaryOp::Not => match tag {
            TypeTag::Bool => {
                if a == 0 {
                    1
                } else {
                    0
                }
            }
            TypeTag::Unsigned { width } | TypeTag::Signed { width } => {
                let mask = if width >= 64 { u64::MAX } else { (1u64 << width) - 1 };
                (!a) & mask
            }
            _ => !a,
        },
        AluUnaryOp::Negate => match tag {
            TypeTag::Unsigned { width } | TypeTag::Signed { width } => {
                let mask = if width >= 64 { u64::MAX } else { (1u64 << width) - 1 };
                ((a as i64).wrapping_neg() as u64) & mask
            }
            _ => (a as i64).wrapping_neg() as u64,
        },
        AluUnaryOp::ReductionOr => {
            if a != 0 {
                1
            } else {
                0
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Type tag conversion helpers
// ---------------------------------------------------------------------------

/// Convert a u8 encoding to a `TypeTag`.
///
/// Encoding scheme:
/// - 0 => Uninitialized
/// - 1 => Bool
/// - 2..=127 => Unsigned { width: n }
/// - 128..=255 => Signed { width: n - 128 }
pub(super) fn u8_to_type_tag(tag: u8) -> TypeTag {
    match tag {
        0 => TypeTag::Uninitialized,
        1 => TypeTag::Bool,
        n if n >= 128 => TypeTag::Signed { width: n.wrapping_sub(128) },
        n => TypeTag::Unsigned { width: n },
    }
}

/// Convert a `TypeTag` to its u8 encoding.
pub(super) fn type_tag_to_u8(tag: &TypeTag) -> u8 {
    match tag {
        TypeTag::Uninitialized => 0,
        TypeTag::Bool => 1,
        TypeTag::Unsigned { width } => *width,
        TypeTag::Signed { width } => width.wrapping_add(128),
        TypeTag::Interval { .. } => 2, // Encode as generic unsigned for tag byte
    }
}

/// Convert a width in bits to a `TypeTag`.
///
/// Widths of 0 or 1 map to `Bool`; all others map to `Unsigned`.
pub(super) fn width_to_type_tag(width: u32) -> TypeTag {
    if width <= 1 {
        TypeTag::Bool
    } else if width <= 127 {
        TypeTag::Unsigned { width: width as u8 }
    } else {
        TypeTag::Unsigned { width: 127 }
    }
}
