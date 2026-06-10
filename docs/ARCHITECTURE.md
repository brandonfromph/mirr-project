# MIRR Project Architecture & Directory Topology

Date: 2026-05-17

This document provides a comprehensive map of the repository, detailing the structural layout, data flow pathways, and architectural components of the MIRR compiler platform.

> [!IMPORTANT]
> **MAINTENANCE MANDATE**: This file is the single source of truth for the repository's topology. Under the project's zero-debt policy, whenever a new module, crate, or major architectural flow is added, moved, or removed, **you must immediately update this document** to reflect the new state. Outdated architecture mapping is strictly forbidden.

## 1. Directory Map

A clear tree breakdown explaining the directory structure of the repository:

```text
mirr-private/
├── src/                      # Core Compiler Pipeline
│   ├── ast/                  # Abstract Syntax Tree definitions
│   ├── parser/               # Front-end parsing (Lexer & Parser)
│   ├── ecs/                  # Entity Component System (ECS) architecture (Registry)
│   ├── cert/                 # MEGA-4 Proof certificate format & MEGA-16 PCC verifier
│   ├── typeck/               # Type checking & constraint validation
│   ├── width/                # Width constraint solving (SCC paths)
│   ├── temporal/             # Temporal lowering of hardware guards
│   ├── symbolic/             # Hardware-preparatory symbolic evaluation engine
│   ├── emit/                 # Emission backends (SystemVerilog, FIRRTL, R-SPU, etc.)
│   ├── bin/                  # Executable entrypoints (mirr-compile, mirr-general, etc.)
│   └── lib.rs                # Public module exports
├── crates/                   # Consumer & Control Plane Surfaces
│   ├── mirr-wasm/            # WASM compilation API (browser/JS consumers)
│   ├── mirr-arsenal-wasm/    # Validation and compile-contract wrapper
│   ├── lra-cli/              # Living Research Artifact CLI tooling
│   ├── mirrc-kb/       # Knowledge Base native engine (RAG / Vector search)
│   └── mirr-mcp-control-plane/ # Model Context Protocol bridge & governance
├── tests/                    # Integration & Parity Test Matrix
├── fuzz/                     # Fuzzing harnesses for robustness
├── proofs/                   # Formal verification (Coq/Rocq)
├── docs/                     # Documentation and Architecture definitions
├── proposals/                # Architectural proposals and RFCs
├── rspu_chip/                # Primary hardware project (16-core R-SPU processor)
├── stdlib/                   # Standard library definitions
└── compiler_mirr/            # Self-hosting bootstrap implementation
```

## 2. One-Screen Mental Model

MIRR is a safety-critical compiler platform (61k+ LOC) with three distinct layers:

1. **Core Compiler Sub-Engines**: 14 specialized modules (Temporal, Symbolic, SAT, HLS, etc.) that perform rigorous behavioral-to-physical translation.
2. **Control Planes**: MRT / Presidential Arsenal and KB-native (RAG) for autonomic governance and knowledge-backed synthesis.
3. **Consumer Bridges**: WASM, LRA-CLI, and MCP-Control-Plane providing multi-surface accessibility.

### Project Specialization: The R-SPU Architecture
MIRR is a specialized tool optimized for designing and verifying the **R-SPU (Reflexive Signal Processing Unit)**. It is intended for the high-assurance "brains" of robotic systems and autonomous medical/aerospace hardware.

### Known Technical Flaws
The following flaws have been identified in the current compiler implementation (Phase 6):

1.  **Single-Clock Rigidity**: The compiler lacks native support for Multiple Clock Domains (CDC). All logic is currently tied to a single synchronous `clk`.
2.  **Physical P&R Agnosticism**: The compiler has no awareness of physical geometry or floorplanning, which can lead to timing closure failures on large-scale chips (like the 16-core R-SPU).

**Inputs:**
- MIRR specifications (Signals, Guards, Reflexes, Properties, Patterns)

**Outputs:**
- Verilog RTL (always_ff synchronous), FIRRTL, R-SPU Assembly, JSON Netlists, Formal S-Expressions.

## 3. The 14 Architectural Sub-Engines

The core of MIRR is partitioned into 14 high-assurance engines, each adhering to NASA's Power-of-10 rules for ultra-reliable execution.

### Logic & Optimization
1.  **Temporal Guard Compiler** (`src/temporal/compiler.rs`): Translates high-level temporal guards into deterministic shift-register and counter primitives.
2.  **SAT Logic Solver** (`src/sat/mod.rs`): A bounded iterative DPLL solver for proving expression equivalences and verifying simplification candidates.
3.  **High-Level Synthesis (HLS) Optimizer** (`src/hls/mod.rs`): Performs ASAP/ALAP scheduling, resource sharing, and FIFO streaming synthesis for hardware realization.
4.  **Register Retiming Optimizer** (`src/temporal/retiming.rs`): Employs Leiserson-Saxe retiming to minimize critical path delay by moving registers across combinational logic.

### Verification & Assurance
5.  **Symbolic Evaluation Engine** (`src/symbolic/mod.rs`): Implements interval-based abstract interpretation to prove signal value bounds and structural netlist equivalence.
6.  **Totality Engine** (`src/totality/mod.rs`): Verifies the five Pillars of Totality: resource bounds, output completeness, guard coverage, temporal finiteness, and dependency acyclicity.
7.  **S-Expression Transpiler** (`src/sexpr/convert/to_sexpr.rs`): Generates homoiconic IR for formal verification bridges (Z3, Rocq).
8.  **R-SPU Silicon Simulator** (`src/emit/rspu_sim/mod.rs`): Provides cycle-accurate, bit-precise simulation for 16-core R-SPU programs.

### Infrastructure & Orchestration
9.  **ECS Registry** (`src/ecs/registry.rs`): A high-performance SoA (Structure of Arrays) registry managing up to 1M hardware entities. Currently serves as the final synthesis IR.
10. **Cross-Module Symbol Resolver** (`src/symbols/resolver.rs`): Manages cross-crate namespace resolution and visibility.
11. **S-Expression "Code as Data" Engine** (`src/sexpr/mod.rs`): A homoiconic IR with a bounded, iterative eval/apply core. **Current Status**: Not wired to the main pipeline. Used exclusively for self-hosting bootstrap and formal verification bridges.
12. **Semantic Type Checker** (`src/typeck/mod.rs`): Enforces signedness consistency. **Current Role**: Production-grade AST checker; ECS-native typechecking is currently a shadow gate.
13. **MAPE-K Analyzer** (`src/mape_k/analyzer.rs`): Evaluates bounded LTL properties over rolling windows for autonomic safety.
14. **MAPE-K Telemetry Bridge** (`src/mape_k/bridge/mod.rs`): Orchestrates the 16-core telemetry fabric (Proposal 045) for cross-core safety coordination.

## 4. Data Flow Pathways

The following pathways describe the lifecycle of a MIRR specification:

1. **Compiler Pipeline Flow:** 
   - Source code is ingested by the [Lexer/Parser](../src/parser/module_parser/mod.rs), generating contiguous entities and flat-data arrays directly.
   - The raw entity data is populated into the [ECS Registry](../src/ecs/registry.rs), where all hardware declarations become entities.
   - [Semantic Validation](../src/ecs/semantic_validate.rs) ensures entity integrity (Name and Kind constraints).
   - [Width Solver](../src/width/solver.rs) infers missing signal widths using SCC propagation. The ECS-native width inference system is the primary fully-functional engine.
   - [Temporal Lowering](../src/temporal/mod.rs) translates hardware guards into deterministic netlist primitives. This pass is fully ECS-native (Phase 3 transition complete), using the ECS Registry as the primary source of truth. It synthesizes `TemporalNodeComponent` metadata for each guard entity, closing the "Temporal Seam".
   - [Symbolic Evaluation Engine](../src/symbolic/mod.rs) provides abstract interpretation, discrete calculus approximations, anomaly signature fingerprinting, and a term rewriting engine for runtime logic optimization.
   - Finally, [Emission Backends](../src/emit/mod.rs) generate the target artifacts.

2. **Control Plane & RAG Integration:**
   - The [MCP Semantic Bridge](../crates/mirr-mcp-control-plane/) routes cross-subsystem tool calls.
   - The [Knowledge Base Engine](../crates/mirrc-kb/) serves as a RAG vector store. The compiler queries `mirrc-kb` for synthesis precedents and formal proofs before resolving complex guard patterns.

3. **Consumer Facades:**
   - The [mirr-wasm](../crates/mirr-wasm/) crate provides a JS-compatible binding over the core compiler, enabling browser-based IDEs and demonstrations.
   - The [lra-cli](../crates/lra-cli/) interacts with the Arsenal tooling for markdown/HTML validation and receipt signing.

4. **Yosys-to-MIRR Roundtrip Parity Validation Pipeline:**
   - Arbitrary input Verilog/RTL is synthesized using Yosys into a technology-mapped JSON netlist.
   - The [MIRR Hydrator](../src/bin/mirr-hydrate.rs) ingests this JSON and maps the cells directly back into MIRR signals and reflexes.
   - The compiled MIRR output is then lower-synthesized again via Yosys and formally verified using SAT Equivalence Checking (`equiv`) to guarantee absolute codegen parity and correct optimization.

   ## 5. 12-Month R&D Roadmap

   This section outlines the strategic development plan for the next 12 months, aimed at maturing MIRR into a production-grade EDA platform for the R-SPU architecture.

   ### Goal: Scaling and Robustness
   The primary objective is to transition from a high-assurance prototype to a mass-scale logic generator, increasing the test suite from **4,000+** to **8,000+** individual test cases.

   ### Key Milestones:
   1.  **ECS-Native Transition**: Complete the migration of the compiler pipeline from a tree-based AST to a high-performance ECS Registry.
   2.  **Homoiconicity Integration**: Implement a "Code as Data" core to enable autonomic self-healing and knowledge-backed synthesis.
   3.  **Scale-Blocker Debugging**: Perform a rigorous audit to identify and resolve logic bottlenecks that prevent scaling beyond 16-core designs.
   4.  **Engine Wiring**: Complete and wire the 14 identified sub-engines (Symbolic, SAT, MAPE-K, etc.) into a unified, high-assurance pipeline.
   5.  **Compiler Ergonomics**: Improve the MIRR language syntax and CLI feedback loops to reduce the barrier to entry for hardware architects.
   6.  **Clock Domain Crossing (CDC)**: Implement native support for multiple clock domains to support industrial-grade SoC designs.
   7.  ~~**Source-Level Debugger**~~ *(Completed)*: Implement a bit-precise hardware debugger that maps generated Verilog waveforms back to the original MIRR source lines.

## 6. The 1-Billion Transistor Vision (Wafer-Scale AI Engine)

Because the R-SPU is a spatial architecture, it scales "out" rather than "up." If the R-SPU ever hits 1 billion transistors, it won't be because we added bloated x86 legacy baggage, deep branch predictors, or out-of-order execution pipelines. It will be because we built a **massively parallel synthetic brain** for robotics.

A 1-Billion Transistor R-SPU would likely consist of:

1. **Massive Core Count (1,024+ Cores):**
   Scaling the NoC to thousands of independent cores allows massively parallel, hard real-time spatial processing. For example, assigning 2,000 cores just to process incoming lidar/camera arrays deterministically (a synthetic visual cortex), or thousands of cores independently controlling micro-actuators without OS scheduling overhead.

2. **Massive "Local" Memory (SRAM):**
   True deterministic AI and robotics cannot afford to wait hundreds of clock cycles for external DRAM. By embedding massive amounts of ultra-fast SRAM directly inside the R-SPU silicon (e.g., 1MB per core across 1,024 cores), the chip could store entire neural network models locally.

3. **Matrix Math Units (Tensor Cores):**
   Augmenting the simple 64-bit ALUs with dedicated Matrix Multiplication units on every single core. This allows the deterministic execution of advanced AI models (like localized Vision Transformers or LLMs) in a single clock cycle across the spatial grid.

**The Ultimate Goal:**
A sprawling grid of thousands of tiny, hyper-efficient, jitter-free cores swimming in a sea of local memory, communicating over a massive NoC router, and processing thousands of physical reflexes simultaneously without dropping a single frame.

