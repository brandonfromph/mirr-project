# MIRR Project Architecture & Directory Topology

Date: 2026-07-08

This document provides a comprehensive map of the repository, detailing the structural layout, data flow pathways, architectural components, and strategic roadmap of the MIRR compiler platform.

> [!IMPORTANT]
> **MAINTENANCE MANDATE**: This file is the single source of truth for the repository's topology. Under the project's zero-debt policy, whenever a new module, crate, or major architectural flow is added, moved, or removed, **you must immediately update this document** to reflect the new state. Outdated architecture mapping is strictly forbidden.

---

## 1. Directory Map

A comprehensive tree breakdown of the repository's structural layout:

```text
mirr-private/
├── src/                          # Core Compiler Pipeline (~69k LOC)
│   ├── ast/                      # Abstract Syntax Tree definitions (types, expr, program, property, pattern)
│   ├── lexer/                    # Tokenizer and lexical analysis
│   ├── parser/                   # Front-end parsing (module parser, expr parser, pattern parser)
│   ├── ecs/                      # Entity Component System architecture (Registry, components, systems, typeck, semantic validation)
│   ├── cert/                     # MEGA-4 Proof certificate format & MEGA-16 PCC verifier
│   ├── typeck/                   # Type checking & constraint validation (core + MEGA-1 extended type checker)
│   ├── width/                    # FIRWINE width constraint solving (SCC paths, graph, Tarjan, verify)
│   ├── temporal/                 # Temporal lowering of hardware guards (compiler, retiming, clock domain, allocator)
│   ├── symbolic/                 # Symbolic evaluation engine (interval, fingerprint, rewrite, diff, integration, pattern, statistics)
│   ├── sat/                      # Bounded DPLL SAT solver (CNF encoding, simplification, solver)
│   ├── hls/                      # High-Level Synthesis optimizer (ASAP/ALAP scheduling, FIFO streaming)
│   ├── totality/                 # Totality engine (resource bounds, guard coverage, acyclicity checks)
│   ├── sexpr/                    # S-Expression "Code as Data" engine (parser, eval, reader, printer, macro expansion, convert/)
│   ├── expand/                   # Pattern expansion engine (hygienic macros, name prefixing, scoping)
│   ├── validation/               # Semantic validation passes
│   ├── symbols/                  # Cross-module symbol resolution and namespace management
│   ├── mape_k/                   # MAPE-K autonomic control loop (monitor, analyzer, planner, executor, LTL, knowledge, bridge/)
│   ├── lsp/                      # Language Server Protocol implementation (server, transport, diagnostics)
│   ├── diagnostic/               # Rich diagnostic engine (formal trace, VCD parser, error formatting)
│   ├── toolchain/                # EDA toolchain orchestration (Yosys, Verilator, SBY, EQY, IceTime, OpenLane, ABC)
│   ├── emit/                     # Emission backends (11 targets)
│   │   ├── verilog/              # SystemVerilog RTL + SVA assertions
│   │   ├── mape_k_rtl/          # Synthesizable MAPE-K hardware
│   │   ├── rspu_sim/            # Cycle-accurate R-SPU silicon simulator
│   │   ├── rspu_encoding/       # R-SPU instruction encoding
│   │   ├── firrtl.rs            # FIRRTL intermediate representation
│   │   ├── rspu.rs              # R-SPU native code emission
│   │   ├── rspu_isa.rs          # R-SPU ISA definitions (30 instructions)
│   │   ├── rspu_tagged.rs       # Tagged-word architecture emission
│   │   ├── rspu_regalloc.rs     # Register allocation
│   │   ├── rspu_opt.rs          # R-SPU peephole optimization
│   │   ├── rspu_exceptions.rs   # Exception model emission
│   │   ├── rspu_helpers.rs      # R-SPU emission utilities
│   │   ├── arm.rs               # ARM backend
│   │   ├── riscv.rs             # RISC-V backend
│   │   ├── dot.rs               # Graphviz DOT visualization
│   │   ├── json_netlist.rs      # JSON netlist export
│   │   ├── sexpr.rs             # S-Expression output
│   │   ├── cert.rs              # PCC certificate emission
│   │   ├── fpga_scaffold.rs     # FPGA scaffold generation
│   │   ├── fpga_target.rs       # FPGA target definitions (6 families)
│   │   ├── dsp.rs               # DSP block emission
│   │   ├── testbench.rs         # Testbench generation
│   │   └── provenance.rs        # Source provenance tracking
│   ├── bootstrap_runner/         # Self-hosting bootstrap pipeline
│   ├── mirr_executor/            # Signal evaluator for MAPE-K harness
│   ├── util/                     # Shared utilities
│   ├── bin/                      # Executable entrypoints
│   │   ├── mirr.rs              # Unified CLI router (compile, lsp, kb, brain, audit, proof-audit, general)
│   │   ├── mirr-compile/        # Compilation pipeline CLI
│   │   ├── mirr-diff/           # Structural diff tool
│   │   ├── mirr-explain/        # Error explanation tool
│   │   ├── mirr-general.rs      # General-purpose orchestration CLI
│   │   ├── mirr-audit.rs        # Code quality audit tool
│   │   ├── mirr-brain.rs        # Knowledge base CLI
│   │   ├── mirr-hydrate.rs      # Yosys JSON → MIRR roundtrip hydrator
│   │   ├── mirr-lsp.rs          # LSP server entrypoint
│   │   ├── mirr-proof-audit.rs  # Rocq proof coverage tracker
│   │   ├── mirr-simplify.rs     # Standalone logic simplifier CLI
│   │   ├── mirr-simulate.rs     # MAPE-K simulation harness CLI
│   │   ├── mirr-wave.rs         # VCD waveform viewer / source-level debugger
│   │   └── mirr-width.rs        # Bit-width inference CLI
│   ├── pipeline.rs               # Unified compilation pipeline orchestrator
│   ├── simplify.rs               # Logic simplifier — 33 algebraic rules
│   ├── lsp_bridge.rs             # LSP ↔ compiler bridge
│   ├── lsp_incremental.rs        # Incremental recompilation for LSP
│   ├── mirr_daemon.rs            # Background compilation daemon
│   ├── mirr_daemon_security.rs   # Daemon security model
│   ├── mirr_driver.rs            # Compilation driver
│   ├── mirr_runtime.rs           # Runtime execution model
│   ├── mrt_host.rs               # MRT host interface
│   ├── mrt_auth.rs               # MRT authentication
│   ├── mrt_schema.rs             # MRT schema definitions
│   ├── workspace.rs              # Multi-file workspace management
│   ├── error.rs                  # Shared error types (162 error codes, E100–E902)
│   ├── error_codes.rs            # Error code registry
│   ├── span.rs                   # Source span tracking
│   ├── suggest.rs                # Did-you-mean suggestions
│   ├── diagnostic_builder.rs     # Diagnostic message builder
│   └── lib.rs                    # Public module exports
├── crates/                       # Consumer & Control Plane Surfaces
│   ├── mirr-wasm/                # WASM compilation API (browser/JS consumers)
│   ├── mirr-arsenal-wasm/        # Validation and compile-contract wrapper
│   ├── lra-cli/                  # Living Research Artifact CLI tooling
│   ├── mirr-kb-native/           # Knowledge Base native engine (RAG / Vector search)
│   └── mirr-mcp-control-plane/   # Model Context Protocol bridge & governance
├── tests/                        # Integration & Parity Test Matrix (5,200+ tests)
├── fuzz/                         # Fuzzing harnesses for robustness
├── proofs/                       # Formal verification (Rocq/Coq)
│   ├── width/                    # FIRWINE width inference proofs (incl. SCC/)
│   ├── cert/                     # PCC verifier proofs
│   ├── compiler/                 # Compiler pass correctness proofs
│   ├── language/                 # Language semantics proofs
│   ├── mape_k/                   # MAPE-K behavioral equivalence proofs
│   └── rspu/                     # R-SPU ISA proofs
├── docs/                         # Documentation and Architecture definitions
├── proposals/                    # Architectural proposals and RFCs
├── reflex_soc/                   # Primary hardware project (64-core R-SPU, NoC, crossbar, pipeline formal verification)
├── rspu_chip/                    # R-SPU chip-level design (top-level MIRR)
├── science_hls/                  # Scientific computing HLS experiments
├── stdlib/                       # Standard library definitions (mirr_core/)
├── compiler_mirr/                # Self-hosting bootstrap implementation
├── examples/                     # Example MIRR programs
├── scripts/                      # Build and generation scripts
├── vscode-mirr/                  # VS Code language extension
├── _wasm/                        # WASM build artifacts
├── demos/                        # Interactive demonstrations
├── benches/                      # Performance benchmarks
└── paper/                        # Academic paper source
```

---

## 2. One-Screen Mental Model

MIRR is a safety-critical compiler platform (**155k+ LOC** across 293+ Rust source files) with **5,200+** passing tests, zero `unsafe` code, and zero clippy warnings. It is organized into five distinct architectural layers:

1. **Core Compiler Sub-Engines**: 17 specialized modules (Temporal, Symbolic, SAT, HLS, Totality, MAPE-K, etc.) that perform rigorous behavioral-to-physical translation. Each engine adheres to NASA's Power-of-10 rules for ultra-reliable execution.
2. **Control Planes**: MRT / Presidential Arsenal and KB-native (RAG) for autonomic governance and knowledge-backed synthesis. The MCP Semantic Bridge routes cross-subsystem tool calls.
3. **Unified Interface**: The `mirr` CLI router integrates all toolchain functions (`compile`, `lsp`, `kb`, `brain`, `audit`, `proof-audit`, `general`, `wave`) into a single cohesive UX surface via `clap` categorization.
4. **Consumer Bridges**: WASM, LRA-CLI, and MCP-Control-Plane providing multi-surface accessibility for browser-based IDEs, CI/CD pipelines, and AI-assisted design.
5. **Language Server**: A production LSP implementation (`src/lsp/`) providing real-time diagnostics, incremental recompilation (`lsp_incremental.rs`), and IDE integration via `mirr-lsp`.

### Project Specialization: The R-SPU Architecture

MIRR is a specialized tool optimized for designing and verifying the **R-SPU (Reflexive Signal Processing Unit)**. The R-SPU is a multi-core, MIMD architecture designed for zero-jitter, hard real-time spatial processing. It is intended for the high-assurance "brains" of robotic systems, autonomous medical devices, and aerospace hardware. We do not build bloated, general-purpose CPUs — we map logic directly to dedicated physical hardware via asynchronous reflex triggers.

### Current Completion Status (Phase 8a+)

The compiler has reached operational maturity through Phases 0–7h (all complete), with Phase 7i (Verified Compilation) and Phase 8a (R-SPU Core Architecture) in active progress. The 64-core R-SPU SoC has been synthesized through Yosys, formally verified via SymbiYosys BMC and K-Induction, and optimized via ABC logic minimization. The `reflex_soc/` directory contains fully synthesized SystemVerilog exceeding 3.2M lines for the 64-core design.

### Known Technical Flaws

The following flaws have been identified in the current compiler implementation:

1. **Physical P&R Agnosticism**: The compiler has no awareness of physical geometry or floorplanning, which can lead to timing closure failures on large-scale chips (like the 64-core R-SPU). NextPNR integration exists (`src/toolchain/`) but is not yet feeding placement data back into the ECS.
2. **ECS-AST Duality**: Several subsystems still maintain parallel AST and ECS code paths. The ECS is the canonical source of truth, but legacy AST paths remain in some emission backends.
3. **S-Expression Engine Disconnection**: The `src/sexpr/` homoiconic IR is not wired into the main compilation pipeline. It is used exclusively for self-hosting bootstrap and formal verification bridges.
4. **Incremental Compilation Immaturity**: The LSP incremental recompilation (`lsp_incremental.rs`) performs full re-analysis on each edit. True incremental compilation with dependency tracking is not yet implemented.

**Inputs:**
- MIRR specifications (Signals, Guards, Reflexes, Properties, Patterns, Temporal Guards, For-Generate Loops, Module Imports)

**Outputs:**
- SystemVerilog RTL (always_ff synchronous), FIRRTL, R-SPU Assembly, ARM Assembly, RISC-V Assembly, JSON Netlists, Graphviz DOT, Formal S-Expressions, PCC Certificates, SVA Assertions, Testbenches, DSP Block Configurations, FPGA Scaffolds (Xilinx 7-Series, Xilinx UltraScale+, Intel Cyclone, Lattice iCE40, Lattice ECP5, Lattice Nexus)

---

## 3. The 17 Architectural Sub-Engines

The core of MIRR is partitioned into 17 high-assurance engines, each adhering to NASA's Power-of-10 rules for ultra-reliable execution (bounded loops, no recursion, no dynamic allocation after initialization).

### Logic & Optimization

1. **Temporal Guard Compiler** (`src/temporal/compiler.rs`): Translates high-level temporal guards (Cement2-inspired `delay(k)`) into deterministic shift-register and counter primitives. Reduces gate count from O(N × 64) to O(N × k). Includes clock domain support (`clock_domain.rs`) and a resource allocator (`allocator.rs`). **Status: Production.**

2. **SAT Logic Solver** (`src/sat/`): A bounded iterative DPLL solver (MAX_SAT_VARIABLES = 256, MAX_SAT_CLAUSES = 1024) for proving expression equivalences and verifying simplification candidates. Includes CNF encoding (`cnf.rs`), core solver (`solver.rs`), and guard simplification (`simplify_sat.rs`). Error codes E900–E902. **Status: Production.**

3. **High-Level Synthesis (HLS) Optimizer** (`src/hls/mod.rs`): Performs ASAP/ALAP scheduling, resource sharing, and FIFO streaming synthesis (`fifo.rs`) for hardware realization. Maps behavioral descriptions to scheduled, resource-shared datapaths. **Status: Production.**

4. **Register Retiming Optimizer** (`src/temporal/retiming.rs`): Employs Leiserson-Saxe retiming to minimize critical path delay by moving registers across combinational logic. Integrated with the temporal compiler for post-lowering optimization. **Status: Production.**

5. **Logic Simplifier** (`src/simplify.rs`): SmaRTLy-inspired algebraic simplification with 33 rules covering boolean identity/annihilation, arithmetic folding, comparison folding, and fixpoint iteration. Bounded by MAX_PASSES. Wired as a pre-lowering pass in the temporal pipeline. **Status: Production (58+ tests).**

### Verification & Assurance

6. **Symbolic Evaluation Engine** (`src/symbolic/`): Implements interval-based abstract interpretation (`interval.rs`), discrete calculus approximations (`diff.rs`, `integration.rs`), anomaly signature fingerprinting (`fingerprint.rs`), pattern matching (`pattern.rs`), moving-window statistics (`statistics.rs`), and a term rewriting engine (`rewrite.rs`) for runtime logic optimization. 8 files, fully tested. **Status: Production.**

7. **Totality Engine** (`src/totality/`): Verifies the five Pillars of Totality: resource bounds, output completeness, guard coverage, temporal finiteness, and dependency acyclicity. Core checks in `checks.rs`, type definitions in `types.rs`. **Status: Production.**

8. **S-Expression Transpiler** (`src/sexpr/convert/`): Generates homoiconic IR for formal verification bridges (Z3, Rocq). Bidirectional AST ↔ S-expression conversion with round-trip invariant. Error codes E800–E815. **Status: Production (not wired to main pipeline).**

9. **R-SPU Silicon Simulator** (`src/emit/rspu_sim/`): Provides cycle-accurate, bit-precise simulation for R-SPU programs. Supports the full 30-instruction ISA with tagged-word architecture, dual-mode execution, and exception handling. Bounded by MAX_SIM_CYCLES. **Status: Production.**

10. **MAPE-K Analyzer** (`src/mape_k/analyzer.rs`): Evaluates bounded LTL properties (`ltl.rs`) over rolling windows for autonomic safety. Includes hardware-accelerated LTL checking, knowledge base integration (`knowledge.rs`), sensor modeling (`sensor.rs`), and partition-aware analysis (`partition.rs`). 10 files + bridge subdirectory. **Status: Production.**

11. **MAPE-K Telemetry Bridge** (`src/mape_k/bridge/`): Orchestrates the 64-core telemetry fabric for cross-core safety coordination. Manages monitor → analyze → plan → execute data flow across the NoC. **Status: Production.**

### Infrastructure & Orchestration

12. **Unified CLI Router** (`src/bin/mirr.rs`): The single entrypoint for the hardware toolchain. Dispatches execution to compilation pipelines, LSP servers, specialized verification engines, and knowledge base tooling. Functional categories:
    - **Core Systems**: `mirr compile` and `mirr lsp`
    - **Verification & Assurance**: `mirr proof-audit` and `mirr audit`
    - **Debugging**: `mirr wave` (VCD waveform viewer) and `mirr explain`
    - **Stress Testing**: `mirr generate-stress`
    - **Orchestration**: `mirr general`
    - **Knowledge Base**: `mirr brain`, `mirr kb`
    - **Status: Production.**

13. **ECS Registry** (`src/ecs/registry.rs`): A high-performance SoA (Structure of Arrays) registry managing up to 1M hardware entities. 53k+ lines in `registry.rs` alone. Includes component definitions (`components.rs`), ECS systems (`systems.rs`, `systems/`), type checking (`typeck.rs`), semantic validation (`semantic_validate.rs`), registry validation (`registry_validate.rs`), string interning (`intern.rs`), and AST adapter (`adapter.rs`). The ECS is the sole source of truth and is strictly as capable as the legacy AST. **Status: Production.**

14. **Cross-Module Symbol Resolver** (`src/symbols/`): Manages cross-crate namespace resolution and visibility for multi-module MIRR designs. **Status: Production.**

15. **EDA Toolchain Orchestrator** (`src/toolchain/`): Integrates the full open-source EDA toolchain into the MIRR compilation pipeline. 9 files orchestrating:
    - **Yosys** (`optimize.rs`): ABC logic minimization and technology mapping
    - **Verilator** (`verilator.rs`): Compiled C++ simulation and lint checking
    - **SymbiYosys** (`sby.rs`, `formal.rs`): Bounded Model Checking (BMC), K-Induction proving, multi-engine solver selection (Z3, Yices, Btor, Bitwuzla)
    - **EQY** (`eqy.rs`): Equivalence checking between MIRR revisions
    - **IceTime** (`icetime.rs`): Static timing analysis for iCE40 targets
    - **OpenLane** (`openlane.rs`): ASIC flow integration (GDSII generation)
    - **SDC** (`sdc.rs`): Synopsys Design Constraints for timing
    - **Status: Production.**

16. **LSP Server** (`src/lsp/`): A Language Server Protocol implementation providing real-time diagnostics (`diagnostics.rs`), JSON-RPC transport (`transport.rs`), and incremental recompilation support. Bridges to the compiler via `lsp_bridge.rs` and `lsp_incremental.rs`. **Status: Production (incremental mode maturing).**

17. **Diagnostic Engine** (`src/diagnostic/`): Rich error formatting and source-level debugging. Includes formal trace analysis (`formal_trace.rs`), VCD waveform parsing (`vcd_parser.rs`), and integrated test coverage (`tests.rs`). The `mirr-wave` CLI provides source-level waveform viewing. Error codes span E100–E902 across 162 unique codes. **Status: Production.**

---

## 4. Data Flow Pathways

The following pathways describe the complete lifecycle of a MIRR specification through the compiler and its supporting tools:

### Pathway 1: Core Compiler Pipeline

Source code enters the compiler and flows through a deterministic sequence of transformation passes:

```
Source (.mirr) → Lexer/Tokenizer → Parser → Raw AST
    → Pattern Expansion (expand/) → Validated AST
    → ECS Registry Population (ecs/adapter.rs)
    → Semantic Validation (ecs/semantic_validate.rs)
    → Type Checking (typeck/ + ecs/typeck.rs)
    → Width Inference (width/solver.rs, SCC paths)
    → Logic Simplification (simplify.rs, 33 algebraic rules)
    → Temporal Lowering (temporal/compiler.rs → shift registers, counters)
    → HLS Scheduling (hls/, optional)
    → Symbolic Evaluation (symbolic/, optional)
    → Totality Verification (totality/)
    → Emission Backend (emit/ → Verilog, FIRRTL, R-SPU, ARM, RISC-V, DOT, JSON, S-Expr, Cert, Testbench)
```

Each pass is bounded and iterative (no recursion). The ECS Registry (`src/ecs/registry.rs`) is the canonical intermediate representation from which all emission backends read.

### Pathway 2: Formal Verification Pipeline

Compiled MIRR specifications can be formally verified through the integrated EDA toolchain:

```
MIRR Source → Compiler → SystemVerilog + SVA Assertions
    → SymbiYosys BMC (--formal, bounded model checking)
    → SymbiYosys K-Induction (--formal-prove --formal-depth N)
    → EQY Equivalence Checking (--eqy, revision-to-revision parity)
    → ABC Logic Minimization (--optimize, gate reduction)
```

Solver engines are selectable: Z3, Yices, Btor, Bitwuzla. All formal verification results are bounded by configurable depth limits.

### Pathway 3: FPGA Implementation Pipeline

MIRR designs can be taken from specification to physical FPGA implementation:

```
MIRR Source → Compiler → SystemVerilog RTL
    → Yosys Synthesis (technology mapping for target FPGA)
    → NextPNR Place & Route (--pnr)
    → IceTime Static Timing Analysis (--timing)
    → Bitstream Generation (target-specific)
```

Supported FPGA targets: `generic`, `xilinx-7`, `xilinx-us`, `intel-cyclone`, `lattice-ice40`, `lattice-ecp5`, `lattice-nexus`.

### Pathway 4: Yosys-to-MIRR Roundtrip Parity Validation

This pipeline guarantees codegen correctness through roundtrip equivalence:

```
Input Verilog → Yosys Synthesis → JSON Netlist
    → MIRR Hydrator (mirr-hydrate) → MIRR Signals + Reflexes
    → MIRR Compiler → SystemVerilog
    → Yosys Re-synthesis → SAT Equivalence Checking (equiv)
```

This pipeline proves that the MIRR compiler's output is functionally identical to the original Verilog input.

### Pathway 5: Control Plane & RAG Integration

The MCP Semantic Bridge and Knowledge Base provide AI-assisted design capabilities:

```
User Query → MCP Control Plane (crates/mirr-mcp-control-plane/)
    → Tool Dispatch (compile, verify, optimize, explain)
    → Knowledge Base (crates/mirr-kb-native/) RAG Vector Search
    → Synthesis Precedents & Formal Proofs
    → Compiler Pipeline (guided by retrieved knowledge)
```

### Pathway 6: Consumer Facades

Multiple consumer surfaces provide access to the compiler:

- **mirr-wasm** (`crates/mirr-wasm/`): JS-compatible WASM binding for browser-based IDEs and demonstrations
- **lra-cli** (`crates/lra-cli/`): Living Research Artifact tooling for markdown/HTML validation and receipt signing
- **mirr-arsenal-wasm** (`crates/mirr-arsenal-wasm/`): Validation and compile-contract wrapper for the Presidential Arsenal

### Pathway 7: LSP Incremental Recompilation

The LSP server provides a continuous feedback loop for IDE integration:

```
Editor Edit Event → LSP Transport (JSON-RPC)
    → Incremental Re-parse (lsp_incremental.rs)
    → Full Pipeline Re-analysis
    → Diagnostics Generation (diagnostic/)
    → LSP Response (errors, warnings, completions)
```

---

## 5. Development Roadmap & Strategic Milestones

This section tracks the project's completed development phases and the forward-looking milestones for open-source EDA tooling and scientific computing.

### Completed Phases (Phases 0–7h)

All of the following phases have been completed and validated:

| Phase | Name | Status | Key Deliverable |
|-------|------|--------|-----------------|
| 0 | Foundation | ✅ Complete | NASA/JPL-compliant Rust toolchain, `#![forbid(unsafe_code)]` |
| 1 | Mini MIRR DSL | ✅ Complete | Hand-written lexer/parser, strongly-typed AST |
| 2 | Temporal Guard Compiler | ✅ Complete | Cement2-inspired shift-register synthesis |
| 3 | Logic Simplifier | ✅ Complete | SmaRTLy-inspired 33-rule algebraic engine |
| 4 | Bit-Width Inference | ✅ Complete | FIRWINE SCC solver, Unique Least Solution |
| 5 | MAPE-K Simulation | ✅ Complete | Autonomic control loop with LTL checking |
| 6 | Integration & Visualization | ✅ Complete | Unified `mirr-compile` pipeline, DOT/JSON/Verilog emission |
| 7a | Safety Properties & SVA | ✅ Complete | `property` keyword → SystemVerilog Assertions |
| 7b | Homoiconic Pattern System | ✅ Complete | `def`/`reflect` macros with DO-178C traceability |
| 7c | Advanced Type System | ✅ Complete | Dependent, linear, refinement types (E610–E625) |
| 7d | S-Expression IR | ✅ Complete | Bidirectional AST ↔ S-expr with round-trip invariant |
| 7e | R-SPU ISA | ✅ Complete | 30-instruction ISA, tagged-word architecture, cycle-accurate simulator |
| 7f | Proof-Carrying Code | ✅ Complete | PCC certificates, hardware proof verification unit |
| 7g | Symbolic Evaluation | ✅ Complete | Interval analysis, fingerprinting, term rewriting |
| 7h | MAPE-K Hardware RTL | ✅ Complete | Synthesizable autonomic hardware, Yosys-validated |

### In-Progress Phases

| Phase | Name | Status | Key Activity |
|-------|------|--------|--------------|
| 7i | Verified Compilation | 🔄 In Progress | Rocq-verified compiler passes, CompCert-inspired simulation relations |
| 8a | R-SPU Core Architecture | 🔄 In Progress | 64-bit tagged pipeline, 5-stage core with PCC interlock |

### Key Milestones (Updated)

1. ~~**ECS-Native Transition**~~ *(Completed)*: Migrated the compiler pipeline from a tree-based AST to a high-performance ECS Registry. The ECS is the sole source of truth.
2. ~~**Compiler Ergonomics**~~ *(Completed)*: Migrated the entire toolchain into the unified `mirr` CLI router with `clap` interface categorization.
3. ~~**Clock Domain Crossing (CDC)**~~ *(Completed)*: Native support for multiple clock domains via `ClockDomainsComponent` and `PhantomTagsComponent` within the ECS. `src/temporal/clock_domain.rs` handles cross-domain synchronization.
4. ~~**Source-Level Debugger**~~ *(MVP Complete)*: The `mirr-wave` CLI (`src/bin/mirr-wave.rs`) provides VCD waveform viewing with source mapping. The `src/diagnostic/vcd_parser.rs` handles waveform ingestion. Graduation to interactive temporal scrubbing remains future work.
5. ~~**SAT Logic Simplification**~~ *(Completed)*: The `src/sat/` module provides bounded DPLL solving with CNF encoding for guard redundancy elimination. Error codes E900–E902.
6. ~~**MAPE-K Hardware Realization**~~ *(Completed)*: `src/emit/mape_k_rtl/` provides synthesizable autonomic control hardware with Yosys validation and Rocq behavioral equivalence proofs.
7. **Homoiconicity Integration** *(In Progress)*: The `src/sexpr/` engine exists but is not yet wired to the main pipeline. Full "Code as Data" autonomic self-healing is future work.
8. **Scale-Blocker Debugging** *(Ongoing)*: The 64-core R-SPU synthesizes successfully (3.2M+ lines of SystemVerilog). Scaling to 1,024+ cores requires addressing NoC routing congestion and synthesis runtime.
9. **Rocq Proof Engine** *(In Progress)*: 90+ Rocq proofs across 6 proof directories (`proofs/width/`, `proofs/cert/`, `proofs/compiler/`, `proofs/language/`, `proofs/mape_k/`, `proofs/rspu/`). 13.54% symbol coverage. Full pipeline coverage is the Phase 7i target.

### Engine MVP Graduation Status

| Engine | Status | Remaining Work |
|--------|--------|----------------|
| Source-Level Debugger (VCD Parser) | MVP ✅ | Upgrade to interactive cycle-accurate temporal scrubbing |
| ASIC OpenLane Integration | MVP ✅ | Deep GDSII parsing for silicon timing feedback into ECS |
| SAT Logic Simplification | ✅ Graduated | Production-grade with E900–E902 error codes |
| S-Expression Frontend | ✅ Graduated | Full round-trip serialization with macro expansion |
| Multi-Clock Domain Crossing | ✅ Graduated | `clock_domain.rs` with ECS `ClockDomainsComponent` |
| Rocq Proof Engine | 🔄 In Progress | Expand from 13.54% to full pipeline coverage |

---

## 6. Advanced Scientific Computing for Hardware Synthesis (NLnet Milestones)

Your original vision was correct: attempting to build a general-purpose scientific DSL to compete with Python (NumPy/PyTorch) is a losing battle. Python has already won that space. 

Instead, MIRR integrates **Scientific Computing directly into the EDA toolchain**. The goal is not to have physicists write simulations in MIRR, but to give researchers and hardware engineers a compiler that uses advanced mathematics (Wolfram-style symbolic evaluation, PDEs, and tensor optimizations) to generate superhuman chip designs.

### SC-1: Symbolic Circuit Equivalence Prover
**Objective**: Enhance the compiler's ability to use deep mathematical logic to prove that two hardware designs are mathematically identical without running a single simulation cycle.
**Deliverables**:
- Upgrade the `src/symbolic/` engine to support algebraic ring theory and Galois fields.
- Integrate automated theorem proving (similar to Wolfram's `FullSimplify[]`) to mathematically reduce millions of logic gates down to their minimal theoretical form.
- Provide a cryptographic "proof receipt" that researchers can verify independently.

### SC-2: Topological & Spatial Routing (Tensor Math)
**Objective**: Solve MIRR's current "Physical Agnosticism" flaw by using multidimensional tensor mathematics to optimally route the 1,024-core R-SPU.
**Deliverables**:
- Implement force-directed graph drawing and simulated annealing algorithms inside the compiler to calculate the optimal spatial layout of logic blocks.
- Use native tensor operations to model the NoC routing congestion as a network-flow problem, ensuring zero-jitter pathways before handing off to NextPNR.

### SC-3: Thermodynamic & RC Delay PDEs
**Objective**: Use continuous-time mathematical models (Differential Equations) inside the compiler to simulate the physical reality of the chip.
**Deliverables**:
- A compiler phase that calculates the resistor-capacitor (RC) delay of every wire using discrete calculus.
- Thermal dissipation modeling: Using partial differential equations (PDEs) to map heat density across the 3D Cube architecture, allowing the compiler to automatically spread out hot-running logic blocks to prevent thermal throttling.

### SC-4: Automated Algorithmic Retiming
**Objective**: Use linear programming and optimization math to automatically pipeline and retime the generated hardware.
**Deliverables**:
- The compiler mathematically calculates the critical path of the spatial architecture and automatically inserts pipeline registers to maximize the clock frequency (Fmax).
- Guarantees mathematical equivalence using the SC-1 Symbolic Prover after retiming.

### SC-5: AI-Driven Architecture Exploration
**Objective**: Instead of researchers writing AI models *in* MIRR, the MIRR compiler uses AI search algorithms to explore the hardware design space.
**Deliverables**:
- Implement genetic algorithms and gradient descent within the compilation pipeline to find the optimal trade-off between power, area, and speed (PPA).
- Allows researchers to say: "Give me the most power-efficient flight controller," and the compiler uses scientific computing to generate and test 10,000 variations mathematically.

### SC-6: The Researcher's Analysis API
**Objective**: Provide a bridge that lets researchers use Python to analyze MIRR's hardware outputs, rather than forcing them to leave Python.
**Deliverables**:
- A Python/Jupyter binding (`mirr-py`) that allows scientists to query the MIRR compiler via RPC.
- Researchers can write Python scripts to extract the formal proofs, thermal maps, and logic density of their compiled MIRR hardware directly into Pandas or Matplotlib for analysis.

---

## 7. Scientific Computing Milestone Summary

| Milestone | Title | Deliverable | Effort |
|-----------|-------|-------------|--------|
| **SC-1** | Symbolic Prover | Algebraic gate minimization, Proof receipts | 4–6 wk |
| **SC-2** | Topological Routing | Tensor-based NoC congestion optimization | 6–8 wk |
| **SC-3** | Thermodynamic PDEs | Heat mapping and RC delay simulation | 8–10 wk |
| **SC-4** | Automated Retiming | Linear programming for critical path optimization | 4–6 wk |
| **SC-5** | AI Architecture Search | Genetic algorithms for PPA optimization | 10–12 wk |
| **SC-6** | Researcher API | Python/Jupyter bridge (`mirr-py`) for EDA analysis | 8–10 wk |

**Total estimated effort**: 40–52 weeks (8–12 months).

---

## 8. The 1-Billion Transistor Vision (Wafer-Scale AI Engine)

Because the R-SPU is a spatial architecture, it scales "out" rather than "up." If the R-SPU ever hits 1 billion transistors, it won't be because we added bloated x86 legacy baggage, deep branch predictors, or out-of-order execution pipelines. It will be because we built a **massively parallel synthetic brain** for robotics.

A 1-Billion Transistor R-SPU would consist of:

1. **Massive Core Count (1,024+ Cores)**:
   Scaling the NoC to thousands of independent cores allows massively parallel, hard real-time spatial processing. The existing 64-core R-SPU (`reflex_soc/`) has been validated through Yosys synthesis and SymbiYosys formal verification. Scaling to 1,024 cores requires solving the NoC routing congestion problem — the current mesh topology does not scale linearly. Hierarchical NoC with local clusters (16 cores per cluster, 64 clusters) is the proposed solution, with each cluster sharing a local crossbar (`reflex_soc/crossbar_formal.mirr`) and inter-cluster communication via a hierarchical router (`reflex_soc/noc_router_formal.mirr`).

2. **Massive "Local" Memory (SRAM)**:
   True deterministic AI and robotics cannot afford to wait hundreds of clock cycles for external DRAM. By embedding massive amounts of ultra-fast SRAM directly inside the R-SPU silicon (e.g., 1MB per core across 1,024 cores = 1GB total on-chip), the chip could store entire neural network models locally. The MIRR compiler already supports memory subsystem modeling through the ECS `MemoryComponent`, but physical SRAM macro instantiation requires ASIC-specific backend support (OpenLane integration, SC-5 target).

3. **Matrix Math Units (Tensor Cores)**:
   Augmenting the 64-bit ALUs with dedicated Matrix Multiplication units on every core. The R-SPU ISA already includes tagged-word support for different numeric types (Phase 7e), and SC-2's tensor primitives provide the language-level abstraction. Hardware realization maps tensor operations to systolic arrays synthesized through the HLS optimizer (`src/hls/`). This allows the deterministic execution of advanced AI models (Vision Transformers, localized LLMs) across the spatial grid without non-deterministic memory access patterns.

4. **Dedicated Safety Fabric**:
   A physically separate network connecting all MAPE-K monitor units across every core, independent of the data NoC. This safety fabric ensures that autonomic health monitoring is never starved by data traffic. Each core's LTL checker reports to a hierarchical aggregator that can trigger chip-wide emergency responses within a single clock cycle.

**The Ultimate Goal:**
A sprawling grid of thousands of tiny, hyper-efficient, jitter-free cores swimming in a sea of local memory, communicating over a massive hierarchical NoC, and processing thousands of physical reflexes simultaneously without dropping a single frame. Every core carries its own PCC certificate. Every signal path is formally verified. Every safety invariant is checked in hardware at wire speed.

---

## 9. The Cube Architecture (3D Spatial Silicon)

Moving beyond planar 2D scaling, the architecture roadmap includes exploratory support for **3D Spatial Silicon** (internally referred to as "The Cube").

Drawing inspiration from techniques like Huawei's LogicFolding and Tau (τ) Scaling, this methodology allows the MIRR compiler to synthesize logic that spans both horizontal and vertical silicon layers simultaneously. The key technical challenges and proposed solutions:

1. **Vertical Signal Routing**: Through-Silicon Vias (TSVs) provide the z-axis connections, but their pitch is orders of magnitude larger than planar metal routing. The MIRR compiler must be aware of TSV placement constraints during synthesis, adding a new dimension to the physical P&R problem identified in the Known Technical Flaws section.

2. **Thermal Management**: By utilizing ultra-efficient cores (similar to Apple Silicon's performance-per-watt optimization) and internal micro-fluidic liquid cooling (data-center grade two-phase immersion), the R-SPU can achieve staggering logic density without thermal throttling. The MAPE-K monitor stage already includes die temperature sensors — in a 3D architecture, each layer would have independent thermal monitoring feeding into the Analyze stage.

3. **Layer Assignment**: The compiler must decide which logic blocks occupy which silicon layer. Timing-critical paths should be placed on the same layer to avoid TSV latency. The R-SPU's natural cluster hierarchy (from Section 8) maps well: each cluster occupies a single layer, with inter-cluster communication routed vertically.

4. **Yield and Redundancy**: 3D fabrication has lower yield per layer. The R-SPU's spatial architecture is naturally fault-tolerant — a failed core in a 1,024-core grid can be disabled and its workload redistributed without redesigning the chip. MAPE-K's Plan stage selects alternative routing around failed cores.

---

## 10. The Minecraft Redstone Paradigm

To conceptualize the R-SPU and the future 3D Spatial Silicon architectures, it is highly accurate to draw a direct architectural parallel to **Minecraft Redstone** computers.

Standard von Neumann CPUs process instructions sequentially, utilizing software loops and threading overhead to emulate parallelism. In contrast, both the R-SPU and Minecraft Redstone computers embody true **Spatial Computing**. Logic is laid out as physical gates in space, and data flows through them continuously and asynchronously.

The parallels are precise:

| Minecraft Redstone | R-SPU Architecture |
|--------------------|--------------------|
| Redstone dust (wire) | Signal declarations (`signal x: u8`) |
| Repeater (delay) | Temporal guard (`delay(k)` → shift register) |
| Comparator (logic) | Reflex block (`reflex compute { ... }`) |
| Piston (output) | Output signal (`signal out: out bool`) |
| Observer (event) | Guard trigger (`when signal_x for 3 cycles`) |
| 3D vertical stacking | Cube Architecture (TSV-connected layers) |
| Chunk loading distance | RC delay / routing congestion |

Furthermore, Redstone architectures naturally evolve into vertical stacking (3D structures) to minimize signal delay and bypass horizontal rendering distances. This 3D spatial logic routing is precisely the goal of MIRR's future Cube Architecture, mirroring the real-world engineering challenge of minimizing resistive-capacitive (RC) delay by stacking logic blocks vertically.

The MIRR compiler is, in a very real sense, a **Redstone compiler** — it takes a behavioral description of what the machine should do and maps it to a spatial arrangement of gates, wires, and delays in physical space.

---

## 11. MIRR's Scientific Heritage

### Heritage: From Software Language to HDL

MIRR was not born as a Hardware Description Language. It was originally conceived as a **software programming language** — a Domain-Specific Language (DSL) conceptually similar to the **Wolfram Language (Mathematica)**. In its earliest incarnation, MIRR was designed for symbolic evaluation, mathematical modeling, and functional transformations of complex state systems.

This heritage is not accidental — it is the reason MIRR has capabilities that no other HDL possesses. Traditional HDLs (Verilog, VHDL, Chisel) are purely descriptive: they describe hardware structure. MIRR's software-language DNA gives it the ability to **reason about** hardware, not just describe it. The symbolic evaluation engine (`src/symbolic/`) doesn't just pass signals through — it performs interval analysis, computes discrete derivatives, fingerprints waveform signatures, and rewrites expressions algebraically. 

### Why This Matters for Open Source EDA

The current open-source EDA landscape (Yosys, OpenROAD, NextPNR) is incredible, but the tools are highly disjointed and rely on decades-old C++ algorithms that were not built for massive parallelism or modern AI-driven architectural searches.

By embedding heavy scientific computation **directly into the compiler**, researchers don't have to abandon Python to use MIRR. Instead, they use Python (via `mirr-py`) to instruct the MIRR compiler to perform hyper-advanced mathematical synthesis on their designs.


### Why This Matters for Open Science

The current scientific computing landscape is fragmented:
- **MATLAB/Wolfram**: Proprietary, expensive, closed-source
- **Python/NumPy/SciPy**: Open-source but dynamically typed, no formal verification, no hardware synthesis path
- **Julia**: Excellent numerics but no hardware target
- **HDLs** (Verilog/VHDL): Hardware-only, no scientific computing capability

MIRR, with the scientific computing milestones (SC-1 through SC-6), occupies a unique position: an **open-source, formally verified, dual-use language** that bridges scientific computing and hardware design. A researcher publishes a MIRR program as an executable paper. A hardware engineer takes that same program and synthesizes it to an FPGA. The formal properties travel with the code, ensuring that the hardware implementation preserves the mathematical guarantees of the research model.

This is what makes the scientific computing extension transformative: it isn't just another numerical tool. It is the **bridge between mathematical models and physical hardware**, with formal verification as the structural guarantee.

---

## 12. Research Foundation & Key Benchmarks

### Target Benchmarks

| Technology | Metric | Source |
|---|---|---|
| Cement2 | 377 MHz timing closure on RISC-V soft-core | Xiao et al. 2025 |
| SmaRTLy | 8.95% AIG reduction on RISC-V (early milestone) | Li et al. 2025 |
| SmaRTLy | 47.2% AIG reduction vs Yosys (industrial, full milestone) | Li et al. 2025 |
| FIRWINE | Unique Least Solution — formally proven optimal | Wang et al. 2026 |
| R-SPU LTL | Sub-cycle fault detection (nanosecond response) | Architecture goal |
| R-SPU DPR | Millisecond reconfiguration + static clamp in 1 cycle | Architecture goal |
| MIRR Compiler | 155k+ LOC, 5,200+ tests, 0 unsafe, 0 clippy warnings | Current state |
| R-SPU 64-core | 3.2M+ lines synthesized SystemVerilog | Current state |
| Formal Proofs | 90+ Rocq proofs, 13.54% symbol coverage | Current state |

### Core Technology References

- Xiao, Y. et al. (2025). Cement2: Temporal hardware transactions for FPGA programming. arXiv:2511.15073
- Li, C. et al. (2025). SmaRTLy: RTL optimization with logic inferencing and structural rebuilding. arXiv:2510.17251
- Wang, K. et al. (2026). FIRWINE: A formally verified procedure for width inference in FIRRTL. arXiv:2601.12813
- Arcaini, P. et al. (2015). Modeling and analyzing MAPE-K feedback loops for self-adaptation. SEAMS 2015.
- Pnueli, A. (1977). The temporal logic of programs. 18th Annual Symposium on Foundations of Computer Science. IEEE.
- Cong, J., & Zhang, Z. (2006). An efficient and versatile scheduling algorithm based on SDC formulation. DAC 2006.
- Lin, I.-C. et al. (2016). Aging-aware reliable multiplier design with adaptive hold logic. IEEE Trans. VLSI Systems, 24(3), 844–853.
