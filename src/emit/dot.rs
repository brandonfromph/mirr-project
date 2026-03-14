//! Graphviz DOT emitter for MIRR IR.
//!
//! Produces a `digraph` showing signals as nodes, guards as diamond nodes,
//! reflex assignments as edges, and `Prev` back-edges as dashed red edges.
//!
//! Two detail levels:
//! - Module-level (default): one node per signal, one node per guard.
//! - Expr-level (`--dot-detail expr`): every AST node as a DOT node.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::program::Module;
use crate::ast::types::{SignalKind, SignalType};
use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};
use crate::validation::collect_signal_refs;

/// Maximum nodes to emit before truncating (prevents runaway on huge IR).
const MAX_DOT_NODES: usize = 4096;

/// Emit module-level DOT graph from pipeline results.
pub fn emit_module_dot(result: &PipelineResult) -> String {
    let module = &result.program.module;
    let mut out = String::with_capacity(2048);
    out.push_str("digraph ");
    out.push_str(&sanitize_id(&module.name));
    out.push_str(" {\n");
    out.push_str("  rankdir=LR;\n");
    out.push_str("  node [fontname=\"monospace\"];\n\n");

    emit_pattern_origin_comments(module, &mut out);
    emit_signal_nodes(module, &mut out);
    emit_guard_nodes(module, &mut out);
    emit_guard_edges(module, &mut out);
    emit_reflex_edges(module, &mut out);
    emit_property_nodes(module, &mut out);

    if let Some(netlist) = &result.temporal_netlist {
        emit_temporal_subgraph(netlist, &mut out);
    }

    out.push_str("}\n");
    out
}

/// Emit expr-level DOT graph (full AST tree per expression).
pub fn emit_expr_dot(result: &PipelineResult) -> String {
    let module = &result.program.module;
    let mut out = String::with_capacity(4096);
    out.push_str("digraph ");
    out.push_str(&sanitize_id(&module.name));
    out.push_str("_expr {\n");
    out.push_str("  rankdir=TB;\n");
    out.push_str("  node [fontname=\"monospace\"];\n\n");

    let mut node_id = 0usize;

    // Guard condition trees.
    for g in &module.guards {
        out.push_str(&format!("  subgraph cluster_guard_{} {{\n", sanitize_id(&g.name)));
        out.push_str(&format!("    label=\"guard: {}\";\n", g.name));
        emit_expr_nodes(&g.condition, &mut node_id, &mut out);
        out.push_str("  }\n");
    }

    // Reflex assignment RHS trees.
    for r in &module.reflexes {
        for a in &r.assignments {
            out.push_str(&format!(
                "  subgraph cluster_{}_{} {{\n",
                sanitize_id(&r.name),
                sanitize_id(&a.target)
            ));
            out.push_str(&format!("    label=\"{}.{}\";\n", r.name, a.target));
            emit_expr_nodes(&a.value, &mut node_id, &mut out);
            out.push_str("  }\n");
        }
    }

    out.push_str("}\n");
    out
}

// -----------------------------------------------------------------------
// Internal helpers
// -----------------------------------------------------------------------

/// Emit DOT comments listing pattern expansions applied to this module.
fn emit_pattern_origin_comments(module: &Module, out: &mut String) {
    if module.pattern_origins.is_empty() {
        return;
    }
    out.push_str("  // ── Pattern Expansions ──\n");
    for origin in &module.pattern_origins {
        out.push_str(&format!(
            "  // Expanded from pattern: {}({})\n",
            origin.pattern_name, origin.call_args_summary
        ));
    }
    out.push('\n');
}

fn emit_signal_nodes(module: &Module, out: &mut String) {
    out.push_str("  // Signals\n");
    for s in &module.signals {
        let shape = match s.kind {
            SignalKind::Input => "invhouse",
            SignalKind::Output => "house",
            SignalKind::Internal => "ellipse",
        };
        let width_label = match s.ty.signal_type() {
            SignalType::Bool => "bool".to_string(),
            SignalType::Unsigned(w) => format!("u{w}"),
            SignalType::Signed(w) => format!("i{w}"),
        };
        let tooltip = match &s.origin {
            Some(origin) => format!(" tooltip=\"Pattern: {origin}\""),
            None => String::new(),
        };
        out.push_str(&format!(
            "  {} [label=\"{}: {}\" shape={shape}{tooltip}];\n",
            sanitize_id(&s.name),
            s.name,
            width_label,
        ));
    }
    out.push('\n');
}

fn emit_guard_nodes(module: &Module, out: &mut String) {
    out.push_str("  // Guards\n");
    for g in &module.guards {
        let tooltip = match &g.origin {
            Some(origin) => format!(" tooltip=\"Pattern: {origin}\""),
            None => String::new(),
        };
        out.push_str(&format!(
            "  {} [label=\"{} ({}c)\" shape=diamond style=filled fillcolor=lightyellow{tooltip}];\n",
            guard_node_id(&g.name),
            g.name,
            g.cycles,
        ));
    }
    out.push('\n');
}

/// Edges from signals referenced in guard conditions to guard nodes.
fn emit_guard_edges(module: &Module, out: &mut String) {
    out.push_str("  // Guard inputs\n");
    for g in &module.guards {
        let refs = collect_signal_refs_bounded(&g.condition);
        for sig in &refs {
            out.push_str(&format!("  {} -> {};\n", sanitize_id(sig), guard_node_id(&g.name),));
        }
        // Prev back-edges rendered as dashed red.
        let prev_refs = collect_prev_refs_bounded(&g.condition);
        for (sig, _delay) in &prev_refs {
            out.push_str(&format!(
                "  {} -> {} [style=dashed color=red label=\"prev\"];\n",
                sanitize_id(sig),
                guard_node_id(&g.name),
            ));
        }
    }
    out.push('\n');
}

/// Edges from guard nodes to output signals via reflex assignments.
fn emit_reflex_edges(module: &Module, out: &mut String) {
    out.push_str("  // Reflex assignments\n");
    for r in &module.reflexes {
        for gname in &r.guard_names {
            for a in &r.assignments {
                out.push_str(&format!(
                    "  {} -> {} [label=\"{}\"];\n",
                    guard_node_id(gname),
                    sanitize_id(&a.target),
                    r.name,
                ));
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

/// Recursively emit AST nodes for expr-level detail.
/// Uses explicit stack (no recursion). Bounded by MAX_DOT_NODES.
fn emit_expr_nodes(expr: &Expr, node_id: &mut usize, out: &mut String) {
    let mut stack: Vec<(&Expr, usize)> = Vec::with_capacity(64);
    let root_id = *node_id;
    *node_id += 1;
    stack.push((expr, root_id));

    let mut iterations = 0usize;
    while let Some((e, my_id)) = stack.pop() {
        iterations += 1;
        if iterations > MAX_DOT_NODES {
            break;
        }
        match e {
            Expr::Literal(lit) => {
                let label = format!("{lit:?}");
                out.push_str(&format!("    n{my_id} [label=\"{label}\" shape=box];\n"));
            }
            Expr::Signal(name) => {
                out.push_str(&format!("    n{my_id} [label=\"{name}\" shape=ellipse];\n"));
            }
            Expr::Prev { signal, delay } => {
                out.push_str(&format!(
                    "    n{my_id} [label=\"prev({signal},{delay})\" shape=ellipse style=dashed color=red];\n"
                ));
            }
            Expr::Unary { op, operand } => {
                out.push_str(&format!("    n{my_id} [label=\"{op:?}\" shape=circle];\n"));
                let child_id = *node_id;
                *node_id += 1;
                out.push_str(&format!("    n{my_id} -> n{child_id};\n"));
                stack.push((operand, child_id));
            }
            Expr::Binary { op, left, right } => {
                out.push_str(&format!("    n{my_id} [label=\"{op:?}\" shape=circle];\n"));
                let left_id = *node_id;
                *node_id += 1;
                let right_id = *node_id;
                *node_id += 1;
                out.push_str(&format!("    n{my_id} -> n{left_id};\n"));
                out.push_str(&format!("    n{my_id} -> n{right_id};\n"));
                stack.push((right, right_id));
                stack.push((left, left_id));
            }
        }
    }
}

/// Collect signal references from an expression (bounded traversal).
fn collect_signal_refs_bounded(expr: &Expr) -> Vec<String> {
    let mut refs = Vec::with_capacity(16);
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_DOT_NODES {
            break;
        }
        match node {
            Expr::Signal(name) => refs.push(name.clone()),
            Expr::Prev { .. } => {} // handled separately
            Expr::Literal(_) => {}
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
        }
    }
    refs
}

/// Collect Prev back-references from an expression (bounded traversal).
fn collect_prev_refs_bounded(expr: &Expr) -> Vec<(String, u64)> {
    let mut refs = Vec::with_capacity(8);
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_DOT_NODES {
            break;
        }
        match node {
            Expr::Prev { signal, delay } => refs.push((signal.clone(), *delay)),
            Expr::Signal(_) | Expr::Literal(_) => {}
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
        }
    }
    refs
}

/// Sanitize a name for use as a DOT identifier.
fn sanitize_id(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            out.push(c);
        } else {
            out.push('_');
        }
    }
    out
}

fn guard_node_id(name: &str) -> String {
    format!("guard_{}", sanitize_id(name))
}

/// Emit property nodes and edges to referenced signals.
fn emit_property_nodes(module: &Module, out: &mut String) {
    if module.properties.is_empty() {
        return;
    }

    out.push_str("  // ── Safety Properties ──\n");
    for prop in &module.properties {
        let prop_id = format!("prop_{}", sanitize_id(&prop.name));
        let fillcolor = match prop.directive {
            crate::ast::property::PropertyDirective::Assert => "lightblue",
            crate::ast::property::PropertyDirective::Cover => "lightyellow",
            crate::ast::property::PropertyDirective::Assume => "lightgreen",
        };
        out.push_str(&format!(
            "  {prop_id} [shape=note style=filled fillcolor={fillcolor} label=\"{}\"];\n",
            prop.name,
        ));

        let exprs = prop.formula.exprs();
        for expr in exprs {
            let refs = collect_signal_refs(expr);
            for sig in &refs {
                out.push_str(&format!(
                    "  {} -> {prop_id} [style=dotted color=blue];\n",
                    sanitize_id(sig),
                ));
            }
        }
    }
    out.push('\n');
}
