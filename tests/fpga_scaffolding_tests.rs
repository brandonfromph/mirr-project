//! Integration tests for FPGA scaffolding and target configuration.

#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::emit::fpga_target::FpgaTarget;

#[test]
fn fpga_target_parse_all() {
    assert_eq!(FpgaTarget::from_str_name("generic"), Some(FpgaTarget::Generic));
    assert_eq!(FpgaTarget::from_str_name("xilinx-7"), Some(FpgaTarget::Xilinx7));
    assert_eq!(FpgaTarget::from_str_name("ultrascale"), Some(FpgaTarget::XilinxUS));
    assert_eq!(FpgaTarget::from_str_name("lattice-ice40"), Some(FpgaTarget::LatticeIce40));
    assert_eq!(FpgaTarget::from_str_name("lattice-ecp5"), Some(FpgaTarget::LatticeEcp5));
    assert_eq!(FpgaTarget::from_str_name("lattice-nexus"), Some(FpgaTarget::LatticeNexus));
    assert_eq!(FpgaTarget::from_str_name("invalid"), None);
}

#[test]
fn fpga_constraint_extensions() {
    assert_eq!(FpgaTarget::Generic.constraint_extension(), "sdc");
    assert_eq!(FpgaTarget::Xilinx7.constraint_extension(), "xdc");
    assert_eq!(FpgaTarget::LatticeIce40.constraint_extension(), "pcf");
    assert_eq!(FpgaTarget::LatticeEcp5.constraint_extension(), "lpf");
}

#[test]
fn fpga_clock_primitives() {
    assert_eq!(FpgaTarget::Xilinx7.clock_primitive(), "MMCME2_BASE");
    assert_eq!(FpgaTarget::XilinxUS.clock_primitive(), "MMCME4_ADV");
    assert_eq!(FpgaTarget::LatticeIce40.clock_primitive(), "SB_PLL40_CORE");
}

#[test]
fn fpga_build_tools() {
    assert_eq!(FpgaTarget::Xilinx7.build_tool(), "vivado");
    assert_eq!(FpgaTarget::LatticeIce40.build_tool(), "nextpnr-ice40");
    assert_eq!(FpgaTarget::IntelCyclone.build_tool(), "quartus_sh");
}

#[test]
fn fpga_default_parts() {
    assert!(!FpgaTarget::Xilinx7.default_part().is_empty());
    assert!(!FpgaTarget::LatticeIce40.default_part().is_empty());
}

#[test]
fn fpga_dsp_primitives() {
    assert_eq!(FpgaTarget::Xilinx7.dsp_primitive(), "DSP48E1");
    assert_eq!(FpgaTarget::LatticeIce40.dsp_primitive(), "SB_MAC16");
}

#[test]
fn fpga_yosys_synth() {
    assert_eq!(FpgaTarget::LatticeIce40.yosys_synth_command(), "synth_ice40");
    assert_eq!(FpgaTarget::LatticeEcp5.yosys_synth_command(), "synth_ecp5");
}

#[test]
fn fpga_display_names() {
    assert_eq!(FpgaTarget::Xilinx7.display_name(), "Xilinx 7-Series");
    assert_eq!(FpgaTarget::LatticeIce40.display_name(), "Lattice iCE40");
}

#[test]
fn fpga_nextpnr_binary() {
    assert_eq!(FpgaTarget::LatticeIce40.nextpnr_binary(), Some("nextpnr-ice40"));
    assert_eq!(FpgaTarget::Generic.nextpnr_binary(), None);
}

#[test]
fn fpga_pack_tool() {
    assert_eq!(FpgaTarget::LatticeIce40.pack_tool(), Some("icepack"));
    assert_eq!(FpgaTarget::Generic.pack_tool(), None);
}
