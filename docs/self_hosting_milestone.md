# MIRR Self-Hosting Milestone — v1

> **Status:** ACHIEVED  
> **Date:** 2026-03-01  
> **IR Contract Version:** 1.0  
> **Reference:** `docs/self_hosting_core_spec.md`, `docs/self_hosting_ir_contract.md`

---

## 1  What "Self-Hosting" Means for MIRR

MIRR self-hosting is the ability for the MIRR compiler pipeline—lexer, parser, semantic validator, temporal guard lowering, and emitter—to be expressed in **MIRR-CORE** (the self-hostable subset of MIRR) and executed by the Rust bootstrap runner to produce output **byte-stable or semantically equivalent** to the reference Rust implementation.

This milestone covers **stage-1 self-hosting** (hosted execution): the Rust runtime loads and executes MIRR-in-MIRR compiler modules. Full native self-hosting (MIRR compiling itself without a Rust host) is a future goal.

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
| 8 | Emitter in MIRR-CORE | `compiler_mirr/emitter.mirr` | ✅ |
| 9 | Bootstrap runner | `src/bootstrap_runner/` + `--selfhost-compile` CLI flag | ✅ |
| 10 | Differential testing + CI gate | `tests/self_hosting_parity_tests.rs` (21 tests) | ✅ |
| 11 | Milestone freeze | `docs/self_hosting_milestone.md` (this document) | ✅ |

---

## 3  Architecture Overview

```text
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
│  src/emit/                     compiler_mirr/emitter.mirr        │
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

Contains **21 rigorous tests** verifying end-to-end self-hosted compiler stage stability and golden fixture matching.

### Expanded Self-Hosting Test Suites

| Suite | File | What It Verifies |
|-------|------|------------------|
| Extended Core | `tests/bootstrap_runner_extended_core_tests.rs` | Deeper core component execution |
| Extended Stress | `tests/bootstrap_runner_extended_stress_tests.rs` | Scaling boundaries and failure recovery |
| MIRR Stages | `tests/bootstrap_runner_mirr_stages_tests.rs` | Direct MIRR-to-MIRR pipeline execution |
| IR Schema | `tests/self_hosting_ir_schema_tests.rs` | JSON schema conformance |
| Bootstrap Parity | `tests/bootstrap_parity_tests.rs` | Rust-to-MIRR compiler byte-stability |

---

## 7  CLI Interface

```bash
# Run the self-hosting bootstrap pipeline
cargo run --bin mirr-compile -- --selfhost-compile examples/neonatal_respirator.mirr

# Also emit the netlist JSON to stdout
cargo run --bin mirr-compile -- --selfhost-compile --selfhost-compile-json examples/neonatal_respirator.mirr
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

### Compiler-in-MIRR Modules

- [x] `compiler_mirr/lexer.mirr` — tokenizer logic ported to MIRR-CORE
- [x] `compiler_mirr/parser.mirr` — parser logic ported to MIRR-CORE
- [x] `compiler_mirr/semantic.mirr` — semantic validation ported to MIRR-CORE
- [x] `compiler_mirr/temporal_lowering.mirr` — temporal guard lowering ported to MIRR-CORE
- [x] `compiler_mirr/emitter.mirr` — netlist emission ported to MIRR-CORE

### Standard Library

- [x] `stdlib/mirr_core/str.mirr` — string slice operations
- [x] `stdlib/mirr_core/token_buffer.mirr` — bounded token buffer
- [x] `stdlib/mirr_core/fixed_map.mirr` — fixed-capacity hash map
- [x] `stdlib/mirr_core/diagnostics.mirr` — diagnostic builder

### Infrastructure

- [x] `src/bootstrap_runner/mod.rs` & `types.rs` — 5-stage pipeline orchestrator
- [x] `--selfhost-compile` CLI flag wired in `src/main.rs`
- [x] Extensive expanded CI test gates (21 base tests + extended suites)

---

## 9  Rust Reference Fallback

The Rust compiler remains the **reference implementation** and safety fallback.
Any discrepancy between the Rust pipeline and the MIRR-CORE pipeline is a bug
in the MIRR-CORE port, not the other way around. The golden fixtures in
`tests/fixtures/` are generated by the Rust pipeline and are authoritative.