# MIRR-OSS Arsenal: Hardware Design & Simulation Skill

*Specialized workflows for writing advanced MIRR (v2), simulating R-SPU prototypes, and integrating with Open Source hardware ecosystems.*

## Core Competencies

### 1. MIRR-V2 Authoring
Expert guidance for advanced language features:
- **Arrays & Composites**: Correct usage of fixed-width arrays and nested structs.
- **Syntactic Sugar**: Expanding shorthands (e.g., `when p then q`, `default { ... }`) into canonical AST nodes.
- **Multi-file Imports**: Managing hierarchical modules and resolving symbol conflicts (E1309).

### 2. R-SPU Simulation (The "Image" Prototype)
Workflow for validating designs on the virtual hardware:
- **`RspuSimulator`**: Running compiled `.rspu` binary images against test inputs.
- **Trace Analysis**: Inspecting signal transitions and guard activations per cycle.
- **Parity Check**: Verifying simulation results against the Rust-side behavioral model.

### 3. OSS Hardware Integration
Bridging the "Last Mile" to real silicon/FPGAs:
- **SystemVerilog Emission**: Generating portable, synthesis-ready RTL.
- **FIRRTL Backend**: Integrating with Chisel/RocketChip ecosystems.
- **Verification**: Running `iverilog` or `verilator` on generated outputs to prove design correctness.

## Remediation Workflow: "Dark Age" Recovery
Use this cycle to repair the 0-byte proposals (058-073) and upgrade MEGA sequences to v2:
1. **Audit**: Trace the existing "Dark Age" implementation (e.g., `src/import/`, `src/hls/`).
2. **Re-spec**: Write a new v2 proposal based on the actual codebase logic and the user's "arsenal" requirements.
3. **Execute**: Implement missing features (e.g., real byte-level lexing, full `Prev` operator support).
4. **CI Gate**: Run `cargo check` and `tests/eda/run_eda_tests.sh` to confirm zero-debt status.

## Safety Mandates (NASA Power-of-10)
- **Bounded Loops**: All loops in simulation and emission must be bounded by `MAX_*` constants.
- **No Recursion**: Use iterative stacks for all AST traversals and graph analyses.
- **Zero-Unsafe**: `#[forbid(unsafe_code)]` at all times.
