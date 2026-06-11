#![forbid(unsafe_code)]

//! Symbolic differentiation for MIRR expressions.
//!
//! Computes d(expr)/d(signal) using an iterative post-order traversal
//! with an explicit work stack.  No recursion.  All loops bounded by
//! `MAX_DIFF_*` constants (NASA Power-of-10).

use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};

// ── NASA Power-of-10 bounds ────────────────────────────────────────────────

/// Maximum expression depth for differentiation traversal.
pub const MAX_DIFF_DEPTH: usize = 32;

/// Maximum work-stack iterations before the engine bails out.
const MAX_DIFF_ITERS: usize = 8192;

// ── Helpers ────────────────────────────────────────────────────────────────

/// Build the constant expression `0`.
fn zero() -> Expr {
    Expr::Literal(LiteralValue::Integer(0))
}

/// Build the constant expression `1`.
fn one() -> Expr {
    Expr::Literal(LiteralValue::Integer(1))
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Compute the symbolic derivative of `expr` with respect to `signal_name`.
///
/// Returns a new `Expr` representing d(expr)/d(signal\_name).
/// Uses iterative post-order traversal (no recursion, bounded by
/// `MAX_DIFF_DEPTH` and `MAX_DIFF_ITERS`).
///
/// # Differentiation rules
///
/// | Expression         | Derivative                              |
/// |--------------------|-----------------------------------------|
/// | constant           | 0                                       |
/// | x (same signal)    | 1                                       |
/// | y (other signal)   | 0                                       |
/// | a + b              | da/dx + db/dx  (sum rule)               |
/// | a - b              | da/dx - db/dx  (difference rule)        |
/// | a * b              | a*(db/dx) + (da/dx)*b  (product rule)   |
/// | a << k             | (da/dx) << k  (concrete shift amount)   |
/// | a >> k             | (da/dx) >> k  (concrete shift amount)   |
/// | -a                 | -(da/dx)                                |
/// | !a, And, Or, Xor   | 0  (bitwise, not differentiable)        |
/// | Lt, Le, Gt, Ge, Eq, Ne | 0  (comparison, not differentiable) |
/// | Prev               | 0  (temporal, not spatial derivative)   |
pub fn sym_diff(expr: &Expr, signal_name: &str) -> Expr {
    // Work items for the explicit differentiation stack.
    enum Work<'a> {
        /// Differentiate `expr` at traversal `depth`.
        Diff(&'a Expr, usize),
        /// Pop two results, build `Binary { op, left, right }`.
        BuildBinary(BinaryOp),
        /// Pop one result, build `Unary { op, operand }`.
        BuildUnary(UnaryOp),
        /// Push a pre-built expression onto the result stack.
        PushExpr(Expr),
    }

    let mut work: Vec<Work<'_>> = Vec::with_capacity(MAX_DIFF_DEPTH * 8);
    let mut results: Vec<Expr> = Vec::with_capacity(MAX_DIFF_DEPTH * 4);

    work.push(Work::Diff(expr, 0));

    let mut iter_count: usize = 0;

    while let Some(item) = work.pop() {
        iter_count += 1;
        if iter_count > MAX_DIFF_ITERS {
            return zero();
        }

        match item {
            Work::Diff(e, depth) => {
                // Depth exceeded: treat as zero to bound traversal.
                if depth > MAX_DIFF_DEPTH {
                    results.push(zero());
                    continue;
                }

                match e {
                    Expr::Literal(_) => {
                        results.push(zero());
                    }
                    Expr::Signal(name) => {
                        if name == signal_name {
                            results.push(one());
                        } else {
                            results.push(zero());
                        }
                    }
                    Expr::Prev { .. } => {
                        results.push(zero());
                    }
                    Expr::Unary { op, operand } => match op {
                        UnaryOp::Not => {
                            // Bitwise NOT is not differentiable.
                            results.push(zero());
                        }
                        UnaryOp::Negate => {
                            // d(-a)/dx = -(da/dx)
                            work.push(Work::BuildUnary(UnaryOp::Negate));
                            work.push(Work::Diff(operand, depth + 1));
                        }
                        UnaryOp::ReductionOr => {
                            // Reduction OR is not differentiable.
                            results.push(zero());
                        }
                    },
                    Expr::UnfoldIndex(_) => {
                        // Unresolved meta-stage index is treated as constant for symbolic diff.
                        results.push(zero());
                    }
                    Expr::Binary { op, left, right } => match op {
                        BinaryOp::Add => {
                            // d(a + b)/dx = da/dx + db/dx
                            work.push(Work::BuildBinary(BinaryOp::Add));
                            work.push(Work::Diff(right, depth + 1));
                            work.push(Work::Diff(left, depth + 1));
                        }
                        BinaryOp::Sub => {
                            // d(a - b)/dx = da/dx - db/dx
                            work.push(Work::BuildBinary(BinaryOp::Sub));
                            work.push(Work::Diff(right, depth + 1));
                            work.push(Work::Diff(left, depth + 1));
                        }
                        BinaryOp::Mul => {
                            // d(a*b)/dx = a*(db/dx) + (da/dx)*b
                            //
                            // Stack schedule (LIFO, bottom to top):
                            //   PushExpr(a)  -> Diff(b)  -> BuildMul  (= a * db)
                            //   Diff(a)      -> PushExpr(b) -> BuildMul (= da * b)
                            //   BuildAdd                    (= a*db + da*b)
                            work.push(Work::BuildBinary(BinaryOp::Add));
                            // Second term: (da/dx) * b
                            work.push(Work::BuildBinary(BinaryOp::Mul));
                            work.push(Work::PushExpr((**right).clone()));
                            work.push(Work::Diff(left, depth + 1));
                            // First term: a * (db/dx)
                            work.push(Work::BuildBinary(BinaryOp::Mul));
                            work.push(Work::Diff(right, depth + 1));
                            work.push(Work::PushExpr((**left).clone()));
                        }
                        BinaryOp::Shl => {
                            // d(a << k)/dx = (da/dx) << k
                            work.push(Work::BuildBinary(BinaryOp::Shl));
                            work.push(Work::PushExpr((**right).clone()));
                            work.push(Work::Diff(left, depth + 1));
                        }
                        BinaryOp::Shr => {
                            // d(a >> k)/dx = (da/dx) >> k
                            work.push(Work::BuildBinary(BinaryOp::Shr));
                            work.push(Work::PushExpr((**right).clone()));
                            work.push(Work::Diff(left, depth + 1));
                        }
                        // Bitwise and comparison ops: not differentiable.
                        BinaryOp::And
                        | BinaryOp::Or
                        | BinaryOp::BitwiseOr
                        | BinaryOp::BitwiseAnd
                        | BinaryOp::Xor
                        | BinaryOp::Lt
                        | BinaryOp::Le
                        | BinaryOp::Gt
                        | BinaryOp::Ge
                        | BinaryOp::Eq
                        | BinaryOp::Ne => {
                            results.push(zero());
                        }
                    },
                    // Composite expressions: not differentiable.
                    Expr::ArrayIndex { .. }
                    | Expr::FieldAccess { .. }
                    | Expr::ArrayLiteral(_)
                    | Expr::StructLiteral { .. } => {
                        results.push(zero());
                    }
                }
            }
            Work::BuildBinary(op) => {
                let rhs = results.pop().unwrap_or_else(zero);
                let lhs = results.pop().unwrap_or_else(zero);
                results.push(Expr::Binary { op, left: Box::new(lhs), right: Box::new(rhs) });
            }
            Work::BuildUnary(op) => {
                let operand = results.pop().unwrap_or_else(zero);
                results.push(Expr::Unary { op, operand: Box::new(operand) });
            }
            Work::PushExpr(e) => {
                results.push(e);
            }
        }
    }

    results.pop().unwrap_or_else(zero)
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: build `Signal(name)`.
    fn sig(name: &str) -> Expr {
        Expr::Signal(name.to_string())
    }

    /// Helper: build `Literal(Integer(n))`.
    fn lit(n: u64) -> Expr {
        Expr::Literal(LiteralValue::Integer(n))
    }

    /// Helper: build `Binary { op, left, right }`.
    fn bin(op: BinaryOp, l: Expr, r: Expr) -> Expr {
        Expr::Binary { op, left: Box::new(l), right: Box::new(r) }
    }

    #[test]
    fn diff_constant() {
        // d(5)/dx = 0
        assert_eq!(sym_diff(&lit(5), "x"), zero());
    }

    #[test]
    fn diff_same_signal() {
        // d(x)/dx = 1
        assert_eq!(sym_diff(&sig("x"), "x"), one());
    }

    #[test]
    fn diff_other_signal() {
        // d(y)/dx = 0
        assert_eq!(sym_diff(&sig("y"), "x"), zero());
    }

    #[test]
    fn diff_sum_with_constant() {
        // d(x + 1)/dx = 1 + 0
        let expr = bin(BinaryOp::Add, sig("x"), lit(1));
        let expected = bin(BinaryOp::Add, one(), zero());
        assert_eq!(sym_diff(&expr, "x"), expected);
    }

    #[test]
    fn diff_sum_signals() {
        // d(x + y)/dx = 1 + 0
        let expr = bin(BinaryOp::Add, sig("x"), sig("y"));
        let expected = bin(BinaryOp::Add, one(), zero());
        assert_eq!(sym_diff(&expr, "x"), expected);
    }

    #[test]
    fn diff_product_same() {
        // d(x * x)/dx = x*1 + 1*x
        let expr = bin(BinaryOp::Mul, sig("x"), sig("x"));
        let expected = bin(
            BinaryOp::Add,
            bin(BinaryOp::Mul, sig("x"), one()),
            bin(BinaryOp::Mul, one(), sig("x")),
        );
        assert_eq!(sym_diff(&expr, "x"), expected);
    }

    #[test]
    fn diff_product_with_constant() {
        // d(x * 5)/dx = x*0 + 1*5
        let expr = bin(BinaryOp::Mul, sig("x"), lit(5));
        let expected = bin(
            BinaryOp::Add,
            bin(BinaryOp::Mul, sig("x"), zero()),
            bin(BinaryOp::Mul, one(), lit(5)),
        );
        assert_eq!(sym_diff(&expr, "x"), expected);
    }

    #[test]
    fn diff_not() {
        // d(!x)/dx = 0
        let expr = Expr::Unary { op: UnaryOp::Not, operand: Box::new(sig("x")) };
        assert_eq!(sym_diff(&expr, "x"), zero());
    }

    #[test]
    fn diff_comparison() {
        // d(x < 5)/dx = 0
        let expr = bin(BinaryOp::Lt, sig("x"), lit(5));
        assert_eq!(sym_diff(&expr, "x"), zero());
    }

    #[test]
    fn diff_prev() {
        // d(prev(x, 1))/dx = 0
        let expr = Expr::Prev { signal: "x".to_string(), delay: 1 };
        assert_eq!(sym_diff(&expr, "x"), zero());
    }
}
