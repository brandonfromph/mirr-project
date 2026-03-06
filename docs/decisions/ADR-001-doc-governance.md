# ADR-001: Documentation Governance and PR Policy

> **Status:** Accepted  
> **Date:** 2026-03-01  
> **Author(s):** MIRR Core Team  
> **Supersedes:** N/A  
> **Superseded by:** N/A

---

## Context

The MIRR project has achieved its self-hosting milestone v1, producing 10+
documents across specs, schemas, milestone records, and design notes. The
project is entering a phase of rapid growth (interpreter, stage-2 parity,
benchmarks, additional fixtures, native compilation research). Without a
governance structure, documentation will become stale, contradictory, or
missing — which in a NASA-style safety-critical project is a path to failure.

Key pain points identified:
1. No canonical index of what documents exist or their authority level.
2. No distinction between frozen (milestone-locked) and living documents.
3. No required templates, leading to inconsistent document structure.
4. No PR gate requiring documentation updates alongside code changes.
5. No deprecation or archival process for outdated material.

## Decision

Adopt a lightweight documentation governance framework consisting of:

1. **Canonical index** (`docs/INDEX.md`) listing every document with a status
   tag (Frozen, Active, Draft, Deprecated).
2. **Status tags** enforced by review:
   - 🟢 **Frozen** — locked for current milestone; changes require version bump + ADR.
   - 🔵 **Active** — current and maintained; normal PR process.
   - 🟡 **Draft** — WIP; must reach Active or be archived within 30 days.
   - 🔴 **Deprecated** — retained for history; not authoritative.
3. **Standard templates** for ADRs, design specs, and test plans
   (`docs/templates/`).
4. **PR checklist item**: any PR that changes behavior, schema, CLI, or safety
   rules must update the affected doc(s) and `docs/INDEX.md`.
5. **Quarterly doc review**: prune stale drafts, archive deprecated docs,
   verify index accuracy.
6. **ADR requirement**: any change to a Frozen document, IR contract, schema,
   or safety rule requires a new ADR before implementation.

## Rationale

- NASA/JPL heritage demands traceability; documentation is a safety artifact.
- Templates reduce friction for contributors and ensure consistency.
- The index prevents "lost" documents that no one knows exist.
- Status tags prevent accidental modification of milestone-locked specs.
- The 30-day draft rule prevents document graveyards.
- Quarterly review catches drift before it becomes dangerous.

## Alternatives Considered

| Alternative | Pros | Cons | Why rejected |
|-------------|------|------|--------------|
| No governance (status quo) | Zero overhead | Docs become stale/conflicting; safety risk | Unacceptable for safety-critical project |
| Heavy doc management tool (Confluence, etc.) | Rich features | External dependency; not in-repo; hard to version | Adds toolchain complexity; docs should live with code |
| Auto-generated docs only | Always current with code | Cannot capture design rationale, ADRs, trade-offs | Insufficient for safety-critical decision tracing |

## Consequences

### Positive
- Every document has a known status and owner.
- Frozen specs cannot drift without explicit, reviewed process.
- New contributors can find authoritative docs via the index.
- PR reviews include documentation quality as a gate.

### Negative
- Small overhead per PR to check docs impact.
- Quarterly review requires calendar discipline.

### Risks
- Risk of index falling behind if PRs skip the checklist. Mitigation: code
  review culture; future CI lint for index freshness.

## Affected Artifacts

| Artifact | Change required |
|----------|----------------|
| `docs/INDEX.md` | Created — canonical document index |
| `docs/templates/adr_template.md` | Created — ADR template |
| `docs/templates/design_spec_template.md` | Created — design spec template |
| `docs/templates/test_plan_template.md` | Created — test plan template |
| `docs/decisions/` | Created — ADR directory |
| `docs/runbooks/` | Created — operational runbook directory |
| `README.md` | Updated — link to `docs/INDEX.md` |

## Compliance Notes

- [x] No impact on frozen specs (this ADR establishes governance, does not modify specs)
- [x] No impact on IR contract version
- [x] No impact on NASA safety rules
- [x] `docs/INDEX.md` updated (this ADR is listed)

---

*This is the first ADR in the MIRR project. It establishes the governance
framework under which all subsequent ADRs and documentation changes operate.*