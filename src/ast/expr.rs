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
}
