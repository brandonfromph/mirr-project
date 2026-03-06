# Benchmark Protocol — MIRR Rust Pipeline vs MIRR-CORE Interpreter

> **Status:** Draft  
> **Version:** 0.1  
> **Date:** 2026-03-01  
> **Owner:** MIRR Core Team  
> **Related docs:** `docs/post_milestone_plan.md`, `docs/interpreter/runtime_spec.md`

---

## 1) Purpose

This protocol defines a repeatable, deterministic method to benchmark the MIRR
compilation pipeline across execution modes:

1. **Rust reference pipeline** (current baseline)
2. **MIRR-CORE interpreter pipeline** (stage-2 self-hosting target)

The objective is to measure throughput, latency, and stability over time,
detect regressions early, and set explicit performance guardrails.

---

## 2) Benchmark Principles

1. **Correctness first:** performance measurements are valid only if output
   parity checks pass.
2. **Determinism:** run on fixed inputs, fixed configuration, and captured
   environment metadata.
3. **Comparability:** both pipelines are measured using the same fixture set and
   reporting format.
4. **Statistical discipline:** report median + p95 + standard deviation, not
   single-run numbers.
5. **Traceability:** every baseline is tied to commit hash and toolchain
   versions.

---

## 3) Scope

### In Scope
- End-to-end compile timing for canonical fixtures.
- Per-stage timing where available (Read / Parse / Validate / TemporalLower / Parity).
- Throughput metrics (`fixtures/sec`, `LOC/sec`, `tokens/sec` where feasible).
- Peak memory (optional in CI, required in scheduled benchmark runs).

### Out of Scope (v0.1)
- Native backend performance (future stream).
- Micro-benchmarking internal helper functions.
- GPU/accelerator experiments.

---

## 4) Benchmark Workloads

### 4.1 Required Fixture Set (v0.1)

| Fixture | Purpose | Category |
|--------|---------|----------|
| `examples/neonatal_respirator.mirr` | Canonical baseline | Normal |
| `examples/shift_register_guard.mirr` | Short temporal guard path | Edge |
| `examples/multi_guard_monitor.mirr` | Mixed strategy + multi-guard | Normal |

> If a required fixture does not yet exist, create it as part of Stream 4 and
> mark benchmark runs as **partial** until complete.

### 4.2 Optional Stress Set

Larger synthetic modules may be added later for trend analysis, but must be
reported separately from required v0.1 fixtures.

---

## 5) Metrics

For each pipeline and each fixture:

1. **E2E latency (ms):** wall-clock time from input read to final output.
2. **Stage latency (ms):** where instrumentation is available.
3. **Throughput:**
   - `fixtures/sec`
   - `source_lines/sec`
   - `tokens/sec` (if tokenizer counters available)
4. **Stability:** standard deviation and coefficient of variation.
5. **Peak RSS memory (MB):** required in scheduled/full benchmarks.

---

## 6) Test Environment Capture

Every benchmark report must include:

- Git commit hash
- Rust version (`rustc --version`)
- Cargo version (`cargo --version`)
- OS name/version
- CPU model and core/thread count
- Memory size
- Power mode (e.g., plugged-in/performance mode on laptops)

### Environment Control Rules

- Close heavy background applications where possible.
- Keep thermal conditions stable (avoid active thermal throttling).
- Run benchmarks at least twice; discard obvious outlier runs only with
  documented reason.

---

## 7) Execution Procedure

### 7.1 Pre-Checks

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test --test self_hosting_parity_tests
```

### 7.2 Build

```bash
cargo build --release
```

### 7.3 Benchmark Runs

Use release mode only.

```bash
# Rust reference pipeline (example)
cargo run --release -- examples/neonatal_respirator.mirr --emit-json

# MIRR-CORE interpreter pipeline (example; CLI may evolve)
cargo run --release -- --stage2-parity examples/neonatal_respirator.mirr
```

Each fixture should run:

- **Warmups:** 5 iterations
- **Measured iterations:** 30 iterations minimum
- **Report:** median, p95, mean, stddev

---

## 8) Regression Thresholds (Initial)

Thresholds are applied against the latest accepted baseline.

| Severity | Condition |
|----------|-----------|
| Warning | > 10% slowdown in median E2E on any required fixture |
| Failure | > 25% slowdown in median E2E on any required fixture |
| Failure | > 30% increase in p95 latency on any required fixture |

### Interpreter-specific note

Interpreter is expected to be slower than Rust native. Guardrails are primarily
for **regression over interpreter's own baseline**, not Rust-equivalence.

---

## 9) Reporting Format

Benchmark outputs should be stored in:

`docs/benchmarks/baseline_YYYYMMDD.md`

Recommended section structure:

1. Environment metadata
2. Fixture list
3. Rust pipeline results table
4. Interpreter pipeline results table
5. Delta analysis
6. Regression verdict
7. Follow-up actions

---

## 10) CI Integration

Two benchmark modes:

1. **PR quick-check (lightweight):**
   - 1 fixture (`neonatal_respirator`)
   - fewer iterations (e.g., 5 measured)
   - warning-only signal

2. **Scheduled full benchmark (nightly/weekly):**
   - full required fixture set
   - full iterations
   - threshold enforcement + report artifact

Benchmark failures should upload artifacts containing raw run data and summary.

---

## 11) Review and Change Control

- Any change to this protocol requires doc review.
- Threshold changes require rationale in PR description.
- If measurement tooling changes, baselines must be re-established and clearly
  marked as a new baseline generation.

---

*Protocol v0.1 (Draft). Update `docs/INDEX.md` when this document status
changes.*
