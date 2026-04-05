# Q9 Agent Report - MRT Surface Maturity and LRA Exposure Audit
Date: 2026-04-05
Scope: c:\Users\elvie\nasa-rust-project

## Audited Files
- mcp_server/src/server.ts
- mcp_server/src/mrt.ts
- crates/lra-cli/src/main.rs
- docs/consumer-contracts.md

## 1) Explicit YES/NO Verdicts
- LRA subcommands from lra-cli exposed through the MCP server: NO.
- Overall MRT maturity fully mature across all audited MRT surfaces: NO.

## 2) Evidence Table (Exact Paths + Lines)

| Finding | Evidence (path:line) | Assessment |
|---|---|---|
| LRA command surface exists in lra-cli | crates/lra-cli/src/main.rs:36, 38, 43, 49, 55, 61, 70, 82, 88, 96, 105, 110, 123, 139, 141, 152, 160, 166, 174 | 18 LRA subcommands are defined in the CLI contract. |
| LRA command dispatch is implemented | crates/lra-cli/src/main.rs:185-259 | Each LRA subcommand has an execution branch. |
| lra-cli compile path uses compiler library entrypoints | crates/lra-cli/src/main.rs:6, 198-199, 265-314; docs/consumer-contracts.md:14 | lra-cli compile path aligns with consumer contract. |
| MCP SDK tool surface contains only mrt_* tools | mcp_server/src/mrt.ts:45, 47, 58, 69, 77, 85, 93, 108, 123 | No lra_* tools are declared in MRT_TOOLS. |
| SDK dispatch binary map contains only mirr-* bins | mcp_server/src/mrt.ts:137-142, 174-179 | No lra binary target is dispatchable in mrt.ts. |
| SDK CallTool switch only implements mrt_* handlers | mcp_server/src/mrt.ts:304-360 | No LRA tool dispatch path exists in the MCP SDK server. |
| server.ts MCP schema advertises only mrt_* MRT tools | mcp_server/src/server.ts:462, 546, 554, 559, 564, 569, 574, 586, 598 | MCP schema has no LRA tool contract entries. |
| server.ts active MRT routes expose only mrt_* endpoints | mcp_server/src/server.ts:964, 968, 972, 976, 980, 984, 988, 992, 996 | Route exposure has no /mrt_lra_* endpoints. |
| Generic cargo bridge cannot invoke lra subcommands | mcp_server/src/server.ts:1189-1201 | run_cargo is restricted to build/test/check; no cargo run exposure for lra-cli. |
| MRT role allowlists are present and explicit for current tools | mcp_server/src/server.ts:45-53; mcp_server/src/mrt.ts:146-154 | Existing MRT surfaces are allowlisted, but LRA tools are absent from policy. |
| MRT surface has bounded execution controls | mcp_server/src/server.ts:9, 184, 215, 252, 672, 823, 826; mcp_server/src/mrt.ts:182-189, 230-235 | Output and concurrency bounds exist for implemented MRT tools. |
| SDK path accepts role from request arguments | mcp_server/src/mrt.ts:277-285, 305-356 | Role trust model is argument-derived in mrt.ts, which is weaker than server-side token-bound role validation. |
| Consumer contract requires typed, allowlisted, stable tool routing for mcp_server | docs/consumer-contracts.md:15 | LRA absence is a feature-coverage gap against expected MCP capability breadth. |

## 3) Three-Column Maturity Table for Every MRT Surface

Legend: X marks the current maturity classification.

| MRT Surface | Partially Implemented | Partially Mature | Fully Mature |
|---|---|---|---|
| mrt_audit |  | X |  |
| mrt_brain_get |  | X |  |
| mrt_general_ci |  | X |  |
| mrt_general_ci_compile |  | X |  |
| mrt_general_ci_fast |  | X |  |
| mrt_wave_dry_run |  | X |  |
| mrt_wave_apply |  | X |  |
| mrt_lsp_diagnostics |  | X |  |
| mrt_execute compatibility endpoint | X |  |  |
| run_cargo bridge for possible CLI delegation | X |  |  |
| LRA subcommand MCP exposure surface | X |  |  |

Rationale for table:
- All eight mrt_* tools are implemented and bounded, but remain partially mature because the duplicated server surfaces (mcp_server/src/server.ts and mcp_server/src/mrt.ts) can drift and mrt.ts consumes role from client arguments instead of token-bound server auth.
- mrt_execute is compatibility-only and disabled by default (feature-gated), so it is partially implemented by design.
- run_cargo does not provide a typed LRA MCP contract and cannot run lra-cli commands, so it cannot satisfy LRA MCP exposure requirements.
- The LRA exposure surface is currently missing explicit tool declarations, dispatch wiring, and routes.

## 4) Surface Topology/Pipeline Diagram (Mermaid)

```mermaid
flowchart TD
    Client[MCP Client]

    Client --> STDIO_DIRECT[mcp_server/src/server.ts\nstdio-direct JSON-RPC adapter]
    Client --> SDK_STDIO[mcp_server/src/mrt.ts\nMCP SDK StdioServerTransport]

    STDIO_DIRECT --> S_ROUTES[/mrt_audit, /mrt_brain_get, /mrt_general_ci,\n/mrt_general_ci_compile, /mrt_general_ci_fast,\n/mrt_wave_dry_run, /mrt_wave_apply, /mrt_lsp_diagnostics/]
    SDK_STDIO --> SDK_TOOLS[MRT_TOOLS + CallTool switch\n(mrt_audit..mrt_lsp_diagnostics)]

    S_ROUTES --> MIRR_BINS[cargo run --bin mirr-audit|mirr-brain|mirr-general|mirr-wave|mirr-lsp]
    SDK_TOOLS --> MIRR_BINS

    STDIO_DIRECT --> RUN_CARGO[/run_cargo (build|test|check only)]

    LRA_CLI[crates/lra-cli/src/main.rs\n18 subcommands]

    S_ROUTES -. missing explicit /mrt_lra_* routes .-> LRA_CLI
    SDK_TOOLS -. missing lra_* tool declarations/handlers .-> LRA_CLI
    RUN_CARGO -. cannot call cargo run -p lra-cli .-> LRA_CLI
```

## 5) Exactly What Is Missing For LRA MCP Exposure

### Missing MCP Tool Surface for Each LRA Subcommand

| lra-cli subcommand (crates/lra-cli/src/main.rs) | Exposed in MCP now? | Missing MCP artifact(s) |
|---|---|---|
| init | NO | No mrt_lra_init tool declaration in mcp_server/src/mrt.ts and no /mrt_lra_init route in mcp_server/src/server.ts |
| validate | NO | No mrt_lra_validate declaration, dispatch case, or route |
| serve | NO | No mrt_lra_serve declaration, dispatch case, or route |
| badge | NO | No mrt_lra_badge declaration, dispatch case, or route |
| build | NO | No mrt_lra_build declaration, dispatch case, or route |
| build-docs | NO | No mrt_lra_build_docs declaration, dispatch case, or route |
| hash | NO | No mrt_lra_hash declaration, dispatch case, or route |
| search | NO | No mrt_lra_search declaration, dispatch case, or route |
| deps | NO | No mrt_lra_deps declaration, dispatch case, or route |
| health | NO | No mrt_lra_health declaration, dispatch case, or route |
| compile | NO | No mrt_lra_compile declaration, dispatch case, or route |
| receipt | NO | No mrt_lra_receipt declaration, dispatch case, or route |
| keygen | NO | No mrt_lra_keygen declaration, dispatch case, or route |
| verify | NO | No mrt_lra_verify declaration, dispatch case, or route |
| sign | NO | No mrt_lra_sign declaration, dispatch case, or route |
| status | NO | No mrt_lra_status declaration, dispatch case, or route |
| crawl | NO | No mrt_lra_crawl declaration, dispatch case, or route |
| verify-receipt | NO | No mrt_lra_verify_receipt declaration, dispatch case, or route |

### Concrete Implementation-First Gap Closure Plan

1. Add explicit LRA MCP contracts first (typed schemas).
   - Update mcp_server/src/mrt.ts MRT_TOOLS with mrt_lra_* tool declarations and required fields.
   - Update mcp_server/src/server.ts /mcp_schema methods with matching mrt_lra_* entries.

2. Add dispatch wiring second (no behavior change to existing MRT tools).
   - Extend mcp_server/src/mrt.ts type unions and dispatcher switch to include LRA handlers.
   - Add an lra execution target in the dispatch map (for example through cargo run -p lra-cli -- <subcommand ...> or an equivalent deterministic executable path).

3. Add server-side route surface and RBAC policy third.
   - Extend mcp_server/src/server.ts requireMrtDispatchRole allowlist with lra tool names.
   - Extend isMrtDispatchTool and resolveMrtInvocation for each mrt_lra_* mapping.
   - Register explicit /mrt_lra_* routes (no reliance on compatibility fallback).

4. Close role-trust gap in mrt.ts while touching dispatch.
   - Stop trusting role from tool arguments in mcp_server/src/mrt.ts; bind role to verified server auth context to match server.ts trust model.

5. Add compatibility and regression tests last.
   - Positive tests: each new mrt_lra_* tool reaches correct lra subcommand.
   - Negative tests: role escalation attempts fail; unknown tool fails closed.
   - Contract tests: /mcp_schema includes stable typed LRA tool entries.

6. Update contract documentation after implementation.
   - Update docs/consumer-contracts.md to explicitly state LRA MCP exposure guarantees and stable mrt_lra_* tool naming.

## Additional Validation Run
- Command run: cargo.exe test -p lra-cli --all-targets
- Result: PASS (all listed tests passed in current workspace run).

READY FOR ORCHESTRATOR
