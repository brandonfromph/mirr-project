# ADR-NNN: [Title]

> **Status:** Proposed | Accepted | Superseded | Deprecated  
> **Date:** YYYY-MM-DD  
> **Author(s):**  
> **Supersedes:** (ADR number, if any)  
> **Superseded by:** (ADR number, if any)

---

## Context

What is the issue or requirement that motivates this decision? Include relevant
technical constraints, safety considerations, and project history.

## Decision

State the architectural decision clearly and concisely.

## Rationale

Why was this option chosen over the alternatives? Reference NASA safety rules,
determinism requirements, or project constraints where applicable.

## Alternatives Considered

| Alternative | Pros | Cons | Why rejected |
|-------------|------|------|--------------|
| Option A | ... | ... | ... |
| Option B | ... | ... | ... |

## Consequences

### Positive
- ...

### Negative
- ...

### Risks
- ...

## Affected Artifacts

| Artifact | Change required |
|----------|----------------|
| `docs/...` | ... |
| `src/...` | ... |
| `tests/...` | ... |
| `docs/schemas/...` | ... |

## Compliance Notes

- [ ] No impact on frozen specs (or ADR for spec change attached)
- [ ] No impact on IR contract version (or version bump planned)
- [ ] No impact on NASA safety rules (bounded loops, no alloc, deterministic output)
- [ ] `docs/INDEX.md` updated

---

*Template version: 1.0 — see `docs/INDEX.md` for governance rules.*