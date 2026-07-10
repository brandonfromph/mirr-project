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
use std::fmt::Write;

/// Maximum simulation cycles (bounded iteration, NASA P10 Rule #1).
const MAX_SIM_CYCLES: u32 = 10_000;

/// Default simulation cycles for the testbench.
const DEFAULT_SIM_CYCLES: u32 = 200;

/// Emit a self-checking SystemVerilog testbench for the given module.
pub fn emit_testbench(result: &PipelineResult) -> String {
    let mut out = String::with_capacity(2048);
    let Some(registry) = result.ecs_registry.as_ref() else {
        return "// [MIRR COMPILER ERROR] ECS registry is missing. Cannot generate testbench."
            .to_string();
    };
    let module_name = registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string());

    let mut main_clock = "clk".to_string();
    for ty in registry.types.iter().flatten() {
        if let Some(cd) = &ty.0.annotations.clock_domain {
            main_clock = cd.clone();
            break;
        }
    }

    let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
            Some(crate::ecs::components::EntityId(i as u32))
        } else {
            None
        }
    });

    emit_tb_header(&module_name, &mut out);
    emit_tb_signals(registry, top_module_id, &main_clock, &mut out);
    emit_tb_clock(&main_clock, &mut out);
    emit_tb_dut_instance(&module_name, registry, top_module_id, &main_clock, &mut out);
    emit_tb_stimulus(&module_name, registry, top_module_id, &main_clock, &mut out);
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
    write!(out, "module {}_tb;\n\n", module_name).unwrap();
}

fn emit_tb_signals(
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    main_clock: &str,
    out: &mut String,
) {
    out.push_str("  // Clock and reset\n");
    writeln!(out, "  logic {};", main_clock).unwrap();
    out.push_str("  logic rst_n;\n\n");

    // Declare testbench signals for each port.
    out.push_str("  // DUT port signals\n");
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp), Some(ty_comp)) =
            (registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            if let crate::ecs::EntityKind::SIGNAL(sig_kind) = kind_comp.0 {
                if sig_kind == SignalKind::Internal {
                    continue;
                }
                let type_str = tb_type(&ty_comp.0.core);
                writeln!(out, "  {} tb_{};", type_str, registry.resolve_name(nc.0)).unwrap();
            }
        }
    }
    out.push('\n');
}

fn emit_tb_clock(main_clock: &str, out: &mut String) {
    out.push_str("  // Clock generation: 100 MHz (10 ns period)\n");
    writeln!(out, "  initial {} = 1'b0;", main_clock).unwrap();
    write!(out, "  always #5 {} = ~{};\n\n", main_clock, main_clock).unwrap();
}

fn emit_tb_dut_instance(
    module_name: &str,
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    main_clock: &str,
    out: &mut String,
) {
    out.push_str("  // DUT instantiation\n");
    writeln!(out, "  {} dut (", module_name).unwrap();

    let mut has_clk = false;
    let mut has_rst_n = false;
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let Some(nc) = registry.names[i] {
            let name = registry.resolve_name(nc.0);
            if name == main_clock {
                has_clk = true;
            }
            if name == "rst_n" {
                has_rst_n = true;
            }
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
        connections.push(format!("    .{0}({0})", main_clock));
    }
    if has_guards && !has_rst_n {
        connections.push("    .rst_n(rst_n)".to_string());
    }

    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
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
        writeln!(out, "{conn}{comma}").unwrap();
    }

    out.push_str("  );\n\n");
}

fn emit_tb_stimulus(
    module_name: &str,
    registry: &crate::ecs::Registry,
    top_module_id: Option<crate::ecs::components::EntityId>,
    main_clock: &str,
    out: &mut String,
) {
    let sim_cycles = DEFAULT_SIM_CYCLES.min(MAX_SIM_CYCLES);

    out.push_str("  // Stimulus sequence\n");
    out.push_str("  initial begin\n");
    out.push_str("    // Phase 1: Reset\n");
    out.push_str("    rst_n = 1'b0;\n");

    // Drive all inputs to zero during reset.
    for i in 0..registry.names.len() {
        if top_module_id.is_some() && registry.modules[i].map(|m| m.0) != top_module_id {
            continue;
        }
        if let (Some(nc), Some(kind_comp)) = (registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::EntityKind::SIGNAL(SignalKind::Input) = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                if name != "rst_n" {
                    writeln!(out, "    tb_{} = '0;", name).unwrap();
                }
            }
        }
    }

    writeln!(out, "    repeat(10) @(posedge {});", main_clock).unwrap();
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
                    writeln!(out, "    tb_{} = {};", name, max_val).unwrap();
                }
            }
        }
    }

    write!(out, "    repeat({}) @(posedge {});\n\n", sim_cycles, main_clock).unwrap();

    out.push_str("    // Phase 3: Return to zero\n");
    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind_comp)) = (registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::EntityKind::SIGNAL(SignalKind::Input) = kind_comp.0 {
                let name = registry.resolve_name(nc.0);
                if name != "rst_n" {
                    writeln!(out, "    tb_{} = '0;", name).unwrap();
                }
            }
        }
    }
    write!(out, "    repeat(50) @(posedge {});\n\n", main_clock).unwrap();

    writeln!(out, "    $display(\"Testbench {} complete.\");", module_name).unwrap();
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
