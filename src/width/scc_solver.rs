//! SCC solvers for Phase 4b FIRWINE-complete width inference.
//!
//! Provides two solvers:
//! - `solve_nonexpansive`: Floyd-Warshall fixpoint for nonexpansive SCCs.
//! - `solve_expansive`: Bound inference from annotations/guards for expansive SCCs.
//!
//! Both solvers are iterative with bounded loops (NASA P10 rules #1, #2).

#![forbid(unsafe_code)]

use super::types::{SccInfo, SccKind, WidthDiag, MAX_SCC_SIZE};
use crate::ecs::components::EntityId;
use crate::ecs::registry::Registry;
use serde::Serialize;

/// Maximum iterations for Floyd-Warshall fixpoint in nonexpansive SCCs.
/// Bounded by SCC_SIZE^2 (worst case for shortest-path convergence).
const MAX_FLOYD_WARSHALL_ITERS: usize = MAX_SCC_SIZE * MAX_SCC_SIZE;

/// Result of solving a single SCC.
#[derive(Debug, Clone, Serialize)]
pub struct SccSolveResult {
    /// Resolved widths for each signal in the SCC.
    /// Indexed parallel to `SccInfo::signals`.
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
pub fn solve_nonexpansive(scc: &SccInfo, registry: &Registry) -> SccSolveResult {
    let n = scc.signals.len();
    let mut diagnostics: Vec<WidthDiag> = Vec::new();

    // Initialize widths from declarations.
    let mut widths: Vec<u32> = Vec::with_capacity(n);
    for &sig_id in &scc.signals {
        let w = registry.types[sig_id.0 as usize].as_ref().map(|tc| tc.0.core.width()).unwrap_or(0);
        widths.push(w);
    }

    // Floyd-Warshall fixpoint: propagate max width across the SCC.
    // In a nonexpansive SCC, all signals must have equal width (the max).
    let mut iters = 0usize;
    loop {
        iters += 1;
        if iters > MAX_FLOYD_WARSHALL_ITERS {
            diagnostics.push(
                WidthDiag::error(format!(
                    "{} nonexpansive SCC solver exceeded iteration budget",
                    crate::error_codes::ec(508)
                ))
                .with_code("E508"),
            );
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
            let name = scc
                .signals
                .get(i)
                .and_then(|&id| registry.names[id.0 as usize].as_ref())
                .map(|s| s.0.as_str())
                .unwrap_or("unknown");
            diagnostics.push(
                WidthDiag::error(format!(
                    "{} signal '{}' in nonexpansive SCC has no width anchor \
                 (add an explicit type annotation)",
                    crate::error_codes::ec(509),
                    name
                ))
                .with_code("E509")
                .with_signal(name),
            );
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
pub fn solve_expansive(scc: &SccInfo, registry: &Registry) -> SccSolveResult {
    let n = scc.signals.len();
    let mut widths: Vec<u32> = Vec::with_capacity(n);
    let mut diagnostics: Vec<WidthDiag> = Vec::new();

    for &sig_id in &scc.signals {
        // Strategy 1: Explicit type annotation.
        let declared_width =
            registry.types[sig_id.0 as usize].as_ref().map(|tc| tc.0.core.width()).unwrap_or(0);

        if declared_width > 0 {
            widths.push(declared_width);
            continue;
        }

        let sig_name =
            registry.names[sig_id.0 as usize].as_ref().map(|n| n.0.as_str()).unwrap_or("unknown");

        // Strategy 2: Infer from guard bounds.
        let inferred = infer_bound_from_guards(sig_id, registry);
        match inferred {
            Some(w) => {
                widths.push(w);
                diagnostics.push(
                    WidthDiag::info(format!(
                        "signal '{}' width inferred as u{} from guard bounds",
                        sig_name, w
                    ))
                    .with_signal(sig_name),
                );
            }
            None => {
                // Strategy 3: Hard error.
                widths.push(0);
                diagnostics.push(
                    WidthDiag::error(format!(
                        "{} signal '{}' is in an expansive SCC but has no provable \
                     width bound. Add an explicit type annotation or a \
                     bounded temporal guard.",
                        crate::error_codes::ec(510),
                        sig_name
                    ))
                    .with_code("E510")
                    .with_signal(sig_name),
                );
            }
        }
    }

    SccSolveResult { widths, diagnostics }
}

/// Attempt to infer an accumulator bound from temporal guards.
///
/// Looks for a reflex that assigns to `signal_id` with RHS pattern
/// `prev(signal_id) + constant`, gated by a guard with `for N cycles`.
/// If found, the maximum value is `constant * N`, and the width is
/// `min_bits_for(constant * N)`.
fn infer_bound_from_guards(signal_id: EntityId, registry: &Registry) -> Option<u32> {
    for reflex in registry.reflex_comps.iter().flatten() {
        for &assignment_id in &reflex.assignments {
            let a = match &registry.assignment_comps[assignment_id.0 as usize] {
                Some(a) => a,
                None => continue,
            };

            if a.target != signal_id {
                continue;
            }

            // Check if RHS is `prev(signal) + constant`
            let bin = match &registry.binary_ops[a.value.0 as usize] {
                Some(b) if b.op == crate::ast::types::BinaryOp::Add => b,
                _ => continue,
            };

            let is_prev = |id: EntityId| -> bool {
                registry.prev_ops[id.0 as usize].as_ref().is_some_and(|p| {
                    if p.signal == signal_id {
                        true
                    } else if let Some(sig_ref) = &registry.signal_refs[p.signal.0 as usize] {
                        sig_ref.0 == signal_id
                    } else {
                        false
                    }
                })
            };

            let get_const = |id: EntityId| -> Option<u64> {
                registry.literals[id.0 as usize].as_ref().and_then(|l| match l.0 {
                    crate::ast::types::LiteralValue::Integer(v) => Some(v),
                    _ => None,
                })
            };

            let increment = if is_prev(bin.left) {
                get_const(bin.right)
            } else if is_prev(bin.right) {
                get_const(bin.left)
            } else {
                None
            };

            let increment = match increment {
                Some(k) if k > 0 => k,
                _ => continue,
            };

            // Find the guard that gates this reflex and extract cycle count.
            for &guard_id in &reflex.guards {
                if let Some(cycles) = &registry.cycles[guard_id.0 as usize] {
                    if cycles.0 > 0 {
                        // SAFE: Check for overflow
                        let max_val = increment.checked_mul(cycles.0)?;
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
pub fn solve_scc(scc: &SccInfo, registry: &Registry) -> SccSolveResult {
    match scc.kind {
        SccKind::Nonexpansive => solve_nonexpansive(scc, registry),
        SccKind::Expansive => solve_expansive(scc, registry),
    }
}

// --- LEGACY AST IMPLEMENTATION (KEPT FOR REFERENCE & SAFE MIGRATION) ---

// //! SCC solvers for Phase 4b FIRWINE-complete width inference.
// //!
// //! Provides two solvers:
// //! - `solve_nonexpansive`: Floyd-Warshall fixpoint for nonexpansive SCCs.
// //! - `solve_expansive`: Bound inference from annotations/guards for expansive SCCs.
// //!
// //! Both solvers are iterative with bounded loops (NASA P10 rules #1, #2).
//
// #![forbid(unsafe_code)]
//
// use super::types::{SccInfo, SccKind, WidthDiag, MAX_SCC_SIZE};
// use crate::ast::program::{Guard, SignalDecl};
// use serde::Serialize;
//
// /// Maximum iterations for Floyd-Warshall fixpoint in nonexpansive SCCs.
// /// Bounded by SCC_SIZE^2 (worst case for shortest-path convergence).
// const MAX_FLOYD_WARSHALL_ITERS: usize = MAX_SCC_SIZE * MAX_SCC_SIZE;
//
// /// Result of solving a single SCC.
// #[derive(Debug, Clone, Serialize)]
// pub struct SccSolveResult {
//     /// Resolved widths for each signal in the SCC.
//     /// Indexed parallel to `SccInfo::signal_indices`.
//     pub widths: Vec<u32>,
//     /// Diagnostics emitted during solving.
//     pub diagnostics: Vec<WidthDiag>,
// }
//
// /// Solve a nonexpansive SCC using Floyd-Warshall fixpoint propagation.
// ///
// /// Widths circulate but don't grow. All signals in the SCC must converge
// /// to the same width (the max of any declared width among them).
// ///
// /// Bounded: at most MAX_FLOYD_WARSHALL_ITERS iterations.
// pub fn solve_nonexpansive(scc: &SccInfo, signals: &[SignalDecl]) -> SccSolveResult {
//     let n = scc.signal_indices.len();
//     let mut diagnostics: Vec<WidthDiag> = Vec::new();
//
//     // Initialize widths from declarations.
//     let mut widths: Vec<u32> = Vec::with_capacity(n);
//     for &sig_idx in &scc.signal_indices {
//         let w = signals.get(sig_idx).map(|s| s.ty.signal_type().width()).unwrap_or(0);
//         widths.push(w);
//     }
//
//     // Floyd-Warshall fixpoint: propagate max width across the SCC.
//     // In a nonexpansive SCC, all signals must have equal width (the max).
//     let mut iters = 0usize;
//     loop {
//         iters += 1;
//         if iters > MAX_FLOYD_WARSHALL_ITERS {
//             diagnostics.push(
//                 WidthDiag::error(format!(
//                     "{} nonexpansive SCC solver exceeded iteration budget",
//                     crate::error_codes::ec(508)
//                 ))
//                 .with_code("E508"),
//             );
//             break;
//         }
//
//         let current_max = widths.iter().copied().max().unwrap_or(0);
//         let mut changed = false;
//         for w in &mut widths {
//             if *w < current_max {
//                 *w = current_max;
//                 changed = true;
//             }
//         }
//
//         if !changed {
//             break;
//         }
//     }
//
//     // Post-solve: check for unresolved nodes.
//     for (i, &w) in widths.iter().enumerate() {
//         if w == 0 {
//             let name = scc
//                 .signal_indices
//                 .get(i)
//                 .and_then(|&idx| signals.get(idx))
//                 .map(|s| s.name.as_str())
//                 .unwrap_or("unknown");
//             diagnostics.push(
//                 WidthDiag::error(format!(
//                     "{} signal '{}' in nonexpansive SCC has no width anchor \
//                  (add an explicit type annotation)",
//                     crate::error_codes::ec(509),
//                     name
//                 ))
//                 .with_code("E509")
//                 .with_signal(name),
//             );
//         }
//     }
//
//     SccSolveResult { widths, diagnostics }
// }
//
// /// Solve an expansive SCC using bound inference.
// ///
// /// Strategy:
// /// 1. If the signal has an explicit type annotation, use that width.
// /// 2. If the signal is assigned in a reflex gated by `for N cycles` and
// ///    the RHS is `prev(signal) + k`, infer width from `k * N`.
// /// 3. Otherwise, emit a hard error.
// ///
// /// Bounded: iterates once per SCC signal.
// pub fn solve_expansive(
//     scc: &SccInfo,
//     signals: &[SignalDecl],
//     guards: &[Guard],
//     program: &crate::ast::program::MirrProgram,
// ) -> SccSolveResult {
//     let n = scc.signal_indices.len();
//     let mut widths: Vec<u32> = Vec::with_capacity(n);
//     let mut diagnostics: Vec<WidthDiag> = Vec::new();
//
//     for &sig_idx in &scc.signal_indices {
//         let sig = match signals.get(sig_idx) {
//             Some(s) => s,
//             None => {
//                 widths.push(0);
//                 continue;
//             }
//         };
//
//         // Strategy 1: Explicit type annotation.
//         let declared_width = sig.ty.signal_type().width();
//
//         // If the signal has a non-default width (> 0), accept it.
//         // In MIRR's strict typing, all signals have explicit types,
//         // so this is the primary resolution path.
//         if declared_width > 0 {
//             widths.push(declared_width);
//             continue;
//         }
//
//         // Strategy 2: Infer from guard bounds.
//         let inferred = infer_bound_from_guards(&sig.name, program, guards);
//         match inferred {
//             Some(w) => {
//                 widths.push(w);
//                 diagnostics.push(
//                     WidthDiag::info(format!(
//                         "signal '{}' width inferred as u{} from guard bounds",
//                         sig.name, w
//                     ))
//                     .with_signal(&sig.name),
//                 );
//             }
//             None => {
//                 // Strategy 3: Hard error.
//                 widths.push(0);
//                 diagnostics.push(
//                     WidthDiag::error(format!(
//                         "{} signal '{}' is in an expansive SCC but has no provable \
//                      width bound. Add an explicit type annotation or a \
//                      bounded temporal guard.",
//                         crate::error_codes::ec(510),
//                         sig.name
//                     ))
//                     .with_code("E510")
//                     .with_signal(&sig.name),
//                 );
//             }
//         }
//     }
//
//     SccSolveResult { widths, diagnostics }
// }
//
// /// Attempt to infer an accumulator bound from temporal guards.
// ///
// /// Looks for a reflex that assigns to `signal_name` with RHS pattern
// /// `prev(signal_name) + constant`, gated by a guard with `for N cycles`.
// /// If found, the maximum value is `constant * N`, and the width is
// /// `min_bits_for(constant * N)`.
// ///
// /// Bounded: iterates over reflexes and guards (finite from parser).
// fn infer_bound_from_guards(
//     signal_name: &str,
//     program: &crate::ast::program::MirrProgram,
//     guards: &[Guard],
// ) -> Option<u32> {
//     use crate::ast::expr::Expr;
//     use crate::ast::types::BinaryOp;
//
//     for r in &program.module.reflexes {
//         for a in &r.assignments {
//             if a.target != signal_name {
//                 continue;
//             }
//
//             // Check if RHS is `prev(signal_name) + constant`.
//             let increment = match &a.value {
//                 Expr::Binary { op: BinaryOp::Add, left, right } => {
//                     let left_is_prev = matches!(
//                         left.as_ref(),
//                         Expr::Prev { signal, .. } if signal == signal_name
//                     );
//                     let right_const = match right.as_ref() {
//                         Expr::Literal(crate::ast::types::LiteralValue::Integer(v)) => Some(*v),
//                         _ => None,
//                     };
//                     if left_is_prev {
//                         right_const
//                     } else {
//                         // Check reversed: constant + prev(signal).
//                         let right_is_prev = matches!(
//                             right.as_ref(),
//                             Expr::Prev { signal, .. } if signal == signal_name
//                         );
//                         let left_const = match left.as_ref() {
//                             Expr::Literal(crate::ast::types::LiteralValue::Integer(v)) => Some(*v),
//                             _ => None,
//                         };
//                         if right_is_prev {
//                             left_const
//                         } else {
//                             None
//                         }
//                     }
//                 }
//                 _ => None,
//             };
//
//             let increment = match increment {
//                 Some(k) if k > 0 => k,
//                 _ => continue,
//             };
//
//             // Find the guard that gates this reflex and extract cycle count.
//             for guard_name in &r.guard_names {
//                 for g in guards {
//                     if g.name == *guard_name && g.cycles > 0 {
//                         // SAFE: Check for overflow instead of saturating silently.
//                         // If the product exceeds u64, we can't represent it in our 64-bit width system.
//                         let max_val = match increment.checked_mul(g.cycles) {
//                             Some(v) => v,
//                             None => {
//                                 // Bit-width would exceed u64 limit.
//                                 return None;
//                             }
//                         };
//                         let bits = super::types::Width::min_bits_for(max_val);
//                         if bits.0 <= 64 {
//                             return Some(bits.0);
//                         }
//                     }
//                 }
//             }
//         }
//     }
//
//     None
// }
//
// /// Dispatch to the appropriate solver based on SCC kind.
// pub fn solve_scc(
//     scc: &SccInfo,
//     signals: &[SignalDecl],
//     guards: &[Guard],
//     program: &crate::ast::program::MirrProgram,
// ) -> SccSolveResult {
//     match scc.kind {
//         SccKind::Nonexpansive => solve_nonexpansive(scc, signals),
//         SccKind::Expansive => solve_expansive(scc, signals, guards, program),
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::ast::expr::Expr;
//     use crate::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
//     use crate::ast::types::{BinaryOp, SignalKind, SignalType};
//     use crate::width::types::SccKind;
//
//     fn make_signal(name: &str, width: u32) -> SignalDecl {
//         SignalDecl {
//             name: name.to_string(),
//             kind: SignalKind::Internal,
//             ty: SignalType::Unsigned(width).into(),
//             origin: None,
//             span: None,
//         }
//     }
//
//     #[test]
//     fn test_solve_nonexpansive_max_iters() {
//         // Create an SCC that forces > MAX_FLOYD_WARSHALL_ITERS updates.
//         // Actually, Floyd Warshall is O(N^2), but our solver just loops until `!changed`.
//         // We can't easily force it to change 16384 times unless we have 16384 nodes.
//         // Let's just mock the iteration limit by calling a unit test specifically designed to hit it.
//         // Since we can't easily hit it without an enormous vector, we just accept this line might be hard to reach.
//         // Wait, what if we use the solver directly? The loop exits when `!changed`. To keep it changing,
//         // we'd need max value to keep updating, but `current_max` is fixed.
//         // Wait, `current_max` is computed every iteration:
//         // `let current_max = widths.iter().copied().max().unwrap_or(0);`
//         // Then:
//         // for w in &mut widths { if *w < current_max { *w = current_max; changed = true; } }
//         // If `changed` is true, it loops again. BUT `current_max` doesn't change!
//         // The second iteration, all widths are `current_max`, so `changed` is false!
//         // IT ALWAYS TERMINATES IN 2 ITERATIONS! The max_iter bound is unreachable!
//     }
//
//     #[test]
//     fn test_solve_expansive_missing_signal() {
//         let scc = SccInfo { signal_indices: vec![100], kind: SccKind::Expansive };
//         let signals = vec![make_signal("x", 8)];
//         let prog = MirrProgram {
//             imports: vec![],
//             patterns: vec![],
//             target: None,
//             module: Module {
//                 name: "m".to_string(),
//                 signals: vec![],
//                 guards: vec![],
//                 reflexes: vec![],
//                 properties: vec![],
//                 pattern_calls: vec![],
//                 pattern_origins: vec![],
//                 span: None,
//             },
//         };
//         let solve = solve_expansive(&scc, &signals, &[], &prog);
//         assert_eq!(solve.widths, vec![0]);
//     }
//
//     #[test]
//     fn test_infer_bound_from_guards_skip_target() {
//         let prog = MirrProgram {
//             imports: vec![],
//             patterns: vec![],
//             target: None,
//             module: Module {
//                 name: "m".to_string(),
//                 signals: vec![],
//                 guards: vec![],
//                 reflexes: vec![Reflex {
//                     name: "r".to_string(),
//                     guard_names: vec!["g".to_string()],
//                     assignments: vec![Assignment {
//                         target: "other".to_string(),
//                         value: Expr::Literal(crate::ast::types::LiteralValue::Integer(1)),
//                         span: None,
//                     }],
//                     origin: None,
//                     span: None,
//                 }],
//                 properties: vec![],
//                 pattern_calls: vec![],
//                 pattern_origins: vec![],
//                 span: None,
//             },
//         };
//         assert_eq!(infer_bound_from_guards("x", &prog, &[]), None);
//     }
//
//     #[test]
//     fn test_infer_bound_from_guards_reverse_add() {
//         let prog = MirrProgram {
//             imports: vec![],
//             patterns: vec![],
//             target: None,
//             module: Module {
//                 name: "m".to_string(),
//                 signals: vec![],
//                 guards: vec![],
//                 reflexes: vec![Reflex {
//                     name: "r".to_string(),
//                     guard_names: vec!["g".to_string()],
//                     assignments: vec![Assignment {
//                         target: "x".to_string(),
//                         value: Expr::Binary {
//                             op: BinaryOp::Add,
//                             left: Box::new(Expr::Literal(
//                                 crate::ast::types::LiteralValue::Integer(5),
//                             )),
//                             right: Box::new(Expr::Prev { signal: "x".to_string(), delay: 1 }),
//                         },
//                         span: None,
//                     }],
//                     origin: None,
//                     span: None,
//                 }],
//                 properties: vec![],
//                 pattern_calls: vec![],
//                 pattern_origins: vec![],
//                 span: None,
//             },
//         };
//         let guard = Guard {
//             name: "g".to_string(),
//             cycles: 10,
//             condition: Expr::Literal(crate::ast::types::LiteralValue::Bool(true)),
//             span: None,
//             origin: None,
//             template_cycles: None,
//         };
//         assert_eq!(infer_bound_from_guards("x", &prog, &[guard]), Some(6)); // 5 * 10 = 50 -> min_bits = 6
//     }
//
//     #[test]
//     fn test_infer_bound_from_guards_reverse_not_const() {
//         let prog = MirrProgram {
//             imports: vec![],
//             patterns: vec![],
//             target: None,
//             module: Module {
//                 name: "m".to_string(),
//                 signals: vec![],
//                 guards: vec![],
//                 reflexes: vec![Reflex {
//                     name: "r".to_string(),
//                     guard_names: vec!["g".to_string()],
//                     assignments: vec![Assignment {
//                         target: "x".to_string(),
//                         value: Expr::Binary {
//                             op: BinaryOp::Add,
//                             left: Box::new(Expr::Signal("y".to_string())),
//                             right: Box::new(Expr::Prev { signal: "x".to_string(), delay: 1 }),
//                         },
//                         span: None,
//                     }],
//                     origin: None,
//                     span: None,
//                 }],
//                 properties: vec![],
//                 pattern_calls: vec![],
//                 pattern_origins: vec![],
//                 span: None,
//             },
//         };
//         assert_eq!(infer_bound_from_guards("x", &prog, &[]), None);
//     }
//
//     #[test]
//     fn test_infer_bound_from_guards_not_add() {
//         let prog = MirrProgram {
//             imports: vec![],
//             patterns: vec![],
//             target: None,
//             module: Module {
//                 name: "m".to_string(),
//                 signals: vec![],
//                 guards: vec![],
//                 reflexes: vec![Reflex {
//                     name: "r".to_string(),
//                     guard_names: vec!["g".to_string()],
//                     assignments: vec![Assignment {
//                         target: "x".to_string(),
//                         value: Expr::Binary {
//                             op: BinaryOp::Sub,
//                             left: Box::new(Expr::Prev { signal: "x".to_string(), delay: 1 }),
//                             right: Box::new(Expr::Literal(
//                                 crate::ast::types::LiteralValue::Integer(5),
//                             )),
//                         },
//                         span: None,
//                     }],
//                     origin: None,
//                     span: None,
//                 }],
//                 properties: vec![],
//                 pattern_calls: vec![],
//                 pattern_origins: vec![],
//                 span: None,
//             },
//         };
//         assert_eq!(infer_bound_from_guards("x", &prog, &[]), None);
//     }
//
//     #[test]
//     fn test_infer_bound_from_guards_negative_increment() {
//         let prog = MirrProgram {
//             imports: vec![],
//             patterns: vec![],
//             target: None,
//             module: Module {
//                 name: "m".to_string(),
//                 signals: vec![],
//                 guards: vec![],
//                 reflexes: vec![Reflex {
//                     name: "r".to_string(),
//                     guard_names: vec!["g".to_string()],
//                     assignments: vec![Assignment {
//                         target: "x".to_string(),
//                         value: Expr::Binary {
//                             op: BinaryOp::Add,
//                             left: Box::new(Expr::Prev { signal: "x".to_string(), delay: 1 }),
//                             right: Box::new(Expr::Literal(
//                                 crate::ast::types::LiteralValue::Integer(0),
//                             )),
//                         },
//                         span: None,
//                     }],
//                     origin: None,
//                     span: None,
//                 }],
//                 properties: vec![],
//                 pattern_calls: vec![],
//                 pattern_origins: vec![],
//                 span: None,
//             },
//         };
//         assert_eq!(infer_bound_from_guards("x", &prog, &[]), None);
//     }
//
//     #[test]
//     fn test_infer_bound_from_guards_overflow() {
//         let prog = MirrProgram {
//             imports: vec![],
//             patterns: vec![],
//             target: None,
//             module: Module {
//                 name: "m".to_string(),
//                 signals: vec![],
//                 guards: vec![],
//                 reflexes: vec![Reflex {
//                     name: "r".to_string(),
//                     guard_names: vec!["g".to_string()],
//                     assignments: vec![Assignment {
//                         target: "x".to_string(),
//                         value: Expr::Binary {
//                             op: BinaryOp::Add,
//                             left: Box::new(Expr::Prev { signal: "x".to_string(), delay: 1 }),
//                             right: Box::new(Expr::Literal(
//                                 crate::ast::types::LiteralValue::Integer(u64::MAX),
//                             )),
//                         },
//                         span: None,
//                     }],
//                     origin: None,
//                     span: None,
//                 }],
//                 properties: vec![],
//                 pattern_calls: vec![],
//                 pattern_origins: vec![],
//                 span: None,
//             },
//         };
//         let guard = Guard {
//             name: "g".to_string(),
//             cycles: 2,
//             condition: Expr::Literal(crate::ast::types::LiteralValue::Bool(true)),
//             span: None,
//             origin: None,
//             template_cycles: None,
//         };
//         assert_eq!(infer_bound_from_guards("x", &prog, &[guard]), None);
//     }
// }
