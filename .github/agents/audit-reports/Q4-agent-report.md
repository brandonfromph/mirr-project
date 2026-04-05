# Q4 Agent Report: MRT Arsenal Naming Audit
Date: 2026-04-05
Scope: mcp_server/src/mrt.ts, mcp_server/src/server.ts, MIRR_ARSENAL_README.md, GEMINI.md, docs/repo-topology.md, docs/consumer-contracts.md

## Executive Verdicts
- Official registered name in MCP server source is "MRT Arsenal": NO.
- Official registered name in MCP server source is "Presidential Arsenal": NO.
- MCP server has an official registered server identifier: YES ("mrt-arsenal").
- Naming is consistent across all scoped files: NO.
- MRT as a named concept is mature and consistent across surfaces: NO (partial maturity with active branding drift).

## Required Command Execution Record
1. Command:
   rg -n "MRT Arsenal|Presidential Arsenal|mrt_arsenal|mirr.arsenal" mcp_server/ docs/ --type ts --type md
   Result: exit code 1, no matches.

2. Command:
   rg -rn "Arsenal" . --type md --type ts --type rs
   Result: command executed; output was affected by rg -r replacement behavior and did not provide clean naming hits.

Note: Supplemental single-file rg scans were used to produce reliable file:line evidence below.

## Evidence Table
| Claim | Evidence | Assessment |
|---|---|---|
| MCP registration name is a machine token, not a prose brand | mcp_server/src/mrt.ts:35 (`name: "mrt-arsenal"`) | Canonical registered identifier is `mrt-arsenal` |
| MCP source comments still use Presidential Arsenal branding | mcp_server/src/mrt.ts:25 | Human-facing naming alias persists in source comments |
| MCP startup log uses MRT Arsenal branding text | mcp_server/src/mrt.ts:377 | Another human-facing variant exists in runtime logs |
| `mcp_server/src/server.ts` is stable on mrt_* method taxonomy | mcp_server/src/server.ts:46-53, mcp_server/src/server.ts:89-96, mcp_server/src/server.ts:964-993 | Operational method namespace is consistent as mrt_* |
| MCP schema advertises a non-MRT top-level schema name | mcp_server/src/server.ts:462 (`name: "local_custom"`) | Top-level discovery identity drifts from `mrt-arsenal` |
| README brands suite as MIRR Arsenal + Presidential framing | MIRR_ARSENAL_README.md:1, MIRR_ARSENAL_README.md:3, MIRR_ARSENAL_README.md:30 | Documentation branding differs from MCP registration token |
| Governance mandates call MRT official while retaining Presidential alias | GEMINI.md:7, GEMINI.md:9 | Official name and alias coexist in same policy document |
| Topology document explicitly calls MRT official but also says Presidential Arsenal | docs/repo-topology.md:16, docs/repo-topology.md:17 | Mixed naming in architecture docs |
| Consumer contract demands stable MRT tool names | docs/consumer-contracts.md:15 | Tool-level naming contract is explicit and stable |
| Arsenal term persists in consumer naming elsewhere | docs/consumer-contracts.md:13, docs/repo-topology.md:21-22 | Arsenal branding remains present in cross-consumer language |

## Naming Drift Maturity Table
| Surface | Current Naming | Maturity |
|---|---|---|
| MCP server registration (`new Server`) | `mrt-arsenal` | consistent |
| MCP method and route namespace | `mrt_*` (audit/brain/general/wave/lsp) | consistent |
| MCP schema public name in `server.ts` | `local_custom` | drifting |
| MCP source comments and logs | "Presidential Arsenal", "MRT Arsenal" | drifting |
| Governance docs (`GEMINI.md`) | "MRT" official + "Presidential Arsenal" alias | partial |
| Topology docs (`docs/repo-topology.md`) | "MRT / Presidential Arsenal" + MRT official statement | partial |
| Brand docs (`MIRR_ARSENAL_README.md`) | "MIRR Arsenal", "Presidential Command Suite" | drifting |
| Consumer contract (`docs/consumer-contracts.md`) | stable MRT tool-name contract | consistent |

## Naming Surface Topology
```text
                            +---------------------------+
                            | Canonical machine IDs     |
                            | - server: mrt-arsenal     |
                            | - tools/routes: mrt_*     |
                            +-------------+-------------+
                                          |
                 +------------------------+------------------------+
                 |                                                 |
       +---------v----------+                           +----------v----------+
       | MCP schema surface |                           | Human-facing labels |
       | server.ts:         |                           | mrt.ts/readme/docs  |
       | name=local_custom  |                           | MRT Arsenal         |
       |                    |                           | Presidential Arsenal|
       +---------+----------+                           | MIRR Arsenal        |
                 |                                      +----------+----------+
                 |                                                 |
                 +------------------------+------------------------+
                                          |
                              +-----------v-----------+
                              | Drift outcome          |
                              | Tool IDs stable,       |
                              | top-level identity and |
                              | branding inconsistent  |
                              +------------------------+
```

## Recommended Canonical Naming Contract
1. Canonical machine identifier:
   - MCP server id: `mrt-arsenal`.
   - Tool and route namespace: `mrt_*`.
2. Canonical human display name:
   - "MRT (MIRR Runtime Tooling)".
3. Allowed historical aliases (docs only):
   - "Presidential Arsenal".
   - "MIRR Arsenal".
4. Disallowed for new protocol identity fields:
   - `local_custom` as top-level MCP schema name.

## Migration Sketch
1. Add naming constants in one source of truth (for example `mcp_server/src/naming.ts`) with fields: `canonical_id`, `canonical_display`, `aliases`.
2. Update `mcp_server/src/mrt.ts` to source both server `name` and startup log text from those constants.
3. Update `mcp_server/src/server.ts` schema payload `name` from `local_custom` to canonical `mrt-arsenal` (or emit both canonical + alias metadata).
4. Keep `mrt_*` methods unchanged (already stable, low-risk).
5. Add regression tests to assert:
   - `new Server(...).name` matches MCP schema `name`.
   - All dispatch/route names remain in `mrt_*` namespace.
6. Normalize docs:
   - First mention always "MRT (MIRR Runtime Tooling)".
   - Move "Presidential Arsenal" and "MIRR Arsenal" into explicit alias note blocks.

## Bottom Line
The MCP server source does not officially register either prose label "MRT Arsenal" or "Presidential Arsenal". The registered identity is `mrt-arsenal`, method taxonomy is consistently `mrt_*`, and the ecosystem is currently in partial maturity with branding drift across source comments, schema identity, and documentation.
