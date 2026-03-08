---
name: propose-campaign
description: 'Generate a rigorous, audited proposal for any MIRR change — from single-file fixes to multi-file campaigns — with risk analysis, bug hunting, parity analysis, and architectural improvement checks.'
argument-hint: 'What should this change accomplish? (e.g., "fix prev validation bug", "add error codes to all messages")'
---

# MIRR Proposal Skill

Generate an auditable proposal for any change to the MIRR compiler. The scope scales — a one-file bug fix gets a lightweight proposal; a 10-file campaign gets the full treatment. Every proposal follows the same standard: read first, hunt bugs, assess risk, respect the philosophy, pin every change.

## Scope Detection

Assess the change size first. This determines how much ceremony is needed:

| Size | Files touched | Audit depth | Risk table | Execution order |
|------|--------------|-------------|------------|-----------------|
| **Patch** | 1-2 | Read the files + their test files | 1-2 rows | Not needed |
| **Campaign** | 3-9 | Read every file in affected subsystems | Full table | Required |
| **Architecture** | 10+ | Full codebase audit | Full table + backward compat matrix | Required with dependency graph |

All sizes require the Philosophy Gate, Bug Hunt, Parity Analysis, and Verification steps. No exceptions.

## Philosophy Gate

Before proposing ANY change, check it against the MIRR design principles:

- **The generative power of three.** Signal, Guard, Reflex. If the proposal adds a 4th to any triad, it must be rejected or restructured. The surface language stays tiny.
- **NASA Power-of-10 compliance.** No recursion, no unbounded loops, `#![forbid(unsafe_code)]`, `#![deny(warnings)]`. Every new algorithm must have an explicit iteration bound.
- **Hardware-synthesizable.** Every language construct must map to finite, synthesizable hardware (shift registers, counters, gates). If it can't be synthesized, it doesn't belong.
- **Properties don't affect hardware.** Properties are verification assertions only. They produce SVA, not RTL.

If the proposed campaign violates any of these, STOP and say why. Do not proceed.

## Multi-Campaign Assessment

If given multiple campaign options, assess ALL of them before choosing one. For each option explicitly state:

| Campaign | Status | Rationale |
|----------|--------|-----------|
| <name> | Not started / Partial / Complete / Defer | <why this ranking> |

Then choose the ONE that delivers the most value right now based on what you found in the codebase. Do not assume — read first.

## MIRR Architecture Reference

The compiler pipeline: parse → validate → expand patterns → simplify → width inference → temporal compile → emit.
```
src/
├── ast/           # Types: Expr, Module, Guard, Reflex, Property, Pattern
│   └── property.rs  # PropertyFormula (Always/Never/AlwaysImplies), PropertyDecl
├── parser/        # parse_mirr(), expr_parser, pattern_parser
├── lexer/         # Expression tokenizer (Token enum)
├── validation/    # semantic.rs: signal refs, duplicates, prev delays
├── expand/        # Pattern expansion (def/reflect → inline)
├── simplify.rs    # Boolean/arithmetic simplification
├── width/         # Width inference + SCC analysis (6 submodules)
├── temporal/      # Guard → shift register/counter compilation
├── emit/          # verilog.rs (SVA), json_netlist.rs, dot.rs
├── pipeline.rs    # run_pipeline() orchestrates all stages
├── mape_k/        # MAPE-K autonomic simulator (6 submodules)
└── bin/           # mirr-compile, mirr-simplify, mirr-width, mirr-simulate
tests/             # ~30 test files, ~643 tests
examples/          # 11 .mirr files (7 compilable, 2 error cases, 2 pattern demos)
```

Key types: `MirrProgram`, `Module`, `Guard`, `Reflex`, `PropertyDecl`, `Expr`, `MirrError` (6 variants).

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

## Step 2 — Bug Hunt

During the audit actively search for bugs related to the campaign area. Do not wait to be told about bugs — find them.

Check specifically:
- **Validation gaps** — is every validation applied to every construct that needs it? If guards get checked, do reflexes? Do properties? Do patterns?
- **Panic paths** — every `unwrap()`, `expect()`, `unreachable!()` is a potential crash on malformed input. Note every one found.
- **Dead code** — enum variants, functions, or error types that are defined but never constructed or called
- **Inconsistencies** — if one subsystem handles a case, check if sibling subsystems handle it too
- **Silent failures** — code paths that should return an error but don't

Report every bug found before writing the proposal. Fix bugs found during the audit as part of the campaign even if they weren't the original target.

Deliver a **Bugs Found** table:
```markdown
| # | File:Line | Bug | Severity | Fix |
|---|-----------|-----|----------|-----|
| B1 | `src/foo.rs:42` | <description> | Low / Medium / High | <proposed fix> |
```

If no bugs are found, explicitly state: "No bugs found in audit scope."

## Step 3 — Parity Analysis

Check if related subsystems have features that the campaign area lacks. The goal is to find gaps where one part of the codebase has advanced further than another.

Ask for every subsystem in scope:
- Does the MAPE-K runtime have constructs the compiler doesn't support yet?
- Does the parser support syntax the emitter can't output?
- Does the AST have variants the validator doesn't check?
- Does the JSON emitter handle cases the DOT emitter doesn't?
- Do guards get treatment that properties or reflexes don't?

Deliver a **Parity Gap** table:
```markdown
| # | Has feature | Missing feature | Gap description |
|---|-------------|-----------------|-----------------|
| P1 | `mape_k/ltl.rs` | `ast/property.rs` | `EventuallyWithin` exists in runtime but not in compiler |
```

If no parity gaps are found, explicitly state: "No parity gaps found."

## Step 4 — Architectural Improvement Check

Before finalizing the proposal ask: does this change enable a cleanup elsewhere?

Specifically check:
- Does adding a new method or abstraction eliminate duplicate match arms across multiple files?
- Does fixing this bug reveal a pattern that should be enforced by the type system instead of runtime checks?
- Does this change make future campaigns easier or harder?
- Is there a centralized dispatch opportunity that reduces future maintenance burden?

Document any architectural improvements as part of the proposal even if they weren't the original target. These are often the most valuable part of a campaign.

Deliver an **Architectural Improvements** section:
```markdown
| # | Improvement | Files affected | Benefit |
|---|-------------|---------------|---------|
| A1 | Add `exprs()`/`exprs_mut()` to `PropertyFormula` | 5 files | Eliminates duplicate match arms, new variants only require updating one location |
```

If none found, explicitly state: "No architectural improvements identified."

## Step 5 — Risk & Constraint Analysis

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

| Test file | Tests | Affected? | Why |
|-----------|-------|-----------|-----|
| `tests/foo.rs` | N | Yes/No/Maybe | <reason> |

For Architecture scope — also provide a full backward compat matrix showing which combinations of existing features interact with the proposed changes.

## Step 6 — Proposal

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

### For new emission strings — pin the exact output including operators and delimiters:
```markdown
| Context | Exact output |
|---------|-------------|
| SVA for AlwaysImplies | `assert property (@(posedge clk) disable iff (rst) P \|-> Q);` |
| JSON kind field for NeverImplies | `"never_implies"` |
| DOT fillcolor for cover property | `lightyelow` |
```

### For new files — full path + description:
```markdown
| File | Purpose |
|------|---------|
| `tests/new_tests.rs` | N tests for feature X |
```

## Step 7 — Execution Order

Table showing implementation sequence and dependencies:
```markdown
| Step | Section | Files | Depends on |
|------|---------|-------|-----------|
| 1 | A | `src/ast/foo.rs` | — |
| 2 | B | `src/parser/bar.rs` | Step 1 |
```

Rule: after each step, `cargo build` must succeed with zero warnings. Do not proceed to the next step if the build fails.

## Step 8 — Verification

Exact commands with expected outcomes:
```bash
# Baseline before starting
cargo test --all 2>&1 | grep "test result:"

# All existing tests pass after changes
cargo test --all

# New tests pass
cargo test --test <new_test_file>

# Zero clippy warnings
cargo clippy --all-targets --all-features -- -D warnings

# Docs build clean
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Examples compile (if applicable)
cargo run --bin mirr-compile -- --emit verilog examples/<file>.mirr
cargo run --bin mirr-compile -- --emit json examples/<file>.mirr
cargo run --bin mirr-compile -- --emit dot examples/<file>.mirr
```

## Step 9 — File Manifest

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

Before finalizing the proposal, verify every item:

- [ ] Philosophy gate passed — no triads broken, all constructs synthesizable
- [ ] Multi-campaign assessment completed if given multiple options
- [ ] Every relevant file was read before proposing changes
- [ ] Bug hunt completed — bugs found are documented and included in campaign scope
- [ ] Parity analysis completed — gaps between subsystems documented
- [ ] Architectural improvements identified and included if found
- [ ] Every change pinned to specific file AND line number
- [ ] Every new error message quoted exactly
- [ ] Every new emission string quoted exactly including operators and delimiters
- [ ] Every new JSON field name quoted exactly
- [ ] Risk table has at least one entry per section
- [ ] Constraint table includes all NASA/CI requirements
- [ ] Backward compatibility explicitly addressed for every affected test file
- [ ] Execution order has explicit dependency chain
- [ ] Verification commands are copy-pasteable with expected outcomes
- [ ] File manifest accounts for every touched file — edited AND new
- [ ] No assumption made about code that wasn't read