---
name: audit-module
description: 'Audit a MIRR source file or module for correctness, completeness, and adherence to the three-construct philosophy. Use this when reviewing .mirr files or checking if a module compiles correctly.'
argument-hint: 'Path to .mirr file or module name (e.g., "examples/neonatal_respirator.mirr")'
---

# MIRR Module Auditor

Perform a thorough audit of a MIRR module file. Check every declaration against the three-construct architecture and report findings.

## Procedure

1. **Read the .mirr file** specified in the argument.

2. **Compile it** through the full pipeline:
   ```bash
   cargo run --bin mirr-compile -- --emit verilog --stats <file>
   ```

3. **Check structure** — verify the module has:
   - At least one signal (input + output)
   - At least one guard with a temporal condition
   - At least one reflex triggered by a guard
   - Properties if safety assertions are needed

4. **Run the test suite** to confirm nothing is broken:
   ```bash
   cargo test --all
   ```

5. **Report findings** in this format:

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
   | No clippy warnings | pass/fail | |

   ### Recommendations
   - <actionable items>
   ```

## Philosophy Check

Verify the module respects the three-construct architecture:
- Only uses signal, guard, reflex (the three primitives)
- Properties use only always/never/implies (the three forms)
- No unbounded constructs
