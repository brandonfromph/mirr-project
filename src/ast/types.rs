// ---------------------------------------------------------------------------
//! Core type definitions for the MIRR AST.
//!
//! Defines signal kinds (input/output/internal), signal types (bool, unsigned),
//! binary and unary operators, and literal values.
// ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};

/// Kind of signal in a MIRR module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalKind {
    /// Hardware input port (read-only from module perspective).
    Input,
    /// Hardware output port (driven by reflexes).
    Output,
    /// Module-internal signal (persists across clock ticks).
    Internal,
}

/// Type of a signal (boolean, fixed-width unsigned, or fixed-width signed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalType {
    /// Single-bit boolean (true/false).
    Bool,
    /// Fixed-width unsigned integer (`u8`, `u16`, `u32`, `u64`).
    Unsigned(u32),
    /// Fixed-width signed two's complement integer (`i8`, `i16`, `i32`, `i64`).
    Signed(u32),
}

impl std::fmt::Display for SignalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalType::Bool => write!(f, "bool"),
            SignalType::Unsigned(width) => write!(f, "u{}", width),
            SignalType::Signed(width) => write!(f, "i{}", width),
        }
    }
}

/// Binary operator in an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    /// Logical AND (`&&`). Requires bool operands.
    And,
    /// Logical OR (`||`). Requires bool operands.
    Or,
    /// Bitwise XOR (`^`). Requires matching types.
    Xor,
    /// Less than (`<`). Returns bool.
    Lt,
    /// Less than or equal (`<=`). Returns bool.
    Le,
    /// Greater than (`>`). Returns bool.
    Gt,
    /// Greater than or equal (`>=`). Returns bool.
    Ge,
    /// Equal (`==`). Returns bool.
    Eq,
    /// Not equal (`!=`). Returns bool.
    Ne,
    /// Addition (`+`). Requires numeric operands, same signedness.
    Add,
    /// Subtraction (`-`). Requires numeric operands, same signedness.
    Sub,
    /// Multiplication (`*`). Requires numeric operands, same signedness.
    Mul,
    /// Left shift (`<<`). Result width = left operand width.
    Shl,
    /// Right shift (`>>`). Result width = left operand width.
    Shr,
}

/// Unary operator in an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    /// Logical/bitwise NOT (`!`). Works on bool, unsigned, and signed.
    Not,
    /// Arithmetic negation (`-`). Unsigned(N) -> Signed(N+1), Signed(N) -> Signed(N).
    Negate,
}

/// Literal value in an expression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiteralValue {
    /// Boolean constant (`true` or `false`).
    Bool(bool),
    /// Unsigned integer constant (inferred as `Unsigned(min_bits)`).
    Integer(u64),
}
