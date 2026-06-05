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
//!
//! ## MEGA-1 Extended Type System
//!
//! The `extended` submodule adds refinement types, linear types, effect types,
//! clock domain qualifiers, phantom types, type-level naturals, dependent types,
//! and session types. See `typeck::extended` for details. Error codes: E610–E625.

#![forbid(unsafe_code)]

pub mod extended;

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
///
/// # Note on Pointer Identity
/// The key `*const Expr` relies on the fact that expression nodes are not
/// cloned during the type checking pass, maintaining stable memory addresses.
pub type TypeMap = HashMap<*const Expr, SignalType>;

/// The type checking mode.
///
/// Determines the strictness of type rules applied to expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypecheckMode {
    /// Standard MIRR strict typechecking (strict type checking of bool/unsigned).
    #[default]
    Standard,
    /// Bootstrap/hydration mode (allows bool in arithmetic and logical ops on unsigned,
    /// used during initial IR loading or interop with raw gate-level netlists).
    Bootstrap,
}

/// Operator display name for error messages.
fn op_symbol(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
        BinaryOp::BitwiseOr => "|",
        BinaryOp::BitwiseAnd => "&",
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

fn expr_node_budget_error(context_span: Option<Span>) -> MirrError {
    MirrError::TypeError {
        message: format!("{} Expression type inference exceeded maximum expression node count (MAX_EXPR_NODES={}).", crate::error_codes::ec(607),
            MAX_EXPR_NODES
        ),
        span: context_span,
    }
}

fn expr_inference_incomplete_error(context_span: Option<Span>) -> MirrError {
    MirrError::TypeError {
        message: format!(
            "{} Expression type inference did not produce a root type within MAX_EXPR_NODES={}.",
            crate::error_codes::ec(607),
            MAX_EXPR_NODES
        ),
        span: context_span,
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
    typecheck_module_with_mode(module, TypecheckMode::Standard)
}

/// Run Stage 2 type checking on an AST module with an explicit `TypecheckMode`.
pub fn typecheck_module_with_mode(
    module: &Module,
    mode: TypecheckMode,
) -> Result<TypeMap, PipelineErrors> {
    // Build signal type lookup table.
    let mut signals: HashMap<&str, SignalType> = HashMap::with_capacity(module.signals.len());
    for sig in &module.signals {
        signals.insert(&sig.name, sig.ty.signal_type());
    }
    // All guards are implicitly boolean signals.
    for guard in &module.guards {
        signals.insert(&guard.name, SignalType::Bool);
    }
    // The 'always' guard is a built-in boolean sentinel.
    signals.insert("always", SignalType::Bool);

    let mut all_types: TypeMap = HashMap::new();
    let mut errors = PipelineErrors::new();

    // T14: Guard conditions must be Bool.
    for guard in &module.guards {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        match infer_expr_type(&guard.condition, &signals, guard.span, mode) {
            Ok((cond_ty, expr_types)) => {
                all_types.extend(expr_types);
                if cond_ty != SignalType::Bool {
                    errors.push(MirrError::TypeError {
                        message: format!(
                            "{} Guard '{}' condition must be bool, got {}.",
                            crate::error_codes::ec(601),
                            guard.name,
                            cond_ty
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
                Some(ty) => ty.clone(),
                None => continue, // Undeclared target — caught by semantic validation.
            };
            match infer_expr_type(&assignment.value, &signals, assignment.span, mode) {
                Ok((expr_ty, expr_types)) => {
                    all_types.extend(expr_types);
                    if !types_compatible(&target_ty, &expr_ty) {
                        let code = crate::error_codes::ec(601); // TypeMismatch
                        errors.push(MirrError::TypeError {
                            message: format!(
                                "{} Assignment to '{}' ({}): expression type {} is not compatible.",
                                code, assignment.target, target_ty, expr_ty
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
    check_property_formulas(&module.properties, &signals, &mut all_types, &mut errors, mode);

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
fn types_compatible(target: &SignalType, expr: &SignalType) -> bool {
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
/// This function performs type inference in two phases to satisfy the NASA P10 constraint
/// of avoiding recursion in tree traversals:
///
/// 1. **Flattening**: Performs a post-order traversal to build an iterative work list (`order`)
///    of all expression nodes.
/// 2. **Evaluation**: Iterates through the work list in reverse order (bottom-up),
///    evaluating the type of each expression node based on its children's already-computed
///    types, which are stored in the `types` map.
///
/// # Arguments
/// * `expr` - The root expression to infer.
/// * `signals` - A map from signal/guard names to their declared types.
/// * `context_span` - The span of the construct containing this expression (for error reporting).
/// * `mode` - The `TypecheckMode` (e.g., Strict or Bootstrap).
///
/// # Returns
/// - `Ok((SignalType, TypeMap))` - The inferred type of the root expression and a map of
///   all inferred node types for downstream passes.
/// - `Err(MirrError)` - If type mismatch or structural errors are detected.
///
/// Bounded: at most `MAX_EXPR_NODES` iterations (enforced iteratively).
fn infer_expr_type(
    expr: &Expr,
    signals: &HashMap<&str, SignalType>,
    context_span: Option<Span>,
    mode: TypecheckMode,
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
            return Err(expr_node_budget_error(context_span));
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
            Expr::ArrayIndex { array, index } => {
                work.push(array);
                work.push(index);
            }
            Expr::FieldAccess { object, .. } => {
                work.push(object);
            }
            Expr::ArrayLiteral(elems) => {
                if elems.len() > MAX_EXPR_NODES {
                    return Err(expr_node_budget_error(context_span));
                }
                let mut i = 0;
                while i < elems.len() {
                    work.push(&elems[i]);
                    i += 1;
                }
            }
            Expr::StructLiteral { fields, .. } => {
                if fields.len() > MAX_EXPR_NODES {
                    return Err(expr_node_budget_error(context_span));
                }
                let mut i = 0;
                while i < fields.len() {
                    work.push(&fields[i].1);
                    i += 1;
                }
            }
            Expr::UnfoldIndex(_) => {
                // Meta-stage artifact is not a typed expression in this pass.
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
                Some(ty) => ty.clone(),
                None => continue, // Undeclared — caught by semantic validation.
            },
            // T13: Prev preserves signal type.
            Expr::Prev { signal, .. } => match signals.get(signal.as_str()) {
                Some(ty) => ty.clone(),
                None => continue, // Undeclared — caught by semantic validation.
            },
            // Unary operators.
            Expr::Unary { op, operand, .. } => {
                let operand_ptr = operand.as_ref() as *const Expr;
                let operand_ty = match types.get(&operand_ptr) {
                    Some(ty) => ty,
                    None => continue,
                };
                match op {
                    // T11/T12: Not works on Bool, Unsigned, and Signed.
                    UnaryOp::Not => {
                        if operand_ty.is_composite() {
                            return Err(MirrError::TypeError {
                                message: format!(
                                    "{} Operator '!' cannot be applied to composite type '{}'.",
                                    crate::error_codes::ec(226),
                                    operand_ty
                                ),
                                span: context_span,
                            });
                        }
                        operand_ty.clone()
                    }
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
                    Some(ty) => ty,
                    None => continue,
                };
                let right_ty = match types.get(&right_ptr) {
                    Some(ty) => ty,
                    None => continue,
                };
                infer_binary_type(*op, left_ty, right_ty, context_span, mode)?
            }
            Expr::ArrayIndex { array, index } => {
                let array_ptr = array.as_ref() as *const Expr;
                let index_ptr = index.as_ref() as *const Expr;
                let array_ty = match types.get(&array_ptr) {
                    Some(ty) => ty,
                    None => continue,
                };
                let index_ty = match types.get(&index_ptr) {
                    Some(ty) => ty,
                    None => continue,
                };
                infer_array_index_type(array_ty, index_ty, context_span)?
            }
            Expr::FieldAccess { object, field } => {
                let object_ptr = object.as_ref() as *const Expr;
                let object_ty = match types.get(&object_ptr) {
                    Some(ty) => ty,
                    None => continue,
                };
                infer_field_access_type(object_ty, field, context_span)?
            }
            Expr::ArrayLiteral(elems) => {
                if elems.is_empty() {
                    SignalType::Array { element: Box::new(SignalType::Unsigned(1)), length: 0 }
                } else {
                    let first_ptr = &elems[0] as *const Expr;
                    let mut element_ty = match types.get(&first_ptr) {
                        Some(ty) => ty.clone(),
                        None => continue,
                    };

                    let mut i = 1usize;
                    while i < elems.len().min(MAX_EXPR_NODES) {
                        let elem_ptr = &elems[i] as *const Expr;
                        let elem_ty = match types.get(&elem_ptr) {
                            Some(ty) => ty,
                            None => {
                                i += 1;
                                continue;
                            }
                        };
                        element_ty = merge_array_element_types(&element_ty, elem_ty, context_span)?;
                        i += 1;
                    }

                    SignalType::Array { element: Box::new(element_ty), length: elems.len() as u64 }
                }
            }
            Expr::StructLiteral { name, fields } => {
                let mut typed_fields: Vec<(String, SignalType)> =
                    Vec::with_capacity(fields.len().min(MAX_EXPR_NODES));
                let mut i = 0usize;
                while i < fields.len().min(MAX_EXPR_NODES) {
                    let (field_name, field_expr) = &fields[i];
                    let field_ptr = field_expr as *const Expr;
                    let field_ty = match types.get(&field_ptr) {
                        Some(ty) => ty.clone(),
                        None => {
                            i += 1;
                            continue;
                        }
                    };
                    typed_fields.push((field_name.clone(), field_ty));
                    i += 1;
                }
                SignalType::Struct { name: name.clone(), fields: typed_fields }
            }
            Expr::UnfoldIndex(_) => SignalType::Unsigned(32),
        };
        types.insert(ptr, ty);
    }

    // The root expression's type.
    let root_ptr = expr as *const Expr;
    match types.get(&root_ptr) {
        Some(ty) => Ok((ty.clone(), types)),
        None => Err(expr_inference_incomplete_error(context_span)),
    }
}

/// Infer the result type of a binary operation, or reject with a type error.
fn infer_binary_type(
    op: BinaryOp,
    left: &SignalType,
    right: &SignalType,
    context_span: Option<Span>,
    mode: TypecheckMode,
) -> Result<SignalType, MirrError> {
    match op {
        // T2/T3: Arithmetic operators require numeric operands.
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
            let (left_w, left_signed) = require_numeric(op, left, right, context_span, mode)?;
            let (right_w, right_signed) = require_numeric(op, right, left, context_span, mode)?;
            // Cross-category: reject mixed signed/unsigned arithmetic.
            if left_signed != right_signed {
                return Err(MirrError::TypeError {
                    message: format!(
                        "{} Operator '{}' cannot mix signed and unsigned operands: {} and {}.",
                        crate::error_codes::ec(608),
                        op_symbol(op),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
            let max_w = left_w.max(right_w);
            if left_signed {
                Ok(SignalType::Signed(max_w))
            } else {
                Ok(SignalType::Unsigned(max_w))
            }
        }

        // T4: Bitwise shifts. Left must be numeric (or bool-as-u1), right must be numeric.
        BinaryOp::Shl | BinaryOp::Shr => {
            let (left_w, left_signed) = match left {
                SignalType::Bool => (1, false),
                _ => require_numeric(op, left, right, context_span, mode)?,
            };
            let (_right_w, _right_signed) = match right {
                SignalType::Bool => (1, false),
                _ => require_numeric(op, right, left, context_span, mode)?,
            };

            if left_signed {
                Ok(SignalType::Signed(left_w))
            } else {
                Ok(SignalType::Unsigned(left_w))
            }
        }

        // T8/T9: Logical/Bitwise AND/OR.
        BinaryOp::And | BinaryOp::Or => {
            if left == &SignalType::Bool && right == &SignalType::Bool {
                Ok(SignalType::Bool)
            } else {
                if mode != TypecheckMode::Bootstrap {
                    return Err(MirrError::TypeError {
                        message: format!(
                            "{} Logical operator '{}' requires bool operands, got {} and {}.",
                            crate::error_codes::ec(604),
                            op_symbol(op),
                            left,
                            right
                        ),
                        span: context_span,
                    });
                }

                // Support bitwise AND/OR for numeric types, including bool-as-u1.
                let (left_w, left_signed) = match left {
                    SignalType::Bool => (1, false),
                    _ => require_numeric(op, left, right, context_span, mode)?,
                };
                let (right_w, right_signed) = match right {
                    SignalType::Bool => (1, false),
                    _ => require_numeric(op, right, left, context_span, mode)?,
                };
                if left_signed != right_signed {
                    return Err(MirrError::TypeError {
                        message: format!(
                            "{} Operator '{}' cannot mix signed and unsigned operands: {} and {}.",
                            crate::error_codes::ec(608),
                            op_symbol(op),
                            left,
                            right
                        ),
                        span: context_span,
                    });
                }
                let max_w = left_w.max(right_w);
                if left_signed {
                    Ok(SignalType::Signed(max_w))
                } else {
                    Ok(SignalType::Unsigned(max_w))
                }
            }
        }

        // Hardware bitwise integer operators: accept any numeric widths, return max-width.
        // These are explicit RTL operators (`|` and `&`) distinct from logical And/Or.
        BinaryOp::BitwiseOr | BinaryOp::BitwiseAnd => {
            let (left_w, left_signed) = match left {
                SignalType::Bool => (1, false),
                _ => require_numeric(op, left, right, context_span, mode)?,
            };
            let (right_w, right_signed) = match right {
                SignalType::Bool => (1, false),
                _ => require_numeric(op, right, left, context_span, mode)?,
            };
            if left_signed != right_signed {
                return Err(MirrError::TypeError {
                    message: format!(
                        "{} Operator '{}' cannot mix signed and unsigned operands: {} and {}.",
                        crate::error_codes::ec(608),
                        op_symbol(op),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
            let max_w = left_w.max(right_w);
            if left_signed {
                Ok(SignalType::Signed(max_w))
            } else {
                Ok(SignalType::Unsigned(max_w))
            }
        }

        // T10: XOR requires matching types. Reject composites.
        BinaryOp::Xor => {
            if left.is_composite() || right.is_composite() {
                return Err(MirrError::TypeError {
                    message: format!(
                        "{} Operator '^' cannot be applied to composite type '{}' and '{}'.",
                        crate::error_codes::ec(226),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
            if left != right {
                // Allow Bool ↔ Unsigned(1) for xor.
                if !types_compatible(left, right) {
                    return Err(MirrError::TypeError {
                        message: format!(
                            "{} Operator '^' (xor) requires matching types, got {} and {}.",
                            crate::error_codes::ec(607),
                            left,
                            right
                        ),
                        span: context_span,
                    });
                }
            }
            Ok(left.clone())
        }

        // T5/T7: Ordering comparisons.
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
            // Reject composites for ordering.
            if left.is_composite() || right.is_composite() {
                return Err(MirrError::TypeError {
                    message: format!(
                        "{} Ordering operator '{}' cannot compare composite types '{}' and '{}'.",
                        crate::error_codes::ec(226),
                        op_symbol(op),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
            // T7: Ordering on Bool is an error.
            if left == &SignalType::Bool || right == &SignalType::Bool {
                return Err(MirrError::TypeError {
                    message: format!(
                        "{} Ordering operator '{}' cannot compare {} and {}.",
                        crate::error_codes::ec(605),
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
                    message: format!("{} Ordering operator '{}' cannot compare {} and {} (signed/unsigned mismatch).", crate::error_codes::ec(605),
                        op_symbol(op), left, right
                    ),
                    span: context_span,
                });
            }
            Ok(SignalType::Bool)
        }

        // T6: Equality comparisons.
        BinaryOp::Eq | BinaryOp::Ne => {
            // Reject composites for equality.
            if left.is_composite() || right.is_composite() {
                return Err(MirrError::TypeError {
                    message: format!(
                        "{} Equality operator '{}' cannot compare composite types '{}' and '{}'.",
                        crate::error_codes::ec(226),
                        op_symbol(op),
                        left,
                        right
                    ),
                    span: context_span,
                });
            }
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
                        "{} Equality operator '{}' cannot compare {} and {}.",
                        crate::error_codes::ec(606),
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
    ty: &SignalType,
    other: &SignalType,
    context_span: Option<Span>,
    mode: TypecheckMode,
) -> Result<(u32, bool), MirrError> {
    match ty {
        SignalType::Unsigned(w) => Ok((*w, false)),
        SignalType::Signed(w) => Ok((*w, true)),
        SignalType::Bool => {
            if mode == TypecheckMode::Bootstrap {
                Ok((1, false))
            } else {
                Err(MirrError::TypeError {
                    message: format!(
                        "{} Operator '{}' requires numeric operands, got {} and {}.",
                        crate::error_codes::ec(603),
                        op_symbol(op),
                        ty,
                        other
                    ),
                    span: context_span,
                })
            }
        }
        SignalType::Array { .. }
        | SignalType::Struct { .. }
        | SignalType::FixedPoint { .. }
        | SignalType::Bundle(_)
        | SignalType::Fifo { .. } => Err(MirrError::TypeError {
            message: format!(
                "{} Operator '{}' cannot be applied to composite type '{}'.",
                crate::error_codes::ec(226),
                op_symbol(op),
                ty
            ),
            span: context_span,
        }),
    }
}

/// Infer the result type of unary negation.
fn infer_negate_type(
    operand: &SignalType,
    context_span: Option<Span>,
) -> Result<SignalType, MirrError> {
    match operand {
        // Negating unsigned N bits needs N+1 signed bits for two's complement.
        SignalType::Unsigned(w) => Ok(SignalType::Signed(w.saturating_add(1).min(64))),
        // Negating signed preserves width.
        SignalType::Signed(w) => Ok(SignalType::Signed(*w)),
        // Negating Bool is nonsensical — use `!` instead.
        SignalType::Bool => Err(MirrError::TypeError {
            message: format!(
                "{} Operator '-' (negate) cannot be applied to bool. Use '!' for logical not.",
                crate::error_codes::ec(609)
            )
            .to_string(),
            span: context_span,
        }),
        SignalType::Array { .. }
        | SignalType::Struct { .. }
        | SignalType::FixedPoint { .. }
        | SignalType::Bundle(_)
        | SignalType::Fifo { .. } => Err(MirrError::TypeError {
            message: format!(
                "{} Operator '-' (negate) cannot be applied to composite type '{}'.",
                crate::error_codes::ec(226),
                operand
            ),
            span: context_span,
        }),
    }
}

fn infer_array_index_type(
    array_ty: &SignalType,
    index_ty: &SignalType,
    context_span: Option<Span>,
) -> Result<SignalType, MirrError> {
    match index_ty {
        SignalType::Unsigned(_) | SignalType::Signed(_) => {}
        _ => {
            return Err(MirrError::TypeError {
                message: format!(
                    "{} Array index must be numeric, got {}.",
                    crate::error_codes::ec(603),
                    index_ty
                ),
                span: context_span,
            });
        }
    }

    match array_ty {
        SignalType::Array { element, .. } => Ok(element.as_ref().clone()),
        SignalType::Unsigned(_) | SignalType::Signed(_) => Ok(SignalType::Bool),
        _ => Err(MirrError::TypeError {
            message: format!(
                "{} Indexing requires an indexable type (array, unsigned, or signed), got {}.",
                crate::error_codes::ec(607),
                array_ty
            ),
            span: context_span,
        }),
    }
}

fn infer_field_access_type(
    object_ty: &SignalType,
    field_name: &str,
    context_span: Option<Span>,
) -> Result<SignalType, MirrError> {
    match object_ty {
        SignalType::Struct { fields, .. } => {
            match fields.iter().find(|(name, _)| name == field_name) {
                Some((_, ty)) => Ok(ty.clone()),
                None => Err(MirrError::TypeError {
                    message: format!(
                        "{} Struct field '{}' does not exist on type {}.",
                        crate::error_codes::ec(607),
                        field_name,
                        object_ty
                    ),
                    span: context_span,
                }),
            }
        }
        _ => Err(MirrError::TypeError {
            message: format!(
                "{} Field access requires a struct operand, got {}.",
                crate::error_codes::ec(607),
                object_ty
            ),
            span: context_span,
        }),
    }
}

fn merge_array_element_types(
    current: &SignalType,
    next: &SignalType,
    context_span: Option<Span>,
) -> Result<SignalType, MirrError> {
    if current == next {
        return Ok(current.clone());
    }

    match (current, next) {
        (SignalType::Unsigned(a), SignalType::Unsigned(b)) => {
            Ok(SignalType::Unsigned((*a).max(*b)))
        }
        (SignalType::Signed(a), SignalType::Signed(b)) => Ok(SignalType::Signed((*a).max(*b))),
        (SignalType::Bool, SignalType::Unsigned(w))
        | (SignalType::Unsigned(w), SignalType::Bool) => Ok(SignalType::Unsigned((*w).max(1))),
        _ => Err(MirrError::TypeError {
            message: format!(
                "{} Array literal elements must have compatible types, got {} and {}.",
                crate::error_codes::ec(607),
                current,
                next
            ),
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
    mode: TypecheckMode,
) {
    for prop in properties {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        for expr in prop.formula.exprs() {
            // Type-check the expression (operator errors caught here).
            match infer_expr_type(expr, signals, prop.span, mode) {
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
