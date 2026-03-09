//! TYPE-001/TYPE-002/TYPE-003: Semantic type checker for MIRR modules.
//!
//! Runs after semantic validation (name/reference checks) and before
//! simplification. Enforces type compatibility across all expressions:
//! guard conditions, reflex assignments, and property formulas.
//!
//! Type rules are documented in `proposals/002-TYPE-001-2026-03-08.md`
//! and `proposals/003-TYPE-002-2026-03-08.md`.
//!
//! Error codes: E601–E607 (see `docs/error_codes.md`).

#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::ast::expr::Expr;
use crate::ast::program::Module;
use crate::ast::property::PropertyDecl;
use crate::ast::types::{BinaryOp, LiteralValue, SignalType, UnaryOp};
use crate::ast::MAX_EXPR_NODES;
use crate::error::{MirrError, PipelineErrors};
use crate::span::Span;

/// Expression type map: maps each expression (by pointer identity) to its
/// inferred `SignalType`. Returned by `typecheck_module` so downstream
/// passes (e.g., width inference) can query signedness without re-walking.
pub type TypeMap = HashMap<*const Expr, SignalType>;

/// Operator display name for error messages.
fn op_symbol(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
        BinaryOp::Xor => "^",
        BinaryOp::Lt => "<",
        BinaryOp::Le => "<=",
        BinaryOp::Gt => ">",
        BinaryOp::Ge => ">=",
        BinaryOp::Eq => "==",
        BinaryOp::Ne => "!=",
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Shl => "<<",
        BinaryOp::Shr => ">>",
    }
}

/// Type-check all expressions in a parsed module.
///
/// Verifies:
/// - Guard conditions evaluate to `Bool`.
/// - Assignment types are compatible with their target signals.
/// - All operator applications are well-typed.
/// - Property formulas are well-typed.
///
/// Returns a `TypeMap` containing the inferred type for every expression
/// node visited. Downstream passes can query this map instead of
/// re-walking the expression trees.
///
/// Bounded: iterates over guards + reflexes + properties, each expression
/// bounded by MAX_EXPR_NODES.
///
/// Errors are accumulated across expressions (inter-expression accumulation)
/// but within a single expression tree, inference stops at the first error
/// (intra-expression fail-fast) because parent node types depend on children.
pub fn typecheck_module(module: &Module) -> Result<TypeMap, PipelineErrors> {
    // Build signal type lookup table.
    let mut signals: HashMap<&str, SignalType> = HashMap::with_capacity(module.signals.len());
    for sig in &module.signals {
        signals.insert(&sig.name, sig.ty);
    }

    let mut all_types: TypeMap = HashMap::new();
    let mut errors = PipelineErrors::new();

    // T14: Guard conditions must be Bool.
    for guard in &module.guards {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        match infer_expr_type(&guard.condition, &signals, guard.span) {
            Ok((cond_ty, expr_types)) => {
                all_types.extend(expr_types);
                if cond_ty != SignalType::Bool {
                    errors.push(MirrError::TypeError {
                        message: format!(
                            "[E601] Guard '{}' condition must be bool, got {}.",
                            guard.name, cond_ty
                        ),
                        span: guard.span,
                    });
                }
            }
            Err(e) => {
                errors.push(e);
            }
        }
    }

    // T1: Assignment type compatibility.
    for reflex in &module.reflexes {
        for assignment in &reflex.assignments {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                break;
            }
            let target_ty = match signals.get(assignment.target.as_str()) {
                Some(ty) => *ty,
                None => continue, // Undeclared target — caught by semantic validation.
            };
            match infer_expr_type(&assignment.value, &signals, assignment.span) {
                Ok((expr_ty, expr_types)) => {
                    all_types.extend(expr_types);
                    if !types_compatible(target_ty, expr_ty) {
                        errors.push(MirrError::TypeError {
                            message: format!(
                                "[E602] Assignment to '{}' ({}): expression type {} is not compatible.",
                                assignment.target, target_ty, expr_ty
                            ),
                            span: assignment.span,
                        });
                    }
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }
    }

    // Type-check property formulas.
    check_property_formulas(&module.properties, &signals, &mut all_types, &mut errors);

    if errors.is_empty() {
        Ok(all_types)
    } else {
        Err(errors)
    }
}

/// Check whether an expression type is compatible with a target type.
///
/// Exact match is always compatible. Additionally:
/// - `Bool` ↔ `Unsigned(1)` promotion in either direction.
/// - `Unsigned(N)` → `Unsigned(M)` when N ≤ M (safe zero-extension).
/// - `Signed(N)` → `Signed(M)` when N ≤ M (safe sign-extension).
/// - No cross-category: Signed ↔ Unsigned is always rejected.
/// - No `Bool` ↔ `Signed(1)` promotion (1-bit signed = {-1,0}, not {false,true}).
fn types_compatible(target: SignalType, expr: SignalType) -> bool {
    if target == expr {
        return true;
    }
    match (target, expr) {
        // Bool ↔ Unsigned(1) promotion.
        (SignalType::Bool, SignalType::Unsigned(1))
        | (SignalType::Unsigned(1), SignalType::Bool) => true,
        // Safe unsigned widening: narrower fits in wider without truncation.
        (SignalType::Unsigned(target_w), SignalType::Unsigned(expr_w)) => expr_w <= target_w,
        // Safe signed widening: narrower fits in wider via sign-extension.
        (SignalType::Signed(target_w), SignalType::Signed(expr_w)) => expr_w <= target_w,
        _ => false,
    }
}

/// Infer the type of an expression, reporting type errors.
///
/// Uses an explicit stack to avoid recursion (NASA P10 compliance).
/// Bounded: at most MAX_EXPR_NODES iterations.
fn infer_expr_type(
    expr: &Expr,
    signals: &HashMap<&str, SignalType>,
    context_span: Option<Span>,
) -> Result<(SignalType, TypeMap), MirrError> {
    // For bounded, non-recursive traversal we use a two-phase approach:
    // 1. Flatten the expression tree into a post-order work list.
    // 2. Evaluate types bottom-up from the work list.
    let mut work: Vec<&Expr> = Vec::with_capacity(32);
    let mut order: Vec<&Expr> = Vec::with_capacity(32);
    work.push(expr);
    let mut visited = 0usize;

    // Phase 1: Post-order traversal to build evaluation order.
    while let Some(node) = work.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        order.push(node);
        match node {
            Expr::Literal(_) | Expr::Signal(_) | Expr::Prev { .. } => {}
            Expr::Unary { operand, .. } => {
                work.push(operand);
            }
            Expr::Binary { left, right, .. } => {
                work.push(left);
                work.push(right);
            }
        }
    }

    // Phase 2: Evaluate types bottom-up.
    // We use a HashMap keyed by pointer identity to store computed types.
    let mut types: HashMap<*const Expr, SignalType> = HashMap::with_capacity(order.len());

    for node in order.iter().rev() {
        let ptr = *node as *const Expr;
        let ty = match node {
            // T15: Literal bool → Bool.
            Expr::Literal(LiteralValue::Bool(_)) => SignalType::Bool,
            // T16: Literal integer → Unsigned(min_bits).
            Expr::Literal(LiteralValue::Integer(v)) => {
                let bits = min_bits_for(*v);
                SignalType::Unsigned(bits)
            }
            // Signal → declared type.
            Expr::Signal(name) => match signals.get(name.as_str()) {
                Some(ty) => *ty,
                None => continue, // Undeclared — caught by semantic validation.
            },
            // T13: Prev preserves signal type.
            Expr::Prev { signal, .. } => match signals.get(signal.as_str()) {
                Some(ty) => *ty,
                None => continue, // Undeclared — caught by semantic validation.
            },
            // Unary operators.
            Expr::Unary { op, operand, .. } => {
                let operand_ptr = operand.as_ref() as *const Expr;
                let operand_ty = match types.get(&operand_ptr) {
                    Some(ty) => *ty,
                    None => continue,
                };
                match op {
                    // T11/T12: Not works on Bool, Unsigned, and Signed.
                    UnaryOp::Not => operand_ty,
                    // Negate: Unsigned(N) → Signed(N+1), Signed(N) → Signed(N),
                    // Bool → error.
                    UnaryOp::Negate => infer_negate_type(operand_ty, context_span)?,
                }
            }
            // Binary operators.
            Expr::Binary { op, left, right, .. } => {
                let left_ptr = left.as_ref() as *const Expr;
                let right_ptr = right.as_ref() as *const Expr;
                let left_ty = match types.get(&left_ptr) {
                    Some(ty) => *ty,
                    None => continue,
                };
                let right_ty = match types.get(&right_ptr) {
                    Some(ty) => *ty,
                    None => continue,
                };
                infer_binary_type(*op, left_ty, right_ty, context_span)?
            }
        };
        types.insert(ptr, ty);
    }

    // The root expression's type.
    let root_ptr = expr as *const Expr;
    match types.get(&root_ptr) {
        Some(ty) => Ok((*ty, types)),
        None => Ok((SignalType::Bool, types)), // Degenerate/empty — default to bool.
    }
}

/// Infer the result type of a binary operation, or reject with a type error.
fn infer_binary_type(
    op: BinaryOp,
    left: SignalType,
    right: SignalType,
    context_span: Option<Span>,
) -> Result<SignalType, MirrError> {
    match op {
        // T2/T3/T4: Arithmetic and shift operators require numeric operands.
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Shl | BinaryOp::Shr => {
            let (left_w, left_signed) = require_numeric(op, left, right, context_span)?;
            let (right_w, right_signed) = require_numeric(op, right, left, context_span)?;
            // Cross-category: reject mixed signed/unsigned arithmetic.
            if left_signed != right_signed {
                return Err(MirrError::TypeError {
                    message: format!(
                        "[E608] Operator '{}' cannot mix signed and unsigned operands: {} and {}.",
                        op_symbol(op),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
            match op {
                // T4: Shift result width = left width, preserving signedness.
                BinaryOp::Shl | BinaryOp::Shr => {
                    if left_signed {
                        Ok(SignalType::Signed(left_w))
                    } else {
                        Ok(SignalType::Unsigned(left_w))
                    }
                }
                // T2: Arithmetic result width = max(left, right), preserving signedness.
                _ => {
                    let max_w = left_w.max(right_w);
                    if left_signed {
                        Ok(SignalType::Signed(max_w))
                    } else {
                        Ok(SignalType::Unsigned(max_w))
                    }
                }
            }
        }

        // T8/T9: Logical operators require bool operands.
        BinaryOp::And | BinaryOp::Or => {
            if left != SignalType::Bool || right != SignalType::Bool {
                return Err(MirrError::TypeError {
                    message: format!(
                        "[E604] Operator '{}' requires bool operands, got {} and {}.",
                        op_symbol(op),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
            Ok(SignalType::Bool)
        }

        // T10: XOR requires matching types.
        BinaryOp::Xor => {
            if left != right {
                // Allow Bool ↔ Unsigned(1) for xor.
                if !types_compatible(left, right) {
                    return Err(MirrError::TypeError {
                        message: format!(
                            "[E607] Operator '^' (xor) requires matching types, got {} and {}.",
                            left, right
                        ),
                        span: context_span,
                    });
                }
            }
            Ok(left)
        }

        // T5/T7: Ordering comparisons.
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
            // T7: Ordering on Bool is an error.
            if left == SignalType::Bool || right == SignalType::Bool {
                return Err(MirrError::TypeError {
                    message: format!(
                        "[E605] Ordering operator '{}' cannot compare {} and {}.",
                        op_symbol(op),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
            // Cross-category: reject signed vs unsigned ordering.
            let left_signed = matches!(left, SignalType::Signed(_));
            let right_signed = matches!(right, SignalType::Signed(_));
            if left_signed != right_signed {
                return Err(MirrError::TypeError {
                    message: format!(
                        "[E605] Ordering operator '{}' cannot compare {} and {} (signed/unsigned mismatch).",
                        op_symbol(op), left, right
                    ),
                    span: context_span,
                });
            }
            Ok(SignalType::Bool)
        }

        // T6: Equality comparisons.
        BinaryOp::Eq | BinaryOp::Ne => {
            // Same category required (both bool, both unsigned, or both signed).
            let same_category = matches!(
                (left, right),
                (SignalType::Bool, SignalType::Bool)
                    | (SignalType::Unsigned(_), SignalType::Unsigned(_))
                    | (SignalType::Signed(_), SignalType::Signed(_))
            );
            if !same_category {
                return Err(MirrError::TypeError {
                    message: format!(
                        "[E606] Equality operator '{}' cannot compare {} and {}.",
                        op_symbol(op),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
            Ok(SignalType::Bool)
        }
    }
}

/// Extract numeric width and signedness, or emit E603 if the operand is Bool.
fn require_numeric(
    op: BinaryOp,
    ty: SignalType,
    other: SignalType,
    context_span: Option<Span>,
) -> Result<(u32, bool), MirrError> {
    match ty {
        SignalType::Unsigned(w) => Ok((w, false)),
        SignalType::Signed(w) => Ok((w, true)),
        SignalType::Bool => Err(MirrError::TypeError {
            message: format!(
                "[E603] Operator '{}' requires numeric operands, got {} and {}.",
                op_symbol(op),
                ty,
                other
            ),
            span: context_span,
        }),
    }
}

/// Infer the result type of unary negation.
fn infer_negate_type(
    operand: SignalType,
    context_span: Option<Span>,
) -> Result<SignalType, MirrError> {
    match operand {
        // Negating unsigned N bits needs N+1 signed bits for two's complement.
        SignalType::Unsigned(w) => Ok(SignalType::Signed(w.saturating_add(1).min(64))),
        // Negating signed preserves width.
        SignalType::Signed(w) => Ok(SignalType::Signed(w)),
        // Negating Bool is nonsensical — use `!` instead.
        SignalType::Bool => Err(MirrError::TypeError {
            message:
                "[E609] Operator '-' (negate) cannot be applied to bool. Use '!' for logical not."
                    .to_string(),
            span: context_span,
        }),
    }
}

/// Minimum bits required to represent an unsigned value.
/// Delegates to the canonical implementation in `width::types::Width`.
fn min_bits_for(v: u64) -> u32 {
    crate::width::types::Width::min_bits_for(v).0
}

/// Type-check all property formula expressions.
///
/// Property expressions must be well-typed. The top-level formula expressions
/// should evaluate to Bool (properties are boolean assertions).
fn check_property_formulas(
    properties: &[PropertyDecl],
    signals: &HashMap<&str, SignalType>,
    all_types: &mut TypeMap,
    errors: &mut PipelineErrors,
) {
    for prop in properties {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        for expr in prop.formula.exprs() {
            // Type-check the expression (operator errors caught here).
            match infer_expr_type(expr, signals, prop.span) {
                Ok((_ty, expr_types)) => {
                    all_types.extend(expr_types);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }
    }
}
