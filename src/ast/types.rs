// ---------------------------------------------------------------------------
// AST primitive types
// ---------------------------------------------------------------------------
// Single responsibility: enumeration types used throughout the AST.
// ---------------------------------------------------------------------------

/// Kind of signal in a MIRR module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalKind {
    Input,
    Output,
    Internal,
}

/// Type of a signal (boolean or fixed-width unsigned).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalType {
    Bool,
    Unsigned(u32),
}

/// Binary operator in an expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
}

/// Literal value in an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralValue {
    Bool(bool),
    Integer(u64),
}