---
name: execute-campaign
description: 'Execute a signed MIRR campaign proposal using parallel wave strategy. Reads the proposal, groups steps into waves, launches parallel agents, runs one test gate per wave. Use this after a proposal has been signed.'
argument-hint: 'Path to the signed proposal (e.g., "proposals/013-FPGA001-2026-03-09.md")'
user-invocable: true
---

# MIRR Campaign Executor

Execute a signed campaign proposal efficiently using parallel wave-based coordination. This skill turns a proposal's execution order into concrete parallel agent launches with minimal compile overhead.

## Prerequisites

- The proposal must have status `SIGNED` (not PROPOSED or VETOED)
- The proposal must have an execution order table with step dependencies
- All CI gates must be green before starting (`cargo test --all` passes)

## Execution Model

### Key Principle: Test Once Per Wave, Not Per Agent

The single most important rule: **run `cargo test --all` exactly ONCE after all edits in a wave are applied**, not inside each parallel agent. Each cargo compile+test cycle costs minutes. Running it per-agent multiplies wall-clock time by the agent count.

### Wave Construction

Read the proposal's execution order table. Group steps into waves using these rules:

1. **Wave N contains all steps whose dependencies are satisfied by waves 1 through N-1**
2. **Steps within a wave must touch different files** (no merge conflicts)
3. **Steps within a wave can run in parallel** (launched as simultaneous agents)

Example wave construction from a 12-step proposal:

```
Execution order:
  Step 1: Edit verilog.rs (no deps)
  Step 2: Edit verilog.rs tests (depends on 1)
  Step 3: Edit firrtl.rs (no deps)
  Step 4: Edit firrtl.rs tests (depends on 3)
  Step 5: Create new_file.rs (no deps)
  Step 6: Edit mod.rs (depends on 5)
  Step 7: Edit CLI (depends on 5, 6)
  Step 8: Create test file A (depends on 5)
  Step 9: Create test file B (depends on 5)
  Step 10: Full CI gate (depends on all)

Wave plan:
  Wave 1: Steps 1+2 (verilog), Steps 3+4 (firrtl), Step 5 (new file)
           — all touch different files, run in parallel
           — ONE cargo test --all after all applied

  Wave 2: Step 6 (mod.rs), Steps 8+9 (test files)
           — depend on wave 1, but independent of each other
           — ONE cargo test --all after all applied

  Wave 3: Step 7 (CLI — depends on mod.rs from wave 2)
           — sequential
           — ONE cargo test --all

  Wave 4: Step 10 (full CI gate)
           — fmt + clippy + test + doc
```

### Agent Strategy

For each wave, launch agents using the Task tool:

```
DO:
  - Edit files directly in the working tree (no worktree isolation)
  - Launch all agents in one message (true parallel execution)
  - Each agent edits its files and reports what changed
  - After ALL agents complete, run ONE cargo test --all

DO NOT:
  - Run cargo test inside each agent (wastes N × 3 minutes)
  - Use worktree isolation for non-conflicting edits (wastes merge time)
  - Wait for agent A to finish before launching agent B (kills parallelism)
  - Run agents sequentially when they touch different files
```

## Execution Procedure

### Phase 1 — Pre-flight

```bash
# Verify clean starting state
cargo test --all
cargo clippy --all-targets -- -D warnings
```

Record the starting test count. If pre-flight fails, fix before proceeding.

### Phase 2 — Wave Execution

For each wave:

1. **Launch all agents in parallel** — one Task tool call per step group, all in the same message
2. **Wait for all agents to complete** — check each agent's report
3. **Run the wave gate:**
   ```bash
   cargo test --all 2>&1 | grep -E "FAILED|^test result"
   ```
4. **If gate fails:** identify which agent's edits broke it, fix, re-run gate
5. **If gate passes:** proceed to next wave immediately (no user confirmation needed)

### Phase 3 — Final CI Gate

After all waves complete:

```bash
# The four CI gates
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Verify examples still compile
cargo run --bin mirr-compile -- --emit verilog examples/tmr_sensor_fusion.mirr > /dev/null
```

### Phase 4 — Close Out

1. Update proposal status: `SIGNED — EXECUTED`
2. Report final test count (should be >= starting count)
3. List all files created and edited

## Error Recovery

| Failure | Action |
|---------|--------|
| One agent's edits break tests | Revert that agent's changes, fix, re-apply |
| Clippy warnings in new code | Fix inline, re-run clippy |
| Merge conflict between agents | Should not happen if wave plan is correct — re-check file independence |
| Pre-flight fails | Do not proceed — fix existing issues first |

## Timing Expectations

| Operation | Typical time |
|-----------|-------------|
| `cargo build` | 30-60s |
| `cargo test --all` | 60-120s |
| `cargo clippy` | 30-60s |
| Agent making edits | 30-120s |
| Full wave (agents + test gate) | 2-4 min |
| Full campaign (3-4 waves) | 10-20 min |

The key insight: a 4-agent wave with one shared test gate takes ~4 minutes. The same 4 agents each running their own test gate takes ~16 minutes. Always share the gate.

## Report Format

After execution, deliver:

```markdown
## Campaign Execution Report: <campaign name>

### Waves
| Wave | Steps | Agents | Gate result |
|------|-------|--------|-------------|
| 1 | 1-4 | 3 parallel | PASS (920 tests) |
| 2 | 5-8 | 2 parallel | PASS (935 tests) |
| 3 | 9-10 | 1 sequential | PASS (940 tests) |

### CI Gate
| Check | Result |
|-------|--------|
| fmt | CLEAN |
| clippy | CLEAN |
| test | 940 passed, 0 failed |
| doc | CLEAN |

### Files Created (N)
- <list>

### Files Edited (N)
- <list>

### Proposal Status
Updated to: SIGNED — EXECUTED
```
