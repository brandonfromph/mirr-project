---
title: Symbolic Analysis Guide
nav_order: 15
---

# Symbolic Analysis Guide

The MIRR compiler includes a symbolic analysis engine that performs abstract
interpretation to detect width violations and refine signal bounds.

## Overview

The symbolic engine uses interval-based abstract interpretation (Cousot & Cousot 1977)
to compute abstract values for signals without executing the hardware.

## Abstract Domain

| Value | Meaning |
|-------|---------|
| `Concrete(v)` | Exactly value `v` |
| `Interval { lo, hi }` | Any value in `[lo, hi]` |
| `Unknown { width }` | Any value of given width |
| `Top` | Any value (no information) |

## Usage

```rust
use mirrc::symbolic::{sym_eval_expr, SymState, SymValue};

let mut state = SymState::new();
state.signals.push(("x".to_string(), SymValue::Concrete(42)));

let expr = /* expression to evaluate */;
let result = sym_eval_expr(&expr, &state);
```

## Transfer Functions

### Binary Operations
- Both `Concrete`: compute exactly
- Either `Top`: result is `Top`
- Either `Unknown`: widen to `Unknown { width: 64 }`
- Interval involved: delegate to interval arithmetic

### Unary Operations
- `Not` on `Concrete`: bitwise complement
- `Negate` on `Concrete`: widen to `Unknown { width: 64 }`

## Widening Operator

The widening operator ensures convergence of fixpoint iterations:
- Same values: stable (return old)
- Different `Concrete`: widen to `Interval`
- Different `Interval`: widen to `Unknown`

## Refinement Checking

```rust
use mirrc::symbolic::sym_check_refinement;

// Check if value provably fits in [0, 255]
let fits = sym_check_refinement(SymValue::Concrete(100), 0, 255); // true
let exceeds = sym_check_refinement(SymValue::Concrete(300), 0, 255); // false
```

## See Also

- [Type System](type-system) — Type rules
- [Width Inference](phase2_temporal_guard_compiler) — Width solver