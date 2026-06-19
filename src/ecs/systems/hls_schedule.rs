#![forbid(unsafe_code)]

use crate::ecs::components::HlsScheduleComponent;
use crate::ecs::{EntityId, Registry};
use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};

/// ECS System: ASAP Scheduling (As Soon As Possible).
///
/// Assigns each operation to the earliest cycle it can execute based on data dependencies.
/// Writes to `HlsScheduleComponent` directly in the Registry.
pub fn hls_asap_schedule_system(registry: &mut Registry) -> Result<(), MirrError> {
    let max_id = registry.active_entities();
    let mut modified = true;

    // Initialize earliest cycles to 0 for all operations and initialize the component
    for i in 0..max_id {
        if registry.hls_dataflow[i].is_some() {
            if registry.hls_schedules[i].is_none() {
                // Determine resource kind based on operation component
                let kind = determine_resource_kind(registry, EntityId(i as u32))
                    .unwrap_or(crate::hls::ResourceKind::Add);

                registry.set_hls_schedule(
                    EntityId(i as u32),
                    HlsScheduleComponent {
                        earliest: 0,
                        latest: 0, // Will be set by ALAP
                        resource: kind,
                    },
                );
            } else {
                if let Some(sched) = &mut registry.hls_schedules[i] {
                    sched.earliest = 0;
                }
            }
        }
    }

    // Find operations with no predecessors (sources)
    let mut sources = Vec::new();
    for i in 0..max_id {
        if let Some(df) = &registry.hls_dataflow[i] {
            if df.predecessors.is_empty() {
                sources.push(i);
            }
        }
    }

    // Iterative ASAP propagation (bounded by max cycles)
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 1024; // NASA P10 bound

    while modified && iterations < MAX_ITERATIONS {
        modified = false;
        iterations += 1;

        for i in 0..max_id {
            if let Some(df) = registry.hls_dataflow[i].clone() {
                // Determine the maximum earliest cycle among predecessors
                let mut max_pred_cycle = 0;
                let mut has_preds = false;

                for &pred in &df.predecessors {
                    has_preds = true;
                    if let Some(pred_sched) = &registry.hls_schedules[pred.0 as usize] {
                        if pred_sched.earliest > max_pred_cycle {
                            max_pred_cycle = pred_sched.earliest;
                        }
                    }
                }

                let target_cycle = if has_preds { max_pred_cycle + 1 } else { 0 };

                if let Some(sched) = &mut registry.hls_schedules[i] {
                    if sched.earliest < target_cycle {
                        sched.earliest = target_cycle;
                        modified = true;
                    }
                }
            }
        }
    }

    if iterations >= MAX_ITERATIONS {
        return Err(mirrcode(
            ErrorCode::HlsSchedulingFailed,
            "ASAP scheduling exceeded max iterations (cycle detected)",
        ));
    }

    Ok(())
}

/// ECS System: ALAP Scheduling (As Late As Possible).
///
/// Assigns each operation to the latest cycle it can execute without violating the target latency.
/// Updates `latest` in `HlsScheduleComponent`.
pub fn hls_alap_schedule_system(
    registry: &mut Registry,
    target_latency: u32,
) -> Result<(), MirrError> {
    let max_id = registry.active_entities();
    let mut modified = true;

    // Initialize latest cycles to target_latency for all operations
    for i in 0..max_id {
        if let Some(sched) = &mut registry.hls_schedules[i] {
            // Check if ASAP already exceeded latency
            if sched.earliest >= target_latency {
                return Err(mirrcode(
                    ErrorCode::HlsSchedulingFailed,
                    format!(
                        "Target latency {} is too tight. Minimum required is {}",
                        target_latency,
                        sched.earliest + 1
                    ),
                ));
            }
            sched.latest = target_latency - 1; // 0-indexed
        }
    }

    // Iterative ALAP propagation (backward from successors)
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 1024;

    while modified && iterations < MAX_ITERATIONS {
        modified = false;
        iterations += 1;

        for i in 0..max_id {
            if let Some(df) = registry.hls_dataflow[i].clone() {
                if df.successors.is_empty() {
                    continue;
                } // Sinks keep target_latency - 1

                // Determine the minimum latest cycle among successors
                let mut min_succ_cycle = u32::MAX;
                let mut has_succs = false;

                for &succ in &df.successors {
                    has_succs = true;
                    if let Some(succ_sched) = &registry.hls_schedules[succ.0 as usize] {
                        if succ_sched.latest < min_succ_cycle {
                            min_succ_cycle = succ_sched.latest;
                        }
                    }
                }

                if has_succs && min_succ_cycle > 0 {
                    let target_cycle = min_succ_cycle - 1;

                    if let Some(sched) = &mut registry.hls_schedules[i] {
                        if sched.latest > target_cycle {
                            sched.latest = target_cycle;
                            modified = true;
                        }
                    }
                }
            }
        }
    }

    if iterations >= MAX_ITERATIONS {
        return Err(mirrcode(
            ErrorCode::HlsSchedulingFailed,
            "ALAP scheduling exceeded max iterations (cycle detected)",
        ));
    }

    // Verify ASAP <= ALAP
    for i in 0..max_id {
        if let Some(sched) = &registry.hls_schedules[i] {
            if sched.earliest > sched.latest {
                return Err(mirrcode(
                    ErrorCode::HlsSchedulingFailed,
                    "Scheduling conflict: ASAP > ALAP",
                ));
            }
        }
    }

    Ok(())
}

fn determine_resource_kind(registry: &Registry, id: EntityId) -> Option<crate::hls::ResourceKind> {
    let idx = id.0 as usize;
    if let Some(binary) = &registry.binary_ops[idx] {
        Some(match binary.op {
            crate::ast::types::BinaryOp::Add => crate::hls::ResourceKind::Add,
            crate::ast::types::BinaryOp::Sub => crate::hls::ResourceKind::Sub,
            crate::ast::types::BinaryOp::Mul => crate::hls::ResourceKind::Mul,
            crate::ast::types::BinaryOp::And | crate::ast::types::BinaryOp::BitwiseAnd => {
                crate::hls::ResourceKind::And
            }
            crate::ast::types::BinaryOp::Or | crate::ast::types::BinaryOp::BitwiseOr => {
                crate::hls::ResourceKind::Or
            }
            crate::ast::types::BinaryOp::Xor => crate::hls::ResourceKind::Xor,
            crate::ast::types::BinaryOp::Eq => crate::hls::ResourceKind::Eq,
            crate::ast::types::BinaryOp::Ne => crate::hls::ResourceKind::Ne,
            crate::ast::types::BinaryOp::Lt => crate::hls::ResourceKind::Lt,
            crate::ast::types::BinaryOp::Le => crate::hls::ResourceKind::Le,
            crate::ast::types::BinaryOp::Gt => crate::hls::ResourceKind::Gt,
            crate::ast::types::BinaryOp::Ge => crate::hls::ResourceKind::Ge,
            crate::ast::types::BinaryOp::Shl => crate::hls::ResourceKind::Shl,
            crate::ast::types::BinaryOp::Shr => crate::hls::ResourceKind::Shr,
        })
    } else {
        registry.unary_ops[idx].as_ref().map(|unary| match unary.op {
            crate::ast::types::UnaryOp::Not => crate::hls::ResourceKind::Not,
            crate::ast::types::UnaryOp::Negate | crate::ast::types::UnaryOp::ReductionOr => {
                crate::hls::ResourceKind::Negate
            }
        })
    }
}
