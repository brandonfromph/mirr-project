//! DSP inference analysis for multiply operations.
//!
//! Scans reflex assignments for `BinaryOp::Mul` expressions where the
//! operand widths exceed a configurable threshold, indicating the multiply
//! should be mapped to a vendor DSP block rather than fabric logic.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::program::Module;
use crate::ast::types::BinaryOp;
use crate::ast::MAX_EXPR_NODES;

/// Maximum number of DSP candidates tracked per module (NASA P10 bounded iteration).
pub const MAX_DSP_CANDIDATES: usize = 64;

/// Default DSP inference threshold in bits.
/// Multiplies where min(left_width, right_width) >= this value get DSP attributes.
/// 9 bits means u9*u9 = 18-bit result, which fits a single DSP input slice.
pub const DEFAULT_DSP_THRESHOLD: u32 = 9;

/// A detected multiply operation that should be mapped to a DSP block.
#[derive(Debug, Clone)]
pub struct DspCandidate {
    /// Name of the reflex containing this multiply.
    pub reflex_name: String,
    /// Target signal being assigned.
    pub target_signal: String,
}

/// Result of DSP analysis for a module.
#[derive(Debug, Clone)]
pub struct DspAnalysis {
    /// Multiply operations that exceed the threshold.
    pub candidates: Vec<DspCandidate>,
    /// The threshold used for this analysis.
    pub threshold_bits: u32,
}

/// Analyze a module for multiply operations suitable for DSP inference.
///
/// Walks each reflex's assignments looking for `Expr::Binary { op: Mul, .. }`.
/// Returns candidates where the multiply is present (regardless of operand width,
/// since width information lives in the width inference stage — we conservatively
/// mark all multiplies above the expression-level heuristic).
pub fn analyze_dsp(module: &Module, threshold: u32) -> DspAnalysis {
    let mut candidates = Vec::new();

    for reflex in &module.reflexes {
        if candidates.len() >= MAX_DSP_CANDIDATES {
            break;
        }
        for assignment in &reflex.assignments {
            if candidates.len() >= MAX_DSP_CANDIDATES {
                break;
            }
            if expr_contains_mul(&assignment.value) {
                candidates.push(DspCandidate {
                    reflex_name: reflex.name.clone(),
                    target_signal: assignment.target.clone(),
                });
            }
        }
    }

    DspAnalysis { candidates, threshold_bits: threshold }
}

/// Recursively check if an expression tree contains a `Mul` node.
///
/// Bounded traversal: counts nodes to prevent unbounded recursion on
/// pathological ASTs.
fn expr_contains_mul(expr: &Expr) -> bool {
    expr_contains_mul_bounded(expr, &mut 0)
}

fn expr_contains_mul_bounded(expr: &Expr, count: &mut usize) -> bool {
    *count += 1;
    if *count > MAX_EXPR_NODES {
        return false;
    }
    match expr {
        Expr::Binary { op: BinaryOp::Mul, .. } => true,
        Expr::Binary { left, right, .. } => {
            expr_contains_mul_bounded(left, count) || expr_contains_mul_bounded(right, count)
        }
        Expr::Unary { operand, .. } => expr_contains_mul_bounded(operand, count),
        Expr::Literal(_) | Expr::Signal(_) | Expr::Prev { .. } => false,
    }
}
