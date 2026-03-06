# MIRR Tangible Evidence Summary

- UTC timestamp: `2026-03-01T14:49:11.797094+00:00`
- Commit: `9c95157`
- Release binary: `C:\Users\elvie\nasa-rust-project\target\release\nasa-rust-project.exe`

## 1) Temporal strategy sweep

- Threshold tested: `N <= 16 => ShiftRegister`, `N > 16 => Counter`
- Cases matched expectation: `14/14`
- Accuracy: `100.00%`

## 2) Determinism

- Runs: `20`
- Unique output hashes: `1`
- Deterministic: `True`
- Mismatch count: `0`

## 3) Throughput baseline (compile --json)

| Fixture | Mean (ms) | Median (ms) | p95 (ms) | Stddev (ms) |
|---|---:|---:|---:|---:|
| `examples\neonatal_respirator.mirr` | 10.9184 | 10.8796 | 12.6578 | 1.1061 |

## 4) Bootstrap failure modes

| Case | Expected Success | Observed Success | First Failed Stage |
|---|---:|---:|---|
| `canonical_example` | True | True | (none) |
| `malformed_parse_error` | False | False | (none) |
| `missing_file_read_error` | False | False | (none) |

## Artifact files

- `strategy_sweep.csv`
- `determinism_runs.csv`
- `throughput_baseline.csv`
- `bootstrap_failure_modes.csv`
- `run_metadata.json`

