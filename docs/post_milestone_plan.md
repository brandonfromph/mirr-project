# MIRR Post-Milestone Plan — Self-Hosting v1 → v2

> **Status:** Draft  
> **Date:** 2026-03-01  
> **Prerequisite:** Self-Hosting Milestone v1 achieved (see `self_hosting_milestone.md`)  
> **Governance:** Per ADR-001 (`decisions/ADR-001-doc-governance.md`)

---

## Overview

With stage-1 self-hosting achieved, the project enters five parallel/sequential
work streams. Each stream follows a **documentation-first** discipline: the
spec/ADR/test-plan is written and reviewed *before* implementation begins.

```
┌──────────────────────────────────────────────────────────────────┐
│                   Post-Milestone Work Streams                    │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│   Stream 1: MIRR-CORE Interpreter          (Weeks 1–2)           │
│       │                                                          │
│       ▼                                                          │
│   Stream 2: Stage-2 Self-Hosting Parity    (Weeks 2–3)           │
│       │                                                          │
│       ├──→ Stream 3: Performance Benchmarks (Week 3, parallel)   │
│       │                                                          │
│       ▼                                                          │
│   Stream 4: Additional Fixtures            (Week 4)              │
│                                                                  │
│   Stream 5: Native Compilation Research    (ongoing, low vel.)   │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

## Stream 1: MIRR-CORE Interpreter

**Goal:** Build a Rust-hosted interpreter that can execute
`compiler_mirr/*.mirr` modules, enabling true cross-pipeline comparison.

### Prerequisites
- Self-hosting milestone v1 frozen ✅
- MIRR-CORE spec frozen at v1 ✅
- IR contract v1.0 frozen ✅

### Phase 1a — Documentation (before any code)

| # | Deliverable | Template | Status |
|---|------------|----------|--------|
| 1 | ADR-002: Interpreter architecture model | `templates/adr_template.md` | 🟡 Draft |
| 2 | Design spec: Interpreter runtime | `templates/design_spec_template.md` | 🟡 Draft |
| 3 | Test plan: Interpreter conformance | `templates/test_plan_template.md` | 🟡 Draft |

**Key design decisions to document in ADR-002:**
- Tree-walking vs bytecode interpreter (recommend tree-walking for v1 simplicity).
- Execution model: single-threaded, bounded stack, no heap.
- Module loading: how `compiler_mirr/*.mirr` modules are discovered and linked.
- Stdlib binding: how `stdlib/mirr_core/*.mirr` primitives are provided (Rust
  intrinsics vs interpreted).
- Trace/debug output format for deterministic replay.
- Error model: how MIRR-CORE `Result` maps to interpreter errors.

**Key design decisions for runtime spec:**
- Call stack depth limit (NASA bounded-recursion rule → iterative only, but
  function call depth still needs a bound).
- Memory model: fixed-size value stack + named local slots.
- Type representation at runtime (tagged values vs static dispatch).
- I/O model: source text input via `&str` slices; JSON output via buffer.

### Phase 1b — Implementation

| # | Task | Module/File | Depends on |
|---|------|-------------|------------|
| 4 | Interpreter skeleton (module load + fn dispatch) | `src/interpreter/mod.rs` | ADR-002 accepted |
| 5 | Expression evaluator (all MIRR-CORE expr types) | `src/interpreter/eval.rs` | #4 |
| 6 | Statement executor (let, assign, if, for, loop, match, return) | `src/interpreter/exec.rs` | #5 |
| 7 | Stdlib intrinsic bindings (str, token_buffer, fixed_map, diagnostics) | `src/interpreter/stdlib.rs` | #6 |
| 8 | CLI flag `--interpret <module.mirr>` | `src/main.rs` | #7 |
| 9 | Smoke tests: interpret a trivial MIRR-CORE function | `tests/interpreter_tests.rs` | #7 |
| 10 | Conformance tests per stdlib primitive | `tests/interpreter_stdlib_tests.rs` | #7 |

### Acceptance Criteria
- [ ] ADR-002 accepted.
- [ ] Runtime spec reviewed and marked Active.
- [ ] Interpreter can execute `compiler_mirr/lexer.mirr` tokenize function on
      a trivial input and produce a token buffer matching Rust output.
- [ ] All conformance tests pass.
- [ ] `docs/INDEX.md` updated.

---

## Stream 2: Stage-2 Self-Hosting Parity

**Goal:** Run the MIRR-CORE compiler modules through the interpreter to produce
output, and diff against the Rust pipeline.

### Prerequisites
- Stream 1 (interpreter) functional for all 4 compiler modules.

### Phase 2a — Documentation

| # | Deliverable | Template | Status |
|---|------------|----------|--------|
| 1 | Design spec: Stage-2 parity pipeline | `templates/design_spec_template.md` | 🟡 Draft |
| 2 | Parity policy addendum: byte-equal vs semantic-equal escalation | (append to IR contract or new doc) | 🟡 Draft |
| 3 | Test plan: Stage-2 parity | `templates/test_plan_template.md` | 🟡 Draft |

**Key decisions to document:**
- Parity comparison method: byte-stable JSON diff vs structural comparison.
- Failure artifact retention: what gets saved on CI failure (inputs, both
  outputs, diff, trace log).
- Stage-2 CI job design: runs after stage-1 parity passes.

### Phase 2b — Implementation

| # | Task | Module/File | Depends on |
|---|------|-------------|------------|
| 4 | Wire stage-2 path: interpreter executes all 4 `compiler_mirr/*.mirr` modules in sequence | `src/bootstrap_runner.rs` or new orchestrator | Stream 1 complete |
| 5 | Capture interpreter pipeline output as JSON | `src/interpreter/...` | #4 |
| 6 | Diff interpreter output vs Rust reference output | `tests/stage2_parity_tests.rs` | #5 |
| 7 | Failure artifact dump (both JSONs + diff + trace) | CI config / test harness | #6 |
| 8 | CLI flag `--stage2-parity` | `src/main.rs` | #6 |

### Acceptance Criteria
- [ ] Design spec and parity policy reviewed.
- [ ] Interpreter pipeline produces AST + netlist JSON for `neonatal_respirator.mirr`.
- [ ] Diff against Rust reference is zero (byte-stable) or documented exceptions.
- [ ] CI job runs and retains failure artifacts.
- [ ] `docs/INDEX.md` updated.

---

## Stream 3: Performance Benchmarking

**Goal:** Measure compilation throughput of MIRR-CORE pipeline vs Rust pipeline;
set performance thresholds.

### Prerequisites
- Stream 2 (stage-2 parity) passing — both pipelines produce correct output.

### Phase 3a — Documentation

| # | Deliverable | Template | Status |
|---|------------|----------|--------|
| 1 | Benchmark protocol | `docs/benchmarks/benchmark_protocol.md` | 🟡 Draft |
| 2 | Baseline report | `docs/benchmarks/baseline_YYYYMMDD.md` | 🟡 Draft |

**Key decisions to document:**
- Hardware profile (reference machine specs for reproducibility).
- Warmup iterations, sample size, statistical method (mean ± stddev, p95).
- KPIs: tokens/sec, parse time, total pipeline time, memory peak.
- Regression thresholds: e.g., >10% slowdown = CI warning, >25% = CI failure.
- Acceptable interpreter overhead vs Rust native (expected to be 10–100×;
  document the target ceiling).

### Phase 3b — Implementation

| # | Task | Module/File | Depends on |
|---|------|-------------|------------|
| 3 | Benchmark harness (Rust criterion or custom) | `benches/pipeline_bench.rs` | Protocol doc |
| 4 | Benchmark: Rust pipeline on canonical fixtures | #3 | |
| 5 | Benchmark: Interpreter pipeline on canonical fixtures | #3 + Stream 1 | |
| 6 | Record baseline numbers | `docs/benchmarks/baseline_YYYYMMDD.md` | #4, #5 |
| 7 | CI guardrail (threshold check) | CI config | #6 |

### Acceptance Criteria
- [ ] Protocol doc reviewed and marked Active.
- [ ] Baseline recorded with documented hardware profile.
- [ ] CI enforces regression thresholds.
- [ ] `docs/INDEX.md` updated.

---

## Stream 4: Additional Fixtures

**Goal:** Add more test programs beyond `neonatal_respirator` to increase parity
coverage.

### Phase 4a — Documentation

| # | Deliverable | Template | Status |
|---|------------|----------|--------|
| 1 | Fixture taxonomy & coverage matrix | `docs/testing/fixture_matrix.md` | 🟡 Draft |

**Key decisions to document:**
- Fixture categories: normal, edge-case, adversarial, error-recovery.
- Coverage mapping: which MIRR-CORE features and IR paths does each fixture exercise?
- Priority ranking of missing coverage areas.

### Phase 4b — Implementation

| # | Task | Example file | Depends on |
|---|------|-------------|------------|
| 2 | Shift-register guard fixture (short delay, ≤16 cycles) | `examples/shift_register_guard.mirr` | Taxonomy doc |
| 3 | Multi-guard module fixture (2+ guards, 2+ reflexes) | `examples/multi_guard_monitor.mirr` | Taxonomy doc |
| 4 | Parser error recovery fixture (malformed input) | `examples/malformed_input.mirr` | Taxonomy doc |
| 5 | Validator error fixture (duplicate signals, undeclared refs) | `examples/validation_errors.mirr` | Taxonomy doc |
| 6 | Generate golden fixtures for each new example | `tests/fixtures/...` | #2–#5 |
| 7 | Add parity tests for each new fixture | `tests/self_hosting_parity_tests.rs` | #6 |
| 8 | Update fixture matrix with coverage confirmation | `docs/testing/fixture_matrix.md` | #7 |

### Acceptance Criteria
- [ ] Fixture matrix reviewed and marked Active.
- [ ] ≥4 new fixtures added (shift-register, multi-guard, parse-error, validation-error).
- [ ] All new parity tests pass.
- [ ] Coverage matrix shows no critical blind spots.
- [ ] `docs/INDEX.md` updated.

---

## Stream 5: Native Compilation Research

**Goal:** Evaluate paths to compiling MIRR-CORE to machine code, achieving full
native self-hosting without a Rust host.

### Phase 5a — Documentation (research track, no implementation commitment)

| # | Deliverable | Template | Status |
|---|------------|----------|--------|
| 1 | ADR-003: Native backend architecture options | `templates/adr_template.md` | 🟡 Draft |
| 2 | Risk register: determinism, verification, certifiability | standalone doc | 🟡 Draft |
| 3 | Milestone decomposition: PoC → limited backend → full bootstrap | standalone doc | 🟡 Draft |

**Key questions to answer before any implementation:**
- Target ISA(s): x86-64, ARM, RISC-V, FPGA bitstream?
- Backend strategy: custom codegen, LLVM, Cranelift, or MIRR-to-C transpiler?
- How to maintain determinism guarantees through native compilation?
- How to maintain parity testing (native output == Rust output == interpreter output)?
- Certification impact: does native compilation affect DO-178C / IEC 62304 posture?
- Go/no-go criteria: what must be true before committing engineering resources?

### Phase 5b — Implementation (future, gated by ADR-003 acceptance)

| # | Task | Notes |
|---|------|-------|
| 4 | PoC backend spike (feature-flagged, non-production) | Behind `--experimental-native` flag |
| 5 | Validate PoC output against interpreter and Rust reference | Triple-pipeline parity |
| 6 | Performance comparison: native vs interpreter vs Rust | Extend benchmark harness |

### Acceptance Criteria (Phase 5a only)
- [ ] ADR-003 documents ≥3 backend options with trade-off analysis.
- [ ] Risk register identifies top 5 risks with mitigations.
- [ ] Clear go/no-go criteria established before implementation.
- [ ] `docs/INDEX.md` updated.

---

## Cross-Cutting: Definition of Done

Every task across all streams must satisfy:

- [ ] Code compiles with `#![forbid(unsafe_code)]` and `#![deny(warnings)]`.
- [ ] `cargo fmt` and `cargo clippy` pass.
- [ ] Relevant tests added or updated.
- [ ] Documentation updated (spec, ADR, test plan, or runbook as applicable).
- [ ] `docs/INDEX.md` updated if new docs created.
- [ ] No regressions in existing test suites (`cargo test` all green).
- [ ] PR description includes rationale and links to relevant docs.

---

## Timeline Summary

| Week | Stream | Key Deliverable |
|------|--------|----------------|
| 1 | 1a | ADR-002 + runtime spec + test plan drafted |
| 1–2 | 1b | Interpreter skeleton → stdlib bindings |
| 2 | 2a | Stage-2 parity design spec |
| 2–3 | 2b | Stage-2 parity pipeline + CI job |
| 3 | 3a+3b | Benchmark protocol + baseline |
| 4 | 4a+4b | Fixture taxonomy + 4 new fixture sets |
| ongoing | 5a | Native compilation ADR + risk register |

---

*This plan is a living document (status: Draft). It will be updated as streams
complete and new information emerges. See `docs/INDEX.md` for document
governance rules.*