---
name: mirr-arsenal
description: The definitive command center for MIRR development. Enforces the CDD (Correctness-Driven Development) lifecycle, Zero-Debt Invariant, and advanced EDA/simulation workflows.
---

# MIRR Arsenal: The Ultimate Developer Command Center

This skill provides the expert procedural guidance required to maintain a safety-critical HDL compiler. It bridges high-level architecture decisions with surgical implementation and rigorous verification.

## 1. The CDD Lifecycle (Audit → Propose → Execute)

Every change, from a minor bug fix to a 50-file architectural overhaul, follows the **Correctness-Driven Development** (CDD) cycle.

### Phase 1: Heavy Audit
- **Rule**: Never trust status claims; read actual file content.
- **Action**: Use `grep_search` and `glob` to map dependencies. Read every file in scope.
- **Verification**: Reproduce the bug or confirm the feature gap with a minimal reproduction case.

### Phase 2: Signed Proposal
- **Requirement**: Create a proposal in `proposals/NNN-ID-YYYY-MM-DD.md`.
- **Content**: Must include a **Debt Audit** (7 Prohibitions) and a **Breakage Map**.
- **Signing**: Proposals must be peer-reviewed or auto-signed if they pass the Philosophy Gate (Signal/Guard/Reflex triad).

### Phase 3: Parallel Execution (Wave Plan)
- **Rule**: Defer compilation until the end.
- **Parallelism**: Partition files exclusively among 15+ agents.
- **Execution**: Coordinator pre-reads files, embeds context in prompts, and launches all agents in one message.

## 2. The Zero-Debt Invariant

You are forbidden from introducing technical debt. If you touch a file, you must fix any existing violations of these seven prohibitions:

| ID | Prohibition | Action |
|---|---|---|
| **D1** | No wrapper functions | Absorb single-line indirection into callers. |
| **D2** | No deprecated aliases | Delete old names; migrate callers immediately. |
| **D3** | No dead code | Delete unreachable/unused code. Orphaned domain logic must be completed, not deleted. |
| **D4** | No redundant abstractions | Collapse layers that only forward calls. |
| **D5** | No backward-compat shims | No `_old` or `// removed` markers. Atomic changes only. |
| **D6** | No duplicate logic | Consolidate identical match arms, constants, and helpers. |
| **D7** | No misleading comments | Comments must match current code exactly. Delete stale ones. |

## 3. Advanced EDA & Simulation Arsenal

Use the following tools (via MCP or direct CLI) for hardware-grade verification:

- **`run_cargo`**: Use `test`, `check`, or `build`.
- **`read_netlist`**: Inspect IR/JSON artifacts for structural correctness.
- **`run_simulator`**: Execute `.rspu` images or Rust behavioral models.
- **`estimate_resources`**: Check LUT/Reg counts to ensure synthesizability.
- **`parity_check`**: Automated golden-file comparison between behavioral and RTL outputs.

## 4. Safety & CI Gates

### Mandatory Git Hooks
- **`pre-commit`**: Automatically blocks protected files (`paper/living-doc/`) from public commits.
- **`pre-push`**: Automatically runs `scripts/ci-local.sh` and strips protected content for public-only pushes.

### The CI Protocol
Before closing out any campaign, you MUST pass:
1. `cargo fmt --check`
2. `cargo clippy --all-targets -- -D warnings`
3. `cargo nextest run --test-threads 8`
4. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`
5. `bash scripts/ci-local.sh` (for example compilation)

## 5. Agent Scaling (Coordinator Role)

When managing large campaigns:
- **Partitioning**: Assign each file to EXACTLY one agent.
- **Context Injection**: Embed full file content in agent prompts to eliminate Read steps.
- **2-Line Reports**: Enforce minimal output from agents to keep the coordinator's context lean.
- **Deferral**: Never compile mid-wave unless the Breakage Map declares a cross-wave dependency.
