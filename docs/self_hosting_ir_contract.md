# MIRR Self-Hosting IR Contract

**Version:** 1.0-draft  
**Status:** Frozen for self-hosting milestone v1  
**Purpose:** Canonical AST + HIR data contracts that both the Rust reference implementation and MIRR compiler modules must produce identically.

---

## 1. Introduction

This document defines the **intermediate representation (IR) contracts** for the MIRR self-hosting pipeline. Both pipelines — the Rust reference compiler and the MIRR-in-MIRR compiler — must serialize their internal IR to JSON using exactly the structure defined here.

Canonical JSON serialization enables:
- Differential testing (fixture comparison).
- Schema validation (CI gate).
- Debugging and visualization.
- Stable ABI between compiler stages.

Refer to `docs/schemas/` for machine-readable JSON Schema files.

---

## 2. Contract Levels

### Level 1 — AST (Abstract Syntax Tree)
The direct output of the parser. Preserves all syntactic information needed for the next pass.

### Level 2 — HIR (High-level IR)
The output of the semantic validation pass. Adds resolved types and links. Used by the temporal lowering pass.

### Level 3 — Temporal Netlist IR
The output of the temporal guard lowering pass. Maps to concrete hardware primitives (shift registers, counters).

---

## 3. AST Contract (Level 1)

### 3.1 Root — `MirrProgram`

```json
{
  "$schema": "docs/schemas/mirr_ast.schema.json",
  "ir_version": "1.0",
  "module": { ... }
}
```

### 3.2 `Module`

```json
{
  "name": "<string>",
  "signals": [ <SignalDecl>... ],
  "guards": [ <Guard>... ],
  "reflexes": [ <Reflex>... ]
}
```

### 3.3 `SignalDecl`

```json
{
  "name": "<string>",
  "kind": "Input" | "Output" | "Internal",
  "ty": <SignalType>
}
```

### 3.4 `SignalType`

```json
"Bool"

// or for unsigned integers:
{ "Unsigned": <integer> }
```

### 3.5 `Guard`

```json
{
  "name": "<string>",
  "condition": <Expr>,
  "cycles": <integer>
}
```

### 3.6 `Reflex`

```json
{
  "name": "<string>",
  "guard_names": [ "<string>"... ],
  "assignments": [ <Assignment>... ]
}
```

### 3.7 `Assignment`

```json
{
  "target": "<string>",
  "value": <Expr>
}
```

### 3.8 `Expr`

Expressions are tagged objects:

```json
// Literal boolean
{ "Literal": { "Bool": true } }
{ "Literal": { "Bool": false } }

// Literal integer
{ "Literal": { "Integer": 42 } }

// Signal reference
{ "Signal": "<name>" }

// Unary expression
{
  "Unary": {
    "op": "Not",
    "operand": <Expr>
  }
}

// Binary expression
{
  "Binary": {
    "op": "<BinaryOp>",
    "left": <Expr>,
    "right": <Expr>
  }
}
```

### 3.9 `BinaryOp` Enum Values

| JSON string | Meaning        |
|-------------|----------------|
| `"And"`     | Logical AND `&&` |
| `"Or"`      | Logical OR `\|\|` |
| `"Xor"`     | Bitwise XOR `^` |
| `"Lt"`      | Less-than `<`  |
| `"Le"`      | Less-or-equal `<=` |
| `"Gt"`      | Greater-than `>` |
| `"Ge"`      | Greater-or-equal `>=` |
| `"Eq"`      | Equality `==`  |
| `"Ne"`      | Not-equal `!=` |
| `"Add"`     | Addition `+`   |
| `"Sub"`     | Subtraction `-` |
| `"Mul"`     | Multiplication `*` |
| `"Shl"`     | Left shift `<<` |
| `"Shr"`     | Right shift `>>` |

### 3.10 `UnaryOp` Enum Values

| JSON string | Meaning     |
|-------------|-------------|
| `"Not"`     | Negation `!` |

---

## 4. HIR Contract (Level 2)

The HIR extends the AST with semantic annotations. It is produced after the semantic validation pass.

### 4.1 Root — `MirrHir`

```json
{
  "$schema": "docs/schemas/mirr_hir.schema.json",
  "ir_version": "1.0",
  "module": { ... },
  "signal_table": { "<name>": <ResolvedSignal>... },
  "guard_table": { "<name>": <ResolvedGuard>... }
}
```

### 4.2 `ResolvedSignal`

```json
{
  "name": "<string>",
  "kind": "Input" | "Output" | "Internal",
  "ty": <SignalType>,
  "writable": true | false
}
```

### 4.3 `ResolvedGuard`

```json
{
  "name": "<string>",
  "condition": <Expr>,
  "cycles": <integer>,
  "condition_signals": [ "<string>"... ]
}
```

The `condition_signals` field is the resolved set of signal names referenced by the guard condition — used by the temporal lowering pass.

---

## 5. Temporal Netlist IR Contract (Level 3)

The temporal netlist IR is the output of the temporal guard lowering pass. This is already serialized to JSON by the Rust implementation (via `serde_json`); this contract formalizes the expected shape.

### 5.1 Root — `TemporalNetlist`

```json
{
  "$schema": "docs/schemas/mirr_temporal_netlist.schema.json",
  "ir_version": "1.0",
  "guards": [ <CompiledGuard>... ],
  "signals": [ <GeneratedSignal>... ],
  "statistics": <CompilationStatistics>
}
```

### 5.2 `CompiledGuard`

Either a `ShiftRegister` or `Counter` variant (tagged):

```json
// Shift register guard (N ≤ 16 cycles)
{
  "ShiftRegister": {
    "name": "<string>",
    "input_signal": "<string>",
    "output_signal": "<string>",
    "stages": [ "<string>"... ],
    "delay_cycles": <integer>,
    "condition_kind": <ConditionKind>
  }
}

// Counter guard (N > 16 cycles)
{
  "Counter": {
    "name": "<string>",
    "input_signal": "<string>",
    "output_signal": "<string>",
    "counter_signal": "<string>",
    "comparator_signal": "<string>",
    "target_count": <integer>,
    "condition_kind": <ConditionKind>
  }
}
```

### 5.3 `ConditionKind`

```json
// Simple signal: when <signal>
{ "SimpleSignal": "<name>" }

// Negated signal: when !<signal>
{ "NegatedSignal": "<name>" }

// Comparison: when <signal> <op> <literal>
{
  "Comparison": {
    "signal": "<name>",
    "op": "<BinaryOp>",
    "value": { "Bool": true } | { "Integer": 42 }
  }
}
```

### 5.4 `GeneratedSignal`

```json
{
  "name": "<string>",
  "ty": <SignalType>,
  "kind": "ShiftRegisterStage" | "Counter" | "Comparator" | "LogicGate" | "Intermediate",
  "source": null | <Expr>
}
```

### 5.5 `CompilationStatistics`

```json
{
  "shift_registers_used": <integer>,
  "counters_used": <integer>,
  "logic_gates_used": <integer>,
  "max_delay_cycles": <integer>,
  "total_signals": <integer>,
  "compilation_time_us": null | <integer>
}
```

---

## 6. Serialization Rules

The following rules ensure both pipelines produce identical artifacts:

1. **Field ordering:** Fields within each object must appear in the order defined in this document. (Rust serde produces alphabetical order by default; MIRR serializer must match.)
2. **No extra fields:** Neither pipeline may emit fields not listed here.
3. **Integer encoding:** All integers are JSON numbers (no string encoding).
4. **Null handling:** Optional fields use JSON `null` when absent (no field omission).
5. **String encoding:** All strings are UTF-8 JSON strings.
6. **Enum tags:** Enum variants are encoded as tagged objects (`{"VariantName": ...}` or `"VariantName"` for unit variants), matching Rust `serde` default behavior.
7. **Array ordering:** All arrays preserve insertion / declaration order from the source file.

---

## 7. Canonical Example

Input MIRR source:
```mirr
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

Expected AST JSON output:
```json
{
  "ir_version": "1.0",
  "module": {
    "name": "neonatal_respirator",
    "signals": [
      { "name": "respirator_enable", "kind": "Input",  "ty": "Bool" },
      { "name": "airway_pressure",   "kind": "Input",  "ty": { "Unsigned": 16 } },
      { "name": "clamp_valve",       "kind": "Output", "ty": "Bool" }
    ],
    "guards": [
      {
        "name": "sustained_pressure_drop",
        "condition": {
          "Binary": {
            "op": "Lt",
            "left":  { "Signal": "airway_pressure" },
            "right": { "Literal": { "Integer": 50 } }
          }
        },
        "cycles": 1000
      }
    ],
    "reflexes": [
      {
        "name": "emergency_clamp",
        "guard_names": [ "sustained_pressure_drop" ],
        "assignments": [
          {
            "target": "clamp_valve",
            "value": { "Literal": { "Bool": true } }
          }
        ]
      }
    ]
  }
}
```

Expected Temporal Netlist JSON output:
```json
{
  "ir_version": "1.0",
  "guards": [
    {
      "Counter": {
        "name": "sustained_pressure_drop",
        "input_signal": "airway_pressure",
        "output_signal": "sustained_pressure_drop_out",
        "counter_signal": "sustained_pressure_drop_counter",
        "comparator_signal": "sustained_pressure_drop_cmp",
        "target_count": 1000,
        "condition_kind": {
          "Comparison": {
            "signal": "airway_pressure",
            "op": "Lt",
            "value": { "Integer": 50 }
          }
        }
      }
    }
  ],
  "signals": [
    {
      "name": "sustained_pressure_drop_counter",
      "ty": { "Unsigned": 11 },
      "kind": "Counter",
      "source": null
    },
    {
      "name": "sustained_pressure_drop_cmp",
      "ty": "Bool",
      "kind": "Comparator",
      "source": null
    },
    {
      "name": "sustained_pressure_drop_out",
      "ty": "Bool",
      "kind": "LogicGate",
      "source": null
    }
  ],
  "statistics": {
    "shift_registers_used": 0,
    "counters_used": 1,
    "logic_gates_used": 1,
    "max_delay_cycles": 1000,
    "total_signals": 3,
    "compilation_time_us": null
  }
}
```

---

## 8. Schema Test Requirements

Schema tests in `tests/self_hosting_ir_schema_tests.rs` must verify:

1. The Rust pipeline produces JSON matching the canonical example exactly.
2. All golden fixture files in `tests/fixtures/ast/` and `tests/fixtures/netlist/` round-trip through parse → emit → compare without modification.
3. Any field missing from or added to the Rust output (not in contract) is a test failure.

---

## 9. Parity Rules

The parity gate in `tests/self_hosting_parity_tests.rs` enforces:

- **Byte-stable parity:** Rust AST JSON == MIRR AST JSON for all fixtures.
- **Byte-stable netlist parity:** Rust netlist JSON == MIRR netlist JSON for all fixtures.
- **Semantic-equivalence fallback:** If byte parity is too strict (e.g., floating-point field ordering), a semantic comparator may be used — but must be documented.

---

## 10. Versioning

- `ir_version` in output JSON tracks this contract document's version.
- Any schema change breaks all parity tests and requires a version bump.
- Both Rust and MIRR implementations must check `ir_version` at runtime and reject mismatched versions.

---

*End of MIRR Self-Hosting IR Contract v1.0-draft*