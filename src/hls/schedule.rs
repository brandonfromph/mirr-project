//! MEGA-12: ASAP/ALAP scheduling for HLS dataflow graphs.
//!
//! ASAP (As Soon As Possible): assigns each operation to the earliest cycle
//! it can execute based on data dependencies.
//!
//! ALAP (As Late As Possible): assigns each operation to the latest cycle
//! it can execute without violating the target latency.
//!
//! Mobility = ALAP - ASAP: operations with mobility > 0 can be scheduled
//! in multiple cycles, enabling resource sharing opportunities.
//!
//! NASA Power-of-10: all loops bounded by graph size (bounded by MAX_HLS_OPERATIONS).

#![forbid(unsafe_code)]

use super::{OpDag, ResourceKind};
use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};

/// Maximum schedule cycles (NASA P10 bound).
pub const MAX_SCHEDULE_CYCLES: u32 = 1024;

/// A scheduled operation with its cycle assignments.
#[derive(Debug, Clone)]
pub struct ScheduleOp {
    /// Index of the operation in the DAG.
    pub op_id: u32,
    /// Earliest possible cycle (ASAP result).
    pub earliest: u32,
    /// Latest possible cycle (ALAP result).
    pub latest: u32,
    /// Resource kind for this operation.
    pub resource: ResourceKind,
}

impl ScheduleOp {
    /// Mobility = latest - earliest. Higher mobility means more sharing opportunities.
    pub fn mobility(&self) -> u32 {
        self.latest.saturating_sub(self.earliest)
    }

    /// Mid-point scheduling: cycle = (earliest + latest) / 2.
    pub fn mid_cycle(&self) -> u32 {
        (self.earliest + self.latest) / 2
    }
}

/// Run ASAP scheduling on a DAG.
///
/// Each operation is scheduled at the earliest cycle it can execute,
/// based on data dependencies. Bounded by graph size.
pub fn asap_schedule(dag: &OpDag) -> Result<Vec<ScheduleOp>, MirrError> {
    let mut schedule: Vec<ScheduleOp> = Vec::new();
    let mut visited: Vec<bool> = vec![false; dag.ops.len()];

    // Process operations with no predecessors first (sources).
    let mut i = 0;
    while i < dag.ops.len() {
        let op = &dag.ops[i];
        if op.predecessors.is_empty() {
            asap_from_node(dag, &mut schedule, &mut visited, op.op_id, 0)?;
        }
        i += 1;
    }

    // Verify all operations were scheduled.
    if schedule.len() != dag.ops.len() {
        return Err(mirrcode(
            ErrorCode::HlsSchedulingFailed,
            "Not all operations could be scheduled (disconnected graph or cycle)",
        ));
    }

    // Sort by op_id for consistent ordering.
    schedule.sort_by_key(|s| s.op_id);

    Ok(schedule)
}

/// Recursively schedule from a node using DFS (bounded by MAX_SCHEDULE_CYCLES).
fn asap_from_node(
    dag: &OpDag,
    schedule: &mut Vec<ScheduleOp>,
    visited: &mut [bool],
    op_id: u32,
    cycle: u32,
) -> Result<(), MirrError> {
    if op_id >= dag.ops.len() as u32 {
        return Ok(());
    }

    let idx = op_id as usize;

    // Already visited — update cycle if this path is earlier.
    if visited[idx] {
        // Find existing entry and update if this cycle is earlier.
        let mut j = 0;
        while j < schedule.len() {
            if schedule[j].op_id == op_id && cycle < schedule[j].earliest {
                schedule[j].earliest = cycle;
                break;
            }
            j += 1;
        }
        return Ok(());
    }

    visited[idx] = true;

    let op = &dag.ops[idx];

    // Compute actual earliest: max of predecessor cycles + 1.
    let mut actual_earliest = cycle;
    let mut pred_idx = 0;
    while pred_idx < op.predecessors.len() {
        let pred_id = op.predecessors[pred_idx];
        if pred_id < dag.ops.len() as u32 {
            // Find predecessor's scheduled cycle.
            let mut j = 0;
            while j < schedule.len() {
                if schedule[j].op_id == pred_id {
                    let pred_cycle = schedule[j].earliest + 1;
                    if pred_cycle > actual_earliest {
                        actual_earliest = pred_cycle;
                    }
                    break;
                }
                j += 1;
            }
        }
        pred_idx += 1;
    }

    if actual_earliest >= MAX_SCHEDULE_CYCLES {
        return Err(mirrcode(ErrorCode::HlsSchedulingFailed, "Schedule exceeds maximum cycles"));
    }

    schedule.push(ScheduleOp {
        op_id,
        earliest: actual_earliest,
        latest: actual_earliest, // Will be updated by ALAP.
        resource: op.kind,
    });

    // Schedule successors.
    let mut succ_idx = 0;
    while succ_idx < op.successors.len() {
        let _ =
            asap_from_node(dag, schedule, visited, op.successors[succ_idx], actual_earliest + 1);
        succ_idx += 1;
    }

    Ok(())
}

/// Run ALAP scheduling on a DAG.
///
/// Each operation is scheduled at the latest cycle it can execute
/// without violating the target latency. Bounded by graph size.
pub fn alap_schedule(dag: &OpDag, latency: u32) -> Result<Vec<ScheduleOp>, MirrError> {
    if latency == 0 || latency > MAX_SCHEDULE_CYCLES {
        return Err(mirrcode(ErrorCode::HlsSchedulingFailed, "Invalid latency: must be 1..=1024"));
    }

    let mut schedule: Vec<ScheduleOp> = Vec::new();
    let mut visited: Vec<bool> = vec![false; dag.ops.len()];

    // Find sink nodes (operations with no successors).
    // Sink nodes can execute at latest at cycle (latency - 1).
    let mut i = 0;
    while i < dag.ops.len() {
        let op = &dag.ops[i];
        if op.successors.is_empty() {
            alap_from_node(dag, &mut schedule, &mut visited, op.op_id, latency.saturating_sub(1))?;
        }
        i += 1;
    }

    // Verify all operations were scheduled.
    if schedule.len() != dag.ops.len() {
        return Err(mirrcode(
            ErrorCode::HlsSchedulingFailed,
            "Not all operations could be scheduled (disconnected graph or cycle)",
        ));
    }

    // Sort by op_id for consistent ordering.
    schedule.sort_by_key(|s| s.op_id);

    Ok(schedule)
}

/// Recursively schedule from a node using DFS for ALAP (bounded by MAX_SCHEDULE_CYCLES).
fn alap_from_node(
    dag: &OpDag,
    schedule: &mut Vec<ScheduleOp>,
    visited: &mut [bool],
    op_id: u32,
    latest_cycle: u32,
) -> Result<(), MirrError> {
    if op_id >= dag.ops.len() as u32 {
        return Ok(());
    }

    let idx = op_id as usize;

    // Already visited — update cycle if this path is later.
    if visited[idx] {
        let mut j = 0;
        while j < schedule.len() {
            if schedule[j].op_id == op_id && latest_cycle > schedule[j].latest {
                schedule[j].latest = latest_cycle;
                break;
            }
            j += 1;
        }
        return Ok(());
    }

    visited[idx] = true;

    let op = &dag.ops[idx];

    // Compute actual latest: min of successor cycles - 1.
    let mut actual_latest = latest_cycle;
    let mut succ_idx = 0;
    while succ_idx < op.successors.len() {
        let succ_id = op.successors[succ_idx];
        if succ_id < dag.ops.len() as u32 {
            let mut j = 0;
            while j < schedule.len() {
                if schedule[j].op_id == succ_id {
                    let succ_cycle = schedule[j].latest;
                    if succ_cycle > 0 {
                        let pred_latest = succ_cycle - 1;
                        if pred_latest < actual_latest {
                            actual_latest = pred_latest;
                        }
                    }
                    break;
                }
                j += 1;
            }
        }
        succ_idx += 1;
    }

    if actual_latest == 0 && !op.predecessors.is_empty() {
        return Err(mirrcode(
            ErrorCode::HlsSchedulingFailed,
            "ALAP scheduling conflict: operation must execute before cycle 0",
        ));
    }

    schedule.push(ScheduleOp {
        op_id,
        earliest: actual_latest, // Will be overwritten by ASAP result.
        latest: actual_latest,
        resource: op.kind,
    });

    // Schedule predecessors (one cycle earlier).
    let mut pred_idx = 0;
    while pred_idx < op.predecessors.len() {
        let _ = alap_from_node(
            dag,
            schedule,
            visited,
            op.predecessors[pred_idx],
            actual_latest.saturating_sub(1),
        );
        pred_idx += 1;
    }

    Ok(())
}

/// Compute mobility for each operation given ASAP and ALAP schedules.
pub fn compute_mobility(asap: &[ScheduleOp], alap: &[ScheduleOp]) -> Vec<u32> {
    let mut mobility: Vec<u32> = Vec::new();

    let mut i = 0;
    while i < asap.len() {
        if i < alap.len() {
            if alap[i].latest >= asap[i].earliest {
                mobility.push(alap[i].latest - asap[i].earliest);
            } else {
                mobility.push(0);
            }
        } else {
            mobility.push(0);
        }
        i += 1;
    }

    mobility
}

/// Generate a schedule summary string for debugging.
pub fn schedule_summary(schedule: &[ScheduleOp]) -> String {
    let mut lines: Vec<String> = Vec::new();

    let mut i = 0;
    while i < schedule.len() {
        let op = &schedule[i];
        let mobility = op.mobility();
        lines.push(format!(
            "Op {}: {} @ [{}..{}] (mobility={})",
            op.op_id, op.resource, op.earliest, op.latest, mobility
        ));
        i += 1;
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asap_single_op() {
        let mut dag = OpDag::new();
        dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]);

        let schedule = asap_schedule(&dag).unwrap();
        assert_eq!(schedule.len(), 1);
        assert_eq!(schedule[0].earliest, 0);
    }

    #[test]
    fn test_asap_chain() {
        let mut dag = OpDag::new();
        let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
        let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8], vec![]).unwrap();
        let c = dag.add_op(ResourceKind::And, 8, vec![16, 8], vec![]).unwrap();
        dag.add_edge(a, b);
        dag.add_edge(b, c);

        let schedule = asap_schedule(&dag).unwrap();
        assert_eq!(schedule.len(), 3);
        assert_eq!(schedule[0].earliest, 0); // a
        assert_eq!(schedule[1].earliest, 1); // b
        assert_eq!(schedule[2].earliest, 2); // c
    }

    #[test]
    fn test_asap_diamond() {
        let mut dag = OpDag::new();
        let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
        let b = dag.add_op(ResourceKind::Mul, 8, vec![8, 8], vec![]).unwrap();
        let c = dag.add_op(ResourceKind::Sub, 8, vec![8, 8], vec![]).unwrap();
        let d = dag.add_op(ResourceKind::Or, 8, vec![8, 8], vec![]).unwrap();
        dag.add_edge(a, c);
        dag.add_edge(b, c);
        dag.add_edge(c, d);

        let schedule = asap_schedule(&dag).unwrap();
        assert_eq!(schedule.len(), 4);
        // a, b at cycle 0; c at cycle 1; d at cycle 2
        assert_eq!(schedule[a as usize].earliest, 0);
        assert_eq!(schedule[b as usize].earliest, 0);
        assert_eq!(schedule[c as usize].earliest, 1);
        assert_eq!(schedule[d as usize].earliest, 2);
    }

    #[test]
    fn test_alap_chain() {
        let mut dag = OpDag::new();
        let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8], vec![]).unwrap();
        let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8], vec![]).unwrap();
        let c = dag.add_op(ResourceKind::And, 8, vec![16, 8], vec![]).unwrap();
        dag.add_edge(a, b);
        dag.add_edge(b, c);

        let schedule = alap_schedule(&dag, 3).unwrap();
        assert_eq!(schedule.len(), 3);
        assert_eq!(schedule[2].latest, 2); // c at cycle 2
        assert_eq!(schedule[1].latest, 1); // b at cycle 1
        assert_eq!(schedule[0].latest, 0); // a at cycle 0
    }

    #[test]
    fn test_alap_invalid_latency() {
        let dag = OpDag::new();
        let result = alap_schedule(&dag, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_mobility_computation() {
        let asap = vec![
            ScheduleOp { op_id: 0, earliest: 0, latest: 0, resource: ResourceKind::Add },
            ScheduleOp { op_id: 1, earliest: 1, latest: 1, resource: ResourceKind::Mul },
        ];
        let alap = vec![
            ScheduleOp { op_id: 0, earliest: 0, latest: 0, resource: ResourceKind::Add },
            ScheduleOp { op_id: 1, earliest: 1, latest: 1, resource: ResourceKind::Mul },
        ];

        let mobility = compute_mobility(&asap, &alap);
        assert_eq!(mobility.len(), 2);
        assert_eq!(mobility[0], 0);
        assert_eq!(mobility[1], 0);
    }

    #[test]
    fn test_schedule_summary() {
        let schedule = vec![
            ScheduleOp { op_id: 0, earliest: 0, latest: 2, resource: ResourceKind::Add },
            ScheduleOp { op_id: 1, earliest: 1, latest: 1, resource: ResourceKind::Mul },
        ];

        let summary = schedule_summary(&schedule);
        assert!(summary.contains("Op 0"));
        assert!(summary.contains("add"));
        assert!(summary.contains("mobility=2"));
    }
}
