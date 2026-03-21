---
title: ARM Target Guide
nav_order: 13
---

# ARM Target Guide

The MIRR compiler includes an ARM Thumb-2 backend that translates R-SPU programs
into ARM assembly suitable for Cortex-M microcontrollers.

## Overview

The ARM backend maps R-SPU instructions to Thumb-2 assembly with:
- Memory-mapped I/O at `0x4000_0000`
- IT blocks for conditional execution
- Shift register emulation via data memory
- Counter emulation via subtract-and-compare

## Register Mapping

| R-SPU Register | ARM Register | Purpose |
|----------------|-------------|---------|
| R0–R63 | R0–R63 | Input ports |
| R64–R127 | R64–R127 | Output ports |
| R128–R191 | R128–R191 | Internal signals |
| R192–R255 | R192–R255 | Expression temporaries |

## Memory-Mapped I/O

| Address | Purpose |
|---------|---------|
| `0x4000_0000` | Input port 0 |
| `0x4000_0004` | Input port 1 |
| `0x4000_0008` | Output port 0 |
| ... | ... |

## Compilation

```bash
# Compile MIRR to ARM assembly
cargo run --bin mirr-compile -- --emit rspu examples/flight_controller.mirr
```

## Instruction Mapping

| R-SPU Instruction | ARM Instruction |
|-------------------|-----------------|
| LOAD_INPUT | `ldr` from MMIO |
| STORE_OUTPUT | `str` to MMIO |
| MOV | `mov rd, rs` |
| LOAD_IMM | `mov rd, #imm` or `ldr rd, =imm` |
| ALU ADD | `add rd, rs1, rs2` |
| ALU SUB | `sub rd, rs1, rs2` |
| ALU AND | `and rd, rs1, rs2` |
| ALU OR | `orr rd, rs1, rs2` |
| SR_INIT | `ldr` + `str` loop |
| SR_TICK | Shift loop |
| CTR_INIT | `ldr` + `str` |
| CTR_TICK | `ldr` + `sub` + `str` |
| REFLEX_IF | IT block conditional |
| HALT | `bkpt #0` |

## IT Blocks

The ARM backend uses IT (If-Then) blocks for conditional execution:
```arm
    cmp r0, #0
    it ne
    movne r1, r2
```

## See Also

- [RISC-V Target Guide](riscv-target-guide) — RISC-V backend
- [R-SPU ISA Spec](rspu_isa_spec) — Instruction set reference