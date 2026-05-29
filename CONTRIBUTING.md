# Contributing to MIRR

Thank you for your interest in contributing to MIRR — a safety-critical HDL compiler where correctness is non-negotiable.

**Full contribution guidelines:** [docs/contributing.md](docs/contributing.md)

---

## Quick Reference

### Prerequisites

- Rust stable toolchain (`rustup update stable`)
- `cargo nextest` for running tests (`cargo install cargo-nextest`)
- Optional: `iverilog`, `verilator` for RTL simulation tests

### Before You Submit

All PRs must pass this gate before review:

```bash
# 1. Full test suite (no failures, no skips)
cargo nextest run --workspace --no-fail-fast

# 2. Zero clippy warnings
cargo clippy --workspace -- -D warnings

# 3. Formatting check
cargo fmt --workspace --check
```

### Hard Rules

- **No `unsafe` code** — `#![forbid(unsafe_code)]` is enforced in all crates. PRs adding unsafe blocks will be rejected.
- **No `unwrap()` or `expect()`** in non-test code — use `MirrError` instead.
- **No `TODO` or `FIXME` markers** — all code must be complete when submitted.
- **No unbounded loops** — all iteration must have explicit, provable bounds.
- **Tests required** — every new language feature or synthesis rule needs a headless unit test.

### What We're Looking For

Contributions are especially welcome in:
- **Formal verification** — Rocq proofs for new compiler passes
- **Hardware synthesis** — new emit backends (FPGA families, soft-cores)
- **Safety-critical domains** — real-world MIRR examples for medical, aerospace, automotive
- **Language design** — new guard/reflex patterns with temporal semantics
- **Documentation** — tutorials, error code explanations, architecture guides

### Opening an Issue First

For any non-trivial change, please **open an issue first** to discuss the approach. MIRR has strict invariants around its ECS architecture, compiler pipeline, and formal verification chain — changes that violate these will not be accepted regardless of test coverage.

### Code of Conduct

Be respectful. This is a research-grade engineering project — critique the code, not the person.

---

For the full architecture overview, read [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).
