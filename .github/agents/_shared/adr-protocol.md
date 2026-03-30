# Architecture Decision Record (ADR) Protocol

Captures **why** a decision was made so future agents and developers don't re-litigate resolved choices.

## When to Produce an ADR

| Trigger | Example |
|---------|---------|
| Multi-model decision protocol completes | 4 Researchers analyzed DB choice → user picked PostgreSQL |
| Brainstorming skill resolves a non-trivial technical choice | Advanced Mode escalated to decision protocol, user approved |
| User explicitly overrides a multi-model recommendation | Researchers recommended approach A, user chose B |
| A prior ADR is being superseded or deprecated | New constraint invalidates DR-003 |

**Do NOT create an ADR for:** routine implementation choices, formatting preferences, or anything that doesn't affect architecture, contracts, or cross-cutting concerns.

## Process

### 1. Gather inputs
After decision protocol synthesis (or user override), collect:
- The original question framed to Researchers
- Each Researcher's recommendation + key reasoning
- Agreement/disagreement map
- The user's final choice and rationale

### 2. Determine numbering
- Find the highest existing `DR-NNN` in `docs/decisions/`
- Increment by 1
- If no existing ADRs, start at `DR-001`

### 3. Write the ADR
- Create `docs/decisions/DR-NNN-<slug>.md` using the template below
- Use a slug derived from the short title (lowercase, hyphens, no special chars)
- Set status to **Accepted** (or **Proposed** if pending user confirmation)

### 4. Persist to KB
```
remember({
  title: "DR-NNN: <Short Title>",
  content: "<1-2 sentence summary of what was decided and why>",
  category: "decisions"
})
```

### 5. Commit
- Commit the ADR file with message: `docs: add DR-NNN <short title>`
- ADRs are append-only — never delete, only supersede or deprecate

## Lifecycle

| Status | Meaning |
|--------|---------|
| **Proposed** | Under discussion, not yet binding |
| **Accepted** | Active and binding — follow this decision |
| **Deprecated** | No longer relevant (context changed), but not replaced |
| **Superseded** | Replaced by a newer ADR — link to successor |
| **Rejected** | Considered and explicitly declined |

To supersede: create the new ADR, add "Supersedes DR-NNN" to its Context, and update the old ADR's status to `Superseded by DR-MMM`.

## Template

```markdown
# DR-NNN: {Short Title}

**Status:** Proposed | Accepted | Rejected | Deprecated | Superseded
**Date:** YYYY-MM-DD
**Participants:** {which Researcher variants participated}

## Context
{What is the issue? Why are we making this decision?}
{If superseding, link: "Supersedes DR-NNN."}

## Decision
{What was decided and why — 2-5 sentences max}

## Decision Analysis Summary
| Model | Recommendation | Key Reasoning |
|-------|---------------|---------------|

**Agreements:** {what 3+ models agreed on}
**Disagreements:** {where they diverged}

## Consequences
**Positive:** {benefits}
**Negative:** {trade-offs accepted}
**Risks:** {what could go wrong, and any mitigations}

## Alternatives Considered
{Other approaches evaluated and why they were rejected — keeps the "why not" alongside the "why"}
```

