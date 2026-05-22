---
title: Testing Guide
nav_order: 17
---

# Testing Guide

MIRR has a comprehensive test suite with 3000+ tests covering all compiler subsystems.

## Test Organization

| Directory | Contents |
|-----------|----------|
| `tests/` | Integration tests |
| `src/*/` | Unit tests (inline `#[cfg(test)]`) |
| `benches/` | Benchmarks |
| `fuzz/` | Fuzz targets |

## Writing Tests

### Unit Tests

Unit tests go inline in the module they test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        assert_eq!(result, expected);
    }
}
```

### Integration Tests

Integration tests go in `tests/` directory:

```rust
#![forbid(unsafe_code)]
#![deny(warnings)]

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_full_pipeline() {
    let source = "module test { signals { x: in bool } }";
    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);
    assert!(result.is_ok());
}
```

## Test Conventions

1. **File size limit**: Test files must be ≤ 600 lines (CI enforced)
2. **No unsafe code**: All test files use `#![forbid(unsafe_code)]`
3. **Zero warnings**: All test files use `#![deny(warnings)]`
4. **Bounded loops**: All loops must have explicit upper bounds
5. **Error codes**: Use `[E-codes]` in assertions

## Running Tests

```bash
# All tests
cargo test --all

# With nextest (recommended)
cargo nextest run

# Specific test
cargo nextest run --test riscv_integration_tests

# With output
cargo test -- --nocapture
```

## Test Coverage

| Subsystem | Test Files | Approx Tests |
|-----------|-----------|--------------|
| Parser | 5+ | 200+ |
| Type checker | 3+ | 150+ |
| Width inference | 4+ | 300+ |
| Temporal | 3+ | 200+ |
| Emitters | 5+ | 400+ |
| Symbolic | 2+ | 200+ |
| MAPE-K | 2+ | 300+ |
| S-expression | 3+ | 200+ |
| Patterns | 2+ | 100+ |
| Integration | 10+ | 500+ |

## See Also

- [Nextest Guide](nextest-guide) — Optimized test runner
