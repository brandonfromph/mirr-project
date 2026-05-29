---
title: Contributing
nav_order: 12
---

# Contributing to MIRR

This guide covers coding standards, campaign workflow, error code allocation, and testing requirements.

---

## Coding Standards

### Mandatory for All Files

- `#![forbid(unsafe_code)]` — every `.rs` file, no exceptions
- `#![deny(warnings)]` — zero compiler warnings
- `cargo fmt` — all code formatted
- `cargo clippy -- -D warnings` — zero clippy diagnostics

### NASA Power-of-10 Compliance

1. **No unbounded recursion.** Use iterative algorithms with explicit stacks.
2. **All loops must have a fixed upper bound.** Document the bound as a `const`.
3. **No dynamic memory allocation after initialization** (where feasible).
4. **All functions must have a bounded stack footprint** (≤ 64 KiB worst case).
5. **No function pointers or indirect calls** (except trait objects for emission backends).

### Naming Conventions

- Modules: `snake_case`
- Types/Enums: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Error codes: `E{phase}{sequence}` (e.g., E601 = type checker, first code)

---

## Error Code Allocation

Error codes are allocated per subsystem to avoid collisions:

| Range | Subsystem | Module |
|-------|-----------|--------|
| E1xx | Parse errors (lexer + parser) | `src/lexer/`, `src/parser/` |
| E2xx | Semantic / validation errors | `src/ecs/semantic_validate.rs`, `src/validation/` |
| E3xx | Temporal compilation errors | `src/temporal/`, `src/ecs/systems.rs` |
| E4xx | Pattern matching errors | `src/expand/` |
| E5xx | Width inference errors | `src/width/` |
| E6xx | Extended type checking errors | `src/typeck/` |
| E7xx | R-SPU backend errors | `src/emit/rspu.rs` |
| E8xx | S-expression / Struct errors | `src/sexpr/`, `src/ast/` |
| E9xx | SAT / Equivalence errors | `src/sat/`, `src/verify/` |
| E10xx | Symbolic analysis errors | `src/symbolic/` |
| E11xx | Totality checker errors | `src/totality/` |
| E12xx | Testing & Tooling errors | `src/tools/`, `src/emit/hls.rs` |

When adding a new error code:
1. Choose the next sequential number in the appropriate range
2. Add the code to the subsystem's diagnostic enum
3. Add a test that triggers the error and checks the code string
4. Update `docs/error_codes.md` if it exists

---

## Campaign Workflow

All changes to MIRR follow the proposal-based campaign workflow defined in `.github/skills/propose-campaign/SKILL.md`.

### Quick Summary

1. **Propose** — write a proposal in `proposals/` with scope, risk analysis, wave plan
2. **Review** — user reviews; status must be SIGNED before execution
3. **Execute** — follow the wave plan; each wave has a build gate
4. **Verify** — run `cargo test --all`, `cargo clippy`, `cargo fmt --check`
5. **Close** — update proposal status to EXECUTED

### Scope Categories

| Scope | Changes | Living doc required? |
|-------|---------|---------------------|
| Patch | 1–3 files, bug fix or typo | Only if metrics change |
| Campaign | 4–15 files, feature or refactor | Yes |
| Architecture | 15+ files, structural change | Yes |

---

## Testing Requirements

### Every New Feature Must Have Tests

- Unit tests alongside the implementation (in `#[cfg(test)]` modules)
- Integration tests in `tests/` for cross-module behavior
- Every error code must have at least one test that triggers it

### Test Organization

| File Pattern | Purpose |
|---|---|
| `tests/*_tests.rs` | Integration tests for each subsystem |
| `tests/fixtures/*.mirr` | MIRR source files for test input |
| `#[cfg(test)] mod tests` | Unit tests within source files |

### Running Tests

```bash
# All tests
cargo test --all

# Specific test file
cargo test --test simplify_tests

# With output
cargo test -- --nocapture
```

### Conditional Skip Pattern

For tests that require external tools (Yosys, Verilator, sby, etc.):

```rust
fn tool_available(name: &str) -> bool {
    std::process::Command::new(name).arg("--version").output().is_ok()
}

#[test]
fn test_yosys_synthesis() {
    if !tool_available("yosys") {
        eprintln!("skipping: yosys not in PATH");
        return;
    }
    // ... test body
}
```

---

## Documentation Requirements

- New modules need a `//!` doc comment at the top
- Public APIs need `///` doc comments
- Run `cargo doc --no-deps` and verify zero warnings
- Update the living document (`paper/living-doc/`) for Campaign and Architecture scope changes

---

## See Also

- [Roadmap](roadmap) — Project phases and architecture
- [Glossary](glossary) — Terminology reference
- [Type System](type-system) — Type checker reference
