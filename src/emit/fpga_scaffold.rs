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

use crate::ast::program::Module;
use crate::ast::types::{SignalKind, SignalType};
use crate::emit::fpga_target::FpgaTarget;
use crate::emit::fpga_target::MAX_CONSTRAINT_LINES;
use crate::pipeline::PipelineResult;

/// Emit a constraint file for the given FPGA target.
pub fn emit_constraints(result: &PipelineResult, target: &FpgaTarget) -> String {
    let module = &result.program.module;
    match target {
        FpgaTarget::Xilinx7 | FpgaTarget::XilinxUS => emit_xdc(module, target),
        FpgaTarget::IntelCyclone => emit_sdc(module, target),
        FpgaTarget::LatticeIce40 => emit_pcf(module),
        FpgaTarget::LatticeEcp5 => emit_lpf(module, target),
        FpgaTarget::LatticeNexus => emit_pdc(module, target),
        FpgaTarget::Generic => emit_sdc(module, target),
    }
}

/// Emit a build script for the given FPGA target.
pub fn emit_build_script(result: &PipelineResult, target: &FpgaTarget) -> String {
    let module = &result.program.module;
    match target {
        FpgaTarget::Xilinx7 | FpgaTarget::XilinxUS => emit_vivado_tcl(module, target),
        FpgaTarget::IntelCyclone => emit_quartus_tcl(module, target),
        FpgaTarget::LatticeIce40 => emit_lattice_sh(module, target),
        FpgaTarget::LatticeEcp5 => emit_ecp5_sh(module, target),
        FpgaTarget::LatticeNexus => emit_nexus_sh(module, target),
        FpgaTarget::Generic => emit_yosys_sh(module),
    }
}

// -----------------------------------------------------------------------
// Xilinx XDC
// -----------------------------------------------------------------------

fn emit_xdc(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(1024);
    let mut lines = 0usize;

    out.push_str(&format!(
        "## Auto-generated XDC constraints for {} ({})\n",
        module.name,
        target.display_name()
    ));
    out.push_str("## Fill in PACKAGE_PIN values for your board.\n\n");

    // Clock constraint.
    out.push_str("create_clock -period 10.000 -name clk [get_ports clk]\n");
    lines += 1;

    // Port constraints.
    for s in &module.signals {
        if lines >= MAX_CONSTRAINT_LINES {
            break;
        }
        if s.kind == SignalKind::Internal {
            continue;
        }
        let width = signal_width(&s.ty);
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

fn emit_sdc(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(1024);

    out.push_str(&format!(
        "## Auto-generated SDC constraints for {} ({})\n",
        module.name,
        target.display_name()
    ));
    out.push_str("## Fill in pin assignments for your board.\n\n");

    out.push_str("create_clock -period 10.000 -name clk [get_ports clk]\n");
    out.push_str("derive_pll_clocks\n");
    out.push_str("derive_clock_uncertainty\n\n");

    // Input/output delay constraints.
    for s in &module.signals {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let constraint = match s.kind {
            SignalKind::Input => "set_input_delay",
            SignalKind::Output => "set_output_delay",
            SignalKind::Internal => continue,
        };
        out.push_str(&format!("{} -clock clk 2.000 [get_ports {}]\n", constraint, s.name));
    }

    out
}

// -----------------------------------------------------------------------
// Lattice PCF
// -----------------------------------------------------------------------

fn emit_pcf(module: &Module) -> String {
    let mut out = String::with_capacity(512);

    out.push_str(&format!("# Auto-generated PCF constraints for {}\n", module.name));
    out.push_str("# Fill in pin numbers for your board.\n\n");

    for s in &module.signals {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let width = signal_width(&s.ty);
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

fn emit_vivado_tcl(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str(&format!("# Auto-generated Vivado build script for {}\n", module.name));
    out.push_str(&format!(
        "create_project {} ./build -part {} -force\n",
        module.name,
        target.default_part()
    ));
    out.push_str(&format!("add_files {}.sv\n", module.name));
    out.push_str(&format!("add_files -fileset constrs_1 {}.xdc\n", module.name));
    out.push_str("launch_runs synth_1 -jobs 4\n");
    out.push_str("wait_on_run synth_1\n");
    out.push_str("launch_runs impl_1 -to_step write_bitstream -jobs 4\n");
    out.push_str("wait_on_run impl_1\n");

    out
}

// -----------------------------------------------------------------------
// Quartus build.tcl
// -----------------------------------------------------------------------

fn emit_quartus_tcl(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str(&format!("# Auto-generated Quartus build script for {}\n", module.name));
    out.push_str(&format!("project_new {} -overwrite\n", module.name));
    out.push_str("set_global_assignment -name FAMILY \"Cyclone V\"\n");
    out.push_str(&format!("set_global_assignment -name DEVICE {}\n", target.default_part()));
    out.push_str(&format!("set_global_assignment -name SYSTEMVERILOG_FILE {}.sv\n", module.name));
    out.push_str(&format!("set_global_assignment -name SDC_FILE {}.sdc\n", module.name));
    out.push_str("execute_flow -compile\n");
    out.push_str("project_close\n");

    out
}

// -----------------------------------------------------------------------
// Lattice build.sh (Yosys + nextpnr)
// -----------------------------------------------------------------------

fn emit_lattice_sh(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str("#!/usr/bin/env bash\n");
    out.push_str(&format!(
        "# Auto-generated build script for {} ({})\n",
        module.name,
        target.display_name()
    ));
    out.push_str("set -euo pipefail\n\n");
    out.push_str(&format!(
        "# NOTE: Use --strip-sva when generating {}.sv for synthesis\n",
        module.name
    ));
    out.push_str(&format!(
        "yosys -p \"read_verilog -sv {0}.sv; synth_ice40 -top {0} -json {0}.json\"\n",
        module.name
    ));
    out.push_str(&format!(
        "nextpnr-ice40 --hx8k --package ct256 --json {0}.json --pcf {0}.pcf --asc {0}.asc\n",
        module.name
    ));
    out.push_str(&format!("icepack {0}.asc {0}.bin\n", module.name));
    out.push_str(&format!("echo \"Bitstream ready: {}.bin\"\n", module.name));

    out
}

// -----------------------------------------------------------------------
// Generic Yosys build.sh
// -----------------------------------------------------------------------

fn emit_yosys_sh(module: &Module) -> String {
    let mut out = String::with_capacity(256);

    out.push_str("#!/usr/bin/env bash\n");
    out.push_str(&format!("# Auto-generated Yosys synthesis script for {}\n", module.name));
    out.push_str("set -euo pipefail\n\n");
    out.push_str(&format!(
        "# NOTE: Use --strip-sva when generating {}.sv for synthesis\n",
        module.name
    ));
    out.push_str(&format!(
        "yosys -p \"read_verilog -sv {0}.sv; synth -top {0}; write_json {0}.json\"\n",
        module.name
    ));
    out.push_str(&format!("echo \"Netlist ready: {}.json\"\n", module.name));

    out
}

// -----------------------------------------------------------------------
// Lattice ECP5 LPF constraints
// -----------------------------------------------------------------------

fn emit_lpf(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(1024);

    out.push_str(&format!(
        "# Auto-generated LPF constraints for {} ({})\n",
        module.name,
        target.display_name()
    ));
    out.push_str("# Fill in LOC values for your board.\n\n");

    out.push_str("FREQUENCY NET \"clk\" 100.000000 MHz;\n\n");

    for s in &module.signals {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let width = signal_width(&s.ty);
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

fn emit_pdc(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(1024);

    out.push_str(&format!(
        "# Auto-generated PDC constraints for {} ({})\n",
        module.name,
        target.display_name()
    ));
    out.push_str("# Fill in pin assignments for your board.\n\n");

    out.push_str("create_clock -name {clk} -period 10.000 [get_ports clk]\n\n");

    for s in &module.signals {
        if s.kind == SignalKind::Internal {
            continue;
        }
        let width = signal_width(&s.ty);
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

fn emit_ecp5_sh(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str("#!/usr/bin/env bash\n");
    out.push_str(&format!(
        "# Auto-generated build script for {} ({})\n",
        module.name,
        target.display_name()
    ));
    out.push_str("set -euo pipefail\n\n");
    out.push_str(&format!(
        "# NOTE: Use --strip-sva when generating {}.sv for synthesis\n",
        module.name
    ));
    out.push_str(&format!(
        "yosys -p \"read_verilog -sv {0}.sv; synth_ecp5 -top {0} -json {0}.json\"\n",
        module.name
    ));
    out.push_str(&format!(
        "nextpnr-ecp5 --85k --package CABGA381 --json {0}.json --lpf {0}.lpf --textcfg {0}.config\n",
        module.name
    ));
    out.push_str(&format!("ecppack {0}.config {0}.bit\n", module.name));
    out.push_str(&format!("echo \"Bitstream ready: {}.bit\"\n", module.name));

    out
}

// -----------------------------------------------------------------------
// Nexus build.sh (Yosys + nextpnr-nexus + prjoxide)
// -----------------------------------------------------------------------

fn emit_nexus_sh(module: &Module, target: &FpgaTarget) -> String {
    let mut out = String::with_capacity(512);

    out.push_str("#!/usr/bin/env bash\n");
    out.push_str(&format!(
        "# Auto-generated build script for {} ({})\n",
        module.name,
        target.display_name()
    ));
    out.push_str("set -euo pipefail\n\n");
    out.push_str(&format!(
        "# NOTE: Use --strip-sva when generating {}.sv for synthesis\n",
        module.name
    ));
    out.push_str(&format!(
        "yosys -p \"read_verilog -sv {0}.sv; synth_nexus -top {0} -json {0}.json\"\n",
        module.name
    ));
    out.push_str(&format!(
        "nextpnr-nexus --device {} --json {1}.json --pdc {1}.pdc --fasm {1}.fasm\n",
        target.default_part(),
        module.name
    ));
    out.push_str(&format!("prjoxide pack {0}.fasm {0}.bit\n", module.name));
    out.push_str(&format!("echo \"Bitstream ready: {}.bit\"\n", module.name));

    out
}

// -----------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------

fn signal_width(ty: &SignalType) -> u32 {
    match ty {
        SignalType::Bool => 1,
        SignalType::Unsigned(w) | SignalType::Signed(w) => *w,
    }
}
