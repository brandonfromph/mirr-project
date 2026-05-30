## MIRR Language – Minimal Core Specification (Draft)

> **Deprecated.** This spec describes the Phase 1 minimal core only.
> For the current language, see [Tutorial](tutorial) and [Type System](type-system).
> Current MIRR includes signed types (`iN`), `property` declarations, `def`/`reflect`
> pattern system, `prev()` temporal back-references, and 9 type error codes (E601–E609).

This document specifies a **minimal MIRR core** designed for reflexive clinical hardware. The goal is to keep the surface language extremely small (3 primitives) while allowing powerful compiler passes (Cement2‑style timing, SmaRTLy‑style optimization, FIRWINE‑style width inference) underneath. This project is in the (EDA) Electronic Design Automation Domain.

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
signals {
    respirator_enable: in bool
    airway_pressure:   in u16
    clamp_valve:       out bool
    pressure_error:    internal u16
}
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
    signals {
        ...
    }

    // 2. pattern calls
    calls {
        pattern_name(...)
    }

    // 3. guards
    guard ... { ... }

    // 4. reflexes
    reflex ... { ... }
}
```

**Example: Neonatal Respirator Reflex Module**

```text
module neonatal_respirator {
    signals {
        respirator_enable: in bool
        airway_pressure:   in u16
        clamp_valve:       out bool
    }

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

1. **Parsing & ECS Flat Array construction (Phase 1)**  
   - Recognize modules, signals, guards, reflexes.  
   - Populate pure flat-data arrays directly in the ECS registry.

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


