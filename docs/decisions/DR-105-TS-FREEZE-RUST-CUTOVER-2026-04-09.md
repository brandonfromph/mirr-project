# DR-105: TypeScript Prototype Freeze And Rust-Primary Cutover Gate

**Status:** Proposed
**Date:** 2026-04-09
**Participants:** Researcher-Alpha, Researcher-Beta, Researcher-Gamma, Researcher-Delta, Architect-Reviewer-Alpha, Architect-Reviewer-Beta

## Context
The repository has substantial Rust control-plane implementation and parity tests under `crates/mirr-mcp-control-plane`, while live MCP runtime startup and dispatch authority still execute through TypeScript in `mcp_server`.

The decision question was: should TypeScript be completely frozen and Rust made primary immediately.

## Decision
Do not perform immediate hard cutover today.

Adopt a **conditional freeze** policy:
1. Freeze TypeScript for feature development now.
2. Keep TypeScript runtime active until Rust satisfies mandatory cutover gates.
3. Shift to Rust-primary runtime only after all gates pass.

Interpretation:
- Immediate hard replacement: **NO_GO**.
- Structured freeze with gated cutover: **GO_CONDITIONAL**.

## Decision Analysis Summary
| Model | Recommendation | Key Reasoning |
|-------|---------------|---------------|
| Researcher-Alpha | GO_CONDITIONAL | Rust surface is strong, but TS still owns active runtime startup/dispatch and some fallback paths. |
| Researcher-Beta | NO_GO | Entry-point and CI authority remain TS; immediate freeze/cutover would break governance and test contracts. |
| Researcher-Gamma | NO_GO | Readiness score low for runtime ownership despite strong parity work; TS still controls transport and host startup. |
| Researcher-Delta | GO_CONDITIONAL | Freeze feature growth now, but keep TS as runtime until Rust host path is proven under load and parity. |
| Architect-Reviewer-Alpha | GO_CONDITIONAL | Architecture direction is correct, but TS remains authority; enforce non-negotiable pre-cutover prerequisites. |
| Architect-Reviewer-Beta | GO_CONDITIONAL | Staged migration required; avoid big-bang replacement and dual-authority drift. |

**Agreements:**
- Rust-primary is the target architecture.
- TypeScript should stop receiving net-new behavior.
- Immediate hard cutover is unsafe while TS remains runtime authority.

**Disagreements:**
- Some reviewers classify current state as NO_GO, others as GO_CONDITIONAL.
- Synthesis resolves this by treating GO_CONDITIONAL as staged NO_GO for immediate cutover.

## Consequences
**Positive:**
- Prevents further TS logic expansion and migration debt.
- Keeps runtime stable while completing Rust ownership gates.
- Preserves rollback safety during transition.

**Negative:**
- Temporary hybrid period remains.
- Additional cutover engineering and CI work required before full Rust ownership claim.

**Risks:**
- Contract drift between TS and Rust if parity gates are not enforced.
- False confidence if TS fallback paths mask Rust boundary failures.
- CI deadlock if Rust-host gates are introduced without an overlap rollout phase.

## Mandatory Cutover Gates
1. Rust MCP host binary becomes canonical runtime entrypoint in local and CI.
2. Rust host proves stdio/stream behavior parity for auth, unknown-method handling, schema rejection, and stable error envelopes.
3. Rust owns tool catalog + role matrix + route registry contracts; TS consumes generated artifacts only.
4. Quota/concurrency production mode is fail-closed on Rust boundary failure (no permissive in-memory fallback in production mode).
5. Dual-run differential gate passes across representative request corpus.
6. Two consecutive full CI runs pass with Rust-first host path.

## Alternatives Considered
1. **Immediate hard cutover now**
Rejected: runtime startup and CI authority still TS-owned.

2. **Continue hybrid with no freeze**
Rejected: invites new TS feature debt and extends split-brain risk.

3. **Conditional freeze with explicit gates (chosen)**
Accepted: balances stability and decisive migration progress.
