//! SCC solvers for Phase 4b FIRWINE-complete width inference.
//!
//! Provides two solvers:
//! - `solve_nonexpansive`: Floyd-Warshall fixpoint for nonexpansive SCCs.
//! - `solve_expansive`: Bound inference from annotations/guards for expansive SCCs.
//!
//! Both solvers are iterative with bounded loops (NASA P10 rules #1, #2).

#![forbid(unsafe_code)]

use crate::ast::program::{Guard, SignalDecl};
use crate::ast::types::SignalType;
use super::types::{SccInfo, SccKind, WidthDiag, MAX_SCC_SIZE};

/// Maximum iterations for Floyd-Warshall fixpoint in nonexpansive SCCs.
/// Bounded by SCC_SIZE^2 (worst case for shortest-path convergence).
const MAX_FLOYD_WARSHALL_ITERS: usize = MAX_SCC_SIZE * MAX_SCC_SIZE;

/// Result of solving a single SCC.
pub struct SccSolveResult {
    /// Resolved widths for each signal in the SCC.
    /// Indexed parallel to `SccInfo::signal_indices`.
    pub widths: Vec<u32>,
    /// Diagnostics emitted during solving.
    pub diagnostics: Vec<WidthDiag>,
}

/// Solve a nonexpansive SCC using Floyd-Warshall fixpoint propagation.
///
/// Widths circulate but don't grow. All signals in the SCC must converge
/// to the same width (the max of any declared width among them).
///
/// Bounded: at most MAX_FLOYD_WARSHALL_ITERS iterations.
pub fn solve_nonexpansive(
    scc: &SccInfo,
    signals: &[SignalDecl],
) -> SccSolveResult {
    let n = scc.signal_indices.len();
    let mut diagnostics: Vec<WidthDiag> = Vec::new();

    // Initialize widths from declarations.
    let mut widths: Vec<u32> = Vec::with_capacity(n);
    for &sig_idx in &scc.signal_indices {
        let w = signals.get(sig_idx)
            .map(|s| match s.ty {
                SignalType::Bool => 1u32,
                SignalType::Unsigned(w) => w,
            })
            .unwrap_or(0);
        widths.push(w);
    }

    // Floyd-Warshall fixpoint: propagate max width across the SCC.
    // In a nonexpansive SCC, all signals must have equal width (the max).
    let mut iters = 0usize;
    loop {
        iters += 1;
        if iters > MAX_FLOYD_WARSHALL_ITERS {
            diagnostics.push(WidthDiag::error(
                "nonexpansive SCC solver exceeded iteration budget".to_string(),
            ));
            break;
        }

        let current_max = widths.iter().copied().max().unwrap_or(0);
        let mut changed = false;
        for w in &mut widths {
            if *w < current_max {
                *w = current_max;
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    // Post-solve: check for unresolved nodes.
    for (i, &w) in widths.iter().enumerate() {
        if w == 0 {
            let name = scc.signal_indices.get(i)
                .and_then(|&idx| signals.get(idx))
                .map(|s| s.name.as_str())
                .unwrap_or("unknown");
            diagnostics.push(WidthDiag::error(format!(
                "signal '{}' in nonexpansive SCC has no width anchor \
                 (add an explicit type annotation)", name
            )));
        }
    }

    SccSolveResult { widths, diagnostics }
}

/// Solve an expansive SCC using bound inference.
///
/// Strategy:
/// 1. If the signal has an explicit type annotation, use that width.
/// 2. If the signal is assigned in a reflex gated by `for N cycles` and
///    the RHS is `prev(signal) + k`, infer width from `k * N`.
/// 3. Otherwise, emit a hard error.
///
/// Bounded: iterates once per SCC signal.
pub fn solve_expansive(
    scc: &SccInfo,
    signals: &[SignalDecl],
    guards: &[Guard],
    program: &crate::ast::program::MirrProgram,
) -> SccSolveResult {
    let n = scc.signal_indices.len();
    let mut widths: Vec<u32> = Vec::with_capacity(n);
    let mut diagnostics: Vec<WidthDiag> = Vec::new();

    for &sig_idx in &scc.signal_indices {
        let sig = match signals.get(sig_idx) {
            Some(s) => s,
            None => {
                widths.push(0);
                continue;
            }
        };

        // Strategy 1: Explicit type annotation.
        let declared_width = match sig.ty {
            SignalType::Bool => 1u32,
            SignalType::Unsigned(w) => w,
        };

        // If the signal has a non-default width (> 0), accept it.
        // In MIRR's strict typing, all signals have explicit types,
        // so this is the primary resolution path.
        if declared_width > 0 {
            widths.push(declared_width);
            continue;
        }

        // Strategy 2: Infer from guard bounds.
        let inferred = infer_bound_from_guards(
            &sig.name, program, guards,
        );
        match inferred {
            Some(w) => {
                widths.push(w);
                diagnostics.push(WidthDiag::info(format!(
                    "signal '{}' width inferred as u{} from guard bounds",
                    sig.name, w
                )));
            }
            None => {
                // Strategy 3: Hard error.
                widths.push(0);
                diagnostics.push(WidthDiag::error(format!(
                    "signal '{}' is in an expansive SCC but has no provable \
                     width bound. Add an explicit type annotation or a \
                     bounded temporal guard.", sig.name
                )));
            }
        }
    }

    SccSolveResult { widths, diagnostics }
}

/// Attempt to infer an accumulator bound from temporal guards.
///
/// Looks for a reflex that assigns to `signal_name` with RHS pattern
/// `prev(signal_name) + constant`, gated by a guard with `for N cycles`.
/// If found, the maximum value is `constant * N`, and the width is
/// `min_bits_for(constant * N)`.
///
/// Bounded: iterates over reflexes and guards (finite from parser).
fn infer_bound_from_guards(
    signal_name: &str,
    program: &crate::ast::program::MirrProgram,
    guards: &[Guard],
) -> Option<u32> {
    use crate::ast::expr::Expr;
    use crate::ast::types::BinaryOp;

    for r in &program.module.reflexes {
        for a in &r.assignments {
            if a.target != signal_name {
                continue;
            }

            // Check if RHS is `prev(signal_name) + constant`.
            let increment = match &a.value {
                Expr::Binary {
                    op: BinaryOp::Add,
                    left,
                    right,
                } => {
                    let left_is_prev = matches!(
                        left.as_ref(),
                        Expr::Prev { signal, .. } if signal == signal_name
                    );
                    let right_const = match right.as_ref() {
                        Expr::Literal(crate::ast::types::LiteralValue::Integer(v)) => Some(*v),
                        _ => None,
                    };
                    if left_is_prev {
                        right_const
                    } else {
                        // Check reversed: constant + prev(signal).
                        let right_is_prev = matches!(
                            right.as_ref(),
                            Expr::Prev { signal, .. } if signal == signal_name
                        );
                        let left_const = match left.as_ref() {
                            Expr::Literal(crate::ast::types::LiteralValue::Integer(v)) => {
                                Some(*v)
                            }
                            _ => None,
                        };
                        if right_is_prev { left_const } else { None }
                    }
                }
                _ => None,
            };

            let increment = match increment {
                Some(k) if k > 0 => k,
                _ => continue,
            };

            // Find the guard that gates this reflex and extract cycle count.
            for guard_name in &r.guard_names {
                for g in guards {
                    if g.name == *guard_name && g.cycles > 0 {
                        let max_val = increment.saturating_mul(g.cycles);
                        let bits = super::types::Width::min_bits_for(max_val);
                        if bits.0 <= 64 {
                            return Some(bits.0);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Dispatch to the appropriate solver based on SCC kind.
pub fn solve_scc(
    scc: &SccInfo,
    signals: &[SignalDecl],
    guards: &[Guard],
    program: &crate::ast::program::MirrProgram,
) -> SccSolveResult {
    match scc.kind {
        SccKind::Nonexpansive => solve_nonexpansive(scc, signals),
        SccKind::Expansive => solve_expansive(scc, signals, guards, program),
    }
}
