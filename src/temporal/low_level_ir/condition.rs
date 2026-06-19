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
                    BinaryOp::BitwiseOr => "BITOR",
                    BinaryOp::BitwiseAnd => "BITAND",
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
    /// Returns `Err(MirrError)` for unsupported forms — strongly typed structured diagnostic.
    /// The caller embeds the guard name in diagnostics.
    pub fn try_from_expr(expr: &Expr) -> Result<Self, MirrError> {
        match expr {
            Expr::Literal(LiteralValue::Bool(true)) => Ok(ConditionKind::AlwaysTrue),
            Expr::Signal(name) => Ok(ConditionKind::SimpleSignal(name.clone())),
            Expr::ArrayIndex { array, index } => {
                if let (Expr::Signal(arr), Expr::Literal(LiteralValue::Integer(idx))) =
                    (array.as_ref(), index.as_ref())
                {
                    Ok(ConditionKind::SimpleSignal(format!("{}[{}]", arr, idx)))
                } else {
                    Err(mirrcode(
                        ErrorCode::TemporalCondUnsupported,
                        "unsupported condition expression form",
                    ))
                }
            }
            Expr::Prev { signal, delay } => {
                Ok(ConditionKind::PrevSignal { signal: signal.clone(), delay: *delay })
            }
            Expr::Unary { op: UnaryOp::Not, operand } => match operand.as_ref() {
                Expr::Signal(name) => Ok(ConditionKind::NegatedSignal(name.clone())),
                Expr::ArrayIndex { array, index } => {
                    if let (Expr::Signal(arr), Expr::Literal(LiteralValue::Integer(idx))) =
                        (array.as_ref(), index.as_ref())
                    {
                        Ok(ConditionKind::NegatedSignal(format!("{}[{}]", arr, idx)))
                    } else {
                        Err(mirrcode(
                            ErrorCode::TemporalCondUnsupported,
                            "negation of non-signal expressions is not supported",
                        ))
                    }
                }
                Expr::Prev { .. } => Err(mirrcode(
                    ErrorCode::TemporalCondUnsupported,
                    "negation of prev() is not yet supported in temporal guards",
                )),
                _ => Err(mirrcode(
                    ErrorCode::TemporalCondUnsupported,
                    "negation of non-signal expressions is not supported",
                )),
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
                    (Expr::ArrayIndex { array, index }, Expr::Literal(v)) => {
                        if let (Expr::Signal(arr), Expr::Literal(LiteralValue::Integer(idx))) =
                            (array.as_ref(), index.as_ref())
                        {
                            Ok(ConditionKind::Comparison {
                                signal: format!("{}[{}]", arr, idx),
                                op: *op,
                                value: v.clone(),
                            })
                        } else {
                            Err(mirrcode(
                                ErrorCode::TemporalCondUnsupported,
                                "comparisons must be of the form <signal> <op> <literal>",
                            ))
                        }
                    }
                    _ => Err(mirrcode(
                        ErrorCode::TemporalCondUnsupported,
                        "comparisons must be of the form <signal> <op> <literal>",
                    )),
                }
            }
            _ => Err(mirrcode(
                ErrorCode::TemporalCondUnsupported,
                "unsupported condition expression form",
            )),
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

        if let Some(lit) = &registry.literals[idx] {
            if lit.0 == LiteralValue::Bool(true) {
                return Ok(ConditionKind::AlwaysTrue);
            }
        }

        // Helper to extract a signal name (either simple or array index)
        let get_signal_name = |ent_idx: usize| -> Result<String, MirrError> {
            if let Some(sig_ref) = &registry.signal_refs[ent_idx] {
                registry.names[sig_ref.0 .0 as usize]
                    .as_ref()
                    .map(|nc| registry.resolve_name(nc.0).to_string())
                    .ok_or_else(|| {
                        mirrcode(
                            ErrorCode::TemporalCondLowerFailed,
                            "Signal reference to unnamed entity",
                        )
                    })
            } else if let Some(arr_idx) = &registry.array_indices[ent_idx] {
                let arr_ent = arr_idx.array.0 as usize;
                let arr_name = if let Some(sig_ref) = &registry.signal_refs[arr_ent] {
                    registry.names[sig_ref.0 .0 as usize]
                        .as_ref()
                        .map(|nc| registry.resolve_name(nc.0).to_string())
                } else {
                    None
                };
                let index_val = if let Some(lit) = &registry.literals[arr_idx.index.0 as usize] {
                    if let LiteralValue::Integer(idx_val) = lit.0 {
                        Some(idx_val)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let (Some(name), Some(val)) = (arr_name, index_val) {
                    Ok(format!("{}[{}]", name, val))
                } else {
                    Err(mirrcode(
                        ErrorCode::TemporalCondLowerFailed,
                        "Complex array index not supported in guards",
                    ))
                }
            } else {
                Err(mirrcode(ErrorCode::TemporalCondLowerFailed, "Expected signal or array index"))
            }
        };

        if registry.signal_refs[idx].is_some() || registry.array_indices[idx].is_some() {
            let name = get_signal_name(idx)?;
            Ok(ConditionKind::SimpleSignal(name))
        } else if let Some(prev) = &registry.prev_ops[idx] {
            let name = get_signal_name(prev.signal.0 as usize)?;
            Ok(ConditionKind::PrevSignal { signal: name, delay: prev.delay })
        } else if let Some(unary) = &registry.unary_ops[idx] {
            if unary.op == UnaryOp::Not {
                let op_idx = unary.operand.0 as usize;
                if registry.signal_refs[op_idx].is_some()
                    || registry.array_indices[op_idx].is_some()
                {
                    let name = get_signal_name(op_idx)?;
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

                    let signal_name = if registry.signal_refs[left_idx].is_some()
                        || registry.array_indices[left_idx].is_some()
                    {
                        get_signal_name(left_idx)?
                    } else {
                        return Err(mirrcode(
                            ErrorCode::TemporalCondUnsupported,
                            "Comparisons must have a signal or array index on the left-hand side",
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
