---
title: Home
nav_order: 0
permalink: /
---

<div class="mirr-hero" markdown="0">
  <img src="assets/images/mirr_logo.svg"
       alt="MIRR ouroboros logo" class="mirr-hero-logo">
  <h1 class="mirr-hero-title">MIRR</h1>
  <p class="mirr-tagline">A hardware rule language for safety-critical systems.</p>
  <p class="mirr-subline">
    Write a rule in plain code. Get nanosecond-speed hardware logic &mdash;
    no OS, no scheduler, no delays.
  </p>
  <div class="mirr-status-bar">
    <span><span class="mirr-status-dot green"></span> Compiler operational</span>
    <span><span class="mirr-status-dot blue"></span> 10 emit targets</span>
    <span><span class="mirr-status-dot amber"></span> v0.3.0</span>
  </div>
</div>

## The Three Primitives

MIRR programs are built from exactly three constructs. Every safety rule
you write uses only these:

<div class="mirr-features" markdown="0">
  <div class="mirr-feature-card">
    <h3><code>signal</code> &mdash; The Wire</h3>
    <p>A named data path carrying a typed value every clock cycle.
    Signals are the inputs and outputs of your safety logic.</p>
  </div>
  <div class="mirr-feature-card">
    <h3><code>guard</code> &mdash; The Watcher</h3>
    <p>A temporal condition that monitors signals over time. Guards
    count consecutive cycles a condition holds before triggering.</p>
  </div>
  <div class="mirr-feature-card">
    <h3><code>reflex</code> &mdash; The Responder</h3>
    <p>An action that fires when a guard triggers. Reflexes are the
    only way to drive output signals in hardware.</p>
  </div>
</div>

The compiler turns your rules into synthesizable Verilog RTL that enforces
them in hardware, with nanosecond response times.

## Quick Example

```mirr
module neonatal_respirator {
    signals {
        airway_pressure: in u16
        clamp_valve:     out bool
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

If `airway_pressure` stays below 50 for 1000 consecutive clock cycles,
`clamp_valve` is set to `true` -- enforced directly in hardware.

## Documentation

| Document | Description |
|----------|-------------|
| [Tutorial](tutorial) | 10-lesson beginner guide -- no hardware experience needed |
| [Error Codes](error_codes) | Complete catalogue of compiler diagnostics |
| [Type System](type-system) | Signed/unsigned types, width inference, and error codes |
| [R-SPU Reference](rspu_isa_spec) | R-SPU instruction set architecture and register file |
| [Migration Guide](migration-guide) | Upgrade notes for 0.1.0 through 0.3.0 |
| [Roadmap](roadmap) | Phase 0-10 project roadmap |
| [Glossary](glossary) | Project terminology and acronyms |
| [Contributing](contributing) | Coding standards, workflow, error allocation |
| [FPGA Targets Guide](fpga-targets-guide) | FPGA toolchain, synthesis, and target configuration |
| [MAPE-K Guide](mape-k-guide) | Autonomic feedback loop simulator and LTL monitoring |
| [S-Expression Guide](sexpr-guide) | Homoiconic S-expression IR, round-trip invariant |
| [Documentation Index](doc-index) | Canonical index for all project docs |

## Getting Started

```bash
# Clone and build
git clone https://github.com/brandonfromph/mirr-project.git
cd mirr-project
cargo build

# Compile an example
cargo run --bin mirr-compile -- --emit verilog examples/neonatal_respirator.mirr
```

> **Tip:** MIRR requires no hardware experience. Start with the [Tutorial](tutorial) for a
> step-by-step introduction to signals, guards, and reflexes.

## License

Distributed under the Apache-2.0 License. See [LICENSE](https://github.com/brandonfromph/mirr-project/blob/main/LICENSE) for details.
