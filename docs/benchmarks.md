# MIRR Compiler Benchmarks

Criterion benchmarks for measuring parser, ECS transformations, simulation engines, and full-pipeline performance.

## Running

```bash
cargo bench
```

HTML reports are written to `target/criterion/`.

## Benchmark Suites

The `benches/` directory contains highly specialized performance tests covering the entire compiler architecture:

- **pipeline_bench.rs**: Full `run_pipeline()` benchmarks (parse + pattern expand + ECS validation + width + temporal lowering). Tests latency across Small, Medium, and Large scaling tiers.
- **rspu_bench.rs**: Benchmarks the RSPU (Rule-Based Stream Processing Unit) instruction simulator and memory footprint tracking.
- **sexpr_bench.rs**: Benchmarks the LISP-like S-expression AST generator and parser limits.
- **emit_analysis_bench.rs**: Tests the latency of target generation (SystemVerilog, JSON netlist) under extreme component counts.
- **pattern_bench.rs**: Benchmarks the speed of higher-order pattern/template expansion.
- **temporal_bench.rs**: Tests the speed of the Temporal Guard synthesis passes.

## Standard Pipeline Tiers

| Tier | Signals | Guards | Reflexes | Purpose |
|------|---------|--------|----------|---------|
| Small | 2 | 1 | 1 | Baseline latency |
| Medium | 8 | 4 | 4 | Typical usage |
| Large | 32 | 16 | 16 | Stress test / cache miss measurement |

## Comparing changes

```bash
# Save a baseline
cargo bench -- --save-baseline before

# Make changes, then compare
cargo bench -- --baseline before
```

Results in `target/criterion/` include statistical comparison with confidence intervals.
