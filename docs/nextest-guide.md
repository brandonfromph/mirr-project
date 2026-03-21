---
title: Nextest Configuration Guide
nav_order: 16
---

# Nextest Configuration Guide

MIRR uses cargo-nextest for optimized test execution, especially for the 3000+ test suite.

## Installation

```bash
cargo install cargo-nextest
```

## Configuration

The `.config/nextest.toml` file configures test execution:

```toml
[store]
dir = "target/nextest"

[profile.default]
retries = 0
fail-fast = true
slow-timeout = { period = "60s", terminate-after = 2 }

[profile.ci]
retries = 1
fail-fast = false
slow-timeout = { period = "120s", terminate-after = 3 }
```

## Test Groups

Tests are organized into groups for parallel execution:

| Group | Filter | Tests |
|-------|--------|-------|
| parser | `test(parser)` | Parser tests |
| typeck | `test(typeck)` | Type checker tests |
| width | `test(width)` | Width inference tests |
| temporal | `test(temporal)` | Temporal compilation tests |
| emit | `test(emit)` | Emitter tests |
| symbolic | `test(symbolic)` | Symbolic analysis tests |
| mape-k | `test(mape_k)` | MAPE-K simulation tests |
| sexpr | `test(sexpr)` | S-expression tests |
| patterns | `test(pattern)` | Pattern expansion tests |
| integration | `test(integration)` | Integration tests |
| bootstrap | `test(bootstrap)` | Bootstrap parity tests |

## Usage

```bash
# Run all tests
cargo nextest run

# Run with CI profile (retries, no fail-fast)
cargo nextest run --profile ci

# Run specific test group
cargo nextest run --test-group parser

# Run specific test
cargo nextest run --test riscv_integration_tests
```

## Performance

| Tool | 3000+ tests | Time |
|------|-------------|------|
| cargo test | ~120s | Sequential |
| cargo nextest | ~30s | Parallel |

## See Also

- [Testing Guide](testing-guide) — Writing tests