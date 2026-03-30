# Code Agent — Shared Base Instructions

> This file contains shared protocols for all code-modifying agents (Implementer, Frontend, Refactor, Debugger). Each agent's definition file contains only its unique identity, constraints, and workflow. **Do not duplicate this content in agent files.**

## Invocation Mode Detection

You may be invoked in two modes:
1. **Direct** — you have full KB tool access. Execute KB Recall normally.
2. **Sub-agent** (via Orchestrator) — you may have limited MCP tool access.
   The Orchestrator provides context under "## Prior KB Context" in your prompt.
   If present, skip KB Recall and use the provided context instead.

**Detection:** If your prompt contains "## Prior KB Context", you are in sub-agent mode.

---

## FORGE Protocol (Quality Gate)

**Quick reference:**
1. If the Orchestrator provided FORGE tier in your prompt, use it. Otherwise, run `forge_classify` to determine tier.
2. **Floor tier** → implement directly, no evidence map needed.
3. **Standard/Critical tier** → Use `evidence_map` to track each critical-path claim as V/A/U during your work.
4. After implementation, run `evidence_map(gate, task_id)` to check gate status.
5. Use `stratum_card` for quick file context instead of reading full files. Use `digest` to compress accumulated context.

---

## KB Recall (BLOCKING — Before ANY Code Change)

1. **Search for relevant context:**
   ```
   search("feature/area keywords")
   scope_map("what you are doing")
   ```
2. **Check for existing patterns** — reuse established conventions
3. **Read design decisions** that constrain your implementation
4. **If KB has no hits**, proceed but **remember your findings at the end**

**Proceed only after KB search is complete.**

---

## KB Learn (After Completing Work)

Before returning your handoff, persist discoveries to KB:
- Architecture insights → `remember({ title, content, category: "patterns" })`
- Non-obvious solutions → `remember({ title, content, category: "troubleshooting" })`
- Key decisions made → `remember({ title, content, category: "decisions" })`
- Outdated KB entries → `update(path, content, reason)`

---

## Handoff Format

Always return this structure when invoked as a sub-agent:

```markdown
<handoff>
  <status>SUCCESS | PARTIAL | FAILED | ESCALATE</status>
  <summary>{1 sentence summary}</summary>
  <artifacts>
    - Created: {files}
    - Modified: {files}
    - Deleted: {files}
  </artifacts>
  <context>{what the next agent needs to know}</context>
  <blockers>{any blocking issues}</blockers>
</handoff>
```

