---
name: propose-campaign
description: 'Full lifecycle for any MIRR change — audit, propose, execute, close out. Covers everything from single-file fixes to multi-file campaigns. One skill, one workflow: read first, plan it, sign it, ship it.'
argument-hint: 'What should this change accomplish? (e.g., "fix prev validation bug", "add error codes to all messages")'
---

# MIRR Campaign Skill

One skill for the full lifecycle of any change to the MIRR compiler: **audit → propose → sign → execute → close out**. The scope scales — a one-file bug fix gets lightweight treatment; a 10-file campaign gets the full ceremony. Every change follows the same standard: read first, assess risk, respect the philosophy, pin every change, ship it clean.

---

# Part 1 — Propose

## Proposal Archive

All signed (executed) and vetoed proposals are frozen in `proposals/` at the repo root. Before proposing a new campaign, **read the archive** to understand what has already been done, what error code ranges are claimed, and what Phase 7 sequencing decisions have been made.

```
proposals/
├── 001-SEM-001-2026-03-08.md   # Unique semantic error codes (E201-E215)
├── 002-TYPE-001-2026-03-08.md  # Semantic type checker (E601-E607) — Phase 7 foundation
├── 003-TYPE-002-2026-03-08.md  # Signed integer types (i1-i64) — extends type system
├── 004-TYPE-003-2026-03-08.md  # Signed-aware width inference — bridges typeck/width
├── 005-TYPE-004-2026-03-08.md  # Linear signal ownership (E216) — single-writer enforcement
├── 006-ROCQ-001-2026-03-08.md  # Width inference proofs in Rocq — formal verification
├── 007-TYPE005-RSPU001-2026-03-08.md  # Higher-order patterns + R-SPU ISA emission
├── 011-SPAN001-LSP001-2026-03-09.md  # Span infrastructure + LSP server
├── 012-ERR001-VSCODE001-2026-03-09.md # Error codes + diagnostics + VS Code extension
├── 013-FPGA001-2026-03-09.md  # Synthesis-ready SystemVerilog emission
├── 016-SAFE001-2026-03-09.md  # Restore #![forbid(unsafe_code)] defense-in-depth
└── ...                          # Future proposals follow NNN-ID-YYYY-MM-DD.md
```

**Naming convention:** `NNN-CAMPAIGN_ID-YYYY-MM-DD.md` where NNN is a zero-padded sequence number.

**Before proposing:** read the latest proposal to check for claimed error code ranges, open dependencies, and the Phase 7 sequencing graph. Error code ranges currently allocated: E1xx (parse, E101-E166, E170-E181), E2xx (semantic, E201-E216), E3xx (temporal), E4xx (pattern, E400-E425), E5xx (width, E500-E511), E6xx (type, E601-E609), E7xx (R-SPU, E701-E705).

## Scope Detection

Assess the change size first. This determines how much ceremony is needed:

| Size | Files touched | Audit depth | Risk table | Execution order |
|------|--------------|-------------|------------|-----------------|
| **Patch** | 1-2 | Read the files + their test files | 1-2 rows | Not needed |
| **Campaign** | 3-9 | Read every file in affected subsystems | Full table | Required |
| **Architecture** | 10+ | Full codebase audit | Full table + backward compat matrix | Required with dependency graph + **parallel wave plan** |

All sizes require the Philosophy Gate and Verification steps. No exceptions.

## Philosophy Gate

Before proposing ANY change, check it against the MIRR design principles:

- **The generative power of three.** Signal, Guard, Reflex. If the proposal adds a 4th to any triad, it must be rejected or restructured. The surface language stays tiny.
- **NASA Power-of-10 compliance.** No recursion, no unbounded loops, `#![forbid(unsafe_code)]`, `#![deny(warnings)]`. Every new algorithm must have an explicit iteration bound.
- **Hardware-synthesizable.** Every language construct must map to finite, synthesizable hardware (shift registers, counters, gates). If it can't be synthesized, it doesn't belong.
- **Properties don't affect hardware.** Properties are verification assertions only. They produce SVA, not RTL.
- **Zero-Debt Invariant.** The codebase never ages. Every campaign must leave the codebase at zero technical debt — same as if it were written today from scratch with full knowledge of what it needs to be. This is not aspirational; it is a hard gate. See the Zero-Debt Invariant section below.

If the proposed campaign violates any of these, STOP and say why. Do not proceed.

## Zero-Debt Invariant

The MIRR codebase is a safety-critical system. Technical debt in safety-critical systems kills. The codebase must read as if it were written today — no archaeological layers, no compatibility shims, no "we'll clean this up later." Every campaign enforces this invariant both on ingress (new code) and on the existing codebase it touches.

### The Seven Prohibitions

Every proposal must certify that it introduces **none** of the following. If existing code in the campaign's scope contains any of these, the proposal must fix them as part of the campaign — not in a follow-up.

| # | Prohibition | What it means | Detection |
|---|-------------|---------------|-----------|
| **D1** | **No wrapper functions** | If function A exists only to call function B with the same or fewer arguments, absorb A into its callers or delete it. No `fn foo() { bar() }` indirection layers. | Grep for single-line function bodies that are just a call to another function. |
| **D2** | **No deprecated aliases** | If a type/function was renamed, delete the old name entirely. No `pub use OldName as _` or `#[deprecated]` shims. Callers must migrate immediately. | Grep for `#[deprecated]`, `#[allow(deprecated)]`, re-exports that exist only for backward compatibility. |
| **D3** | **No dead code** | If code is unreachable, unused, or guarded by `#[allow(dead_code)]`, delete it. No "keeping it around in case we need it later." Version control is the archive, not the source tree. | `cargo clippy` + grep for `#[allow(dead_code)]`, `#[cfg(never)]`, commented-out code blocks. |
| **D4** | **No redundant abstractions** | If a struct/trait/module exists to "wrap" something that could be used directly, collapse the layer. Every abstraction must justify its existence with at least two distinct call sites that benefit from the abstraction. | Audit: does this type add behavior, or just forward calls? |
| **D5** | **No backward-compatibility shims** | No re-exports of removed types, no `_unused` parameter renames, no `// removed` tombstone comments, no feature flags that gate old behavior. When something changes, it changes everywhere atomically. | Grep for `_unused`, `_old`, `_compat`, `_legacy`, `// removed`, `// deprecated`, `// TODO: remove`. |
| **D6** | **No duplicate logic** | If the same logic exists in two places, extract it to one canonical location or pick one and delete the other. Constants, helper functions, error constructors — one source of truth. | Grep for identical or near-identical function bodies, duplicate `const` declarations, copy-pasted match arms. |
| **D7** | **No misleading comments** | Comments must describe what the code *actually does today*, not what it used to do or aspirationally will do. Dead comments are worse than no comments — they actively mislead. | Read every comment in scope and verify it matches the code. |

### Debt Audit Requirement

Every proposal must include a **Debt Audit** section (between the Current State Assessment and the Risk Analysis) that certifies:

```markdown
## Debt Audit

| # | Prohibition | Findings in scope | Action |
|---|-------------|-------------------|--------|
| D1 | No wrapper functions | None found / Found: `foo()` in `bar.rs:42` | N/A / Absorbed into caller in Section B |
| D2 | No deprecated aliases | None found | N/A |
| D3 | No dead code | Found: `old_helper()` in `baz.rs:99` | Deleted in Section A |
| D4 | No redundant abstractions | None found | N/A |
| D5 | No backward-compat shims | None found | N/A |
| D6 | No duplicate logic | Found: `MAX_X` in 3 files | Consolidated in Section C |
| D7 | No misleading comments | Found: stale comment in `qux.rs:15` | Updated in Section A |
```

**Rules:**
- Every row must have a finding or explicitly say "None found."
- Every finding must have an action that resolves it **within this proposal** — not "will fix in follow-up."
- If the scope of the campaign touches a file, ALL seven prohibitions must be checked for that file.
- The Debt Audit is a **hard gate**: the proposal cannot be signed if any finding lacks a resolution.

## MIRR Architecture Reference

The compiler pipeline: parse -> validate -> expand patterns -> simplify -> width inference -> type check -> temporal compile -> emit.

```
src/
├── ast/           # Types: Expr, Module, Guard, Reflex, Property, Pattern
│   └── property.rs  # PropertyFormula (6 forms), PropertyDecl
├── parser/        # parse_mirr(), expr_parser, pattern_parser
├── lexer/         # Expression tokenizer (Token enum)
├── validation/    # semantic.rs: signal refs, duplicates, prev delays
├── expand/        # Pattern expansion (def/reflect -> inline)
├── typeck/        # Type checker for signedness consistency (E6xx)
├── simplify.rs    # Boolean/arithmetic simplification
├── width/         # Width inference + SCC analysis (6 submodules)
├── temporal/      # Guard -> shift register/counter compilation
├── emit/          # verilog.rs, firrtl.rs, json_netlist.rs, dot.rs, rspu.rs,
│   │                testbench.rs, fpga_scaffold.rs, fpga_target.rs
│   └── mod.rs
├── diagnostic.rs  # Rich diagnostic renderer (ERR-001)
├── suggest.rs     # Did-you-mean fuzzy matcher (ERR-001)
├── pipeline.rs    # run_pipeline() orchestrates all stages
├── mape_k/        # MAPE-K autonomic simulator (6 submodules)
├── lsp/           # LSP server (diagnostics.rs)
└── bin/           # mirr-compile, mirr-simplify, mirr-width, mirr-simulate
tests/             # 40+ test files, 950+ tests
examples/          # .mirr files including tmr_sensor_fusion.mirr
```

Key types: `MirrProgram`, `Module`, `Guard`, `Reflex`, `PropertyDecl`, `Expr`, `MirrError` (6 variants: ParseError, SemanticError, TemporalCompilationError, PatternError, TypeError, RspuError).

Error codes: `[E1xx]` parse, `[E2xx]` semantic, `[E3xx]` temporal, `[E4xx]` pattern, `[E5xx]` width, `[E6xx]` type, `[E7xx]` R-SPU.

Six property forms: `always (P)`, `never (P)`, `always (P -> Q)`, `never (P -> Q)`, `eventually within N (P)`, `always (P followed_by N Q)`.

## Step 1 — Audit

Read every file relevant to the campaign scope. Do not propose changes to code you haven't read.

For each file in scope, record:
- Path and line count
- Public API surface (functions, structs, enums)
- Error message strings (exact text)
- Panic paths (unwrap, expect, unreachable)
- TODO/FIXME/HACK comments

Deliver a **Current State Assessment** table:

```markdown
| Area | Status | Evidence |
|------|--------|----------|
| <area> | Not started / Partial / Complete | <file:line or "no files found"> |
```

## Step 2 — Risk & Constraint Analysis

For every proposed change, assess:

### Risks

| # | Risk | Severity | Mitigation |
|---|------|----------|------------|
| R1 | <what could go wrong> | Low / Medium / High | <how to prevent it> |

Severity criteria:
- **High** — could break existing tests, corrupt output, or violate NASA/safety constraints
- **Medium** — could cause clippy warnings, doc build failures, or CI regressions
- **Low** — cosmetic or style inconsistency

### Constraints

| # | Constraint | Source | Enforced by |
|---|-----------|--------|-------------|
| C1 | No unsafe code | NASA P10 | `#![forbid(unsafe_code)]` in every `.rs` file |
| C2 | Zero warnings | CI | `#![deny(warnings)]` + clippy |
| C3 | All algorithms bounded | NASA P10 | Explicit `MAX_*` constants |
| C4 | Surface language stays minimal | MIRR philosophy | Three-construct architecture |
| C5 | Backward compatibility | Existing tests | `cargo test --all` must pass |
| C6 | No wrapper functions | Zero-Debt D1 | Debt Audit table |
| C7 | No deprecated aliases | Zero-Debt D2 | Debt Audit table |
| C8 | No dead code | Zero-Debt D3 | Debt Audit table + `#[allow(dead_code)]` grep |
| C9 | No redundant abstractions | Zero-Debt D4 | Debt Audit table |
| C10 | No backward-compat shims | Zero-Debt D5 | Debt Audit table |
| C11 | No duplicate logic | Zero-Debt D6 | Debt Audit table |
| C12 | No misleading comments | Zero-Debt D7 | Debt Audit table |

Add project-specific constraints as needed.

### Backward Compatibility Check

List every existing test file that could break. For each:
- File path
- Number of tests
- Why it should / should not be affected

## Step 3 — Proposal

Organize into lettered sections (A, B, C, ...). Each section must include:

### Section header format
```
## Section X: <Title> — <file_path> (currently N lines)
```

### Change table format
```markdown
| # | File:Line | Current | Proposed | Rationale |
|---|-----------|---------|----------|-----------|
| X1 | `src/foo.rs:42` | `old code` | `new code` | why |
```

### For new error messages — pin the exact string:
```markdown
| Message | Trigger condition |
|---------|-------------------|
| `"Property '{}': description."` | When X happens |
```

### For new emission strings — pin the exact output:
```markdown
| Context | Output |
|---------|--------|
| SVA for variant X | `assert property (@(posedge clk) ...);` |
```

### For new files — full path + description:
```markdown
| File | Purpose |
|------|---------|
| `tests/new_tests.rs` | N tests for feature X |
```

## Step 4 — Execution Plan

Table showing implementation sequence and dependencies:

```markdown
| Step | Section | Files | Depends on |
|------|---------|-------|-----------|
| 1 | A | `src/ast/foo.rs` | — |
| 2 | B | `src/parser/bar.rs` | Step 1 |
```

### Breakage Map

The proposal must explicitly declare where breakage is expected:

```markdown
### Breakage Map

| Step | What breaks | Why | Fixed in |
|------|------------|-----|----------|
| 3 | `typecheck_module()` signature changes | Added `&mut PipelineErrors` param | Steps 4, 5, 6 fix all callers |
| 7 | `tests/typecheck_tests.rs` won't compile | Tests use old return type | Step 8 (Test Agent) |
| — | No other breakage expected | All other steps are additive | — |
```

This map drives the compilation strategy during execution (see Part 2).

### Parallel Wave Plan (Campaign and Architecture scope)

For proposals with 5+ steps, include a **wave plan** that groups independent steps for parallel execution:

```markdown

## Execution Protocol (MANDATORY)

After proposal is written:
1. Auto-sign if philosophy gate passed and debt audit complete
2. Immediately spawn subagents using Task tool — NOT todos
3. Pre-read ALL target files before spawning any agent
4. Each Task agent prompt contains:
   - Full file contents embedded verbatim
   - Exact line numbers
   - Exact replacement text
   - Instruction: "Use only Read + Edit tools"
   - Instruction: "2-line report only: DONE. X lines changed in Y file"
5. Coordinator waits for all Tasks to complete
6. Coordinator runs single CI gate
7. Coordinator git commits with campaign message
### Wave Plan

| Wave | Steps | Parallelizable? | Breakage expected? |
|------|-------|-----------------|-------------------|
| 1 | 1-4 | Yes — touch different files | No — all additive |
| 2 | 5-8 | Yes — all independent | Yes — Step 5 changes API, Steps 6-8 fix callers |
| 3 | 9-10 | Sequential — depends on wave 2 | No |
```

**Wave plan rules:**
- Steps within a wave MUST touch different files (no merge conflicts)
- Maximize agents per wave — target 1-3 files per agent, 15-20 agents for large campaigns
- Do NOT use worktree isolation for steps that edit different files — edit directly in the working tree
- Agents use ONLY Read and Edit — no Bash, no cargo, no git
- Each agent makes its edits and returns a 2-line report, then the coordinator handles verification
- Wave boundaries are the only mandatory sync points — minimize waves, maximize parallelism

## Step 5 — Verification

Exact commands with expected outcomes:

```bash
# All existing tests pass (backward compat)
cargo test --all

# New tests pass
cargo test --test <new_test_file>

# Zero clippy warnings
cargo clippy --all-targets -- -D warnings

# Zero doc warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Formatting clean
cargo fmt --check

# Examples compile (if applicable)
cargo run --bin mirr-compile -- --emit verilog examples/<file>.mirr
cargo run --bin mirr-compile -- --emit verilog --target xilinx-7 --testbench --scaffold examples/<file>.mirr
```

## Step 6 — File Manifest

Two tables:

```markdown
### Edited (N files)
| File | Change summary |
|------|---------------|

### New (N files)
| File | Description |
|------|-------------|
```

## Quality Checklist

Before finalizing the proposal, verify:

- [ ] Every change is pinned to a specific file and line number
- [ ] Every new string (error message, emission) is quoted exactly
- [ ] No assumption is made about code that wasn't read
- [ ] The philosophy gate passed — no triads broken
- [ ] **Zero-Debt Invariant passed** — Debt Audit table complete with all 7 prohibitions checked
- [ ] **Every debt finding has an in-proposal resolution** — no "fix later" items
- [ ] Risk table has at least one entry per section
- [ ] Constraint table includes all NASA/CI/Zero-Debt requirements (C1-C12)
- [ ] Backward compatibility is explicitly addressed
- [ ] **Breakage Map included** — every known breakage point declared with its fix
- [ ] Verification commands are copy-pasteable
- [ ] File manifest accounts for every touched file
- [ ] **Wave plan included** for Campaign/Architecture scope (5+ steps)
- [ ] **Living document updated** -- `paper/living-doc/` chapters reflect any
      new features, metrics, or architectural changes; `metrics.tex` values
      updated if any counts changed; row added to `campaign-log.tex`

---

# Part 2 — Execute

## Prerequisites

- The proposal must have status `SIGNED` (not PROPOSED or VETOED)
- The proposal must have an execution plan with step dependencies and a Breakage Map

## Compilation Strategy: Defer Everything

This codebase is engineered for predictability. Every change is planned in advance. The Breakage Map declares exactly what will break and where. We do not waste minutes compiling mid-campaign to discover what the proposal already told us.

### The Rule

**Zero compilation during waves. One full CI gate at the end.**

| When | What to run | Why |
|------|------------|-----|
| **Pre-flight** (once, before any edits) | `cargo test --all` | Confirm clean starting state |
| **During any wave** | **Nothing** | Agents edit files. No compilation. The proposal already mapped every change. |
| **After ALL waves complete** | Full CI gate (fmt + clippy + test + doc) | One compilation cycle validates everything |

D3 EXCEPTION — Orphaned Features:
Before deleting any dead function, ask:
"Does this function implement real domain logic?"
If YES → classify as ORPHANED, not dead.
Orphaned features get a completion campaign, not deletion.
Dead code markers: trivial helpers, debug prints, scaffolding.
Orphaned markers: hardware logic, algorithms, domain knowledge.

### Exception: Known Breakage Points

When the Breakage Map declares that a step changes an API signature that downstream steps depend on, and those downstream steps are in a **later wave**, use `cargo check` (type-checking only, no codegen, no tests) before launching the dependent wave. This costs ~15 seconds instead of ~120 seconds.

```
Wave 1: Steps 1-4 (additive, no breakage) → no compilation
Wave 2: Step 5 changes function signature → cargo check before wave 3
Wave 3: Steps 6-8 fix all callers of the changed signature → no compilation
Final: Full CI gate
```

**Never use `cargo test --all` mid-campaign.** Never use `cargo build` mid-campaign. If you need a mid-campaign type-check, `cargo check` is the maximum.

### Why This Works

1. **The proposal already knows what breaks.** The Breakage Map is a pre-computed dependency graph. We don't need the compiler to discover what we already planned.
2. **This codebase doesn't have surprise breakage.** It's built with `#![deny(warnings)]`, `#![forbid(unsafe_code)]`, and 950+ tests. Changes are surgical, not speculative.
3. **Compilation time scales with crate graph, not with confidence.** Running `cargo test --all` four times doesn't make the code four times more correct. It makes the campaign four times slower.
4. **When something does break unexpectedly**, the final CI gate catches it, and the fix is applied once — not discovered and patched incrementally across waves.

### Cost Comparison

| Strategy | Campaign example | Compile cycles | Agent time | Total wall time |
|----------|-----------------|---------------|------------|----------------|
| **Old: test per agent** | 15 agents × `cargo test` | 15 × ~120s | ~120s | ~30+ min |
| **Old: test per wave** | 4 waves × `cargo test` | 4 × ~120s | ~3 min | ~11 min |
| **New: 5 agents, deferred** | 1 × full CI gate | 1 × ~120s | ~3 min | ~5 min |
| **New: 17 agents, deferred** | 1 × full CI gate | 1 × ~120s | ~45s | ~2.75 min |

## Agent Scaling: Maximum Parallelism (15+ Agents)

Use the Task tool to spawn real parallel subagents.
Each subagent receives:
- Exact file contents embedded in prompt (no reads needed)
- Exact line numbers for every edit
- Only Read + Edit tools enabled
- 2-line completion report only

The deferred compilation strategy unlocks massive parallelism. Since no agent ever invokes `cargo`, agents are just reading and editing files — lightweight operations that scale to 15, 20, or more simultaneous agents without resource contention.

### Why 15+ Agents Don't Crash

| Old model (per-agent compilation) | New model (deferred compilation) |
|---|---|
| Each agent runs `cargo test` → 15 simultaneous compilations fighting for `target/` → lock contention → crashes | Agents only Read + Edit files → no shared resource contention → scales freely |
| Each agent holds compilation output in context → context bloat → coordinator loses track | Agents return 2-line reports → coordinator stays thin |
| 15 × 120s compile time = 30 min wasted | 0 compile time during waves + 1 × 120s at end = 2 min total |

### Agent Design Rules for Scale

**1. One agent per 1-3 files (not per section)**

Maximize parallelism by decomposing work to the file level. A 50-file campaign gets 15-20 agents, not 5.

```
BAD:  Agent 1 handles Section A (13 files) — agent runs for 5 min, others wait
GOOD: Agent 1 handles ast/mod.rs + ast/types.rs + ast/expr.rs (3 files) — done in 30s
      Agent 2 handles ast/pattern.rs + ast/property.rs + ast/program.rs (3 files) — done in 30s
      Agent 3 handles expand/mod.rs + lexer/mod.rs + lexer/tokenizer.rs (3 files) — done in 30s
      ... (all launched simultaneously)
```

**2. Self-contained prompts — no exploration**

Each agent must receive everything it needs in its prompt. No agent should need to search the codebase, grep for patterns, or read files to figure out what to do. The proposal already did the audit.

```
BAD:  "Add #![forbid(unsafe_code)] to all files in src/ast/ — find the right insertion point"
GOOD: "Edit these 3 files. In each, insert '#![forbid(unsafe_code)]' on a new line after the
       closing '// ---' separator (line 6 in all three). Files:
       - c:\Users\elvie\nasa-rust-project\src\ast\mod.rs
       - c:\Users\elvie\nasa-rust-project\src\ast\types.rs
       - c:\Users\elvie\nasa-rust-project\src\ast\expr.rs
       Read each file first, then use Edit to insert."
```

**3. Minimal output — 2 lines per agent**

Agents report what they changed, nothing more. Verbose output from 15+ agents would flood the coordinator's context.

```
BAD:  Agent returns 500 lines of file contents, diffs, and explanations
GOOD: Agent returns "Edited 3 files: ast/mod.rs (line 7), ast/types.rs (line 7), ast/expr.rs (line 7). All insertions verified."
```

**4. No cargo, no git, no shell commands inside agents**

Agents use ONLY Read and Edit tools. No Bash calls. No `cargo check`. No `git status`. Every shell command is a potential resource lock or slowdown. The coordinator handles all compilation after agents finish.

**5. Pre-read files before launching agents**

If an agent needs to know the current content of a file to make its edit (e.g., finding the exact insertion point), read the file BEFORE launching the agent and include the relevant lines in the agent's prompt. This eliminates the agent's Read step and halves its runtime.

```
GOOD: Coordinator reads all 50 files → extracts insertion points → launches 17 agents
      with exact line numbers in their prompts → agents only Edit, no Read needed
```

### Wave Sizing at Scale

| Campaign size | Target agents per wave | Files per agent | Waves |
|--------------|----------------------|----------------|-------|
| 5-10 files | 3-5 agents | 2-3 files | 1 wave |
| 10-25 files | 5-10 agents | 2-3 files | 1 wave |
| 25-50 files | 10-17 agents | 2-4 files | 1-2 waves |
| 50-75 files | 15-25 agents | 2-4 files | 1-2 waves |
| 75+ files | 20+ agents | 3-5 files | 2-3 waves |

**The goal: minimize waves, maximize agents per wave.** Every wave boundary adds coordination overhead. Fewer waves = faster campaign.

### Example: 52-File Campaign at Full Scale

The SAFE-001 campaign edited 52 files. Here's how it scales:

```
Conservative (5 agents):
  Wave 1: 5 agents × ~10 files each → agents run 2-3 min each → total ~3 min
  Final CI gate: ~2 min
  Total: ~5 min

Maximum parallelism (17 agents):
  Wave 1: 17 agents × 3 files each → agents run 30-45s each → total ~45s
  Final CI gate: ~2 min
  Total: ~2.75 min
```

The difference: 5 minutes vs 2.75 minutes. For larger campaigns (100+ files), the gap widens further.

## Execution Procedure

### Phase 1 — Pre-flight

```bash
cargo test --all
cargo clippy --all-targets -- -D warnings
```

Record the starting test count. If pre-flight fails, fix before proceeding.

If the campaign touches 10+ files, **pre-read all target files** during pre-flight. Extract the exact insertion points, line numbers, and surrounding context. This data goes into agent prompts so agents can edit without reading.

### Phase 2 — Wave Execution

For each wave:

1. **Read the proposal's wave plan** — identify which steps are in this wave
2. **Decompose into agents** — 1-3 files per agent, self-contained prompts with exact edit instructions
3. **Launch ALL agents in one message** — every Task tool call in a single response
4. **Wait for all agents to complete** — check each agent's 2-line report
5. **Check the Breakage Map:**
   - If the next wave depends on a signature change from this wave → run `cargo check` (~15s)
   - If no downstream breakage expected → proceed directly to next wave
6. **Proceed to next wave** (no user confirmation needed between waves)

```
DO:
  - Launch ALL agents in ONE message per wave (true parallel execution)
  - Target 15-20 agents for large campaigns (25+ files)
  - Give each agent exact file paths, exact line numbers, exact edit content
  - Pre-read files and embed context in agent prompts to eliminate agent Read steps
  - Edit files directly in the working tree (no worktree isolation)
  - Proceed wave-to-wave without compilation when the Breakage Map says it's safe

DO NOT:
  - Run cargo test, cargo build, or cargo check inside any agent
  - Run any Bash command inside agents — agents use only Read and Edit
  - Give agents vague instructions that require exploration
  - Let agents return verbose output — enforce 2-line reports
  - Use fewer agents than the file count allows — maximize parallelism
  - Use worktree isolation (agents edit different files, no conflicts possible)
  - Compile "just to be safe" — trust the proposal's Breakage Map
```

### Phase 3 — Final CI Gate

After ALL waves complete, run the full CI gate exactly once:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo run --bin mirr-compile -- --emit verilog examples/tmr_sensor_fusion.mirr > /dev/null
```

If anything fails, fix it, then re-run the full gate. Do not re-run individual checks — always re-run the complete gate to confirm nothing was missed.

### Phase 4 — Zero-Debt Gate

After CI passes, verify the Zero-Debt Invariant holds for every file touched by the campaign:

```bash
# D3: No dead code
grep -rn '#\[allow(dead_code)\]' <touched_files>

# D2: No deprecated aliases
grep -rn '#\[deprecated\]' <touched_files>

# D5: No backward-compat shims
grep -rn '_unused\|_old\|_compat\|_legacy\|// removed\|// deprecated\|// TODO: remove' <touched_files>
```

For D1, D4, D6, D7 — brief manual review of touched files.

Report:

```markdown
### Zero-Debt Gate
| # | Prohibition | Status |
|---|-------------|--------|
| D1 | No wrapper functions | PASS/FAIL |
| D2 | No deprecated aliases | PASS/FAIL |
| D3 | No dead code | PASS/FAIL |
| D4 | No redundant abstractions | PASS/FAIL |
| D5 | No backward-compat shims | PASS/FAIL |
| D6 | No duplicate logic | PASS/FAIL |
| D7 | No misleading comments | PASS/FAIL |
```

If any prohibition fails, fix it before closing out.

### Phase 4.5 -- Living Document Gate

After the Zero-Debt Gate, update the living document at `paper/living-doc/`:

1. **Metrics update:** Update `metrics.tex` if any counts changed.
2. **Chapter update:** Update relevant `ch-*.tex` files. Create new `ch-<slug>.tex`
   for wholly new subsystems.
3. **Campaign log:** Append row to `campaign-log.tex`.
4. **Compile check:** `cd paper/living-doc && pdflatex main.tex` (deferred if pdflatex unavailable).

**Mandatory for Campaign and Architecture scope.** Patch-scope may skip if no user-visible changes,
but must still update `metrics.tex` if counts changed.

### Phase 5 — Close Out

1. Update proposal status: `SIGNED — EXECUTED`
2. Report final test count (must be >= starting count)
3. List all files created and edited
4. Confirm living document gate passed (or state Patch-scope exemption)

## Error Recovery

| Failure | Action |
|---------|--------|
| Final CI gate fails — test error | Read the failure, fix, re-run full CI gate |
| Final CI gate fails — clippy warning | Fix inline, re-run full CI gate |
| `cargo check` fails at a breakage point | Fix the signature mismatch, re-run `cargo check` |
| Zero-Debt Gate fails | Fix the violation, re-run Zero-Debt checks |
| Pre-flight fails | Do not proceed — fix existing issues first |

## Execution Report Format

```markdown
## Campaign Execution Report: <campaign name>

### Waves
| Wave | Steps | Agents | Mid-wave check | Result |
|------|-------|--------|---------------|--------|
| 1 | 1-4 | 3 parallel | None (no breakage) | Edits applied |
| 2 | 5-8 | 4 parallel | cargo check (API change in step 5) | Types OK |
| 3 | 9-10 | 2 parallel | None | Edits applied |

### CI Gate
| Check | Result |
|-------|--------|
| fmt | CLEAN |
| clippy | CLEAN |
| test | N passed, 0 failed |
| doc | CLEAN |

### Zero-Debt Gate
| # | Prohibition | Status |
|---|-------------|--------|
| D1-D7 | All | PASS |

### Files Created (N)
- <list>

### Files Edited (N)
- <list>

### Proposal Status
Updated to: SIGNED — EXECUTED
```
