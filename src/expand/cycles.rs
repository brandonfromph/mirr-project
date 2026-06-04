//! Cycle detection for pattern expansion.

#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::pattern::PatternDef;
use crate::error::MirrError;

use super::pattern_err;

pub(super) fn detect_pattern_cycles(patterns: &[PatternDef]) -> Result<(), MirrError> {
    // Build name set for quick lookup.
    let pattern_names: HashSet<&str> = patterns.iter().map(|p| p.name.as_str()).collect();

    // Build adjacency list: for each pattern, which other patterns does it call?
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::with_capacity(patterns.len());
    for pat in patterns {
        let mut callees = Vec::new();
        for call in &pat.body.pattern_calls {
            let callee = call.pattern_name.as_str();
            if pattern_names.contains(callee) {
                callees.push(callee);
            }
        }
        adj.insert(pat.name.as_str(), callees);
    }

    // DFS cycle detection with explicit stack (no recursion).
    // States: 0 = unvisited, 1 = in-progress (on stack), 2 = done.
    let mut state: HashMap<&str, u8> = HashMap::with_capacity(patterns.len());
    let mut path: Vec<&str> = Vec::with_capacity(patterns.len());

    for pat in patterns {
        let start = pat.name.as_str();
        if *state.get(start).unwrap_or(&0) != 0 {
            continue;
        }

        // Explicit DFS stack: (node, child_index).
        let mut stack: Vec<(&str, usize)> = vec![(start, 0)];
        state.insert(start, 1);
        path.push(start);

        while let Some((node, idx)) = stack.last_mut() {
            let children = adj.get(node).map_or(&[] as &[&str], |v| v.as_slice());
            if *idx >= children.len() {
                // Done with this node.
                state.insert(node, 2);
                path.pop();
                stack.pop();
                continue;
            }

            let child = children[*idx];
            *idx += 1;

            match state.get(child).unwrap_or(&0) {
                0 => {
                    // Unvisited — descend.
                    state.insert(child, 1);
                    path.push(child);
                    stack.push((child, 0));
                }
                1 => {
                    // Back edge — cycle detected.
                    let cycle_start = path.iter().position(|&n| n == child).unwrap_or(0);
                    let cycle_path: Vec<&str> = path[cycle_start..].to_vec();
                    let cycle_str = cycle_path
                        .iter()
                        .copied()
                        .chain(std::iter::once(child))
                        .collect::<Vec<_>>()
                        .join(" -> ");
                    return Err(pattern_err(format!(
                        "{} Circular pattern reference detected: {cycle_str}.",
                        crate::error_codes::ec(428)
                    )));
                }
                _ => {} // Already done, skip.
            }
        }
    }

    Ok(())
}
