---
name: write-mirr
description: 'Write valid MIRR source code that follows the three-construct philosophy, valid types, temporal guard syntax, and NASA safety standards. Use this when creating or editing .mirr files.'
user-invocable: true
disable-model-invocation: false
---

# MIRR Writer

This skill knows how to write correct MIRR source code. It understands the syntax rules, the three-construct philosophy, valid types, guard semantics, and NASA Power-of-10 constraints.

## The Three-Construct Philosophy

MIRR is built on the generative power of three — inspired by Taoism: "The Tao gives birth to One. One gives birth to Two. Two gives birth to Three. Three gives birth to all things."

Three is the organizing number at every layer:

| Layer | The Three |
|-------|-----------|
| Primitives | `signal`, `guard`, `reflex` |
| Property forms | `always`, `never`, `implies` |
| Signal directions | `in`, `out`, `internal` |
| Backend engines | Verilog, FIRRTL, JSON |

The surface language stays tiny by design. Do not add a 4th to any triad. Complexity emerges from composition of three simple parts.

## Syntax Rules

### Module Declaration

Every `.mirr` file contains exactly one module:

```mirr
module <name> {
    // declarations go here
}
```

### Signals

```mirr
signal <name>: <direction> <type>;
```

- **Directions**: `in`, `out`, `internal`
- **Types**: `bool`, `u8`, `u16`, `u32`, `u64`
- No implicit casting between types
- Every signal line ends with `;`

### Guards (Temporal Conditions)

```mirr
guard <name> {
    when <boolean_expression>
    for <N> cycles;
}
```

- The `when` clause takes a boolean expression over signals
- The `for` clause specifies how many consecutive cycles the condition must hold
- Guards compile to shift registers (small N) or counters (large N)
- Expression operators: `&&`, `||`, `!`, `^`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `+`, `-`, `*`, `<<`, `>>`

### Reflexes (Reactive Assignments)

```mirr
reflex <name> {
    on <guard_name> [and <guard_name>...] {
        <signal> = <expression>;
    }
}
```

- The `on` clause references one or more guards by name
- Multiple guards use `and`: `on guard_a and guard_b { ... }`
- Assignments only write to `out` or `internal` signals — never `in`
- No loops, conditionals, or function calls inside the body
- Each assignment is `target = expression;`

### Properties (Verification Assertions)

Three forms only:

```mirr
// Invariant: P must always hold
property <name> {
    always (<expression>);
}

// Exclusion: P must never hold
property <name> {
    never (<expression>);
}

// Implication: when P holds, Q must also hold
property <name> {
    always (<antecedent> -> <consequent>);
}
```

- Properties generate SVA assertions — they do NOT affect RTL
- Properties can reference any declared signal
- Expressions in properties follow the same syntax as guard conditions

### Patterns (Reusable Templates)

```mirr
def <name>(<param>: <direction> <type>, ...) {
    // internal signal/guard/reflex declarations
    reflect {
        // body with substituted parameters
    }
}

// Call site:
<name>(<arg1>, <arg2>, ...);
```

## Valid Expression Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `&&` | Logical AND | `a && b` |
| `\|\|` | Logical OR | `a \|\| b` |
| `!` | Logical NOT | `!a` |
| `^` | XOR | `a ^ b` |
| `==` | Equal | `a == 0` |
| `!=` | Not equal | `a != 0` |
| `<` | Less than | `a < 100` |
| `<=` | Less or equal | `a <= 100` |
| `>` | Greater than | `a > 100` |
| `>=` | Greater or equal | `a >= 100` |
| `+` | Add | `a + b` |
| `-` | Subtract | `a - b` |
| `*` | Multiply | `a * b` |
| `<<` | Shift left | `a << 2` |
| `>>` | Shift right | `a >> 2` |

## NASA Power-of-10 Constraints

When writing MIRR source:
- Guard cycle counts must be finite positive integers — no unbounded monitoring
- Signal widths must be explicitly typed — no inference at the language level
- Module names, signal names, guard names, reflex names must be non-empty identifiers
- Lines starting with `//` are comments and are stripped during parsing

## Complete Example

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

    property valve_bounded {
        always (airway_pressure < 50 -> clamp_valve == true);
    }
}
```

## Verification

After writing a `.mirr` file, verify it compiles:

```bash
# Compile to all backends
cargo run --bin mirr-compile -- --emit verilog <file.mirr>
cargo run --bin mirr-compile -- --emit firrtl <file.mirr>
cargo run --bin mirr-compile -- --emit json <file.mirr>
```
