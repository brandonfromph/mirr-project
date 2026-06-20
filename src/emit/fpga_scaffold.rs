//! FPGA constraint and build script scaffold generator.
//!
//! Generates vendor-specific project files:
//! - Xilinx: `.xdc` constraints + `build.tcl` for Vivado
//! - Intel: `.sdc` constraints + `.qsf` settings for Quartus
//! - Lattice: `.pcf` constraints + `build.sh` for Yosys+nextpnr
//! - Generic: `.sdc` constraints only
//!
//! All generated files are clearly marked as scaffolding with
//! PLACEHOLDER pins that must be filled in by the engineer.

#![forbid(unsafe_code)]

use crate::ast::types::SignalKind;
use crate::emit::fpga_target::FpgaTarget;
use crate::emit::fpga_target::MAX_CONSTRAINT_LINES;
use crate::pipeline::PipelineResult;

struct Port {
    name: String,
    kind: SignalKind,
    width: u32,
}

fn get_ports(registry: &crate::ecs::Registry) -> Vec<Port> {
    let mut ports = Vec::new();
    for i in 0..registry.names.len() {
        if let (Some(name), Some(kind), Some(ty)) =
            (&registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            use crate::ecs::components::EntityKind;
            if let EntityKind::SIGNAL(skind) = kind.0 {
                let width = ty.0.core.width();
                ports.push(Port {
                    name: registry.resolve_name(name.0).to_string(),
                    kind: skind,
                    width,
                });
            }
        }
    }
    ports
}

fn get_main_clock(registry: &crate::ecs::Registry) -> String {
    for ty in registry.types.iter().flatten() {
        if let Some(cd) = &ty.0.annotations.clock_domain {
            return cd.clone();
        }
    }
    "clk".to_string()
}

/// Emit a constraint file for the given FPGA target.
pub fn emit_constraints(result: &PipelineResult, target: &FpgaTarget) -> String {
    let registry = match &result.ecs_registry {
        Some(r) => r,
        None => return "// No ECS registry available\n".to_string(),
    };
    match target {
        FpgaTarget::Xilinx7 | FpgaTarget::XilinxUS => emit_xdc(registry, target),
        FpgaTarget::IntelCyclone => emit_sdc(registry, target),
        FpgaTarget::LatticeIce40 => emit_pcf(registry),
        FpgaTarget::LatticeEcp5 => emit_lpf(registry, target),
        FpgaTarget::LatticeNexus => emit_pdc(registry, target),
        FpgaTarget::Generic => emit_sdc(registry, target),
    }
}

/// Emit a build script for the given FPGA target.
pub fn emit_build_script(result: &PipelineResult, target: &FpgaTarget) -> String {
    let registry = match &result.ecs_registry {
        Some(r) => r,
        None => return "// No ECS registry available\n".to_string(),
    };
    match target {
        FpgaTarget::Xilinx7 | FpgaTarget::XilinxUS => emit_vivado_tcl(registry, target),
        FpgaTarget::IntelCyclone => emit_quartus_tcl(registry, target),
        FpgaTarget::LatticeIce40 => emit_lattice_sh(registry, target),
        FpgaTarget::LatticeEcp5 => emit_ecp5_sh(registry, target),
        FpgaTarget::LatticeNexus => emit_nexus_sh(registry, target),
        FpgaTarget::Generic => emit_yosys_sh(registry),
    }
}

// -----------------------------------------------------------------------
// Xilinx XDC
// -----------------------------------------------------------------------

fn emit_xdc(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(1024);
    let mut lines = 0usize;

    out.push_str(&format!(
        "## Auto-generated XDC constraints for {} ({})\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string()),
        target.display_name()
    ));
    out.push_str("## Fill in PACKAGE_PIN values for your board.\n\n");

    let main_clock = get_main_clock(registry);
    out.push_str(&format!("create_clock -period 10.000 -name {0} [get_ports {0}]\n", main_clock));
    lines += 1;

    // Port constraints.
    for s in &get_ports(registry) {
        if lines >= MAX_CONSTRAINT_LINES {
            break;
        }
        if s.kind == SignalKind::Internal {
            continue;
        }
        let width = s.width;
        if width == 1 {
            out.push_str(&format!(
                "# set_property PACKAGE_PIN {{PLACEHOLDER}} [get_ports {}]\n",
                s.name
            ));
            out.push_str(&format!("# set_property IOSTANDARD LVCMOS33 [get_ports {}]\n", s.name));
            lines += 2;
        } else {
            let mut bit = 0u32;
            while bit < width && lines < MAX_CONSTRAINT_LINES {
                out.push_str(&format!(
                    "# set_property PACKAGE_PIN {{PLACEHOLDER}} [get_ports {{{}[{}]}}]\n",
                    s.name, bit
                ));
                lines += 1;
                bit += 1;
            }
        }
    }

    out
}

// -----------------------------------------------------------------------
// Intel SDC
// -----------------------------------------------------------------------

fn emit_sdc(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(1024);

    out.push_str(&format!(
        "## Auto-generated SDC constraints for {} ({})\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string()),
        target.display_name()
    ));
    out.push_str("## Fill in pin assignments for your board.\n\n");

    let main_clock = get_main_clock(registry);
    out.push_str(&format!("create_clock -period 10.000 -name {0} [get_ports {0}]\n", main_clock));
    out.push_str("derive_pll_clocks\n");
    out.push_str("derive_clock_uncertainty\n\n");

    // Input/output delay constraints.
    for s in &get_ports(registry) {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let constraint = match s.kind {
            SignalKind::Input => "set_input_delay",
            SignalKind::Output => "set_output_delay",
            SignalKind::Internal => continue,
        };
        out.push_str(&format!("{} -clock {} 2.000 [get_ports {}]\n", constraint, main_clock, s.name));
    }

    out
}

// -----------------------------------------------------------------------
// Lattice PCF
// -----------------------------------------------------------------------

fn emit_pcf(registry: &crate::ecs::Registry) -> String {
    let mut out = String::with_capacity(512);

    out.push_str(&format!(
        "# Auto-generated PCF constraints for {}\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str("# Fill in pin numbers for your board.\n\n");

    for s in &get_ports(registry) {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let width = s.width;
        if width == 1 {
            out.push_str(&format!("set_io {} PLACEHOLDER\n", s.name));
        } else {
            let mut bit = 0u32;
            while bit < width {
                out.push_str(&format!("set_io {}[{}] PLACEHOLDER\n", s.name, bit));
                bit += 1;
            }
        }
    }

    out
}

// -----------------------------------------------------------------------
// Vivado build.tcl
// -----------------------------------------------------------------------

fn emit_vivado_tcl(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str(&format!(
        "# Auto-generated Vivado build script for {}\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "create_project {} ./build -part {} -force\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string()),
        target.default_part()
    ));
    out.push_str(&format!(
        "add_files {}.sv\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "add_files -fileset constrs_1 {}.xdc\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str("launch_runs synth_1 -jobs 4\n");
    out.push_str("wait_on_run synth_1\n");
    out.push_str("launch_runs impl_1 -to_step write_bitstream -jobs 4\n");
    out.push_str("wait_on_run impl_1\n");

    out
}

// -----------------------------------------------------------------------
// Quartus build.tcl
// -----------------------------------------------------------------------

fn emit_quartus_tcl(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str(&format!(
        "# Auto-generated Quartus build script for {}\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "project_new {} -overwrite\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str("set_global_assignment -name FAMILY \"Cyclone V\"\n");
    out.push_str(&format!("set_global_assignment -name DEVICE {}\n", target.default_part()));
    out.push_str(&format!(
        "set_global_assignment -name SYSTEMVERILOG_FILE {}.sv\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "set_global_assignment -name SDC_FILE {}.sdc\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str("execute_flow -compile\n");
    out.push_str("project_close\n");

    out
}

// -----------------------------------------------------------------------
// Lattice build.sh (Yosys + nextpnr)
// -----------------------------------------------------------------------

fn emit_lattice_sh(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str("#!/usr/bin/env bash\n");
    out.push_str(&format!(
        "# Auto-generated build script for {} ({})\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string()),
        target.display_name()
    ));
    out.push_str("set -euo pipefail\n\n");
    out.push_str(&format!(
        "# NOTE: Use --strip-sva when generating {}.sv for synthesis\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "yosys -p \"read_verilog -sv {0}.sv; synth_ice40 -top {0} -json {0}.json\"\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "nextpnr-ice40 --hx8k --package ct256 --json {0}.json --pcf {0}.pcf --asc {0}.asc\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "icepack {0}.asc {0}.bin\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "echo \"Bitstream ready: {}.bin\"\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));

    out
}

// -----------------------------------------------------------------------
// Generic Yosys build.sh
// -----------------------------------------------------------------------

fn emit_yosys_sh(registry: &crate::ecs::Registry) -> String {
    let mut out = String::with_capacity(256);

    out.push_str("#!/usr/bin/env bash\n");
    out.push_str(&format!(
        "# Auto-generated Yosys synthesis script for {}\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str("set -euo pipefail\n\n");
    out.push_str(&format!(
        "# NOTE: Use --strip-sva when generating {}.sv for synthesis\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "yosys -p \"read_verilog -sv {0}.sv; synth -top {0}; write_json {0}.json\"\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "echo \"Netlist ready: {}.json\"\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));

    out
}

// -----------------------------------------------------------------------
// Lattice ECP5 LPF constraints
// -----------------------------------------------------------------------

fn emit_lpf(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(1024);

    out.push_str(&format!(
        "# Auto-generated LPF constraints for {} ({})\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string()),
        target.display_name()
    ));
    out.push_str("# Fill in LOC values for your board.\n\n");

    let main_clock = get_main_clock(registry);
    out.push_str(&format!("FREQUENCY NET \"{}\" 100.000000 MHz;\n\n", main_clock));

    for s in &get_ports(registry) {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let width = s.width;
        if width == 1 {
            out.push_str(&format!("LOCATE COMP \"{}\" SITE \"PLACEHOLDER\";\n", s.name));
            out.push_str(&format!("IOBUF PORT \"{}\" IO_TYPE=LVCMOS33;\n", s.name));
        } else {
            let mut bit = 0u32;
            while bit < width {
                out.push_str(&format!(
                    "LOCATE COMP \"{}[{}]\" SITE \"PLACEHOLDER\";\n",
                    s.name, bit
                ));
                bit += 1;
            }
        }
    }

    out
}

// -----------------------------------------------------------------------
// Lattice Nexus PDC constraints
// -----------------------------------------------------------------------

fn emit_pdc(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(1024);

    out.push_str(&format!(
        "# Auto-generated PDC constraints for {} ({})\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string()),
        target.display_name()
    ));
    out.push_str("# Fill in pin assignments for your board.\n\n");

    let main_clock = get_main_clock(registry);
    out.push_str(&format!("create_clock -name {{{0}}} -period 10.000 [get_ports {0}]\n\n", main_clock));

    for s in &get_ports(registry) {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let width = s.width;
        if width == 1 {
            out.push_str(&format!(
                "ldc_set_location -site {{PLACEHOLDER}} [get_ports {{{}}}]\n",
                s.name
            ));
        } else {
            let mut bit = 0u32;
            while bit < width {
                out.push_str(&format!(
                    "ldc_set_location -site {{PLACEHOLDER}} [get_ports {{{}[{}]}}]\n",
                    s.name, bit
                ));
                bit += 1;
            }
        }
    }

    out
}

// -----------------------------------------------------------------------
// ECP5 build.sh (Yosys + nextpnr-ecp5 + ecppack)
// -----------------------------------------------------------------------

fn emit_ecp5_sh(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str("#!/usr/bin/env bash\n");
    out.push_str(&format!(
        "# Auto-generated build script for {} ({})\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string()),
        target.display_name()
    ));
    out.push_str("set -euo pipefail\n\n");
    out.push_str(&format!(
        "# NOTE: Use --strip-sva when generating {}.sv for synthesis\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "yosys -p \"read_verilog -sv {0}.sv; synth_ecp5 -top {0} -json {0}.json\"\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "nextpnr-ecp5 --85k --package CABGA381 --json {0}.json --lpf {0}.lpf --textcfg {0}.config\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "ecppack {0}.config {0}.bit\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "echo \"Bitstream ready: {}.bit\"\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));

    out
}

// -----------------------------------------------------------------------
// Nexus build.sh (Yosys + nextpnr-nexus + prjoxide)
// -----------------------------------------------------------------------

fn emit_nexus_sh(registry: &crate::ecs::Registry, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str("#!/usr/bin/env bash\n");
    out.push_str(&format!(
        "# Auto-generated build script for {} ({})\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string()),
        target.display_name()
    ));
    out.push_str("set -euo pipefail\n\n");
    out.push_str(&format!(
        "# NOTE: Use --strip-sva when generating {}.sv for synthesis\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "yosys -p \"read_verilog -sv {0}.sv; synth_nexus -top {0} -json {0}.json\"\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "nextpnr-nexus --device {} --json {1}.json --pdc {1}.pdc --fasm {1}.fasm\n",
        target.default_part(),
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "prjoxide pack {0}.fasm {0}.bit\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));
    out.push_str(&format!(
        "echo \"Bitstream ready: {}.bit\"\n",
        registry.get_module_name().unwrap_or_else(|| "unknown_module".to_string())
    ));

    out
}
