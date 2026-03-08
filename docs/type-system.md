# MIRR Type System Reference

> **Status:** Active
> **Module:** `src/typeck/mod.rs`
> **Campaigns:** TYPE-001, TYPE-002, TYPE-003
> **Error codes:** E601–E607

The MIRR type checker runs after semantic validation and before simplification.
It enforces signedness consistency across all expressions: guard conditions,
reflex assignments, and property formulas.

---

## Signal Types

MIRR has three categories of signal types:

| Category | Types | Representation |
|----------|-------|----------------|
| Boolean  | `bool` | 1-bit logical (true/false) |
| Unsigned | `u8`, `u16`, `u32`, `u64` | N-bit unsigned integer |
| Signed   | `i8`, `i16`, `i32`, `i64` | N-bit two's complement signed integer |

Declared in MIRR source:

```mirr
module example {
    signal sensor: in u16;
    signal offset: in i16;
    signal alarm:  out bool;
    signal error_code: out u8;
}
```

---

## Type Rules

### T1: Assignment Compatibility

An assignment `target = expr` requires the expression type to be compatible
with the target signal type. Compatible means:

- **Exact match** — always accepted
- **Bool <-> Unsigned(1)** — bidirectional promotion
- **Unsigned(N) -> Unsigned(M)** where N <= M — safe zero-extension
- **Signed(N) -> Signed(M)** where N <= M — safe sign-extension
- **Signed <-> Unsigned** — always rejected (E602)

### T2: Arithmetic Operators (`+`, `-`, `*`)

- Both operands must be numeric (not Bool — E603)
- Both operands must be the same category (both signed or both unsigned — E603)
- Result width = `max(left_width, right_width)`, preserving signedness

### T3: Shift Operators (`<<`, `>>`)

- Both operands must be numeric (not Bool — E603)
- Both must be same category (signed/unsigned — E603)
- Result width = left operand width, preserving signedness

### T4: Logical Operators (`&&`, `||`)

- Both operands must be `Bool` (E604)
- Result type: `Bool`

### T5: XOR Operator (`^`)

- Both operands must match types (E607)
- Bool <-> Unsigned(1) allowed
- Result type: left operand type

### T6: Ordering Comparisons (`<`, `<=`, `>`, `>=`)

- Cannot compare `Bool` values (E605)
- Cannot compare signed vs unsigned (E605)
- Result type: `Bool`

### T7: Equality Comparisons (`==`, `!=`)

- Both operands must be same category: both Bool, both Unsigned, or both Signed (E606)
- Result type: `Bool`

### T8: Logical Not (`!`)

- Works on `Bool`, `Unsigned`, and `Signed`
- Result type: same as operand

### T9: Arithmetic Negation (`-expr`)

- `Unsigned(N)` -> `Signed(N+1)` — two's complement requires one extra bit
- `Signed(N)` -> `Signed(N)` — preserves width
- `Bool` -> error (E603) — use `!` for logical negation

### T10: Previous-tick (`prev(signal, delay)`)

- Preserves the declared type of the signal
- Delay is a compile-time constant (not type-checked)

### T11: Guard Conditions

- Guard conditions must evaluate to `Bool` (E601)

### T12: Literal Inference

- Boolean literals (`true`, `false`) -> `Bool`
- Integer literals -> `Unsigned(min_bits)` where min_bits is the minimum
  bit width needed to represent the value

---

## The Cross-Category Rule

The fundamental invariant: **signed and unsigned types never mix in the same
expression**. This eliminates an entire class of bugs common in C/C++ where
implicit signed-to-unsigned conversion causes unexpected behavior.

```mirr
// REJECTED — E603: cannot mix signed and unsigned
signal a: in u16;
signal b: in i16;
reflex always_on {
    // error_val = a + b;  // E603
}

// CORRECT — explicit same-category usage
signal a: in i16;
signal b: in i16;
reflex always_on {
    result = a + b;  // OK: both signed
}
```

---

## TypeMap Output

The type checker returns a `TypeMap` (`HashMap<*const Expr, SignalType>`) that
maps every visited expression node (by pointer identity) to its inferred type.
Downstream passes (width inference, R-SPU emission) can query this map to
determine signedness without re-walking expression trees.

```rust
use mirr::{run_pipeline, PipelineConfig};

let config = PipelineConfig {
    typecheck: true,
    // ... other fields
};
let result = run_pipeline(source, config)?;
let type_map = &result.type_map;  // Option<TypeMap>
```

---

## Error Codes

| Code | Rule | Description |
|------|------|-------------|
| E601 | T11  | Guard condition must be `Bool` |
| E602 | T1   | Assignment type incompatible with target signal |
| E603 | T2/T3/T9 | Arithmetic operator requires numeric operands / signed-unsigned mismatch |
| E604 | T4   | Logical operator requires `Bool` operands |
| E605 | T6   | Ordering comparison on `Bool` or signed/unsigned mismatch |
| E606 | T7   | Equality comparison across type categories |
| E607 | T5   | XOR requires matching types |

---

## Pipeline Integration

Type checking is optional and controlled by `PipelineConfig::typecheck`:

```rust
let config = PipelineConfig {
    typecheck: true,   // Enable type checking (default: false)
    // ...
};
```

When enabled, the type checker runs between semantic validation (Step 4) and
simplification (Step 6) in the pipeline. If disabled, downstream passes still
function correctly but without signedness enforcement.

---

## See Also

- [`docs/error_codes.md`](error_codes.md) — Full error code catalogue (E6xx section)
- [`docs/tutorial.md`](tutorial.md) — Lesson 8: Signed types
- `proposals/002-TYPE-001-2026-03-08.md` — TYPE-001 campaign proposal
- `proposals/003-TYPE-002-2026-03-08.md` — TYPE-002 campaign proposal
- `src/typeck/mod.rs` — Type checker implementation
