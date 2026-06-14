//! Graphviz DOT emitter for MIRR IR.
//!
//! Produces a `digraph` showing signals as nodes, guards as diamond nodes,
//! reflex assignments as edges, and `Prev` back-edges as dashed red edges.
//!
//! Two detail levels:
//! - Module-level (default): one node per signal, one node per guard.
//! - Expr-level (`--dot-detail expr`): every AST node as a DOT node.

#![forbid(unsafe_code)]

use crate::ast::types::{SignalKind, SignalType};
use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};

/// Maximum nodes to emit before truncating (prevents runaway on huge IR).
const MAX_DOT_NODES: usize = 4096;

/// Emit module-level DOT graph from pipeline results.
pub fn emit_module_dot(result: &PipelineResult) -> String {
    let mut out = String::with_capacity(2048);
    let registry = result.ecs_registry.as_ref().unwrap();
    let module_name = registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string());

    out.push_str("digraph ");
    out.push_str(&sanitize_id(&module_name));
    out.push_str(" {\n");
    out.push_str("  rankdir=LR;\n");
    out.push_str("  node [fontname=\"monospace\"];\n\n");

    emit_pattern_origin_comments(registry, &mut out);
    emit_signal_nodes(registry, &mut out);
    emit_guard_nodes(registry, &mut out);
    emit_guard_edges(registry, &mut out);
    emit_reflex_edges(registry, &mut out);
    emit_property_nodes(registry, &mut out);

    if let Some(netlist) = &result.temporal_netlist {
        emit_temporal_subgraph(netlist, &mut out);
    }

    out.push_str("}\n");
    out
}

/// Emit expr-level DOT graph (full AST tree per expression).
pub fn emit_expr_dot(result: &PipelineResult) -> String {
    let mut out = String::with_capacity(4096);
    let registry = result.ecs_registry.as_ref().unwrap();
    let module_name = registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string());

    out.push_str("digraph ");
    out.push_str(&sanitize_id(&module_name));
    out.push_str("_expr {\n");
    out.push_str("  rankdir=TB;\n");
    out.push_str("  node [fontname=\"monospace\"];\n\n");

    let mut node_id = 0usize;

    // Guard condition trees.
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(cond_comp)) =
            (&registry.names[i], &registry.kinds[i], &registry.conditions[i])
        {
            if let crate::ecs::EntityKind::GUARD = kind_comp.0 {
                out.push_str(&format!(
                    "  subgraph cluster_guard_{} {{\n",
                    sanitize_id(&name_comp.0)
                ));
                out.push_str(&format!("    label=\"guard: {}\";\n", name_comp.0));
                emit_expr_nodes_ecs(registry, cond_comp.0, &mut node_id, &mut out);
                out.push_str("  }\n");
            }
        }
    }

    // Reflex assignment RHS trees.
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(r)) =
            (&registry.names[i], &registry.kinds[i], &registry.reflex_comps[i])
        {
            if let crate::ecs::EntityKind::REFLEX = kind_comp.0 {
                for a_id in &r.assignments {
                    if let Some(assign) = &registry.assignment_comps[a_id.0 as usize] {
                        let target_name =
                            &registry.names[assign.target.0 as usize].as_ref().unwrap().0;
                        out.push_str(&format!(
                            "  subgraph cluster_{}_{} {{\n",
                            sanitize_id(&name_comp.0),
                            sanitize_id(target_name)
                        ));
                        out.push_str(&format!("    label=\"{}.{}\";\n", name_comp.0, target_name));
                        emit_expr_nodes_ecs(registry, assign.value, &mut node_id, &mut out);
                        out.push_str("  }\n");
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
fn emit_pattern_origin_comments(registry: &crate::ecs::Registry, out: &mut String) {
    let mut has_patterns = false;
    for def in &registry.pattern_defs {
        if def.is_some() {
            has_patterns = true;
            break;
        }
    }
    if !has_patterns {
        return;
    }

    out.push_str("  // ── Pattern Definitions ──\n");
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(_)) = (&registry.names[i], &registry.pattern_defs[i]) {
            out.push_str(&format!("  // Pattern available: {}\n", name_comp.0));
        }
    }
    out.push('\n');
}

fn emit_signal_nodes(registry: &crate::ecs::Registry, out: &mut String) {
    out.push_str("  // Signals\n");
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(ty_comp)) =
            (&registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            if let crate::ecs::EntityKind::SIGNAL(sig_kind) = kind_comp.0 {
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
                out.push_str(&format!(
                    "  {} [label=\"{}: {}\" shape={shape}];\n",
                    sanitize_id(&name_comp.0),
                    name_comp.0,
                    width_label,
                ));
            }
        }
    }
    out.push('\n');
}

fn emit_guard_nodes(registry: &crate::ecs::Registry, out: &mut String) {
    out.push_str("  // Guards\n");
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(cycles_comp)) =
            (&registry.names[i], &registry.kinds[i], &registry.cycles[i])
        {
            if let crate::ecs::EntityKind::GUARD = kind_comp.0 {
                out.push_str(&format!(
                    "  {} [label=\"{} ({}c)\" shape=diamond style=filled fillcolor=lightyellow];\n",
                    guard_node_id(&name_comp.0),
                    name_comp.0,
                    cycles_comp.0,
                ));
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
            if let Some(n) = &registry.names[sig_id.0 as usize] {
                refs.push(n.0.clone());
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
                if let Some(n) = &registry.names[sig_id.0 as usize] {
                    refs.push((n.0.clone(), *delay));
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
fn emit_guard_edges(registry: &crate::ecs::Registry, out: &mut String) {
    out.push_str("  // Guard inputs\n");
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(cond_comp)) =
            (&registry.names[i], &registry.kinds[i], &registry.conditions[i])
        {
            if let crate::ecs::EntityKind::GUARD = kind_comp.0 {
                let refs = collect_signal_refs_ecs(registry, cond_comp.0);
                for sig in &refs {
                    out.push_str(&format!(
                        "  {} -> {};\n",
                        sanitize_id(sig),
                        guard_node_id(&name_comp.0)
                    ));
                }
                // Prev back-edges rendered as dashed red.
                let prev_refs = collect_prev_refs_ecs(registry, cond_comp.0);
                for (sig, _delay) in &prev_refs {
                    out.push_str(&format!(
                        "  {} -> {} [style=dashed color=red label=\"prev\"];\n",
                        sanitize_id(sig),
                        guard_node_id(&name_comp.0),
                    ));
                }
            }
        }
    }
    out.push('\n');
}

/// Edges from guard nodes to output signals via reflex assignments.
fn emit_reflex_edges(registry: &crate::ecs::Registry, out: &mut String) {
    out.push_str("  // Reflex assignments\n");
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(r)) =
            (&registry.names[i], &registry.kinds[i], &registry.reflex_comps[i])
        {
            if let crate::ecs::EntityKind::REFLEX = kind_comp.0 {
                for gname in &r.guards {
                    let g_name_str = &registry.names[gname.0 as usize].as_ref().unwrap().0;
                    for a_id in &r.assignments {
                        if let Some(assign) = &registry.assignment_comps[a_id.0 as usize] {
                            let target_name =
                                &registry.names[assign.target.0 as usize].as_ref().unwrap().0;
                            out.push_str(&format!(
                                "  {} -> {} [label=\"{}\"];\n",
                                guard_node_id(g_name_str),
                                sanitize_id(target_name),
                                name_comp.0,
                            ));
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
                out.push_str(&format!(
                    "    {} [label=\"SR: {} ({}c)\" shape=record];\n",
                    sanitize_id(&sr.output_signal),
                    sr.name,
                    sr.delay_cycles,
                ));
                nodes_emitted += 1;
            }
            CompiledGuard::Counter(c) => {
                out.push_str(&format!(
                    "    {} [label=\"CTR: {} ({}c)\" shape=record];\n",
                    sanitize_id(&c.output_signal),
                    c.name,
                    c.target_count,
                ));
                nodes_emitted += 1;
            }
            CompiledGuard::Complex(cx) => {
                out.push_str(&format!(
                    "    {} [label=\"COMPLEX: {}\" shape=record];\n",
                    sanitize_id(&cx.output_signal),
                    cx.name,
                ));
                nodes_emitted += 1;
            }
            CompiledGuard::DynamicCounter(dc) => {
                out.push_str(&format!(
                    "    {} [label=\"DYN: {} (max {}c)\" shape=record];\n",
                    sanitize_id(&dc.output_signal),
                    dc.name,
                    dc.max_delay,
                ));
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
            out.push_str(&format!("    n{my_id} [label=\"{label}\" shape=box];\n"));
        } else if let Some(crate::ecs::components::SignalRefComponent(sig_ent)) =
            registry.signal_refs[idx]
        {
            let name = registry.names[sig_ent.0 as usize].as_ref().unwrap().0.clone();
            out.push_str(&format!("    n{my_id} [label=\"{name}\" shape=ellipse];\n"));
        } else if let Some(crate::ecs::components::PendingSignalRef(name)) =
            &registry.pending_signal_refs[idx]
        {
            out.push_str(&format!("    n{my_id} [label=\"{name}\" shape=ellipse];\n"));
        } else if let Some(crate::ecs::components::PrevComponent { signal, delay }) =
            &registry.prev_ops[idx]
        {
            let name = if let Some(crate::ecs::components::SignalRefComponent(decl)) =
                registry.signal_refs[signal.0 as usize]
            {
                registry.names[decl.0 as usize].as_ref().map(|n| n.0.clone()).unwrap_or_default()
            } else if let Some(crate::ecs::components::PendingSignalRef(n)) =
                &registry.pending_signal_refs[signal.0 as usize]
            {
                n.clone()
            } else {
                String::new()
            };
            out.push_str(&format!("    n{my_id} [label=\"prev({name},{delay})\" shape=ellipse style=dashed color=red];\n"));
        } else if let Some(crate::ecs::components::UnaryComponent { op, operand }) =
            &registry.unary_ops[idx]
        {
            out.push_str(&format!("    n{my_id} [label=\"{:?}\" shape=circle];\n", op));
            let child_id = *node_id;
            *node_id += 1;
            out.push_str(&format!("    n{my_id} -> n{child_id};\n"));
            stack.push((*operand, child_id));
        } else if let Some(crate::ecs::components::BinaryComponent { op, left, right }) =
            &registry.binary_ops[idx]
        {
            out.push_str(&format!("    n{my_id} [label=\"{:?}\" shape=circle];\n", op));
            let left_id = *node_id;
            *node_id += 1;
            let right_id = *node_id;
            *node_id += 1;
            out.push_str(&format!("    n{my_id} -> n{left_id};\n"));
            out.push_str(&format!("    n{my_id} -> n{right_id};\n"));
            stack.push((*right, right_id));
            stack.push((*left, left_id));
        } else if let Some(crate::ecs::components::ArrayIndexComponent { array, index }) =
            &registry.array_indices[idx]
        {
            out.push_str(&format!("    n{my_id} [label=\"[]\" shape=circle];\n"));
            let arr_id = *node_id;
            *node_id += 1;
            let idx_id = *node_id;
            *node_id += 1;
            out.push_str(&format!("    n{my_id} -> n{arr_id};\n"));
            out.push_str(&format!("    n{my_id} -> n{idx_id};\n"));
            stack.push((*index, idx_id));
            stack.push((*array, arr_id));
        } else if let Some(crate::ecs::components::FieldAccessComponent { object, field }) =
            &registry.field_accesses[idx]
        {
            out.push_str(&format!("    n{my_id} [label=\".{field}\" shape=circle];\n"));
            let obj_id = *node_id;
            *node_id += 1;
            out.push_str(&format!("    n{my_id} -> n{obj_id};\n"));
            stack.push((*object, obj_id));
        } else if let Some(crate::ecs::components::ArrayLiteralComponent(elems)) =
            &registry.array_literals[idx]
        {
            out.push_str(&format!("    n{my_id} [label=\"[...]\" shape=circle];\n"));
            for elem in elems.iter().take(MAX_DOT_NODES) {
                let elem_id = *node_id;
                *node_id += 1;
                out.push_str(&format!("    n{my_id} -> n{elem_id};\n"));
                stack.push((*elem, elem_id));
            }
        } else if let Some(crate::ecs::components::StructLiteralComponent { name: _, fields }) =
            &registry.struct_literals[idx]
        {
            out.push_str(&format!("    n{my_id} [label=\"{{...}}\" shape=circle];\n"));
            for (_, fval) in fields.iter().take(MAX_DOT_NODES) {
                let fval_id = *node_id;
                *node_id += 1;
                out.push_str(&format!("    n{my_id} -> n{fval_id};\n"));
                stack.push((*fval, fval_id));
            }
        } else if let Some(crate::ecs::components::MuxComponent { select, true_val, false_val }) =
            &registry.muxes[idx]
        {
            out.push_str(&format!("    n{my_id} [label=\"mux\" shape=circle];\n"));
            let c_id = *node_id;
            *node_id += 1;
            let t_id = *node_id;
            *node_id += 1;
            let f_id = *node_id;
            *node_id += 1;
            out.push_str(&format!("    n{my_id} -> n{c_id};\n"));
            out.push_str(&format!("    n{my_id} -> n{t_id};\n"));
            out.push_str(&format!("    n{my_id} -> n{f_id};\n"));
            stack.push((*false_val, f_id));
            stack.push((*true_val, t_id));
            stack.push((*select, c_id));
        }
    }
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
fn emit_property_nodes(registry: &crate::ecs::Registry, out: &mut String) {
    let mut has_props = false;
    for k in &registry.kinds {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::PROPERTY)) = k {
            has_props = true;
            break;
        }
    }
    if !has_props {
        return;
    }

    out.push_str("  // ── Safety Properties ──\n");
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(prop)) =
            (&registry.names[i], &registry.kinds[i], &registry.property_comps[i])
        {
            if let crate::ecs::EntityKind::PROPERTY = kind_comp.0 {
                let prop_id = format!("prop_{}", sanitize_id(&name_comp.0));
                let fillcolor = match prop.directive {
                    crate::ast::property::PropertyDirective::Assert => "lightblue",
                    crate::ast::property::PropertyDirective::Cover => "lightyellow",
                    crate::ast::property::PropertyDirective::Assume => "lightgreen",
                };
                out.push_str(&format!(
                    "  {prop_id} [shape=note style=filled fillcolor={fillcolor} label=\"{}\"];\n",
                    name_comp.0,
                ));

                for expr_id in &prop.formula_exprs {
                    let refs = collect_signal_refs_ecs(registry, *expr_id);
                    for sig in &refs {
                        out.push_str(&format!(
                            "  {} -> {prop_id} [style=dotted color=blue];\n",
                            sanitize_id(sig),
                        ));
                    }
                }
            }
        }
    }
    out.push('\n');
}
