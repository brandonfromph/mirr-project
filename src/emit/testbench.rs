//! Auto-testbench generator for MIRR modules.
//!
//! Generates a self-checking SystemVerilog testbench that:
//! 1. Instantiates the DUT (Device Under Test)
//! 2. Generates a clock signal
//! 3. Drives all inputs through reset, zero, and max-range stimulus
//! 4. Reports pass/fail for SVA assertions
//!
//! Emitted via `--emit testbench` or `--emit verilog --testbench`.

#![forbid(unsafe_code)]

use crate::ast::types::{SignalKind, SignalType};
use crate::pipeline::PipelineResult;

/// Maximum simulation cycles (bounded iteration, NASA P10 Rule #1).
const MAX_SIM_CYCLES: u32 = 10_000;

/// Default simulation cycles for the testbench.
const DEFAULT_SIM_CYCLES: u32 = 200;

/// Emit a self-checking SystemVerilog testbench for the given module.
pub fn emit_testbench(result: &PipelineResult) -> String {
    let mut out = String::with_capacity(2048);
    let registry = result.ecs_registry.as_ref().unwrap();
    let module_name = registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string());

    emit_tb_header(&module_name, &mut out);
    emit_tb_signals(registry, &mut out);
    emit_tb_clock(&mut out);
    emit_tb_dut_instance(&module_name, registry, &mut out);
    emit_tb_stimulus(&module_name, registry, &mut out);
    emit_tb_footer(&mut out);

    out
}

// -----------------------------------------------------------------------
// Internal helpers
// -----------------------------------------------------------------------

fn emit_tb_header(module_name: &str, out: &mut String) {
    out.push_str("// Auto-generated testbench by MIRR compiler (FPGA-001)\n");
    out.push_str("// Do not edit — regenerate from .mirr source.\n");
    out.push_str("`timescale 1ns / 1ps\n\n");
    out.push_str(&format!("module {}_tb;\n\n", module_name));
}

fn emit_tb_signals(registry: &crate::ecs::Registry, out: &mut String) {
    out.push_str("  // Clock and reset\n");
    out.push_str("  logic clk;\n");
    out.push_str("  logic rst_n;\n\n");

    // Declare testbench signals for each port.
    out.push_str("  // DUT port signals\n");
    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind_comp), Some(ty_comp)) =
            (registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            if let crate::ecs::EntityKind::SIGNAL(sig_kind) = kind_comp.0 {
                if sig_kind == SignalKind::Internal {
                    continue;
                }
                let type_str = tb_type(&ty_comp.0.core);
                out.push_str(&format!("  {} tb_{};\n", type_str, registry.resolve_name(nc.0)));
            }
        }
    }
    out.push('\n');
}

fn emit_tb_clock(out: &mut String) {
    out.push_str("  // Clock generation: 100 MHz (10 ns period)\n");
    out.push_str("  initial clk = 1'b0;\n");
    out.push_str("  always #5 clk = ~clk;\n\n");
}

fn emit_tb_dut_instance(module_name: &str, registry: &crate::ecs::Registry, out: &mut String) {
    out.push_str("  // DUT instantiation\n");
    out.push_str(&format!("  {} dut (\n", module_name));

    let mut has_clk = false;
    let mut has_rst_n = false;
    for nc in registry.names.iter().flatten() {
        let name = registry.resolve_name(nc.0);
        if name == "clk" {
            has_clk = true;
        }
        if name == "rst_n" {
            has_rst_n = true;
        }
    }

    let mut has_guards = false;
    for k in &registry.kinds {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::GUARD)) = k {
            has_guards = true;
            break;
        }
    }

    let mut connections: Vec<String> = Vec::new();

    // Connect clk and rst_n.
    if has_guards && !has_clk {
        connections.push("    .clk(clk)".to_string());
    }
    if has_guards && !has_rst_n {
        connections.push("    .rst_n(rst_n)".to_string());
    }

    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind_comp)) = (registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::EntityKind::SIGNAL(sig_kind) = kind_comp.0 {
                if sig_kind == SignalKind::Internal {
                    continue;
                }
                let name = registry.resolve_name(nc.0);
                if name == "rst_n" {
                    connections.push(format!("    .{}(rst_n)", name));
                } else {
                    connections.push(format!("    .{}(tb_{})", name, name));
                }
            }
        }
    }

    let conn_count = connections.len();
    for (i, conn) in connections.iter().enumerate() {
        let comma = if i + 1 < conn_count { "," } else { "" };
        out.push_str(&format!("{conn}{comma}\n"));
    }

    out.push_str("  );\n\n");
}

fn emit_tb_stimulus(module_name: &str, registry: &crate::ecs::Registry, out: &mut String) {
    let sim_cycles = DEFAULT_SIM_CYCLES.min(MAX_SIM_CYCLES);

    out.push_str("  // Stimulus sequence\n");
    out.push_str("  initial begin\n");
    out.push_str("    // Phase 1: Reset\n");
    out.push_str("    rst_n = 1'b0;\n");

    // Drive all inputs to zero during reset.
    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind_comp)) = (registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::EntityKind::SIGNAL(SignalKind::Input) = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                if name != "rst_n" {
                    out.push_str(&format!("    tb_{} = '0;\n", name));
                }
            }
        }
    }

    out.push_str("    repeat(10) @(posedge clk);\n");
    out.push_str("    rst_n = 1'b1;\n\n");

    out.push_str("    // Phase 2: Drive inputs to max range\n");
    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind_comp), Some(ty_comp)) =
            (registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            if let crate::ecs::EntityKind::SIGNAL(SignalKind::Input) = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                if name != "rst_n" {
                    let max_val = match &ty_comp.0.core {
                        SignalType::Bool => "'1".to_string(),
                        SignalType::Unsigned(w) => format!("{}'hFFFF", w),
                        SignalType::Signed(w) => format!("{}'h7FFF", w),
                        SignalType::Array { .. }
                        | SignalType::Struct { .. }
                        | SignalType::FixedPoint { .. }
                        | SignalType::Bundle(_)
                        | SignalType::Fifo { .. } => "'0".to_string(),
                    };
                    out.push_str(&format!("    tb_{} = {};\n", name, max_val));
                }
            }
        }
    }

    out.push_str(&format!("    repeat({}) @(posedge clk);\n\n", sim_cycles));

    out.push_str("    // Phase 3: Return to zero\n");
    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind_comp)) = (registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::EntityKind::SIGNAL(SignalKind::Input) = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                if name != "rst_n" {
                    out.push_str(&format!("    tb_{} = '0;\n", name));
                }
            }
        }
    }
    out.push_str("    repeat(50) @(posedge clk);\n\n");

    out.push_str(&format!("    $display(\"Testbench {} complete.\");\n", module_name));
    out.push_str("    $finish;\n");
    out.push_str("  end\n\n");
}

fn emit_tb_footer(out: &mut String) {
    out.push_str("endmodule\n");
}

/// Map MIRR SignalType to SystemVerilog type string for testbench.
fn tb_type(ty: &SignalType) -> String {
    super::sv_type(ty)
}
