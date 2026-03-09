//! FPGA target definitions for vendor-specific emission.
//!
//! Each target describes the constraint file extension, clock primitive,
//! and build tool used by that FPGA family. The emitter uses this to
//! generate correct scaffolding files.

#![forbid(unsafe_code)]

/// Supported FPGA target families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FpgaTarget {
    /// Generic target — no vendor-specific features.
    #[default]
    Generic,
    /// Xilinx 7-series (Artix-7, Kintex-7, Zynq-7000).
    Xilinx7,
    /// Xilinx UltraScale / UltraScale+.
    XilinxUS,
    /// Intel Cyclone V / Cyclone 10.
    IntelCyclone,
    /// Lattice iCE40 (Yosys + nextpnr flow).
    LatticeIce40,
    /// Lattice ECP5 (Yosys + nextpnr-ecp5 flow).
    LatticeEcp5,
    /// Lattice Nexus / CrossLink-NX (Yosys + nextpnr-nexus flow).
    LatticeNexus,
}

/// Maximum synchronizer stages allowed.
pub const MAX_SYNC_STAGES: u32 = 4;

/// Maximum lines in a generated constraint file.
pub const MAX_CONSTRAINT_LINES: usize = 256;

impl FpgaTarget {
    /// Parse a target name from a CLI string.
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s {
            "generic" => Some(Self::Generic),
            "xilinx-7" | "xilinx7" => Some(Self::Xilinx7),
            "xilinx-us" | "ultrascale" => Some(Self::XilinxUS),
            "intel-cyclone" | "cyclone" => Some(Self::IntelCyclone),
            "lattice-ice40" | "ice40" => Some(Self::LatticeIce40),
            "lattice-ecp5" | "ecp5" => Some(Self::LatticeEcp5),
            "lattice-nexus" | "nexus" | "crosslink-nx" => Some(Self::LatticeNexus),
            _ => None,
        }
    }

    /// File extension for the constraint file.
    pub fn constraint_extension(&self) -> &'static str {
        match self {
            Self::Generic => "sdc",
            Self::Xilinx7 | Self::XilinxUS => "xdc",
            Self::IntelCyclone => "sdc",
            Self::LatticeIce40 => "pcf",
            Self::LatticeEcp5 => "lpf",
            Self::LatticeNexus => "pdc",
        }
    }

    /// Name of the clock primitive for this family.
    pub fn clock_primitive(&self) -> &'static str {
        match self {
            Self::Generic => "PLL",
            Self::Xilinx7 => "MMCME2_BASE",
            Self::XilinxUS => "MMCME4_ADV",
            Self::IntelCyclone => "altpll",
            Self::LatticeIce40 => "SB_PLL40_CORE",
            Self::LatticeEcp5 => "EHXPLLL",
            Self::LatticeNexus => "OSCA",
        }
    }

    /// Build tool command for this family.
    pub fn build_tool(&self) -> &'static str {
        match self {
            Self::Generic => "yosys",
            Self::Xilinx7 | Self::XilinxUS => "vivado",
            Self::IntelCyclone => "quartus_sh",
            Self::LatticeIce40 => "nextpnr-ice40",
            Self::LatticeEcp5 => "nextpnr-ecp5",
            Self::LatticeNexus => "nextpnr-nexus",
        }
    }

    /// FPGA part string for build script templates.
    pub fn default_part(&self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Xilinx7 => "xc7a35tcpg236-1",
            Self::XilinxUS => "xcku5p-ffvb676-2-e",
            Self::IntelCyclone => "5CSEBA6U23I7",
            Self::LatticeIce40 => "iCE40-HX8K-CT256",
            Self::LatticeEcp5 => "LFE5U-85F-6BG381C",
            Self::LatticeNexus => "LIFCL-40-9BG400C",
        }
    }

    /// Display name for comments and headers.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Generic => "Generic",
            Self::Xilinx7 => "Xilinx 7-Series",
            Self::XilinxUS => "Xilinx UltraScale+",
            Self::IntelCyclone => "Intel Cyclone",
            Self::LatticeIce40 => "Lattice iCE40",
            Self::LatticeEcp5 => "Lattice ECP5",
            Self::LatticeNexus => "Lattice Nexus",
        }
    }

    /// Vendor-specific DSP primitive name.
    pub fn dsp_primitive(&self) -> &'static str {
        match self {
            Self::Generic => "dsp",
            Self::Xilinx7 => "DSP48E1",
            Self::XilinxUS => "DSP48E2",
            Self::IntelCyclone => "cyclonev_mac",
            Self::LatticeIce40 => "SB_MAC16",
            Self::LatticeEcp5 => "ALU54B",
            Self::LatticeNexus => "MULT18X18",
        }
    }

    /// Synthesis attribute to force DSP mapping for multiply operations.
    pub fn dsp_attribute(&self) -> &'static str {
        match self {
            Self::Generic => "(* use_dsp = \"yes\" *)",
            Self::Xilinx7 | Self::XilinxUS => "(* use_dsp48 = \"yes\" *)",
            Self::IntelCyclone => "(* multstyle = \"dsp\" *)",
            Self::LatticeIce40 => "(* use_dsp = \"yes\" *)",
            Self::LatticeEcp5 => "(* use_dsp = \"yes\" *)",
            Self::LatticeNexus => "(* use_dsp = \"yes\" *)",
        }
    }

    /// Maximum single-operand input width for the vendor DSP block.
    pub fn dsp_max_input_width(&self) -> u32 {
        match self {
            Self::Generic => 18,
            Self::Xilinx7 => 25,
            Self::XilinxUS => 27,
            Self::IntelCyclone => 27,
            Self::LatticeIce40 => 16,
            Self::LatticeEcp5 => 18,
            Self::LatticeNexus => 18,
        }
    }

    /// The nextpnr binary name for this target, if applicable.
    pub fn nextpnr_binary(&self) -> Option<&'static str> {
        match self {
            Self::LatticeIce40 => Some("nextpnr-ice40"),
            Self::LatticeEcp5 => Some("nextpnr-ecp5"),
            Self::LatticeNexus => Some("nextpnr-nexus"),
            _ => None,
        }
    }

    /// The icetime device flag for static timing analysis (iCE40 only).
    pub fn icetime_device(&self) -> Option<&'static str> {
        match self {
            Self::LatticeIce40 => Some("hx8k"),
            _ => None,
        }
    }

    /// The Yosys synthesis command for this target.
    pub fn yosys_synth_command(&self) -> &'static str {
        match self {
            Self::LatticeIce40 => "synth_ice40",
            Self::LatticeEcp5 => "synth_ecp5",
            Self::LatticeNexus => "synth_nexus",
            Self::Xilinx7 | Self::XilinxUS => "synth_xilinx",
            Self::IntelCyclone => "synth_intel",
            Self::Generic => "synth",
        }
    }

    /// The bitstream packing tool for this target, if applicable.
    pub fn pack_tool(&self) -> Option<&'static str> {
        match self {
            Self::LatticeIce40 => Some("icepack"),
            Self::LatticeEcp5 => Some("ecppack"),
            Self::LatticeNexus => Some("prjoxide"),
            _ => None,
        }
    }
}
