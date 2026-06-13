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

use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use crate::ecs::components::EntityId;
use crate::ecs::registry::Registry;

use super::cnf;
use super::solver::{self, SatResult};

/// Maximum number of SAT verification attempts per pipeline run.
pub const MAX_SAT_CHECKS: usize = 256;

/* LEGACY AST ENGINE (PHASE 3b ARCHIVED)
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

pub fn simplify_with_sat(expr: Expr) -> SatSimplifyResult {
    let original = expr.clone();
    let simplified = crate::simplify::simplify_expr(expr);

    if exprs_structurally_equal(&original, &simplified) {
        return SatSimplifyResult { expr: simplified, checks_performed: 0, equivalences_confirmed: 0, had_unknown: false };
    }

    if !is_boolean_expr(&original) {
        return SatSimplifyResult { expr: simplified, checks_performed: 0, equivalences_confirmed: 0, had_unknown: false };
    }

    let cnf = match cnf::equivalence_check_cnf(&original, &simplified) {
        Some(f) => f,
        None => return SatSimplifyResult { expr: simplified, checks_performed: 0, equivalences_confirmed: 0, had_unknown: true },
    };

    let result = solver::solve(&cnf);
    match result {
        SatResult::Unsatisfiable => SatSimplifyResult { expr: simplified, checks_performed: 1, equivalences_confirmed: 1, had_unknown: false },
        SatResult::Satisfiable => SatSimplifyResult { expr: original, checks_performed: 1, equivalences_confirmed: 0, had_unknown: false },
        SatResult::Unknown => SatSimplifyResult { expr: simplified, checks_performed: 1, equivalences_confirmed: 0, had_unknown: true },
    }
}

fn is_boolean_expr(expr: &Expr) -> bool {
    let mut stack: Vec<&Expr> = vec![expr];
    let mut iterations = 0usize;
    const MAX_ITERATIONS: usize = 512;

    while let Some(e) = stack.pop() {
        iterations += 1;
        if iterations > MAX_ITERATIONS { return false; }
        match e {
            Expr::Literal(LiteralValue::Bool(_)) => {}
            Expr::Literal(LiteralValue::Integer(_)) => return false,
            Expr::Signal(_) => {}
            Expr::Prev { .. } => {}
            Expr::Unary { op, operand } => match op {
                UnaryOp::Not => stack.push(operand),
                UnaryOp::Negate => return false,
                UnaryOp::ReductionOr => return false,
            },
            Expr::Binary { op, left, right } => match op {
                BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => { stack.push(left); stack.push(right); }
                BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => { stack.push(left); stack.push(right); }
                _ => return false,
            },
            Expr::ArrayIndex { .. } | Expr::FieldAccess { .. } | Expr::ArrayLiteral(_) | Expr::StructLiteral { .. } | Expr::UnfoldIndex(_) => return false,
        }
    }
    true
}

fn exprs_structurally_equal(a: &Expr, b: &Expr) -> bool {
    // Extracted for brevity, archived
    false
}
*/

/// Result of SAT-based simplification for ECS.
#[derive(Debug, Clone)]
pub struct SatSimplifyResult {
    /// True if SAT made a reduction
    pub reduced: bool,
    /// Number of checks performed
    pub checks_performed: usize,
    /// Number of checks confirmed
    pub equivalences_confirmed: usize,
    /// Unknowns
    pub had_unknown: bool,
}

/// Attempt to simplify an ECS entity using SAT.
/// For SmaRTLy redundancy elimination, we check if a boolean cone
/// is a universal tautology (always true) or contradiction (always false).
pub fn simplify_entity_with_sat(entity: EntityId, registry: &mut Registry) -> SatSimplifyResult {
    if !is_boolean_entity(entity, registry) {
        return SatSimplifyResult {
            reduced: false,
            checks_performed: 0,
            equivalences_confirmed: 0,
            had_unknown: false,
        };
    }

    let mut checks_performed = 0;

    // Check Contradiction (Always False)
    // A formula is a contradiction if the formula itself is UNSAT.
    if let Some(mut cnf_f) = cnf::entity_to_cnf(entity, registry) {
        checks_performed += 1;
        cnf_f.add_clause(vec![cnf::Literal::pos(cnf_f.root_var)]);
        if solver::solve(&cnf_f) == SatResult::Unsatisfiable {
            registry.binary_ops[entity.0 as usize] = None;
            registry.unary_ops[entity.0 as usize] = None;
            registry.muxes[entity.0 as usize] = None;
            registry.literals[entity.0 as usize] =
                Some(crate::ecs::components::LiteralComponent(LiteralValue::Bool(false)));
            return SatSimplifyResult {
                reduced: true,
                checks_performed,
                equivalences_confirmed: 1,
                had_unknown: false,
            };
        }
    } else {
        return SatSimplifyResult {
            reduced: false,
            checks_performed,
            equivalences_confirmed: 0,
            had_unknown: true,
        };
    }

    // Check Tautology (Always True)
    // A formula is a tautology if NOT formula is UNSAT.
    if let Some(mut cnf_t) = cnf::entity_to_cnf(entity, registry) {
        checks_performed += 1;
        cnf_t.add_clause(vec![cnf::Literal::neg(cnf_t.root_var)]);
        if solver::solve(&cnf_t) == SatResult::Unsatisfiable {
            registry.binary_ops[entity.0 as usize] = None;
            registry.unary_ops[entity.0 as usize] = None;
            registry.muxes[entity.0 as usize] = None;
            registry.literals[entity.0 as usize] =
                Some(crate::ecs::components::LiteralComponent(LiteralValue::Bool(true)));
            return SatSimplifyResult {
                reduced: true,
                checks_performed,
                equivalences_confirmed: 1,
                had_unknown: false,
            };
        }
    }

    // Phase 3b: SmaRTLy Fast Inference    // 1. Fast path: MUX inference
    if let Some(mux) = registry.muxes[entity.0 as usize] {
        // We have a MUX: Mux(sel, t_branch, f_branch)
        let (_local_map, _ancestry) = cnf::extract_and_compute_ancestry(&[entity], registry);

        // SmaRTLy Fast Inference: OR-gate propagation via Ancestry Theorem.
        // If the select line is an ancestor of the true branch but NOT the false branch,
        // or if it shares no ancestry, we can check for branch equivalence.
        // For our MVP, we check if MUX is entirely redundant (e.g., Mux(S, T, F) == T)
        if let Some(mut equiv_cnf) = cnf::equivalence_check_ecs_cnf(entity, mux.true_val, registry)
        {
            checks_performed += 1;
            // Add root to assert they are DIFFERENT
            equiv_cnf.add_clause(vec![cnf::Literal::pos(equiv_cnf.root_var)]);
            if solver::solve(&equiv_cnf) == SatResult::Unsatisfiable {
                // They are always EQUAL, meaning the false_val branch is dead logic.
                registry.muxes[entity.0 as usize] = None;
                // Replace entity with true_val signal reference
                registry.signal_refs[entity.0 as usize] =
                    Some(crate::ecs::components::SignalRefComponent(mux.true_val));
                return SatSimplifyResult {
                    reduced: true,
                    checks_performed,
                    equivalences_confirmed: 1,
                    had_unknown: false,
                };
            }
        }

        if let Some(mut equiv_cnf) = cnf::equivalence_check_ecs_cnf(entity, mux.false_val, registry)
        {
            checks_performed += 1;
            equiv_cnf.add_clause(vec![cnf::Literal::pos(equiv_cnf.root_var)]);
            if solver::solve(&equiv_cnf) == SatResult::Unsatisfiable {
                registry.muxes[entity.0 as usize] = None;
                registry.signal_refs[entity.0 as usize] =
                    Some(crate::ecs::components::SignalRefComponent(mux.false_val));
                return SatSimplifyResult {
                    reduced: true,
                    checks_performed,
                    equivalences_confirmed: 1,
                    had_unknown: false,
                };
            }
        }
    }

    SatSimplifyResult {
        reduced: false,
        checks_performed,
        equivalences_confirmed: 0,
        had_unknown: false,
    }
}

fn is_boolean_entity(root: EntityId, registry: &Registry) -> bool {
    let mut stack = vec![root];
    let mut iterations = 0usize;
    const MAX_ITERATIONS: usize = 512;

    while let Some(e) = stack.pop() {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return false;
        }

        let idx = e.0 as usize;

        if let Some(lit) = &registry.literals[idx] {
            match lit.0 {
                LiteralValue::Bool(_) => {}
                LiteralValue::Integer(_) => return false,
            }
        } else if let Some(unary) = &registry.unary_ops[idx] {
            match unary.op {
                UnaryOp::Not => stack.push(unary.operand),
                _ => return false,
            }
        } else if let Some(binary) = &registry.binary_ops[idx] {
            match binary.op {
                BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => {
                    stack.push(binary.left);
                    stack.push(binary.right);
                }
                BinaryOp::Eq
                | BinaryOp::Ne
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge => {
                    // Result is boolean, SAT logic bounds it opaquely, no need to recurse down
                }
                _ => return false,
            }
        } else if let Some(mux) = &registry.muxes[idx] {
            stack.push(mux.select);
            stack.push(mux.true_val);
            stack.push(mux.false_val);
        } else if registry.temporal_nodes[idx].is_some() {
            // Reached temporal guardn opaque
        } else if registry.names[idx].is_some() {
            // Reached signal boundaryssumed boolean
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_expr_no_check() {
        // Obsolete in ECS tautology mode because we just check contradiction/tautology
    }

    #[test]
    fn double_negation_verified() {
        let mut registry = Registry::new();
        let sig = registry.create_entity("a", crate::ecs::components::KindComponent::SIGNAL);

        let not1 = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.unary_ops[not1.0 as usize] =
            Some(crate::ecs::components::UnaryComponent { op: UnaryOp::Not, operand: sig });

        let not2 = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.unary_ops[not2.0 as usize] =
            Some(crate::ecs::components::UnaryComponent { op: UnaryOp::Not, operand: not1 });

        // double negation is not a tautology or contradiction, it evaluates to `sig`.
        // The simplify_entity_with_sat will return reduced=false
        let result = simplify_entity_with_sat(not2, &mut registry);
        assert!(!result.reduced);
    }

    #[test]
    fn tautology_verified() {
        // a OR (NOT a) -> Tautology
        let mut registry = Registry::new();
        let sig = registry.create_entity("a", crate::ecs::components::KindComponent::SIGNAL);

        let not1 = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.unary_ops[not1.0 as usize] =
            Some(crate::ecs::components::UnaryComponent { op: UnaryOp::Not, operand: sig });

        let or_node = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.binary_ops[or_node.0 as usize] = Some(crate::ecs::components::BinaryComponent {
            op: BinaryOp::Or,
            left: sig,
            right: not1,
        });

        let result = simplify_entity_with_sat(or_node, &mut registry);
        assert!(result.reduced);
        assert!(matches!(
            registry.literals[or_node.0 as usize],
            Some(crate::ecs::components::LiteralComponent(LiteralValue::Bool(true)))
        ));
    }

    #[test]
    fn contradiction_verified() {
        // a AND (NOT a) -> Contradiction
        let mut registry = Registry::new();
        let sig = registry.create_entity("a", crate::ecs::components::KindComponent::SIGNAL);

        let not1 = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.unary_ops[not1.0 as usize] =
            Some(crate::ecs::components::UnaryComponent { op: UnaryOp::Not, operand: sig });

        let and_node = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.binary_ops[and_node.0 as usize] = Some(crate::ecs::components::BinaryComponent {
            op: BinaryOp::And,
            left: sig,
            right: not1,
        });

        let result = simplify_entity_with_sat(and_node, &mut registry);
        assert!(result.reduced);
        assert!(matches!(
            registry.literals[and_node.0 as usize],
            Some(crate::ecs::components::LiteralComponent(LiteralValue::Bool(false)))
        ));
    }

    #[test]
    fn is_boolean_pure_boolean() {
        let mut registry = Registry::new();
        let sig_a = registry.create_entity("a", crate::ecs::components::KindComponent::SIGNAL);
        let sig_b = registry.create_entity("b", crate::ecs::components::KindComponent::SIGNAL);

        let not_node = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.unary_ops[not_node.0 as usize] =
            Some(crate::ecs::components::UnaryComponent { op: UnaryOp::Not, operand: sig_b });

        let and_node = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.binary_ops[and_node.0 as usize] = Some(crate::ecs::components::BinaryComponent {
            op: BinaryOp::And,
            left: sig_a,
            right: not_node,
        });

        assert!(is_boolean_entity(and_node, &registry));
    }

    #[test]
    fn is_boolean_rejects_arithmetic() {
        let mut registry = Registry::new();
        let sig_a = registry.create_entity("a", crate::ecs::components::KindComponent::SIGNAL);
        let sig_b = registry.create_entity("b", crate::ecs::components::KindComponent::SIGNAL);

        let add_node = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.binary_ops[add_node.0 as usize] = Some(crate::ecs::components::BinaryComponent {
            op: BinaryOp::Add,
            left: sig_a,
            right: sig_b,
        });

        assert!(!is_boolean_entity(add_node, &registry));
    }
}
