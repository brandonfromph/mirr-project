---
name: audit-module
description: 'Audit a MIRR source file or module for correctness, completeness, and adherence to the three-construct philosophy. Use this when reviewing .mirr files or checking if a module compiles correctly.'
argument-hint: 'Path to .mirr file or module name (e.g., "examples/tmr_sensor_fusion.mirr")'
---

# MIRR Module Auditor

Perform a thorough audit of a MIRR module file. Check every declaration against the three-construct architecture, compile to all backends, and report findings.

## Procedure

1. **Read the .mirr file** specified in the argument.

2. **Compile it through all backends in parallel** — launch parallel agents for each:
   ```bash
   cargo run --bin mirr-compile -- --emit verilog --stats <file>
   cargo run --bin mirr-compile -- --emit firrtl <file>
   cargo run --bin mirr-compile -- --emit json <file>
   cargo run --bin mirr-compile -- --emit sva <file>
   cargo run --bin mirr-compile -- --emit testbench <file>
   ```

3. **Compile with FPGA targets** (if the module has temporal guards):
   ```bash
   cargo run --bin mirr-compile -- --emit verilog --target xilinx-7 --testbench --scaffold <file>
   cargo run --bin mirr-compile -- --emit verilog --target lattice-ice40 --scaffold <file>
   ```

4. **Check structure** — verify the module has:
   - At least one signal (input + output)
   - At least one guard with a temporal condition
   - At least one reflex triggered by a guard
   - Properties if safety assertions are needed
   - No single-writer violations (E216)
   - No signed/unsigned mismatches (E608)

5. **Run the test suite** to confirm nothing is broken:
   ```bash
   cargo test --all
   ```

6. **Report findings** in this format:

   ```markdown
   ## Module Audit: <module_name>

   | Check | Result | Notes |
   |-------|--------|-------|
   | Signals declared | pass/fail | count and types |
   | Guards well-formed | pass/fail | temporal conditions valid |
   | Reflexes trigger correctly | pass/fail | guard references valid |
   | Properties present | pass/fail | safety assertions |
   | Compiles to Verilog | pass/fail | error details if any |
   | Compiles to FIRRTL | pass/fail | error details if any |
   | Compiles to JSON | pass/fail | error details if any |
   | Testbench generates | pass/fail | DUT instantiation correct |
   | FPGA scaffold generates | pass/fail | constraints + build script |
   | No clippy warnings | pass/fail | |

   ### SystemVerilog Quality
   - [ ] clk/rst_n auto-injected (if temporal guards present)
   - [ ] Multi-guard reflexes use AND (not OR)
   - [ ] 1-cycle guards are combinational (no always_ff)
   - [ ] Default assignments prevent latch inference
   - [ ] Guard _out wires declared

   ### Recommendations
   - <actionable items>
   ```

## Philosophy Check

Verify the module respects the three-construct architecture:
- Only uses signal, guard, reflex (the three primitives)
- Properties use the six forms: always, never, always-implies, never-implies, eventually-within, always-followed-by
- No unbounded constructs
- Guard conditions use supported forms: signal, !signal, signal <op> literal, boolean AND/OR

## Guard Condition Limits

These are NOT supported in guard `when` clauses:
- Signal-to-signal comparison (`when a == b`) — use boolean flags instead
- Signal-to-signal arithmetic (`when a + b > 100`)
- Function calls

Supported forms:
- `when signal_name` (boolean test)
- `when !signal_name` (negated boolean)
- `when signal > 100` (signal vs literal comparison)
- `when signal == true` (signal vs literal equality)
- Boolean AND/OR of the above
