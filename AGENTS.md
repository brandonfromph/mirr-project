# nasa-rust-project — Agent Instructions

## KB Knowledge Base

This project has a **@vpxa/kb** MCP server providing search, analysis, memory, and developer tools.

### Skills Reference

| Context | Skill | Details |
|---------|-------|--------|
| KB search, analysis, memory | `kb` | See [.github/skills/knowledge-base/SKILL.md](.github/skills/knowledge-base/SKILL.md) or run `status({})` |
| Brainstorming & design | `brainstorming` | See [.github/skills/brainstorming/SKILL.md](.github/skills/brainstorming/SKILL.md) |

### Available Tool Categories

| Category | Tools | Purpose |
|----------|-------|---------|
| Search & Discovery | `search`, `find`, `symbol`, `trace`, `scope_map`, `lookup`, `dead_symbols`, `file_summary` | Find code, symbols, data flow, reading plans |
| Code Analysis | `analyze_structure`, `analyze_dependencies`, `analyze_symbols`, `analyze_patterns`, `analyze_entry_points`, `analyze_diagram`, `blast_radius` | Structure, deps, patterns, impact, diagrams |
| Knowledge | `remember`, `read`, `update`, `forget`, `list`, `produce_knowledge` | Persistent cross-session memory |
| Execution | `check`, `test_run`, `eval`, `batch`, `audit` | Typecheck, lint, test, run code, unified audit. `check` defaults to summary output (~300 tokens) |
| Code Manipulation | `rename`, `codemod`, `diff_parse`, `data_transform` | Safe renames, transforms, diff parsing |
| Context | `compact`, `workset`, `stash`, `checkpoint`, `parse_output` | Manage working sets, save progress. `compact` accepts `path` for server-side file read |
| FORGE | `forge_ground`, `forge_classify`, `evidence_map`, `digest`, `stratum_card` | Quality gates, context compression |
| Web & API | `web_fetch`, `web_search`, `http` | Fetch pages, search web, test APIs |
| Lanes | `lane` | Isolated file copies for parallel exploration (create/list/status/diff/merge/discard) |
| Git & Environment | `git_context`, `process`, `watch`, `delegate` | Git info, process management |
| Utilities | `regex_test`, `encode`, `measure`, `changelog`, `schema_validate`, `snippet`, `env`, `time` | Regex, encoding, metrics, validation |
| System | `status`, `reindex`, `health`, `guide`, `onboard`, `graph`, `queue`, `replay` | Index management, health checks, tool discovery, knowledge graph |

---

## MANDATORY: Context Reduction Protocol

**Every agent interaction MUST minimize context window usage.** Raw file reads waste tokens and degrade LLM output quality. Use these tools instead:

### Decision Tree — How to Read Code

```
Need to understand a file?
├─ Just structure? → file_summary (exports, imports, functions — ~50 tokens)
├─ Specific section? → compact({ path: "file.ts", query: "topic" }) — 5-20x reduction
├─ Multiple files? → digest (multi-source compression — token-budgeted)
├─ Repeated reference? → stratum_card (T1/T2 card — 10-100x reduction)
└─ Full file needed? → ONLY as last resort, and compact it after reading
```

### Rules
1. **NEVER read a file >100 lines without compressing it first**
2. **ALWAYS use `file_summary` before `read_file`** — often the summary is sufficient
3. **ALWAYS use `compact` when you only need specific sections** of a file (use `path` param to read server-side)
4. **Use `digest` when synthesizing from 3+ sources** — don't accumulate raw text
5. **Use `stratum_card` for files you'll reference repeatedly** in a session

---

## MANDATORY: Memory Protocol

**Every session MUST read and write persistent memory.** Without this, every conversation starts from zero.

### Session Start (MUST do ALL of these)
```
status({})                                              # Verify KB is ready
list()                                                  # See what knowledge exists
search({ query: "SESSION CHECKPOINT", origin: "curated" })  # Resume prior work
```

### During Session
| Situation | Action |
|-----------|--------|
| Found a useful intermediate result | `stash({ key: "name", value: "data" })` |
| Completed a milestone | `checkpoint({ action: "save", name: "milestone" })` |
| Made an architecture decision | `remember({ title: "...", category: "decisions" })` |
| Discovered a pattern or convention | `remember({ title: "...", category: "patterns" })` |
| Found a non-obvious solution | `remember({ title: "...", category: "troubleshooting" })` |
| About to propose a new approach | `search({ query: "..." })` — check if decided before |

### Session End (MUST do this)
```
remember({
  title: "Session checkpoint: <topic>",
  content: "<what was done, decisions made, blockers, next steps>",
  category: "conventions"
})
```

### Memory Decision Tree
```
Is this data temporary (scratch, intermediate)?
├─ Yes → stash (session-scoped key-value)
└─ No → Is it resumable progress?
   ├─ Yes → checkpoint (session-scoped snapshot)
   └─ No → remember (permanent, survives reindex)
       Categories: decisions | patterns | conventions | troubleshooting
```

---

## MANDATORY: Search-Before-Act Protocol

**NEVER write or modify code without first searching for context.**

```
# Before ANY code change:
search({ query: "<what you're about to change>" })     # Prior decisions?
scope_map({ task: "<description>" })                    # What files to read?
symbol({ name: "<key symbol>" })                        # Where is it defined/used?
```

If `search` returns a prior decision about the topic, you MUST follow it or explicitly explain why you're deviating.

---

## MANDATORY: Validation Protocol

**NEVER commit or present code without validation.**

```
check({})                                    # Typecheck + lint (tsc + biome)
test_run({})                                 # Run tests
blast_radius({ changed_files: ["..."] })     # Impact analysis
audit({})                                    # Unified project audit (structure, deps, patterns, health, dead symbols, entry points)
```

---

## FORGE Protocol (for complex tasks)

For tasks touching 3+ files or involving architectural decisions:

```
forge_classify({ task: "<description>" })        # Quick: Floor/Standard/Critical tier
forge_ground({ task: "<description>" })           # Full: scope + constraints + evidence
evidence_map({ claims: ["claim1", "claim2"] })   # Track verified vs assumed
```

---

## Search Modes

| Mode | When | Example |
|------|------|---------|
| `hybrid` (default) | General queries | `search({ query: "error handling" })` |
| `semantic` | Conceptual/meaning-based | `search({ query: "retry with backoff", search_mode: "semantic" })` |
| `keyword` | Exact identifiers | `search({ query: "CircuitBreaker", search_mode: "keyword" })` |

Filters: `origin` (`indexed`/`curated`/`produced`), `category`, `content_type`, `tags`, `min_score`.

---

## Workflow Chains

**Codebase onboarding:**
```
onboard({ path: "." }) → produce_knowledge({ path: "src/" }) → remember(...)
```

**Planning a task:**
```
search({ query: "task keywords" })
→ scope_map({ task: "description" })
→ file_summary for each file in scope
→ compact({ path: "relevant-file.ts", query: "detail needed" }) for files needing detail
→ workset({ action: "save", name: "task", files: [...] })
```

**Bug investigation:**
```
parse_output({ output: "<error>" })
→ symbol({ name: "failingFn" })
→ trace({ symbol: "failingFn", direction: "backward" })
→ blast_radius({ changed_files: ["suspect.ts"] })
```

**Safe refactor with lanes:**
```
lane({ action: "create", name: "refactor", files: [...] })
→ [make changes]
→ lane({ action: "diff", name: "refactor" })
→ check({}) → test_run({})
→ lane({ action: "merge", name: "refactor" })
```

**After making changes:**
```
blast_radius({ changed_files: ["src/file.ts"] })
→ check({}) → test_run({})
→ reindex({})
→ remember(...)
```

---

## Knowledge Categories

| Category | What to store |
|----------|---------------|
| `decisions` | Architecture choices, trade-offs, rejected alternatives |
| `patterns` | Code patterns, naming conventions, structural patterns |
| `conventions` | Session checkpoints, workflow conventions, team agreements |
| `troubleshooting` | Non-obvious fixes, debugging strategies, workarounds |

---

## Core Rules Summary

1. **Search KB before proposing anything new** — prior decisions exist
2. **Compress context aggressively** — `file_summary` → `compact` → `digest`
3. **Use persistent memory** — `remember` decisions, `stash` temporary data
4. **Validate before committing** — `check` + `test_run` + `blast_radius`
5. **Follow `_Next:` hints** in tool responses for guided workflow
6. **Use FORGE for complex tasks** — `forge_classify` → `forge_ground` → `evidence_map`
