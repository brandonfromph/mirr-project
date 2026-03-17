//! Logic simplification for MIRR expressions.
//!
//! Implements algebraic simplification rules for boolean, comparison, and
//! arithmetic expressions using a bounded iterative traversal (NASA Power-of-10
//! rule #1 compliant — no unbounded recursion).
//!
//! The simplifier runs to fixpoint (bounded by MAX_PASSES) to catch cascading
//! reductions like `(a && true) || false => a || false => a`.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use serde::Serialize;

/// Maximum expression tree depth the simplifier will traverse.
/// Matches the parser's MAX_EXPR_DEPTH to guarantee we never exceed stack budget.
const MAX_SIMPLIFY_DEPTH: usize = 128;

/// Maximum number of fixpoint passes before stopping.
/// Each pass reduces the tree; 8 passes handles any realistic cascade chain.
const MAX_PASSES: usize = 8;

// ---------------------------------------------------------------------------
// Statistics
// ---------------------------------------------------------------------------

/// Statistics from a simplification run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SimplifyStats {
    /// Total number of rule applications across all passes.
    pub rules_applied: usize,
    /// Number of AST nodes before simplification.
    pub nodes_before: usize,
    /// Number of AST nodes after simplification.
    pub nodes_after: usize,
}

// ---------------------------------------------------------------------------
// Node counting
// ---------------------------------------------------------------------------

/// Count AST nodes iteratively (bounded).
fn count_nodes(expr: &Expr) -> usize {
    let mut stack: Vec<&Expr> = Vec::with_capacity(MAX_SIMPLIFY_DEPTH);
    stack.push(expr);
    let mut count = 0usize;
    while let Some(e) = stack.pop() {
        if count >= MAX_SIMPLIFY_DEPTH * 4 {
            break; // Bound iteration for pathological trees.
        }
        count += 1;
        match e {
            Expr::Literal(_) | Expr::Signal(_) | Expr::Prev { .. } => {}
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            Expr::ArrayIndex { array, index } => {
                stack.push(array);
                stack.push(index);
            }
            Expr::FieldAccess { object, .. } => {
                stack.push(object);
            }
            Expr::ArrayLiteral(elems) => {
                let mut j = 0;
                while j < elems.len() && j < MAX_SIMPLIFY_DEPTH {
                    stack.push(&elems[j]);
                    j += 1;
                }
            }
            Expr::StructLiteral { fields, .. } => {
                let mut j = 0;
                while j < fields.len() && j < MAX_SIMPLIFY_DEPTH {
                    stack.push(&fields[j].1);
                    j += 1;
                }
            }
        }
    }
    count
}

// ---------------------------------------------------------------------------
// Core single-node rewrite rules
// ---------------------------------------------------------------------------

/// Apply algebraic rules to a single Unary node whose operand has already been
/// simplified. Returns `(simplified_expr, did_fire)`.
fn simplify_unary(op: UnaryOp, operand: Expr) -> (Expr, bool) {
    match (op, &operand) {
        // !! X  =>  X   (double negation elimination)
        (UnaryOp::Not, Expr::Unary { op: UnaryOp::Not, operand: inner }) => (*inner.clone(), true),
        // !true => false,  !false => true   (literal negation)
        (UnaryOp::Not, Expr::Literal(LiteralValue::Bool(b))) => {
            (Expr::Literal(LiteralValue::Bool(!b)), true)
        }
        _ => (Expr::Unary { op, operand: Box::new(operand) }, false),
    }
}

/// Apply algebraic rules to a single Binary node whose children have already
/// been simplified. Returns `(simplified_expr, did_fire)`.
fn simplify_binary(op: BinaryOp, left: Expr, right: Expr) -> (Expr, bool) {
    use BinaryOp::*;
    use LiteralValue::*;

    // Helper: check if two expressions are structurally equal.
    let eq = |a: &Expr, b: &Expr| a == b;

    // Helper: check if `b` is `!a`.
    let is_negation_of = |a: &Expr, b: &Expr| -> bool {
        matches!(b, Expr::Unary { op: UnaryOp::Not, operand } if operand.as_ref() == a)
    };

    match (op, &left, &right) {
        // =================================================================
        // Boolean identity / annihilation
        // =================================================================

        // X && true => X,  true && X => X
        (And, _, Expr::Literal(Bool(true))) => (left, true),
        (And, Expr::Literal(Bool(true)), _) => (right, true),
        // X && false => false
        (And, _, Expr::Literal(Bool(false))) | (And, Expr::Literal(Bool(false)), _) => {
            (Expr::Literal(Bool(false)), true)
        }

        // X || false => X,  false || X => X
        (Or, _, Expr::Literal(Bool(false))) => (left, true),
        (Or, Expr::Literal(Bool(false)), _) => (right, true),
        // X || true => true
        (Or, _, Expr::Literal(Bool(true))) | (Or, Expr::Literal(Bool(true)), _) => {
            (Expr::Literal(Bool(true)), true)
        }

        // X ^ false => X,  false ^ X => X
        (Xor, _, Expr::Literal(Bool(false))) => (left, true),
        (Xor, Expr::Literal(Bool(false)), _) => (right, true),
        // X ^ true => !X,  true ^ X => !X
        (Xor, x, Expr::Literal(Bool(true))) | (Xor, Expr::Literal(Bool(true)), x) => {
            (Expr::Unary { op: UnaryOp::Not, operand: Box::new(x.clone()) }, true)
        }

        // =================================================================
        // Boolean idempotence / absorption
        // =================================================================

        // a && a => a
        (And, l, r) if eq(l, r) => (left, true),
        // a || a => a
        (Or, l, r) if eq(l, r) => (left, true),
        // a ^ a => false
        (Xor, l, r) if eq(l, r) => (Expr::Literal(Bool(false)), true),
        // a && !a => false,  !a && a => false
        (And, a, b) if is_negation_of(a, b) || is_negation_of(b, a) => {
            (Expr::Literal(Bool(false)), true)
        }
        // a || !a => true,  !a || a => true
        (Or, a, b) if is_negation_of(a, b) || is_negation_of(b, a) => {
            (Expr::Literal(Bool(true)), true)
        }

        // =================================================================
        // Comparison constant folding  (both sides Integer literals)
        // =================================================================
        (Lt, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Bool(*a < *b)), true)
        }
        (Le, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Bool(*a <= *b)), true)
        }
        (Gt, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Bool(*a > *b)), true)
        }
        (Ge, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Bool(*a >= *b)), true)
        }
        (Eq, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Bool(*a == *b)), true)
        }
        (Ne, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Bool(*a != *b)), true)
        }

        // =================================================================
        // Arithmetic identity / annihilation
        // =================================================================

        // x + 0 => x,  0 + x => x
        (Add, _, Expr::Literal(Integer(0))) => (left, true),
        (Add, Expr::Literal(Integer(0)), _) => (right, true),
        // x - 0 => x
        (Sub, _, Expr::Literal(Integer(0))) => (left, true),
        // x * 1 => x,  1 * x => x
        (Mul, _, Expr::Literal(Integer(1))) => (left, true),
        (Mul, Expr::Literal(Integer(1)), _) => (right, true),
        // x * 0 => 0,  0 * x => 0
        (Mul, _, Expr::Literal(Integer(0))) | (Mul, Expr::Literal(Integer(0)), _) => {
            (Expr::Literal(Integer(0)), true)
        }
        // x << 0 => x,  x >> 0 => x
        (Shl, _, Expr::Literal(Integer(0))) => (left, true),
        (Shr, _, Expr::Literal(Integer(0))) => (left, true),

        // =================================================================
        // Arithmetic constant folding  (both sides Integer literals)
        // Wrapping semantics match eval_expr in mirr_executor.rs.
        // Shift amounts clamped to 63 matching CRIT-01 fix.
        // =================================================================
        (Add, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Integer(a.wrapping_add(*b))), true)
        }
        (Sub, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Integer(a.wrapping_sub(*b))), true)
        }
        (Mul, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            (Expr::Literal(Integer(a.wrapping_mul(*b))), true)
        }
        (Shl, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            let amt = (*b).min(63);
            (Expr::Literal(Integer(a << amt)), true)
        }
        (Shr, Expr::Literal(Integer(a)), Expr::Literal(Integer(b))) => {
            let amt = (*b).min(63);
            (Expr::Literal(Integer(a >> amt)), true)
        }

        // =================================================================
        // No rule matched — reconstruct unchanged.
        // =================================================================
        _ => (Expr::Binary { op, left: Box::new(left), right: Box::new(right) }, false),
    }
}

// ---------------------------------------------------------------------------
// Iterative bottom-up simplification (single pass)
// ---------------------------------------------------------------------------

/// Work items for iterative post-order traversal.
enum WorkItem {
    /// Push this expression's children, then schedule a Combine.
    Descend(Expr),
    /// Pop children from result stack, apply rules, push result.
    CombineUnary(UnaryOp),
    CombineBinary(BinaryOp),
    /// Reassemble composite expressions from simplified children.
    CombineArrayIndex,
    CombineFieldAccess(String),
    CombineArrayLiteral(usize),
    CombineStructLiteral {
        name: String,
        field_names: Vec<String>,
        count: usize,
    },
}

/// One bottom-up simplification pass. Returns `(simplified, rules_fired)`.
fn simplify_one_pass(expr: Expr) -> (Expr, usize) {
    let mut work: Vec<WorkItem> = Vec::with_capacity(MAX_SIMPLIFY_DEPTH);
    let mut results: Vec<Expr> = Vec::with_capacity(MAX_SIMPLIFY_DEPTH);
    let mut rules_fired: usize = 0;

    work.push(WorkItem::Descend(expr));

    // Bounded iteration: each Descend produces at most 3 work items (2 children
    // + 1 Combine), so total items <= 3 * MAX_SIMPLIFY_DEPTH.
    let max_iterations = MAX_SIMPLIFY_DEPTH * 4;
    let mut iterations = 0;

    while let Some(item) = work.pop() {
        iterations += 1;
        if iterations > max_iterations {
            // Depth exceeded — return whatever we have so far.
            break;
        }
        match item {
            WorkItem::Descend(e) => match e {
                Expr::Literal(_) | Expr::Signal(_) | Expr::Prev { .. } => {
                    results.push(e);
                }
                Expr::Unary { op, operand } => {
                    work.push(WorkItem::CombineUnary(op));
                    work.push(WorkItem::Descend(*operand));
                }
                Expr::Binary { op, left, right } => {
                    work.push(WorkItem::CombineBinary(op));
                    // Push right first so left is processed first (stack is LIFO).
                    work.push(WorkItem::Descend(*right));
                    work.push(WorkItem::Descend(*left));
                }
                // Composite data variants — descend into sub-expressions.
                Expr::ArrayIndex { array, index } => {
                    work.push(WorkItem::CombineArrayIndex);
                    work.push(WorkItem::Descend(*index));
                    work.push(WorkItem::Descend(*array));
                }
                Expr::FieldAccess { object, field } => {
                    work.push(WorkItem::CombineFieldAccess(field));
                    work.push(WorkItem::Descend(*object));
                }
                Expr::ArrayLiteral(elems) => {
                    let count = elems.len().min(MAX_SIMPLIFY_DEPTH);
                    work.push(WorkItem::CombineArrayLiteral(count));
                    let mut j = count;
                    while j > 0 {
                        j -= 1;
                        work.push(WorkItem::Descend(elems[j].clone()));
                    }
                }
                Expr::StructLiteral { name, fields } => {
                    let count = fields.len().min(MAX_SIMPLIFY_DEPTH);
                    let fnames: Vec<String> =
                        fields.iter().take(count).map(|(n, _)| n.clone()).collect();
                    work.push(WorkItem::CombineStructLiteral { name, field_names: fnames, count });
                    let mut j = count;
                    while j > 0 {
                        j -= 1;
                        work.push(WorkItem::Descend(fields[j].1.clone()));
                    }
                }
            },
            WorkItem::CombineUnary(op) => {
                let operand = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                let (result, fired) = simplify_unary(op, operand);
                if fired {
                    rules_fired += 1;
                }
                results.push(result);
            }
            WorkItem::CombineBinary(op) => {
                // Results stack has [... left, right] (left pushed first, right on top).
                let right = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                let left = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                let (result, fired) = simplify_binary(op, left, right);
                if fired {
                    rules_fired += 1;
                }
                results.push(result);
            }
            WorkItem::CombineArrayIndex => {
                let index = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                let array = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                results.push(Expr::ArrayIndex { array: Box::new(array), index: Box::new(index) });
            }
            WorkItem::CombineFieldAccess(field) => {
                let object = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                results.push(Expr::FieldAccess { object: Box::new(object), field });
            }
            WorkItem::CombineArrayLiteral(count) => {
                let mut elems = Vec::with_capacity(count);
                let mut j = 0;
                while j < count {
                    elems.push(results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false))));
                    j += 1;
                }
                elems.reverse();
                results.push(Expr::ArrayLiteral(elems));
            }
            WorkItem::CombineStructLiteral { name, field_names, count } => {
                let mut vals = Vec::with_capacity(count);
                let mut j = 0;
                while j < count {
                    vals.push(results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false))));
                    j += 1;
                }
                vals.reverse();
                let fields: Vec<(String, Expr)> = field_names.into_iter().zip(vals).collect();
                results.push(Expr::StructLiteral { name, fields });
            }
        }
    }

    let result = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
    (result, rules_fired)
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Simplify a MIRR expression tree, running to fixpoint.
///
/// Convenience wrapper that discards statistics.
/// Applies algebraic rules iteratively (no recursion) and repeats until
/// no more rules fire or MAX_PASSES is reached.
pub fn simplify_expr(expr: Expr) -> Expr {
    simplify_expr_with_stats(expr).0
}

/// Simplify a MIRR expression tree with statistics.
///
/// Returns the simplified expression and a [`SimplifyStats`] recording how
/// many rules fired and the before/after node counts.
pub fn simplify_expr_with_stats(expr: Expr) -> (Expr, SimplifyStats) {
    let nodes_before = count_nodes(&expr);
    let mut current = expr;
    let mut total_rules = 0usize;

    for _pass in 0..MAX_PASSES {
        let (simplified, rules_fired) = simplify_one_pass(current);
        total_rules += rules_fired;
        current = simplified;
        if rules_fired == 0 {
            break; // Fixpoint reached.
        }
    }

    let nodes_after = count_nodes(&current);
    let stats = SimplifyStats { rules_applied: total_rules, nodes_before, nodes_after };
    (current, stats)
}
