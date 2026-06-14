//! Bounded-iteration analyses 1–4: resource bounds, output completeness,
//! guard coverage, and temporal bound.

#![forbid(unsafe_code)]

use crate::ast::types::SignalKind;
use crate::emit::rspu_isa::{TargetSpec, MAX_GUARDS, MAX_INSTRUCTIONS};

use super::types::{
    GuardCoverageResult, OutputCompletenessResult, ResourceBound, TemporalBoundResult,
};
use super::MAX_DEP_NODES;

/// Count hardware resource usage and check against MAX_REGISTERS, MAX_INSTRUCTIONS,
/// MAX_GUARDS. Returns pass=true if all resources fit.
///
/// Bounded: iterates over signals, guards, reflexes.
pub fn check_resource_bounds(
    registry: &crate::ecs::Registry,
    target: &TargetSpec,
) -> ResourceBound {
    let mut regs: u32 = 0;
    let mut guards: u32 = 0;
    let mut reflex_instrs: u32 = 0;
    let mut prop_instrs: u32 = 0;
    let mut max_cycles: u64 = 0;

    for i in 0..registry.names.len() {
        if let Some(kind) = &registry.kinds[i] {
            use crate::ecs::components::EntityKind;
            match kind.0 {
                EntityKind::SIGNAL(_) => regs = regs.saturating_add(1),
                EntityKind::GUARD => {
                    guards = guards.saturating_add(1);
                    if let Some(c) = &registry.cycles[i] {
                        if c.0 > max_cycles {
                            max_cycles = c.0;
                        }
                    }
                }
                EntityKind::REFLEX => {
                    if let Some(r) = &registry.reflex_comps[i] {
                        reflex_instrs = reflex_instrs.saturating_add(r.assignments.len() as u32);
                    }
                }
                EntityKind::PROPERTY => prop_instrs = prop_instrs.saturating_add(1),
                _ => {}
            }
        }
    }

    let signal_instrs = regs;
    let guard_instrs = guards.saturating_mul(3);

    let instructions_estimate = signal_instrs
        .saturating_add(guard_instrs)
        .saturating_add(reflex_instrs)
        .saturating_add(prop_instrs);

    let pass = (regs as usize) <= target.max_registers()
        && (instructions_estimate as usize) <= MAX_INSTRUCTIONS
        && (guards as usize) <= MAX_GUARDS;

    ResourceBound { registers: regs, instructions_estimate, guards, max_cycles, pass }
}

/// Check that every output signal has at least one reflex that assigns to it.
///
/// Bounded: iterates over signals × reflexes.
pub fn check_output_completeness(registry: &crate::ecs::Registry) -> OutputCompletenessResult {
    let mut undriven: Vec<String> = Vec::new();

    for i in 0..registry.names.len() {
        if let (Some(name), Some(kind)) = (&registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::components::EntityKind::SIGNAL(SignalKind::Output) = kind.0 {
                let mut driven = false;

                for r_opt in registry.reflex_comps.iter().flatten() {
                    for a_ent in &r_opt.assignments {
                        if let Some(a) = &registry.assignment_comps[a_ent.0 as usize] {
                            if a.target.0 as usize == i {
                                driven = true;
                                break;
                            }
                        }
                    }
                    if driven {
                        break;
                    }
                }

                if !driven {
                    undriven.push(name.0.clone());
                }
            }
        }
    }

    let pass = undriven.is_empty();
    OutputCompletenessResult { undriven_outputs: undriven, pass }
}

/// Check guard coverage: for each output signal, verify at least one guard driving it exists.
///
/// Bounded: iterates over outputs × reflexes.
pub fn check_guard_coverage(registry: &crate::ecs::Registry) -> GuardCoverageResult {
    let mut covered: u32 = 0;
    let mut total: u32 = 0;

    for i in 0..registry.names.len() {
        if let Some(kind) = &registry.kinds[i] {
            if let crate::ecs::components::EntityKind::SIGNAL(SignalKind::Output) = kind.0 {
                total += 1;
                let mut has_guard = false;

                for r_opt in registry.reflex_comps.iter().flatten() {
                    let mut drives_this = false;
                    for a_ent in &r_opt.assignments {
                        if let Some(a) = &registry.assignment_comps[a_ent.0 as usize] {
                            if a.target.0 as usize == i {
                                drives_this = true;
                                break;
                            }
                        }
                    }

                    if drives_this && !r_opt.guards.is_empty() {
                        has_guard = true;
                        break;
                    }
                }

                if has_guard {
                    covered += 1;
                }
            }
        }
    }

    let pass = covered >= total;
    GuardCoverageResult { covered_outputs: covered, total_outputs: total, pass }
}

/// Compute worst-case temporal latency: max guard cycles + max prev delay.
///
/// Bounded: iterates over guards and reflexes.
pub fn check_temporal_bound(registry: &crate::ecs::Registry) -> TemporalBoundResult {
    let mut max_guard_cycles: u64 = 0;

    for i in 0..registry.names.len() {
        if let Some(kind) = &registry.kinds[i] {
            if let crate::ecs::components::EntityKind::GUARD = kind.0 {
                if let Some(c) = &registry.cycles[i] {
                    if c.0 > max_guard_cycles {
                        max_guard_cycles = c.0;
                    }
                }
            }
        }
    }

    let mut max_prev_delay: u64 = 0;
    for r_opt in registry.reflex_comps.iter().flatten() {
        for a_ent in &r_opt.assignments {
            if let Some(a) = &registry.assignment_comps[a_ent.0 as usize] {
                let delay = max_prev_in_ecs_expr(a.value, registry);
                if delay > max_prev_delay {
                    max_prev_delay = delay;
                }
            }
        }
    }

    for i in 0..registry.names.len() {
        if let Some(kind) = &registry.kinds[i] {
            if let crate::ecs::components::EntityKind::GUARD = kind.0 {
                if let Some(cond_comp) = &registry.conditions[i] {
                    let delay = max_prev_in_ecs_expr(cond_comp.0, registry);
                    if delay > max_prev_delay {
                        max_prev_delay = delay;
                    }
                }
            }
        }
    }

    let worst_case_latency = max_guard_cycles.saturating_add(max_prev_delay);
    TemporalBoundResult { max_guard_cycles, max_prev_delay, worst_case_latency, pass: true }
}

/// Bounded iteration over an ECS expression tree to find the maximum `prev` delay.
fn max_prev_in_ecs_expr(root: crate::ecs::EntityId, registry: &crate::ecs::Registry) -> u64 {
    let mut max_delay = 0u64;
    let mut stack = Vec::new();
    stack.push(root);

    let mut visited = 0usize;
    while let Some(ent) = stack.pop() {
        visited += 1;
        if visited >= MAX_DEP_NODES {
            break;
        }

        let i = ent.0 as usize;
        if i >= registry.names.len() {
            continue;
        }

        if let Some(p) = &registry.prev_ops[i] {
            if p.delay > max_delay {
                max_delay = p.delay;
            }
            stack.push(p.signal);
        } else if let Some(b) = &registry.binary_ops[i] {
            stack.push(b.left);
            stack.push(b.right);
        } else if let Some(u) = &registry.unary_ops[i] {
            stack.push(u.operand);
        } else if let Some(m) = &registry.muxes[i] {
            stack.push(m.select);
            stack.push(m.true_val);
            stack.push(m.false_val);
        } else if let Some(sl) = &registry.struct_literals[i] {
            for f in &sl.fields {
                stack.push(f.1);
            }
        } else if let Some(al) = &registry.array_literals[i] {
            for &el in &al.0 {
                stack.push(el);
            }
        } else if let Some(ai) = &registry.array_indices[i] {
            stack.push(ai.array);
            stack.push(ai.index);
        } else if let Some(fa) = &registry.field_accesses[i] {
            stack.push(fa.object);
        }
    }
    max_delay
}
