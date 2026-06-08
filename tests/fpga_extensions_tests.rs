#![forbid(unsafe_code)]

//! Integration tests for ECP5 and Nexus FPGA target extensions.
//!
//! Covers parsing, constraint files, build scripts, and all accessor
//! methods introduced for the Lattice ECP5 and Lattice Nexus targets.

use mirrc::emit::fpga_scaffold;
use mirrc::emit::fpga_target::FpgaTarget;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

/// A minimal MIRR program used by scaffold emission tests.
const TEST_MIRR: &str = r#"
module test_fpga {
    signal sensor: in u16;
    signal alarm: out bool;

    guard check {
        when sensor > 100
        for 5 cycles;
    }

    reflex act {
        on check {
            alarm = true;
        }
    }
}
"#;

/// Run the pipeline on `TEST_MIRR` and return the result.
fn pipeline_result() -> mirrc::pipeline::PipelineResult {
    let config = PipelineConfig::default();
    run_pipeline(TEST_MIRR, &config).expect("pipeline should succeed for test_fpga module")
}

// ===========================================================================
// 1. FpgaTarget parsing from string
// ===========================================================================

#[test]
fn parse_lattice_ecp5_from_canonical_name() {
    assert_eq!(FpgaTarget::from_str_name("lattice-ecp5"), Some(FpgaTarget::LatticeEcp5));
}

#[test]
fn parse_lattice_ecp5_from_short_name() {
    assert_eq!(FpgaTarget::from_str_name("ecp5"), Some(FpgaTarget::LatticeEcp5));
}

#[test]
fn parse_lattice_nexus_from_canonical_name() {
    assert_eq!(FpgaTarget::from_str_name("lattice-nexus"), Some(FpgaTarget::LatticeNexus));
}

#[test]
fn parse_lattice_nexus_from_short_name() {
    assert_eq!(FpgaTarget::from_str_name("nexus"), Some(FpgaTarget::LatticeNexus));
}

#[test]
fn parse_lattice_nexus_from_crosslink_alias() {
    assert_eq!(FpgaTarget::from_str_name("crosslink-nx"), Some(FpgaTarget::LatticeNexus));
}

// ===========================================================================
// 2. Constraint extension for new targets
// ===========================================================================

#[test]
fn ecp5_constraint_extension_is_lpf() {
    assert_eq!(FpgaTarget::LatticeEcp5.constraint_extension(), "lpf");
}

#[test]
fn nexus_constraint_extension_is_pdc() {
    assert_eq!(FpgaTarget::LatticeNexus.constraint_extension(), "pdc");
}

// ===========================================================================
// 3. Clock primitives for new targets
// ===========================================================================

#[test]
fn ecp5_clock_primitive_is_ehxplll() {
    assert_eq!(FpgaTarget::LatticeEcp5.clock_primitive(), "EHXPLLL");
}

#[test]
fn nexus_clock_primitive_is_osca() {
    assert_eq!(FpgaTarget::LatticeNexus.clock_primitive(), "OSCA");
}

// ===========================================================================
// 4. Build tools for new targets
// ===========================================================================

#[test]
fn ecp5_build_tool_is_nextpnr_ecp5() {
    assert_eq!(FpgaTarget::LatticeEcp5.build_tool(), "nextpnr-ecp5");
}

#[test]
fn nexus_build_tool_is_nextpnr_nexus() {
    assert_eq!(FpgaTarget::LatticeNexus.build_tool(), "nextpnr-nexus");
}

// ===========================================================================
// 5. Default parts for new targets
// ===========================================================================

#[test]
fn ecp5_default_part() {
    let part = FpgaTarget::LatticeEcp5.default_part();
    assert!(part.contains("LFE5U"), "ECP5 default part should be an LFE5U device, got: {part}");
}

#[test]
fn nexus_default_part() {
    let part = FpgaTarget::LatticeNexus.default_part();
    assert!(part.contains("LIFCL"), "Nexus default part should be a LIFCL device, got: {part}");
}

// ===========================================================================
// 6. Display names
// ===========================================================================

#[test]
fn ecp5_display_name() {
    assert_eq!(FpgaTarget::LatticeEcp5.display_name(), "Lattice ECP5");
}

#[test]
fn nexus_display_name() {
    assert_eq!(FpgaTarget::LatticeNexus.display_name(), "Lattice Nexus");
}

// ===========================================================================
// 7. DSP primitives and max input widths
// ===========================================================================

#[test]
fn ecp5_dsp_primitive_and_width() {
    assert_eq!(FpgaTarget::LatticeEcp5.dsp_primitive(), "ALU54B");
    assert_eq!(FpgaTarget::LatticeEcp5.dsp_max_input_width(), 18);
}

#[test]
fn nexus_dsp_primitive_and_width() {
    assert_eq!(FpgaTarget::LatticeNexus.dsp_primitive(), "MULT18X18");
    assert_eq!(FpgaTarget::LatticeNexus.dsp_max_input_width(), 18);
}

// ===========================================================================
// 8. New methods: nextpnr_binary, icetime_device, yosys_synth_command,
//    pack_tool
// ===========================================================================

#[test]
fn ecp5_nextpnr_binary() {
    assert_eq!(FpgaTarget::LatticeEcp5.nextpnr_binary(), Some("nextpnr-ecp5"));
}

#[test]
fn nexus_nextpnr_binary() {
    assert_eq!(FpgaTarget::LatticeNexus.nextpnr_binary(), Some("nextpnr-nexus"));
}

#[test]
fn ecp5_and_nexus_icetime_device_is_none() {
    // icetime is iCE40-only; both ECP5 and Nexus should return None.
    assert_eq!(FpgaTarget::LatticeEcp5.icetime_device(), None);
    assert_eq!(FpgaTarget::LatticeNexus.icetime_device(), None);
}

#[test]
fn ecp5_yosys_synth_command() {
    assert_eq!(FpgaTarget::LatticeEcp5.yosys_synth_command(), "synth_ecp5");
}

#[test]
fn nexus_yosys_synth_command() {
    assert_eq!(FpgaTarget::LatticeNexus.yosys_synth_command(), "synth_nexus");
}

#[test]
fn ecp5_pack_tool() {
    assert_eq!(FpgaTarget::LatticeEcp5.pack_tool(), Some("ecppack"));
}

#[test]
fn nexus_pack_tool() {
    assert_eq!(FpgaTarget::LatticeNexus.pack_tool(), Some("prjoxide"));
}

// ===========================================================================
// 9. ECP5 and Nexus constraint generation (emit_constraints)
// ===========================================================================

#[test]
fn ecp5_constraints_contain_frequency_and_locate() {
    let result = pipeline_result();
    let lpf = fpga_scaffold::emit_constraints(&result, &FpgaTarget::LatticeEcp5);

    assert!(lpf.contains("FREQUENCY NET"), "LPF should contain FREQUENCY NET directive");
    assert!(lpf.contains("100.000000 MHz"), "LPF should specify 100 MHz clock");
    assert!(lpf.contains("LOCATE COMP"), "LPF should contain LOCATE COMP for pins");
    assert!(lpf.contains("IOBUF PORT"), "LPF should set IO_TYPE via IOBUF PORT");
    assert!(lpf.contains("LVCMOS33"), "LPF should default to LVCMOS33 IO standard");
    assert!(lpf.contains("Lattice ECP5"), "LPF header should mention Lattice ECP5");
}

#[test]
fn nexus_constraints_contain_create_clock_and_ldc() {
    let result = pipeline_result();
    let pdc = fpga_scaffold::emit_constraints(&result, &FpgaTarget::LatticeNexus);

    assert!(pdc.contains("create_clock"), "PDC should contain create_clock");
    assert!(pdc.contains("-period 10.000"), "PDC should specify 10 ns period");
    assert!(pdc.contains("ldc_set_location"), "PDC should contain ldc_set_location");
    assert!(pdc.contains("PLACEHOLDER"), "PDC should have PLACEHOLDER sites");
    assert!(pdc.contains("Lattice Nexus"), "PDC header should mention Lattice Nexus");
}

// ===========================================================================
// 10. ECP5 and Nexus build script generation (emit_build_script)
// ===========================================================================

#[test]
fn ecp5_build_script_has_yosys_nextpnr_ecppack() {
    let result = pipeline_result();
    let sh = fpga_scaffold::emit_build_script(&result, &FpgaTarget::LatticeEcp5);

    assert!(sh.starts_with("#!/usr/bin/env bash"), "ECP5 script should have bash shebang");
    assert!(sh.contains("set -euo pipefail"), "ECP5 script should use strict mode");
    assert!(sh.contains("synth_ecp5"), "ECP5 script should call synth_ecp5");
    assert!(sh.contains("nextpnr-ecp5"), "ECP5 script should invoke nextpnr-ecp5");
    assert!(sh.contains("--85k"), "ECP5 script should target 85k device");
    assert!(sh.contains("ecppack"), "ECP5 script should use ecppack for bitstream");
    assert!(sh.contains(".bit"), "ECP5 script should produce a .bit file");
}

#[test]
fn nexus_build_script_has_yosys_nextpnr_prjoxide() {
    let result = pipeline_result();
    let sh = fpga_scaffold::emit_build_script(&result, &FpgaTarget::LatticeNexus);

    assert!(sh.starts_with("#!/usr/bin/env bash"), "Nexus script should have bash shebang");
    assert!(sh.contains("set -euo pipefail"), "Nexus script should use strict mode");
    assert!(sh.contains("synth_nexus"), "Nexus script should call synth_nexus");
    assert!(sh.contains("nextpnr-nexus"), "Nexus script should invoke nextpnr-nexus");
    assert!(sh.contains("LIFCL-40"), "Nexus script should target the LIFCL-40 device");
    assert!(sh.contains("prjoxide"), "Nexus script should use prjoxide for bitstream packing");
    assert!(sh.contains(".fasm"), "Nexus script should produce a .fasm intermediate");
    assert!(sh.contains(".bit"), "Nexus script should produce a .bit file");
}
