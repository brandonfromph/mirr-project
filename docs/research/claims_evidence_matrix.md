# MIRR Claims ↔ Evidence Matrix (DAC Readiness)

> **Status:** Draft  
> **Version:** 0.1  
> **Date:** 2026-03-01  
> **Purpose:** Prevent over-claiming by mapping every claim to concrete, reproducible evidence.

---

## 1) Why this matrix exists

Your current risk is not “bad idea,” it is **claim-evidence mismatch**.

This matrix classifies each statement as one of:

1. **Implemented + measured in this repo**
2. **Supported by external literature (not reproduced here)**
3. **Proposed future work (no direct evidence yet)**

Only class (1) should be presented as MIRR experimental results.

---

## 2) Claims table

| ID | Claim statement | Class | Evidence in repo | Repro command(s) | DAC-safe wording |
|---|---|---|---|---|---|
| C1 | MIRR source can be parsed into structured AST | (1) Implemented+measured | `src/parser/*`, parser tests | `cargo test --test module_tests` | “Our implementation parses MIRR modules and validates syntax via test suite.” |
| C2 | MIRR semantic errors are caught before lowering | (1) Implemented+measured | `src/validation/*`, validation tests | `cargo test --test validation_tests` | “Semantic validation rejects invalid signal/guard references.” |
| C3 | Temporal guards lower to shift-register or counter strategy based on cycle count | (1) Implemented+measured | `src/temporal/compiler.rs`, `tests/temporal_lowering_tests.rs` | `cargo test --test temporal_lowering_tests` | “We implement adaptive lowering with threshold-based strategy selection.” |
| C4 | Netlist JSON follows explicit contract/schema | (1) Implemented+measured | `docs/self_hosting_ir_contract.md`, `docs/schemas/*`, `tests/self_hosting_ir_schema_tests.rs` | `cargo test --test self_hosting_ir_schema_tests` | “Generated netlists are validated against versioned IR schemas.” |
| C5 | Stage-1 self-host bootstrap pipeline executes and reports pass/fail by stage | (1) Implemented+measured | `src/bootstrap_runner.rs`, `tests/self_hosting_parity_tests.rs` | `cargo run -- --selfhost-compile examples/neonatal_respirator.mirr` | “A staged bootstrap verifier is implemented and test-gated.” |
| C6 | Compiler output is deterministic for fixed input | (1) Implemented+measured (experiment) | `scripts/research/run_experiments.py` | `py -3 scripts/research/run_experiments.py --skip-build` (Windows), `python3 scripts/research/run_experiments.py --skip-build` (POSIX) | “Determinism is evaluated via repeated-output hashing.” |
| C7 | Cement2 enables 377MHz timing closure | (2) External literature | Cited in manuscript | N/A (not reproduced) | “Prior work reports up to 377MHz; not re-implemented in MIRR yet.” |
| C8 | SmaRTLy achieves ~47% area reduction | (2) External literature | Cited in manuscript | N/A (not reproduced) | “Prior work reports area reduction; MIRR integration remains future work.” |
| C9 | SCC-based width inference with Rocq proofs | (1) Implemented+measured | `src/width/*`, `proofs/*.v`, width tests | `cargo test --test width_tests` | “MIRR implements SCC-based width inference with 816 lines of Rocq proofs, inspired by FIRWINE.” |
| C10 | MIRR compiles to SystemVerilog RTL | (1) Implemented+measured | `src/emit/verilog.rs`, verilog emission tests | `cargo test --test emit_verilog_tests` | “SystemVerilog RTL emission is implemented and tested.” |
| C11 | MIRR currently performs DPR on FPGA hardware | (3) Future work | Not implemented in repo | N/A | “DPR integration is future hardware validation work.” |
| C12 | MIRR currently guarantees fail-safe clinical behavior | (3) Future work | Not clinically validated | N/A | “Safety guarantees are a target objective pending hardware and clinical validation.” |

---

## 3) Red-flag wording to avoid in current draft

Avoid these until backed by your own measurements:

- “guaranteed fail-safe performance”
- “nanosecond-level precision” (unless timed on actual hardware)
- “MIRR achieves 377 MHz / 47% area reduction”

Use instead:

- “design goal,” “proposed architecture,” “future integration target”
- “inspired by Cement2/SmaRTLy/FIRWINE”
- “currently validated at compiler/IR level with deterministic test artifacts”

---

## 4) Minimum reproducible artifact set (for reviewers)

1. `cargo test --test self_hosting_parity_tests`
2. `cargo test --test self_hosting_ir_schema_tests`
3. `cargo run -- --selfhost-compile examples/neonatal_respirator.mirr`
4. `py -3 scripts/research/run_experiments.py --skip-build` (Windows) or `python3 scripts/research/run_experiments.py --skip-build` (POSIX)
5. Archive `artifacts/research/*` as supplementary material.

---

## 5) Paper positioning recommendation

Best current positioning:

> “MIRR is an implemented reflex-oriented compiler prototype with deterministic
> temporal lowering and stage-1 self-hosting verification. We provide
> reproducible experiments at the compiler/netlist-contract level and identify
> hardware-level autonomic execution as future validation work.”

This is truthful, strong, and publishable as a systems/toolchain paper while
you continue toward FPGA-level validation.
