//! Width Dependency Graph construction for Phase 4b SCC analysis.
//!
//! Builds a directed graph where each node is a signal and each edge
//! represents a width dependency through reflex assignments. Edges are
//! derived from signal and `prev()` references in assignment RHS expressions.
//!
//! Bounded: all traversals limited by MAX_SIGNALS.

#![forbid(unsafe_code)]

use crate::ast::MAX_EXPR_NODES;
use crate::ecs::components::EntityId;
use crate::ecs::registry::Registry;

/// Directed graph of width dependencies between signals.
pub struct WidthDepGraph {
    /// Number of nodes (max entity ID + 1).
    pub node_count: usize,
    /// Adjacency list: `adj[b]` contains indices of signals that b depends on.
    pub adj: Vec<Vec<usize>>,
    /// For each edge (source, target), the BinaryOp on the cycle path (if any).
    pub edge_ops: Vec<(usize, usize, Vec<crate::ast::types::BinaryOp>)>,
}

/// Build a Width Dependency Graph from the ECS Registry.
///
/// Walks all reflex assignments and collects signal references (including
/// `Prev` references) to build the adjacency list.
pub fn build_graph(registry: &Registry) -> WidthDepGraph {
    let n = registry.names.len(); // Upper bound for entity indices

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut edge_ops: Vec<(usize, usize, Vec<crate::ast::types::BinaryOp>)> = Vec::new();

    // Walk all assignments.
    for assignment in registry.assignment_comps.iter().flatten() {
        let target_idx = assignment.target.0 as usize;
        if target_idx >= n {
            continue;
        }

        let refs = collect_refs_with_ops_ecs(registry, assignment.value);
        for (source_id, ops) in &refs {
            let source_idx = source_id.0 as usize;
            if source_idx >= n {
                continue;
            }

            if !adj[target_idx].contains(&source_idx) {
                adj[target_idx].push(source_idx);
            }
            edge_ops.push((source_idx, target_idx, ops.clone()));
        }
    }

    WidthDepGraph { node_count: n, adj, edge_ops }
}

type RefWithOps = (EntityId, Vec<crate::ast::types::BinaryOp>);

/// Collect all signal/prev references from an ECS expression, along with the
/// binary operators on the path from that reference to the expression root.
fn collect_refs_with_ops_ecs(registry: &Registry, expr_root: EntityId) -> Vec<RefWithOps> {
    let mut results: Vec<RefWithOps> = Vec::with_capacity(16);

    let mut stack: Vec<(EntityId, Vec<crate::ast::types::BinaryOp>)> = Vec::with_capacity(32);
    stack.push((expr_root, Vec::new()));

    let mut visited = 0usize;
    while let Some((node, ops)) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }

        let idx = node.0 as usize;

        if let Some(sig_ref) = &registry.signal_refs[idx] {
            results.push((sig_ref.0, ops));
            continue;
        }
        if let Some(prev) = &registry.prev_ops[idx] {
            results.push((prev.signal, ops));
            continue;
        }
        if registry.literals[idx].is_some() {
            continue;
        }
        if let Some(un) = &registry.unary_ops[idx] {
            stack.push((un.operand, ops));
            continue;
        }
        if let Some(bin) = &registry.binary_ops[idx] {
            let mut left_ops = ops.clone();
            left_ops.push(bin.op);
            let mut right_ops = ops;
            right_ops.push(bin.op);
            stack.push((bin.left, left_ops));
            stack.push((bin.right, right_ops));
            continue;
        }
        if let Some(arr_idx) = &registry.array_indices[idx] {
            stack.push((arr_idx.array, ops.clone()));
            stack.push((arr_idx.index, ops));
            continue;
        }
        if let Some(field) = &registry.field_accesses[idx] {
            stack.push((field.object, ops));
            continue;
        }
        if let Some(arr_lit) = &registry.array_literals[idx] {
            for elem in arr_lit.0.iter().take(MAX_EXPR_NODES) {
                stack.push((*elem, ops.clone()));
            }
            continue;
        }
        if let Some(struct_lit) = &registry.struct_literals[idx] {
            for (_, elem) in struct_lit.fields.iter().take(MAX_EXPR_NODES) {
                stack.push((*elem, ops.clone()));
            }
            continue;
        }
    }

    results
}
