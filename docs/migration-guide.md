---
title: Migration Guide
nav_order: 3
---

# Migration Guide

This guide covers breaking and notable changes between MIRR compiler versions.
Each version section is organized by audience.

## 0.1.0 → 0.2.0

> [!NOTE]
> All new fields use `#[serde(default)]`, so JSON deserialization of data
> produced by older compiler versions continues to work without changes.

---

## For Rust API Consumers

### New enum variants on `PropertyFormula`

> [!IMPORTANT]
> If your code matches on `PropertyFormula` without a wildcard arm, you
> must add cases for the three new variants or compilation will fail.

`PropertyFormula` now has **6 variants** (was 3). If your code matches on this
enum without a wildcard arm, you must handle the new variants:

| New Variant | MIRR Syntax |
|-------------|-------------|
| `NeverImplies { antecedent, consequent }` | `never (P -> Q)` |
| `EventuallyWithin { expr, cycles }` | `eventually within N (P)` |
| `AlwaysFollowedBy { trigger, response, delay_cycles }` | `always (P followed_by N Q)` |

The `exprs()` and `exprs_mut()` methods on `PropertyFormula` handle all 6
variants, so code using those methods needs no changes.

### New `directive` field on `PropertyDecl`

`PropertyDecl` has a new field:

```rust
pub directive: PropertyDirective,  // Assert (default), Cover, Assume
```

It is annotated with `#[serde(default)]`, so JSON deserialization of old data
still works (defaults to `Assert`).

### New fields on `Module`

Three new fields were added to `Module`:

| Field | Type | Default |
|-------|------|---------|
| `properties` | `Vec<PropertyDecl>` | `[]` (serde default) |
| `pattern_calls` | `Vec<PatternCall>` | `[]` (serde default) |
| `pattern_origins` | `Vec<PatternOrigin>` | `[]` (serde default) |

All three have `#[serde(default)]`, so deserialization of old JSON is
backward-compatible.

### New public types

| Type | Module |
|------|--------|
| `PropertyDirective` | `crate::ast::property` |
| `PropertyFormula` (3 new variants) | `crate::ast::property` |
| `PatternDef` | `crate::ast::pattern` |
| `PatternOrigin` | `crate::ast::pattern` |
| `PatternParam` | `crate::ast::pattern` |

### New public functions

| Function | Purpose |
|----------|---------|
| `expand_patterns()` | Expand `def`/`reflect` pattern calls |
| `emit_firrtl()` | Emit FIRRTL output |
| `emit_sva_only()` | Emit standalone SVA (no Verilog wrapper) |

---

## For JSON Netlist Consumers

### New top-level field: `schema_version`

The JSON netlist now includes a `schema_version` field as the first key:

```json
{
  "schema_version": "0.2.0",
  "ir_version": "1.0",
  ...
}
```

`ir_version` remains `"1.0"` and tracks the IR contract version.
`schema_version` tracks the JSON output schema and will be bumped when
fields are added, removed, or renamed.

**Action required:** If your JSON parser rejects unknown fields, allow
`schema_version` (type: string).

### `properties` array is always present

Previously, the `properties` key was absent when no properties existed.
Now it is always present as an array (may be empty `[]`).

### Property objects have new fields and values

Each object in the `properties` array now has:

| Field | New? | Values |
|-------|------|--------|
| `name` | No | Property name string |
| `directive` | **Yes** | `"assert"`, `"cover"`, or `"assume"` |
| `kind` | No (3 new values) | `"always"`, `"never"`, `"always_implies"`, `"never_implies"`, `"eventually_within"`, `"always_followed_by"` |
| `formula_text` | No | Human-readable formula string |

### `pattern_origins` array (conditional)

When a module uses pattern calls, a `pattern_origins` array appears:

```json
"pattern_origins": [
  { "pattern_name": "threshold_guard", "args_summary": "temperature, 100, 5, temp_alarm" }
]
```

This array is omitted (not present) when empty.

---

## For MIRR Syntax / Tool Authors

### New keywords in `property` blocks

Five new keywords are recognized inside `property` blocks:

| Keyword | Context |
|---------|---------|
| `cover` | Directive prefix: `cover eventually within N (P)` |
| `assume` | Directive prefix: `assume always (P)` |
| `eventually` | Formula keyword: `eventually within N (P)` |
| `within` | Used with `eventually`: `eventually within N (P)` |
| `followed_by` | Used in implication: `always (P followed_by N Q)` |

These keywords only have special meaning inside `property {}` blocks.
They do **not** affect parsing of `signal`, `guard`, or `reflex` blocks.

### New top-level keywords: `def` and `reflect`

The pattern system introduces two new top-level constructs:

```
def pattern_name(param1: type, ...) {
    reflect {
        guard ... { ... }
        reflex ... { ... }
        property ... { ... }
    }
}
```

Pattern calls appear at module scope: `pattern_name(arg1, arg2, ...);`

---

## For Error Message Parsers

All error messages now start with a structured error code prefix:

| Prefix | Category | Example |
|--------|----------|---------|
| `[E1xx]` | Parse errors (E101–E166, E170–E181) | `[E100] Parse error: [E171] Unbalanced parentheses in expression.` |
| `[E2xx]` | Semantic errors | `Semantic error: [E201] Duplicate signal name: 'x'.` (unique codes E201–E216) |
| `[E300]` | Temporal errors | `[E300] Temporal compilation error: ...` |
| `[E4xx]` | Pattern errors (E400–E425) | `[E400] Pattern error: [E401] ...` |

If you parse error output, you can now match on the `[Ennn]` prefix to
categorize errors programmatically.

---

## 0.2.0 → 0.3.0

> [!WARNING]
> Version 0.3.0 introduces signed types, a type checker, and the R-SPU
> backend. Code that matches exhaustively on `SignalType`, `UnaryOp`, or
> `PatternParamKind` will require updates.

Version 0.3.0 ships the results of 8 implementation campaigns. This section
covers all breaking and notable changes.

### Campaign summary

| Campaign | Description |
|----------|-------------|
| **SEM-001** | `prev(signal, delay)` temporal back-references with semantic validation (delay ≥ 1) and E209 error code |
| **TYPE-001** | Signed integer types `i8`, `i16`, `i32`, `i64` via `SignalType::Signed(u8)` and unary negate operator `UnaryOp::Negate` |
| **TYPE-002** | Type checker module `src/typeck/` with `typecheck_module()`, enforcing signedness consistency, E6xx errors (E601–E609) |
| **TYPE-003** | Width inference updated for signed types with two's complement semantics, E5xx errors updated |
| **TYPE-004** | 3 new property forms: `never (P -> Q)`, `eventually_within(P, N)`, `always_followed_by(P, Q, N)` — total 6 property forms |
| **TYPE-005** | Higher-order patterns — `PatternParamKind::Pattern` and `PatternArg::PatternRef` for passing patterns as arguments |
| **ROCQ-001** | Rocq/Coq proof framework in `proofs/` directory |
| **RSPU-001** | R-SPU instruction set backend — `emit_rspu()`, `RspuProgram`, `RspuInstruction`, register allocator, E7xx errors (E700–E714) |

---

### For Rust API Consumers

#### New public types

| Type | Module | Campaign |
|------|--------|----------|
| `SignalType::Signed(u8)` | `crate::ast::types` | TYPE-001 |
| `UnaryOp::Negate` | `crate::ast::types` | TYPE-001 |
| `TypeError` | `crate::typeck` | TYPE-002 |
| `TypeMap` | `crate::typeck` | TYPE-002 |
| `PatternParamKind::Pattern` | `crate::ast::pattern` | TYPE-005 |
| `PatternArg::PatternRef` | `crate::ast::pattern` | TYPE-005 |
| `RspuProgram` | `crate::emit::rspu` | RSPU-001 |
| `RspuInstruction` | `crate::emit::rspu` | RSPU-001 |
| `RspuError` | `crate::emit::rspu` | RSPU-001 |

#### New public functions

| Function | Purpose | Campaign |
|----------|---------|----------|
| `typecheck_module()` | Run signedness/type consistency checks on a `Module`; returns `Vec<TypeError>` | TYPE-002 |
| `emit_rspu()` | Emit an `RspuProgram` from a `Module`; returns `Result<RspuProgram, RspuError>` | RSPU-001 |

#### New `PipelineConfig` fields

Two new boolean fields control the additional pipeline stages:

| Field | Type | Default | Purpose |
|-------|------|---------|---------|
| `typecheck` | `bool` | `false` | Enables the type checking stage (`typecheck_module()`) |
| `rspu` | `bool` | `false` | Enables R-SPU instruction set emission (`emit_rspu()`) |

Both fields default to `false` for backward compatibility. Existing
`PipelineConfig` values continue to work without modification.

#### New enum variants

`SignalType` gains a `Signed(u8)` variant (TYPE-001). If your code matches on
`SignalType` without a wildcard arm, you must handle:

```rust
SignalType::Signed(width) => { /* i8, i16, i32, i64 */ }
```

`UnaryOp` gains a `Negate` variant (TYPE-001). If your code matches on
`UnaryOp`, add:

```rust
UnaryOp::Negate => { /* unary minus, e.g. -x */ }
```

`PatternParamKind` gains a `Pattern` variant (TYPE-005). Pattern parameters
can now accept other patterns as arguments via `PatternArg::PatternRef`.

---

### For JSON Netlist Consumers

#### `schema_version` bumped to `"0.3.0"`

The `schema_version` field is now `"0.3.0"`. `ir_version` remains `"1.0"`.

#### Signal objects may contain `signed: true`

Signals declared with signed types (`i8`, `i16`, `i32`, `i64`) include a
`"signed": true` field. Unsigned signals omit this field or set it to `false`.

---

### For MIRR Syntax / Tool Authors

#### Signed type keywords

Four new type keywords are recognized in `signal` declarations:

| Keyword | Width |
|---------|-------|
| `i8` | 8 bits, signed |
| `i16` | 16 bits, signed |
| `i32` | 32 bits, signed |
| `i64` | 64 bits, signed |

#### `prev()` temporal back-reference

The `prev(signal, delay)` built-in is now available in guard and reflex
expressions. The `delay` argument must be a positive integer (≥ 1).

#### Higher-order pattern arguments

Pattern definitions can now accept other patterns as parameters using the
`pattern` parameter kind, enabling composition of reusable design patterns.

---

### For Error Message Parsers

The structured error code table is extended with the following new entries:

| Prefix | Category | Example |
|--------|----------|---------|
| `[E210]` | Semantic error — duplicate property name | `Semantic error: [E210] Duplicate property name: 'watchdog'.` |
| `[E5xx]` | Width inference errors (E500–E511) | `Width error: [E500] Cannot infer width for signal 'x'.` |
| `[E6xx]` | Type errors (E601–E609) | `Type error: [E601] Signedness mismatch in binary operation.` |
| `[E7xx]` | R-SPU errors (E701–E715) | `RSPU error: [E701] Register allocation failed for signal 'y'.` |
| `[E8xx]` | S-expression errors (E800–E815) | `SExpr error: [E800] Invalid token in S-expression.` |

The full error code table (0.1.0 through 0.3.0) is:

| Range | Category |
|-------|----------|
| `[E1xx]` (E101–E166, E170–E181) | Parse errors |
| `[E2xx]` (E201–E216) | Semantic errors |
| `[E300]` | Temporal errors |
| `[E4xx]` (E400–E425) | Pattern errors |
| `[E5xx]` (E500–E511) | Width inference errors |
| `[E6xx]` (E601–E609) | Type errors |
| `[E7xx]` (E701–E715) | R-SPU backend errors |
| `[E8xx]` (E800–E815) | S-expression errors |

---

## 0.3.0 → 0.4.0 (Phase 3 ECS Migration)

> [!WARNING]
> Version 0.4.0 introduces the Data-Oriented Entity Component System (ECS) architecture. AST `Expr` nodes are no longer used for logic evaluation in production pipelines; all expressions are lowered into flat contiguous arrays in `src/ecs/registry.rs`.

### For Rust API Consumers

#### New ECS Pipeline
The compilation pipeline now centers entirely around the ECS `Registry`. The `Expr` AST tree is strictly used for the parsing phase and legacy bootstrap tools. If your code directly manipulates `Expr` enum variants for passes like constant folding, it must be rewritten to iterate over `Registry` component arrays.

#### New `PipelineConfig` Fields
Several new execution controls have been added:
- `bootstrap_mode`: Runs the typechecker in a relaxed hydration mode.
- `sat_simplify`: Enables formal SAT-based logic simplification.
- `mape_k`: Runs the MAPE-K autonomic simulation pass.
- `emit_mape_k_rtl`: Emits full SystemVerilog logic from the MAPE-K simulation.

#### Rayon Concurrency
Systems in `src/ecs/systems.rs` (e.g., `parallel_constant_folding_system`) heavily utilize `rayon`. To prevent thread starvation, avoid invoking the MIRR pipeline from within external tight `rayon` threadpools without allocating capacity.

### For Error Message Parsers
New structured error code ranges were introduced:
| Range | Category | Example |
|-------|----------|---------|
| `[E9xx]` | SAT / Equivalence | `Sat error: [E901] Receipt generation failed.` |
| `[E10xx]`| Symbolic Analysis | `Symbolic error: [E1003] Signal limit exceeded.` |
| `[E11xx]`| Totality | `Totality error: [E1100] Fallback.` |
| `[E12xx]`| Testing & Tooling | `Tooling error: [E1201] RISC-V emission failed.` |

---

## See Also

- [Error Codes](error_codes) — Complete error code catalogue
- [Type System](type-system) — New type rules added in 0.2.0+
- [R-SPU ISA Spec](rspu_isa_spec) — R-SPU backend added in 0.3.0
- [Tutorial](tutorial) — Updated for all current features
- [Roadmap](roadmap) — Upcoming changes to plan for
