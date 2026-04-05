# Q2-Agent Report: MRT Arsenal MCP AI/Dev Discovery Audit
Date: 2026-04-05
Scope: mcp_server/src/server.ts, mcp_server/src/mrt.ts, mcp_server/src/mrt_kb_lite.ts

Question: Does MCP provide a mature AI/dev-facing function that lists all available CLI commands with arguments, roles, and descriptions in machine-readable form?

## Explicit Verdict
NO.

There are machine-readable discovery surfaces, but none is a complete, canonical AI/dev contract containing all four required dimensions together:
1. command list
2. argument schema
3. role allowlist
4. description

## Required Command Execution
1) rg -n "list_handlers|mcp_schema|help|describe|tools\(\)" mcp_server/src/
- Key hits: mcp_server/src/server.ts:462, 753, 754, 862

2) rg -n "description.*string|inputSchema" mcp_server/src/mrt.ts
- Key hits: mcp_server/src/mrt.ts:49, 60, 71, 79, 87, 95, 110, 125

## Evidence Table (Path:Line)
| Finding | Evidence | What it proves |
|---|---|---|
| MCP ListTools exists and returns a machine-readable tool list | mcp_server/src/mrt.ts:295, mcp_server/src/mrt.ts:296 | Discovery exists for MRT tools |
| MRT tool catalog entries include descriptions and JSON argument schemas | mcp_server/src/mrt.ts:45, mcp_server/src/mrt.ts:49, mcp_server/src/mrt.ts:60, mcp_server/src/mrt.ts:71, mcp_server/src/mrt.ts:79, mcp_server/src/mrt.ts:87, mcp_server/src/mrt.ts:95, mcp_server/src/mrt.ts:110, mcp_server/src/mrt.ts:125 | Command description + argument schema are machine-readable |
| Role policy exists but is separate from ListTools payload | mcp_server/src/mrt.ts:146, mcp_server/src/mrt.ts:276, mcp_server/src/mrt.ts:289 | Roles are enforced internally, not published in the discovery response |
| CLI invocation path exists but is separate from discovery metadata | mcp_server/src/mrt.ts:163, mcp_server/src/mrt.ts:174, mcp_server/src/mrt.ts:182, mcp_server/src/mrt.ts:184 | Tool-to-CLI mapping and execution are implemented, not exposed as unified catalog metadata |
| Alternate schema endpoint exists in Express server | mcp_server/src/server.ts:462, mcp_server/src/server.ts:465, mcp_server/src/server.ts:605 | A second machine-readable surface exists |
| Express /mcp_schema includes descriptions and parameters but no explicit role field | mcp_server/src/server.ts:547, mcp_server/src/server.ts:549, mcp_server/src/server.ts:555, mcp_server/src/server.ts:557, mcp_server/src/server.ts:587, mcp_server/src/server.ts:589, mcp_server/src/server.ts:599, mcp_server/src/server.ts:601 | Parameters/descriptions are present, role allowlists are not first-class metadata |
| Express server role allowlist is separate from /mcp_schema payload | mcp_server/src/server.ts:44, mcp_server/src/server.ts:45, mcp_server/src/server.ts:55, mcp_server/src/server.ts:226 | Same split-brain pattern as mrt.ts |
| Stdio discovery maps ListTools to /mcp_schema in server.ts | mcp_server/src/server.ts:753, mcp_server/src/server.ts:754 | Discovery behavior depends on route mapping layer, not a dedicated AI/dev catalog API |
| list_handlers endpoint returns only route names | mcp_server/src/server.ts:862, mcp_server/src/server.ts:863 | Not sufficient: no args/roles/descriptions contract |
| mcp_server/src/mrt_kb_lite.ts contains arg builders and constraints, not a discovery catalog | mcp_server/src/mrt_kb_lite.ts:9, mcp_server/src/mrt_kb_lite.ts:80, mcp_server/src/mrt_kb_lite.ts:86, mcp_server/src/mrt_kb_lite.ts:90, mcp_server/src/mrt_kb_lite.ts:107, mcp_server/src/mrt_kb_lite.ts:133, mcp_server/src/mrt_kb_lite.ts:141, mcp_server/src/mrt_kb_lite.ts:149 | Argument construction is distributed across implementation files |

## Maturity Assessment
| Dimension | Status | Evidence | Gap |
|---|---|---|---|
| Machine-readable command discovery | Partial | mcp_server/src/mrt.ts:295, mcp_server/src/server.ts:462 | Two surfaces, no single canonical contract |
| Argument schema quality | Good | mcp_server/src/mrt.ts:49, mcp_server/src/mrt.ts:60, mcp_server/src/mrt.ts:110, mcp_server/src/mrt.ts:125 | Schemas exist, but split from role and CLI metadata |
| Role visibility in discovery | Immature | mcp_server/src/mrt.ts:146, mcp_server/src/server.ts:45 | Roles enforced only at runtime policy layer |
| Description coverage | Partial-Good | mcp_server/src/mrt.ts:45, mcp_server/src/server.ts:547 | Present, but fragmented across formats/surfaces |
| Unified CLI mapping in discovery payload | Immature | mcp_server/src/mrt.ts:163, mcp_server/src/mrt.ts:174, mcp_server/src/mrt_kb_lite.ts:80 | CLI mapping/arg resolvers are not emitted in discovery API |
| Single source of truth | Immature | mcp_server/src/mrt.ts:45, mcp_server/src/server.ts:462, mcp_server/src/mrt_kb_lite.ts:9 | Metadata duplicated and distributed |

## Topology/Pipeline Diagram
```mermaid
flowchart TD
  C[AI/Dev Client] --> D1[ListToolsRequestSchema]
  D1 --> D2[mcp_server/src/mrt.ts:295]
  D2 --> D3[tools: MRT_TOOLS]
  D3 --> D4[name + description + inputSchema]
  D4 -. roles not embedded .-> R1[TOOL_ROLE_ALLOWLIST]

  C --> S1[stdio JSON-RPC ListTools]
  S1 --> S2[mcp_server/src/server.ts:753-754]
  S2 --> S3[/mcp_schema]
  S3 --> S4[methods + parameters + description]
  S4 -. roles not embedded .-> R2[toolRoleAllowlist]

  C --> I1[CallTool or /mrt_* route]
  I1 --> I2[role checks]
  I2 --> I3[resolve args]
  I3 --> I4[cargo run --bin mirr-*]
```

## Code-First Remediation Sketch (Specific Files/Functions)
Because verdict is NO, remediation is required.

1. Add canonical catalog module
- File: mcp_server/src/mrt_catalog.ts (new)
- Export typed entries including:
  - tool id
  - description
  - inputSchema
  - allowedRoles
  - cliBin
  - argBuilderId
  - route

2. Refactor mrt.ts to consume catalog
- Replace const MRT_TOOLS in mcp_server/src/mrt.ts:45 with projection from mcp_server/src/mrt_catalog.ts
- Replace const TOOL_ROLE_ALLOWLIST in mcp_server/src/mrt.ts:146 with projection from catalog
- Keep setRequestHandler(ListToolsRequestSchema) in mcp_server/src/mrt.ts:295, but enrich payload to include roles + cli metadata for AI/dev consumers

3. Refactor server.ts /mcp_schema builder to consume same catalog
- Replace hardcoded schema object in mcp_server/src/server.ts:462 with generated object from mcp_server/src/mrt_catalog.ts
- Keep backwards-compatible fields, add explicit machine fields:
  - roles: string[]
  - cli.bin
  - cli.argBuilder

4. Expose dedicated AI/dev catalog endpoint
- Add GET /mrt_catalog in mcp_server/src/server.ts
- Return versioned contract (schema_version, generated_at, tools[])

5. Centralize argument-resolver registry
- In mcp_server/src/mrt_kb_lite.ts, export a registry map from argBuilderId to function (brainGetArgs, generalCiCompileArgs, generalCiFastArgs, waveDryRunArgs, waveApplyArgs, lspDiagnosticsInvocation)
- Use this from both mcp_server/src/mrt.ts and mcp_server/src/server.ts invocation paths to eliminate drift

6. Add contract tests
- Ensure every route and every MrtDispatchTool has exactly one catalog entry
- Ensure roles in enforcement path equal roles emitted in discovery payload
- Ensure ListTools payload and /mrt_catalog payload stay in sync

READY FOR ORCHESTRATOR
