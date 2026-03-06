## Phase 2 — Temporal Guard Compiler: Design Note

**Document ID:** MIRR-PHASE2-001  
**Status:** Active  
**Revision:** 1.2 — Step 2.2: Deterministic Condition Lowering + IR Enrichment  

---

### 1. Purpose

This document defines the design intent, scope, safety constraints, and requirement-to-test
traceability for the Phase 2 Temporal Guard Compiler pass of the R-SPU MIRR toolchain.

The Phase 2 artifact is a CLI-accessible pass (`--compile`) that reads a parsed MIRR module,
lowers each `guard` declaration into a concrete hardware representation (shift registers for
short delays, counter-comparators for long delays), and emits a structured JSON or Graphviz
DOT description of the resulting netlist.

---

### 2. Scope

**In scope:**
- Lowering `guard { when <expr> for <N> cycles; }` into:
  - `ShiftRegisterGuard` when `N ≤ 16` (shift register chain)
  - `CounterGuard` when `N > 16` (counter + comparator)
- Resource estimation (shift registers, counters, logic gates)
- Netlist emission as JSON (via `serde_json`)
- Netlist emission as Graphviz DOT
- Compilation statistics tracking

**Out of scope for Phase 2:**
- Complex multi-guard combination logic
- Logic simplification (Phase 3)
- Bit-width inference (Phase 4)
- MAPE-K simulation (Phase 5)
- HDL/RTL output (Phase 7+)

---

### 3. Module structure (NASA rule: no monolithic files)

```
src/temporal/
├── mod.rs          — thin public façade; re-exports only
├── compiler.rs     — guard lowering logic + resource estimator
├── emit.rs         — JSON and DOT emitters
└── low_level_ir.rs — IR types: TemporalNetlist, CompiledGuard, GeneratedSignal

tests/
├── temporal_lowering_tests.rs  — shift register / counter compilation tests
├── temporal_emit_tests.rs      — JSON / DOT output tests
└── temporal_resource_tests.rs  — resource estimation + strategy selection tests
```

The public API surface area is kept minimal in accordance with NASA/JPL minimization of
interface complexity.

---

### 4. Safety constraints

- `#![forbid(unsafe_code)]` applies to all temporal modules.
- `#![deny(warnings)]` — no warnings allowed; treat as errors.
- No `unwrap()` or `expect()` in non-test production paths.
- All `Result`/`Option` values are handled explicitly.
- No recursive data-structure traversal without a bounded depth limit.
- No global mutable state.
- Error variants are defined in `src/error.rs` (centralized authority).

---

### 5. Adaptive strategy threshold

The threshold constant `SHIFT_REGISTER_THRESHOLD = 16` determines which implementation
strategy is chosen:

| Delay cycles | Strategy           | Rationale                              |
|--------------|--------------------|----------------------------------------|
| N ≤ 16       | Shift register     | Direct pipeline; low area overhead    |
| N > 16       | Counter-comparator | Logarithmic area scaling; saves gates |

This threshold is named, documented, and testable. It must not be an anonymous magic number.

---

### 6. Requirement-to-test traceability

| Requirement ID | Description                                          | Test file                        | Test name                          |
|----------------|------------------------------------------------------|----------------------------------|------------------------------------|
| P2-REQ-001     | Short guard (N≤16) lowers to ShiftRegisterGuard      | temporal_lowering_tests.rs       | test_shift_register_compilation    |
| P2-REQ-002     | Long guard (N>16) lowers to CounterGuard             | temporal_lowering_tests.rs       | test_counter_compilation           |
| P2-REQ-003     | Mixed guards produce both types                      | temporal_lowering_tests.rs       | test_mixed_guard_compilation       |
| P2-REQ-004     | Guard names are preserved in netlist                 | temporal_lowering_tests.rs       | test_guard_names_preserved         |
| P2-REQ-005     | Zero-delay guard lowers to empty ShiftRegister       | temporal_lowering_tests.rs       | test_zero_delay_guard              |
| P2-REQ-006     | Generated signal count matches expectation           | temporal_lowering_tests.rs       | test_generated_signal_counts       |
| P2-REQ-007     | JSON emission includes guard names and statistics    | temporal_emit_tests.rs           | test_json_emission_structure       |
| P2-REQ-008     | DOT emission is syntactically well-formed            | temporal_emit_tests.rs           | test_dot_emission_structure        |
| P2-REQ-009     | Resource estimate for shift register is correct      | temporal_resource_tests.rs       | test_shift_register_resource_est   |
| P2-REQ-010     | Resource estimate for counter is correct             | temporal_resource_tests.rs       | test_counter_resource_est          |
| P2-REQ-011     | Strategy selection respects threshold                | temporal_resource_tests.rs       | test_strategy_selection_threshold  |
| P2-REQ-012     | Counter width is log2(N)+1 bits                      | temporal_resource_tests.rs       | test_counter_width_calculation     |
| P2-REQ-013     | Unsupported condition form → explicit TemporalCompilationError (no silent fallback) | temporal_lowering_tests.rs | test_unsupported_condition_is_rejected |
| P2-REQ-014     | Negated-signal condition (`!sig`) lowers to `ConditionKind::NegatedSignal` | temporal_lowering_tests.rs | test_negated_signal_condition_lowering |
| P2-REQ-015     | All six comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`) lower to `ConditionKind::Comparison`; logical ops (AND, OR) rejected | temporal_lowering_tests.rs | test_comparison_condition_lowering |
| P2-REQ-016     | `condition_kind` field is populated in `ShiftRegisterGuard` IR after compilation | temporal_lowering_tests.rs | test_condition_kind_stored_in_shift_register_ir |
| P2-REQ-017     | `condition_kind` field is populated in `CounterGuard` IR after compilation | temporal_lowering_tests.rs | test_condition_kind_stored_in_counter_ir |
| P2-REQ-018     | Full-pipeline compilation of `neonatal_respirator.mirr` succeeds end-to-end | temporal_lowering_tests.rs | test_neonatal_respirator_compiles |

---

### 7. Known limitations

- **Step 2.1 resolved:** Guard conditions not in the supported set now return an explicit
  `MirrError::TemporalCompilationError` — the silent synthetic fallback has been removed.
- **Step 2.2 resolved:** All six comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`)
  now lower to magnitude comparator circuits. The `neonatal_respirator.mirr` example
  (`airway_pressure < 50 for 1000 cycles`) compiles end-to-end.
- `ComplexGuard` variant is defined in the IR but not yet produced by the compiler.
- DOT output for `ComplexGuard` is a no-op stub.
- Multi-signal conditions (AND, OR between two signals) are still rejected — deferred to Phase 3.

---

### 8. Step 2.1 — ConditionKind contract additions (revision 1.1)

Added in revision 1.1:

- **`ConditionKind` enum** (`src/temporal/low_level_ir.rs`): Typed representation of every
  guard condition form the compiler can lower. Variants: `SimpleSignal(String)`,
  `NegatedSignal(String)`, `Comparison { signal, op, value }`.
- **`ConditionKind::try_from_expr`**: Infallible pattern-match on `Expr`; returns `Err(&'static str)`
  for any unsupported form. No heap allocation on the error path.
- **`lower_condition()`** in `compiler.rs`: Replaces the removed `get_input_signal_name()`.
  Maps `try_from_expr` errors to `MirrError::TemporalCompilationError` with the guard name
  embedded in the diagnostic message.

---

### 9. Step 2.2 — IR enrichment and explainability (revision 1.2)

Added in revision 1.2:

- **Extended `ConditionKind::Comparison`** to accept all six comparison operators (`Eq`, `Ne`,
  `Lt`, `Le`, `Gt`, `Ge`). Previously only `Eq`/`Ne` were accepted. `Lt`, `Le`, `Gt`, `Ge`
  now lower to magnitude comparator hardware circuits.
- **`condition_kind: ConditionKind` field** added to `ShiftRegisterGuard` and `CounterGuard`.
  The IR is now self-describing: every compiled guard carries the full semantics of the original
  condition, not just the primary signal name.
- **`ConditionKind::describe()`** method added for human-readable emission in DOT labels and
  the HTML demo viewer.
- **DOT labels enriched** (`emit.rs`): cluster labels now include condition description and
  delay cycles (e.g., `Counter: sustained_pressure_drop\nwhen airway_pressure < 50\nfor 1000 cycles`).
- **`docs/demo_viewer.html`** created: static HTML page showing the full compiler output for
  `neonatal_respirator.mirr` — guards table, pipeline diagram, generated signals, JSON netlist,
  and DOT source. Open in any browser.

---

### 10. Acceptance criteria for Phase 2 Step 2

- `cargo fmt` passes with no changes.
- `cargo clippy -- -D warnings` passes with zero diagnostics.
- `cargo test` passes with all 98 tests green.
- No `unsafe` blocks exist in the temporal module tree.
- `condition_kind` field populated in all compiled guard variants.
- `neonatal_respirator.mirr` compiles end-to-end without error (P2-REQ-018).
- `docs/demo_viewer.html` opens in a browser and correctly displays compiler output.
- All traceability table entries (P2-REQ-001..018) have passing tests.

---

### 11. Acceptance criteria for Phase 2 Step 1 (archived)

- `cargo fmt` passes with no changes.
- `cargo clippy -- -D warnings` passes with zero diagnostics.
- `cargo test` passes with all tests green.
- No `unsafe` blocks exist in the temporal module tree.
- No monolithic file exceeds ~150 lines of non-test Rust.
- All traceability table entries above have passing tests.