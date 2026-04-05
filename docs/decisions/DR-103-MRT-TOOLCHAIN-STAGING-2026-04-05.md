# DR-103: Stage MRT Toolchain Hardening Before Wasm-Host Migration

**Status:** Accepted
**Date:** 2026-04-05
**Participants:** Researcher-Alpha, Researcher-Beta, Researcher-Gamma, Researcher-Delta

## Context

Proposal 102 established explicit MRT routing, role gates, and bounded output behavior in the TypeScript MCP interface. The team now wants the next step to make the developer toolchain easier to use while preserving safety-critical constraints. We must choose between immediate wasm-first runtime migration, TS-interface hardening, or a staged hybrid approach.

## Decision

Use a staged strategy:
- Proposal 103 executes TS-interface/toolchain hardening and developer workflow improvements now.
- Wasm-host runtime migration is deferred to a later proposal after explicit readiness gates are met.

## Decision Analysis Summary

| Model | Recommendation | Key Reasoning |
|-------|---------------|---------------|
| Researcher-Alpha | Option C (A now, B later) | Compounds Proposal 102 and lowers risk while preserving migration path |
| Researcher-Beta | Option C (A now, B later) | Runtime path is TS today; hardening is immediate productivity gain |
| Researcher-Gamma | Option C with runtime authority preserved | Avoids destabilizing current stdio runtime and policy enforcement |
| Researcher-Delta | Option A immediate, C acceptable | TS hardening is the safest immediate increment |

**Agreements:**
- Keep explicit static MRT tool names and deny-by-default role policy.
- Keep bounded I/O enforcement as a hard contract.
- Avoid full runtime rewrite in the next step.

**Disagreements:**
- Whether to label the recommendation as strict A or staged C.
- How aggressively to sunset compatibility behavior in the same proposal.

## Consequences

**Positive:**
- Fastest path to developer productivity with minimal architecture churn.
- Preserves established safety controls and verification gates.
- Keeps a clean, explicit runway for later wasm-host migration.

**Negative:**
- Delays full wasm-host performance/consolidation benefits.
- Requires disciplined follow-up proposal to avoid indefinite staging.

**Risks:**
- Drift between runtime and SDK dispatch surfaces if shared contracts are not enforced.
- Compatibility behavior may continue to mask unknown-method mistakes unless strict mode is adopted.

## Alternatives Considered

1. Immediate wasm-first migration (Option B).
Reason rejected: high near-term risk and larger validation surface in safety-critical workflow.

2. TS hardening only with no migration plan (Option A only).
Reason rejected: strong immediate value, but no explicit route to wasm-host readiness.

3. Staged hybrid (chosen).
Reason accepted: balances immediate reliability/productivity with explicit long-term migration gates.

## Validation

Closeout validation results:
- `npm.cmd --prefix mcp_server test` (mcp_server tests): pass.
- RWFI2 contract tests: pass.
- `cargo check --all-targets`: pass.
- `cargo.exe clippy --all-targets -- -D warnings`: pass.
