# Design Spec: MIRR-CORE Interpreter Runtime

> **Status:** Draft  
> **Version:** 0.1  
> **Date:** 2026-03-01  
> **Author(s):** MIRR Core Team  
> **Related ADR(s):** ADR-002  
> **Related Milestone:** Post-Milestone Stream 1 (MIRR-CORE Interpreter)

---

## 1. Purpose

The MIRR-CORE interpreter is a Rust-hosted tree-walking executor for
MIRR-CORE programs. It enables stage-2 self-hosting: executing the
`compiler_mirr/*.mirr` compiler modules to produce AST and netlist output,
which is then compared to the Rust reference pipeline for parity.

## 2. Goals and Non-Goals

### Goals
- Execute any valid MIRR-CORE program (as defined by `self_hosting_core_spec.md`).
- Produce deterministic output for the same input, always.
- Enforce NASA safety invariants at runtime (bounded stack, bounded loops).
- Provide clear, actionable error messages on failure.
- Support trace output for debugging.
- Serve as the executable semantics reference for MIRR-CORE.

### Non-Goals
- High performance (tree-walking is acceptable; optimization is Stream 3/5).
- Interpreting full MIRR (only MIRR-CORE subset).
- Garbage collection (no heap allocation in MIRR-CORE).
- Concurrency or parallelism.
- Interactive REPL.

## 3. Background

- Language subset: `docs/self_hosting_core_spec.md` (frozen v1).
- IR contract: `docs/self_hosting_ir_contract.md` (frozen v1.0).
- Architecture decision: `docs/decisions/ADR-002-interpreter-architecture.md`.
- Stdlib primitives: `stdlib/mirr_core/*.mirr`.

## 4. Architecture

### 4.1 High-Level Diagram

```
                    ┌─────────────────────────────────────┐
                    │         Interpreter Host (Rust)      │
                    │                                      │
  .mirr source ──→  │  ┌────────┐   ┌──────────────────┐  │
                    │  │ Parser │──→│ Module Registry   │  │
                    │  │ (reuse)│   │ (fn table, types) │  │
                    │  └────────┘   └────────┬─────────┘  │
                    │                        │             │
                    │              ┌─────────▼──────────┐  │
                    │              │ Tree-Walking Engine │  │
                    │              │  ┌──────┐ ┌──────┐ │  │
                    │              │  │ eval │ │ exec │ │  │
                    │              │  └──────┘ └──────┘ │  │
                    │              └─────────┬──────────┘  │
                    │                        │             │
                    │              ┌─────────▼──────────┐  │
                    │              │  Stdlib Intrinsics  │  │
                    │              │  (Rust functions)   │  │
                    │              └─────────┬──────────┘  │
                    │                        │             │
                    │                        ▼             │
                    │              Result<Value, Error>    │──→ JSON output
                    │                                      │
                    └─────────────────────────────────────┘
```

### 4.2 Key Components

| Component | Responsibility | Module/File |
|-----------|---------------|-------------|
| Module Registry | Stores parsed modules, function lookup | `src/interpreter/mod.rs` |
| Value | Runtime value representation | `src/interpreter/value.rs` |
| Environment | Scope chain, call frames, local variables | `src/interpreter/env.rs` |
| Expression Evaluator | Evaluates MIRR-CORE expressions to Values | `src/interpreter/eval.rs` |
| Statement Executor | Executes statements, manages control flow | `src/interpreter/exec.rs` |
| Stdlib Intrinsics | Rust implementations of `stdlib/mirr_core/*` | `src/interpreter/stdlib.rs` |
| Trace Logger | Optional deterministic execution trace | `src/interpreter/trace.rs` |

### 4.3 Data Flow

1. **Input:** `.mirr` source file path.
2. **Parse:** Reuse `src/parser/` to produce AST.
3. **Register:** Load module functions and types into registry.
4. **Execute:** Call the designated entry-point function.
5. **Output:** Return value (typically a JSON-serializable structure).

### 4.4 Interfaces

#### Public API

```rust
/// Main interpreter configuration.
pub struct InterpreterConfig {
    /// Maximum call stack depth (default: 256).
    pub max_call_depth: usize,
    /// Maximum iterations per loop instance (default: 1_000_000).
    pub max_loop_iterations: usize,
    /// Enable trace logging.
    pub trace_enabled: bool,
}

/// Interpret a parsed MIRR-CORE module, calling the named function.
pub fn interpret_module(
    module: &Module,
    entry_fn: &str,
    args: &[Value],
    config: &InterpreterConfig,
) -> Result<Value, InterpError>;

/// Register stdlib intrinsics into the interpreter environment.
pub fn register_stdlib(env: &mut Environment);
```

#### CLI

```
cargo run -- --interpret <module.mirr> [--entry <fn_name>] [--interpreter-trace]
```

## 5. Detailed Design

### 5.1 Runtime Value Representation

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    /// Fixed-size array. Length is immutable after creation.
    Array(Vec<Value>),
    /// Struct with ordered fields (BTreeMap for deterministic iteration).
    Struct(BTreeMap<String, Value>),
    /// String slice (modeled as owned String for interpreter simplicity).
    Str(String),
    /// Enum variant: (variant_name, optional payload).
    Enum(String, Option<Box<Value>>),
    /// Unit / void return.
    Void,
}
```

**Determinism rule:** `BTreeMap` is used for struct fields to guarantee
iteration order is alphabetical by field name. This matches `serde_json`
default behavior and ensures JSON output is byte-stable.

### 5.2 Environment and Scope Chain

```rust
pub struct CallFrame {
    /// Function name (for trace/error reporting).
    pub fn_name: String,
    /// Local variable bindings.
    pub locals: BTreeMap<String, Value>,
}

pub struct Environment {
    /// Call stack (bounded by config.max_call_depth).
    pub call_stack: Vec<CallFrame>,
    /// Module-level function registry.
    pub functions: BTreeMap<String, FnDef>,
    /// Stdlib intrinsic registry.
    pub intrinsics: BTreeMap<String, IntrinsicFn>,
    /// Interpreter configuration.
    pub config: InterpreterConfig,
    /// Trace log (if enabled).
    pub trace: Option<Vec<TraceEntry>>,
}

/// Type alias for intrinsic functions.
pub type IntrinsicFn = fn(&[Value]) -> Result<Value, InterpError>;
```

**Call stack depth enforcement:**
```rust
fn push_frame(&mut self, frame: CallFrame) -> Result<(), InterpError> {
    if self.call_stack.len() >= self.config.max_call_depth {
        return Err(InterpError::StackOverflow {
            depth: self.call_stack.len(),
            limit: self.config.max_call_depth,
            fn_name: frame.fn_name,
        });
    }
    self.call_stack.push(frame);
    Ok(())
}
```

### 5.3 Expression Evaluation

The evaluator walks AST `Expr` nodes and returns `Result<Value, InterpError>`:

| Expr variant | Evaluation rule |
|-------------|----------------|
| `Literal(Bool(b))` | → `Value::Bool(b)` |
| `Literal(Integer(n))` | → `Value::U32(n)` (default) or typed by context |
| `Signal(name)` | → lookup `name` in current scope locals |
| `Unary(Not, e)` | → evaluate `e`, apply `!` (bool or bitwise) |
| `Binary(op, l, r)` | → evaluate both, apply operation with type checking |
| `FnCall(name, args)` | → evaluate args, dispatch to function or intrinsic |
| `FieldAccess(e, f)` | → evaluate `e` as Struct, lookup field `f` |
| `Index(e, i)` | → evaluate `e` as Array, `i` as usize, bounds-check |

**Bounds checking:** every array index access checks `i < array.len()` and
returns `InterpError::IndexOutOfBounds` on violation.

### 5.4 Statement Execution

| Statement | Execution rule |
|-----------|---------------|
| `Let(name, ty, expr)` | Evaluate `expr`, bind `name` in current frame |
| `Assign(lvalue, expr)` | Evaluate `expr`, update binding at `lvalue` |
| `If(cond, then, else)` | Evaluate `cond` as bool, execute branch |
| `For(var, start, end, body)` | Bounded loop: iterate `var` from `start` to `end` |
| `Loop(body)` | Execute body with iteration counter; require explicit `break` |
| `While(cond, body)` | Evaluate `cond` each iteration; enforce iteration limit |
| `Match(expr, arms)` | Evaluate `expr`, find matching arm, execute |
| `Return(expr)` | Evaluate `expr`, signal early return to caller |
| `Break` | Signal loop exit |
| `ExprStmt(expr)` | Evaluate `expr`, discard result |

**Loop safety backstop:**
```rust
fn exec_for(&mut self, var: &str, start: usize, end: usize, body: &[Stmt]) 
    -> Result<ControlFlow, InterpError> 
{
    if end.saturating_sub(start) > self.env.config.max_loop_iterations {
        return Err(InterpError::LoopLimitExceeded { 
            requested: end - start,
            limit: self.env.config.max_loop_iterations,
        });
    }
    for i in start..end {
        self.env.set_local(var, Value::Usize(i));
        match self.exec_block(body)? {
            ControlFlow::Break => break,
            ControlFlow::Return(v) => return Ok(ControlFlow::Return(v)),
            ControlFlow::Continue => {}
        }
    }
    Ok(ControlFlow::Continue)
}
```

### 5.5 Stdlib Intrinsic Bindings

| Intrinsic | MIRR-CORE signature | Rust implementation |
|-----------|--------------------|--------------------|
| `str_len` | `fn str_len(s: &str) -> usize` | `s.len()` |
| `str_byte_at` | `fn str_byte_at(s: &str, i: usize) -> u8` | `s.as_bytes()[i]` with bounds check |
| `str_slice` | `fn str_slice(s: &str, start: usize, end: usize) -> &str` | `&s[start..end]` with bounds check |
| `str_eq` | `fn str_eq(a: &str, b: &str) -> bool` | `a == b` |
| `token_buffer_new` | `fn token_buffer_new() -> TokenBuffer` | Create empty struct |
| `token_buffer_push` | `fn token_buffer_push(buf: &mut TokenBuffer, tok: Token)` | Append with capacity check |
| `token_buffer_len` | `fn token_buffer_len(buf: &TokenBuffer) -> usize` | Return count |
| `fixed_map_new` | `fn fixed_map_new() -> FixedMap` | Create empty BTreeMap |
| `fixed_map_insert` | `fn fixed_map_insert(m: &mut FixedMap, k: &str, v: u32) -> bool` | Insert with capacity check |
| `fixed_map_get` | `fn fixed_map_get(m: &FixedMap, k: &str) -> Option<u32>` | Lookup |
| `diagnostic_new` | `fn diagnostic_new(code: u32, msg: &str) -> Diagnostic` | Create struct |
| `diagnostic_emit` | `fn diagnostic_emit(d: &Diagnostic)` | Append to diagnostic log |

### 5.6 Error Handling

```rust
#[derive(Debug)]
pub enum InterpError {
    TypeError { expected: String, got: String, context: String },
    IndexOutOfBounds { index: usize, length: usize, array_name: String },
    StackOverflow { depth: usize, limit: usize, fn_name: String },
    LoopLimitExceeded { requested: usize, limit: usize },
    UndefinedFunction(String),
    UndefinedVariable(String),
    DivisionByZero { context: String },
    AssertionFailed(String),
    /// An explicit error return from MIRR-CORE code.
    MirrError { code: u32, message: String },
}
```

All errors include enough context for actionable diagnostics. No panics in
the interpreter; all failures are `Result::Err`.

### 5.7 Trace Output

When `--interpreter-trace` is enabled, the interpreter logs entries:

```json
[
  { "step": 1, "kind": "call", "fn": "tokenize", "args": ["...source..."] },
  { "step": 2, "kind": "let", "var": "pos", "value": 0 },
  { "step": 3, "kind": "for_enter", "var": "i", "range": [0, 366] },
  ...
  { "step": N, "kind": "return", "fn": "tokenize", "value": { "len": 42 } }
]
```

Trace output is deterministic for the same input. It is written to stderr
(not stdout) to avoid contaminating pipeline output.

### 5.8 Determinism & Safety

- [x] All loops bounded (parse-time check + runtime backstop counter)
- [x] No heap allocation in interpreted code (interpreter host uses Rust heap)
- [x] Deterministic output for same input (BTreeMap ordering, no randomness)
- [x] No hidden mutable global state (all state in Environment)

## 6. IR / Schema Impact

| Contract level | Schema file | Change required? |
|---------------|-------------|-----------------|
| AST (Level 1) | `mirr_ast.schema.json` | No |
| Netlist (Level 3) | `mirr_temporal_netlist.schema.json` | No |

The interpreter consumes existing IR; it does not modify schemas.

## 7. Test Strategy

| Category | Location | What it verifies |
|----------|----------|-----------------|
| Unit: eval | `src/interpreter/eval.rs` (mod tests) | Each expr type evaluates correctly |
| Unit: exec | `src/interpreter/exec.rs` (mod tests) | Each stmt type executes correctly |
| Unit: stdlib | `src/interpreter/stdlib.rs` (mod tests) | Each intrinsic produces correct output |
| Integration: smoke | `tests/interpreter_tests.rs` | End-to-end: parse → interpret → check output |
| Integration: stdlib | `tests/interpreter_stdlib_tests.rs` | All stdlib primitives via MIRR-CORE calls |
| Safety: stack overflow | `tests/interpreter_tests.rs` | Deep call chain hits limit gracefully |
| Safety: loop limit | `tests/interpreter_tests.rs` | Excessive loop detected and reported |
| Parity: stage-2 | `tests/stage2_parity_tests.rs` | Interpreter output == Rust reference output |

## 8. Performance Considerations

- Tree-walking expected to be 10–100× slower than Rust native pipeline.
- Acceptable for current fixture suite (single small module).
- If expanded fixtures cause unacceptable test times, upgrade path is bytecode
  VM (documented in ADR-002 alternatives).
- Benchmark protocol (Stream 3) will establish baselines.

## 9. Migration / Rollout

- New `src/interpreter/` module added behind existing compiler infrastructure.
- CLI flag `--interpret` is additive; no changes to existing flags.
- No feature flags needed; interpreter is always compiled (small code size).
- Stage-2 parity tests are additive to existing test suite.

## 10. Open Questions

- [ ] Should the interpreter support multi-module programs (e.g., lexer module
      calling str module) from the start, or single-module first?
      **Recommendation:** multi-module from the start, since the compiler
      pipeline requires all 4 modules + stdlib.
- [ ] Should trace output be structured JSON or human-readable text?
      **Recommendation:** structured JSON for machine processing; a separate
      `--interpreter-trace-pretty` flag for human reading (future).
- [ ] How should `&mut` references be modeled? Copy-on-write, or mutable
      borrow tracking?
      **Recommendation:** pass-by-value with explicit writeback for v1
      simplicity; revisit if semantics diverge from spec.

---

*Design spec version: 0.1 (Draft) — see `docs/INDEX.md` for governance rules.*