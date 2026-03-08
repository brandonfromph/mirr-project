//! Iterative Tarjan's SCC detection for the Width Dependency Graph.
//!
//! Finds all strongly connected components in the graph and classifies
//! each as expansive or nonexpansive based on the operators on cycle edges.
//!
//! Implementation uses an explicit stack (no recursion, NASA P10 rule #1).
//! Bounded by MAX_SIGNALS * 3 iterations.

#![forbid(unsafe_code)]

use super::graph::WidthDepGraph;
use super::types::WidthDiag;
use super::types::{SccInfo, SccKind, MAX_SCC_SIZE, MAX_SIGNALS};
use crate::ast::types::BinaryOp;

/// Result of SCC detection.
pub struct SccResult {
    /// Non-trivial SCCs (size >= 2 or self-loops).
    pub sccs: Vec<SccInfo>,
    /// Diagnostics emitted during detection.
    pub diagnostics: Vec<WidthDiag>,
}

/// Find all non-trivial SCCs in the width dependency graph.
///
/// Uses iterative Tarjan's algorithm with an explicit call stack.
/// Bounded: at most MAX_SIGNALS * 3 total iterations.
pub fn find_sccs(graph: &WidthDepGraph) -> SccResult {
    let n = graph.node_count.min(MAX_SIGNALS);
    let mut diagnostics: Vec<WidthDiag> = Vec::new();

    // Tarjan's state.
    let mut index_counter: u32 = 0;
    let mut indices: Vec<Option<u32>> = vec![None; n];
    let mut lowlinks: Vec<u32> = vec![0; n];
    let mut on_stack: Vec<bool> = vec![false; n];
    let mut tarjan_stack: Vec<usize> = Vec::with_capacity(n);
    let mut all_sccs: Vec<Vec<usize>> = Vec::new();

    // Iterative DFS call stack.
    // Each frame: (node, neighbor_index_into_adj, caller_return_idx)
    // caller_return_idx is None for top-level calls.
    let mut call_stack: Vec<(usize, usize, Option<usize>)> = Vec::with_capacity(n);

    let max_iters = n.saturating_mul(3).max(1);
    let mut iters = 0usize;

    for start in 0..n {
        if indices[start].is_some() {
            continue;
        }
        call_stack.push((start, 0, None));

        while let Some(&mut (v, ref mut ni, caller)) = call_stack.last_mut() {
            iters += 1;
            if iters > max_iters {
                diagnostics.push(WidthDiag::error(
                    "[E506] SCC detection exceeded iteration budget".to_string(),
                ));
                return SccResult { sccs: Vec::new(), diagnostics };
            }

            // First visit: assign index and lowlink.
            if indices[v].is_none() {
                indices[v] = Some(index_counter);
                lowlinks[v] = index_counter;
                index_counter += 1;
                on_stack[v] = true;
                tarjan_stack.push(v);
            }

            // Explore neighbors.
            let adj = &graph.adj[v];
            let mut descended = false;
            while *ni < adj.len() {
                let w = adj[*ni];
                *ni += 1;
                if w >= n {
                    continue;
                }
                if indices[w].is_none() {
                    // Not yet visited — descend.
                    call_stack.push((w, 0, Some(v)));
                    descended = true;
                    break;
                } else if on_stack[w] {
                    // Back-edge: update lowlink.
                    if let Some(w_idx) = indices[w] {
                        if w_idx < lowlinks[v] {
                            lowlinks[v] = w_idx;
                        }
                    }
                }
            }
            if descended {
                continue;
            }

            // All neighbors explored — check if v is SCC root.
            if indices[v] == Some(lowlinks[v]) {
                let mut scc_members: Vec<usize> = Vec::new();
                while let Some(w) = tarjan_stack.pop() {
                    on_stack[w] = false;
                    scc_members.push(w);
                    if w == v {
                        break;
                    }
                }
                // Only keep non-trivial SCCs.
                let is_self_loop =
                    scc_members.len() == 1 && graph.adj[scc_members[0]].contains(&scc_members[0]);
                if scc_members.len() > 1 || is_self_loop {
                    all_sccs.push(scc_members);
                }
            }

            // Pop this frame and update caller's lowlink.
            let popped_v = v;
            let popped_lowlink = lowlinks[popped_v];
            call_stack.pop();

            if let Some(c) = caller {
                if popped_lowlink < lowlinks[c] {
                    lowlinks[c] = popped_lowlink;
                }
            }
        }
    }

    // Classify each SCC.
    let mut sccs: Vec<SccInfo> = Vec::with_capacity(all_sccs.len());
    for members in all_sccs {
        if members.len() > MAX_SCC_SIZE {
            let names: Vec<&str> = members
                .iter()
                .take(5)
                .filter_map(|&i| graph.signal_names.get(i).map(|s| s.as_str()))
                .collect();
            diagnostics.push(WidthDiag::error(format!(
                "[E507] SCC with {} signals exceeds maximum size of {}; signals include: {}",
                members.len(),
                MAX_SCC_SIZE,
                names.join(", ")
            )));
            continue;
        }

        let kind = classify_scc(&members, graph);
        sccs.push(SccInfo { signal_indices: members, kind });
    }

    SccResult { sccs, diagnostics }
}

/// Classify an SCC as expansive or nonexpansive.
///
/// Expansive: any edge within the SCC involves Add, Mul, or Shl.
/// Nonexpansive: all edges are Prev-only, And/Or/Xor, Sub, Shr, comparisons.
///
/// Bounded: iterates over edge_ops (finite from graph construction).
fn classify_scc(members: &[usize], graph: &WidthDepGraph) -> SccKind {
    let member_set: std::collections::HashSet<usize> = members.iter().copied().collect();

    for (source, target, ops) in &graph.edge_ops {
        // Only consider edges within this SCC.
        if !member_set.contains(source) || !member_set.contains(target) {
            continue;
        }
        for op in ops {
            match op {
                BinaryOp::Add | BinaryOp::Mul | BinaryOp::Shl => {
                    return SccKind::Expansive;
                }
                _ => {}
            }
        }
    }

    SccKind::Nonexpansive
}
