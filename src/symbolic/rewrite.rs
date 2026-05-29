// ---------------------------------------------------------------------------
//! Term rewriting engine for runtime logic optimization.
//!
//! Dynamically simplifies monitoring expressions as runtime knowledge
//! (interval/concrete ranges in SymState) narrows signal values.
//! Reuses SmaRTLy algebraic rules (via `crate::simplify::simplify_expr`)
//! in a symbolic, interval-aware context. All operations are bounded
//! to satisfy NASA Power-of-10 rules.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::collections::HashMap;

use super::{sym_eval_expr, SymState, SymValue};
use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, LiteralValue, SignalType, UnaryOp};
use crate::simplify::simplify_expr;

/// Maximum rewriting fixpoint iterations to prevent infinite loops.
pub const MAX_REWRITE_PASSES: usize = 16;

/// Term Rewriting Engine context.
pub struct RewriteEngine {
    /// Declared types of module signals.
    pub signal_types: HashMap<String, SignalType>,
}

impl RewriteEngine {
    /// Create a new rewrite engine from a list of signal declarations.
    pub fn new(signals: &[crate::ast::program::SignalDecl]) -> Self {
        let mut signal_types = HashMap::new();
        let limit = signals.len().min(super::MAX_SYM_SIGNALS);
        for sig in signals.iter().take(limit) {
            signal_types.insert(sig.name.clone(), sig.ty.signal_type());
        }
        Self { signal_types }
    }

    /// Rewrites a single node in the AST using SymState.
    ///
    /// If the entire node evaluates to a concrete value, it is immediately folded.
    /// Otherwise, we check if it is a signal with a known concrete value.
    fn rewrite_node(&self, expr: &Expr, state: &SymState) -> Expr {
        // 1. If it is a signal, substitute with its concrete value if known.
        if let Expr::Signal(name) = expr {
            if let SymValue::Concrete(v) = state.lookup(name) {
                if let Some(SignalType::Bool) = self.signal_types.get(name) {
                    return Expr::Literal(LiteralValue::Bool(v != 0));
                } else {
                    return Expr::Literal(LiteralValue::Integer(v));
                }
            }
        }

        // 2. Check if the entire subexpression evaluates to a concrete value.
        let val = sym_eval_expr(expr, state);
        if let SymValue::Concrete(v) = val {
            if is_boolean_producing(expr) {
                return Expr::Literal(LiteralValue::Bool(v != 0));
            } else {
                return Expr::Literal(LiteralValue::Integer(v));
            }
        }

        expr.clone()
    }

    /// Rewrites an expression bottom-up iteratively (no recursion).
    ///
    /// Bounded by expression depth constraints.
    pub fn rewrite_expr_single_pass(&self, expr: Expr, state: &SymState) -> Expr {
        enum Work {
            Descend(Expr),
            CombineUnary(UnaryOp),
            CombineBinary(BinaryOp),
            CombineArrayIndex,
            CombineFieldAccess(String),
            CombineArrayLiteral(usize),
            CombineStructLiteral { name: String, field_names: Vec<String>, count: usize },
        }

        let mut work: Vec<Work> = Vec::with_capacity(super::MAX_SYM_DEPTH);
        let mut results: Vec<Expr> = Vec::with_capacity(super::MAX_SYM_DEPTH);

        work.push(Work::Descend(expr));

        let max_iters = super::MAX_SYM_DEPTH * 4;
        let mut iter_count = 0;

        while let Some(item) = work.pop() {
            iter_count += 1;
            if iter_count > max_iters {
                break;
            }

            match item {
                Work::Descend(e) => {
                    // Try to rewrite/fold the node first
                    let folded = self.rewrite_node(&e, state);
                    if folded != e {
                        // If it successfully folded/rewrote to a literal, push it directly
                        results.push(folded);
                    } else {
                        // Otherwise, descend into children
                        match e {
                            Expr::Literal(_)
                            | Expr::Signal(_)
                            | Expr::Prev { .. }
                            | Expr::UnfoldIndex(_) => {
                                results.push(e);
                            }
                            Expr::Unary { op, operand } => {
                                work.push(Work::CombineUnary(op));
                                work.push(Work::Descend(*operand));
                            }
                            Expr::Binary { op, left, right } => {
                                work.push(Work::CombineBinary(op));
                                work.push(Work::Descend(*right));
                                work.push(Work::Descend(*left));
                            }
                            Expr::ArrayIndex { array, index } => {
                                work.push(Work::CombineArrayIndex);
                                work.push(Work::Descend(*index));
                                work.push(Work::Descend(*array));
                            }
                            Expr::FieldAccess { object, field } => {
                                work.push(Work::CombineFieldAccess(field));
                                work.push(Work::Descend(*object));
                            }
                            Expr::ArrayLiteral(elems) => {
                                let count = elems.len().min(super::MAX_SYM_DEPTH);
                                work.push(Work::CombineArrayLiteral(count));
                                for el in elems.into_iter().take(count).rev() {
                                    work.push(Work::Descend(el));
                                }
                            }
                            Expr::StructLiteral { name, fields } => {
                                let count = fields.len().min(super::MAX_SYM_DEPTH);
                                let fnames: Vec<String> =
                                    fields.iter().take(count).map(|(n, _)| n.clone()).collect();
                                work.push(Work::CombineStructLiteral {
                                    name,
                                    field_names: fnames,
                                    count,
                                });
                                for (_, val) in fields.into_iter().take(count).rev() {
                                    work.push(Work::Descend(val));
                                }
                            }
                        }
                    }
                }
                Work::CombineUnary(op) => {
                    let operand = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                    let node = Expr::Unary { op, operand: Box::new(operand) };
                    // Apply algebraic simplifier
                    results.push(simplify_expr(node));
                }
                Work::CombineBinary(op) => {
                    let right = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                    let left = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                    let node = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
                    // Apply algebraic simplifier
                    results.push(simplify_expr(node));
                }
                Work::CombineArrayIndex => {
                    let index = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                    let array = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                    results
                        .push(Expr::ArrayIndex { array: Box::new(array), index: Box::new(index) });
                }
                Work::CombineFieldAccess(field) => {
                    let object = results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)));
                    results.push(Expr::FieldAccess { object: Box::new(object), field });
                }
                Work::CombineArrayLiteral(count) => {
                    let mut elems = Vec::with_capacity(count);
                    for _ in 0..count {
                        elems.push(
                            results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false))),
                        );
                    }
                    elems.reverse();
                    results.push(Expr::ArrayLiteral(elems));
                }
                Work::CombineStructLiteral { name, field_names, count } => {
                    let mut vals = Vec::with_capacity(count);
                    for _ in 0..count {
                        vals.push(
                            results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false))),
                        );
                    }
                    vals.reverse();
                    let fields: Vec<(String, Expr)> = field_names.into_iter().zip(vals).collect();
                    results.push(Expr::StructLiteral { name, fields });
                }
            }
        }

        results.pop().unwrap_or(Expr::Literal(LiteralValue::Bool(false)))
    }

    /// Rewrite an expression, running to a fixpoint.
    ///
    /// Applies the rewriter and SmaRTLy simplifier repeatedly up to
    /// `MAX_REWRITE_PASSES` times or until the expression stops changing.
    pub fn rewrite_expr(&self, expr: Expr, state: &SymState) -> Expr {
        let mut current = expr;
        for _pass in 0..MAX_REWRITE_PASSES {
            let next = self.rewrite_expr_single_pass(current.clone(), state);
            if next == current {
                break; // Fixpoint reached.
            }
            current = next;
        }
        current
    }
}

/// Returns true if the expression produces a boolean result type.
fn is_boolean_producing(expr: &Expr) -> bool {
    match expr {
        Expr::Literal(LiteralValue::Bool(_)) => true,
        Expr::Binary { op, .. } => matches!(
            op,
            BinaryOp::And
                | BinaryOp::Or
                | BinaryOp::Lt
                | BinaryOp::Le
                | BinaryOp::Gt
                | BinaryOp::Ge
                | BinaryOp::Eq
                | BinaryOp::Ne
        ),
        Expr::Unary { op, .. } => matches!(op, UnaryOp::Not),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::SignalDecl;
    use crate::ast::types::ExtendedType;

    fn make_decl(name: &str, ty: SignalType) -> SignalDecl {
        SignalDecl {
            name: name.to_string(),
            kind: crate::ast::types::SignalKind::Internal,
            ty: ExtendedType::from_core(ty),
            origin: None,
            span: None,
        }
    }

    #[test]
    fn test_rewrite_signal_substitution() {
        let signals =
            vec![make_decl("x", SignalType::Unsigned(8)), make_decl("y", SignalType::Bool)];
        let engine = RewriteEngine::new(&signals);

        let mut state = SymState::new();
        state.signals.push(("x".to_string(), SymValue::Concrete(42)));
        state.signals.push(("y".to_string(), SymValue::Concrete(1)));

        let expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Signal("x".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(8))),
        };

        // x + 8 -> 42 + 8 -> 50
        let rewritten = engine.rewrite_expr(expr, &state);
        assert_eq!(rewritten, Expr::Literal(LiteralValue::Integer(50)));

        let expr_bool = Expr::Signal("y".to_string());
        let rewritten_bool = engine.rewrite_expr(expr_bool, &state);
        assert_eq!(rewritten_bool, Expr::Literal(LiteralValue::Bool(true)));
    }

    #[test]
    fn test_rewrite_logic_simplification() {
        let signals = vec![make_decl("x", SignalType::Unsigned(8))];
        let engine = RewriteEngine::new(&signals);

        // x is in [10, 20]
        let mut state = SymState::new();
        state.signals.push(("x".to_string(), SymValue::Interval { lo: 10, hi: 20 }));

        // (x > 5) && y  ->  true && y  ->  y
        let expr = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("x".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
            }),
            right: Box::new(Expr::Signal("y".to_string())),
        };

        let rewritten = engine.rewrite_expr(expr, &state);
        assert_eq!(rewritten, Expr::Signal("y".to_string()));
    }
}
