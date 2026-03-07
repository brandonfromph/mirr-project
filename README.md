<div align="center">

# MIRR

**A hardware rule language for safety-critical systems.**  
Write a rule in plain code. Get nanosecond-speed hardware logic — no OS, no scheduler, no delays.

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%20v3-blue.svg?style=for-the-badge)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)]()
[![Tests](https://img.shields.io/badge/tests-711%20passing-brightgreen?style=for-the-badge)]()
[![Language: Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Target: Verilog RTL](https://img.shields.io/badge/Target-Verilog%20RTL-blueviolet?style=for-the-badge)]()

[Read the Design Doc](docs/roadmap.md) · [Read the Tutorial](docs/tutorial.md) · [CHANGELOG](CHANGELOG.md) · [Migration Guide](docs/migration-guide.md) · [Report a Bug](https://github.com/brandonfromph/mirr-project/issues) · [Request a Feature](https://github.com/brandonfromph/mirr-project/issues)

</div>

---

## Table of Contents

1. [What MIRR does](#what-mirr-does)
2. [Example](#example)
3. [Writing MIRR](#writing-mirr)
   * [The three concepts](#the-three-concepts)
   * [Signals](#signals)
   * [Guards](#guards)
   * [Reflexes](#reflexes)
   * [Properties](#properties)
   * [Patterns (def / reflect)](#patterns-def--reflect)
   * [Types](#types)
   * [Expressions](#expressions)
   * [A complete program](#a-complete-program)
   * [Common mistakes](#common-mistakes)
4. [Design philosophy](#design-philosophy)
5. [Built with](#built-with)
6. [Getting started](#getting-started)
   * [Prerequisites](#prerequisites)
   * [Installation](#installation)
7. [Usage](#usage)
   * [Parse a MIRR file](#parse-a-mirr-file)
   * [Compile temporal guards](#compile-temporal-guards)
   * [Simplify logic](#simplify-logic)
   * [Check bit widths](#check-bit-widths)
   * [Compile (full pipeline)](#compile-full-pipeline)
   * [Run all tests](#run-all-tests)
8. [Roadmap](#roadmap)
9. [Contributing](#contributing)
10. [License](#license)
11. [Contact & acknowledgments](#contact--acknowledgments)

---

## What MIRR does

MIRR is a language for writing hardware rules that react to real-world conditions.

You write a rule like: *"if airway pressure drops for more than a second, close the valve immediately."* MIRR compiles that into hardware logic that enforces it in nanoseconds — with no operating system, no thread scheduler, and no possibility of a missed deadline.

The target domain is safety-critical embedded hardware: ventilators, flight controllers, autonomous vehicle systems. Places where a software bug or scheduling delay is not a crash report — it is a physical consequence.

([back to top](#mirr))

---

## Example

A neonatal respirator emergency clamp:

```mirr
module neonatal_respirator {
    signal airway_pressure: in u16;
    signal clamp_valve:     out bool;

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

If `airway_pressure` stays below 50 for 1000 consecutive clock cycles, `clamp_valve` is set to `true`.

The compiler turns the `for 1000 cycles` rule into a shift register chain in hardware. There is no polling loop, no interrupt handler, no kernel — the hardware enforces the rule directly.

Running the compiler on this file produces:

```
[parse]    module neonatal_respirator
[parse]      signal airway_pressure  in  u16
[parse]      signal clamp_valve      out bool
[guard]    sustained_pressure_drop
             condition : airway_pressure < 50
             duration  : 1000 cycles
             hardware  : shift-register chain (1000 stages)
[reflex]   emergency_clamp
             trigger   : sustained_pressure_drop
             action    : clamp_valve = true
[width]    airway_pressure  u16  (16 bits — safe for values 0–65535)
[width]    cycle counter    u10  (10 bits — minimum safe width for count up to 1000)
[width]    clamp_valve      bool (1 bit)
```

([back to top](#mirr))

---

## Writing MIRR

### The three concepts

Every MIRR program is built from exactly three things:

| Concept | What it is | Real-world analogy |
|---|---|---|
| **Signal** | A named wire — carries a value in or out of the module | A sensor reading or a switch |
| **Guard** | A named condition — watches signals and fires when a rule becomes true | A circuit breaker that trips |
| **Reflex** | A named reaction — responds to a guard firing and sets an output | The breaker cutting power when it trips |

That's the whole language. Signals carry data. Guards watch for danger. Reflexes act on it.

---

### Signals

A signal is a wire. You declare it with a name, a direction (`in` or `out`), and a type.

```mirr
signal airway_pressure: in  u16;
signal clamp_valve:     out bool;
signal status_code:     out u8;
```

- `in` means the value comes from outside — a sensor, a pin, another module.
- `out` means your module sets this value — it drives a physical output.
- The type tells the hardware how wide the wire is. See [Types](#types) below.

You cannot assign to an `in` signal. You cannot read an `out` signal inside the module. This is enforced at compile time.

---

### Guards

A guard watches one or more signals and fires when a condition becomes true.

The simplest guard: fire immediately when a condition is true.

```mirr
guard pressure_too_low {
    when airway_pressure < 50;
}
```

A guard with a duration: fire only after the condition has been continuously true for N clock cycles.

```mirr
guard sustained_pressure_drop {
    when airway_pressure < 50
    for  1000 cycles;
}
```

The `for N cycles` part is the key difference between MIRR and a software `if` statement. In software, `if (pressure < 50)` fires the instant it becomes true — including noise spikes. In MIRR, `for 1000 cycles` means the condition must hold for 1000 consecutive cycles without interruption. If pressure goes back above 50 at cycle 999, the counter resets to zero.

The compiler turns `for 1000 cycles` into a 1000-stage shift register in hardware. It is not a software timer. It cannot be delayed by an interrupt or a context switch.

You can combine conditions with `&&` (and) and `||` (or):

```mirr
guard critical_combined_fault {
    when airway_pressure < 50 && battery_level < 10
    for  500 cycles;
}
```

---

### Reflexes

A reflex listens for a guard and sets output signals in response.

```mirr
reflex emergency_clamp {
    on sustained_pressure_drop {
        clamp_valve = true;
    }
}
```

When `sustained_pressure_drop` fires, `clamp_valve` is set to `true`. This happens in the same clock cycle the guard fires — no latency, no scheduling.

A reflex can set multiple outputs:

```mirr
reflex full_shutdown {
    on critical_combined_fault {
        clamp_valve    = true;
        status_code    = 0xFF;
        alarm_active   = true;
    }
}
```

A reflex can respond to multiple guards:

```mirr
reflex set_alarm {
    on sustained_pressure_drop {
        alarm_active = true;
    }
    on temperature_overrun {
        alarm_active = true;
    }
}
```

You cannot put loops, function calls, or conditionals inside a reflex. A reflex is a set of assignments — nothing more. This is intentional. The compiler maps each assignment directly to hardware logic. Loops and conditionals would produce non-synthesizable output.

---

### Properties

A property declares a safety invariant that the compiler emits as a SystemVerilog Assertion (SVA). Properties do not affect the generated hardware — they produce verification assertions that can be checked by simulation or formal tools.

Three forms are supported:

**Always** — the condition must hold at every clock cycle:

```mirr
property pressure_in_range {
    always (airway_pressure > 10);
}
```

**Never** — the condition must never be true:

```mirr
property no_spurious_clamp {
    never (clamp_valve && alarm_active == false);
}
```

**Implication** — whenever the antecedent is true, the consequent must also be true:

```mirr
property pressure_response {
    always (airway_pressure < 50 -> clamp_valve);
}
```

The compiler emits these as SVA `assert property` blocks. If the module declares a `rst_n` input signal, the assertion includes `disable iff (!rst_n)` automatically.

All signal references in a property are validated at compile time — referencing an undeclared signal is a hard error.

---

### Patterns (def / reflect)

A pattern defines a reusable hardware template that expands at compile time. Define it once with `def`, call it many times inside any module.

```mirr
def monitor_sensor(
    sensor: signal in u16,
    low:    u16,
    high:   u16,
    cycles: u32,
    alarm:  signal out bool
) {
    reflect {
        guard ${sensor}_too_low {
            when ${sensor} < ${low}
            for  ${cycles} cycles;
        }

        guard ${sensor}_too_high {
            when ${sensor} > ${high}
            for  ${cycles} cycles;
        }

        reflex ${sensor}_response {
            on ${sensor}_too_low {
                ${alarm} = true;
            }
        }

        property ${sensor}_alarm_correct {
            always (${sensor} < ${low} -> ${alarm});
        }
    }
}
```

Call the pattern inside a module — each call expands into prefixed, collision-proof hardware:

```mirr
module ventilator {
    signal airway_pressure: in  u16;
    signal heart_rate:      in  u16;
    signal pressure_alarm:  out bool;
    signal heartrate_alarm: out bool;

    monitor_sensor(airway_pressure, 50, 200, 1000, pressure_alarm);
    monitor_sensor(heart_rate, 40, 180, 500, heartrate_alarm);
}
```

Parameters can be signals (`signal in u16`, `signal out bool`) or compile-time constants (`u16`, `u32`, `bool`). The `${param}` markers are substituted before re-parsing. Each expansion gets a unique name prefix (`monitor_sensor_0_`, `monitor_sensor_1_`) and an `origin` tag for DO-178C traceability.

---

### Types

MIRR has a small, explicit type system. Every signal has a physical width. There are no implicit conversions.

| Type | Width | Range | Use for |
|---|---|---|---|
| `bool` | 1 bit | true / false | Flags, switches, valve states |
| `u8` | 8 bits | 0 – 255 | Small counters, status codes |
| `u16` | 16 bits | 0 – 65,535 | Sensor readings (pressure, temperature) |
| `u32` | 32 bits | 0 – 4,294,967,295 | Large counters, timestamps |
| `u64` | 64 bits | 0 – 2⁶⁴−1 | Maximum precision values |

Pick the narrowest type that can hold your maximum value. Wider types use more silicon area.

The compiler will reject an assignment where the source is wider than the destination:

```mirr
signal pressure: in  u16;
signal display:  out u8;

reflex bad_example {
    on some_guard {
        display = pressure;  // ERROR: u16 cannot be truncated to u8 silently
    }
}
```

This is intentional. A silent truncation in hardware is data corruption. If you genuinely need to narrow a value, that operation must be explicit — the compiler will tell you how.

---

### Expressions

Guard conditions and reflex assignments can use the following operators:

**Comparison** (for guard conditions)

```mirr
when pressure < 50        // less than
when pressure > 200       // greater than
when pressure == 100      // equal
when pressure != 100      // not equal
when pressure <= 50       // less than or equal
when pressure >= 200      // greater than or equal
```

**Logic** (combine conditions)

```mirr
when a < 50 && b < 10     // both must be true
when a < 50 || b < 10     // either must be true
when !(a < 50)            // negation
```

**Arithmetic** (for computed values in reflexes)

```mirr
status_code = pressure + offset;
status_code = raw_value * 2;
status_code = raw_value >> 1;   // right shift (divide by 2 in hardware)
```

All arithmetic uses wrapping semantics — the same as Rust's wrapping integers. If a `u8` value reaches 255 and you add 1, it wraps to 0. This matches how hardware behaves at the bit level. The bit-width checker will warn you if the result of an operation can overflow its destination.

---

### A complete program

Here is a complete, realistic MIRR program for a neonatal respiratory monitor. Read it top to bottom — it should be self-explanatory at this point.

```mirr
module neonatal_respirator {

    // ── Inputs ──────────────────────────────────────────────────────────────
    signal airway_pressure:  in u16;   // 0–65535, from pressure transducer
    signal battery_level:    in u8;    // 0–100 percent
    signal sensor_connected: in bool;  // true if sensor is physically connected

    // ── Outputs ─────────────────────────────────────────────────────────────
    signal clamp_valve:      out bool; // true = emergency closed
    signal alarm_active:     out bool; // true = audible/visual alarm
    signal status_code:      out u8;   // 0 = ok, nonzero = fault code

    // ── Guards ──────────────────────────────────────────────────────────────

    // Pressure has been dangerously low for at least 1000 cycles (1 second at 1kHz)
    guard sustained_low_pressure {
        when airway_pressure < 50
        for  1000 cycles;
    }

    // Battery is critically low and has been for 500 cycles
    guard critical_battery {
        when battery_level < 5
        for  500 cycles;
    }

    // Sensor has been disconnected for any amount of time
    guard sensor_lost {
        when sensor_connected == false
        for  10 cycles;    // small debounce window
    }

    // ── Reflexes ─────────────────────────────────────────────────────────────

    // If pressure drops: clamp the valve, sound the alarm, set fault code
    reflex emergency_pressure_response {
        on sustained_low_pressure {
            clamp_valve   = true;
            alarm_active  = true;
            status_code   = 1;
        }
    }

    // If battery is critical: sound the alarm, set a different fault code
    reflex battery_warning {
        on critical_battery {
            alarm_active  = true;
            status_code   = 2;
        }
    }

    // If sensor is lost: clamp the valve (fail safe), sound the alarm
    reflex sensor_disconnect_response {
        on sensor_lost {
            clamp_valve   = true;
            alarm_active  = true;
            status_code   = 3;
        }
    }
}
```

---

### Common mistakes

**Using `=` in a guard condition**

```mirr
// WRONG
guard bad {
    when pressure = 50;    // = is assignment, not comparison
}

// CORRECT
guard good {
    when pressure == 50;
}
```

**Assigning to an input signal**

```mirr
// WRONG — pressure is declared `in`, you cannot set it
reflex bad {
    on some_guard {
        airway_pressure = 0;
    }
}
```

**Leaving a guard with no matching reflex**

A guard with no reflex is legal but does nothing. The compiler will warn you. This is almost always a mistake.

**Using a type that is too narrow**

If your sensor can read values up to 1000, use `u16` (max 65,535), not `u8` (max 255). The compiler will catch assignments that overflow, but it will not catch a guard condition that silently never fires because the sensor value was truncated before it reached the guard.

**Expecting immediate behavior without `for N cycles`**

A guard without `for N cycles` fires the instant the condition is true — including on electrical noise spikes. For physical sensors, always use a `for` duration to debounce. Even `for 2 cycles` is better than nothing.

([back to top](#mirr))

---

## Design philosophy

MIRR exists because C++ and standard RTL are both wrong for safety-critical hardware — in opposite directions.

C++ has no native concept of hardware timing or physical bit widths. A missed deadline in C++ is a latency spike. In a ventilator, it is a patient event. Standard RTL (Verilog/VHDL) is cycle-accurate but gives you no abstraction above gate-level logic — writing and auditing safety invariants by hand in Verilog does not scale.

MIRR occupies the space between them. It enforces three guarantees at compile time:

| Constraint | How it is enforced | Why it matters |
|---|---|---|
| Temporal correctness | Guards compile to shift registers, not software timers | A `for N cycles` rule cannot be preempted or delayed |
| Bit-width safety | Width inference assigns minimum safe widths; unsafe truncation is a hard error | A 12-bit sensor value silently truncated to 8 bits is data corruption in hardware |
| No unsafe code | `#![forbid(unsafe_code)]` across all crates | Compiler cannot introduce undefined behavior |
| No unbounded loops | All passes are iterative with explicit bounds (NASA Power-of-10) | Compiler cannot hang on pathological input |
| Zero warnings | `#![deny(warnings)]` enforced in CI | Warning suppression is not permitted |

The compiler is built to the same standards as its output targets. Safety properties are not aspirational — they are enforced by the build system.

([back to top](#mirr))

---

## Built with

* [![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
* [![Clap](https://img.shields.io/badge/clap-CLI%20framework-orange?style=for-the-badge)](https://github.com/clap-rs/clap)
* [![Serde](https://img.shields.io/badge/serde-serialization-lightgrey?style=for-the-badge)](https://serde.rs/)

([back to top](#mirr))

---

## Getting started

### Prerequisites

You need the Rust toolchain. Nothing else.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify it installed:

```bash
rustc --version
cargo --version
```

### Installation

1. Clone the repository

```bash
git clone https://github.com/brandonfromph/mirr-project.git
cd mirr-project
```

2. Build all binaries

```bash
cargo build --release
```

3. Run the test suite to confirm everything is working

```bash
cargo test
```

All tests should pass with zero warnings.

([back to top](#mirr))

---

## Usage

### Parse a MIRR file

Reads a `.mirr` source file and prints the parsed structure — signals, guards, and reflexes.

```bash
cargo run --bin mirr-parse -- examples/neonatal_respirator.mirr
```

### Compile temporal guards

Takes each `for N cycles` rule and shows the hardware structure it maps to: shift register, counter-comparator, or FSM.

```bash
cargo run --bin mirr-temporal -- examples/neonatal_respirator.mirr
```

### Simplify logic

Applies algebraic simplification rules to the combinational logic and reports gate count reduction.

```bash
cargo run --bin mirr-simplify -- --stats examples/neonatal_respirator.mirr
```

### Check bit widths

Infers the minimum safe bit width for every signal and reports any unsafe truncations as hard errors.

```bash
cargo run --bin mirr-width -- examples/neonatal_respirator.mirr
```

### Compile (full pipeline)

Runs the full pipeline: parse, validate, simplify, width-infer, temporal-lower, and emit output.

```bash
# Emit SystemVerilog RTL to stdout
cargo run --bin mirr-compile -- examples/neonatal_respirator.mirr --emit verilog

# Emit Graphviz DOT to a file
cargo run --bin mirr-compile -- examples/neonatal_respirator.mirr --emit dot -o graph.dot

# Emit JSON netlist
cargo run --bin mirr-compile -- examples/neonatal_respirator.mirr --emit json

# Emit SVA assertions only (no module wrapper)
cargo run --bin mirr-compile -- examples/neonatal_respirator.mirr --emit sva

# Full AST detail in DOT output
cargo run --bin mirr-compile -- examples/neonatal_respirator.mirr --emit dot --dot-detail expr
```

### Run all tests

```bash
cargo test
```

([back to top](#mirr))

---

## Roadmap

- [x] Phase 0 — Foundation: NASA/JPL coding standards, zero warnings, zero unsafe
- [x] Phase 1 — Parser: MIRR DSL lexer, parser, typed AST (`mirr-parse`)
- [x] Phase 2 — Temporal compiler: shift registers and counter-comparators (`mirr-temporal`)
- [x] Phase 3 — Logic simplifier: 33 algebraic rules, fixpoint iteration (`mirr-simplify`)
- [x] Phase 4 — Bit-width inference: constraint propagation, SCC handling, truncation errors (`mirr-width`)
- [x] Phase 5 — MAPE-K simulation: autonomic feedback loop harness (`mirr-simulate`)
- [x] Phase 6 — Integration: unified pipeline, SystemVerilog/DOT/JSON emit (`mirr-compile`)
- [x] Phase 7a — Safety properties: `property` keyword, SVA assertion emission, JSON/DOT property support
- [x] Phase 7b — Homoiconic pattern system: `def`/`reflect` keywords, compile-time pattern expansion, origin tagging
- [ ] Phase 7 — Formal verification: Rocq proofs, verified width inference
- [ ] Phase 8 — R-SPU RTL: full Reflexive Processing Unit hardware architecture
- [ ] Phase 9 — Multi-core fabric
- [ ] Phase 10 — Production certification: DO-178C, IEC 62304, ISO 26262

See [docs/roadmap.md](docs/roadmap.md) for the full technical specification of each phase, and the [open issues](https://github.com/brandonfromph/mirr-project/issues) for known bugs and proposed features.

([back to top](#mirr))

---

## Contributing

This project is an open invitation for collaboration — particularly from researchers in formal verification, hardware synthesis, and safety-critical systems.

If you want to contribute, the best starting point is reading [docs/roadmap.md](docs/roadmap.md) to understand the architecture, then opening an issue to discuss what you want to work on before writing code.

Standard contribution flow:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -m 'Add your-feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

All contributions must pass `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` before review.

([back to top](#mirr))

---

## License

Distributed under the GPL-3.0 License. See [`LICENSE`](LICENSE) for the full terms.

([back to top](#mirr))

---

## Contact & acknowledgments

This project was conceptualized and is maintained by a first-year undergraduate student as an independent research effort. It is offered as a design exploration and an invitation for mentorship and collaboration from researchers in VLSI, formal methods, and safety-critical systems.

**Built on the work of:**
- Xiao et al. (2025) — [Cement2: Temporal Hardware Transactions](https://doi.org/10.48550/arXiv.2511.15073)
- Li et al. (2025) — [SmaRTLy: RTL Optimization with Logic Inferencing](https://doi.org/10.48550/arXiv.2510.17251)
- Wang et al. (2026) — [FIRWINE: Formally Verified Width Inference](https://doi.org/10.48550/arXiv.2601.12813)
- Pnueli, A. (1977) — [The Temporal Logic of Programs](https://doi.org/10.1109/SFCS.1977.32)

([back to top](#mirr))