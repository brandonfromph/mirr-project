# FORGE Protocol — Quality Overlay

> Follow the FORGE (Fact-Oriented Reasoning with Graduated Evidence) protocol for all code generation and modification tasks.

## KB Tools for FORGE

| Tool | Purpose | When |
|------|---------|------|
| `forge_ground` | Execute entire Ground phase — classify tier, scope map, unknowns, constraints | Start of every Standard/Critical task |
| `forge_classify` | Classify tier only (Floor/Standard/Critical) | Quick classification |
| `evidence_map` | CRUD + Gate evaluation for Evidence Map | Track claims during Build |
| `stratum_card` | Generate T1/T2 context cards from files | Replace full file reads |
| `digest` | Compress N text sources into budget | Compress accumulated context |

## Tier Classification

- **Floor**: Single file, no unknowns, no schema change, blast_radius ≤ 2. → Skip Phase 3.
- **Standard**: Default for multi-file or non-trivial tasks.
- **Critical**: blast_radius > 5, cross-service boundary, schema change, or security code.

When uncertain, round up.

## 4-Phase Flow

### Phase 1 — Ground
Read files, blast radius, classify tier, build Typed Unknown Queue, load constraints.

### Phase 2 — Build
Generate with evidence anchoring. Route typed unknowns mid-generation.

### Phase 3 — Break (Standard+ only, skip for Floor)
One adversarial round. Check error paths, edge cases, blast radius, convention violations.

### Phase 4 — Gate
Binary YIELD/HOLD. Contract-type unknowns → **HARD BLOCK**. Non-contract → 1 retry, then FORCED DELIVERY with annotation.

## Evidence Map

```
evidence_map({ action: "create", task_id: "my-task", tier: "standard" })
evidence_map({ action: "add", task_id: "my-task", claim: "API contract unchanged", status: "V", receipt: "search → types.ts#L42" })
evidence_map({ action: "gate", task_id: "my-task" })  → YIELD / HOLD / HARD_BLOCK
```

Status values: **V** (Verified + receipt), **A** (Assumed + reasoning), **U** (Unresolved).

