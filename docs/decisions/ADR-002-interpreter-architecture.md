# ADR-002: MIRR-CORE Interpreter Architecture

> **Status:** Proposed  
> **Date:** 2026-03-01  
> **Author(s):** MIRR Core Team  
> **Supersedes:** N/A  
> **Superseded by:** N/A

---

## Context

With self-hosting milestone v1 achieved, the next step is a Rust-hosted
interpreter that can **execute** `compiler_mirr/*.mirr` modules. Currently, the
bootstrap runner simulates pipeline stages but does not actually interpret
MIRR-CORE code. A real interpreter is required for:

1. **Stage-2 self-hosting:** run MIRR compiler modules to produce output and
   compare against the Rust reference pipeline.
2. **Validation of MIRR-CORE semantics:** prove the language subset is
   executable and deterministic beyond static checking.
3. **Foundation for native compilation:** the interpreter serves as the
   executable semantics reference against which any future native backend is
   tested.

### Constraints (NASA-derived)
- No heap allocation in interpreted MIRR-CORE code (interpreter host may use
  Rust heap internally, but interpreted programs must not trigger unbounded
  growth).
- All loops in interpreted code must be bounded (enforced at parse/validate
  time; interpreter adds a cycle counter as a safety backstop).
- Deterministic: same input → same output, always. No randomness, no
  environment-dependent behavior in the interpreted path.
- Single-threaded execution.

## Decision

Build a **tree-walking interpreter** hosted in Rust that directly evaluates
the MIRR-CORE AST. The interpreter will:

1. **Parse** `.mirr` source files using the existing Rust parser (reuse
   `src/parser/`).
2. **Load** module declarations into an in-memory module registry.
3. **Dispatch** function calls by looking up the function name in the module.
4. **Evaluate** expressions and execute statements by walking the AST.
5. **Bind** stdlib primitives (`str`, `token_buffer`, `fixed_map`,
   `diagnostics`) as Rust-native intrinsics callable from MIRR-CORE.
6. **Enforce** a configurable call-stack depth limit (default: 256 frames).
7. **Enforce** a configurable loop iteration limit per loop instance as a
   safety backstop (default: 1,000,000 iterations — well above any bounded
   loop in current MIRR-CORE programs).
8. **Produce** deterministic trace output for debugging (opt-in via
   `--interpreter-trace`).

### Module layout

```
src/interpreter/
├── mod.rs          # Public API: interpret_module(), InterpreterConfig
├── value.rs        # Runtime value representation (Value enum)
├── env.rs          # Environment / scope chain (local variables, call frames)
├── eval.rs         # Expression evaluator
├── exec.rs         # Statement executor
├── stdlib.rs       # Intrinsic bindings for stdlib/mirr_core/*
└── trace.rs        # Optional deterministic trace logger
```

### Runtime value representation

```rust
enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    Array(Box<[Value]>),       // fixed-size, allocated once at let-binding
    Struct(BTreeMap<String, Value>),  // ordered fields
    Str(String),               // borrowed &str modeled as owned for simplicity
    Void,                      // unit / no return value
}
```

Using `BTreeMap` for struct fields ensures deterministic field iteration order.

### Stdlib binding strategy

Stdlib primitives are **Rust intrinsics** registered in the interpreter at
startup. When MIRR-CORE code calls `str_len(s)`, the interpreter dispatches to
a Rust function `intrinsic_str_len(args: &[Value]) -> Result<Value, InterpError>`.

This avoids circular dependency (stdlib would need to be interpreted to
interpret the compiler, which uses the stdlib). In a future version, stdlib
could be interpreted once the interpreter is mature.

### Error model

- Interpreter errors are a Rust `Result<Value, InterpError>`.
- `InterpError` variants: `TypeError`, `IndexOutOfBounds`, `StackOverflow`,
  `LoopLimitExceeded`, `UndefinedFunction`, `UndefinedVariable`,
  `DivisionByZero`, `AssertionFailed`.
- MIRR-CORE `Result` types are modeled as a two-variant enum `Value`; the
  interpreter propagates early returns on error variants.

## Rationale

### Why tree-walking (not bytecode)?

| Criterion | Tree-walking | Bytecode VM |
|-----------|-------------|-------------|
| Implementation complexity | Low — walk existing AST | High — need compiler + VM + instruction set |
| Time to first working version | Days | Weeks |
| Debugging / traceability | Excellent — AST nodes map directly to source | Harder — need source maps |
| Performance | Slower (10–100× vs native) | Faster (5–50× vs native) |
| NASA safety audit | Simple — small codebase | Larger attack surface |

For stage-2 self-hosting, **correctness and auditability** matter far more
than performance. The interpreter only needs to be fast enough to run the
compiler modules on small test inputs. Performance optimization (bytecode or
native) can follow in Stream 5.

### Why Rust intrinsics for stdlib (not interpreted stdlib)?

1. Avoids bootstrap circularity.
2. Keeps the initial interpreter scope small.
3. Intrinsics can be individually tested for correctness.
4. Future migration to interpreted stdlib is non-breaking (swap dispatch target).

## Alternatives Considered

| Alternative | Pros | Cons | Why rejected |
|-------------|------|------|--------------|
| Bytecode VM | Better perf | Much higher complexity; delays stage-2 parity | Premature optimization |
| MIRR-to-Rust transpiler | Reuses Rust compiler for execution | Fragile; hard to maintain parity; debugging is indirect | Maintenance burden too high |
| MIRR-to-WASM | Portable, sandboxed | Adds WASM toolchain dependency; overkill for stage-2 | Unnecessary complexity |
| Skip interpreter, jump to native | Maximum performance | No correctness reference; huge risk | Violates documentation-first principle |

## Consequences

### Positive
- Fast path to stage-2 self-hosting parity.
- Small, auditable interpreter codebase.
- Deterministic trace output aids debugging.
- Foundation for all future execution strategies.

### Negative
- Tree-walking is slow; not suitable for large-scale compilation.
- Stdlib as intrinsics means stdlib bugs could hide behind Rust implementation.

### Risks
- Risk: interpreter semantics diverge from MIRR-CORE spec. Mitigation:
  conformance test suite per primitive and per control-flow construct.
- Risk: performance too slow for expanded fixture suite. Mitigation: benchmark
  protocol (Stream 3) will detect this early; bytecode upgrade is a known path.

## Affected Artifacts

| Artifact | Change required |
|----------|----------------|
| `src/interpreter/` | New module — interpreter implementation |
| `src/main.rs` | New CLI flags: `--interpret`, `--interpreter-trace` |
| `tests/interpreter_tests.rs` | New — smoke and conformance tests |
| `tests/interpreter_stdlib_tests.rs` | New — stdlib intrinsic tests |
| `docs/interpreter/runtime_spec.md` | New — runtime semantics spec |
| `docs/INDEX.md` | Updated with new docs |
| `Cargo.toml` | No new dependencies expected (pure Rust) |

## Compliance Notes

- [x] No impact on frozen specs (interpreter is a new component, does not modify language or IR)
- [x] No impact on IR contract version (interpreter consumes existing IR)
- [x] NASA safety rules maintained: bounded stack, bounded loops, deterministic, no unsafe
- [ ] `docs/INDEX.md` updated (will be updated when implementation begins)

---

*ADR-002 — proposed as part of Stream 1 (MIRR-CORE Interpreter) in the
post-milestone plan.*