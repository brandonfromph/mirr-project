# MIRR Compiler (Rust) — Robust User & Developer Guide

> ⚠️ **Living document:** the repository is evolving rapidly.  This README
> reflects the state as of early March 2026; for the most up-to-date index of
> design docs and their status, see [`docs/INDEX.md`](docs/INDEX.md).  When
> contributing new files or significant structural changes, update both the
> README and `docs/INDEX.md` accordingly.


Welcome to the MIRR project! This document is your **comprehensive guide** to using, experimenting with, and extending MIRR — a domain-specific language and toolchain for safety-critical reflex logic.

---

## Table of Contents

1. [What is MIRR?](#what-is-mirr)
2. [Why Use MIRR?](#why-use-mirr)
3. [Who Should Use MIRR?](#who-should-use-mirr)
4. [Quickstart: Try MIRR in 5 Minutes](#quickstart-try-mirr-in-5-minutes)
5. [Beginner Quickstart (Step-by-Step)](#beginner-quickstart-step-by-step)
6. [CLI Reference](#cli-reference)
7. [Experimenting & Research: Advanced Usage](#experimenting--research-advanced-usage)
8. [How MIRR Works: Architecture & Internals](#how-mirr-works-architecture--internals)
9. [How to Extend MIRR (Developer Guide)](#how-to-extend-mirr-developer-guide)
10. [Key Documentation & Specs](#key-documentation--specs)
11. [Repository Structure](#repository-structure)
12. [Repo Hygiene & Contribution Rules](#repo-hygiene--contribution-rules)

---

## What is MIRR?

**MIRR** is a small, self-hosting domain-specific language for **safety-critical reflex logic**. It lets you describe rules like:

> “If this dangerous condition stays true for N cycles, trigger a protective action immediately.”

**Example:**

```mirr
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
```

MIRR is designed for **deterministic, bounded behavior** so designs can be reasoned about and lowered to finite hardware-style structures (counters, shift registers, logic).

---

## Why Use MIRR?

- **Learn compiler internals** without huge complexity: lexer → parser → semantic checks → lowering.
- **Practice temporal logic modeling**: not just “if condition,” but “if condition for duration.”
- **Explore safety-oriented language design**: explicit bounds, deterministic behavior, no hidden magic.
- **See self-hosting in action**: the incremental lexer module is already implemented in MIRR and exercised by the interpreter (try `cargo run --bin run_lexer`). other compiler stages are currently placeholders; full self-hosting will arrive as these are ported (see `compiler_mirr/` and `docs/roadmap.md`).
- **Experiment with research-grade reproducibility**: run experiments, benchmarks, and parity tests.

---

## Who Should Use MIRR?

- Learners interested in compilers, DSLs, or language design.
- Language/tooling developers seeking a compact, real-world experimental language.
- Engineers working on temporal/safety logic for control systems.

**Note:** MIRR is not a general-purpose app framework. It is a focused language + compiler engineering project.

---

## Quickstart: Try MIRR in 5 Minutes

```bash
# 1) Build
cargo build

# 2) Parse a sample program (prints AST)
cargo run -- examples/neonatal_respirator.mirr

# 3) Lower temporal guards (prints summary)
cargo run -- --compile examples/neonatal_respirator.mirr

# 4) Emit machine-readable IR JSON
cargo run -- --compile --json examples/neonatal_respirator.mirr
```

**Expected output:**
- Step 2: structured AST output.
- Step 3: temporal compilation summary.
- Step 4: netlist JSON with guards/signals/statistics.

---

## Beginner Quickstart (Step-by-Step)

### 1. Prerequisites
- Rust toolchain (stable): [https://rustup.rs](https://rustup.rs)
- Windows PowerShell (for `run-mirr.ps1`) or any shell that can run `cargo`

### 2. Build
```bash
cargo build          # compiles the core library + CLI
cargo build --bin generate_mirr_stress  # also compiles the stress-test generator binary
```

### 3. Parse an Example MIRR File
```bash
cargo run -- examples/neonatal_respirator.mirr
```
Prints the parsed AST.

### 4. Compile Temporal Guards
```bash
cargo run -- --compile examples/neonatal_respirator.mirr
```

#### JSON Output
```bash
cargo run -- --compile --json examples/neonatal_respirator.mirr
```

#### Graphviz DOT Output
```bash
cargo run -- --compile --dot examples/neonatal_respirator.mirr
```

### 5. Run All Tests
```bash
cargo test            # runs the full Rust unit/integration suite
```

### Optional: Run Stress Generator
```bash
cargo run --bin generate_mirr_stress -- --type mux_forest --size 1000 > big_test.mirr
# then compile/parse that file with the CLI to exercise parser/compiler
```

### MCP Server (if you need agent tooling)
The `mcp_server/` directory contains a TypeScript project that provides a
multi-channel protocol server over stdio.  It must be built separately via
`npm install && npm run build` before use (see `mcp_server/README.md`).

### 6. Optional PowerShell Shortcut

**Demo the MIRR lexer**
```powershell
cargo run --bin run_lexer   # exercises compiler_mirr/lexer.mirr
```

```powershell
./run-mirr.ps1
./run-mirr.ps1 ./examples/neonatal_respirator.mirr
```

---

## CLI Reference

```text
nasa-rust-project [OPTIONS] <file.mirr>

Options:
  -c, --compile                Compile temporal guards
  -j, --json                   Emit netlist JSON (with --compile)
  -d, --dot                    Emit DOT graph (with --compile)
  --verilog                    Emit simple Verilog module (with --compile)
      --selfhost-compile       Run self-hosting bootstrap pipeline
      --selfhost-compile-json  Same as above, also emit netlist JSON
  -h, --help                   Show help
```

**Examples:**
- `nasa-rust-project example.mirr` — Parse and display AST
- `nasa-rust-project --compile example.mirr` — Compile temporal guards
- `nasa-rust-project --compile --json example.mirr` — Compile and emit JSON
- `nasa-rust-project --compile --dot example.mirr` — Compile and emit DOT
- `nasa-rust-project --compile --verilog example.mirr` — Compile and emit simple Verilog
- `nasa-rust-project --selfhost-compile example.mirr` — Run self-hosting pipeline

---

## Experimenting & Research: Advanced Usage

MIRR is designed for **reproducible research and experimentation**. The `scripts/research/run_experiments.py` script automates:

- **Temporal strategy sweeps** (ShiftRegister vs Counter)
- **Determinism runs** (output hash stability)
- **Throughput benchmarks** (median/p95/stddev)
- **Bootstrap failure-mode checks** (pipeline error handling)

**To run all experiments and generate CSV/Markdown artifacts:**

```bash
python scripts/research/run_experiments.py
# Artifacts will be written to artifacts/research/
```

Artifacts include:
- `strategy_sweep.csv`, `determinism_runs.csv`, `throughput_baseline.csv`, `bootstrap_failure_modes.csv`, `run_metadata.json`, `summary.md`

See the script and `artifacts/research/summary.md` for full methodology and results.

---

## How MIRR Works: Architecture & Internals

### Pipeline Overview

```
Source (.mirr)
   │
   ▼
┌─────────┐   ┌──────────┐   ┌────────────┐   ┌───────────┐
│  Read   │ → │  Parse   │ → │  Validate  │ → │ Temporal  │
│         │   │          │   │            │   │  Lower    │
└─────────┘   └──────────┘   └────────────┘   └───────────┘
   │              │              │                  │
   │              │              │                  ▼
   ▼              ▼              ▼          ┌─────────────┐
BootstrapResult { stages: [Read, Parse, Validate, TemporalLower, FixtureParity] }

Rust Reference Pipeline       MIRR-CORE Pipeline (future)
────────────────────────      ──────────────────────────────
src/lexer/                    compiler_mirr/lexer.mirr
src/parser/                   compiler_mirr/parser.mirr
src/validation/               compiler_mirr/semantic.mirr
src/temporal/                 compiler_mirr/temporal_lowering.mirr

Both produce output conforming to:
  docs/schemas/mirr_ast.schema.json
  docs/schemas/mirr_temporal_netlist.schema.json

**Emitter note:** currently the last stage serializes the `TemporalNetlist`
into JSON (and optionally Graphviz/DOT) for testing, visualization, and
external tooling.  A future extension could add a hardware‑generation pass
(Verilog, VHDL, etc.) that consumes the same netlist structure.
```

### Key Concepts
- **Self-hosting:** MIRR compiler modules are *intended* to be written in MIRR and executed by the Rust bootstrap runner.  At the moment only the lexer is fully ported and exercised; the parser/semantic/temporal/emitter modules are scaffolds waiting for future porting (see `compiler_mirr/` and `docs/roadmap.md`).
- **Determinism:** All outputs are reproducible and contract-checked.
- **Safety:** No heap allocation, bounded loops, explicit error handling.

---

## How to Extend MIRR (Developer Guide)

### Where to Make Changes
- **New syntax/token behavior:** `src/lexer/*`, `src/parser/*`, tests in `tests/*`
- **AST/data model updates:** `src/ast/*` + fixtures in `tests/fixtures/*`
- **Semantic rules:** `src/validation/semantic.rs` + validation tests
- **Temporal lowering / netlist emission:** `src/temporal/*` + temporal tests
- **Self-hosting pipeline behavior:** `src/bootstrap_runner.rs` + parity/schema tests

### Repository Structure

```text
.
├── Cargo.toml
├── README.md
├── run-mirr.ps1
├── src/
│   ├── main.rs                    # CLI entrypoint
│   ├── lib.rs                     # Public API / module exports
│   ├── error.rs                   # Shared error types
│   ├── bootstrap_runner.rs        # Self-host bootstrap pipeline
│   ├── bin/                       # helper binaries compiled via `cargo run --bin`
│   │   ├── mirr-simplify.rs       # standalone logic simplifier tool
│   │   └── generate_mirr_stress.rs # Rust stress–test generator (replaces Python prototype)
│   ├── ast/
│   │   ├── types.rs
│   │   ├── expr.rs
│   │   └── program.rs
│   ├── lexer/
│   │   └── tokenizer.rs
│   ├── parser/
│   │   ├── expr_parser.rs
│   │   └── module_parser.rs
│   ├── validation/
│   │   └── semantic.rs
│   └── temporal/
│       ├── compiler.rs
│       ├── emit.rs
│       └── low_level_ir.rs
├── tests/
│   ├── *_tests.rs                # Unit/integration suites
│   └── fixtures/
│       ├── ast/
│       ├── parse/
│       ├── semantic/
│       ├── tokens/
│       └── netlist/
├── compiler_mirr/
│   ├── lexer.mirr                # Partially ported (signal-level primitives)
│   ├── lexer.mirr.bak            # Original full implementation (backup)
│   ├── parser.mirr               # Minimal placeholder / incremental port
│   ├── parser.mirr.bak
│   ├── semantic.mirr
│   ├── semantic.mirr.bak
│   ├── temporal_lowering.mirr
│   ├── temporal_lowering.mirr.bak
│   ├── emitter.mirr
│   ├── emitter.mirr.bak
│   └── PORTING_STEPS.md          # Porting checklist and status
├── stdlib/
│   └── mirr_core/
│       ├── diagnostics.mirr
│       ├── fixed_map.mirr
│       ├── str.mirr
│       └── token_buffer.mirr
├── examples/
│   └── neonatal_respirator.mirr
├── mcp_server/                   # TypeScript/Node project implementing the MCP stdio server
├── docs/
│   ├── INDEX.md                  # Canonical docs map
│   ├── mirr_spec.md
│   └── ...
├── scripts/
│   ├── research/                  # experimental benchmark and reproducibility scripts
│   └── generate_mirr_stress.py    # legacy Python stress generator (deprecated; see src/bin)
└── artifacts/
    └── research/
```

---

## Key Documentation & Specs

- **Start here:** `docs/INDEX.md` (canonical docs map)
- **Language/core spec:** `docs/mirr_spec.md`, `docs/self_hosting_core_spec.md`
- **IR contract:** `docs/self_hosting_ir_contract.md`
- **Roadmap:** `docs/roadmap.md`
- **Interpreter runtime:** `docs/interpreter/runtime_spec.md`
- **Architecture decisions:** `docs/decisions/ADR-002-interpreter-architecture.md`
- **Testing & benchmarks:** `docs/testing/fixture_matrix.md`, `docs/benchmarks/benchmark_protocol.md`
- **Research scripts:** `scripts/research/run_experiments.py`

---

## Repo Hygiene & Contribution Rules

- **Never commit build outputs** (like `target/`).
- **Keep generated installers/logs out of Git.**
- **Preserve MIRR source files** (`*.mirr`) used by compiler/self-hosting workflows.
- **Update `docs/INDEX.md`** with every new or changed document.

---

## Need Help or Want to Contribute?

Open an issue or PR! See the docs and code comments for guidance. All contributions should follow the documentation and testing standards described above.
