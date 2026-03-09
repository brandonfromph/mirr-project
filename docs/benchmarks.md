# MIRR Compiler Benchmarks

Criterion benchmarks for measuring parser and full-pipeline performance.

## Running

```bash
cargo bench
```

HTML reports are written to `target/criterion/`.

## Tiers

| Tier | Signals | Guards | Reflexes | Purpose |
|------|---------|--------|----------|---------|
| Small | 2 | 1 | 1 | Baseline latency |
| Medium | 8 | 4 | 4 | Typical usage |
| Large | 32 | 16 | 16 | Stress test |

## Benchmark groups

- **parse/small**, **parse/medium**, **parse/large** — `parse_mirr()` only
- **pipeline/small**, **pipeline/medium**, **pipeline/large** — full `run_pipeline()` (parse + pattern expand + validate + typecheck + simplify + width + temporal)

## Comparing changes

```bash
# Save a baseline
cargo bench -- --save-baseline before

# Make changes, then compare
cargo bench -- --baseline before
```

Results in `target/criterion/` include statistical comparison with confidence intervals.
