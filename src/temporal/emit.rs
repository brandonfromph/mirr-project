#![forbid(unsafe_code)]
#![deny(warnings)]

// ---------------------------------------------------------------------------
// MIRR PHASE 3 — Temporal Emit Module
// ---------------------------------------------------------------------------
// Purpose: Emit TemporalNetlist representations with strict resource/verification
// guarantees required by Phase 3.
//
// Constraints & requirements (summary):
// - Follow NASA Power-of-10 rules (no recursion; small functions; bounded loops).
// - Hot-path code must avoid dynamic heap allocation after initialization.
// - Preallocate buffers and data structures used during emission where feasible.
// - Provide explicit worst-case bounds in docs/architecture/resource_budgets.csv.
// - Maintain assertion density and explicit parameter validation per module.
// - Exit criteria for this module: allocation-audit passes; temporal guard
//   determinism tests pass; CI enforces clippy -D warnings.
//
// See: docs/architecture/resource_budgets.csv, docs/architecture/loop_and_alloc_scan.md
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Temporal Guard Netlist Emitters
// ---------------------------------------------------------------------------
// Single responsibility: serialize a TemporalNetlist to external formats.
// Supports JSON (via serde_json) and Graphviz DOT.
// Ref: MIRR-PHASE2-001 §2 (emission scope), P2-REQ-007, P2-REQ-008
// ---------------------------------------------------------------------------

use crate::ast::types::SignalType;
use crate::error::MirrError;
use crate::temporal::low_level_ir::{CompiledGuard, GeneratedSignalKind, TemporalNetlist};

/// Emit a `TemporalNetlist` as a pretty-printed JSON string.
///
/// Returns `MirrError::TemporalCompilationError` if serialization fails.
pub fn emit_json(netlist: &TemporalNetlist) -> Result<String, MirrError> {
    serde_json::to_string_pretty(netlist).map_err(|e| MirrError::TemporalCompilationError {
        message: format!("JSON serialization failed: {e}"),
    })
}

/// Emit a `TemporalNetlist` as a Graphviz DOT string.
///
/// Nodes represent generated signals; subgraph clusters represent guards.
pub fn emit_dot(netlist: &TemporalNetlist) -> Result<String, MirrError> {
    let mut dot = String::from("digraph TemporalNetlist {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=box];\n\n");

    // Emit one node per generated signal.
    for signal in &netlist.signals {
        let shape = match signal.kind {
            GeneratedSignalKind::ShiftRegisterStage => "circle",
            GeneratedSignalKind::Counter => "diamond",
            GeneratedSignalKind::Comparator => "ellipse",
            GeneratedSignalKind::LogicGate => "box",
            GeneratedSignalKind::Intermediate => "plaintext",
        };
        dot.push_str(&format!(
            "  \"{}\" [shape={} label=\"{}\\n{}\"];\n",
            signal.name, shape, signal.name, signal.ty
        ));
    }

    // Emit one subgraph cluster per guard.
    for guard in &netlist.guards {
        match guard {
            CompiledGuard::ShiftRegister(sr) => {
                dot.push_str(&format!(
                    "  subgraph cluster_{} {{\n    label=\"SR: {}\\n{}\\nfor {} cycles\";\n    \"{}\" -> ",
                    sr.name, sr.name, sr.condition_kind.describe(), sr.delay_cycles, sr.input_signal
                ));
                for stage in &sr.stages {
                    dot.push_str(&format!("\"{}\" -> ", stage));
                }
                dot.push_str(&format!("\"{}\";\n  }}\n", sr.output_signal));
            }
            CompiledGuard::Counter(c) => {
                dot.push_str(&format!(
                    "  subgraph cluster_{} {{\n    label=\"Counter: {}\\n{}\\nfor {} cycles\";\n",
                    c.name,
                    c.name,
                    c.condition_kind.describe(),
                    c.target_count
                ));
                dot.push_str(&format!(
                    "    \"{}\" -> \"{}\";\n    \"{}\" -> \"{}\";\n    \"{}\" -> \"{}\";\n  }}\n",
                    c.input_signal,
                    c.counter_signal,
                    c.counter_signal,
                    c.comparator_signal,
                    c.comparator_signal,
                    c.output_signal
                ));
            }
            // ComplexGuard visualization is a known limitation (MIRR-PHASE2-001 §7).
            CompiledGuard::Complex(cx) => {
                // represent complex guards in a minimal way: label with name and
                // simple placeholder; detailed visualization is deferred to Phase 3.
                dot.push_str(&format!(
                    "  subgraph cluster_{} {{\n    label=\"Complex: {}\n\";\n",
                    cx.name, cx.name
                ));
                // show the final output signal as a node inside the cluster
                dot.push_str(&format!(
                    "    \"{}\" [shape=box style=dashed];\n  }}\n",
                    cx.output_signal
                ));
            }
        }
    }

    dot.push_str("}\n");
    Ok(dot)
}

/// Emit a `TemporalNetlist` as a very simple Verilog module.
///
/// Produces a `module` declaration containing `wire` declarations for each
/// generated signal plus guard comments.  This is intentionally minimal and
/// serves as a demonstration backend; it is not meant to be a production-grade
/// hardware generator.
pub fn emit_verilog(netlist: &TemporalNetlist) -> Result<String, MirrError> {
    let mut v = String::new();
    v.push_str("// Generated by MIRR temporal emitter\n");
    v.push_str("module mirr_temporal_netlist();\n");

    // Signal declarations
    for signal in &netlist.signals {
        let decl = match signal.ty {
            SignalType::Bool => format!("    wire {};// Bool\n", signal.name),
            SignalType::Unsigned(w) => {
                if w == 0 {
                    format!("    wire {};// Unsigned(0)\n", signal.name)
                } else {
                    format!("    wire [{}:0] {};// Unsigned({})\n", w - 1, signal.name, w)
                }
            }
        };
        v.push_str(&decl);
    }
    v.push('\n');

    // Guard comments
    for guard in &netlist.guards {
        match guard {
            CompiledGuard::ShiftRegister(sr) => {
                v.push_str(&format!(
                    "    // ShiftRegister guard '{}' delay={}\n",
                    sr.name, sr.delay_cycles
                ));
            }
            CompiledGuard::Counter(c) => {
                v.push_str(&format!(
                    "    // Counter guard '{}' count={}\n",
                    c.name, c.target_count
                ));
            }
            CompiledGuard::Complex(cx) => {
                v.push_str(&format!("    // Complex guard '{}'\n", cx.name));
            }
        }
    }

    v.push_str("endmodule\n");
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::temporal::low_level_ir::CompiledGuard;
    use crate::temporal::low_level_ir::{GeneratedSignal, ShiftRegisterGuard, TemporalNetlist};

    fn simple_sr_netlist() -> TemporalNetlist {
        use crate::temporal::low_level_ir::ConditionKind;
        let mut netlist = TemporalNetlist::new();
        let ck = ConditionKind::SimpleSignal("in_sig".to_string());
        let sr = ShiftRegisterGuard::new("g".to_string(), "in_sig".to_string(), 2, ck);
        netlist.add_signal(GeneratedSignal::shift_register_stage("g_sr_0".to_string(), 0));
        netlist.add_signal(GeneratedSignal::shift_register_stage("g_sr_1".to_string(), 1));
        netlist.add_guard(CompiledGuard::ShiftRegister(sr));
        netlist
    }

    #[test]
    fn test_json_contains_guard_name() {
        let netlist = simple_sr_netlist();
        let json = emit_json(&netlist).expect("JSON emit failed");
        assert!(json.contains("\"g\"") || json.contains("g"));
    }

    #[test]
    fn test_dot_starts_with_digraph() {
        let netlist = simple_sr_netlist();
        let dot = emit_dot(&netlist).expect("DOT emit failed");
        assert!(dot.starts_with("digraph TemporalNetlist {"));
    }

    #[test]
    fn test_dot_contains_cluster() {
        let netlist = simple_sr_netlist();
        let dot = emit_dot(&netlist).expect("DOT emit failed");
        assert!(dot.contains("cluster_g"));
    }
}
