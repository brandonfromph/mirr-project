---
name: propose-campaign
description: 'Generate a rigorous, audited proposal for any MIRR change — from single-file fixes to multi-file campaigns — with risk analysis and constraints. Use this when planning changes to the MIRR compiler before implementing them.'
argument-hint: 'What should this change accomplish? (e.g., "fix prev validation bug", "add error codes to all messages")'
---

# MIRR Proposal Skill

Generate an auditable proposal for any change to the MIRR compiler. The scope scales — a one-file bug fix gets a lightweight proposal; a 10-file campaign gets the full treatment. Every proposal follows the same standard: read first, assess risk, respect the philosophy, pin every change.

## Scope Detection

Assess the change size first. This determines how much ceremony is needed:

| Size | Files touched | Audit depth | Risk table | Execution order |
|------|--------------|-------------|------------|-----------------|
| **Patch** | 1-2 | Read the files + their test files | 1-2 rows | Not needed |
| **Campaign** | 3-9 | Read every file in affected subsystems | Full table | Required |
| **Architecture** | 10+ | Full codebase audit | Full table + backward compat matrix | Required with dependency graph |

All sizes require the Philosophy Gate and Verification steps. No exceptions.

## Philosophy Gate

Before proposing ANY change, check it against the MIRR design principles:

- **The generative power of three.** Signal, Guard, Reflex. Always, Never, Implies. If the proposal adds a 4th to any triad, it must be rejected or restructured. The surface language stays tiny.
- **NASA Power-of-10 compliance.** No recursion, no unbounded loops, `#![forbid(unsafe_code)]`, `#![deny(warnings)]`. Every new algorithm must have an explicit iteration bound.
- **Hardware-synthesizable.** Every language construct must map to finite, synthesizable hardware (shift registers, counters, gates). If it can't be synthesized, it doesn't belong.
- **Properties don't affect hardware.** Properties are verification assertions only. They produce SVA, not RTL.

If the proposed campaign violates any of these, STOP and say why. Do not proceed.

## MIRR Architecture Reference

The compiler pipeline: parse -> validate -> expand patterns -> simplify -> width inference -> temporal compile -> emit.

```
src/
├── ast/           # Types: Expr, Module, Guard, Reflex, Property, Pattern
│   └── property.rs  # PropertyFormula (Always/Never/AlwaysImplies), PropertyDecl
├── parser/        # parse_mirr(), expr_parser, pattern_parser
├── lexer/         # Expression tokenizer (Token enum)
├── validation/    # semantic.rs: signal refs, duplicates, prev delays
├── expand/        # Pattern expansion (def/reflect -> inline)
├── simplify.rs    # Boolean/arithmetic simplification
├── width/         # Width inference + SCC analysis (6 submodules)
├── temporal/      # Guard -> shift register/counter compilation
├── emit/          # verilog.rs (SVA), firrtl.rs, json_netlist.rs, dot.rs
├── pipeline.rs    # run_pipeline() orchestrates all stages
├── mape_k/        # MAPE-K autonomic simulator (6 submodules)
└── bin/           # mirr-compile, mirr-simplify, mirr-width, mirr-simulate
tests/             # ~30 test files, ~655 tests
examples/          # 11 .mirr files (7 compilable, 2 error cases, 2 pattern demos)
```

Key types: `MirrProgram`, `Module`, `Guard`, `Reflex`, `PropertyDecl`, `Expr`, `MirrError` (4 variants: ParseError, SemanticError, TemporalCompilationError, PatternError).

Error codes: `[E1xx]` parse, `[E2xx]` semantic, `[E3xx]` temporal, `[E4xx]` pattern.

Three property forms: `always (P)`, `never (P)`, `always (P -> Q)`. No more.

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

## Step 4 — Execution Order

Table showing implementation sequence and dependencies:

```markdown
| Step | Section | Files | Depends on |
|------|---------|-------|-----------|
| 1 | A | `src/ast/foo.rs` | — |
| 2 | B | `src/parser/bar.rs` | Step 1 |
```

Rule: after each step, `cargo build` must succeed with zero warnings.

## Step 5 — Verification

Exact commands with expected outcomes:

```bash
# All existing tests pass (backward compat)
cargo test --all

# New tests pass
cargo test --test <new_test_file>

# Zero clippy warnings
cargo clippy --all-targets --all-features -- -D warnings

# Examples compile (if applicable)
cargo run --bin mirr-compile -- --emit verilog examples/<file>.mirr
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
