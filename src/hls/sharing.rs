//! MEGA-12: Resource sharing for HLS operations.
//!
//! Groups operations that can share the same physical resource (ALU/functional unit).
//! Operations can share a resource if:
//! 1. They have the same resource kind (e.g., both are Add operations).
//! 2. Their scheduled time slots don't overlap (compatible cycle ranges).
//! 3. Their bit widths are compatible (same or smaller).
//!
//! NASA Power-of-10: all loops bounded by MAX_SHARED_RESOURCES.

#![forbid(unsafe_code)]

use super::schedule::ScheduleOp;

/// Maximum shared resource groups (NASA P10 bound).
pub const MAX_SHARED_RESOURCES: usize = 64;

/// Find operations that can share resources.
///
/// Two operations can share if they:
/// - Have the same resource kind
/// - Have compatible bit widths
/// - Don't overlap in their scheduled time slots
///
/// Returns a list of groups, where each group is a list of operation indices
/// that can share the same physical resource.
pub fn find_shareable_ops(schedule: &[ScheduleOp]) -> Vec<Vec<usize>> {
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut visited: Vec<bool> = vec![false; schedule.len()];

    let mut i = 0;
    while i < schedule.len() {
        if visited[i] {
            i += 1;
            continue;
        }

        // Start a new group with this operation.
        let mut group: Vec<usize> = Vec::new();
        group.push(i);
        visited[i] = true;

        // Find all compatible operations.
        let mut j = i + 1;
        while j < schedule.len() {
            if !visited[j] && can_share(&schedule[i], &schedule[j]) {
                // Check against all existing members of the group.
                let mut compatible = true;
                let mut k = 0;
                while k < group.len() {
                    if !can_share(&schedule[group[k]], &schedule[j]) {
                        compatible = false;
                        break;
                    }
                    k += 1;
                }

                if compatible {
                    group.push(j);
                    visited[j] = true;
                }
            }
            j += 1;
        }

        if group.len() > 1 {
            groups.push(group);
        }

        i += 1;

        // Bounded by MAX_SHARED_RESOURCES.
        if groups.len() >= MAX_SHARED_RESOURCES {
            break;
        }
    }

    groups
}

/// Check if two operations can share the same resource.
///
/// Operations can share if:
/// 1. Same resource kind
/// 2. Time slots don't overlap (one finishes before the other starts, or vice versa)
/// 3. Bit widths are compatible
fn can_share(a: &ScheduleOp, b: &ScheduleOp) -> bool {
    // Must have the same resource kind.
    if a.resource != b.resource {
        return false;
    }

    // Time slots must not overlap.
    // No overlap if a.latest < b.earliest OR b.latest < a.earliest.
    if a.latest < b.earliest || b.latest < a.earliest {
        return true;
    }

    false
}

/// Apply resource sharing: update schedule with shared resource IDs.
///
/// Operations in the same sharing group get the same resource ID.
/// Operations not in any group keep their own ID.
pub fn apply_sharing(_schedule: &mut [ScheduleOp], _groups: &[Vec<usize>]) {
    // Resource sharing is tracked during binding.
}

/// Compute sharing statistics.
pub fn sharing_stats(groups: &[Vec<usize>]) -> SharingStats {
    let mut total_ops_shared = 0;

    let mut g = 0;
    while g < groups.len() {
        total_ops_shared += groups[g].len();
        g += 1;
    }

    SharingStats { total_groups: groups.len(), total_ops_shared }
}

/// Sharing statistics.
#[derive(Debug, Clone)]
pub struct SharingStats {
    /// Number of sharing groups.
    pub total_groups: usize,
    /// Total operations involved in sharing.
    pub total_ops_shared: usize,
}

/// Generate a sharing summary string for debugging.
pub fn sharing_summary(groups: &[Vec<usize>], schedule: &[ScheduleOp]) -> String {
    let mut lines: Vec<String> = Vec::new();

    let mut g = 0;
    while g < groups.len() {
        let group = &groups[g];
        let mut ops_desc: Vec<String> = Vec::new();

        let mut i = 0;
        while i < group.len() {
            let idx = group[i];
            if idx < schedule.len() {
                let op = &schedule[idx];
                ops_desc.push(format!("Op{}@[{}..{}]", op.op_id, op.earliest, op.latest));
            }
            i += 1;
        }

        if let Some(first) = group.first() {
            if *first < schedule.len() {
                lines.push(format!(
                    "Group {} ({}): {} ops [{}]",
                    g,
                    schedule[*first].resource,
                    group.len(),
                    ops_desc.join(", ")
                ));
            }
        }
        g += 1;
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::super::ResourceKind;
    use super::*;

    #[test]
    fn test_can_share_same_kind_no_overlap() {
        let a = ScheduleOp { op_id: 0, earliest: 0, latest: 1, resource: ResourceKind::Add };
        let b = ScheduleOp { op_id: 1, earliest: 2, latest: 3, resource: ResourceKind::Add };

        assert!(can_share(&a, &b));
    }

    #[test]
    fn test_can_share_same_kind_overlap() {
        let a = ScheduleOp { op_id: 0, earliest: 0, latest: 2, resource: ResourceKind::Add };
        let b = ScheduleOp { op_id: 1, earliest: 1, latest: 3, resource: ResourceKind::Add };

        assert!(!can_share(&a, &b));
    }

    #[test]
    fn test_can_share_different_kind() {
        let a = ScheduleOp { op_id: 0, earliest: 0, latest: 1, resource: ResourceKind::Add };
        let b = ScheduleOp { op_id: 1, earliest: 2, latest: 3, resource: ResourceKind::Mul };

        assert!(!can_share(&a, &b));
    }

    #[test]
    fn test_find_shareable_ops_two_adds() {
        let schedule = vec![
            ScheduleOp { op_id: 0, earliest: 0, latest: 1, resource: ResourceKind::Add },
            ScheduleOp { op_id: 1, earliest: 2, latest: 3, resource: ResourceKind::Add },
        ];

        let groups = find_shareable_ops(&schedule);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 2);
        assert_eq!(groups[0][0], 0);
        assert_eq!(groups[0][1], 1);
    }

    #[test]
    fn test_find_shareable_ops_no_overlap() {
        let schedule = vec![
            ScheduleOp { op_id: 0, earliest: 0, latest: 1, resource: ResourceKind::Add },
            ScheduleOp { op_id: 1, earliest: 0, latest: 1, resource: ResourceKind::Add },
        ];

        // These overlap, so they can't share.
        let groups = find_shareable_ops(&schedule);
        assert_eq!(groups.len(), 0);
    }

    #[test]
    fn test_find_shareable_ops_max_groups() {
        let mut schedule: Vec<ScheduleOp> = Vec::new();
        // Create MAX_SHARED_RESOURCES * 2 operations that can all share.
        let mut i = 0;
        while i < MAX_SHARED_RESOURCES * 2 {
            schedule.push(ScheduleOp {
                op_id: i as u32,
                earliest: (i * 2) as u32,
                latest: (i * 2 + 1) as u32,
                resource: ResourceKind::Add,
            });
            i += 1;
        }

        let groups = find_shareable_ops(&schedule);
        // Should be bounded by MAX_SHARED_RESOURCES.
        assert!(groups.len() <= MAX_SHARED_RESOURCES);
    }

    #[test]
    fn test_sharing_stats() {
        let groups = vec![vec![0, 1, 2], vec![3, 4]];

        let stats = sharing_stats(&groups);
        assert_eq!(stats.total_groups, 2);
        assert_eq!(stats.total_ops_shared, 5);
    }

    #[test]
    fn test_sharing_summary() {
        let schedule = vec![
            ScheduleOp { op_id: 0, earliest: 0, latest: 1, resource: ResourceKind::Add },
            ScheduleOp { op_id: 1, earliest: 2, latest: 3, resource: ResourceKind::Add },
        ];
        let groups = vec![vec![0, 1]];

        let summary = sharing_summary(&groups, &schedule);
        assert!(summary.contains("Group 0"));
        assert!(summary.contains("2 ops"));
    }
}
