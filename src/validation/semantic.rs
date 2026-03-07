// ---------------------------------------------------------------------------
// Semantic validation
// ---------------------------------------------------------------------------
// Single responsibility: validate a parsed module for semantic correctness.
// No parsing logic lives here — only post-parse analysis.
// ---------------------------------------------------------------------------

use std::collections::HashSet;

use crate::ast::expr::Expr;
use crate::ast::program::Module;
use crate::ast::types::SignalKind;
use crate::error::MirrError;

/// Validate a parsed module for semantic correctness:
///
/// - No duplicate signal names.
/// - No duplicate guard names.
/// - No duplicate reflex names.
/// - Guard conditions only reference declared signals.
/// - Reflex `on` clauses only reference declared guards.
/// - Assignment targets are declared output or internal signals.
/// - Assignment expressions only reference declared signals.
///
/// NASA-style optimization: pre-allocate hash sets and use efficient lookups.
pub fn validate_module(module: &Module) -> Result<(), MirrError> {
    // Pre-allocate hash sets with estimated capacity for better performance.
    let signal_capacity = module.signals.len();
    let guard_capacity = module.guards.len();
    let reflex_capacity = module.reflexes.len();

    // Collect signal names and check for duplicates.
    let mut signal_names: HashSet<&str> = HashSet::with_capacity(signal_capacity);
    for sig in &module.signals {
        if !signal_names.insert(&sig.name) {
            return Err(MirrError::SemanticError {
                message: format!("Duplicate signal name: '{}'.", sig.name),
            });
        }
    }

    // Collect guard names and check for duplicates.
    let mut guard_names: HashSet<&str> = HashSet::with_capacity(guard_capacity);
    for guard in &module.guards {
        if !guard_names.insert(&guard.name) {
            return Err(MirrError::SemanticError {
                message: format!("Duplicate guard name: '{}'.", guard.name),
            });
        }
    }

    // Collect reflex names and check for duplicates.
    let mut reflex_names: HashSet<&str> = HashSet::with_capacity(reflex_capacity);
    for reflex in &module.reflexes {
        if !reflex_names.insert(&reflex.name) {
            return Err(MirrError::SemanticError {
                message: format!("Duplicate reflex name: '{}'.", reflex.name),
            });
        }
    }

    // Build set of output/internal signals (valid assignment targets).
    // NASA-style optimization: reserve capacity and filter efficiently.
    let writable_capacity = module
        .signals
        .iter()
        .filter(|s| s.kind == SignalKind::Output || s.kind == SignalKind::Internal)
        .count();
    let writable_signals: HashSet<&str> = {
        let mut set = HashSet::with_capacity(writable_capacity);
        for s in &module.signals {
            if s.kind == SignalKind::Output || s.kind == SignalKind::Internal {
                set.insert(s.name.as_str());
            }
        }
        set
    };

    // Validate guard conditions reference declared signals.
    // NASA-style optimization: batch validation and early exit.
    for guard in &module.guards {
        validate_prev_delays(&guard.condition, &guard.name)?;
        let refs = collect_signal_refs(&guard.condition);
        for sig_ref in &refs {
            if !signal_names.contains(sig_ref.as_str()) {
                return Err(MirrError::SemanticError {
                    message: format!(
                        "Guard '{}' references undeclared signal '{}'.",
                        guard.name, sig_ref
                    ),
                });
            }
        }
    }

    // Validate reflexes with optimized lookups.
    for reflex in &module.reflexes {
        // Check guard references.
        for gname in &reflex.guard_names {
            if !guard_names.contains(gname.as_str()) {
                return Err(MirrError::SemanticError {
                    message: format!(
                        "Reflex '{}' references undeclared guard '{}'.",
                        reflex.name, gname
                    ),
                });
            }
        }

        // Check assignments.
        for assignment in &reflex.assignments {
            // Target must be a writable signal.
            if !writable_signals.contains(assignment.target.as_str()) {
                if signal_names.contains(assignment.target.as_str()) {
                    return Err(MirrError::SemanticError {
                        message: format!(
                            "Reflex '{}' assigns to input signal '{}', which is not writable.",
                            reflex.name, assignment.target
                        ),
                    });
                }
                return Err(MirrError::SemanticError {
                    message: format!(
                        "Reflex '{}' assigns to undeclared signal '{}'.",
                        reflex.name, assignment.target
                    ),
                });
            }

            // Validate Prev delays in RHS expressions.
            validate_prev_delays(&assignment.value, &reflex.name)?;

            // RHS expression signals must be declared.
            let refs = collect_signal_refs(&assignment.value);
            for sig_ref in &refs {
                if !signal_names.contains(sig_ref.as_str()) {
                    return Err(MirrError::SemanticError {
                        message: format!(
                            "Reflex '{}' assignment references undeclared signal '{}'.",
                            reflex.name, sig_ref
                        ),
                    });
                }
            }
        }
    }

    Ok(())
}

/// Validate that all `Prev` nodes in an expression have delay >= 1.
/// Uses an explicit stack to avoid recursion.
/// Bounded: at most MAX_EXPR_NODES iterations.
fn validate_prev_delays(expr: &Expr, context_name: &str) -> Result<(), MirrError> {
    const MAX_EXPR_NODES: usize = 512;
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        match node {
            Expr::Prev { signal, delay } => {
                if *delay == 0 {
                    return Err(MirrError::SemanticError {
                        message: format!(
                            "'{}' contains prev('{}') with delay 0; delay must be >= 1.",
                            context_name, signal
                        ),
                    });
                }
            }
            Expr::Literal(_) | Expr::Signal(_) => {}
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
        }
    }
    Ok(())
}

/// Collect all signal references from an expression tree.
/// Uses an explicit stack to avoid recursion.
/// NASA-style optimization: pre-allocate vector and minimize allocations.
pub fn collect_signal_refs(expr: &Expr) -> Vec<String> {
    // Pre-allocate with reasonable capacity estimate.
    // In practice, expressions rarely have more than 10-20 signal references.
    let mut refs = Vec::with_capacity(16);
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);

    while let Some(node) = stack.pop() {
        match node {
            Expr::Signal(name) => refs.push(name.clone()),
            Expr::Prev { signal, .. } => refs.push(signal.clone()),
            Expr::Literal(_) => {}
            Expr::Unary { operand, .. } => {
                stack.push(operand);
            }
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
        }
    }

    refs
}
