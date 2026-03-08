# Copilot & AI Agent Instructions for MIRR

## Philosophy

MIRR is built on the generative power of three. Every layer of the system uses exactly three concepts. The surface language stays tiny. Power comes from constraint, not accumulation.

| Layer | The Three |
|-------|-----------|
| Language primitives | Signal, Guard, Reflex |
| Property forms | `always (P)`, `never (P)`, `always (P -> Q)`, `never (P -> Q)`, `eventually_within(P, N)`, `always_followed_by(P, Q, N)` |
| Backend engines | Cement2 (temporal), SmaRTLy (simplification), FIRWINE (width inference) |
| System roles | Design language, compiler toolchain, runtime instruction language |

The compiler is built to the same standards as its safety-critical output: `#![forbid(unsafe_code)]`, `#![deny(warnings)]`, no unbounded loops, no recursion (NASA Power-of-10).

## Architecture

```
src/
├── ast/           # AST types: Expr, Module, Guard, Reflex, Property, Pattern
├── parser/        # module_parser (parse_mirr), expr_parser, pattern_parser
├── lexer/         # Expression tokenizer
├── validation/    # Semantic checks (signal refs, duplicates, prev delays)
├── expand/        # Pattern expansion engine (def/reflect → inline)
├── typeck/         # Type checker: signedness consistency, type map (Phase 3b)
├── simplify.rs    # Boolean/arithmetic simplification (Phase 3)
├── width/         # Width inference + SCC analysis (Phase 4, FIRWINE)
├── temporal/      # Temporal guard compilation (Phase 2, Cement2)
├── emit/          # Verilog/SVA, FIRRTL, JSON netlist, DOT graph, R-SPU assembly output
├── pipeline.rs    # Full pipeline: parse → validate → expand → simplify → width → temporal
├── mape_k/        # MAPE-K autonomic simulator (monitor/analyze/plan/execute)
├── bin/           # CLI binaries (mirr-compile, mirr-simplify, mirr-width, mirr-simulate)
└── lib.rs         # Public API re-exports
```

## The Three Primitives

```mirr
module patient_monitor {
    signal heart_rate: in u16;      // Signal: a named wire
    signal alarm: out bool;

    guard bradycardia {             // Guard: a temporal condition
        when heart_rate < 60
        for 500 cycles;
    }

    reflex cardiac_alarm {          // Reflex: a reactive assignment
        on bradycardia {
            alarm = true;
        }
    }

    property hr_bounded {           // Property: a verification assertion
        always (heart_rate < 300);
    }
}
```

Properties do not affect generated hardware. They compile to SVA assertions for formal verification.

## Developer Commands

```bash
# Build
cargo build

# Run all tests
cargo test --all

# Compile a .mirr file to SystemVerilog
cargo run --bin mirr-compile -- --emit verilog examples/neonatal_respirator.mirr

# Emit SVA assertions only
cargo run --bin mirr-compile -- --emit sva examples/safety_property.mirr

# Emit JSON netlist
cargo run --bin mirr-compile -- --emit json examples/neonatal_respirator.mirr

# Emit FIRRTL
cargo run --bin mirr-compile -- --emit firrtl examples/neonatal_respirator.mirr

# Emit DOT graph
cargo run --bin mirr-compile -- --emit dot examples/neonatal_respirator.mirr

# Simplify an expression
cargo run --bin mirr-simplify -- '!(a && b) || a'

# Width inference
cargo run --bin mirr-width -- examples/neonatal_respirator.mirr --scc

# MAPE-K simulation
cargo run --bin mirr-simulate -- --ticks 10000

# Clippy (CI enforces zero warnings)
cargo clippy --all-targets --all-features -- -D warnings

# Benchmarks
cargo bench
```

## Pipeline Stages

`run_pipeline(source, config)` executes:

1. **Parse** — `parse_mirr()` → `MirrProgram` (AST with patterns + module)
2. **Validate patterns** — check pattern definitions for well-formedness
3. **Expand patterns** — `def`/`reflect` pattern calls inlined into module
4. **Validate module** — signal refs, duplicate names, prev delays, property formulas
5. **Typecheck** — signedness consistency check (optional)
6. **Simplify** — boolean/arithmetic expression simplification (optional)
7. **Width inference** — assign minimum safe bit widths, detect SCCs (optional)
8. **Temporal compile** — guards → shift registers or counters (optional)
9. **R-SPU emission** — guard/reflex → instruction stream (optional)

## Key Conventions

- All source files use `.mirr` extension
- `when signal_expr for N cycles` — temporal guard syntax
- `on guard_name { target = expr; }` — reflex syntax
- Three property forms only: `always`, `never`, `always (P -> Q)`
- `def`/`reflect` for reusable patterns (Phase 7b)
- No heap allocation or unbounded loops in safety-critical paths
- All new docs must be indexed in `docs/INDEX.md`
- Types: `bool`, `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64` — no implicit casting
- Guard combination: `on guard_a and guard_b { ... }`
- No loops, function calls, or conditionals inside reflexes
- Error codes: `[E1xx]` parse, `[E2xx]` semantic, `[E3xx]` temporal, `[E4xx]` pattern, `[E5xx]` width, `[E6xx]` type, `[E7xx]` R-SPU — see `docs/error_codes.md`
- Signed types: `i8`–`i64` use two's complement; mixing signed/unsigned is a type error (E6xx)
- Higher-order patterns: `def` can accept pattern parameters via `param: pattern` syntax

## Example: Adding a New Validation Check

1. Add the check function in `src/validation/semantic.rs`
2. Call it from `validate_module()`
3. Return `MirrError::SemanticError { message }` on failure
4. Add tests in `tests/semantic_validation_tests.rs`
5. Run `cargo test --all && cargo clippy --all-targets -- -D warnings`

## Files Reference

| Path | Purpose |
|------|---------|
| `src/pipeline.rs` | Full compilation pipeline |
| `src/parser/module_parser.rs` | Main parser (`parse_mirr`) |
| `src/parser/pattern_parser.rs` | Pattern `def`/`reflect` parsing |
| `src/validation/semantic.rs` | Semantic validation |
| `src/expand/mod.rs` | Pattern expansion engine |
| `src/emit/verilog.rs` | SystemVerilog + SVA emission |
| `src/emit/firrtl.rs` | FIRRTL emission |
| `src/emit/json_netlist.rs` | JSON netlist emission |
| `src/emit/dot.rs` | Graphviz DOT emission |
| `src/temporal/compiler.rs` | Guard → shift register/counter compilation |
| `src/width/` | Width inference, SCC detection, constraint solving |
| `src/mape_k/` | MAPE-K autonomic loop simulator |
| `examples/*.mirr` | 12 example programs (8 compilable, 2 error cases, 2 pattern demos) |
| `benches/pipeline_bench.rs` | Criterion benchmarks (3 tiers × 2 targets) |
| `fuzz/` | cargo-fuzz targets for parse_mirr and run_pipeline |
| `vscode-mirr/` | VS Code syntax highlighting extension |

---

For more, see `README.md` and `docs/INDEX.md`.
