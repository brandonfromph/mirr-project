---
name: execute-campaign
description: 'Merged into propose-campaign. Use /propose-campaign for the full lifecycle: audit, propose, sign, execute, close out.'
argument-hint: 'This skill has been merged into /propose-campaign. Use that instead.'
user-invocable: true
---

# Merged into propose-campaign

This skill has been absorbed into the unified **propose-campaign** skill, which now covers the full campaign lifecycle:

1. **Part 1 — Propose**: Audit, risk analysis, debt audit, wave plan, breakage map
2. **Part 2 — Execute**: Pre-flight, wave execution, deferred CI gate, zero-debt gate, close out

Use `/propose-campaign` for everything. One skill, one workflow.

## Why merged?

- The proposal already contains the wave plan, breakage map, and execution order — having a second skill re-read and re-interpret the same document was redundant (D4: no redundant abstractions).
- The compilation strategy (defer everything to final gate) is part of the proposal's breakage map, not a separate execution concern.
- One skill means one context, one workflow, zero context-switching between documents.
