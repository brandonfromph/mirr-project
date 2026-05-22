---
title: Type System
nav_order: 6
---

# MIRR Type System Reference

> **Status:** Active
> **MEGA-1 Extended:** Refinement types, linear types, effect types, clock domains, phantom tags — all wired into the pipeline via `typecheck_extended()`. Opt-in with `PipelineConfig::extended_typecheck`.
> **Module:** `src/typeck/mod.rs`
> **Campaigns:** TYPE-001 through TYPE-005, FOUNDATION-001, MEGA-1, MEGA-1b
> **Error codes:** E601–E625

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
    signals {
        sensor: in u16
        offset: in i16
        alarm:  out bool
        error_code: out u8
    }
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
- Both operands must be the same category (both signed or both unsigned — E608)
- Result width = `max(left_width, right_width)`, preserving signedness

### T3: Shift Operators (`<<`, `>>`)

- Both operands must be numeric (not Bool — E603)
- Both must be same category (signed/unsigned — E608)
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

{: .warning }
> Negating an unsigned value produces a signed result one bit wider:
> `-(u8)` yields `i9`. This is intentional -- two's complement negation
> requires the sign bit. Plan your signal widths accordingly.

- `Unsigned(N)` -> `Signed(N+1)` — two's complement requires one extra bit
- `Signed(N)` -> `Signed(N)` — preserves width
- `Bool` -> error (E609) — use `!` for logical negation

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

{: .important }
> Signed and unsigned types never mix in the same expression. There is no
> implicit conversion. This eliminates an entire class of bugs common in
> C/C++ where signed-to-unsigned promotion causes unexpected behavior.

The fundamental invariant: **signed and unsigned types never mix in the same
expression**. This eliminates an entire class of bugs common in C/C++ where
implicit signed-to-unsigned conversion causes unexpected behavior.

```mirr
// REJECTED — E608: cannot mix signed and unsigned
signals {
    a: in u16
    b: in i16
}
reflex always_on {
    // error_val = a + b;  // E608
}

// CORRECT — explicit same-category usage
signals {
    a: in i16
    b: in i16
}
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

The TypeMap is now wired into the compilation pipeline and will be consumed
by width inference in a future update (proposal 025, Section F). An extended
type map (`ExtendedTypeMap`) is defined in `src/typeck/extended.rs` for
MEGA-1 programs.

---

## Error Codes

| Code | Rule | Description |
|------|------|-------------|
| E601 | T11  | Guard condition must be `Bool` |
| E602 | T1   | Assignment type incompatible with target signal |
| E603 | T2/T3 | Arithmetic operator requires numeric operands (not Bool) |
| E604 | T4   | Logical operator requires `Bool` operands |
| E605 | T6   | Ordering comparison on `Bool` or signed/unsigned mismatch |
| E606 | T7   | Equality comparison across type categories |
| E607 | T5   | XOR requires matching types |
| E608 | T2/T3 | Arithmetic operator cannot mix signed and unsigned operands |
| E609 | T9   | Arithmetic negation cannot be applied to `Bool` |

---

## Pipeline Integration

Type checking is optional and controlled by `PipelineConfig::typecheck`:

```rust
let config = PipelineConfig {
    typecheck: true,   // Enable type checking (default: true)
    // ...
};
```

When enabled, the type checker runs between semantic validation (Step 4) and
simplification (Step 6) in the pipeline. If disabled, downstream passes still
function correctly but without signedness enforcement.

---

## MEGA-1 Type Annotations (Phase 7c -- Complete)

> **Status:** Complete (all sections of proposals 025 + 027 delivered). Extended type checker wired into pipeline.
> **Module:** `src/typeck/extended.rs` (2138 lines)
> **Error codes:** E610--E625

MIRR's type system is being extended with compile-time annotations that
encode hardware constraints directly in the type language. All annotations
are opt-in -- existing programs remain valid. Annotations generate zero
additional hardware; they are checked at compile time only.

### Type Annotation Syntax

Signal declarations now support optional qualifier and constraint syntax:

```mirr
signal sensor: in linear u16 where 0..1023 @clk_fast #Sensor;
```

Grammar:
```text
<kind> [linear] [stateful|pure] <base_type> [where <refinement>] [@<clock>] [#<Tag>]
```

### Linearity

`linear` signals must be consumed exactly once per clock cycle. This
prevents the hardware anti-pattern of multiple drivers on a single net.

### Effect Qualification

- `stateful` -- signal carries state across clock cycles (implies register)
- `pure` -- signal is purely combinational (no register inference)

### Refinement Types

`where` clauses constrain signal value ranges at compile time:
- `where 0..1023` -- inclusive range
- `where value < 1024` -- predicate form

### Clock Domain Qualifiers

`@domain` annotations tag signals with their clock domain. Cross-domain
assignments without explicit synchronizers produce E618.

### Phantom Tags

`#Tag` annotations add zero-cost provenance tracking. Mismatched phantom
tags in assignments produce E620.

### Extended Error Codes (E610--E625)

| Code | Rule | Description |
|------|------|-------------|
| E610 | REF-BOUND | Refinement lower bound exceeds upper bound |
| E611 | REF-RANGE | Value outside refinement range at compile time |
| E612 | REF-WIDTH | Refinement bound exceeds declared bit-width capacity |
| E613 | LIN-UNUSED | Linear signal declared but never consumed |
| E614 | LIN-DOUBLE | Linear signal consumed more than once |
| E615 | LIN-ESCAPE | Linear signal escapes owning module |
| E616 | EFF-PURE | Stateful operation in pure context |
| E617 | EFF-MIX | Stateful signal used in pure expression |
| E618 | CLK-CROSS | Clock domain crossing without synchronizer |
| E619 | CLK-UNDEF | Reference to undeclared clock domain |
| E620 | PHT-MISMATCH | Phantom tag mismatch |
| E621 | PHT-UNDEF | Reference to undeclared phantom tag |
| E622 | NAT-OVERFLOW | Type-level natural exceeds MAX_TYPE_NAT |
| E623 | NAT-MISMATCH | Dimension mismatch |
| E624 | DEP-MISMATCH | Dependent type parameter mismatch |
| E625 | SES-PROTOCOL | Session type protocol violation |

---

## See Also

- [Error Codes](error_codes) — Full error code catalogue (E6xx section)
- [Tutorial](tutorial.md) — Lesson 8: Reading compiler output
- [R-SPU ISA Spec](rspu_isa_spec.md) — How types map to R-SPU registers
- [Roadmap](roadmap) — TYPE campaigns overview
