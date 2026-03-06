# Copilot & AI Agent Instructions for MIRR Project

## Project Overview
- MIRR is a self-hosting, safety-critical DSL and toolchain for temporal logic and hardware compilation.
- The codebase is split into Rust (reference/host) and MIRR-in-MIRR (self-hosting) modules.
- Key flows: parse MIRR → validate → lower to temporal IR/netlist → emit JSON/DOT → (future) logic simplification.

## Architecture & Key Components
- `src/` — Rust reference implementation: lexer, parser, AST, validation, temporal lowering, error handling, CLI.
- `compiler_mirr/` — Self-hosting MIRR compiler modules (written in MIRR).
- `tests/` — Unit/integration tests, fixtures for AST, netlist, semantic, and temporal logic.
- `scripts/research/` — Python scripts for research-grade determinism, throughput, and failure-mode experiments.
- `docs/` — Specs, architecture, roadmap, and research methodology.
- `artifacts/research/` — Output from experiments (CSV, Markdown, JSON).

## Developer Workflows
- **Build:** `cargo build`
- **Run example:** `cargo run -- examples/neonatal_respirator.mirr`
- **Compile temporal guards:** `cargo run -- --compile examples/neonatal_respirator.mirr`
- **Emit IR JSON:** `cargo run -- --compile --json examples/neonatal_respirator.mirr`
- **Run all tests:** `cargo test`
- **Run research experiments:** `python scripts/research/run_experiments.py`
- **Self-hosting pipeline:** `cargo run -- --selfhost-compile <file.mirr>`

## Project-Specific Patterns & Conventions
- All MIRR source files use `.mirr` extension.
- Temporal logic is modeled explicitly (e.g., `when signal for N cycles`).
- IR/netlist output conforms to JSON schemas in `docs/schemas/`.
- Self-hosting modules mirror Rust pipeline: see `compiler_mirr/` vs `src/`.
- Tests use fixtures in `tests/fixtures/` for reproducibility.
- No heap allocation or unbounded loops in safety-critical paths.
- All new docs must be indexed in `docs/INDEX.md`.

## Integration & Extensibility
- Logic simplification (Phase 3+) is in `src/simplify.rs` and related CLI/tests.
- Research/benchmarking is automated via Python scripts and outputs to `artifacts/research/`.
- New language features require updates to both Rust and MIRR-in-MIRR modules.

## Key Files & Directories
- `src/main.rs`, `src/lib.rs` — Entrypoints and exports
- `src/temporal/low_level_ir.rs` — Temporal IR/netlist
- `src/simplify.rs` — Logic simplification (Phase 3+)
- `compiler_mirr/*.mirr` — Self-hosting compiler modules
- `tests/`, `tests/fixtures/` — Test suites and golden files
- `scripts/research/run_experiments.py` — Research automation
- `docs/` — Specs, architecture, and methodology

## Example: Adding a New Temporal Guard
1. Update parser/AST in `src/parser/`, `src/ast/`.
2. Add/modify semantic checks in `src/validation/semantic.rs`.
3. Extend temporal lowering in `src/temporal/`.
4. Add/adjust tests in `tests/` and fixtures.
5. Update docs/specs as needed.

---

For more, see `README.md`, `docs/INDEX.md`, and code comments. All AI agents should follow these conventions for maximum productivity and maintainability.
