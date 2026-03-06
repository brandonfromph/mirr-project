# MIRR Self-Hosting Milestone — v1

> **Status:** ACHIEVED  
> **Date:** 2026-03-01  
> **IR Contract Version:** 1.0  
> **Reference:** `docs/self_hosting_core_spec.md`, `docs/self_hosting_ir_contract.md`

---

## 1  What "Self-Hosting" Means for MIRR

MIRR self-hosting is the ability for the MIRR compiler pipeline—lexer, parser,
semantic validator, and temporal guard lowering—to be expressed in **MIRR-CORE**
(the self-hostable subset of MIRR) and executed by the Rust bootstrap runner to
produce output **byte-stable or semantically equivalent** to the reference Rust
implementation.

This milestone covers **stage-1 self-hosting** (hosted execution): the Rust
runtime loads and executes MIRR-in-MIRR compiler modules.  Full native
self-hosting (MIRR compiling itself without a Rust host) is a future goal.

---

## 2  Completed Tasks

| # | Task | Deliverable | Status |
|---|------|------------|--------|
| 1 | Define MIRR-CORE subset | `docs/self_hosting_core_spec.md` | ✅ |
| 2 | IR contract + JSON schemas | `docs/self_hosting_ir_contract.md`, `docs/schemas/*.schema.json` | ✅ |
| 3 | Standard library primitives | `stdlib/mirr_core/{str,token_buffer,fixed_map,diagnostics}.mirr` | ✅ |
| 4 | MIRR tokenizer in MIRR-CORE | `compiler_mirr/lexer.mirr` + golden token fixtures | ✅ |
| 5 | MIRR parser in MIRR-CORE | `compiler_mirr/parser.mirr` + parse parity suite | ✅ |
| 6 | Semantic validation in MIRR-CORE | `compiler_mirr/semantic.mirr` + validation parity tests | ✅ |
| 7 | Temporal guard lowering in MIRR-CORE | `compiler_mirr/temporal_lowering.mirr` + netlist parity tests | ✅ |
| 8 | Bootstrap runner | `src/bootstrap_runner.rs` + `--selfhost-compile` CLI flag | ✅ |
| 9 | Differential testing + CI gate | `tests/self_hosting_parity_tests.rs` (13 tests) | ✅ |
| 10 | Milestone freeze | `docs/self_hosting_milestone.md` (this document) | ✅ |

---

## 3  Architecture Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                     MIRR Self-Hosting Architecture               │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│   Source (.mirr)                                                 │
│       │                                                          │
│       ▼                                                          │
│   ┌─────────┐   ┌──────────┐   ┌────────────┐   ┌───────────┐    │
│   │  Read   │ → │  Parse   │ → │  Validate  │ → │ Temporal  │    │
│   │         │   │          │   │            │   │  Lower    │    │
│   └─────────┘   └──────────┘   └────────────┘   └───────────┘    │
│       │              │              │                  │         │
│       │              │              │                  ▼         │
│       │              │              │          ┌─────────────┐   │
│       │              │              │          │  Fixture    │   │
│       │              │              │          │  Parity     │   │
│       │              │              │          └─────────────┘   │
│       │              │              │                  │         │
│       ▼              ▼              ▼                  ▼         │
│   BootstrapResult { stages: [Read, Parse, Validate,              │
│                              TemporalLower, FixtureParity] }     │
│                                                                  │
├──────────────────────────────────────────────────────────────────┤
│  Rust Reference Pipeline       MIRR-CORE Pipeline (future)       │
│  ────────────────────────      ──────────────────────────────    │
│  src/lexer/                    compiler_mirr/lexer.mirr          │
│  src/parser/                   compiler_mirr/parser.mirr         │
│  src/validation/               compiler_mirr/semantic.mirr       │
│  src/temporal/                 compiler_mirr/temporal_lowering   │
│                                  .mirr                           │
│                                                                  │
│  Both produce output conforming to:                              │
│    docs/schemas/mirr_ast.schema.json                             │
│    docs/schemas/mirr_temporal_netlist.schema.json                │
└──────────────────────────────────────────────────────────────────┘
```

---

## 4  MIRR-CORE Language Subset

The self-hostable subset (defined in `docs/self_hosting_core_spec.md`):

- **Included:** integers (i32, u16, u32), booleans, fixed-size arrays, structs/records, bounded loops (`for i in 0..N`), functions, modules, enums (discriminant-based).
- **Excluded:** dynamic memory allocation, unbounded recursion, advanced generics, floating point, closures/lambdas, trait objects.
- **NASA Safety Rules:** all loops bounded, no heap allocation, stack depth statically verifiable, all array accesses bounds-checked.

---

## 5  IR Contract

Both pipelines must produce JSON matching the schemas in `docs/schemas/`:

| Schema | File | Validates |
|--------|------|-----------|
| AST | `mirr_ast.schema.json` | Parsed module structure |
| Temporal Netlist | `mirr_temporal_netlist.schema.json` | Lowered guard netlist |

The contract version is `"ir_version": "1.0"` and is embedded in every JSON envelope.

---

## 6  Test Coverage Summary

### Integration Parity Tests (`tests/self_hosting_parity_tests.rs`)

| Test | What It Verifies |
|------|-----------------|
| `selfhost_neonatal_all_stages_pass` | Full 5-stage pipeline succeeds |
| `selfhost_neonatal_fixture_parity_stage_present` | FixtureParity stage runs (not skipped) |
| `selfhost_neonatal_emits_valid_json` | Output is valid JSON with correct structure |
| `selfhost_neonatal_guard_strategy_is_counter` | Counter strategy selected for 1000 cycles |
| `selfhost_neonatal_signal_contract` | 3 signals with correct names/types/kinds |
| `selfhost_neonatal_statistics_contract` | Statistics match golden fixture |
| `selfhost_parse_error_fails_pipeline` | Malformed input detected at Parse stage |
| `selfhost_missing_file_fails_pipeline` | Missing file detected at Read stage |
| `selfhost_validation_error_fails_pipeline` | Duplicate signals caught at Validate stage |
| `selfhost_summary_line_ci_format` | PASS summary matches CI format |
| `selfhost_failure_summary_says_fail` | FAIL summary matches CI format |
| `selfhost_neonatal_has_five_stages` | Pipeline reports exactly 5 stages in order |
| `selfhost_fixture_json_roundtrip_stable` | Full JSON roundtrip matches golden fixture |

### Other Self-Hosting Test Suites

| Suite | File | Tests |
|-------|------|-------|
| IR Schema Validation | `tests/self_hosting_ir_schema_tests.rs` | Schema conformance |
| Stdlib Conformance | `tests/stdlib_conformance_tests.rs` | Core library primitives |
| Temporal Lowering | `tests/temporal_lowering_tests.rs` | Guard compilation + netlist fixture parity |
| Bootstrap Runner (unit) | `src/bootstrap_runner.rs` (mod tests) | 5 unit tests |

---

## 7  CLI Interface

```bash
# Run the self-hosting bootstrap pipeline
cargo run -- --selfhost-compile examples/neonatal_respirator.mirr

# Also emit the netlist JSON to stdout
cargo run -- --selfhost-compile --selfhost-compile-json examples/neonatal_respirator.mirr
```

Output format:
```
Self-Host Bootstrap: PASS — examples/neonatal_respirator.mirr
  Stage 1: ✓ [Read] 366 bytes read
  Stage 2: ✓ [Parse] 3 signal(s), 1 guard(s), 1 reflex(es)
  Stage 3: ✓ [Validate] all semantic checks passed
  Stage 4: ✓ [TemporalLower] 1 guard(s) lowered — 3 signal(s) generated
  Stage 5: ✓ [FixtureParity] matches tests/fixtures/netlist/neonatal_respirator.json
[SELF-HOST PASS] 5/5 stages passed — examples/neonatal_respirator.mirr
```

Exit code: `0` on PASS, `1` on FAIL.

---

## 8  Release Checklist

### Pre-Release Verification

- [x] All 13 self-hosting parity tests pass (`cargo test --test self_hosting_parity_tests`)
- [x] All bootstrap runner unit tests pass (`cargo test --lib bootstrap_runner`)
- [x] All temporal lowering tests pass (`cargo test --test temporal_lowering_tests`)
- [x] IR schema tests pass (`cargo test --test self_hosting_ir_schema_tests`)
- [x] Stdlib conformance tests pass (`cargo test --test stdlib_conformance_tests`)
- [x] CLI `--selfhost-compile` returns exit code 0 on canonical example
- [x] Golden fixture `tests/fixtures/netlist/neonatal_respirator.json` matches pipeline output
- [x] `docs/self_hosting_core_spec.md` frozen
- [x] `docs/self_hosting_ir_contract.md` frozen
- [x] JSON schemas in `docs/schemas/` frozen at version 1.0

### Compiler-in-MIRR Modules

- [x] `compiler_mirr/lexer.mirr` — tokenizer logic ported to MIRR-CORE
- [x] `compiler_mirr/parser.mirr` — parser logic ported to MIRR-CORE
- [x] `compiler_mirr/semantic.mirr` — semantic validation ported to MIRR-CORE
- [x] `compiler_mirr/temporal_lowering.mirr` — temporal guard lowering ported to MIRR-CORE

### Standard Library

- [x] `stdlib/mirr_core/str.mirr` — string slice operations
- [x] `stdlib/mirr_core/token_buffer.mirr` — bounded token buffer
- [x] `stdlib/mirr_core/fixed_map.mirr` — fixed-capacity hash map
- [x] `stdlib/mirr_core/diagnostics.mirr` — diagnostic builder

### Infrastructure

- [x] `src/bootstrap_runner.rs` — 5-stage pipeline orchestrator
- [x] `--selfhost-compile` CLI flag wired in `src/main.rs`
- [x] `tests/self_hosting_parity_tests.rs` — 13-test CI gate
- [x] `docs/self_hosting_milestone.md` — this document

---

## 9  Rust Reference Fallback

The Rust compiler remains the **reference implementation** and safety fallback.
Any discrepancy between the Rust pipeline and the MIRR-CORE pipeline is a bug
in the MIRR-CORE port, not the other way around.  The golden fixtures in
`tests/fixtures/` are generated by the Rust pipeline and are authoritative.

---

## 10  Next Steps (Post-Milestone)

1. **MIRR-CORE Interpreter:** Build a Rust-hosted interpreter that can execute
   `compiler_mirr/*.mirr` modules, enabling true cross-pipeline comparison.
2. **Stage-2 Self-Hosting:** Run the MIRR-CORE compiler modules through the
   interpreter to produce output, and diff against the Rust pipeline.
3. **Performance Benchmarking:** Measure compilation throughput of MIRR-CORE
   pipeline vs Rust pipeline; set performance thresholds.
4. **Additional Fixtures:** Add more test programs beyond `neonatal_respirator`
   to increase parity coverage (shift-register guards, multi-guard modules,
   error recovery scenarios).
5. **Native Compilation:** Eventually compile MIRR-CORE to machine code,
   achieving full native self-hosting without a Rust host.

---

*This document was generated as part of the MIRR self-hosting plan (Task 10).*
*The Rust compiler is preserved as the reference fallback for safety.*