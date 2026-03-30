//! Width Dependency Graph construction for Phase 4b SCC analysis.
//!
//! Builds a directed graph where each node is a signal and each edge
//! represents a width dependency through reflex assignments. Edges are
//! derived from signal and `prev()` references in assignment RHS expressions.
//!
//! Bounded: all traversals limited by MAX_SIGNALS.

#![forbid(unsafe_code)]

use super::types::MAX_SIGNALS;
use crate::ast::expr::Expr;
use crate::ast::program::MirrProgram;
use crate::ast::MAX_EXPR_NODES;

/// Directed graph of width dependencies between signals.
///
/// Nodes are indexed by their position in the program's signal declarations.
/// An edge `(a, b)` means "signal b's width depends on signal a's width"
/// — i.e., signal a appears in the RHS of an assignment to signal b.
pub struct WidthDepGraph {
    /// Number of nodes (signals).
    pub node_count: usize,
    /// Adjacency list: `adj[b]` contains indices of signals that b depends on.
    pub adj: Vec<Vec<usize>>,
    /// Signal names indexed by node id.
    pub signal_names: Vec<String>,
    /// For each edge (source, target), the BinaryOp on the cycle path (if any).
    /// Used by SCC classification to determine expansive vs nonexpansive.
    pub edge_ops: Vec<(usize, usize, Vec<crate::ast::types::BinaryOp>)>,
}

/// Build a Width Dependency Graph from a MIRR program.
///
/// Walks all reflex assignments and collects signal references (including
/// `Prev` references) to build the adjacency list.
///
/// Bounded: outer loop over reflexes/assignments is finite (from parser).
/// Inner expression walk bounded by MAX_EXPR_NODES.
pub fn build_graph(program: &MirrProgram) -> WidthDepGraph {
    let signals = &program.module.signals;
    let n = signals.len().min(MAX_SIGNALS);

    // Build name-to-index map.
    let mut name_to_idx: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::with_capacity(n);
    let mut signal_names: Vec<String> = Vec::with_capacity(n);
    for (i, s) in signals.iter().enumerate() {
        if i >= MAX_SIGNALS {
            break;
        }
        name_to_idx.insert(&s.name, i);
        signal_names.push(s.name.clone());
    }

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut edge_ops: Vec<(usize, usize, Vec<crate::ast::types::BinaryOp>)> = Vec::new();

    // Walk all reflex assignments.
    for r in &program.module.reflexes {
        for a in &r.assignments {
            let target_idx = match name_to_idx.get(a.target.as_str()) {
                Some(&idx) => idx,
                None => continue,
            };

            // Collect signal references and ops from the RHS expression.
            let refs = collect_refs_with_ops(&a.value);
            for (ref_name, ops) in &refs {
                if let Some(&source_idx) = name_to_idx.get(ref_name.as_str()) {
                    // Avoid duplicate edges.
                    if !adj[target_idx].contains(&source_idx) {
                        adj[target_idx].push(source_idx);
                    }
                    edge_ops.push((source_idx, target_idx, ops.clone()));
                }
            }
        }
    }

    WidthDepGraph { node_count: n, adj, signal_names, edge_ops }
}

/// Collected reference: (signal_name, binary_ops_on_path_to_root).
type RefWithOps = (String, Vec<crate::ast::types::BinaryOp>);

/// Collect all signal/prev references from an expression, along with the
/// binary operators on the path from that reference to the expression root.
///
/// Uses an explicit work stack (no recursion). Bounded by MAX_EXPR_NODES.
fn collect_refs_with_ops(expr: &Expr) -> Vec<RefWithOps> {
    let mut results: Vec<RefWithOps> = Vec::with_capacity(16);

    // Work stack: (expression, ops_on_path_to_here).
    let mut stack: Vec<(&Expr, Vec<crate::ast::types::BinaryOp>)> = Vec::with_capacity(32);
    stack.push((expr, Vec::new()));

    let mut visited = 0usize;
    while let Some((node, ops)) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        match node {
            Expr::Signal(name) => {
                results.push((name.clone(), ops));
            }
            Expr::Prev { signal, .. } => {
                results.push((signal.clone(), ops));
            }
            Expr::Literal(_) => {}
            Expr::Unary { operand, .. } => {
                stack.push((operand, ops));
            }
            Expr::Binary { op, left, right } => {
                let mut left_ops = ops.clone();
                left_ops.push(*op);
                let mut right_ops = ops;
                right_ops.push(*op);
                stack.push((left, left_ops));
                stack.push((right, right_ops));
            }
            Expr::ArrayIndex { array, index } => {
                stack.push((array, ops.clone()));
                stack.push((index, ops));
            }
            Expr::FieldAccess { object, .. } => {
                stack.push((object, ops));
            }
            Expr::ArrayLiteral(elems) => {
                let mut i = 0;
                while i < elems.len().min(MAX_EXPR_NODES) {
                    stack.push((&elems[i], ops.clone()));
                    i += 1;
                }
            }
            Expr::StructLiteral { fields, .. } => {
                let mut i = 0;
                while i < fields.len().min(MAX_EXPR_NODES) {
                    stack.push((&fields[i].1, ops.clone()));
                    i += 1;
                }
            }
            Expr::UnfoldIndex(_) => {
                // Unresolved meta-stage index has no signal refs.
            }
        }
    }

    results
}
