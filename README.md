<div align="center">

# MIRR

**A hardware rule language for safety-critical systems.**

[![Build](https://img.shields.io/github/actions/workflow/status/brandonfromph/mirr-project/ci.yml?branch=main&style=flat-square)](https://github.com/brandonfromph/mirr-project/actions)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square)](https://www.rust-lang.org/)

</div>

---

## What MIRR does

MIRR is a language for writing hardware rules that react to real-world conditions.

You write a rule like: *"if airway pressure drops for more than a second, close the valve immediately."* MIRR compiles that into hardware logic that enforces it in nanoseconds — with no operating system, no thread scheduler, and no possibility of a missed deadline.

The target domain is safety-critical embedded hardware: ventilators, flight controllers, autonomous vehicle systems. Places where a software bug or scheduling delay is not a crash report — it is a physical consequence.

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

---

## Design philosophy

MIRR occupies the space between C++ (no hardware timing primitives) and raw RTL (no safety abstractions). It enforces these guarantees at compile time:

| Constraint | How it is enforced |
|---|---|
| Temporal correctness | Guards compile to shift registers, not software timers |
| Bit-width safety | Width inference assigns minimum safe widths; unsafe truncation is a hard error |
| No unsafe code | `#![forbid(unsafe_code)]` across all crates |
| No unbounded loops | All passes are iterative with explicit bounds (NASA Power-of-10) |
| Zero warnings | `#![deny(warnings)]` enforced in CI |

The language has exactly three constructs — **Signal**, **Guard**, **Reflex** — plus verification **Properties** and reusable **Patterns**. See the [Tutorial](docs/tutorial.md) for the full language guide.

---

## Living Research Artifact

MIRR is published as a Living Research Artifact (LRA) — an interactive
paper where the compiler runs live in the browser.

**[Read the interactive paper](https://brandonfromph.github.io/mirr-project/paper/)**

The paper, the compiler, the Coq proofs, and the browser demos are one
GPL-3.0 licensed artifact. The paper cannot be separated from the code
and submitted to a journal under a Copyright Transfer Agreement without
violating the GPL already granted to the public.

To cite MIRR, use [`CITATION.cff`](CITATION.cff) or cite the commit
hash of the version you used.

To verify claims locally:

```bash
# Build and run the compiler
cargo run --bin mirr-compile -- --emit verilog examples/tmr_sensor_fusion.mirr

# Build the wasm demo
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build crates/mirr-wasm --target web --out-dir ../../demos --release

# Serve locally
cd paper && python3 -m http.server 8080
```

---

## Getting started

### Prerequisites

You need the Rust toolchain. Nothing else.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

```bash
git clone https://github.com/brandonfromph/mirr-project.git
cd mirr-project
cargo build --release
cargo test   # all tests should pass with zero warnings
```

---

## Usage

```bash
# Simplify logic
cargo run --bin mirr-simplify -- --stats examples/neonatal_respirator.mirr

# Check bit widths
cargo run --bin mirr-width -- examples/neonatal_respirator.mirr

# Full pipeline — emit SystemVerilog RTL
cargo run --bin mirr-compile -- examples/neonatal_respirator.mirr --emit verilog

# Other emit formats: json, dot, sva, firrtl, rspu
cargo run --bin mirr-compile -- examples/neonatal_respirator.mirr --emit json
```

---

## Documentation

| Document | Description |
|----------|-------------|
| [Tutorial](docs/tutorial.md) | 10-lesson beginner guide — no hardware experience needed |
| [Error Codes](docs/error_codes.md) | Complete catalogue of compiler diagnostics (E1xx–E7xx) |
| [Type System](docs/type-system.md) | Signed/unsigned types, width inference, and error codes |
| [R-SPU ISA Spec](docs/rspu_isa_spec.md) | R-SPU instruction set architecture and register file |
| [Migration Guide](docs/migration-guide.md) | Upgrade notes for 0.1.0 to 0.2.0 |
| [Roadmap](docs/roadmap.md) | Phase 0–10 project roadmap |
| [CHANGELOG](CHANGELOG.md) | Versioned change history |

---

## Roadmap

- [x] Phase 0–6 — Foundation through integration pipeline
- [x] Phase 7a — Safety properties and SVA assertion emission
- [x] Phase 7b — Homoiconic pattern system (`def`/`reflect`)
- [x] Phase 7 — Formal verification (Rocq proofs)
- [x] Phase 8 — R-SPU instruction set architecture
- [ ] Phase 9 — Multi-core fabric
- [ ] Phase 10 — Production certification (DO-178C, IEC 62304, ISO 26262)

See [docs/roadmap.md](docs/roadmap.md) for the full technical specification of each phase.

---

## Contributing

Contributions are welcome — particularly from researchers in formal verification, hardware synthesis, and safety-critical systems. Read [docs/roadmap.md](docs/roadmap.md) to understand the architecture, then open an issue to discuss before writing code.

1. Fork the project
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes
4. Open a Pull Request

All contributions must pass `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check`.

---

## License

Distributed under the GPL-3.0 License. See [`LICENSE`](LICENSE) for the full terms.

---

## Contact & acknowledgments

This project was conceptualized and is maintained by a first-year undergraduate student as an independent research effort. It is offered as a design exploration and an invitation for mentorship and collaboration from researchers in VLSI, formal methods, and safety-critical systems.

**Built on the work of:**
- Xiao et al. (2025) — [Cement2: Temporal Hardware Transactions](https://doi.org/10.48550/arXiv.2511.15073)
- Li et al. (2025) — [SmaRTLy: RTL Optimization with Logic Inferencing](https://doi.org/10.48550/arXiv.2510.17251)
- Wang et al. (2026) — [FIRWINE: Formally Verified Width Inference](https://doi.org/10.48550/arXiv.2601.12813)
- Pnueli, A. (1977) — [The Temporal Logic of Programs](https://doi.org/10.1109/SFCS.1977.32)
