// ---------------------------------------------------------------------------
// Expression AST node
// ---------------------------------------------------------------------------
// Single responsibility: the recursive expression tree structure.
// ---------------------------------------------------------------------------

use super::types::{BinaryOp, LiteralValue, UnaryOp};

use serde::{Deserialize, Serialize};

/// Expression tree node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expr {
    Literal(LiteralValue),
    Signal(String),
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Previous-tick reference: reads `signal` at tick `t - delay`.
    /// In hardware, this maps to a register chain of length `delay`.
    /// `delay` must be >= 1 (enforced by validation).
    Prev {
        signal: String,
        delay: u64,
    },
}
