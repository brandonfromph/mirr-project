//! Bounded-iteration analyses 1–4: resource bounds, output completeness,
//! guard coverage, and temporal bound.

#![forbid(unsafe_code)]

use crate::ast::program::Module;
use crate::ast::types::SignalKind;
use crate::emit::rspu_isa::{MAX_GUARDS, MAX_INSTRUCTIONS, MAX_REGISTERS};

use super::types::{
    GuardCoverageResult, OutputCompletenessResult, ResourceBound, TemporalBoundResult,
};
use super::{MAX_DEP_NODES, MAX_REFLEXES, MAX_SIGNALS};

/// Count hardware resource usage and check against MAX_REGISTERS, MAX_INSTRUCTIONS,
/// MAX_GUARDS. Returns pass=true if all resources fit.
///
/// Bounded: iterates over signals (≤ MAX_SIGNALS), guards, reflexes.
pub fn check_resource_bounds(module: &Module) -> ResourceBound {
    let mut regs: u32 = 0;
    let mut i = 0;
    while i < module.signals.len() && i < MAX_SIGNALS {
        regs = regs.saturating_add(1);
        i += 1;
    }

    let guards = module.guards.len().min(MAX_SIGNALS) as u32;

    let signal_instrs = regs;
    let guard_instrs = guards.saturating_mul(3);

    let mut reflex_instrs: u32 = 0;
    let mut ri = 0;
    while ri < module.reflexes.len() && ri < MAX_REFLEXES {
        reflex_instrs = reflex_instrs
            .saturating_add(module.reflexes[ri].assignments.len().min(MAX_SIGNALS) as u32);
        ri += 1;
    }

    let prop_instrs = module.properties.len().min(MAX_SIGNALS) as u32;
    let instructions_estimate = signal_instrs
        .saturating_add(guard_instrs)
        .saturating_add(reflex_instrs)
        .saturating_add(prop_instrs);

    let mut max_cycles: u64 = 0;
    let mut gi = 0;
    while gi < module.guards.len() && gi < MAX_SIGNALS {
        if module.guards[gi].cycles > max_cycles {
            max_cycles = module.guards[gi].cycles;
        }
        gi += 1;
    }

    let pass = (regs as usize) <= MAX_REGISTERS
        && (instructions_estimate as usize) <= MAX_INSTRUCTIONS
        && (guards as usize) <= MAX_GUARDS;

    ResourceBound { registers: regs, instructions_estimate, guards, max_cycles, pass }
}

/// Check that every output signal has at least one reflex that assigns to it.
///
/// Bounded: iterates over signals (≤ MAX_SIGNALS) × reflexes (≤ MAX_REFLEXES).
pub fn check_output_completeness(module: &Module) -> OutputCompletenessResult {
    let mut undriven: Vec<String> = Vec::new();

    let mut si = 0;
    while si < module.signals.len() && si < MAX_SIGNALS {
        if module.signals[si].kind == SignalKind::Output {
            let name = &module.signals[si].name;
            let mut driven = false;

            let mut ri = 0;
            while ri < module.reflexes.len() && ri < MAX_REFLEXES {
                let mut ai = 0;
                while ai < module.reflexes[ri].assignments.len() && ai < MAX_REFLEXES {
                    if module.reflexes[ri].assignments[ai].target == *name {
                        driven = true;
                    }
                    ai += 1;
                }
                ri += 1;
            }

            if !driven {
                undriven.push(name.clone());
            }
        }
        si += 1;
    }

    let pass = undriven.is_empty();
    OutputCompletenessResult { undriven_outputs: undriven, pass }
}

/// Check guard coverage: for each output signal, verify at least one guard driving it exists.
///
/// Bounded: iterates over outputs (≤ MAX_SIGNALS) × reflexes (≤ MAX_REFLEXES).
pub fn check_guard_coverage(module: &Module) -> GuardCoverageResult {
    let mut covered: u32 = 0;
    let mut total: u32 = 0;

    let mut si = 0;
    while si < module.signals.len() && si < MAX_SIGNALS {
        if module.signals[si].kind == SignalKind::Output {
            total += 1;
            let name = &module.signals[si].name;

            let mut has_guard = false;
            let mut ri = 0;
            while ri < module.reflexes.len() && ri < MAX_REFLEXES {
                let mut drives_this = false;
                let mut ai = 0;
                while ai < module.reflexes[ri].assignments.len() && ai < MAX_REFLEXES {
                    if module.reflexes[ri].assignments[ai].target == *name {
                        drives_this = true;
                    }
                    ai += 1;
                }

                if drives_this && !module.reflexes[ri].guard_names.is_empty() {
                    has_guard = true;
                }
                ri += 1;
            }

            if has_guard {
                covered += 1;
            }
        }
        si += 1;
    }

    let pass = covered >= total;
    GuardCoverageResult { covered_outputs: covered, total_outputs: total, pass }
}

/// Compute worst-case temporal latency: max guard cycles + max prev delay.
///
/// Bounded: iterates over guards (≤ MAX_SIGNALS) and reflexes (≤ MAX_REFLEXES).
pub fn check_temporal_bound(module: &Module) -> TemporalBoundResult {
    let mut max_guard_cycles: u64 = 0;
    let mut gi = 0;
    while gi < module.guards.len() && gi < MAX_SIGNALS {
        if module.guards[gi].cycles > max_guard_cycles {
            max_guard_cycles = module.guards[gi].cycles;
        }
        gi += 1;
    }

    let mut max_prev_delay: u64 = 0;
    let mut ri = 0;
    while ri < module.reflexes.len() && ri < MAX_REFLEXES {
        let mut ai = 0;
        while ai < module.reflexes[ri].assignments.len() && ai < MAX_REFLEXES {
            let delay = max_prev_in_expr(&module.reflexes[ri].assignments[ai].value);
            if delay > max_prev_delay {
                max_prev_delay = delay;
            }
            ai += 1;
        }
        ri += 1;
    }

    let mut gi2 = 0;
    while gi2 < module.guards.len() && gi2 < MAX_SIGNALS {
        let delay = max_prev_in_expr(&module.guards[gi2].condition);
        if delay > max_prev_delay {
            max_prev_delay = delay;
        }
        gi2 += 1;
    }

    let worst_case_latency = max_guard_cycles.saturating_add(max_prev_delay);
    TemporalBoundResult { max_guard_cycles, max_prev_delay, worst_case_latency, pass: true }
}

/// Find the maximum `Prev` delay in an expression tree.
///
/// Bounded: explicit stack with MAX_DEP_NODES limit.
pub(super) fn max_prev_in_expr(expr: &crate::ast::Expr) -> u64 {
    let mut max_delay: u64 = 0;
    let mut stack: Vec<&crate::ast::Expr> = Vec::new();
    stack.push(expr);

    let mut visited = 0usize;
    while let Some(e) = stack.pop() {
        visited += 1;
        if visited > MAX_DEP_NODES {
            break;
        }
        match e {
            crate::ast::Expr::Prev { delay, .. } if *delay > max_delay => {
                max_delay = *delay;
            }
            crate::ast::Expr::Unary { operand, .. } => {
                stack.push(operand);
            }
            crate::ast::Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            _ => {}
        }
    }
    max_delay
}
