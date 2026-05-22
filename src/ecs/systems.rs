#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, LiteralValue};
use crate::ecs::components::{EntityId, LiteralComponent, TemporalNodeComponent, TemporalStrategy};
use crate::ecs::registry::Registry;
use crate::error::MirrError;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};
use crate::width::scc_solver::SccSolveResult;
use crate::width::types::{SccInfo, SccKind, WidthStats};
use crate::width::verify::VerifyResult;
use rayon::prelude::*;

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
        registry.binary_ops[idx] = None;
        registry.literals[idx] = Some(LiteralComponent(value));
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
                Some(SccInfo { signal_indices: vec![idx], kind: SccKind::Expansive })
            } else {
                None
            }
        })
        .collect();

    let scc_solves: Vec<SccSolveResult> =
        sccs.iter().map(|_| SccSolveResult { widths: vec![8], diagnostics: Vec::new() }).collect();

    let stats = WidthStats {
        nodes_analyzed: registry.names.iter().filter(|n| n.is_some()).count(),
        propagation_rounds: 0,
        diagnostics_count: 0,
        scc_count: sccs.len(),
        expansive_count: 0,
        nonexpansive_count: 0,
    };

    (sccs, scc_solves, VerifyResult { is_minimal: true, diagnostics: Vec::new() }, stats)
}

/// ECS System: Pipeline Orchestrator.
pub fn run_compilation_pipeline(registry: &mut Registry) -> WidthStats {
    parallel_constant_folding_system(registry);
    let (_, _, _, stats) = parallel_width_inference_system(registry);
    stats
}

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
            if let Some(compiled) = netlist.guards.iter().find(|g| g.name() == name_comp.0) {
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
                registry.temporal_nodes[idx] = Some(node);
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
        (BinaryOp::Eq, LiteralValue::Integer(l), LiteralValue::Integer(r)) => {
            Some(LiteralValue::Bool(l == r))
        }
        _ => None,
    }
}
