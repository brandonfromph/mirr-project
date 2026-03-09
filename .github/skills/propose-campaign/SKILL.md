---
name: propose-campaign
description: 'Generate a rigorous, audited proposal for any MIRR change — from single-file fixes to multi-file campaigns — with risk analysis, constraints, and a parallel execution plan. Use this when planning changes to the MIRR compiler before implementing them.'
argument-hint: 'What should this change accomplish? (e.g., "fix prev validation bug", "add error codes to all messages")'
---

# MIRR Proposal Skill

Generate an auditable proposal for any change to the MIRR compiler. The scope scales — a one-file bug fix gets a lightweight proposal; a 10-file campaign gets the full treatment. Every proposal follows the same standard: read first, assess risk, respect the philosophy, pin every change.

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

If the proposed campaign violates any of these, STOP and say why. Do not proceed.

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
tests/             # 40+ test files, 900+ tests
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
| C1 | No unsafe code | NASA P10 | `#![forbid(unsafe_code)]` |
| C2 | Zero warnings | CI | `#![deny(warnings)]` + clippy |
| C3 | All algorithms bounded | NASA P10 | Explicit `MAX_*` constants |
| C4 | Surface language stays minimal | MIRR philosophy | Three-construct architecture |
| C5 | Backward compatibility | Existing tests | `cargo test --all` must pass |

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

## Step 4 — Execution Order with Parallel Wave Plan

Table showing implementation sequence and dependencies:

```markdown
| Step | Section | Files | Depends on |
|------|---------|-------|-----------|
| 1 | A | `src/ast/foo.rs` | — |
| 2 | B | `src/parser/bar.rs` | Step 1 |
```

Rule: after each step, `cargo build` must succeed with zero warnings.

### Parallel Execution Strategy (Campaign and Architecture scope)

For proposals with 5+ steps, include a **wave plan** that groups independent steps for parallel execution:

```markdown
### Wave Plan

| Wave | Steps | Parallelizable? | Gate |
|------|-------|-----------------|------|
| 1 | 1-2, 3-4, 5-6 | Yes — touch different files | `cargo test --all` after all wave-1 edits applied |
| 2 | 7-10 | Yes — all independent | `cargo test --all` after all wave-2 edits applied |
| 3 | 11-12 | Sequential — depends on wave 2 | Full CI gate |
```

**Wave plan rules:**
- Steps within a wave MUST touch different files (no merge conflicts)
- Run `cargo test --all` ONCE per wave (not per step and not per agent) — this is critical for speed
- Do NOT use worktree isolation for steps that edit different files — edit directly in the working tree
- Each agent makes its edits, then the coordinator runs one combined test gate
- If any test fails after a wave, identify which step caused it and fix before proceeding
- Wave boundaries are the only mandatory sync points

**Anti-patterns to avoid:**
- Running `cargo test --all` inside every parallel agent (multiplies compile time by agent count)
- Using git worktrees for non-conflicting edits (adds merge overhead for no benefit)
- Waiting for agent A to finish before launching agent B when they touch different files

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
- [ ] Risk table has at least one entry per section
- [ ] Constraint table includes all NASA/CI requirements
- [ ] Backward compatibility is explicitly addressed
- [ ] Verification commands are copy-pasteable
- [ ] File manifest accounts for every touched file
- [ ] **Wave plan included** for Campaign/Architecture scope (5+ steps)
- [ ] **Wave plan avoids per-agent test runs** — one `cargo test --all` per wave only
