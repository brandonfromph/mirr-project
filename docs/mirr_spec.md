## MIRR Language – Minimal Core Specification (Draft)

This document specifies a **minimal MIRR core** designed for reflexive clinical hardware. The goal is to keep the surface language extremely small (3 primitives) while allowing powerful compiler passes (Cement2‑style timing, SmaRTLy‑style optimization, FIRWINE‑style width inference) underneath.This project is in the (EDA) Electronic Design Automation Domain.

### 1. Design goals

- **Reflex‑first**: Express *when* and *under what clinical condition* hardware must react, not low‑level implementation detail.
- **Temporal safety**: Make timing and duration explicit, so the compiler can check and synthesize temporal guards.
- **Meta‑friendly**: MIRR code can generate or transform other MIRR modules (future work), but the core syntax stays tiny.
- **Hardware‑targeted**: Every construct must map to a finite, synthesizable structure (shift registers, counters, gates).

### 2. Core concepts

- **Signals**: Named wires or buses carrying boolean or numeric values.
- **Reflexes**: Deterministic, safety‑critical actions that must fire when a condition is satisfied.
- **Temporal guards**: Conditions over time (e.g. “signal X high for N cycles”).
- **Scopes (modules)**: Named blocks that encapsulate inputs, outputs, internal signals, and reflexes.

### 3. Core primitives

The minimal MIRR core exposes **3 top‑level primitives**:

1. `signal` – declare typed signals and their roles.
2. `guard` – specify temporal safety conditions.
3. `reflex` – bind guards to deterministic actions.

#### 3.1 `signal`

**Purpose**: Declare inputs, outputs, and internal signals with explicit type and width.

**Syntax (core idea)**:

```text
signal <name>: <kind> <type>;
```

Where:

- `<kind>` ∈ `{in, out, internal}`.
- `<type>` ∈:
  - `bool`
  - `uN` (unsigned N‑bit, e.g. `u8`, `u16`, `u32`, `u64`)
  - `iN` (signed N‑bit, e.g. `i8`, `i16`, `i32`, `i64`)

**Examples**:

```text
signal respirator_enable: in bool;
signal airway_pressure:   in u16;
signal clamp_valve:       out bool;
signal pressure_error:    internal u16;
```

#### 3.2 `guard`

**Purpose**: Define named **temporal conditions** that must be satisfied before a reflex can fire. These are the main hooks for Cement2‑style compilation.

**Syntax (core idea)**:

```text
guard <name> {
    when <boolean_expr>
    for  <cycles> cycles;
}
```

Where:

- `<boolean_expr>` is a pure boolean expression over signals and comparisons, using:
  - logical operators: `!`, `&&`, `||`
  - relational operators for numeric signals: `<`, `<=`, `>`, `>=`, `==`, `!=`
- `<cycles>` is a positive integer literal (bounded delay).

**Semantics**:

- The guard `G` is **satisfied** at time \( t \) if and only if `<boolean_expr>` has been continuously true for the previous `<cycles>` clock cycles.
- The compiler will lower this into:
  - **shift‑register structures** for short delays, and/or
  - **counter‑comparator structures** for longer delays (Adaptive Temporal Synthesis).

**Examples**:

```text
guard sustained_pressure_drop {
    when airway_pressure < 50
    for  1000 cycles;
}

guard seizure_pattern {
    when eeg_spike && !artifact_noise
    for  32 cycles;
}
```

#### 3.3 `reflex`

**Purpose**: Define **deterministic actions** that must occur when one or more guards are satisfied. These are the “hardware reflexes” for clinical safety.

**Syntax (core idea)**:

```text
reflex <name> {
    on <guard_name> [and <guard_name> ...] {
        <assignments>
    }
}
```

Assignments:

```text
<signal> = <expression>;
```

Where `<expression>` is:

- boolean algebra over `bool` signals: `!`, `&&`, `||`, `^`
- arithmetic over numeric signals: `+`, `-`, `*`, shifts `<<`, `>>`

**Semantics**:

- When the `on` condition becomes true (all referenced guards satisfied), the assignments in the block are **atomically committed**.
- The compiler ensures:
  - No combinational cycles in reflex logic.
  - Bit‑width safety of arithmetic (via width inference pass).

**Example**:

```text
reflex emergency_clamp {
    on sustained_pressure_drop {
        clamp_valve = true;
    }
}
```

### 4. Modules and full program structure

At the top level, MIRR code is organized into **modules**:

```text
module <name> {
    // 1. signal declarations
    signal ...;

    // 2. guards
    guard ... { ... }

    // 3. reflexes
    reflex ... { ... }
}
```

**Example: Neonatal Respirator Reflex Module**

```text
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
```

### 5. Compiler passes (Phase 1+ roadmap connection)

While MIRR exposes only `signal`, `guard`, and `reflex`, the Rust compiler will implement multiple internal passes:

1. **Parsing & AST / IR construction (Phase 1)**  
   - Recognize modules, signals, guards, reflexes.  
   - Build a typed IR with explicit temporal guard nodes.

2. **Temporal lowering (Cement2‑inspired, Phase 2)**  
   - Convert `guard` nodes into concrete timing hardware (shift registers vs counters).

3. **Logic simplification (SmaRTLy‑inspired, Phase 3)**  
   - Reduce boolean and arithmetic expressions in reflex bodies and guard expressions.

4. **Bit‑width inference (FIRWINE‑inspired, Phase 4)**  
   - Compute safe widths for all numeric signals and expressions.  

5. **MAPE‑K simulation (Phase 5)**  
   - Use MIRR modules as the specification for the reflex domain in software simulations.

### 6. Immediate implementation targets (Phase 1)

For Phase 1 of the project, the spec is intentionally simplified:

- Support:
  - Single module per file.
  - `signal`, `guard`, `reflex` as defined above.
  - Boolean expressions and simple integer comparisons.
  - `for <N> cycles` with compile‑time constant \( N \).
- A Rust CLI `mirr-parse` should:
  - Read a `.mirr` file.
  - Parse into an IR.
  - Pretty‑print the IR as text or JSON.
  - Exit with clear errors on syntax/typing issues.

Later phases will extend the spec (e.g., meta‑programming, multiple modules, parameterization), but this document defines the **minimal, implementable MIRR core** to start coding immediately.


src/
├── main.rs                    # Entry point — CLI only
├── lib.rs                     # Public API re-exports
├── error.rs                   # Centralized error authority
├── ast/
│   ├── mod.rs                 # Re-exports all AST types
│   ├── types.rs               # SignalKind, SignalType, BinaryOp, UnaryOp, LiteralValue
│   ├── expr.rs                # Expr enum (expression tree)
│   └── program.rs             # SignalDecl, Guard, Assignment, Reflex, Module, MirrProgram
├── lexer/
│   ├── mod.rs                 # Re-exports
│   └── tokenizer.rs           # Token enum + tokenize_expr() with performance optimizations
├── parser/
│   ├── mod.rs                 # Re-exports parse_mirr + parse_expression
│   ├── expr_parser.rs         # Pratt parser (precedence-climbing) with early validation
│   |── module_parser.rs       # Line-based module/signal/guard/reflex parser
|   └── temporal
|   
└── validation/
    ├── mod.rs                 # Re-exports
    └── semantic.rs            # validate_module + collect_signal_refs with pre-allocated collections

tests/
├── expr_tests.rs              # 17 expression parser tests
├── module_tests.rs            # 23 module parser + error tests
├── validation_tests.rs        # 9 semantic validation tests
└── stress_tests.rs            # 5 stress/edge-case tests
