# DR-104: Massive MRT Convergence Strategy (Wasm-Host, LSP, KB, Daemon)

**Status:** Accepted and Executed (Convergence Package)
**Date:** 2026-04-05
**Participants:** Researcher-Alpha, Researcher-Beta, Researcher-Gamma, Researcher-Delta

## Context

Proposal 104 requires one massive architecture plan spanning four campaign tracks: staged wasm-host migration, LSP validation/upgrade, KB readiness for daemonization, and MRT daemon architecture. The repo already contains hardened MRT contracts and a real mirr-lsp surface, but unresolved decisions remained on migration sequencing, runtime boundaries, and daemon IPC/security model.

## Decision

Use a staged convergence strategy with explicit readiness gates and rollback.

1. Preserve TS MCP edge as policy authority during transition
2. Treat mirr-lsp as an existing implementation and upgrade it
3. Daemonize KB path first before broader MRT daemon expansion
4. Use local Windows Named Pipe IPC with bounded request/response contracts
5. Enforce strict fail-closed behavior and hard security posture before integrated rollout

## Decision Analysis Summary

| Model | Recommendation | Key Reasoning |
|-------|---------------|---------------|
| Researcher-Alpha | Hybrid staged migration with adapter + shadow mode + rollback | Existing runtime is subprocess-driven and contract-hardened; big-bang replacement is high risk |
| Researcher-Beta | Upgrade existing mirr-lsp, do not rebuild | LSP binary/server are real; main gap is extension/runtime integration and diagnostics depth |
| Researcher-Gamma | Daemonization readiness first via KB path and security hardening | KB-lite stack is strong, but supervision/auth/strictness gaps and plaintext token debt remain |
| Researcher-Delta | Single-owner daemon with Named Pipe IPC and bounded contracts | Deterministic Windows-local IPC and bounded queues/timeouts align with safety constraints |

**Agreements:**
1. No big-bang migration
2. Preserve bounded deterministic contracts at every boundary
3. Require explicit rollback and parity gates before cutover
4. Address security posture before daemon expansion

**Disagreements:**
1. Ordering emphasis differed (LSP-first vs daemon-first), resolved by interleaved 8-wave dependency graph
2. Scope breadth for early daemon rollout differed, resolved by KB-first daemonization as an initial bounded slice

## Consequences

**Positive:**
1. Reduces migration risk while maintaining velocity through staged waves
2. Aligns product reality (real LSP implementation) with proposal direction
3. Keeps architecture deterministic, bounded, and auditable

**Negative:**
1. Requires maintaining temporary dual-path compatibility during transition
2. Increases short-term planning overhead and gate complexity

**Risks:**
1. Route drift between runtime and SDK surfaces; mitigate with shared contract and parity tests
2. False parity confidence from weak comparators; mitigate with normalized semantic diff criteria
3. Daemon security regressions; mitigate with strict schema validation, deny-by-default auth, and bounded IPC

## Alternatives Considered

1. Immediate full wasm-host runtime replacement
- Rejected due to high regression risk and insufficient host-callable extraction in current subprocess-heavy paths

2. Rebuild LSP server from scratch
- Rejected because mirr-lsp already exists and is functional; upgrade path is lower risk and faster

3. Full daemonization of all MRT tools in one wave
- Rejected because it violates bounded staged rollout and would obscure breakage attribution

## Execution Linkage

Proposal execution evidence is recorded in `proposals/evidence/104/WAVE-EXECUTION-2026-04-05.md`.

Execution scope in this session:
1. Wave 1, 6, and 8 hardening edits in runtime/config/test surfaces.
2. Wave 2, 4, 5, and 7 architecture packages with bounded contracts and gate matrices.

Deferred (out of scope for this execution package):
1. Full daemon binary delivery.
2. Full wasm-host backend cutover.
3. VS Code language-client runtime implementation.

## Validation

Validation below confirms Proposal 104 convergence package execution and regression health. It does not claim full daemon binary or wasm-host runtime cutover delivery.

- `npm.cmd --prefix mcp_server run build; npm.cmd --prefix mcp_server test` passed.
- `cargo.exe test --test rwfi2_mrt_contract_tests` passed.
- `cargo.exe check --all-targets` passed.
- `cargo.exe clippy --all-targets -- -D warnings` passed.
- `cargo.exe test --all` passed.
