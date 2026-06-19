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

/// Analyze a module for multiply operations suitable for DSP inference using the ECS Registry.
pub fn analyze_dsp_ecs(registry: &crate::ecs::Registry, threshold: u32) -> DspAnalysis {
    let mut candidates = Vec::new();

    for i in 0..registry.reflex_comps.len() {
        if let Some(reflex) = &registry.reflex_comps[i] {
            if candidates.len() >= MAX_DSP_CANDIDATES {
                break;
            }

            for asgn_ent in &reflex.assignments {
                if let Some(asgn) = &registry.assignment_comps[asgn_ent.0 as usize] {
                    if has_multiply_ecs(asgn.value, registry) {
                        let reflex_name = registry.names[i]
                            .map(|nc| registry.resolve_name(nc.0).to_string())
                            .unwrap_or_else(|| "unnamed_reflex".to_string());
                        let target_name = registry.names[asgn.target.0 as usize]
                            .map(|nc| registry.resolve_name(nc.0).to_string())
                            .unwrap_or_else(|| "unnamed_target".to_string());
                        candidates.push(DspCandidate { reflex_name, target_signal: target_name });
                        break;
                    }
                }
            }
        }
    }

    DspAnalysis { candidates, threshold_bits: threshold }
}

fn has_multiply_ecs(root: crate::ecs::EntityId, registry: &crate::ecs::Registry) -> bool {
    let mut stack = Vec::new();
    stack.push(root);

    let mut visited = 0;
    while let Some(ent) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }

        let i = ent.0 as usize;
        if let Some(bin) = &registry.binary_ops[i] {
            if bin.op == BinaryOp::Mul {
                return true;
            }
            stack.push(bin.left);
            stack.push(bin.right);
        } else if let Some(un) = &registry.unary_ops[i] {
            stack.push(un.operand);
        } else if let Some(m) = &registry.muxes[i] {
            stack.push(m.select);
            stack.push(m.true_val);
            stack.push(m.false_val);
        }
        // ... other expression nodes if needed, but Mul is usually binary
    }
    false
}

/// Analyze a module for multiply operations suitable for DSP inference.
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
        Expr::ArrayIndex { array, index } => {
            expr_contains_mul_bounded(array, count) || expr_contains_mul_bounded(index, count)
        }
        Expr::FieldAccess { object, .. } => expr_contains_mul_bounded(object, count),
        Expr::ArrayLiteral(elems) => {
            let mut j = 0;
            while j < elems.len() && j < MAX_EXPR_NODES {
                if expr_contains_mul_bounded(&elems[j], count) {
                    return true;
                }
                j += 1;
            }
            false
        }
        Expr::StructLiteral { fields, .. } => {
            let mut j = 0;
            while j < fields.len() && j < MAX_EXPR_NODES {
                if expr_contains_mul_bounded(&fields[j].1, count) {
                    return true;
                }
                j += 1;
            }
            false
        }
        Expr::UnfoldIndex(_) => {
            unreachable!("UnfoldIndex reached DSP analysis stage")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expr::Expr;
    use crate::ast::program::{Assignment, Module, Reflex};
    use crate::ast::types::{BinaryOp, LiteralValue};

    fn make_module(reflexes: Vec<Reflex>) -> Module {
        Module {
            name: "test_dsp".to_string(),
            signals: vec![],
            guards: vec![],
            reflexes,
            pattern_calls: vec![],
            pattern_origins: vec![],
            properties: vec![],
            span: None,
        }
    }

    fn make_reflex(name: &str, assignments: Vec<Assignment>) -> Reflex {
        Reflex {
            name: name.to_string(),
            guard_names: vec![],
            assignments,
            origin: None,
            span: None,
        }
    }

    fn make_assignment(target: &str, value: Expr) -> Assignment {
        Assignment { target: target.to_string(), value, span: None }
    }

    #[test]
    fn test_dsp_no_multiply() {
        let module = make_module(vec![make_reflex(
            "r1",
            vec![make_assignment("out", Expr::Literal(LiteralValue::Integer(42)))],
        )]);
        let analysis = analyze_dsp(&module, DEFAULT_DSP_THRESHOLD);
        assert!(analysis.candidates.is_empty());
        assert_eq!(analysis.threshold_bits, DEFAULT_DSP_THRESHOLD);
    }

    #[test]
    fn test_dsp_detects_multiply() {
        let mul_expr = Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(Expr::Signal("a".to_string())),
            right: Box::new(Expr::Signal("b".to_string())),
        };
        let module =
            make_module(vec![make_reflex("r1", vec![make_assignment("product", mul_expr)])]);
        let analysis = analyze_dsp(&module, DEFAULT_DSP_THRESHOLD);
        assert_eq!(analysis.candidates.len(), 1);
        assert_eq!(analysis.candidates[0].reflex_name, "r1");
        assert_eq!(analysis.candidates[0].target_signal, "product");
    }

    #[test]
    fn test_dsp_nested_multiply() {
        let mul_expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Binary {
                op: BinaryOp::Mul,
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Signal("b".to_string())),
            }),
            right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
        };
        let module = make_module(vec![make_reflex("r1", vec![make_assignment("out", mul_expr)])]);
        let analysis = analyze_dsp(&module, DEFAULT_DSP_THRESHOLD);
        assert_eq!(analysis.candidates.len(), 1);
    }

    #[test]
    fn test_dsp_respects_max_candidates() {
        let analysis = analyze_dsp(&make_module(vec![]), DEFAULT_DSP_THRESHOLD);
        assert!(analysis.candidates.len() <= MAX_DSP_CANDIDATES);
    }
}
