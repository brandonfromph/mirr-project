// ---------------------------------------------------------------------------
//! Expression AST for MIRR guard conditions and reflex assignments.
//!
//! Supports signals, literals, binary/unary operations, and temporal
//! back-references (`Prev`). All variants are `Serialize`/`Deserialize`.
// ---------------------------------------------------------------------------

use super::types::{BinaryOp, LiteralValue, UnaryOp};

use serde::{Deserialize, Serialize};

/// Expression tree node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expr {
    /// A literal value (boolean or integer constant).
    Literal(LiteralValue),
    /// A signal reference by name.
    Signal(String),
    /// Unary operation (NOT or negate) applied to an operand.
    Unary { op: UnaryOp, operand: Box<Expr> },
    /// Binary operation applied to left and right operands.
    Binary { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
    /// Previous-tick reference: reads `signal` at tick `t - delay`.
    /// In hardware, this maps to a register chain of length `delay`.
    /// `delay` must be >= 1 (enforced by validation).
    Prev { signal: String, delay: u64 },
}
