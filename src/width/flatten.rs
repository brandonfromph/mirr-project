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
use crate::ast::types::{LiteralValue, SignalType};

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
    EmitArrayIndex {
        width: u32,
        signed: bool,
    },
    EmitFieldAccess {
        field: String,
        width: u32,
        signed: bool,
    },
    EmitArrayLiteral {
        len: usize,
    },
    EmitStructLiteral {
        name: String,
        field_names: Vec<String>,
    },
    EmitUnfoldIndex {
        name: &'a str,
    },
}

/// Flatten an `Expr` tree into a `Vec<FlatNode>` in post-order.
///
/// `signals` provides signal declarations for signedness lookup.
///
/// Returns `None` if the tree exceeds `MAX_FLAT_NODES` nodes.
/// Bounded: the work stack processes at most `MAX_FLAT_NODES * 3` items
/// (each node produces at most 3 work items: visit + children + emit).
pub fn flatten_expr(expr: &Expr, signals: &[crate::ast::SignalDecl]) -> Option<Vec<FlatNode>> {
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
                Expr::ArrayIndex { array, index } => {
                    let (width, signed) = infer_array_index_shape(array, signals);
                    work.push(FlatWork::EmitArrayIndex { width, signed });
                    // Visit array first, then index (LIFO).
                    work.push(FlatWork::Visit(index));
                    work.push(FlatWork::Visit(array));
                }
                Expr::FieldAccess { object, field } => {
                    let (width, signed) = infer_field_access_shape(object, field, signals);
                    work.push(FlatWork::EmitFieldAccess { field: field.clone(), width, signed });
                    work.push(FlatWork::Visit(object));
                }
                Expr::ArrayLiteral(elements) => {
                    work.push(FlatWork::EmitArrayLiteral { len: elements.len() });
                    for elem in elements.iter().rev() {
                        work.push(FlatWork::Visit(elem));
                    }
                }
                Expr::StructLiteral { name, fields } => {
                    let field_names: Vec<String> =
                        fields.iter().map(|(field_name, _)| field_name.clone()).collect();
                    work.push(FlatWork::EmitStructLiteral { name: name.clone(), field_names });
                    for (_, value) in fields.iter().rev() {
                        work.push(FlatWork::Visit(value));
                    }
                }
                Expr::UnfoldIndex(name) => {
                    work.push(FlatWork::EmitUnfoldIndex { name });
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
                nodes.push(FlatNode::Signal {
                    name: name.to_string(),
                    signed: is_signed(name, signals),
                });
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
                nodes.push(FlatNode::Prev {
                    signal: signal.to_string(),
                    delay,
                    signed: is_signed(signal, signals),
                });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitArrayIndex { width, signed } => {
                // Array was visited before index; index is on top.
                let index_idx = idx_stack.pop()?;
                let array_idx = idx_stack.pop()?;
                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::ArrayIndex {
                    array: array_idx,
                    index: index_idx,
                    width,
                    signed,
                });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitFieldAccess { field, width, signed } => {
                let object_idx = idx_stack.pop()?;
                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::FieldAccess { object: object_idx, field, width, signed });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitArrayLiteral { len } => {
                let mut elements_rev: Vec<u32> = Vec::with_capacity(len);
                for _ in 0..len {
                    elements_rev.push(idx_stack.pop()?);
                }
                elements_rev.reverse();

                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::ArrayLiteral { elements: elements_rev, width: 0 });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitStructLiteral { name, field_names } => {
                let mut field_ids_rev: Vec<u32> = Vec::with_capacity(field_names.len());
                for _ in 0..field_names.len() {
                    field_ids_rev.push(idx_stack.pop()?);
                }
                field_ids_rev.reverse();

                let mut flat_fields: Vec<(String, u32)> = Vec::with_capacity(field_names.len());
                for (field_name, field_id) in field_names.into_iter().zip(field_ids_rev.into_iter())
                {
                    flat_fields.push((field_name, field_id));
                }

                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::StructLiteral { name, fields: flat_fields, width: 0 });
                idx_stack.push(idx as u32);
            }
            FlatWork::EmitUnfoldIndex { name } => {
                let idx = nodes.len();
                if idx >= MAX_FLAT_NODES {
                    return None;
                }
                nodes.push(FlatNode::UnfoldIndex { name: name.to_string() });
                idx_stack.push(idx as u32);
            }
        }
    }

    Some(nodes)
}

/// Check whether a signal is declared as signed.
fn is_signed(name: &str, signals: &[crate::ast::SignalDecl]) -> bool {
    signals.iter().any(|s| s.name == name && matches!(s.ty.signal_type(), SignalType::Signed(_)))
}

fn infer_array_index_shape(array_expr: &Expr, signals: &[crate::ast::SignalDecl]) -> (u32, bool) {
    match array_expr {
        Expr::Signal(name) => {
            if let Some(sig) = signals.iter().find(|s| s.name == *name) {
                if let SignalType::Array { element, .. } = sig.ty.signal_type() {
                    return (element.width(), matches!(element.as_ref(), SignalType::Signed(_)));
                }
            }
            (32, false)
        }
        _ => (32, false),
    }
}

fn infer_field_access_shape(
    object_expr: &Expr,
    field_name: &str,
    signals: &[crate::ast::SignalDecl],
) -> (u32, bool) {
    match object_expr {
        Expr::Signal(name) => {
            if let Some(sig) = signals.iter().find(|s| s.name == *name) {
                if let SignalType::Struct { fields, .. } = sig.ty.signal_type() {
                    if let Some((_, field_ty)) =
                        fields.iter().find(|(fname, _)| fname == field_name)
                    {
                        return (field_ty.width(), matches!(field_ty, SignalType::Signed(_)));
                    }
                }
            }
            (32, false)
        }
        Expr::StructLiteral { fields, .. } => {
            if let Some((_, field_expr)) = fields.iter().find(|(fname, _)| fname == field_name) {
                return infer_expr_shape(field_expr, signals);
            }
            (32, false)
        }
        _ => (32, false),
    }
}

fn infer_expr_shape(expr: &Expr, signals: &[crate::ast::SignalDecl]) -> (u32, bool) {
    match expr {
        Expr::Literal(LiteralValue::Bool(_)) => (1, false),
        Expr::Literal(LiteralValue::Integer(v)) => (super::types::Width::min_bits_for(*v).0, false),
        Expr::Signal(name) | Expr::Prev { signal: name, .. } => {
            if let Some(sig) = signals.iter().find(|s| s.name == *name) {
                let ty = sig.ty.signal_type();
                return (ty.width(), matches!(ty, SignalType::Signed(_)));
            }
            (32, false)
        }
        _ => (32, false),
    }
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
            FlatNode::Signal { name, .. } => WidthExpr::Signal { name: name.clone(), width: w },
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
            FlatNode::Prev { signal, delay, .. } => {
                WidthExpr::Prev { signal: signal.clone(), delay: *delay, width: w }
            }
            FlatNode::ArrayIndex { .. }
            | FlatNode::FieldAccess { .. }
            | FlatNode::ArrayLiteral { .. }
            | FlatNode::StructLiteral { .. }
            | FlatNode::UnfoldIndex { .. } => {
                // Non-atomic/wide composite values are represented as a placeholder.
                WidthExpr::Literal { value: 0, width: w }
            }
        };
        built.push(Some(we));
    }

    // The last node is the root.
    built.last_mut().and_then(|slot| slot.take())
}
