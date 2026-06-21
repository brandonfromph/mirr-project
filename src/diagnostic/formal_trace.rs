//! Formal Trace Analyzer
//!
//! Subsystem for translating a formal trace and provenance graph into native compiler diagnostics.

use crate::diagnostic::{Diagnostic, Label, LabelKind};
use crate::emit::provenance::ProvenanceGraph;
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

/// Analyze a formal verification failure and build a `FormalTraceReport`.
///
/// Takes the in-memory `ProvenanceGraph` and (optionally) the path to the formal
/// engine's `trace.vcd` (or similar) to construct the causal traceback.
pub fn analyze_failure(
    graph: &ProvenanceGraph,
    failed_prop: &str,
    trace_path: Option<&std::path::Path>,
    _file_table: &crate::span::FileTable,
) -> FormalTraceReport {
    let mut causal_chain = Vec::new();
    let mut current_signal = failed_prop.to_string();
    let mut origin_span = None;

    let mut final_state = None;
    if let Some(path) = trace_path {
        if let Ok(state_map) = crate::diagnostic::vcd_parser::parse_vcd_final_state(path) {
            final_state = Some(state_map);
        }
    }

    // Simple causal traversal mock (to be expanded with VCD parsing)
    // We walk backwards through the provenance graph.
    for _ in 0..8 {
        if let Some(node) = graph.nodes.get(&current_signal) {
            let span = node.origin;

            if origin_span.is_none() {
                origin_span = span;
            } else {
                let mut value_info = None;
                if let Some(ref state_map) = final_state {
                    if let Some(val) = state_map.get(&node.signal) {
                        // Format the binary value nicely
                        if val.len() == 1 {
                            value_info = Some(if val == "1" {
                                "true".to_string()
                            } else {
                                "false".to_string()
                            });
                        } else {
                            // Convert long binary to Hex if it fits in u64, otherwise keep binary
                            if let Ok(num) = u64::from_str_radix(val, 2) {
                                value_info = Some(format!("{}'h{:X}", val.len(), num));
                            } else {
                                value_info = Some(format!("{}'b{}", val.len(), val));
                            }
                        }
                    }
                }

                causal_chain.push(TraceNode { signal: node.signal.clone(), span, value_info });
            }

            if let Some(next_sig) = node.depends_on.first() {
                current_signal = next_sig.clone();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    FormalTraceReport { failed_property: failed_prop.to_string(), causal_chain, origin_span }
}
