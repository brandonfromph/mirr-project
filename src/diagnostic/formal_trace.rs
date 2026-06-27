//! Formal Trace Analyzer
//!
//! Subsystem for translating a formal trace and provenance graph into native compiler diagnostics.

use crate::diagnostic::{Diagnostic, Label, LabelKind};

use crate::error_codes::ErrorCode;
use crate::span::Span;

/// A structured report describing a formal invariant failure.
pub struct FormalTraceReport {
    /// The flattened Verilog name of the signal/property that failed.
    pub failed_property: String,
    /// The causal chain of dependencies leading to the failure.
    pub causal_chain: Vec<TraceNode>,
    /// The span of the property that failed, if available.
    pub origin_span: Option<Span>,
}

pub struct TraceNode {
    pub signal: String,
    pub span: Option<Span>,
    pub value_info: Option<String>,
}

impl FormalTraceReport {
    /// Converts this structured report into native terminal-ready `Diagnostic` objects.
    pub fn to_diagnostics(&self) -> Vec<Diagnostic> {
        let mut diag =
            Diagnostic::error(format!("Formal property `{}` violated", self.failed_property))
                .with_code(ErrorCode::FormalInvariantViolated.as_str())
                .with_span(self.origin_span);

        // Map the causal chain into diagnostic labels (up to MAX_LABELS limit).
        // The diagnostic engine natively supports pointing to multiple spans.
        for (i, node) in self.causal_chain.iter().enumerate() {
            if i >= 8 {
                break; // Adhere to MAX_LABELS in the diagnostic engine
            }

            let mut message = format!("Originating signal: `{}`", node.signal);
            if let Some(ref val) = node.value_info {
                message = format!("{} (value: {})", message, val);
            }

            diag = diag.with_label(Label { span: node.span, message, kind: LabelKind::Note });
        }

        vec![diag]
    }
}

use crate::ecs::components::{EntityId, EntityKind, KindComponent};
use crate::ecs::Registry;
use std::collections::{BTreeMap, BTreeSet};

/// Analyze a formal verification failure and build a `FormalTraceReport`.
///
/// Takes the ECS `Registry` and (optionally) the path to the formal
/// engine's `trace.vcd` (or similar) to construct the causal traceback natively.
pub fn analyze_failure(
    registry: &Registry,
    failed_prop: &str,
    trace_path: Option<&std::path::Path>,
    target_step: Option<usize>,
    _file_table: &crate::span::FileTable,
) -> FormalTraceReport {
    let mut causal_chain = Vec::new();
    let mut current_signal = failed_prop.to_string();
    let mut origin_span = None;

    let mut final_state = None;
    if let Some(path) = trace_path {
        if let Ok(state_map) =
            crate::diagnostic::vcd_parser::parse_vcd_state_at_step(path, target_step)
        {
            final_state = Some(state_map);
        }
    }

    // Build a reverse lookup from Assignment -> Target Entity
    // and Target Entity -> Expression Root Entity.
    let mut target_to_expr = BTreeMap::new();
    for i in 0..registry.names.len() {
        if let Some(crate::ecs::components::AssignmentComponent { target, value, .. }) =
            &registry.assignment_comps[i]
        {
            target_to_expr.insert(*target, *value);
        }
    }

    // Traverse causal chain backward
    for _ in 0..8 {
        let entity_id = match registry.get_entity_by_name(&current_signal) {
            Some(id) => id,
            None => break,
        };

        let idx = entity_id.0 as usize;
        let span = registry.spans[idx].map(|s| s.0);

        if origin_span.is_none() {
            origin_span = span;
        } else {
            let mut value_info = None;
            if let Some(ref state_map) = final_state {
                if let Some(val) = state_map.get(&current_signal) {
                    if val.len() == 1 {
                        value_info =
                            Some(if val == "1" { "true".to_string() } else { "false".to_string() });
                    } else {
                        if let Ok(num) = u64::from_str_radix(val, 2) {
                            value_info = Some(format!("{}'h{:X}", val.len(), num));
                        } else {
                            value_info = Some(format!("{}'b{}", val.len(), val));
                        }
                    }
                }
            }

            causal_chain.push(TraceNode { signal: current_signal.clone(), span, value_info });
        }

        let mut deps = BTreeSet::new();

        if let Some(KindComponent(kind)) = &registry.kinds[idx] {
            if matches!(kind, EntityKind::SIGNAL(_)) {
                if let Some(expr_root) = target_to_expr.get(&entity_id) {
                    collect_dependencies(*expr_root, registry, &mut deps);
                }
            } else if let Some(prop_comp) = &registry.property_comps[idx] {
                for expr_id in &prop_comp.formula_exprs {
                    collect_dependencies(*expr_id, registry, &mut deps);
                }
            }
        }

        if let Some(next_sig) = deps.into_iter().next() {
            current_signal = next_sig;
        } else {
            break;
        }
    }

    FormalTraceReport { failed_property: failed_prop.to_string(), causal_chain, origin_span }
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
