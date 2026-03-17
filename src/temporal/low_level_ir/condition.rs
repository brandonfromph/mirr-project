//! ConditionKind — typed representation of supported guard condition forms.

#![forbid(unsafe_code)]

use crate::ast::{
    types::{BinaryOp, LiteralValue, UnaryOp},
    Expr,
};
use serde::{Deserialize, Serialize};

/// The set of condition forms that the Temporal Guard Compiler can lower.
///
/// Only variants listed here are accepted. Any other `Expr` form causes a
/// `MirrError::TemporalCompilationError` — there is no silent fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionKind {
    /// `when <signal>` — monitor signal going high
    SimpleSignal(String),
    /// `when !<signal>` / `when not <signal>` — monitor signal going low
    NegatedSignal(String),
    /// `when <signal> <op> <literal>` — magnitude or equality comparison.
    ///
    /// Supported operators: `==`, `!=`, `<`, `<=`, `>`, `>=`.
    /// All six forms lower to a hardware comparator circuit. (P2-REQ-015, Step 2.2)
    Comparison {
        /// The signal being compared
        signal: String,
        /// The comparison operator
        op: BinaryOp,
        /// The literal value on the right-hand side
        value: LiteralValue,
    },
}

impl ConditionKind {
    /// Return the primary signal name driven by this condition.
    pub fn primary_signal(&self) -> &str {
        match self {
            ConditionKind::SimpleSignal(s) => s,
            ConditionKind::NegatedSignal(s) => s,
            ConditionKind::Comparison { signal, .. } => signal,
        }
    }

    /// Return a human-readable description suitable for DOT/HTML emission.
    pub fn describe(&self) -> String {
        match self {
            ConditionKind::SimpleSignal(s) => format!("when {s} (high)"),
            ConditionKind::NegatedSignal(s) => format!("when !{s} (low)"),
            ConditionKind::Comparison { signal, op, value } => {
                let op_str = match op {
                    BinaryOp::Eq => "==",
                    BinaryOp::Ne => "!=",
                    BinaryOp::Lt => "<",
                    BinaryOp::Le => "<=",
                    BinaryOp::Gt => ">",
                    BinaryOp::Ge => ">=",
                    BinaryOp::And => "AND",
                    BinaryOp::Or => "OR",
                    BinaryOp::Xor => "XOR",
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Shl => "<<",
                    BinaryOp::Shr => ">>",
                };
                let val_str = match value {
                    LiteralValue::Integer(n) => format!("{n}"),
                    LiteralValue::Bool(b) => format!("{b}"),
                };
                format!("when {signal} {op_str} {val_str}")
            }
        }
    }

    /// Attempt to lower an `Expr` into a `ConditionKind`.
    ///
    /// Returns `Err(&'static str)` for unsupported forms — no heap allocation
    /// on the error path. The caller embeds the guard name in diagnostics.
    pub fn try_from_expr(expr: &Expr) -> Result<Self, &'static str> {
        match expr {
            Expr::Signal(name) => Ok(ConditionKind::SimpleSignal(name.clone())),
            Expr::Unary { op: UnaryOp::Not, operand } => match operand.as_ref() {
                Expr::Signal(name) => Ok(ConditionKind::NegatedSignal(name.clone())),
                _ => Err("negation of non-signal expressions is not supported"),
            },
            Expr::Binary { op, left, right }
                if matches!(
                    op,
                    BinaryOp::Eq
                        | BinaryOp::Ne
                        | BinaryOp::Lt
                        | BinaryOp::Le
                        | BinaryOp::Gt
                        | BinaryOp::Ge
                ) =>
            {
                match (left.as_ref(), right.as_ref()) {
                    (Expr::Signal(s), Expr::Literal(v)) => Ok(ConditionKind::Comparison {
                        signal: s.clone(),
                        op: *op,
                        value: v.clone(),
                    }),
                    _ => Err("comparisons must be of the form <signal> <op> <literal>"),
                }
            }
            _ => Err("unsupported condition expression form"),
        }
    }
}
