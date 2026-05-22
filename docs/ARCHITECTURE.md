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
│   ├── typeck/               # Type checking & constraint validation
│   ├── width/                # Width constraint solving (SCC paths)
│   ├── temporal/             # Temporal lowering of hardware guards
│   ├── emit/                 # Emission backends (SystemVerilog, FIRRTL, R-SPU, etc.)
│   ├── bin/                  # Executable entrypoints (mirr-compile, mirr-general, etc.)
│   └── lib.rs                # Public module exports
├── crates/                   # Consumer & Control Plane Surfaces
│   ├── mirr-wasm/            # WASM compilation API (browser/JS consumers)
│   ├── mirr-arsenal-wasm/    # Validation and compile-contract wrapper
│   ├── lra-cli/              # Living Research Artifact CLI tooling
│   ├── mirr-kb-native/       # Knowledge Base native engine (RAG / Vector search)
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

MIRR is a safety-critical compiler platform with three kinds of surfaces:

1. **Core compiler**: Parse -> Validate/Expand -> Type/Width -> Temporal Lowering -> Emit.
2. **Control planes**: MRT / Presidential Arsenal, KB-lite, and private campaign planning.
3. **Consumers and bridges**: WASM, LRA, MCP, VS Code, paper/demos, proofs, fuzz, and CI scripts.

**Inputs:**
- MIRR language (Signal/Guard/Reflex + properties/patterns)

**Outputs:**
- SystemVerilog, FIRRTL, JSON netlist, DOT graphs, S-expression IR, R-SPU assembly/binary, Yosys-compatible netlists

## 3. Data Flow Pathways

The following statements describe how services and subsystems interact within the MIRR ecosystem:

1. **Compiler Pipeline Flow:** 
   - Source code is ingested by the [Lexer/Parser](../src/parser/module_parser/mod.rs), generating an AST.
   - The AST is hydrated into the [ECS Registry](../src/ecs/registry.rs), where all hardware declarations become entities.
   - [Semantic Validation](../src/ecs/semantic_validate.rs) ensures entity integrity (Name and Kind constraints).
   - [Width Solver](../src/width/solver.rs) infers missing signal widths using SCC propagation.
   - [Temporal Lowering](../src/temporal/mod.rs) translates hardware guards into deterministic netlist primitives. This pass is fully ECS-native (Phase 3 transition complete), using the ECS Registry as the primary source of truth. It synthesizes `TemporalNodeComponent` metadata for each guard entity, closing the "Temporal Seam".
   - Finally, [Emission Backends](../src/emit/mod.rs) generate the target artifacts.

2. **Control Plane & RAG Integration:**
   - The [MCP Semantic Bridge](../crates/mirr-mcp-control-plane/) routes cross-subsystem tool calls.
   - The [Knowledge Base Engine](../crates/mirr-kb-native/) serves as a RAG vector store. The compiler queries `mirr-kb-native` for synthesis precedents and formal proofs before resolving complex guard patterns.

3. **Consumer Facades:**
   - The [mirr-wasm](../crates/mirr-wasm/) crate provides a JS-compatible binding over the core compiler, enabling browser-based IDEs and demonstrations.
   - The [lra-cli](../crates/lra-cli/) interacts with the Arsenal tooling for markdown/HTML validation and receipt signing.

4. **Yosys-to-MIRR Roundtrip Parity Validation Pipeline:**
   - Arbitrary input Verilog/RTL is synthesized using Yosys into a technology-mapped JSON netlist.
   - The [MIRR Hydrator](../src/bin/mirr-hydrate.rs) ingests this JSON and maps the cells directly back into MIRR signals and reflexes.
   - The compiled MIRR output is then lower-synthesized again via Yosys and formally verified using SAT Equivalence Checking (`equiv`) to guarantee absolute codegen parity and correct optimization.

## 4. Architectural Component Graphs

### Core Pipeline Architecture

```mermaid
flowchart TD
    subgraph Core[Core Compiler Pipeline]
        A[Lexer & Parser] -->|AST| B[ECS Hydration & Validation]
        B -->|Entities| C[Type Checking & Width Inference]
        C -->|Typed Netlist| D[Temporal Lowering]
        D -->|Deterministic IR| E[Emit Backends]
    end

    Input([.mirr source]) --> A
    E --> V([SystemVerilog])
    E --> F([FIRRTL])
    E --> R([R-SPU Bytecode])

    subgraph Roundtrip[Yosys Parity Roundtrip]
        YosysIn([Original Verilog RTL]) -->|yosys synth| YosysJSON[JSON Netlist]
        YosysJSON -->|mirr-hydrate| Input
        V -->|gold vs gate| SAT[Yosys SAT Equivalence Check]
        YosysIn -->|gold vs gate| SAT
    end
```

### Ecosystem Integration Map

```mermaid
flowchart LR
    subgraph Core[Core Compiler]
        Pipeline[mirr pipeline]
    end

    subgraph ControlPlane[Command & Control]
        MCP[mcp_server]
        KB[mirr-kb-native]
        Audit[mirr-audit]
        Wave[mirr-wave]
    end

    subgraph Consumers[Consumer Surfaces]
        WASM[mirr-wasm]
        LRA[lra-cli]
        VSCode[vscode-mirr]
    end

    Pipeline <--> KB
    MCP --> Pipeline
    MCP --> Audit
    MCP --> Wave
    WASM --> Pipeline
    LRA --> Pipeline
    VSCode --> MCP
```

## 5. Main Projects At A Glance

### Core Layer

| Project | What it is | Architecture overview | Current phase/status |
|---|---|---|---|
| [`src`](../src) / core compiler | The compiler engine | Front-end -> semantic validation -> type/width solving -> temporal lowering -> emit backends | Phases 0-7e complete; 7f+ not started |
| [`src/bin/mirr-hydrate.rs`](../src/bin/mirr-hydrate.rs) | MIRR Hydrator | Converts Yosys JSON technology-mapped netlists to structured MIRR files | Core component; complete and verified |
| [`compiler_mirr`](../compiler_mirr) | Self-hosting compiler subset | MIRR-written bootstrap implementation | Stage-1 hosted self-hosting, in progress |

### Control Plane Layer

| Project | What it is | Architecture overview | Current phase/status |
|---|---|---|---|
| `MRT / Presidential Arsenal` | The command/control plane | `mirr-audit`, `mirr-brain`, `mirr-wave`, `mirr-general`, `mirr-lsp`; KB-lite local governance via `mcp_server` | Shared governance/roadmap layer |

### Consumer / Bridge Layer

| Project | What it is | Architecture overview | Current phase/status |
|---|---|---|---|
| [`crates/mirr-wasm`](../crates/mirr-wasm) | Browser/WASM compiler API | wasm-bindgen facade over `run_pipeline` | Consumer surface; parity-gated |
| [`crates/mirr-arsenal-wasm`](../crates/mirr-arsenal-wasm) | Arsenal/RWFI2 contract bridge | WASM validation wrapper over core compiler | Consumer surface; parity-gated |
| [`crates/lra-cli`](../crates/lra-cli) | Living Research Artifact CLI | Arsenal-facing CLI for local serving & validation | Arsenal-facing CLI surface |
| [`crates/mirr-mcp-control-plane`](../crates/mirr-mcp-control-plane) | MRT MCP bridge | Stdio MCP bridge dispatching to `mirr-*` CLI | Bridge surface; contract-tested |

## 6. First 90 Minutes: Repository Onboarding Sequence

1. **Read Product & Constraints:**
   - [`README.md`](../README.md)
   - [`GEMINI.md`](../GEMINI.md)

2. **Read Compiler API Surface:**
   - [`src/lib.rs`](../src/lib.rs)

3. **Read Entrypoint Binaries:**
   - [`src/bin/mirr-compile/main.rs`](../src/bin/mirr-compile/main.rs)
   - [`src/bin/mirr-general/main.rs`](../src/bin/mirr-general/main.rs)

4. **Run Confidence Gate:**
   - `./target/debug/mirr-general ci --format json`

## 7. Roadmap Crosswalk

| Phase | Status | What it is |
|---|---|---|
| Phase 0 - 4 | Complete | Foundation through Width inference |
| Phase 5 | Complete | MAPE-K simulation harness |
| Phase 6 | Complete | Integration and visualization |
| Phase 7a-d | Complete | Safety properties, pattern system, advanced types, S-expression IR |
| Phase 7e | Complete | Hardened Temporal Simulator (Double-buffered, Cycle-accurate, Prev) |
| Phase 7f-h | In Progress | Proof-carrying code, symbolic evaluation, MAPE-K hardware realization |

## 8. Practical Pitfalls In This Workspace

- **Environment Note:** `cargo run --bin <name> -- <args>` is broken in this environment due to a rustup home-dir error. Always invoke compiled binaries directly (`./target/debug/<name>`).
- Do not trust stale status docs/logs over source and tests.
- Self-hosting is active but still evolving; parity tests are a better truth source than narrative status files.
