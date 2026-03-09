---
name: add-emit-target
description: 'Add or modify an emission backend for the MIRR compiler (verilog, firrtl, json, dot, testbench, scaffold, or new formats). Use this when working on the emit subsystem.'
argument-hint: 'Emit format to work on (e.g., "firrtl", "verilog", "testbench", "new-format-name")'
---

# MIRR Emit Target Skill

Guide for adding or modifying an emission backend in the MIRR compiler.

## Architecture

All emitters live in `src/emit/` and follow the same contract:

```
src/emit/
├── mod.rs            # barrel module — register new emitters here
├── verilog.rs        # SystemVerilog RTL + SVA assertions
├── firrtl.rs         # FIRRTL intermediate representation
├── json_netlist.rs   # Machine-readable JSON
├── dot.rs            # Graphviz DOT graph
├── rspu.rs           # R-SPU assembly emission
├── rspu_isa.rs       # R-SPU instruction set types
├── rspu_regalloc.rs  # R-SPU register allocator
├── testbench.rs      # Auto-generated SystemVerilog testbench
├── fpga_scaffold.rs  # FPGA constraint files + build scripts
└── fpga_target.rs    # FPGA target enum (Xilinx, Intel, Lattice, Generic)
```

### Emitter Contract

Every emitter must:

1. **Accept `&PipelineResult`** as the primary input (some also take `&FpgaTarget`)
2. **Return `String`** (or `Result<String, _>` for serialization)
3. **Be a pure function** — no side effects
4. **Use `#![forbid(unsafe_code)]`**
5. **Bound all traversals** with `MAX_*` constants (NASA Power-of-10)

### Entry Point Pattern

```rust
pub fn emit_<format>(result: &PipelineResult) -> String {
    let module = &result.program.module;
    let mut out = String::with_capacity(4096);
    // ... emit sections ...
    out
}
```

For target-aware emitters:

```rust
pub fn emit_<format>(result: &PipelineResult, target: &FpgaTarget) -> String {
    // target determines vendor-specific output
}
```

## Steps for a New Backend

1. **Create `src/emit/<format>.rs`** following the contract above
2. **Register in `src/emit/mod.rs`**: add `pub mod <format>;` in alphabetical order
3. **Wire into CLI** in `src/bin/mirr-compile.rs`:
   - Add `"<format>" =>` arm in the emit match
   - Update the error message listing valid formats
   - Update `print_help()` to document the new format
   - If target-aware, use the `fpga_target` variable from `--target` flag
4. **Add tests** in `tests/emit_<format>_tests.rs` covering:
   - Basic structure (header, module declaration)
   - Port declarations (input, output, internal)
   - Temporal guard hardware (if applicable)
   - Reflex assignments
   - Property annotations
5. **Update docs**: `.github/copilot-instructions.md` commands section

## Key Types to Map

| MIRR Type | Description |
|-----------|-------------|
| `SignalType::Bool` | Single-bit boolean |
| `SignalType::Unsigned(w)` | Fixed-width unsigned integer |
| `SignalType::Signed(w)` | Fixed-width signed integer |
| `BinaryOp::*` | 13 binary operators (And, Or, Lt, Add, etc.) |
| `UnaryOp::Not` | Logical negation |
| `PropertyFormula::Always/Never/AlwaysImplies/NeverImplies/EventuallyWithin/AlwaysFollowedBy` | Six property forms |
| `CompiledGuard::ShiftRegister/Counter/Complex` | Temporal hardware primitives |

## FPGA Target System

When adding target-aware features, use the `FpgaTarget` enum from `src/emit/fpga_target.rs`:

| Target | CLI name | Part | Constraint format |
|--------|----------|------|-------------------|
| `Generic` | `generic` | — | `.sdc` |
| `Xilinx7` | `xilinx-7` | xc7a35t | `.xdc` |
| `XilinxUS` | `xilinx-us` | xcku040 | `.xdc` |
| `IntelCyclone` | `intel-cyclone` | 5CSEMA5F31C6 | `.sdc` |
| `LatticeIce40` | `lattice-ice40` | iCE40HX8K | `.pcf` |

## CLI Flags

The compiler CLI (`src/bin/mirr-compile.rs`) supports:

```
--emit FORMAT       dot, verilog, json, sva, firrtl, rspu, testbench, scaffold, build-script
--target FAMILY     generic, xilinx-7, xilinx-us, intel-cyclone, lattice-ice40
--sync-stages N     Input synchronizer chain depth (default: 2)
--testbench         Also emit testbench alongside verilog output
--scaffold          Also emit constraint + build script alongside verilog output
--output FILE, -o   Write to file instead of stdout
--stats             Show pipeline statistics
```

## Verification

```bash
# Build
cargo build

# Run new tests
cargo test --test emit_<format>_tests

# All tests pass
cargo test --all

# Zero clippy warnings
cargo clippy --all-targets -- -D warnings

# Compile an example
cargo run --bin mirr-compile -- --emit <format> examples/tmr_sensor_fusion.mirr

# Compile with FPGA target (if target-aware)
cargo run --bin mirr-compile -- --emit verilog --target xilinx-7 --testbench --scaffold examples/tmr_sensor_fusion.mirr
```
