---
title: FPGA Targets Guide
nav_order: 10
---

# FPGA Targets Guide

MIRR generates vendor-specific constraint files and build scripts for
six FPGA target families. This enables one-command synthesis-to-bitstream
workflows for prototyping safety-critical hardware monitors on real
FPGA development boards.

---

## Overview

When you compile a MIRR module, the compiler generates SystemVerilog
(`.sv`) output. To place that design onto a real FPGA, you need two
additional artifacts:

1. **Constraint file** -- maps logical signal names to physical pins
   on your FPGA board and defines clock timing.
2. **Build script** -- invokes the vendor synthesis, place-and-route,
   and bitstream generation tools in the correct order.

The MIRR FPGA scaffold generator (`src/emit/fpga_scaffold.rs` and
`src/emit/fpga_target.rs`) produces both artifacts for your chosen
target family. All generated files contain `PLACEHOLDER` pin
assignments that you must fill in for your specific board.

---

## Supported FPGA families

MIRR supports six FPGA targets plus a generic fallback:

| Target name | Family | Vendor | Build tool | Default part |
|-------------|--------|--------|------------|--------------|
| `generic` | Generic | -- | Yosys | -- |
| `xilinx-7` | Xilinx 7-Series | AMD/Xilinx | Vivado | xc7a35tcpg236-1 |
| `xilinx-us` | Xilinx UltraScale+ | AMD/Xilinx | Vivado | xcku5p-ffvb676-2-e |
| `intel-cyclone` | Cyclone V/10 | Intel/Altera | Quartus | 5CSEBA6U23I7 |
| `lattice-ice40` | iCE40 | Lattice | nextpnr-ice40 | iCE40-HX8K-CT256 |
| `lattice-ecp5` | ECP5 | Lattice | nextpnr-ecp5 | LFE5U-85F-6BG381C |
| `lattice-nexus` | Nexus/CrossLink-NX | Lattice | nextpnr-nexus | LIFCL-40-9BG400C |

### CLI name aliases

Several aliases are accepted for convenience:

| Canonical name | Also accepts |
|---------------|-------------|
| `xilinx-7` | `xilinx7` |
| `xilinx-us` | `ultrascale` |
| `intel-cyclone` | `cyclone` |
| `lattice-ice40` | `ice40` |
| `lattice-ecp5` | `ecp5` |
| `lattice-nexus` | `nexus`, `crosslink-nx` |

---

## Constraint file formats

Each FPGA family uses a different constraint file format. MIRR generates
the appropriate format based on the selected target.

### PCF -- Lattice iCE40

Physical Constraints Format. Used by nextpnr-ice40.

```
# Auto-generated PCF constraints for neonatal_respirator
# Fill in pin numbers for your board.

set_io respirator_enable PLACEHOLDER
set_io clamp_valve PLACEHOLDER
set_io airway_pressure[0] PLACEHOLDER
set_io airway_pressure[1] PLACEHOLDER
...
```

Each signal gets a `set_io` line mapping it to a physical pin. Multi-bit
signals are expanded bit-by-bit.

### LPF -- Lattice ECP5

Lattice Preference File. Used by nextpnr-ecp5.

```
# Auto-generated LPF constraints for neonatal_respirator (Lattice ECP5)
# Fill in LOC values for your board.

FREQUENCY NET "clk" 100.000000 MHz;

LOCATE COMP "respirator_enable" SITE "PLACEHOLDER";
IOBUF PORT "respirator_enable" IO_TYPE=LVCMOS33;
...
```

LPF files include I/O type specifications (defaulting to LVCMOS33) and
a frequency constraint for the clock net.

### PDC -- Lattice Nexus

Physical Design Constraints. Used by nextpnr-nexus.

```
# Auto-generated PDC constraints for neonatal_respirator (Lattice Nexus)
# Fill in pin assignments for your board.

create_clock -name {clk} -period 10.000 [get_ports clk]

ldc_set_location -site {PLACEHOLDER} [get_ports {respirator_enable}]
...
```

PDC files use Tcl-like syntax with `ldc_set_location` commands.

### XDC -- Xilinx 7-Series and UltraScale+

Xilinx Design Constraints. Used by Vivado.

```
## Auto-generated XDC constraints for neonatal_respirator (Xilinx 7-Series)
## Fill in PACKAGE_PIN values for your board.

create_clock -period 10.000 -name clk [get_ports clk]
# set_property PACKAGE_PIN {PLACEHOLDER} [get_ports respirator_enable]
# set_property IOSTANDARD LVCMOS33 [get_ports respirator_enable]
...
```

XDC constraints are generated as comments (prefixed with `#`) so they
do not cause errors before you fill in the pin assignments. Each port
gets a `PACKAGE_PIN` and `IOSTANDARD` property.

### SDC -- Intel Cyclone and Generic

Synopsys Design Constraints. Used by Quartus and as the generic format.

```
## Auto-generated SDC constraints for neonatal_respirator (Intel Cyclone)
## Fill in pin assignments for your board.

create_clock -period 10.000 -name clk [get_ports clk]
derive_pll_clocks
derive_clock_uncertainty

set_input_delay -clock clk 2.000 [get_ports respirator_enable]
set_input_delay -clock clk 2.000 [get_ports airway_pressure]
set_output_delay -clock clk 2.000 [get_ports clamp_valve]
```

SDC files include input/output delay constraints relative to the clock.

---

## Build script generation

MIRR generates complete build scripts for each target family. All scripts
are clearly marked as auto-generated scaffolding.

### Vivado Tcl (Xilinx)

For Xilinx 7-Series and UltraScale+ targets, MIRR generates a Vivado
Tcl build script:

```tcl
# Auto-generated Vivado build script for neonatal_respirator
create_project neonatal_respirator ./build -part xc7a35tcpg236-1 -force
add_files neonatal_respirator.sv
add_files -fileset constrs_1 neonatal_respirator.xdc
launch_runs synth_1 -jobs 4
wait_on_run synth_1
launch_runs impl_1 -to_step write_bitstream -jobs 4
wait_on_run impl_1
```

Run with: `vivado -mode batch -source build.tcl`

### Quartus Tcl (Intel)

For Intel Cyclone targets:

```tcl
# Auto-generated Quartus build script for neonatal_respirator
project_new neonatal_respirator -overwrite
set_global_assignment -name FAMILY "Cyclone V"
set_global_assignment -name DEVICE 5CSEBA6U23I7
set_global_assignment -name SYSTEMVERILOG_FILE neonatal_respirator.sv
set_global_assignment -name SDC_FILE neonatal_respirator.sdc
execute_flow -compile
project_close
```

Run with: `quartus_sh -t build.tcl`

### Shell scripts (Lattice and Generic)

For Lattice targets and the generic target, MIRR generates bash scripts
that invoke the open-source Yosys/nextpnr toolchain.

**iCE40 example:**

```bash
#!/usr/bin/env bash
# Auto-generated build script for neonatal_respirator (Lattice iCE40)
set -euo pipefail

# NOTE: Use --strip-sva when generating neonatal_respirator.sv for synthesis
yosys -p "read_verilog -sv neonatal_respirator.sv; synth_ice40 -top neonatal_respirator -json neonatal_respirator.json"
nextpnr-ice40 --hx8k --package ct256 --json neonatal_respirator.json --pcf neonatal_respirator.pcf --asc neonatal_respirator.asc
icepack neonatal_respirator.asc neonatal_respirator.bin
echo "Bitstream ready: neonatal_respirator.bin"
```

**ECP5 example:**

```bash
#!/usr/bin/env bash
set -euo pipefail
yosys -p "read_verilog -sv module.sv; synth_ecp5 -top module -json module.json"
nextpnr-ecp5 --85k --package CABGA381 --json module.json --lpf module.lpf --textcfg module.config
ecppack module.config module.bit
echo "Bitstream ready: module.bit"
```

**Nexus example:**

```bash
#!/usr/bin/env bash
set -euo pipefail
yosys -p "read_verilog -sv module.sv; synth_nexus -top module -json module.json"
nextpnr-nexus --device LIFCL-40-9BG400C --json module.json --pdc module.pdc --fasm module.fasm
prjoxide pack module.fasm module.bit
echo "Bitstream ready: module.bit"
```

---

## Yosys and nextpnr integration

The Lattice targets and the generic target use the open-source
oss-cad-suite toolchain:

| Tool | Purpose | Used by |
|------|---------|---------|
| **Yosys** | RTL synthesis (Verilog to netlist) | All Lattice targets, Generic |
| **nextpnr-ice40** | Place and route for iCE40 | `lattice-ice40` |
| **nextpnr-ecp5** | Place and route for ECP5 | `lattice-ecp5` |
| **nextpnr-nexus** | Place and route for Nexus | `lattice-nexus` |
| **icepack** | Bitstream packing for iCE40 | `lattice-ice40` |
| **ecppack** | Bitstream packing for ECP5 | `lattice-ecp5` |
| **prjoxide** | Bitstream packing for Nexus | `lattice-nexus` |
| **icetime** | Static timing analysis (iCE40) | `lattice-ice40` (optional) |

### SVA stripping

SystemVerilog Assertions (SVA) generated by the MIRR compiler's
property blocks are not synthesizable. When targeting synthesis, use
the `--strip-sva` flag to generate clean RTL without assertion blocks.
The generated build scripts include a reminder comment about this.

### Yosys synthesis commands

Each target uses a target-specific Yosys synthesis command:

| Target | Yosys command |
|--------|--------------|
| `lattice-ice40` | `synth_ice40` |
| `lattice-ecp5` | `synth_ecp5` |
| `lattice-nexus` | `synth_nexus` |
| `xilinx-7` / `xilinx-us` | `synth_xilinx` |
| `intel-cyclone` | `synth_intel` |
| `generic` | `synth` |

---

## Example usage

### Step 1: Compile to SystemVerilog

```bash
cargo run --bin mirr-compile -- --emit verilog my_monitor.mirr > my_monitor.sv
```

### Step 2: Generate FPGA scaffold

The scaffold generator is available through the Rust API:

```rust
use mirr::emit::fpga_scaffold::{emit_constraints, emit_build_script};
use mirr::emit::fpga_target::FpgaTarget;

let target = FpgaTarget::from_str_name("lattice-ice40").unwrap();

// Generate constraint file
let constraints = emit_constraints(&pipeline_result, &target);
std::fs::write("my_monitor.pcf", &constraints)?;

// Generate build script
let build_script = emit_build_script(&pipeline_result, &target);
std::fs::write("build.sh", &build_script)?;
```

### Step 3: Fill in pin assignments

Open the generated constraint file and replace every `PLACEHOLDER`
with the actual pin name from your board's datasheet or schematic.

For example, on a Lattice iCE40-HX8K breakout board:

```
# Before:
set_io clk PLACEHOLDER

# After:
set_io clk J3
```

### Step 4: Run the build

```bash
chmod +x build.sh
./build.sh
```

This produces a bitstream file (`.bin`, `.bit`, or similar) that can
be loaded onto your FPGA.

---

## Target-specific details

### Clock primitives

Each FPGA family has a different PLL/clock primitive:

| Target | Clock primitive |
|--------|----------------|
| Generic | PLL |
| Xilinx 7-Series | MMCME2_BASE |
| Xilinx UltraScale+ | MMCME4_ADV |
| Intel Cyclone | altpll |
| Lattice iCE40 | SB_PLL40_CORE |
| Lattice ECP5 | EHXPLLL |
| Lattice Nexus | OSCA |

### Default clock period

All generated constraint files assume a 100 MHz clock (10 ns period).
Adjust the `create_clock` or `FREQUENCY` directive if your board uses
a different oscillator.

### I/O standards

- Xilinx targets default to LVCMOS33.
- Lattice ECP5 targets default to LVCMOS33 via `IOBUF` directives.
- Other targets leave I/O standards to be configured by the engineer.

---

## DSP mapping

MIRR emits vendor-specific synthesis attributes to map multiply
operations to hardware DSP blocks when possible:

| Target | DSP primitive | Attribute | Max input width |
|--------|--------------|-----------|-----------------|
| Xilinx 7-Series | DSP48E1 | `(* use_dsp48 = "yes" *)` | 25 bits |
| Xilinx UltraScale+ | DSP48E2 | `(* use_dsp48 = "yes" *)` | 27 bits |
| Intel Cyclone | cyclonev_mac | `(* multstyle = "dsp" *)` | 27 bits |
| Lattice iCE40 | SB_MAC16 | `(* use_dsp = "yes" *)` | 16 bits |
| Lattice ECP5 | ALU54B | `(* use_dsp = "yes" *)` | 18 bits |
| Lattice Nexus | MULT18X18 | `(* use_dsp = "yes" *)` | 18 bits |
| Generic | -- | `(* use_dsp = "yes" *)` | 18 bits |

If a multiply operand exceeds the DSP block's maximum input width, the
synthesis tool will infer logic-based multiplication instead.

---

## Resource bounds

The scaffold generator enforces the following NASA Power-of-10 bounds:

| Constant | Value | Purpose |
|----------|-------|---------|
| `MAX_CONSTRAINT_LINES` | 256 | Maximum lines in a generated constraint file |
| `MAX_SYNC_STAGES` | 4 | Maximum clock-domain synchronizer stages |

Multi-bit signals are expanded bit-by-bit in constraint files. If the
total line count reaches `MAX_CONSTRAINT_LINES`, generation stops to
prevent unbounded output.

---

## See Also

- [Tutorial](tutorial) -- Getting started with MIRR
- [R-SPU Reference](rspu_isa_spec) -- Instruction set architecture
- [Contributing](contributing) -- Coding standards and campaign workflow
