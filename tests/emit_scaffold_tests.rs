//! FPGA scaffold emission tests.
//!
//! Verifies constraint file and build script generation for all supported
//! FPGA targets: Xilinx, Intel, Lattice, and Generic.

use nasa_rust_project::emit::fpga_scaffold;
use nasa_rust_project::emit::fpga_target::FpgaTarget;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

const SIMPLE_MODULE: &str = r#"
module scaffold_test {
    signal clk_in: in bool;
    signal data: in u8;
    signal result: out u8;

    guard g {
        when clk_in
        for 2 cycles;
    }

    reflex r {
        on g {
            result = data;
        }
    }
}
"#;

fn pipeline_result() -> nasa_rust_project::pipeline::PipelineResult {
    let config = PipelineConfig::default();
    run_pipeline(SIMPLE_MODULE, &config).expect("pipeline should succeed")
}

// ---------------------------------------------------------------------------
// Constraint file tests
// ---------------------------------------------------------------------------

#[test]
fn xdc_contains_create_clock() {
    let result = pipeline_result();
    let xdc = fpga_scaffold::emit_constraints(&result, &FpgaTarget::Xilinx7);
    assert!(xdc.contains("create_clock"), "XDC should contain create_clock");
    assert!(xdc.contains("-period 10.000"), "XDC should specify clock period");
}

#[test]
fn xdc_contains_package_pin_placeholders() {
    let result = pipeline_result();
    let xdc = fpga_scaffold::emit_constraints(&result, &FpgaTarget::Xilinx7);
    assert!(xdc.contains("PACKAGE_PIN"), "XDC should have PACKAGE_PIN placeholders");
    assert!(xdc.contains("IOSTANDARD"), "XDC should have IOSTANDARD");
}

#[test]
fn sdc_contains_derive_clocks() {
    let result = pipeline_result();
    let sdc = fpga_scaffold::emit_constraints(&result, &FpgaTarget::IntelCyclone);
    assert!(sdc.contains("derive_pll_clocks"), "SDC should have derive_pll_clocks");
    assert!(sdc.contains("derive_clock_uncertainty"), "SDC should have derive_clock_uncertainty");
}

#[test]
fn pcf_contains_set_io() {
    let result = pipeline_result();
    let pcf = fpga_scaffold::emit_constraints(&result, &FpgaTarget::LatticeIce40);
    assert!(pcf.contains("set_io"), "PCF should have set_io directives");
    assert!(pcf.contains("PLACEHOLDER"), "PCF should have PLACEHOLDER pins");
}

#[test]
fn generic_sdc_emitted() {
    let result = pipeline_result();
    let sdc = fpga_scaffold::emit_constraints(&result, &FpgaTarget::Generic);
    assert!(sdc.contains("create_clock"), "Generic SDC should have create_clock");
}

// ---------------------------------------------------------------------------
// Build script tests
// ---------------------------------------------------------------------------

#[test]
fn vivado_tcl_has_create_project() {
    let result = pipeline_result();
    let tcl = fpga_scaffold::emit_build_script(&result, &FpgaTarget::Xilinx7);
    assert!(tcl.contains("create_project"), "Vivado TCL should have create_project");
    assert!(tcl.contains("xc7a35t"), "Vivado TCL should have Artix-7 part");
    assert!(tcl.contains("launch_runs"), "Vivado TCL should have launch_runs");
}

#[test]
fn quartus_tcl_has_project_new() {
    let result = pipeline_result();
    let tcl = fpga_scaffold::emit_build_script(&result, &FpgaTarget::IntelCyclone);
    assert!(tcl.contains("project_new"), "Quartus TCL should have project_new");
    assert!(tcl.contains("Cyclone V"), "Quartus TCL should have Cyclone V family");
}

#[test]
fn lattice_sh_has_yosys_nextpnr() {
    let result = pipeline_result();
    let sh = fpga_scaffold::emit_build_script(&result, &FpgaTarget::LatticeIce40);
    assert!(sh.contains("yosys"), "Lattice script should use yosys");
    assert!(sh.contains("nextpnr-ice40"), "Lattice script should use nextpnr-ice40");
    assert!(sh.contains("icepack"), "Lattice script should use icepack");
}

#[test]
fn generic_sh_has_yosys() {
    let result = pipeline_result();
    let sh = fpga_scaffold::emit_build_script(&result, &FpgaTarget::Generic);
    assert!(sh.contains("yosys"), "Generic script should use yosys");
}
