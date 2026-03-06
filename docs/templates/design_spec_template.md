# Design Spec: [Component / Feature Name]

> **Status:** Draft | Review | Accepted | Frozen  
> **Version:** 0.1  
> **Date:** YYYY-MM-DD  
> **Author(s):**  
> **Related ADR(s):** ADR-NNN  
> **Related Milestone:** (e.g., Self-Hosting v1, Interpreter v1)

---

## 1. Purpose

One paragraph: what does this component do and why does it exist?

## 2. Goals and Non-Goals

### Goals
- ...

### Non-Goals (explicit exclusions)
- ...

## 3. Background

Link to prior art, related specs, frozen documents, or external references that
inform this design.

## 4. Architecture

### 4.1 High-Level Diagram

```
(ASCII diagram or reference to an image/SVG)
```

### 4.2 Key Components

| Component | Responsibility | Module/File |
|-----------|---------------|-------------|
| ... | ... | `src/...` |

### 4.3 Data Flow

Describe the input → processing → output pipeline for this component.

### 4.4 Interfaces

#### Public API
```rust
// Key function signatures or trait definitions
```

#### Configuration / CLI
```
(flags, environment variables, config file fields)
```

## 5. Detailed Design

### 5.1 [Subsection per major design element]

...

### 5.2 Error Handling

How does this component report and propagate errors? Reference `src/error.rs`
conventions and NASA safety rules (explicit `Result`, no panics).

### 5.3 Determinism & Safety

- [ ] All loops bounded
- [ ] No heap allocation (or justified exception with ADR)
- [ ] Deterministic output for same input
- [ ] No hidden mutable global state

## 6. IR / Schema Impact

Does this component produce or consume IR covered by the IR contract?

| Contract level | Schema file | Change required? |
|---------------|-------------|-----------------|
| AST (Level 1) | `mirr_ast.schema.json` | Yes / No |
| Netlist (Level 3) | `mirr_temporal_netlist.schema.json` | Yes / No |

If yes, a version bump and ADR are required.

## 7. Test Strategy

| Category | Location | What it verifies |
|----------|----------|-----------------|
| Unit tests | `src/.../mod.rs` (mod tests) | ... |
| Integration tests | `tests/...` | ... |
| Parity tests | `tests/self_hosting_parity_tests.rs` | ... |
| Fixture files | `tests/fixtures/...` | ... |

Reference the test plan template (`docs/templates/test_plan_template.md`) for
detailed test planning if the scope is large.

## 8. Performance Considerations

Expected throughput, memory budget, or latency constraints. Reference benchmark
protocol if applicable.

## 9. Migration / Rollout

How will this be integrated? Feature flags, phased rollout, backward
compatibility notes.

## 10. Open Questions

- [ ] ...
- [ ] ...

---

*Template version: 1.0 — see `docs/INDEX.md` for governance rules.*