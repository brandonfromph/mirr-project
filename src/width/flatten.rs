//! Iterative tree flattening and reconstruction for width inference.
//!
//! Converts `Expr` trees into a flat `Vec<FlatNode>` in post-order (children
//! always have lower indices than their parent), and reconstructs a
//! `WidthExpr` tree from flat nodes plus resolved widths.
//!
//! All traversals are iterative with bounded loop counts (NASA P10 rule #1).

#![forbid(unsafe_code)]

use super::types::{FlatNode, Width, WidthExpr, MAX_FLAT_NODES};
use crate::ast::expr::Expr;
use crate::ast::types::LiteralValue;

// ---------------------------------------------------------------------------
// Flatten: Expr -> Vec<FlatNode>
// ---------------------------------------------------------------------------

/// Work item for iterative post-order flattening.
enum FlatWork<'a> {
    /// Visit this expression's children, then schedule an Emit.
    Visit(&'a Expr),
    /// Children have been emitted; emit this node and record child indices.
    EmitLiteral {
        value: u64,
    },
    EmitSignal {
        name: &'a str,
    },
    EmitUnary {
        op: crate::ast::types::UnaryOp,
    },
    EmitBinary {
        op: crate::ast::types::BinaryOp,
    },
    EmitPrev {
        signal: &'a str,
        delay: u64,
    },
}

/// Flatten an `Expr` tree into a `Vec<FlatNode>` in post-order.
///
/// Returns `None` if the tree exceeds `MAX_FLAT_NODES` nodes.
/// Bounded: the work stack processes at most `MAX_FLAT_NODES * 3` items
/// (each node produces at most 3 work items: visit + children + emit).
pub fn flatten_expr(expr: &Expr) -> Option<Vec<FlatNode>> {
    let mut work: Vec<FlatWork<'_>> = Vec::with_capacity(128);
    let mut nodes: Vec<FlatNode> = Vec::with_capacity(128);
    // Index stack tracks which FlatNode index each child was emitted to.
    let mut idx_stack: Vec<u32> = Vec::with_capacity(128);

    work.push(FlatWork::Visit(expr));

    let max_iters = MAX_FLAT_NODES * 3;
    let mut iters = 0usize;

    while let Some(item) = work.pop() {
        iters += 1;
        if iters > max_iters {
            return None; // Exceeded node budget.
        }

        match item {
            FlatWork::Visit(e) => match e {
                Expr::Literal(LiteralValue::Bool(b)) => {
                    let v = if *b { 1u64 } else { 0u64 };
                    work.push(FlatWork::EmitLiteral { value: v });
                }
                Expr::Literal(LiteralValue::Integer(i)) => {
                    work.push(FlatWork::EmitLiteral { value: *i });
                }
                Expr::Signal(name) => {
                    work.push(FlatWork::EmitSignal { name });
                }
                Expr::Unary { op, operand } => {
                    work.push(FlatWork::EmitUnary { op: *op });
                    work.push(FlatWork::Visit(operand));
                }
                Expr::Binary { op, left, right } => {
                    work.push(FlatWork::EmitBinary { op: *op });
                    // Push right first so left is visited first (LIFO).
                    work.push(FlatWork::Visit(right));
                    work.push(FlatWork::Visit(left));
                }
                Expr::Prev { signal, delay } => {
                    work.push(FlatWork::EmitPrev { signal, delay: *delay });
                }
            },
            FlatWork::EmitLiteral { value } => {
                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::Literal { value });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitSignal { name } => {
                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::Signal { name: name.to_string() });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitUnary { op } => {
                // pop() returning None means the work stack is malformed;
                // propagate None rather than silently using index 0.
                let operand_idx = idx_stack.pop()?;
                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::Unary { op, operand: operand_idx });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitBinary { op } => {
                // Left was visited first (LIFO), so right sits on top.
                // pop() returning None means malformed tree — propagate None.
                let right_idx = idx_stack.pop()?;
                let left_idx = idx_stack.pop()?;
                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::Binary { op, left: left_idx, right: right_idx });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitPrev { signal, delay } => {
                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::Prev { signal: signal.to_string(), delay });
                idx_stack.push(idx as u32);
            }
        }
    }

    Some(nodes)
}

// ---------------------------------------------------------------------------
// Reconstruct: Vec<FlatNode> + widths -> WidthExpr
// ---------------------------------------------------------------------------

/// Reconstruct a `WidthExpr` tree from flat nodes and their resolved widths.
///
/// `widths` must have the same length as `nodes`.
/// Returns `None` if the arrays are empty or mismatched.
///
/// Bounded: iterates once over `nodes` (len <= MAX_FLAT_NODES).
pub fn reconstruct_width_expr(nodes: &[FlatNode], widths: &[Width]) -> Option<WidthExpr> {
    if nodes.is_empty() || nodes.len() != widths.len() {
        return None;
    }

    // Build WidthExpr bottom-up: since nodes are in post-order, children
    // always have lower indices. We store built WidthExpr nodes in a vec
    // and reference them by index.
    let mut built: Vec<Option<WidthExpr>> = Vec::with_capacity(nodes.len());

    for (i, node) in nodes.iter().enumerate() {
        if i >= MAX_FLAT_NODES {
            break; // Bounded.
        }
        let w = widths[i];
        let we = match node {
            FlatNode::Literal { value } => WidthExpr::Literal { value: *value, width: w },
            FlatNode::Signal { name } => WidthExpr::Signal { name: name.clone(), width: w },
            FlatNode::Unary { op, operand } => {
                let operand_expr = built.get_mut(*operand as usize).and_then(|slot| slot.take())?;
                WidthExpr::Unary { op: *op, operand: Box::new(operand_expr), width: w }
            }
            FlatNode::Binary { op, left, right } => {
                let left_expr = built.get_mut(*left as usize).and_then(|slot| slot.take())?;
                let right_expr = built.get_mut(*right as usize).and_then(|slot| slot.take())?;
                WidthExpr::Binary {
                    op: *op,
                    left: Box::new(left_expr),
                    right: Box::new(right_expr),
                    width: w,
                }
            }
            FlatNode::Prev { signal, delay } => {
                WidthExpr::Prev { signal: signal.clone(), delay: *delay, width: w }
            }
        };
        built.push(Some(we));
    }

    // The last node is the root.
    built.last_mut().and_then(|slot| slot.take())
}
