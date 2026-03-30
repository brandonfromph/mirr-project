//! Internal signal scoping validation after pattern expansion.

#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::ast::expr::Expr;
use crate::ast::program::Module;
use crate::ast::property::PropertyFormula;
use crate::ast::types::SignalKind;
use crate::ast::MAX_EXPR_NODES;
use crate::error::MirrError;
use crate::span::Span;

pub(super) fn validate_internal_signal_scoping(module: &Module) -> Result<(), MirrError> {
    // Collect all pattern-internal signal names and their origin.
    let mut internal_signals: HashMap<&str, (&str, Option<Span>)> = HashMap::with_capacity(16);
    for sig in &module.signals {
        if sig.kind == SignalKind::Internal {
            if let Some(ref origin) = sig.origin {
                internal_signals.insert(&sig.name, (origin, sig.span));
            }
        }
    }

    if internal_signals.is_empty() {
        return Ok(());
    }

    // Check hand-written guards (origin == None).
    for guard in &module.guards {
        if guard.origin.is_none() {
            check_expr_no_internal_refs(&guard.condition, &internal_signals)?;
        }
    }

    // Check hand-written reflexes (origin == None).
    for reflex in &module.reflexes {
        if reflex.origin.is_none() {
            for assignment in &reflex.assignments {
                if let Some((origin, sig_span)) = internal_signals.get(assignment.target.as_str()) {
                    return Err(MirrError::SemanticError {
                        message: format!(
                            "[E212] signal '{}' is internal to pattern '{}' \
                             and cannot be referenced externally",
                            assignment.target, origin
                        ),
                        span: *sig_span,
                    });
                }
                check_expr_no_internal_refs(&assignment.value, &internal_signals)?;
            }
        }
    }

    // Check hand-written properties (origin == None).
    for prop in &module.properties {
        if prop.origin.is_none() {
            check_property_no_internal_refs(&prop.formula, &internal_signals)?;
        }
    }

    // Check cross-expansion references: a pattern expansion referencing
    // an internal signal from a DIFFERENT expansion.
    for guard in &module.guards {
        if let Some(ref guard_origin) = guard.origin {
            check_expr_cross_expansion(&guard.condition, guard_origin, &internal_signals)?;
        }
    }
    for reflex in &module.reflexes {
        if let Some(ref reflex_origin) = reflex.origin {
            for assignment in &reflex.assignments {
                // Check target.
                if let Some((sig_origin, sig_span)) =
                    internal_signals.get(assignment.target.as_str())
                {
                    if *sig_origin != reflex_origin.as_str() {
                        return Err(MirrError::SemanticError {
                            message: format!(
                                "[E214] signal '{}' is internal to pattern '{}' \
                                 and cannot be referenced externally",
                                assignment.target, sig_origin
                            ),
                            span: *sig_span,
                        });
                    }
                }
                check_expr_cross_expansion(&assignment.value, reflex_origin, &internal_signals)?;
            }
        }
    }

    Ok(())
}

/// Check that an expression does not reference any pattern-internal signals.
///
/// Uses explicit work stack — zero recursion.
pub(super) fn check_expr_no_internal_refs(
    expr: &Expr,
    internal_signals: &HashMap<&str, (&str, Option<Span>)>,
) -> Result<(), MirrError> {
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        let name = match node {
            Expr::Signal(n) => Some(n.as_str()),
            Expr::Prev { signal, .. } => Some(signal.as_str()),
            Expr::UnfoldIndex(name) => Some(name.as_str()),
            Expr::Literal(_) => None,
            Expr::Unary { operand, .. } => {
                stack.push(operand);
                None
            }
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
                None
            }
            Expr::ArrayIndex { array, index } => {
                stack.push(array);
                stack.push(index);
                None
            }
            Expr::FieldAccess { object, .. } => {
                stack.push(object);
                None
            }
            Expr::ArrayLiteral(elems) => {
                let mut j = 0;
                while j < elems.len() && j < MAX_EXPR_NODES {
                    stack.push(&elems[j]);
                    j += 1;
                }
                None
            }
            Expr::StructLiteral { fields, .. } => {
                let mut j = 0;
                while j < fields.len() && j < MAX_EXPR_NODES {
                    stack.push(&fields[j].1);
                    j += 1;
                }
                None
            }
        };
        if let Some(sig_name) = name {
            if let Some((origin, sig_span)) = internal_signals.get(sig_name) {
                return Err(MirrError::SemanticError {
                    message: format!(
                        "[E213] signal '{}' is internal to pattern '{}' \
                         and cannot be referenced externally",
                        sig_name, origin
                    ),
                    span: *sig_span,
                });
            }
        }
    }
    Ok(())
}

/// Check that an expression from one expansion doesn't reference internal
/// signals from a different expansion.
pub(super) fn check_expr_cross_expansion(
    expr: &Expr,
    my_origin: &str,
    internal_signals: &HashMap<&str, (&str, Option<Span>)>,
) -> Result<(), MirrError> {
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        let name = match node {
            Expr::Signal(n) => Some(n.as_str()),
            Expr::Prev { signal, .. } => Some(signal.as_str()),
            Expr::UnfoldIndex(name) => Some(name.as_str()),
            Expr::Literal(_) => None,
            Expr::Unary { operand, .. } => {
                stack.push(operand);
                None
            }
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
                None
            }
            Expr::ArrayIndex { array, index } => {
                stack.push(array);
                stack.push(index);
                None
            }
            Expr::FieldAccess { object, .. } => {
                stack.push(object);
                None
            }
            Expr::ArrayLiteral(elems) => {
                let mut j = 0;
                while j < elems.len() && j < MAX_EXPR_NODES {
                    stack.push(&elems[j]);
                    j += 1;
                }
                None
            }
            Expr::StructLiteral { fields, .. } => {
                let mut j = 0;
                while j < fields.len() && j < MAX_EXPR_NODES {
                    stack.push(&fields[j].1);
                    j += 1;
                }
                None
            }
        };
        if let Some(sig_name) = name {
            if let Some((sig_origin, sig_span)) = internal_signals.get(sig_name) {
                if *sig_origin != my_origin {
                    return Err(MirrError::SemanticError {
                        message: format!(
                            "[E215] signal '{}' is internal to pattern '{}' \
                             and cannot be referenced externally",
                            sig_name, sig_origin
                        ),
                        span: *sig_span,
                    });
                }
            }
        }
    }
    Ok(())
}

/// Check that a property formula does not reference internal signals.
pub(super) fn check_property_no_internal_refs(
    formula: &PropertyFormula,
    internal_signals: &HashMap<&str, (&str, Option<Span>)>,
) -> Result<(), MirrError> {
    for expr in formula.exprs() {
        check_expr_no_internal_refs(expr, internal_signals)?;
    }
    Ok(())
}
