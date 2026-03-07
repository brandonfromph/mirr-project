---
name: add-emit-target
description: 'Add or modify an emission backend for the MIRR compiler (verilog, json, dot, firrtl, or new formats). Use this when working on the emit subsystem.'
argument-hint: 'Emit format to work on (e.g., "firrtl", "verilog", "new-format-name")'
---

# MIRR Emit Target Skill

Guide for adding or modifying an emission backend in the MIRR compiler.

## Architecture

All emitters live in `src/emit/` and follow the same contract:

```
src/emit/
├── mod.rs          # barrel module — register new emitters here
├── verilog.rs      # SystemVerilog RTL + SVA assertions
├── firrtl.rs       # FIRRTL intermediate representation
├── json_netlist.rs # Machine-readable JSON
└── dot.rs          # Graphviz DOT graph
```

### Emitter Contract

Every emitter must:

1. **Accept `&PipelineResult`** as the sole input
2. **Return `String`** (or `Result<String, _>` for serialization)
3. **Be a pure function** — no side effects
4. **Use `#![forbid(unsafe_code)]`**
5. **Bound all traversals** with `MAX_*` constants (NASA Power-of-10)

### Entry Point Pattern

```rust
pub fn emit_<format>(result: &PipelineResult) -> String {
    let module = &result.program.module;
    let mut out = String::with_capacity(4096);
    // ... emit sections ...
    out
}
```

## Steps for a New Backend

1. **Create `src/emit/<format>.rs`** following the contract above
2. **Register in `src/emit/mod.rs`**: add `pub mod <format>;`
3. **Wire into CLI** in `src/bin/mirr-compile.rs`:
   - Add `"<format>" =>` arm in the emit match
   - Update the error message listing valid formats
   - Update `print_help()` to document the new format
4. **Add tests** in `tests/emit_<format>_tests.rs` covering:
   - Basic structure (header, module declaration)
   - Port declarations (input, output, internal)
   - Temporal guard hardware (if applicable)
   - Reflex assignments
   - Property annotations
5. **Update docs**: `.github/copilot-instructions.md` commands section

## Key Types to Map

| MIRR Type | Description |
|-----------|-------------|
| `SignalType::Bool` | Single-bit boolean |
| `SignalType::Unsigned(w)` | Fixed-width unsigned integer |
| `BinaryOp::*` | 13 binary operators (And, Or, Lt, Add, etc.) |
| `UnaryOp::Not` | Logical negation |
| `PropertyFormula::Always/Never/AlwaysImplies` | The three property forms |
| `CompiledGuard::ShiftRegister/Counter/Complex` | Temporal hardware primitives |

## Verification

```bash
# Build
cargo build

# Run new tests
cargo test --test emit_<format>_tests

# All tests pass
cargo test --all

# Zero clippy warnings
cargo clippy --all-targets --all-features -- -D warnings

# Compile an example
cargo run --bin mirr-compile -- --emit <format> examples/neonatal_respirator.mirr
```
