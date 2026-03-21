---
title: RISC-V Target Guide
nav_order: 12
---

# RISC-V Target Guide

The MIRR compiler includes a RISC-V RV32I backend that translates R-SPU programs
into RISC-V assembly suitable for soft-core FPGA implementations.

## Overview

The RISC-V backend maps R-SPU instructions to RV32I assembly with:
- Memory-mapped I/O at `0x1000_0000`
- Shift register emulation via data memory
- Counter emulation via decrement-and-compare
- Guard combination via AND/OR logic

## Register Mapping

| R-SPU Register | RISC-V Register | Purpose |
|----------------|-----------------|---------|
| R0–R63 | x0–x63 | Input ports |
| R64–R127 | x64–x127 | Output ports |
| R128–R191 | x128–x191 | Internal signals |
| R192–R255 | x192–x255 | Expression temporaries |

## Memory-Mapped I/O

| Address | Purpose |
|---------|---------|
| `0x1000_0000` | Input port 0 |
| `0x1000_0004` | Input port 1 |
| `0x1000_0008` | Output port 0 |
| ... | ... |

## Compilation

```bash
# Compile MIRR to RISC-V assembly
cargo run --bin mirr-compile -- --emit rspu examples/neonatal_respirator.mirr

# The RISC-V backend is invoked automatically when targeting R-SPU
```

## Instruction Mapping

| R-SPU Instruction | RISC-V Instruction |
|-------------------|-------------------|
| LOAD_INPUT | `lw` from MMIO |
| STORE_OUTPUT | `sw` to MMIO |
| MOV | `addi rd, rs, 0` |
| LOAD_IMM | `li rd, imm` |
| ALU ADD | `add rd, rs1, rs2` |
| ALU SUB | `sub rd, rs1, rs2` |
| ALU AND | `and rd, rs1, rs2` |
| ALU OR | `or rd, rs1, rs2` |
| SR_INIT | `la` + `sw` loop |
| SR_TICK | Shift loop |
| CTR_INIT | `la` + `li` + `sw` |
| CTR_TICK | `lw` + `addi` + `sw` |
| REFLEX_IF | `beq` conditional branch |
| HALT | `ecall` with a7=0 |

## Example

```bash
# Compile neonatal respirator to RISC-V
cargo run --bin mirr-compile -- --emit rspu examples/neonatal_respirator.mirr

# Output includes .text and .data sections
# .text contains the main program
# .data contains signal storage and shift register buffers
```

## See Also

- [ARM Target Guide](arm-target-guide) — ARM Thumb-2 backend
- [R-SPU ISA Spec](rspu_isa_spec) — Instruction set reference