# Fuzz Testing

MIRR uses [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) with libFuzzer to find panics and crashes in the parser and pipeline.

## Prerequisites

```bash
rustup install nightly
cargo install cargo-fuzz
```

## Targets

| Target | Entry point | Description |
|--------|-------------|-------------|
| `fuzz_parse_mirr` | `parse_mirr()` | Fuzz the parser with arbitrary byte strings |
| `fuzz_pipeline` | `run_pipeline()` | Fuzz the full pipeline (parse + validate + simplify + width + temporal) |

## Running

```bash
# Fuzz the parser (runs until stopped with Ctrl+C)
cargo +nightly fuzz run fuzz_parse_mirr

# Fuzz the full pipeline
cargo +nightly fuzz run fuzz_pipeline

# Run for a fixed number of iterations
cargo +nightly fuzz run fuzz_parse_mirr -- -runs=100000
```

## Seed corpus

The `corpus/fuzz_parse_mirr/` directory contains seed inputs (valid `.mirr` files) that give the fuzzer a starting point. Add more valid examples to improve coverage.

## If a crash is found

1. The crashing input is saved to `fuzz/artifacts/`
2. Reproduce: `cargo +nightly fuzz run fuzz_parse_mirr fuzz/artifacts/fuzz_parse_mirr/<crash-file>`
3. Fix the panic, then verify: `cargo +nightly fuzz run fuzz_parse_mirr -- -runs=0`
