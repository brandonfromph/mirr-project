//! Pretty-printing for width-annotated expressions and diagnostics.
//!
//! Provides `Display` implementations and summary formatting used by
//! the `mirr-width` CLI tool.

#![forbid(unsafe_code)]

use super::types::{WidthExpr, WidthStats};

/// Format a `WidthExpr` tree as a human-readable string with width annotations.
///
/// Uses an iterative work-stack to avoid recursion (NASA P10 rule #1).
/// Bounded: at most MAX_FLAT_NODES * 3 iterations.
pub fn format_width_expr(expr: &WidthExpr) -> String {
    // For display purposes, a simple iterative approach using a result stack.
    let mut work: Vec<FormatWork<'_>> = Vec::with_capacity(128);
    let mut results: Vec<String> = Vec::with_capacity(128);

    work.push(FormatWork::Visit(expr));

    let max_iters = 512 * 3;
    let mut iters = 0usize;

    while let Some(item) = work.pop() {
        iters += 1;
        if iters > max_iters {
            break;
        }
        match item {
            FormatWork::Visit(e) => match e {
                WidthExpr::Literal { value, width } => {
                    results.push(format!("{}:{}", value, width));
                }
                WidthExpr::Signal { name, width } => {
                    results.push(format!("{}:{}", name, width));
                }
                WidthExpr::Unary { op, operand, width } => {
                    work.push(FormatWork::CombineUnary { op: format!("{:?}", op), width: *width });
                    work.push(FormatWork::Visit(operand));
                }
                WidthExpr::Binary { op, left, right, width } => {
                    work.push(FormatWork::CombineBinary {
                        op: format_binary_op(*op),
                        width: *width,
                    });
                    work.push(FormatWork::Visit(right));
                    work.push(FormatWork::Visit(left));
                }
            },
            FormatWork::CombineUnary { op, width } => {
                let operand_s = results.pop().unwrap_or_default();
                results.push(format!("({}{}):{}",  op, operand_s, width));
            }
            FormatWork::CombineBinary { op, width } => {
                let right_s = results.pop().unwrap_or_default();
                let left_s = results.pop().unwrap_or_default();
                results.push(format!("({} {} {}):{}", left_s, op, right_s, width));
            }
        }
    }

    results.pop().unwrap_or_default()
}

enum FormatWork<'a> {
    Visit(&'a WidthExpr),
    CombineUnary { op: String, width: super::types::Width },
    CombineBinary { op: String, width: super::types::Width },
}

fn format_binary_op(op: crate::ast::types::BinaryOp) -> String {
    use crate::ast::types::BinaryOp;
    match op {
        BinaryOp::Add => "+".to_string(),
        BinaryOp::Sub => "-".to_string(),
        BinaryOp::Mul => "*".to_string(),
        BinaryOp::Shl => "<<".to_string(),
        BinaryOp::Shr => ">>".to_string(),
        BinaryOp::And => "&".to_string(),
        BinaryOp::Or => "|".to_string(),
        BinaryOp::Xor => "^".to_string(),
        BinaryOp::Lt => "<".to_string(),
        BinaryOp::Le => "<=".to_string(),
        BinaryOp::Gt => ">".to_string(),
        BinaryOp::Ge => ">=".to_string(),
        BinaryOp::Eq => "==".to_string(),
        BinaryOp::Ne => "!=".to_string(),
    }
}

/// Format a `WidthStats` as a summary line.
pub fn format_stats(stats: &WidthStats) -> String {
    format!(
        "nodes={} rounds={} diagnostics={}",
        stats.nodes_analyzed, stats.propagation_rounds, stats.diagnostics_count,
    )
}
