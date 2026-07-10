//! Graphviz DOT emitter for MIRR IR.
//!
//! Produces a `digraph` showing signals as nodes, guards as diamond nodes,
//! `always_comb` blocks for reflex assignments.
//!
//! Two detail levels:
//! - Module-level (default): one node per signal, one node per guard.
//! - Expr-level (`--dot-detail expr`): every AST node as a DOT node.

#![forbid(unsafe_code)]

use crate::ast::types::{SignalKind, SignalType};
use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};
use std::fmt::Write;

/// Maximum nodes to emit before truncating (prevents runaway on huge IR).
const MAX_DOT_NODES: usize = 4096;

/// Emit module-level DOT graph from pipeline results.
pub fn emit_module_dot(result: &PipelineResult) -> String {
    let mut out = String::with_capacity(2048);
    let registry = match &result.ecs_registry {
        Some(r) => r,
        None => return "// No ECS registry available for DOT emission\n".to_string(),
    };
    let module_name = registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string());

    out.push_str("digraph ");
    out.push_str(&sanitize_id(&module_name));
    out.push_str(" {\n");
    out.push_str("  rankdir=LR;\n");
    out.push_str("  node [fontname=\"monospace\"];\n\n");

    let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
            Some(crate::ecs::components::EntityId(i as u32))
        } else {
            None
        }
    });

    emit_pattern_origin_comments(registry, top_module_id, &mut out);
    emit_signal_nodes(registry, top_module_id, &mut out);
    emit_guard_nodes(registry, top_module_id, &mut out);
    emit_guard_edges(registry, top_module_id, &mut out);
    emit_reflex_edges(registry, top_module_id, &mut out);
    emit_property_nodes(registry, top_module_id, &mut out);

    emit_pattern_origins_ecs(registry, top_module_id, &mut out);

    if let Some(netlist) = &result.temporal_netlist {
        emit_temporal_subgraph(netlist, &mut out);
    }

    out.push_str("}\n");
    out
}

/// Emit expr-level DOT graph (full AST tree per expression).
pub fn emit_expr_dot(result: &PipelineResult) -> String {
    let mut out = String::with_capacity(4096);
    let registry = match &result.ecs_registry {
        Some(r) => r,
        None => return "// No ECS registry available for DOT emission\n".to_string(),
    };
    let module_name = registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string());

    let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
            Some(crate::ecs::components::EntityId(i as u32))
        } else {
            None
        }
    });

    out.push_str("digraph ");
    out.push_str(&sanitize_id(&module_name));
    out.push_str("_expr {\n");
    out.push_str("  rankdir=TB;\n");
    out.push_str("  node [fontname=\"monospace\"];\n\n");

    let mut node_id = 0usize;

    // Pattern definitions
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(_pat)) = (registry.names[i], &registry.pattern_defs[i]) {
            writeln!(out, "  // Pattern: {}", registry.resolve_name(nc.0)).unwrap();
        }
    }

    // Guard condition trees.
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp), Some(cond_comp)) =
            (registry.names[i], &registry.kinds[i], &registry.conditions[i])
        {
            if let crate::ecs::EntityKind::GUARD = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                writeln!(out, "  subgraph cluster_guard_{} {{", sanitize_id(name)).unwrap();
                writeln!(out, "    label=\"guard: {}\";", name).unwrap();
                emit_expr_nodes_ecs(registry, cond_comp.0, &mut node_id, &mut out);
                out.push_str("  }\n");
            }
        }
    }

    // Reflex assignment RHS trees.
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp), Some(r)) =
            (registry.names[i], &registry.kinds[i], &registry.reflex_comps[i])
        {
            if let crate::ecs::EntityKind::REFLEX = kind_comp.0 {
                let reflex_name = registry.resolve_name(nc.0);
                for a_id in &r.assignments {
                    if let Some(assign) = &registry.assignment_comps[a_id.0 as usize] {
                        let target_name_opt = registry.names[assign.target.0 as usize]
                            .map(|n| registry.resolve_name(n.0));
                        if let Some(target_name) = target_name_opt {
                            writeln!(
                                out,
                                "  subgraph cluster_{}_{} {{",
                                sanitize_id(reflex_name),
                                sanitize_id(target_name)
                            )
                            .unwrap();
                            writeln!(out, "    label=\"{}.{}\";", reflex_name, target_name)
                                .unwrap();
                            emit_expr_nodes_ecs(registry, assign.value, &mut node_id, &mut out);
                            out.push_str("  }\n");
                        }
                    }
                }
            }
        }
    }

    out.push_str("}\n");
    out
}

// -----------------------------------------------------------------------
// Internal helpers
// -----------------------------------------------------------------------

/// Emit DOT comments listing pattern definitions available in the registry.
fn emit_pattern_origin_comments(
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    out: &mut String,
) {
    let mut has_patterns = false;
    for i in 0..registry.pattern_defs.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if registry.pattern_defs[i].is_some() {
            has_patterns = true;
            break;
        }
    }
    if !has_patterns {
        return;
    }

    out.push_str("  // ── Pattern Definitions ──\n");
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(_)) = (registry.names[i], &registry.pattern_defs[i]) {
            writeln!(out, "  // Pattern: {}", registry.resolve_name(nc.0)).unwrap();
        }
    }
    out.push('\n');
}

fn emit_signal_nodes(
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    out: &mut String,
) {
    out.push_str("  // ── Signals ──\n");
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp), Some(ty_comp)) =
            (registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            if let crate::ecs::EntityKind::SIGNAL(sig_kind) = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                let shape = match sig_kind {
                    SignalKind::Input => "invhouse",
                    SignalKind::Output => "house",
                    SignalKind::Internal => "ellipse",
                };
                let width_label = match &ty_comp.0.core {
                    SignalType::Bool => "bool".to_string(),
                    SignalType::Unsigned(w) => format!("u{w}"),
                    SignalType::Signed(w) => format!("i{w}"),
                    other => other.to_string(),
                };
                writeln!(
                    out,
                    "  {} [label=\"{}: {}\" shape={shape}];",
                    sanitize_id(name),
                    name,
                    width_label,
                )
                .unwrap();
            }
        }
    }
    out.push('\n');
}

fn emit_guard_nodes(
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    out: &mut String,
) {
    out.push_str("\n  // ── Guards ──\n");
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp), Some(cycles_comp)) =
            (registry.names[i], &registry.kinds[i], &registry.cycles[i])
        {
            if let crate::ecs::EntityKind::GUARD = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                writeln!(
                    out,
                    "  {} [label=\"{} ({}c)\" shape=diamond style=filled fillcolor=lightyellow];",
                    guard_node_id(name),
                    name,
                    cycles_comp.0,
                )
                .unwrap();
            }
        }
    }
    out.push('\n');
}

fn collect_signal_refs_ecs(
    registry: &crate::ecs::Registry,
    expr_id: crate::ecs::EntityId,
) -> Vec<String> {
    let mut refs = Vec::new();
    let mut stack = vec![expr_id];
    let mut visited = 0;
    while let Some(id) = stack.pop() {
        visited += 1;
        if visited > MAX_DOT_NODES {
            break;
        }
        let idx = id.0 as usize;

        if let Some(crate::ecs::components::SignalRefComponent(sig_id)) = registry.signal_refs[idx]
        {
            if let Some(nc) = registry.names[sig_id.0 as usize] {
                refs.push(registry.resolve_name(nc.0).to_string());
            }
        } else if let Some(crate::ecs::components::PendingSignalRef(n)) =
            &registry.pending_signal_refs[idx]
        {
            refs.push(n.clone());
        } else if let Some(crate::ecs::components::UnaryComponent { operand, .. }) =
            &registry.unary_ops[idx]
        {
            stack.push(*operand);
        } else if let Some(crate::ecs::components::BinaryComponent { left, right, .. }) =
            &registry.binary_ops[idx]
        {
            stack.push(*left);
            stack.push(*right);
        } else if let Some(crate::ecs::components::ArrayIndexComponent { array, index }) =
            &registry.array_indices[idx]
        {
            stack.push(*array);
            stack.push(*index);
        } else if let Some(crate::ecs::components::FieldAccessComponent { object, .. }) =
            &registry.field_accesses[idx]
        {
            stack.push(*object);
        } else if let Some(crate::ecs::components::ArrayLiteralComponent(elems)) =
            &registry.array_literals[idx]
        {
            for e in elems {
                stack.push(*e);
            }
        } else if let Some(crate::ecs::components::StructLiteralComponent { fields, .. }) =
            &registry.struct_literals[idx]
        {
            for (_, e) in fields {
                stack.push(*e);
            }
        } else if let Some(crate::ecs::components::MuxComponent { select, true_val, false_val }) =
            &registry.muxes[idx]
        {
            stack.push(*select);
            stack.push(*true_val);
            stack.push(*false_val);
        }
    }
    refs
}

fn collect_prev_refs_ecs(
    registry: &crate::ecs::Registry,
    expr_id: crate::ecs::EntityId,
) -> Vec<(String, u64)> {
    let mut refs = Vec::new();
    let mut stack = vec![expr_id];
    let mut visited = 0;
    while let Some(id) = stack.pop() {
        visited += 1;
        if visited > MAX_DOT_NODES {
            break;
        }
        let idx = id.0 as usize;

        if let Some(crate::ecs::components::PrevComponent { signal, delay }) =
            &registry.prev_ops[idx]
        {
            if let Some(crate::ecs::components::SignalRefComponent(sig_id)) =
                registry.signal_refs[signal.0 as usize]
            {
                if let Some(nc) = registry.names[sig_id.0 as usize] {
                    refs.push((registry.resolve_name(nc.0).to_string(), *delay));
                }
            } else if let Some(crate::ecs::components::PendingSignalRef(n)) =
                &registry.pending_signal_refs[signal.0 as usize]
            {
                refs.push((n.clone(), *delay));
            }
        } else if let Some(crate::ecs::components::UnaryComponent { operand, .. }) =
            &registry.unary_ops[idx]
        {
            stack.push(*operand);
        } else if let Some(crate::ecs::components::BinaryComponent { left, right, .. }) =
            &registry.binary_ops[idx]
        {
            stack.push(*left);
            stack.push(*right);
        } else if let Some(crate::ecs::components::ArrayIndexComponent { array, index }) =
            &registry.array_indices[idx]
        {
            stack.push(*array);
            stack.push(*index);
        } else if let Some(crate::ecs::components::FieldAccessComponent { object, .. }) =
            &registry.field_accesses[idx]
        {
            stack.push(*object);
        } else if let Some(crate::ecs::components::ArrayLiteralComponent(elems)) =
            &registry.array_literals[idx]
        {
            for e in elems {
                stack.push(*e);
            }
        } else if let Some(crate::ecs::components::StructLiteralComponent { fields, .. }) =
            &registry.struct_literals[idx]
        {
            for (_, e) in fields {
                stack.push(*e);
            }
        } else if let Some(crate::ecs::components::MuxComponent { select, true_val, false_val }) =
            &registry.muxes[idx]
        {
            stack.push(*select);
            stack.push(*true_val);
            stack.push(*false_val);
        }
    }
    refs
}

/// Edges from signals referenced in guard conditions to guard nodes.
fn emit_guard_edges(
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    out: &mut String,
) {
    out.push_str("\n  // ── Guard Trigger Edges ──\n");
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp), Some(cond_comp)) =
            (registry.names[i], &registry.kinds[i], &registry.conditions[i])
        {
            if let crate::ecs::EntityKind::GUARD = kind_comp.0 {
                let guard_name = registry.resolve_name(nc.0);
                let refs = collect_signal_refs_ecs(registry, cond_comp.0);
                for sig in &refs {
                    writeln!(out, "  {} -> {};", sanitize_id(sig), guard_node_id(guard_name))
                        .unwrap();
                }
                // Prev back-edges rendered as dashed red.
                let prev_refs = collect_prev_refs_ecs(registry, cond_comp.0);
                for (sig, _delay) in &prev_refs {
                    writeln!(
                        out,
                        "  {} -> {} [style=dashed color=red label=\"prev\"];",
                        sanitize_id(sig),
                        guard_node_id(guard_name),
                    )
                    .unwrap();
                }
            }
        }
    }
    out.push('\n');
}

/// Edges from guard nodes to output signals via reflex assignments.
fn emit_reflex_edges(
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    out: &mut String,
) {
    out.push_str("\n  // ── Reflex Assignments ──\n");
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp), Some(r)) =
            (registry.names[i], &registry.kinds[i], &registry.reflex_comps[i])
        {
            if let crate::ecs::EntityKind::REFLEX = kind_comp.0 {
                let reflex_name = registry.resolve_name(nc.0);
                for gname in &r.guards {
                    let g_name_opt =
                        registry.names[gname.0 as usize].map(|n| registry.resolve_name(n.0));
                    if let Some(g_name) = g_name_opt {
                        for a_id in &r.assignments {
                            if let Some(assign) = &registry.assignment_comps[a_id.0 as usize] {
                                let target_name_opt = registry.names[assign.target.0 as usize]
                                    .map(|n| registry.resolve_name(n.0));
                                if let Some(target_name) = target_name_opt {
                                    writeln!(
                                        out,
                                        "  {} -> {} [label=\"{}\"];",
                                        guard_node_id(g_name),
                                        sanitize_id(target_name),
                                        reflex_name,
                                    )
                                    .unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    out.push('\n');
}

/// Temporal lowering subgraph showing hardware primitives.
fn emit_temporal_subgraph(netlist: &TemporalNetlist, out: &mut String) {
    out.push_str("  subgraph cluster_temporal {\n");
    out.push_str("    label=\"Temporal Hardware\";\n");
    out.push_str("    style=dashed;\n");
    out.push_str("    color=blue;\n");

    let mut nodes_emitted = 0usize;
    for guard in &netlist.guards {
        if nodes_emitted >= MAX_DOT_NODES {
            break;
        }
        match guard {
            CompiledGuard::ShiftRegister(sr) => {
                writeln!(
                    out,
                    "    {} [label=\"SR: {} ({}c)\" shape=record];",
                    sanitize_id(&sr.output_signal),
                    sr.name,
                    sr.delay_cycles,
                )
                .unwrap();
                nodes_emitted += 1;
            }
            CompiledGuard::Counter(c) => {
                writeln!(
                    out,
                    "    {} [label=\"CTR: {} ({}c)\" shape=record];",
                    sanitize_id(&c.output_signal),
                    c.name,
                    c.target_count,
                )
                .unwrap();
                nodes_emitted += 1;
            }
            CompiledGuard::Complex(cx) => {
                writeln!(
                    out,
                    "    {} [label=\"COMPLEX: {}\" shape=record];",
                    sanitize_id(&cx.output_signal),
                    cx.name,
                )
                .unwrap();
                nodes_emitted += 1;
            }
            CompiledGuard::DynamicCounter(dc) => {
                writeln!(
                    out,
                    "    {} [label=\"DYN: {} (max {}c)\" shape=record];",
                    sanitize_id(&dc.output_signal),
                    dc.name,
                    dc.max_delay,
                )
                .unwrap();
                nodes_emitted += 1;
            }
        }
    }

    out.push_str("  }\n\n");
}

fn emit_expr_nodes_ecs(
    registry: &crate::ecs::Registry,
    expr_id: crate::ecs::EntityId,
    node_id: &mut usize,
    out: &mut String,
) {
    let mut stack: Vec<(crate::ecs::EntityId, usize)> = Vec::with_capacity(64);
    let root_id = *node_id;
    *node_id += 1;
    stack.push((expr_id, root_id));

    let mut iterations = 0usize;
    while let Some((e, my_id)) = stack.pop() {
        iterations += 1;
        if iterations > MAX_DOT_NODES {
            break;
        }
        let idx = e.0 as usize;

        if let Some(crate::ecs::components::LiteralComponent(lit)) = &registry.literals[idx] {
            let label = format!("{:?}", lit);
            writeln!(out, "    n{my_id} [label=\"{label}\" shape=box];").unwrap();
        } else if let Some(crate::ecs::components::SignalRefComponent(sig_ent)) =
            registry.signal_refs[idx]
        {
            let name = registry.names[sig_ent.0 as usize]
                .map(|n| registry.resolve_name(n.0))
                .unwrap_or("unknown_sig");
            writeln!(out, "    n{my_id} [label=\"{name}\" shape=ellipse];").unwrap();
        } else if let Some(crate::ecs::components::PendingSignalRef(name)) =
            &registry.pending_signal_refs[idx]
        {
            writeln!(out, "    n{my_id} [label=\"{name}\" shape=ellipse];").unwrap();
        } else if let Some(crate::ecs::components::PrevComponent { signal, delay }) =
            &registry.prev_ops[idx]
        {
            let name = if let Some(crate::ecs::components::SignalRefComponent(decl)) =
                registry.signal_refs[signal.0 as usize]
            {
                registry.names[decl.0 as usize]
                    .map(|n| registry.resolve_name(n.0).to_string())
                    .unwrap_or_default()
            } else if let Some(crate::ecs::components::PendingSignalRef(n)) =
                &registry.pending_signal_refs[signal.0 as usize]
            {
                n.clone()
            } else {
                String::new()
            };
            writeln!(out, "    n{my_id} [label=\"prev({name},{delay})\" shape=ellipse style=dashed color=red];").unwrap();
        } else if let Some(crate::ecs::components::UnaryComponent { op, operand }) =
            &registry.unary_ops[idx]
        {
            writeln!(out, "    n{my_id} [label=\"{:?}\" shape=circle];", op).unwrap();
            let child_id = *node_id;
            *node_id += 1;
            writeln!(out, "    n{my_id} -> n{child_id};").unwrap();
            stack.push((*operand, child_id));
        } else if let Some(crate::ecs::components::BinaryComponent { op, left, right }) =
            &registry.binary_ops[idx]
        {
            writeln!(out, "    n{my_id} [label=\"{:?}\" shape=circle];", op).unwrap();
            let left_id = *node_id;
            *node_id += 1;
            let right_id = *node_id;
            *node_id += 1;
            writeln!(out, "    n{my_id} -> n{left_id};").unwrap();
            writeln!(out, "    n{my_id} -> n{right_id};").unwrap();
            stack.push((*right, right_id));
            stack.push((*left, left_id));
        } else if let Some(crate::ecs::components::ArrayIndexComponent { array, index }) =
            &registry.array_indices[idx]
        {
            writeln!(out, "    n{my_id} [label=\"[]\" shape=circle];").unwrap();
            let arr_id = *node_id;
            *node_id += 1;
            let idx_id = *node_id;
            *node_id += 1;
            writeln!(out, "    n{my_id} -> n{arr_id};").unwrap();
            writeln!(out, "    n{my_id} -> n{idx_id};").unwrap();
            stack.push((*index, idx_id));
            stack.push((*array, arr_id));
        } else if let Some(crate::ecs::components::FieldAccessComponent { object, field }) =
            &registry.field_accesses[idx]
        {
            writeln!(out, "    n{my_id} [label=\".{field}\" shape=circle];").unwrap();
            let obj_id = *node_id;
            *node_id += 1;
            writeln!(out, "    n{my_id} -> n{obj_id};").unwrap();
            stack.push((*object, obj_id));
        } else if let Some(crate::ecs::components::ArrayLiteralComponent(elems)) =
            &registry.array_literals[idx]
        {
            writeln!(out, "    n{my_id} [label=\"[...]\" shape=circle];").unwrap();
            for elem in elems.iter().take(MAX_DOT_NODES) {
                let elem_id = *node_id;
                *node_id += 1;
                writeln!(out, "    n{my_id} -> n{elem_id};").unwrap();
                stack.push((*elem, elem_id));
            }
        } else if let Some(crate::ecs::components::StructLiteralComponent { name: _, fields }) =
            &registry.struct_literals[idx]
        {
            writeln!(out, "    n{my_id} [label=\"{{...}}\" shape=circle];").unwrap();
            for (_, fval) in fields.iter().take(MAX_DOT_NODES) {
                let fval_id = *node_id;
                *node_id += 1;
                writeln!(out, "    n{my_id} -> n{fval_id};").unwrap();
                stack.push((*fval, fval_id));
            }
        } else if let Some(crate::ecs::components::MuxComponent { select, true_val, false_val }) =
            &registry.muxes[idx]
        {
            writeln!(out, "    n{my_id} [label=\"mux\" shape=circle];").unwrap();
            let c_id = *node_id;
            *node_id += 1;
            let t_id = *node_id;
            *node_id += 1;
            let f_id = *node_id;
            *node_id += 1;
            writeln!(out, "    n{my_id} -> n{c_id};").unwrap();
            writeln!(out, "    n{my_id} -> n{t_id};").unwrap();
            writeln!(out, "    n{my_id} -> n{f_id};").unwrap();
            stack.push((*false_val, f_id));
            stack.push((*true_val, t_id));
            stack.push((*select, c_id));
        }
    }
}

/// Emit comments describing which signals/guards originated from which patterns.
fn emit_pattern_origins_ecs(
    registry: &crate::ecs::Registry,
    _top_module_id: Option<crate::ecs::components::EntityId>,
    out: &mut String,
) {
    if registry.pattern_origins.is_empty() {
        return;
    }

    out.push_str("  // ── Pattern Origins ──\n");
    for origin in &registry.pattern_origins {
        writeln!(
            out,
            "  // Pattern expanded: {} with args ({})",
            origin.pattern_name, origin.call_args_summary
        )
        .unwrap();
    }
    out.push('\n');
}

/// Sanitize a name for use as a DOT identifier.
fn sanitize_id(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 18);
    let mut needs_disambiguator = false;
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            out.push(c);
        } else {
            out.push('_');
            needs_disambiguator = true;
        }
    }

    if needs_disambiguator {
        out.push('_');
        out.push_str(&stable_name_hash(name));
    }

    out
}

fn stable_name_hash(name: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in name.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn guard_node_id(name: &str) -> String {
    format!("guard_{}", sanitize_id(name))
}

/// Emit property nodes and edges to referenced signals.
fn emit_property_nodes(
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    out: &mut String,
) {
    let mut has_props = false;
    for i in 0..registry.kinds.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::PROPERTY)) =
            &registry.kinds[i]
        {
            has_props = true;
            break;
        }
    }
    if !has_props {
        return;
    }

    out.push_str("\n  // ── Properties ──\n");
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp), Some(prop)) =
            (registry.names[i], &registry.kinds[i], &registry.property_comps[i])
        {
            if let crate::ecs::EntityKind::PROPERTY = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                let prop_id = format!("prop_{}", sanitize_id(name));
                let fillcolor = match prop.directive {
                    crate::ast::property::PropertyDirective::Assert => "lightblue",
                    crate::ast::property::PropertyDirective::Cover => "lightyellow",
                    crate::ast::property::PropertyDirective::Assume => "lightgreen",
                };
                writeln!(
                    out,
                    "  {prop_id} [shape=note style=filled fillcolor={fillcolor} label=\"{}\"];",
                    name,
                )
                .unwrap();

                for expr_id in &prop.formula_exprs {
                    let refs = collect_signal_refs_ecs(registry, *expr_id);
                    for sig in &refs {
                        writeln!(
                            out,
                            "  {} -> {prop_id} [style=dotted color=blue];",
                            sanitize_id(sig),
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    out.push('\n');
}
