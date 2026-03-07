# Proposal: Education Campaign — Absolute Beginner Documentation

**Campaign ID:** EDU-001
**Author:** Claude (AI pair-programmer)
**Status:** PROPOSED
**Date:** 2026-03-08

---

## Problem Statement

The current documentation assumes readers already know what hardware description languages (HDL), RTL, formal verification, and SystemVerilog are. The README is well-written but skips foundational concepts: it references "shift registers," "clock cycles," "SVA assertions," and "FIRRTL" without explaining them. A reader who can write Python or JavaScript but has never touched hardware has no bridge into this project.

The primary reader for this campaign:
- Can read code (any language)
- Has **never** written Verilog, VHDL, or any HDL
- Does **not** know what RTL, FPGA, ASIC, or "clock cycle" means
- Has **never** used formal verification tools
- Wants to understand what MIRR does and why it matters

---

## Scope

Create **one** self-contained tutorial document: `docs/tutorial.md`

This document teaches everything a complete beginner needs to go from zero to writing, compiling, and understanding MIRR output. It does **not** teach Verilog or FIRRTL — it teaches MIRR by building up from first principles.

---

## Deliverables

### 1. `docs/tutorial.md` — "MIRR from Scratch"

A single progressive tutorial structured as a series of lessons. Each lesson builds on the last.

#### Lesson 1: What is hardware and why does it matter?
- What software does vs. what hardware does (with concrete analogy: smoke alarm)
- What a "clock cycle" is (a heartbeat that ticks billions of times per second)
- Why software can be late (OS scheduling, interrupts, garbage collection)
- Why hardware cannot be late (physics — wires carry signals at the speed of light)
- What "safety-critical" means (ventilators, flight controllers, brakes)

#### Lesson 2: The three building blocks
- **Signal** = a wire that carries a value (like a variable, but it's a physical wire)
- **Guard** = a rule that watches wires and fires when danger is detected (like an alarm sensor)
- **Reflex** = an automatic reaction when a guard fires (like the alarm sounding)
- Walk through `neonatal_respirator.mirr` line by line, explaining every keyword

#### Lesson 3: Your first MIRR program
- Install Rust and clone the repo (step-by-step)
- Create a new `.mirr` file from scratch
- Compile it: `cargo run --bin mirr-compile -- examples/neonatal_respirator.mirr`
- Read the output and understand what each line means
- Try breaking it on purpose — see what error messages say

#### Lesson 4: Types and expressions
- `bool` = on/off (one wire)
- `u8`, `u16` = a bundle of wires carrying a number
- Comparisons: `<`, `>`, `==`, `!=`, `<=`, `>=`
- Logic: `&&`, `||`, `!`
- Arithmetic: `+`, `-`, `*`
- `prev(signal, N)` — what the signal was N ticks ago

#### Lesson 5: Temporal guards — the key idea
- What `for N cycles` means physically (shift register diagram: ASCII art)
- Why this matters: filtering noise vs. reacting to real events
- Counter guards vs. shift register guards (the compiler chooses for you)
- Walk through the compiler output when temporal guards are used

#### Lesson 6: Properties — proving your design is correct
- What an "assertion" is (a promise about your design that the compiler checks)
- `always (P)` — P must be true on every single clock tick, forever
- `never (P)` — P must never be true on any clock tick
- `always (P -> Q)` — whenever P is true, Q must also be true
- `never (P -> Q)` — it must never be the case that P leads to Q
- `eventually within N (P)` — P must become true within N ticks
- `always (P followed_by N Q)` — whenever P is true, Q must follow within N ticks
- Directives: `assert` (default), `cover` (can this happen?), `assume` (I promise this is true about the environment)
- What SVA is (one sentence) and why MIRR compiles properties into it

#### Lesson 7: Patterns — reusable templates
- What `def` / `reflect` does
- Parameter substitution with `${param}`
- Walk through `pattern_usage.mirr`
- When to use patterns vs. copy-paste

#### Lesson 8: Reading compiler output
- The DOT graph: what nodes and edges mean
- The Verilog output: what `module`, `always_ff`, `always_comb` mean (just enough to recognize them)
- The JSON netlist: structured data for tooling
- The FIRRTL output: intermediate format for the CHIPS Alliance ecosystem
- The SVA output: standalone assertions for formal verification tools

#### Lesson 9: Common errors and what they mean
- Error code prefixes: `[E1xx]` parse, `[E2xx]` semantic, `[E3xx]` temporal, `[E4xx]` pattern
- Walk through the 8 most common errors with examples and fixes

#### Lesson 10: What MIRR does NOT do
- MIRR does not run on a CPU
- MIRR does not replace all of Verilog — it replaces the safety-critical parts
- MIRR output still needs to be integrated into a larger FPGA/ASIC design
- MIRR is not a simulator (but see MAPE-K)

### 2. Update `docs/INDEX.md`
- Add tutorial.md entry under a new "Getting Started" section at the top

### 3. Update `README.md`
- Add a "New here?" callout near the top linking to `docs/tutorial.md`
- Update test badge count from 632 to 711

---

## What this campaign does NOT do

- Does not teach Verilog, VHDL, or SystemVerilog
- Does not modify any Rust source code
- Does not add tests
- Does not change the compiler behavior
- Does not create video content or interactive notebooks

---

## Constraints

| Constraint | Rule |
|---|---|
| Audience | Can read code, knows nothing about hardware |
| Tone | Plain English, no jargon without definition |
| Examples | Every concept gets a concrete .mirr code example |
| ASCII art | Use text diagrams for shift registers, clock cycles, signal flow |
| Length | Target 800–1200 lines (comprehensive but not overwhelming) |
| Self-contained | Reader should not need to leave this document to understand it |
| No emojis | Per project style |

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Too long, reader gives up | Medium | High | Progressive structure — each lesson is self-contained, reader can stop at any point |
| Over-simplifies hardware concepts | Medium | Low | Include "Going deeper" callouts that link to real references for advanced readers |
| Gets stale as compiler changes | Low | Medium | Tutorial uses only stable, core features (signal/guard/reflex/property) |

---

## Estimated File Changes

| File | Action | Lines |
|---|---|---|
| `docs/tutorial.md` | CREATE | ~1000 |
| `docs/INDEX.md` | EDIT | +5 |
| `README.md` | EDIT | +4 (callout + badge fix) |

---

## Execution Order

| Step | Deliverable | Depends on |
|---|---|---|
| 1 | Write `docs/tutorial.md` Lessons 1–5 | — |
| 2 | Write `docs/tutorial.md` Lessons 6–10 | Step 1 |
| 3 | Update `docs/INDEX.md` | Step 2 |
| 4 | Update `README.md` | Step 2 |
| 5 | Verify all .mirr examples in tutorial compile | Steps 1–2 |
