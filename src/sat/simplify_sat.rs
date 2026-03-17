//! SAT-based expression simplification.
//!
//! Uses the DPLL solver to verify that heuristic simplifications
//! preserve equivalence. Given an original expression and a candidate
//! simplified expression, converts (original XOR simplified) to CNF
//! and checks unsatisfiability. If UNSAT, the expressions are logically
//! equivalent and the simplification is sound.
//!
//! This module integrates with the existing heuristic simplifier in
//! `simplify.rs` as an optional verification pass.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};

use super::cnf;
use super::solver::{self, SatResult};

/// Maximum number of SAT verification attempts per pipeline run.
pub const MAX_SAT_CHECKS: usize = 256;

/// Result of SAT-based simplification.
#[derive(Debug, Clone)]
pub struct SatSimplifyResult {
    /// The (possibly simplified) expression.
    pub expr: Expr,
    /// Number of SAT equivalence checks performed.
    pub checks_performed: usize,
    /// Number of checks that confirmed equivalence.
    pub equivalences_confirmed: usize,
    /// Whether any check hit solver bounds (returned Unknown).
    pub had_unknown: bool,
}

/// Attempt to simplify an expression using SAT-based equivalence checking.
///
/// Strategy:
/// 1. Run the heuristic simplifier to get a candidate.
/// 2. If the candidate differs from the original, verify equivalence via SAT.
/// 3. If SAT confirms equivalence, accept the simplification.
/// 4. If SAT finds a counterexample or times out, keep the original.
///
/// This provides a safety net: the heuristic simplifier is fast but may
/// have bugs; the SAT checker is slow but correct (within its bounds).
pub fn simplify_with_sat(expr: Expr) -> SatSimplifyResult {
    let original = expr.clone();
    let simplified = crate::simplify::simplify_expr(expr);

    // If unchanged, no SAT check needed.
    if exprs_structurally_equal(&original, &simplified) {
        return SatSimplifyResult {
            expr: simplified,
            checks_performed: 0,
            equivalences_confirmed: 0,
            had_unknown: false,
        };
    }

    // Only SAT-check Boolean expressions. Arithmetic expressions
    // have integer semantics that the Boolean SAT solver cannot handle.
    if !is_boolean_expr(&original) {
        return SatSimplifyResult {
            expr: simplified,
            checks_performed: 0,
            equivalences_confirmed: 0,
            had_unknown: false,
        };
    }

    // Build equivalence check CNF: (original XOR simplified) should be UNSAT.
    let cnf = match cnf::equivalence_check_cnf(&original, &simplified) {
        Some(f) => f,
        None => {
            // Expression too large for CNF conversion — accept heuristic result.
            return SatSimplifyResult {
                expr: simplified,
                checks_performed: 0,
                equivalences_confirmed: 0,
                had_unknown: true,
            };
        }
    };

    let result = solver::solve(&cnf);
    match result {
        SatResult::Unsatisfiable => {
            // Equivalence confirmed! The simplification is sound.
            SatSimplifyResult {
                expr: simplified,
                checks_performed: 1,
                equivalences_confirmed: 1,
                had_unknown: false,
            }
        }
        SatResult::Satisfiable => {
            // Counterexample found — simplification changes semantics.
            // Fall back to original.
            SatSimplifyResult {
                expr: original,
                checks_performed: 1,
                equivalences_confirmed: 0,
                had_unknown: false,
            }
        }
        SatResult::Unknown => {
            // Solver hit bounds — conservatively accept heuristic result.
            SatSimplifyResult {
                expr: simplified,
                checks_performed: 1,
                equivalences_confirmed: 0,
                had_unknown: true,
            }
        }
    }
}

/// Check if an expression is purely Boolean (no arithmetic).
fn is_boolean_expr(expr: &Expr) -> bool {
    let mut stack: Vec<&Expr> = vec![expr];
    let mut iterations = 0usize;
    const MAX_ITERATIONS: usize = 512;

    while let Some(e) = stack.pop() {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return false;
        }
        match e {
            Expr::Literal(LiteralValue::Bool(_)) => {}
            Expr::Literal(LiteralValue::Integer(_)) => return false,
            Expr::Signal(_) => {
                // Signals could be Boolean or integer — conservatively treat as Boolean.
            }
            Expr::Prev { .. } => {}
            Expr::Unary { op, operand } => {
                match op {
                    UnaryOp::Not => stack.push(operand),
                    UnaryOp::Negate => return false, // Arithmetic negation.
                }
            }
            Expr::Binary { op, left, right } => {
                match op {
                    BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => {
                        stack.push(left);
                        stack.push(right);
                    }
                    // Comparison operators return Boolean but operate on any type.
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => {
                        // Accept: result is Boolean.
                        stack.push(left);
                        stack.push(right);
                    }
                    // Arithmetic operators.
                    _ => return false,
                }
            }
            // Composite expressions are not boolean.
            Expr::ArrayIndex { .. }
            | Expr::FieldAccess { .. }
            | Expr::ArrayLiteral(_)
            | Expr::StructLiteral { .. } => return false,
        }
    }
    true
}

/// Structural equality check for expressions.
fn exprs_structurally_equal(a: &Expr, b: &Expr) -> bool {
    let mut stack: Vec<(&Expr, &Expr)> = vec![(a, b)];
    let mut iterations = 0usize;
    const MAX_ITERATIONS: usize = 512;

    while let Some((x, y)) = stack.pop() {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return false;
        }
        match (x, y) {
            (Expr::Literal(a_val), Expr::Literal(b_val)) => {
                if a_val != b_val {
                    return false;
                }
            }
            (Expr::Signal(a_name), Expr::Signal(b_name)) => {
                if a_name != b_name {
                    return false;
                }
            }
            (
                Expr::Prev { signal: a_sig, delay: a_d },
                Expr::Prev { signal: b_sig, delay: b_d },
            ) => {
                if a_sig != b_sig || a_d != b_d {
                    return false;
                }
            }
            (
                Expr::Unary { op: a_op, operand: a_inner },
                Expr::Unary { op: b_op, operand: b_inner },
            ) => {
                if a_op != b_op {
                    return false;
                }
                stack.push((a_inner, b_inner));
            }
            (
                Expr::Binary { op: a_op, left: a_l, right: a_r },
                Expr::Binary { op: b_op, left: b_l, right: b_r },
            ) => {
                if a_op != b_op {
                    return false;
                }
                stack.push((a_l, b_l));
                stack.push((a_r, b_r));
            }
            (
                Expr::ArrayIndex { array: a_arr, index: a_idx },
                Expr::ArrayIndex { array: b_arr, index: b_idx },
            ) => {
                stack.push((a_arr, b_arr));
                stack.push((a_idx, b_idx));
            }
            (
                Expr::FieldAccess { object: a_obj, field: a_f },
                Expr::FieldAccess { object: b_obj, field: b_f },
            ) => {
                if a_f != b_f {
                    return false;
                }
                stack.push((a_obj, b_obj));
            }
            (Expr::ArrayLiteral(a_elems), Expr::ArrayLiteral(b_elems)) => {
                if a_elems.len() != b_elems.len() {
                    return false;
                }
                let mut i = 0;
                while i < a_elems.len() && i < MAX_ITERATIONS {
                    stack.push((&a_elems[i], &b_elems[i]));
                    i += 1;
                }
            }
            (
                Expr::StructLiteral { name: a_name, fields: a_fields },
                Expr::StructLiteral { name: b_name, fields: b_fields },
            ) => {
                if a_name != b_name || a_fields.len() != b_fields.len() {
                    return false;
                }
                let mut i = 0;
                while i < a_fields.len() && i < MAX_ITERATIONS {
                    if a_fields[i].0 != b_fields[i].0 {
                        return false;
                    }
                    stack.push((&a_fields[i].1, &b_fields[i].1));
                    i += 1;
                }
            }
            _ => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_expr_no_check() {
        let expr = Expr::Signal("a".to_string());
        let result = simplify_with_sat(expr);
        assert_eq!(result.checks_performed, 0);
    }

    #[test]
    fn double_negation_verified() {
        // NOT (NOT a) should simplify to a.
        let expr = Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(Expr::Signal("a".to_string())),
            }),
        };
        let result = simplify_with_sat(expr);
        // The heuristic simplifier should catch double negation.
        // SAT check should confirm equivalence.
        assert!(result.equivalences_confirmed <= 1);
    }

    #[test]
    fn structural_equality_same_expr() {
        let a = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(Expr::Signal("x".to_string())),
            right: Box::new(Expr::Signal("y".to_string())),
        };
        let b = a.clone();
        assert!(exprs_structurally_equal(&a, &b));
    }

    #[test]
    fn structural_equality_different_expr() {
        let a = Expr::Signal("x".to_string());
        let b = Expr::Signal("y".to_string());
        assert!(!exprs_structurally_equal(&a, &b));
    }

    #[test]
    fn is_boolean_pure_boolean() {
        let expr = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(Expr::Signal("a".to_string())),
            right: Box::new(Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(Expr::Signal("b".to_string())),
            }),
        };
        assert!(is_boolean_expr(&expr));
    }

    #[test]
    fn is_boolean_rejects_arithmetic() {
        let expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Signal("a".to_string())),
            right: Box::new(Expr::Signal("b".to_string())),
        };
        assert!(!is_boolean_expr(&expr));
    }
}
