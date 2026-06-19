#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use crate::ecs::components::{
    EntityId, LiteralComponent, TemporalNodeComponent, TemporalStrategy, UnaryComponent,
};
use crate::ecs::registry::Registry;
use crate::error::MirrError;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};
use crate::width::scc_solver::SccSolveResult;
use crate::width::types::{SccInfo, SccKind, WidthDiag, WidthStats};
use crate::width::verify::VerifyResult;
use rayon::prelude::*;

pub mod hls_schedule;
pub mod hls_sharing;
pub mod pattern_expansion;

/// ECS System: Parallel Constant Folding.
pub fn parallel_constant_folding_system(registry: &mut Registry) {
    let next_id = registry.next_id as usize;

    let reductions: Vec<(EntityId, LiteralValue)> = (0..next_id)
        .into_par_iter()
        .filter_map(|idx| {
            let binary = registry.binary_ops[idx].as_ref()?;
            let left_lit = registry.literals[binary.left.0 as usize].as_ref()?;
            let right_lit = registry.literals[binary.right.0 as usize].as_ref()?;
            fold_binary(binary.op, &left_lit.0, &right_lit.0).map(|val| (EntityId(idx as u32), val))
        })
        .collect();

    for (id, value) in reductions {
        let idx = id.0 as usize;
        registry.unset_binary_op(EntityId(idx as u32));
        registry.set_literal(EntityId(idx as u32), LiteralComponent(value));
    }
}

use crate::ecs::components::WidthConstraintComponent;

/// ECS System: Expression Width Inference.
/// Propagates width constraints to a fixpoint using monotonically increasing updates.
pub fn expression_width_inference_system(registry: &mut Registry) -> (Vec<WidthDiag>, usize) {
    const MAX_PROPAGATION_ROUNDS: usize = 16;
    let next_id = registry.names.len();
    let mut diagnostics: Vec<WidthDiag> = Vec::new();
    let mut rounds = 0usize;

    for _round in 0..MAX_PROPAGATION_ROUNDS {
        rounds += 1;
        let mut changed = false;

        for i in 0..next_id {
            if let Some(c) = &registry.width_constraints[i] {
                let new_w = evaluate_ecs_constraint(c, registry);
                if let Some(w) = new_w {
                    let current_tc = registry.types[i].as_ref();
                    let current_w = current_tc.map_or(0, |tc| tc.0.core.width());

                    let is_narrowing = matches!(c, WidthConstraintComponent::Narrowed { .. });
                    if is_narrowing {
                        if w != current_w {
                            registry.set_type(
                                EntityId(i as u32),
                                crate::ecs::components::TypeComponent::signal(
                                    crate::ast::types::ExtendedType::new(
                                        crate::ast::types::SignalType::Unsigned(w),
                                        Default::default(),
                                    ),
                                ),
                            );
                            changed = true;
                        }
                    } else if w > current_w {
                        registry.set_type(
                            EntityId(i as u32),
                            crate::ecs::components::TypeComponent::signal(
                                crate::ast::types::ExtendedType::new(
                                    crate::ast::types::SignalType::Unsigned(w),
                                    Default::default(),
                                ),
                            ),
                        );
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break; // Fixpoint reached.
        }
    }

    // Post-solve validation
    for i in 0..next_id {
        // Only validate entities that have a width constraint
        if registry.width_constraints[i].is_some() {
            let tc = registry.types[i].as_ref();
            let w = tc.map_or(0, |tc| tc.0.core.width());

            if w == 0 {
                let desc = format!("entity {}", i);
                diagnostics.push(
                    WidthDiag::error(format!(
                        "{} {} ({}) has unresolved width",
                        crate::error_codes::ec(503),
                        i,
                        desc
                    ))
                    .with_code("E503"),
                );
            }
            if w > crate::width::types::Width::MAX.0 {
                let desc = format!("entity {}", i);
                diagnostics.push(
                    WidthDiag::error(format!(
                        "{} {} ({}) requires {} bits, exceeding maximum of {}",
                        crate::error_codes::ec(504),
                        i,
                        desc,
                        w,
                        crate::width::types::Width::MAX.0
                    ))
                    .with_code("E504"),
                );
            }
        }
    }

    (diagnostics, rounds)
}

fn evaluate_ecs_constraint(c: &WidthConstraintComponent, registry: &Registry) -> Option<u32> {
    let get_w = |idx: EntityId| -> u32 {
        registry.types[idx.0 as usize].as_ref().map(|tc| tc.0.core.width()).unwrap_or(0)
    };

    match c {
        WidthConstraintComponent::Fixed(width) => Some(*width),
        WidthConstraintComponent::Boolean => Some(1),
        WidthConstraintComponent::MaxPlusOne { left, right } => {
            let lw = get_w(*left);
            let rw = get_w(*right);
            Some(lw.max(rw).saturating_add(1))
        }
        WidthConstraintComponent::MaxOf { left, right } => {
            let lw = get_w(*left);
            let rw = get_w(*right);
            Some(lw.max(rw))
        }
        WidthConstraintComponent::SumOf { left, right } => {
            let lw = get_w(*left);
            let rw = get_w(*right);
            Some(lw.saturating_add(rw))
        }
        WidthConstraintComponent::SumAll { elements } => {
            let mut total = 0u32;
            for elem in elements {
                total = total.saturating_add(get_w(*elem));
            }
            Some(total)
        }
        WidthConstraintComponent::LeftPlusConst { left, shift_amount } => {
            let lw = get_w(*left);
            Some(lw.saturating_add(*shift_amount))
        }
        WidthConstraintComponent::LeftPlusMaxShift { left } => {
            let lw = get_w(*left);
            Some(lw.saturating_add(crate::width::types::Width::MAX.0 - 1))
        }
        WidthConstraintComponent::LeftMinusConst { left, shift_amount } => {
            let lw = get_w(*left);
            Some(lw.saturating_sub(*shift_amount).max(1))
        }
        WidthConstraintComponent::SameAs { source } => Some(get_w(*source)),
        WidthConstraintComponent::SameAsPlusOne { source } => {
            let sw = get_w(*source);
            Some(sw.saturating_add(1))
        }
        WidthConstraintComponent::Narrowed { source, narrow_width } => {
            let sw = get_w(*source);
            if sw == 0 {
                Some(*narrow_width)
            } else {
                Some(sw.min(*narrow_width))
            }
        }
    }
}

/// ECS System: Parallel Width Inference.
pub fn parallel_width_inference_system(
    registry: &mut Registry,
) -> (Vec<SccInfo>, Vec<SccSolveResult>, VerifyResult, WidthStats) {
    let next_id = registry.next_id as usize;

    let sccs: Vec<SccInfo> = (0..next_id)
        .into_par_iter()
        .filter_map(|idx| {
            if registry.names[idx].is_some() {
                Some(SccInfo {
                    signals: vec![crate::ecs::components::EntityId(idx as u32)],
                    kind: SccKind::Expansive,
                })
            } else {
                None
            }
        })
        .collect();

    let scc_solves: Vec<SccSolveResult> = sccs
        .iter()
        .map(|scc| {
            // Derive the canonical width from the first signal index in the SCC.
            // If the entity has a TypeComponent, use the declared bit-width.
            // Otherwise fall back to 8 (ECS expression entities without a type).
            let width = scc
                .signals
                .first()
                .and_then(|&idx| registry.types.get(idx.0 as usize)?.as_ref())
                .map(|tc| {
                    let w = tc.0.core.width();
                    if w == 0 {
                        8u32
                    } else {
                        w
                    }
                })
                .unwrap_or(8);
            SccSolveResult { widths: vec![width], diagnostics: Vec::new() }
        })
        .collect();

    let (width_diags, rounds) = expression_width_inference_system(registry);

    let stats = WidthStats {
        nodes_analyzed: registry.names.iter().filter(|n| n.is_some()).count(),
        propagation_rounds: rounds,
        diagnostics_count: width_diags.len(),
        scc_count: sccs.len(),
        expansive_count: 0,
        nonexpansive_count: 0,
    };

    (sccs, scc_solves, VerifyResult { is_minimal: true, diagnostics: width_diags }, stats)
}

/// ECS System: SAT-based Simplification.
/// Verifies boolean cones for universal tautology or contradiction, eliminating deep redundancies.
pub fn sat_simplification_system(
    registry: &mut Registry,
) -> crate::pipeline::SatSimplifyPipelineStats {
    let next_id = registry.next_id as usize;
    let mut stats = crate::pipeline::SatSimplifyPipelineStats {
        checks_performed: 0,
        equivalences_confirmed: 0,
        had_unknown: false,
    };

    for i in 0..next_id {
        if registry.binary_ops[i].is_some() || registry.unary_ops[i].is_some() {
            let res =
                crate::sat::simplify_sat::simplify_entity_with_sat(EntityId(i as u32), registry);
            stats.checks_performed += res.checks_performed;
            stats.equivalences_confirmed += res.equivalences_confirmed;
            stats.had_unknown |= res.had_unknown;
        }
    }

    stats
}

/* --- STRANGLER PATTERN: ORCHESTRATOR ARCHIVED ---
/// ECS System: Pipeline Orchestrator.
pub fn run_compilation_pipeline(
    registry: &mut Registry,
) -> (WidthStats, crate::pipeline::SatSimplifyPipelineStats) {
    let _simplify_stats = simplifier_system(registry);
    let sat_stats = sat_simplification_system(registry);
    let (_, _, _, stats) = parallel_width_inference_system(registry);
    (stats, sat_stats)
}
*/

/// ECS System: Temporal Synthesis (Proposal 110 — Phase 3 ECS Transition).
///
/// This system orchestrates the lowering of high-level hardware guards into
/// deterministic IR primitives. It performs the following steps:
/// 1. Identifies all Guard entities in the Registry.
/// 2. Invokes the `TemporalCompiler` to perform direct ECS-native synthesis.
/// 3. Back-propagates the synthesis results into the Registry by attaching
///    `TemporalNodeComponent` metadata to each guard entity.
/// 4. Returns the final `TemporalNetlist` for downstream emission.
///
/// By closing the "Temporal Seam", this system ensures that the ECS Registry
/// is the absolute source of truth for the entire temporal compilation pass.
///
/// NASA P10 Rule #1: Bounded by `next_id`. No recursion.
pub fn temporal_synthesis_system(registry: &mut Registry) -> Result<TemporalNetlist, MirrError> {
    use crate::temporal::compiler::TemporalCompiler;

    let max_id = registry.next_id as usize;
    let mut guard_entities = Vec::new();

    for i in 0..max_id {
        // Only process Guard entities (those with a CyclesComponent).
        if registry.cycles[i].is_some() && registry.kinds[i].is_some() {
            guard_entities.push(EntityId(i as u32));
        }
    }

    // 3. Lower each guard entity into Temporal IR (direct ECS synthesis)
    let mut compiler = TemporalCompiler::new();
    for &gid in &guard_entities {
        let compiled = compiler.lower_guard_to_ecs(registry, gid)?;
        compiler.context.add_guard(compiled);
    }

    let netlist = compiler.context;

    // Link back to registry: Attach TemporalNodeComponents to top-level guards.
    // We match by name since top-level guards in the netlist correspond to registry entities.
    for &gid in &guard_entities {
        let idx = gid.0 as usize;
        if let Some(name_comp) = &registry.names[idx] {
            // Find this guard in the netlist.
            if let Some(compiled) =
                netlist.guards.iter().find(|g| g.name() == registry.resolve_name(name_comp.0))
            {
                let node = match compiled {
                    CompiledGuard::ShiftRegister(sr) => TemporalNodeComponent {
                        strategy: TemporalStrategy::ShiftRegister,
                        generated_signals: sr.stages.clone(),
                        output_signal: sr.output_signal.clone(),
                        delay_cycles: sr.delay_cycles,
                    },
                    CompiledGuard::Counter(c) => TemporalNodeComponent {
                        strategy: TemporalStrategy::Counter { counter_width: c.counter_width() },
                        generated_signals: vec![
                            c.counter_signal.clone(),
                            c.comparator_signal.clone(),
                        ],
                        output_signal: c.output_signal.clone(),
                        delay_cycles: c.target_count,
                    },
                    CompiledGuard::Complex(cx) => TemporalNodeComponent {
                        strategy: TemporalStrategy::Complex,
                        generated_signals: Vec::new(),
                        output_signal: cx.output_signal.clone(),
                        delay_cycles: 0,
                    },
                    CompiledGuard::DynamicCounter(dc) => TemporalNodeComponent {
                        strategy: TemporalStrategy::DynamicCounter {
                            max_delay: dc.max_delay,
                            counter_width: dc.counter_width(),
                        },
                        generated_signals: vec![dc.counter_signal.clone()],
                        output_signal: dc.output_signal.clone(),
                        delay_cycles: dc.max_delay,
                    },
                };
                registry.set_temporal_node(EntityId(idx as u32), node);
            }
        }
    }

    Ok(netlist)
}

/// ECS System: Parallel Vector Search (Grounding)
pub fn parallel_vector_search_system(
    registry: &Registry,
    query_vector: &[f32],
    limit: usize,
) -> Vec<(EntityId, f32)> {
    let next_id = registry.next_id as usize;

    let mut hits: Vec<(EntityId, f32)> = (0..next_id)
        .into_par_iter()
        .filter_map(|idx| {
            let vec_comp = registry.vectors[idx].as_ref()?;
            let score = cosine_similarity(query_vector, &vec_comp.0);
            Some((EntityId(idx as u32), score))
        })
        .collect();

    hits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    hits.truncate(limit);
    hits
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    (dot / (norm_a * norm_b)).clamp(0.0, 1.0)
}

fn fold_binary(op: BinaryOp, left: &LiteralValue, right: &LiteralValue) -> Option<LiteralValue> {
    match (op, left, right) {
        (BinaryOp::And, LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
            Some(LiteralValue::Bool(*l && *r))
        }
        (BinaryOp::Or, LiteralValue::Bool(l), LiteralValue::Bool(r)) => {
            Some(LiteralValue::Bool(*l || *r))
        }
        (BinaryOp::Add, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Integer(l.wrapping_add(*r)))
        }
        (BinaryOp::Sub, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Integer(l.wrapping_sub(*r)))
        }
        (BinaryOp::Mul, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Integer(l.wrapping_mul(*r)))
        }
        (BinaryOp::Shl, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            let amt = (*r).min(63);
            Some(LiteralValue::Integer(l << amt))
        }
        (BinaryOp::Shr, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            let amt = (*r).min(63);
            Some(LiteralValue::Integer(l >> amt))
        }
        (BinaryOp::Eq, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Bool(l == r))
        }
        (BinaryOp::Ne, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Bool(l != r))
        }
        (BinaryOp::Lt, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Bool(l < r))
        }
        (BinaryOp::Le, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Bool(l <= r))
        }
        (BinaryOp::Gt, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Bool(l > r))
        }
        (BinaryOp::Ge, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Bool(l >= r))
        }
        _ => None,
    }
}

/// ECS System: AST Simplifier Migration (Phase 3)
/// Recursively simplifies expressions using algebraic rules in-place on the ECS.
pub fn simplifier_system(registry: &mut Registry) -> crate::simplify::SimplifyStats {
    let mut total_rules_fired = 0;
    let next_id = registry.next_id as usize;

    for _pass in 0..crate::simplify::MAX_PASSES {
        let mut rules_fired = 0;

        for idx in 0..next_id {
            if let Some(unary) = registry.unary_ops[idx] {
                // Rule: !!X => X
                if unary.op == UnaryOp::Not {
                    let operand_idx = unary.operand.0 as usize;
                    if let Some(inner_unary) = registry.unary_ops[operand_idx] {
                        if inner_unary.op == UnaryOp::Not {
                            let inner_operand = inner_unary.operand;
                            copy_entity_components(registry, inner_operand.0 as usize, idx);
                            rules_fired += 1;
                            continue;
                        }
                    }
                }
                // Rule: !true => false, !false => true
                if unary.op == UnaryOp::Not {
                    let operand_idx = unary.operand.0 as usize;
                    let operand_bool = if let Some(LiteralComponent(LiteralValue::Bool(b))) =
                        &registry.literals[operand_idx]
                    {
                        Some(*b)
                    } else {
                        None
                    };

                    if let Some(b) = operand_bool {
                        registry.unset_unary_op(EntityId(idx as u32));
                        registry.set_literal(
                            EntityId(idx as u32),
                            LiteralComponent(LiteralValue::Bool(!b)),
                        );
                        rules_fired += 1;
                        continue;
                    }
                }
            }

            if let Some(binary) = registry.binary_ops[idx] {
                let left_idx = binary.left.0 as usize;
                let right_idx = binary.right.0 as usize;

                let left_lit = registry.literals[left_idx].as_ref().map(|c| c.0.clone());
                let right_lit = registry.literals[right_idx].as_ref().map(|c| c.0.clone());

                let eq = are_entities_structurally_equal(registry, left_idx, right_idx);
                let is_left_not_right = is_negation_of(registry, left_idx, right_idx);
                let is_right_not_left = is_negation_of(registry, right_idx, left_idx);

                // Constant folding
                if let (Some(l_lit), Some(r_lit)) = (&left_lit, &right_lit) {
                    if let Some(val) = fold_binary(binary.op, l_lit, r_lit) {
                        registry.unset_binary_op(EntityId(idx as u32));
                        registry.set_literal(EntityId(idx as u32), LiteralComponent(val));
                        rules_fired += 1;
                        continue;
                    }
                }

                match binary.op {
                    BinaryOp::And => {
                        if let Some(LiteralValue::Bool(true)) = right_lit {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(true)) = left_lit {
                            copy_entity_components(registry, right_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(false)) = right_lit {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Bool(false)),
                            );
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(false)) = left_lit {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Bool(false)),
                            );
                            rules_fired += 1;
                            continue;
                        } else if eq {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if is_left_not_right || is_right_not_left {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Bool(false)),
                            );
                            rules_fired += 1;
                            continue;
                        }
                    }
                    BinaryOp::Or => {
                        if let Some(LiteralValue::Bool(false)) = right_lit {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(false)) = left_lit {
                            copy_entity_components(registry, right_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(true)) = right_lit {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Bool(true)),
                            );
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(true)) = left_lit {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Bool(true)),
                            );
                            rules_fired += 1;
                            continue;
                        } else if eq {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if is_left_not_right || is_right_not_left {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Bool(true)),
                            );
                            rules_fired += 1;
                            continue;
                        }
                    }
                    BinaryOp::Xor => {
                        if let Some(LiteralValue::Bool(false)) = right_lit {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(false)) = left_lit {
                            copy_entity_components(registry, right_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(true)) = right_lit {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_unary_op(
                                EntityId(idx as u32),
                                UnaryComponent {
                                    op: UnaryOp::Not,
                                    operand: EntityId(left_idx as u32),
                                },
                            );
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Bool(true)) = left_lit {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_unary_op(
                                EntityId(idx as u32),
                                UnaryComponent {
                                    op: UnaryOp::Not,
                                    operand: EntityId(right_idx as u32),
                                },
                            );
                            rules_fired += 1;
                            continue;
                        } else if eq {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Bool(false)),
                            );
                            rules_fired += 1;
                            continue;
                        }
                    }
                    BinaryOp::Add => {
                        if let Some(LiteralValue::Integer(0)) = right_lit {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Integer(0)) = left_lit {
                            copy_entity_components(registry, right_idx, idx);
                            rules_fired += 1;
                            continue;
                        }
                    }
                    BinaryOp::Sub => {
                        if let Some(LiteralValue::Integer(0)) = right_lit {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        }
                    }
                    BinaryOp::Mul => {
                        if let Some(LiteralValue::Integer(1)) = right_lit {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Integer(1)) = left_lit {
                            copy_entity_components(registry, right_idx, idx);
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Integer(0)) = right_lit {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Integer(0)),
                            );
                            rules_fired += 1;
                            continue;
                        } else if let Some(LiteralValue::Integer(0)) = left_lit {
                            registry.unset_binary_op(EntityId(idx as u32));
                            registry.set_literal(
                                EntityId(idx as u32),
                                LiteralComponent(LiteralValue::Integer(0)),
                            );
                            rules_fired += 1;
                            continue;
                        }
                    }
                    BinaryOp::Shl => {
                        if let Some(LiteralValue::Integer(0)) = right_lit {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        }
                    }
                    BinaryOp::Shr => {
                        if let Some(LiteralValue::Integer(0)) = right_lit {
                            copy_entity_components(registry, left_idx, idx);
                            rules_fired += 1;
                            continue;
                        }
                    }
                    _ => {}
                }
            }
        }

        total_rules_fired += rules_fired;
        if rules_fired == 0 {
            break;
        }
    }

    crate::simplify::SimplifyStats {
        rules_applied: total_rules_fired,
        nodes_before: 0, // Migrated to ECS
        nodes_after: 0,
    }
}

fn copy_entity_components(registry: &mut Registry, src: usize, dst: usize) {
    if src == dst {
        return;
    }
    registry.literals[dst] = registry.literals[src].clone();
    registry.unary_ops[dst] = registry.unary_ops[src];
    registry.binary_ops[dst] = registry.binary_ops[src];
    registry.prev_ops[dst] = registry.prev_ops[src];
    registry.signal_refs[dst] = registry.signal_refs[src];
    registry.pending_signal_refs[dst] = registry.pending_signal_refs[src].clone();
    registry.array_indices[dst] = registry.array_indices[src];
    registry.field_accesses[dst] = registry.field_accesses[src].clone();
    registry.array_literals[dst] = registry.array_literals[src].clone();
    registry.struct_literals[dst] = registry.struct_literals[src].clone();
    registry.unfold_indices[dst] = registry.unfold_indices[src].clone();
}

fn are_entities_structurally_equal(registry: &Registry, a: usize, b: usize) -> bool {
    if a == b {
        return true;
    }

    // Check literals
    if registry.literals[a].is_some() || registry.literals[b].is_some() {
        if let (Some(la), Some(lb)) = (&registry.literals[a], &registry.literals[b]) {
            return la.0 == lb.0;
        }
        return false;
    }

    // Check signal refs
    if registry.signal_refs[a].is_some() || registry.signal_refs[b].is_some() {
        if let (Some(sa), Some(sb)) = (&registry.signal_refs[a], &registry.signal_refs[b]) {
            return sa.0 == sb.0;
        }
        return false;
    }

    // Check pending signal refs
    if registry.pending_signal_refs[a].is_some() || registry.pending_signal_refs[b].is_some() {
        if let (Some(sa), Some(sb)) =
            (&registry.pending_signal_refs[a], &registry.pending_signal_refs[b])
        {
            return sa.0 == sb.0;
        }
        return false;
    }

    // Check unary
    if registry.unary_ops[a].is_some() || registry.unary_ops[b].is_some() {
        if let (Some(ua), Some(ub)) = (&registry.unary_ops[a], &registry.unary_ops[b]) {
            if ua.op != ub.op {
                return false;
            }
            return are_entities_structurally_equal(
                registry,
                ua.operand.0 as usize,
                ub.operand.0 as usize,
            );
        }
        return false;
    }

    // Check binary
    if registry.binary_ops[a].is_some() || registry.binary_ops[b].is_some() {
        if let (Some(ba), Some(bb)) = (&registry.binary_ops[a], &registry.binary_ops[b]) {
            if ba.op != bb.op {
                return false;
            }
            return are_entities_structurally_equal(
                registry,
                ba.left.0 as usize,
                bb.left.0 as usize,
            ) && are_entities_structurally_equal(
                registry,
                ba.right.0 as usize,
                bb.right.0 as usize,
            );
        }
        return false;
    }

    false
}

fn is_negation_of(registry: &Registry, a: usize, b: usize) -> bool {
    // Is b the negation of a?
    if let Some(ub) = &registry.unary_ops[b] {
        if ub.op == UnaryOp::Not {
            return are_entities_structurally_equal(registry, a, ub.operand.0 as usize);
        }
    }
    false
}
