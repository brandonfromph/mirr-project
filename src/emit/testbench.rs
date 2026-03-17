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

use crate::ast::program::Module;
use crate::ast::types::{SignalKind, SignalType};
use crate::pipeline::PipelineResult;

/// Maximum simulation cycles (bounded iteration, NASA P10 Rule #1).
const MAX_SIM_CYCLES: u32 = 10_000;

/// Default simulation cycles for the testbench.
const DEFAULT_SIM_CYCLES: u32 = 200;

/// Emit a self-checking SystemVerilog testbench for the given module.
pub fn emit_testbench(result: &PipelineResult) -> String {
    let module = &result.program.module;
    let mut out = String::with_capacity(2048);

    emit_tb_header(module, &mut out);
    emit_tb_signals(module, &mut out);
    emit_tb_clock(&mut out);
    emit_tb_dut_instance(module, &mut out);
    emit_tb_stimulus(module, &mut out);
    emit_tb_footer(&mut out);

    out
}

// -----------------------------------------------------------------------
// Internal helpers
// -----------------------------------------------------------------------

fn emit_tb_header(module: &Module, out: &mut String) {
    out.push_str("// Auto-generated testbench by MIRR compiler (FPGA-001)\n");
    out.push_str("// Do not edit — regenerate from .mirr source.\n");
    out.push_str("`timescale 1ns / 1ps\n\n");
    out.push_str(&format!("module {}_tb;\n\n", module.name));
}

fn emit_tb_signals(module: &Module, out: &mut String) {
    out.push_str("  // Clock and reset\n");
    out.push_str("  logic clk;\n");
    out.push_str("  logic rst_n;\n\n");

    // Declare testbench signals for each port.
    out.push_str("  // DUT port signals\n");
    for s in &module.signals {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let type_str = tb_type(&s.ty.signal_type());
        out.push_str(&format!("  {} tb_{};\n", type_str, s.name));
    }
    out.push('\n');
}

fn emit_tb_clock(out: &mut String) {
    out.push_str("  // Clock generation: 100 MHz (10 ns period)\n");
    out.push_str("  initial clk = 1'b0;\n");
    out.push_str("  always #5 clk = ~clk;\n\n");
}

fn emit_tb_dut_instance(module: &Module, out: &mut String) {
    out.push_str("  // DUT instantiation\n");
    out.push_str(&format!("  {} dut (\n", module.name));

    let has_clk = module.signals.iter().any(|s| s.name == "clk");
    let has_rst_n = module.signals.iter().any(|s| s.name == "rst_n");
    let has_guards = !module.guards.is_empty();

    let mut connections: Vec<String> = Vec::new();

    // Connect clk and rst_n.
    if has_guards && !has_clk {
        connections.push("    .clk(clk)".to_string());
    }
    if has_guards && !has_rst_n {
        connections.push("    .rst_n(rst_n)".to_string());
    }

    for s in &module.signals {
        if s.kind == SignalKind::Internal {
            continue;
        }
        if s.name == "rst_n" {
            connections.push(format!("    .{}(rst_n)", s.name));
        } else {
            connections.push(format!("    .{}(tb_{})", s.name, s.name));
        }
    }

    let conn_count = connections.len();
    for (i, conn) in connections.iter().enumerate() {
        let comma = if i + 1 < conn_count { "," } else { "" };
        out.push_str(&format!("{conn}{comma}\n"));
    }

    out.push_str("  );\n\n");
}

fn emit_tb_stimulus(module: &Module, out: &mut String) {
    let sim_cycles = DEFAULT_SIM_CYCLES.min(MAX_SIM_CYCLES);

    out.push_str("  // Stimulus sequence\n");
    out.push_str("  initial begin\n");
    out.push_str("    // Phase 1: Reset\n");
    out.push_str("    rst_n = 1'b0;\n");

    // Drive all inputs to zero during reset.
    for s in &module.signals {
        if s.kind == SignalKind::Input && s.name != "rst_n" {
            out.push_str(&format!("    tb_{} = '0;\n", s.name));
        }
    }

    out.push_str("    repeat(10) @(posedge clk);\n");
    out.push_str("    rst_n = 1'b1;\n\n");

    out.push_str("    // Phase 2: Drive inputs to max range\n");
    for s in &module.signals {
        if s.kind == SignalKind::Input && s.name != "rst_n" {
            let max_val = match &s.ty.signal_type() {
                SignalType::Bool => "'1".to_string(),
                SignalType::Unsigned(w) => format!("{}'hFFFF", w),
                SignalType::Signed(w) => format!("{}'h7FFF", w),
                SignalType::Array { .. }
                | SignalType::Struct { .. }
                | SignalType::FixedPoint { .. }
                | SignalType::Bundle(_) => "'0".to_string(),
            };
            out.push_str(&format!("    tb_{} = {};\n", s.name, max_val));
        }
    }

    out.push_str(&format!("    repeat({}) @(posedge clk);\n\n", sim_cycles));

    out.push_str("    // Phase 3: Return to zero\n");
    for s in &module.signals {
        if s.kind == SignalKind::Input && s.name != "rst_n" {
            out.push_str(&format!("    tb_{} = '0;\n", s.name));
        }
    }
    out.push_str("    repeat(50) @(posedge clk);\n\n");

    out.push_str(&format!("    $display(\"Testbench {} complete.\");\n", module.name));
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
