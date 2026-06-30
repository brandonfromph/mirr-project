//! Phase 7: Source Provenance Graph Emitter
//!
//! Generates a causal dependency graph mapping every generated Verilog signal
//! back to its original MIRR source span and the upstream signals that drive it.
//! This enables backwards causal traversal for automated root-cause analysis
//! of formal verification traces.

#![forbid(unsafe_code)]

use crate::ecs::components::{EntityId, EntityKind, KindComponent};
use crate::ecs::Registry;
use crate::pipeline::PipelineResult;
use crate::span::Span;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenanceNode {
    /// The flattened Verilog signal name (e.g., `rspu_top.core_12.instr_12`).
    pub signal: String,
    /// The original MIRR source file and line (if available).
    pub origin: Option<Span>,
    /// Flattened Verilog signal names that this signal directly depends on.
    pub depends_on: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvenanceGraph {
    /// Map from flattened Verilog signal name to its provenance node.
    pub nodes: BTreeMap<String, ProvenanceNode>,
}

/// Build the Source Provenance Graph from the ECS Registry in-memory.
pub fn build_provenance_graph(result: &PipelineResult) -> Option<ProvenanceGraph> {
    let registry = result.ecs_registry.as_ref()?;

    let mut graph = ProvenanceGraph { nodes: BTreeMap::new() };

    let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
            Some(crate::ecs::components::EntityId(i as u32))
        } else {
            None
        }
    });

    // Phase 1: Build a reverse lookup from Assignment -> Target Entity
    // and Target Entity -> Expression Root Entity.
    let mut target_to_expr = BTreeMap::new();
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let Some(crate::ecs::components::AssignmentComponent { target, value, .. }) =
            &registry.assignment_comps[i]
        {
            target_to_expr.insert(*target, *value);
        }
    }

    // Phase 2: Traverse all signals and properties and build their dependency trees.
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(name_comp), Some(KindComponent(kind))) =
            (&registry.names[i], &registry.kinds[i])
        {
            if !matches!(kind, EntityKind::SIGNAL(_) | EntityKind::PROPERTY) {
                continue;
            }

            let signal_name = registry.resolve_name(name_comp.0).to_string();

            // Get origin span
            let origin = registry.spans[i].map(|s| s.0);

            let mut depends_on = BTreeSet::new();
            let entity_id = EntityId(i as u32);

            if matches!(kind, EntityKind::SIGNAL(_)) {
                // Find what drives this signal
                if let Some(expr_root) = target_to_expr.get(&entity_id) {
                    collect_dependencies(*expr_root, registry, &mut depends_on);
                }
            } else if let Some(prop_comp) = &registry.property_comps[i] {
                // Find what this property observes
                for expr_id in &prop_comp.formula_exprs {
                    collect_dependencies(*expr_id, registry, &mut depends_on);
                }
            }

            graph.nodes.insert(
                signal_name.clone(),
                ProvenanceNode {
                    signal: signal_name,
                    origin,
                    depends_on: depends_on.into_iter().collect(),
                },
            );
        }
    }

    Some(graph)
}

/// Emit the Source Provenance Graph from the ECS Registry to a JSON string.
pub fn emit_provenance_graph(result: &PipelineResult) -> Result<String, crate::error::MirrError> {
    let graph = match build_provenance_graph(result) {
        Some(g) => g,
        None => return Ok("{}".to_string()),
    };
    let json = serde_json::to_string_pretty(&graph).unwrap_or_else(|_| "{}".to_string());
    Ok(json)
}

/// Recursively walk the ECS expression graph to find signal dependencies.
fn collect_dependencies(expr_id: EntityId, registry: &Registry, deps: &mut BTreeSet<String>) {
    let idx = expr_id.0 as usize;
    if idx >= registry.names.len() {
        return;
    }

    if let Some(crate::ecs::components::SignalRefComponent(sig_ent)) = registry.signal_refs[idx] {
        if let Some(name_comp) = registry.names[sig_ent.0 as usize] {
            deps.insert(registry.resolve_name(name_comp.0).to_string());
        }
    } else if let Some(crate::ecs::components::PendingSignalRef(name)) =
        &registry.pending_signal_refs[idx]
    {
        deps.insert(name.clone());
    } else if let Some(crate::ecs::components::BinaryComponent { left, right, .. }) =
        &registry.binary_ops[idx]
    {
        collect_dependencies(*left, registry, deps);
        collect_dependencies(*right, registry, deps);
    } else if let Some(crate::ecs::components::UnaryComponent { operand, .. }) =
        &registry.unary_ops[idx]
    {
        collect_dependencies(*operand, registry, deps);
    } else if let Some(crate::ecs::components::PrevComponent { signal, .. }) =
        &registry.prev_ops[idx]
    {
        collect_dependencies(*signal, registry, deps);
    } else if let Some(crate::ecs::components::MuxComponent { select, true_val, false_val }) =
        &registry.muxes[idx]
    {
        collect_dependencies(*select, registry, deps);
        collect_dependencies(*true_val, registry, deps);
        collect_dependencies(*false_val, registry, deps);
    } else if let Some(crate::ecs::components::ArrayIndexComponent { array, index }) =
        &registry.array_indices[idx]
    {
        collect_dependencies(*array, registry, deps);
        collect_dependencies(*index, registry, deps);
    } else if let Some(crate::ecs::components::FieldAccessComponent { object, .. }) =
        &registry.field_accesses[idx]
    {
        collect_dependencies(*object, registry, deps);
    } else if let Some(crate::ecs::components::ArrayLiteralComponent(elems)) =
        &registry.array_literals[idx]
    {
        for elem in elems {
            collect_dependencies(*elem, registry, deps);
        }
    } else if let Some(crate::ecs::components::StructLiteralComponent { fields, .. }) =
        &registry.struct_literals[idx]
    {
        for (_, val) in fields {
            collect_dependencies(*val, registry, deps);
        }
    }
}
