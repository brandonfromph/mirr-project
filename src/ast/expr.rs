// ---------------------------------------------------------------------------
//! Expression AST for MIRR guard conditions and reflex assignments.
//!
//! Supports signals, literals, binary/unary operations, temporal
//! back-references (`Prev`), and composite data access (MEGA-10: arrays,
//! structs). All variants are `Serialize`/`Deserialize`.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

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
    /// Array indexing: `array[index]`. The index expression must resolve to
    /// a type whose refinement proves it is within the array bounds.
    ArrayIndex { array: Box<Expr>, index: Box<Expr> },
    /// Struct/bundle field access: `expr.field_name`.
    FieldAccess { object: Box<Expr>, field: String },
    /// Array literal: `[e0, e1, ..., eN]`. All elements must have the same type.
    ArrayLiteral(Vec<Expr>),
    /// Struct literal: `StructName { f1: v1, f2: v2, ... }`.
    StructLiteral { name: String, fields: Vec<(String, Expr)> },
    /// Meta-stage unfolding index (March 17-22 history).
    UnfoldIndex(String),
}
