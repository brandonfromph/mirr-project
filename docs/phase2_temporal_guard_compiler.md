## Phase 2 & 3 — Temporal Guard Synthesis: Design Note

**Document ID:** MIRR-PHASE3-TEMPORAL-001  
**Status:** Active  
**Revision:** 2.0 — Phase 3 ECS Transition & Deterministic Condition Lowering  

---

### 1. Purpose

This document defines the design intent, scope, safety constraints, and traceability for the Temporal Guard Compiler pass of the R-SPU MIRR toolchain. 

Originally launched as an AST-based pass (Phase 2), **this system has fully transitioned to an ECS-native architecture (Phase 3).** The temporal compiler now reads directly from the ECS `Registry`, lowers each `guard` entity into concrete hardware (shift registers or counters), and synthesizes `TemporalNodeComponent` metadata back onto the entities, closing the "Temporal Seam".

### 2. Scope

**In scope:**
- Lowering `guard { when <expr> for <N> cycles; }` via `temporal_synthesis_system()` into:
  - `ShiftRegisterGuard` when `N ≤ 16` (shift register chain)
  - `CounterGuard` when `N > 16` (counter + comparator)
- Clock domain crossing detection and synchronizer insertion (`clock_domain.rs`).
- Resource estimation and physical register allocation (`allocator.rs`).
- ECS metadata synthesis (`TemporalNodeComponent`).
- JSON and Graphviz DOT emission.

**Out of scope:**
- Complex multi-guard combination logic (deferred to future phases).
- Bit-width inference (handled by `width/solver.rs`).

---

### 3. Module structure (NASA rule: no monolithic files)

```text
src/temporal/
├── mod.rs          — public façade; re-exports only
├── compiler.rs     — guard lowering logic (`lower_guard_to_ecs`), strategy selection
├── allocator.rs    — physical R-SPU register allocation limits
├── clock_domain.rs — clock crossing detection and synchronization logic
├── retiming.rs     — pipeline retiming logic for temporal barriers
├── emit.rs         — JSON and DOT emitters
└── low_level_ir/   — Directory for IR types: TemporalNetlist, CompiledGuard, GeneratedSignal

src/ecs/
└── systems.rs      — Contains `temporal_synthesis_system` driving ECS integration
```

### 4. Safety constraints

- `#![forbid(unsafe_code)]` applies to all temporal modules.
- `#![deny(warnings)]` — no warnings allowed; treat as errors.
- Bounded logic (NASA P10 Rule #1 & #2): The `temporal_synthesis_system` iterates strictly from `0` to `registry.next_id`. Clock domain counts are strictly bounded to `16`, and crossings to `128`.
- No `unwrap()` or `expect()` in non-test production paths. All `Result`/`Option` values are handled explicitly.

---

### 5. Adaptive strategy threshold

The threshold constant `SHIFT_REGISTER_THRESHOLD = 16` determines the implementation strategy:

| Delay cycles | Strategy           | Rationale                              |
|--------------|--------------------|----------------------------------------|
| N ≤ 16       | Shift register     | Direct pipeline; low area overhead    |
| N > 16       | Counter-comparator | Logarithmic area scaling; saves gates |

---

### 6. Requirement-to-test traceability

The temporal synthesis engine is rigorously covered by the expansive MIRR integration test matrix. Key requirement files include:

| Requirement ID | Description                                          | Core Test File(s)                        |
|----------------|------------------------------------------------------|------------------------------------------|
| P3-REQ-001     | ECS Metadata mapping closes Temporal Seam            | `temporal_ecs_metadata_tests.rs`         |
| P2-REQ-001     | Short guard (N≤16) lowers to ShiftRegisterGuard      | `temporal_lowering_tests.rs`             |
| P2-REQ-002     | Long guard (N>16) lowers to CounterGuard             | `temporal_lowering_tests.rs`             |
| P2-REQ-011     | Strategy selection respects threshold                | `temporal_resource_tests.rs`             |
| P2-REQ-013     | Unsupported conditions return `TemporalCompilationError` | `temporal_edge_cases_tests.rs`       |
| P2-REQ-018     | End-to-end compilation stability                     | `temporal_compiler_tests.rs`             |
| P2-REQ-CDC     | Clock domain crossing limits respected               | `clock_domain.rs` (inline tests)         |

*(Note: The test matrix for temporal parity now includes extensive chaos and edge case tests like `temporal_chaos_tests.rs` and `temporal_parity_tests.rs`.)*

---

### 7. Phase 3 ECS Upgrades

Added in Revision 2.0:
- **`temporal_synthesis_system`**: Iterates through the ECS Registry to find all entities with a `CyclesComponent`. It spawns the `TemporalCompiler` and directly compiles the entity data.
- **`TemporalNodeComponent`**: After compilation, the resulting strategy and output signals are mapped back to the ECS guard entities.
- **AST Compatibility**: The legacy `compile_module(&[Guard])` function is retained purely for backwards compatibility with pre-ECS pipeline integrations and specific bootstrap tests. 
- **Clock Domains**: Explicit tracking and synchronization of signals across multiple clock domains via `clock_domain.rs`.