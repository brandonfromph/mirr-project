//! ConditionKind — typed representation of supported guard condition forms.

#![forbid(unsafe_code)]

use crate::ast::{
    types::{BinaryOp, LiteralValue, UnaryOp},
    Expr,
};
use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};
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
    /// `when prev(<signal>, <delay>)` — monitor delayed signal
    PrevSignal { signal: String, delay: u64 },
    /// `when <signal> <op> <literal>` — magnitude or equality comparison.
    ///
    /// Supported operators: `==`, `!=`, `<`, `<=`, `>`, `>=`.
    /// All six forms lower to a hardware comparator circuit. (P2-REQ-015, Step 2.2)
    Comparison {
        /// The signal being compared
        signal: String,
        /// The comparison operator
        op: BinaryOp,
        value: LiteralValue,
    },
    /// `when always` — immediately active sentinel
    AlwaysTrue,
}

impl ConditionKind {
    /// Return the primary signal name driven by this condition.
    pub fn primary_signal(&self) -> &str {
        match self {
            ConditionKind::SimpleSignal(s) => s,
            ConditionKind::NegatedSignal(s) => s,
            ConditionKind::PrevSignal { signal, .. } => signal,
            ConditionKind::Comparison { signal, .. } => signal,
            ConditionKind::AlwaysTrue => "true",
        }
    }

    /// Return a human-readable description suitable for DOT/HTML emission.
    pub fn describe(&self) -> String {
        match self {
            ConditionKind::SimpleSignal(s) => format!("when {s} (high)"),
            ConditionKind::NegatedSignal(s) => format!("when !{s} (low)"),
            ConditionKind::PrevSignal { signal, delay } => format!("when prev({signal}, {delay})"),
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
            ConditionKind::AlwaysTrue => "always".to_string(),
        }
    }

    /// Attempt to lower an `Expr` into a `ConditionKind`.
    ///
    /// Returns `Err(&'static str)` for unsupported forms — no heap allocation
    /// on the error path. The caller embeds the guard name in diagnostics.
    pub fn try_from_expr(expr: &Expr) -> Result<Self, &'static str> {
        match expr {
            Expr::Signal(name) => Ok(ConditionKind::SimpleSignal(name.clone())),
            Expr::Prev { signal, delay } => {
                Ok(ConditionKind::PrevSignal { signal: signal.clone(), delay: *delay })
            }
            Expr::Unary { op: UnaryOp::Not, operand } => match operand.as_ref() {
                Expr::Signal(name) => Ok(ConditionKind::NegatedSignal(name.clone())),
                Expr::Prev { .. } => {
                    Err("negation of prev() is not yet supported in temporal guards")
                }
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

    /// Factory: Synthesize a [`ConditionKind`] directly from the ECS Registry.
    ///
    /// This is the modern, "AI-native" alternative to `try_from_expr`. It
    /// traverses the Registry's Structure-of-Arrays (SoA) to reconstruct
    /// a hardware condition (Signal, Prev, or Comparison) from an `EntityId`.
    ///
    /// # Arguments
    /// * `registry` - The ECS Registry containing the hardware components.
    /// * `entity_id` - The ID of the entity representing the condition.
    ///
    /// # Errors
    /// Returns a static string error if the entity does not represent a supported
    /// hardware condition or contains an invalid reference.
    pub fn try_from_ecs(
        registry: &crate::ecs::Registry,
        entity_id: crate::ecs::EntityId,
    ) -> Result<Self, MirrError> {
        let idx = entity_id.0 as usize;

        if let Some(sig_ref) = &registry.signal_refs[idx] {
            let name = registry.names[sig_ref.0 .0 as usize]
                .as_ref()
                .map(|n| n.0.clone())
                .ok_or_else(|| {
                    mirrcode(
                        ErrorCode::TemporalCondLowerFailed,
                        "Signal reference to unnamed entity",
                    )
                })?;
            Ok(ConditionKind::SimpleSignal(name))
        } else if let Some(prev) = &registry.prev_ops[idx] {
            let name = registry.names[prev.signal.0 as usize]
                .as_ref()
                .map(|n| n.0.clone())
                .ok_or_else(|| {
                    mirrcode(ErrorCode::TemporalCondLowerFailed, "Prev reference to unnamed entity")
                })?;
            Ok(ConditionKind::PrevSignal { signal: name, delay: prev.delay })
        } else if let Some(unary) = &registry.unary_ops[idx] {
            if unary.op == UnaryOp::Not {
                if let Some(sig_ref) = &registry.signal_refs[unary.operand.0 as usize] {
                    let name = registry.names[sig_ref.0 .0 as usize]
                        .as_ref()
                        .map(|n| n.0.clone())
                        .ok_or_else(|| {
                            mirrcode(
                                ErrorCode::TemporalCondLowerFailed,
                                "Signal reference to unnamed entity",
                            )
                        })?;
                    Ok(ConditionKind::NegatedSignal(name))
                } else {
                    Err(mirrcode(
                        ErrorCode::TemporalCondUnsupported,
                        "Negation of complex expressions is unsupported in hardware guards",
                    ))
                }
            } else {
                Err(mirrcode(
                    ErrorCode::TemporalCondUnsupported,
                    format!("Unsupported unary operator {:?} in hardware guard", unary.op),
                ))
            }
        } else if let Some(binary) = &registry.binary_ops[idx] {
            match binary.op {
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge => {
                    let left_idx = binary.left.0 as usize;
                    let right_idx = binary.right.0 as usize;

                    let signal_name = if let Some(sig_ref) = &registry.signal_refs[left_idx] {
                        registry.names[sig_ref.0 .0 as usize]
                            .as_ref()
                            .map(|n| n.0.clone())
                            .ok_or_else(|| {
                                mirrcode(
                                    ErrorCode::TemporalCondLowerFailed,
                                    "Signal reference to unnamed entity",
                                )
                            })?
                    } else {
                        return Err(mirrcode(
                            ErrorCode::TemporalCondUnsupported,
                            "Comparisons must have a signal on the left-hand side",
                        ));
                    };

                    let value = if let Some(lit) = &registry.literals[right_idx] {
                        lit.0.clone()
                    } else {
                        return Err(mirrcode(
                            ErrorCode::TemporalCondUnsupported,
                            "Comparisons must have a literal on the right-hand side",
                        ));
                    };

                    Ok(ConditionKind::Comparison { signal: signal_name, op: binary.op, value })
                }
                _ => Err(mirrcode(
                    ErrorCode::TemporalCondUnsupported,
                    format!("Binary operator {:?} is unsupported in guard conditions", binary.op),
                )),
            }
        } else {
            Err(mirrcode(
                ErrorCode::TemporalCondUnsupported,
                "Entity is not a valid hardware condition",
            ))
        }
    }
}
