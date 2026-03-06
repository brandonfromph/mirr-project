// ---------------------------------------------------------------------------
// AST primitive types
// ---------------------------------------------------------------------------
// Single responsibility: enumeration types used throughout the AST.
// ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};

/// Kind of signal in a MIRR module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalKind {
    Input,
    Output,
    Internal,
}

/// Type of a signal (boolean or fixed-width unsigned).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalType {
    Bool,
    Unsigned(u32),
}

impl std::fmt::Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalType::Bool => write!(f, "bool"),
            SignalType::Unsigned(width) => write!(f, "u{}", width),
        }
    }
}

/// Binary operator in an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    // Logical
    And,
    Or,
    Xor,
    // Comparison
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    // Arithmetic
    Add,
    Sub,
    Mul,
    Shl,
    Shr,
}

/// Unary operator in an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
}

/// Literal value in an expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiteralValue {
    Bool(bool),
    Integer(u64),
}