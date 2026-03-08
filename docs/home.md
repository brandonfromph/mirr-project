---
title: Home
nav_order: 0
permalink: /
---

# MIRR

**A hardware rule language for safety-critical systems.**

Write a rule in plain code. Get nanosecond-speed hardware logic -- no OS,
no scheduler, no delays.

---

## What is MIRR?

MIRR is a language for writing hardware rules that react to real-world
conditions. You describe what the hardware should do using three constructs:

- **Signal** -- a wire carrying data
- **Guard** -- a condition that watches signals over time
- **Reflex** -- an action that fires when a guard triggers

The compiler turns your rules into synthesizable Verilog RTL that enforces
them in hardware, with nanosecond response times.

## Quick example

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

If `airway_pressure` stays below 50 for 1000 consecutive clock cycles,
`clamp_valve` is set to `true` -- enforced directly in hardware.

## Documentation

| Document | Description |
|----------|-------------|
| [Tutorial](tutorial) | 10-lesson beginner guide -- no hardware experience needed |
| [Error Codes](error_codes) | Complete catalogue of compiler diagnostics |
| [Type System](type-system) | Signed/unsigned types, width inference, and error codes |
| [R-SPU Reference](rspu-reference) | R-SPU instruction set architecture and register file |
| [Migration Guide](migration-guide) | Upgrade notes for 0.1.0 to 0.2.0 |
| [Roadmap](roadmap) | Phase 0-10 project roadmap |

## Getting started

```bash
# Clone and build
git clone https://github.com/brandonfromph/mirr-project.git
cd mirr-project
cargo build

# Compile an example
cargo run --bin mirr-compile -- --emit verilog examples/neonatal_respirator.mirr
```

## License

Distributed under the GPL-3.0 License. See [LICENSE](../LICENSE) for details.
