//! MEGA-12: Operation binding for HLS.
//!
//! Binds scheduled operations to physical resources using greedy left-edge
//! binding. Each operation is assigned to a physical functional unit (ALU,
//! multiplier, etc.) based on its resource kind and time slot.
//!
//! Operations that can share a resource (non-overlapping time slots, same
//! kind) are bound to the same physical unit.
//!
//! NASA Power-of-10: all loops bounded by schedule size.

#![forbid(unsafe_code)]

use super::schedule::ScheduleOp;

/// Bind operations to physical resources using greedy left-edge binding.
///
/// Algorithm:
/// 1. Sort operations by earliest cycle (left edge).
/// 2. For each operation, try to bind to an existing resource of the same kind.
/// 3. If no existing resource is free, allocate a new one.
///
/// Returns a vector of resource IDs (one per operation in the schedule).
pub fn bind_operations(schedule: &[ScheduleOp]) -> Vec<u32> {
    let mut bindings: Vec<u32> = vec![0; schedule.len()];

    if schedule.is_empty() {
        return bindings;
    }

    // Group operations by resource kind.
    let mut kind_groups: Vec<(super::ResourceKind, Vec<usize>)> = Vec::new();

    let mut i = 0;
    while i < schedule.len() {
        let kind = schedule[i].resource;
        // Find existing group for this kind.
        let mut found = false;
        let mut j = 0;
        while j < kind_groups.len() {
            if kind_groups[j].0 == kind {
                kind_groups[j].1.push(i);
                found = true;
                break;
            }
            j += 1;
        }
        if !found {
            kind_groups.push((kind, vec![i]));
        }
        i += 1;
    }

    // For each kind group, apply greedy left-edge binding.
    let mut global_resource_id: u32 = 0;

    let mut g = 0;
    while g < kind_groups.len() {
        let ops = &kind_groups[g].1;

        // Sort by earliest cycle (left edge).
        let mut sorted_ops: Vec<usize> = ops.clone();
        // Simple insertion sort (bounded by schedule size).
        let mut a = 1;
        while a < sorted_ops.len() {
            let key = sorted_ops[a];
            let mut b = a;
            while b > 0 && schedule[sorted_ops[b - 1]].earliest > schedule[key].earliest {
                sorted_ops[b] = sorted_ops[b - 1];
                b -= 1;
            }
            sorted_ops[b] = key;
            a += 1;
        }

        // Track which operations have been bound.
        let mut bound: Vec<bool> = vec![false; sorted_ops.len()];

        // Greedy left-edge: try to bind each operation to an existing resource.
        let mut a = 0;
        while a < sorted_ops.len() {
            if bound[a] {
                a += 1;
                continue;
            }

            // Allocate a new resource for this operation.
            let resource_id = global_resource_id;
            global_resource_id += 1;

            bindings[sorted_ops[a]] = resource_id;
            bound[a] = true;

            // Try to bind subsequent non-overlapping operations to the same resource.
            let mut b = a + 1;
            while b < sorted_ops.len() {
                if bound[b] {
                    b += 1;
                    continue;
                }

                let op_a = &schedule[sorted_ops[a]];
                let op_b = &schedule[sorted_ops[b]];

                // Check if time slots don't overlap.
                if op_a.latest < op_b.earliest || op_b.latest < op_a.earliest {
                    bindings[sorted_ops[b]] = resource_id;
                    bound[b] = true;
                }
                b += 1;
            }
            a += 1;
        }
        g += 1;
    }

    bindings
}

/// Count the number of physical resources needed.
pub fn count_physical_resources(bindings: &[u32]) -> u32 {
    let mut max_id: u32 = 0;
    let mut i = 0;
    while i < bindings.len() {
        if bindings[i] > max_id {
            max_id = bindings[i];
        }
        i += 1;
    }
    max_id + 1
}

/// Validate that bindings are consistent with sharing groups.
///
/// Operations in the same sharing group should have the same binding.
pub fn validate_bindings(bindings: &[u32], sharing_groups: &[Vec<usize>]) -> bool {
    let mut g = 0;
    while g < sharing_groups.len() {
        let group = &sharing_groups[g];
        if group.is_empty() {
            g += 1;
            continue;
        }

        let expected_binding = bindings[group[0]];
        let mut i = 1;
        while i < group.len() {
            if group[i] < bindings.len() && bindings[group[i]] != expected_binding {
                return false;
            }
            i += 1;
        }
        g += 1;
    }
    true
}

/// Binding summary.
#[derive(Debug, Clone)]
pub struct BindingSummary {
    /// Total physical resources used.
    pub total_resources: u32,
    /// Resource IDs used.
    pub resource_ids: Vec<u32>,
}

/// Compute binding summary from bindings.
pub fn binding_summary(bindings: &[u32]) -> BindingSummary {
    let mut unique_ids: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < bindings.len() {
        if !unique_ids.contains(&bindings[i]) {
            unique_ids.push(bindings[i]);
        }
        i += 1;
    }

    // Sort resource IDs.
    let mut a = 1;
    while a < unique_ids.len() {
        let key = unique_ids[a];
        let mut b = a;
        while b > 0 && unique_ids[b - 1] > key {
            unique_ids[b] = unique_ids[b - 1];
            b -= 1;
        }
        unique_ids[b] = key;
        a += 1;
    }

    BindingSummary { total_resources: unique_ids.len() as u32, resource_ids: unique_ids }
}

/// Generate a binding summary string for debugging.
pub fn binding_debug(schedule: &[ScheduleOp], bindings: &[u32]) -> String {
    let mut lines: Vec<String> = Vec::new();

    let mut i = 0;
    while i < schedule.len() {
        let op = &schedule[i];
        if i < bindings.len() {
            lines.push(format!(
                "Op {} ({} @ [{}..{}]) -> Resource {}",
                op.op_id, op.resource, op.earliest, op.latest, bindings[i]
            ));
        }
        i += 1;
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_operations_single() {
        let schedule = vec![ScheduleOp {
            op_id: 0,
            earliest: 0,
            latest: 1,
            resource: super::super::ResourceKind::Add,
        }];

        let bindings = bind_operations(&schedule);
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0], 0);
    }

    #[test]
    fn test_bind_operations_no_overlap() {
        let schedule = vec![
            ScheduleOp {
                op_id: 0,
                earliest: 0,
                latest: 1,
                resource: super::super::ResourceKind::Add,
            },
            ScheduleOp {
                op_id: 1,
                earliest: 2,
                latest: 3,
                resource: super::super::ResourceKind::Add,
            },
        ];

        let bindings = bind_operations(&schedule);
        assert_eq!(bindings.len(), 2);
        // Non-overlapping ops of same kind should share resource.
        assert_eq!(bindings[0], bindings[1]);
    }

    #[test]
    fn test_bind_operations_overlap() {
        let schedule = vec![
            ScheduleOp {
                op_id: 0,
                earliest: 0,
                latest: 2,
                resource: super::super::ResourceKind::Add,
            },
            ScheduleOp {
                op_id: 1,
                earliest: 1,
                latest: 3,
                resource: super::super::ResourceKind::Add,
            },
        ];

        let bindings = bind_operations(&schedule);
        assert_eq!(bindings.len(), 2);
        // Overlapping ops need different resources.
        assert_ne!(bindings[0], bindings[1]);
    }

    #[test]
    fn test_bind_operations_different_kinds() {
        let schedule = vec![
            ScheduleOp {
                op_id: 0,
                earliest: 0,
                latest: 1,
                resource: super::super::ResourceKind::Add,
            },
            ScheduleOp {
                op_id: 1,
                earliest: 0,
                latest: 1,
                resource: super::super::ResourceKind::Mul,
            },
        ];

        let bindings = bind_operations(&schedule);
        assert_eq!(bindings.len(), 2);
        // Different kinds always get different resources.
        assert_ne!(bindings[0], bindings[1]);
    }

    #[test]
    fn test_count_physical_resources() {
        let bindings = vec![0, 1, 0, 2, 1];
        assert_eq!(count_physical_resources(&bindings), 3);
    }

    #[test]
    fn test_count_physical_resources_empty() {
        let bindings: Vec<u32> = Vec::new();
        assert_eq!(count_physical_resources(&bindings), 1); // max+1 where max=-1 wraps to 0, so 0+1=1
    }

    #[test]
    fn test_validate_bindings_same_group() {
        let bindings = vec![0, 0, 1];
        let sharing_groups = vec![vec![0, 1]];

        assert!(validate_bindings(&bindings, &sharing_groups));
    }

    #[test]
    fn test_validate_bindings_different_group() {
        let bindings = vec![0, 1, 1];
        let sharing_groups = vec![vec![0, 1]];

        assert!(!validate_bindings(&bindings, &sharing_groups));
    }

    #[test]
    fn test_binding_summary() {
        let bindings = vec![0, 1, 0, 2, 1];
        let summary = binding_summary(&bindings);

        assert_eq!(summary.total_resources, 3);
        assert_eq!(summary.resource_ids, vec![0, 1, 2]);
    }

    #[test]
    fn test_binding_debug() {
        let schedule = vec![
            ScheduleOp {
                op_id: 0,
                earliest: 0,
                latest: 1,
                resource: super::super::ResourceKind::Add,
            },
            ScheduleOp {
                op_id: 1,
                earliest: 2,
                latest: 3,
                resource: super::super::ResourceKind::Add,
            },
        ];
        let bindings = vec![0, 0];

        let debug = binding_debug(&schedule, &bindings);
        assert!(debug.contains("Resource 0"));
    }

}
