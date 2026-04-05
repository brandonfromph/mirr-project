# DR-102: Shared Static MRT Contract With Bounded CLI Output

**Status:** Accepted
**Date:** 2026-04-05
**Participants:** Researcher-Alpha, Researcher-Beta, Researcher-Gamma, Researcher-Delta

## Context

The repo has two MRT dispatch surfaces (`mcp_server/src/server.ts` runtime path and `mcp_server/src/mrt.ts` SDK path) with overlapping but not identical behavior. Proposal 101 completed KB-lite backend integration, but control-plane tool exposure remains partial and inconsistent. We need to expose additional verified existing binaries (`mirr-wave`, `mirr-lsp`) while preserving static tool names, deny-by-default auth, and bounded input/output with a mandatory `MAX_OUTPUT_BYTES` contract.

## Decision

Adopt a shared static MRT contract (tool names, argument builders, and bounds) consumed by both dispatch surfaces, and enforce a single bounded-output policy on every CLI invocation. Expand explicit MRT tools to include wave and lsp diagnostics paths without wildcard dispatch or dynamic tool construction.

## Decision Analysis Summary

| Model | Recommendation | Key Reasoning |
|-------|---------------|---------------|
| Researcher-Alpha | Shared static registry + shared bounded executor | Minimizes drift and enforces explicit tool contracts and output bounds |
| Researcher-Beta | Shared static registry + auth anchored in runtime API-key path | Current runtime path has stronger auth semantics than SDK role fallback |
| Researcher-Gamma | Keep runtime as authority, unify contract and runner | Active startup path is runtime server; alignment avoids breaking stdio behavior |
| Researcher-Delta | Direct extension with strict bounds acceptable | Lowest-change path, but still requires explicit `MAX_OUTPUT_BYTES` and no wildcard routing |

**Agreements:**
- Static explicit tool names only (no wildcard or dynamic method generation).
- Add hard output bounds via `MAX_OUTPUT_BYTES` for stdout/stderr on every CLI call.
- Keep deny-by-default tool allowlist and explicit role gating.
- `mirr-wave` and `mirr-lsp` are real binaries and may be exposed with bounded, explicit contracts.

**Disagreements:**
- Whether to fully centralize execution in one surface immediately vs keep dual surfaces with shared constants first.
- Degree of short-term compatibility preservation around legacy wrapper behavior.

## Consequences

**Positive:**
- Reduced contract drift between runtime and SDK dispatch paths.
- Stronger safety posture via deterministic bounded output and explicit tool allowlists.
- Enables incremental expansion of the presidential arsenal MCP surface without new binaries.

**Negative:**
- Moderate refactor across both TS dispatch files and tests.
- Existing stdio/contract tests need updates for expanded explicit tool set.

**Risks:**
- LSP one-shot bridging over JSON-RPC framing can be fragile if not bounded and timeout-controlled.
- Existing compatibility wrappers may still mask drift if not asserted by tests.

## Alternatives Considered

1. Extend each dispatch file independently.
Reason rejected: high drift risk and inconsistent bounds/auth behavior.

2. Migrate immediately to one dispatch surface and deprecate the other.
Reason rejected: higher regression risk for current stdio runtime behavior.

3. Keep existing three-tool surface only.
Reason rejected: fails campaign objective of exposing verified arsenal binaries via MCP.

## Validation

- `npm.cmd --prefix mcp_server test` passed.
- RWFI2 contract tests passed.
- KB-lite tests passed.
- `cargo check --all-targets` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
