# Q10 Agent Audit Report

## Scope
Audit question: whether MRT is correctly documented and implemented as an AI-powered workflow optimization layer for development of both the compiler and R-SPU, and whether this purpose is reflected in MCP tool surface, CLI army, and governance docs.

Required files reviewed:
- MIRR_ARSENAL_README.md
- docs/repo-topology.md
- docs/roadmap.md
- docs/kb-lite-design.md
- mcp_server/src/mrt.ts

Required commands executed (sequentially):
1. `rg -n "R-SPU|rspu|r_spu|compiler.*workflow|workflow.*compiler" mcp_server/ docs/ src/bin/`
2. `rg -n "mrt_rspu|rspu_tool|r_spu" mcp_server/src/`

## Explicit Verdict
NO.

MRT is only partially aligned with the stated dual-domain purpose. The documentation and code clearly position MRT as a governance/control-plane wrapper, but the MCP surface does not expose explicit R-SPU workflow tools (or broader compiler workflow optimization primitives) despite the repo’s compiler+R-SPU mission.

## Key Findings
1. CLI army docs are governance-heavy and compiler-centric, not explicitly dual-domain.
- `MIRR_ARSENAL_README.md:3` states MIRR Arsenal manages MIRR compiler evolution.
- `MIRR_ARSENAL_README.md:21` to `MIRR_ARSENAL_README.md:25` define `mirr-general` around audit/wave/CI orchestration.
- No explicit R-SPU scope appears in the file.

2. Topology docs define MRT as control plane and MCP bridge, but do not encode compiler+R-SPU workflow contract.
- `docs/repo-topology.md:16` and `docs/repo-topology.md:17` define MRT as the official control-plane toolchain and `mcp_server` as its bridge.
- `docs/repo-topology.md:23` explicitly says `mcp_server` is interface bridge, not logic owner.

3. Strategic roadmap strongly frames MIRR as compiler+R-SPU orchestration, but this is not mirrored in MRT MCP tools.
- `docs/roadmap.md:6` names the project as an R-SPU compiler/EDA roadmap.
- `docs/roadmap.md:42`, `docs/roadmap.md:48`, and `docs/roadmap.md:546` assert compiler-toolchain + runtime-instruction unification.
- `docs/roadmap.md:373` sets explicit R-SPU ISA compilation goals.

4. Governance (KB-lite) documents proposal/campaign plumbing, not dual-domain workflow optimization.
- `docs/kb-lite-design.md:8` scopes KB-lite to governance workflows.
- `docs/kb-lite-design.md:10` to `docs/kb-lite-design.md:13` define data/governance/interface planes with `mcp_server` read/search surfaces.
- No explicit compiler+R-SPU operational workflow contract is stated.

5. MCP implementation exposes governance wrappers only; no explicit R-SPU tool surface.
- `mcp_server/src/mrt.ts:24` to `mcp_server/src/mrt.ts:27` describe MCP server bridge intent.
- Exposed tools are `mrt_audit`, `mrt_brain_get`, `mrt_general_ci`, `mrt_general_ci_compile`, `mrt_general_ci_fast`, `mrt_wave_dry_run`, `mrt_wave_apply`, `mrt_lsp_diagnostics` (`mcp_server/src/mrt.ts:47`, `mcp_server/src/mrt.ts:58`, `mcp_server/src/mrt.ts:69`, `mcp_server/src/mrt.ts:77`, `mcp_server/src/mrt.ts:85`, `mcp_server/src/mrt.ts:93`, `mcp_server/src/mrt.ts:108`, `mcp_server/src/mrt.ts:123`).
- Binary dispatch union contains only `mirr-audit`, `mirr-brain`, `mirr-general`, `mirr-wave`, `mirr-lsp` (`mcp_server/src/mrt.ts:138` to `mcp_server/src/mrt.ts:142`).
- Execution is generic cargo wrapper dispatch (`mcp_server/src/mrt.ts:182` to `mcp_server/src/mrt.ts:183`).
- Required command 2 returned no matches for `mrt_rspu|rspu_tool|r_spu` under `mcp_server/src/`.

6. Underlying CLI binaries do contain R-SPU-capable surfaces, but MCP does not surface them as first-class workflows.
- Command 1 found R-SPU-related coverage in `src/bin/mirr-general.rs` (`proofs-rspu` task at `src/bin/mirr-general.rs:274`, file collection at `src/bin/mirr-general.rs:935`).
- Command 1 found R-SPU simulation wording in `src/bin/mirr-simulate.rs:186`.
- Command 1 found R-SPU emission path in `src/bin/mirr-compile/main.rs:176`, `src/bin/mirr-compile/main.rs:184`, and `src/bin/mirr-compile/main.rs:284`.

## Gap Analysis (Stated Purpose vs Implemented Surface)

| Stated purpose | Implemented surface | Evidence | Gap |
|---|---|---|---|
| MRT should optimize workflows for both compiler and R-SPU development | MCP exposes governance/CI/wave/LSP wrappers; no explicit R-SPU workflow tool | `mcp_server/src/mrt.ts:47`..`123`, command 2 no matches | High |
| MRT should be AI-powered optimization layer | MCP comment references Gemini bridge, but tool contracts are procedural wrappers (no optimization/planning endpoints) | `mcp_server/src/mrt.ts:25`, `mcp_server/src/mrt.ts:182`..`183` | Medium |
| Purpose should be consistently reflected in docs | Roadmap frames compiler+R-SPU mission; Arsenal + KB-lite docs remain governance/control-plane centric | `docs/roadmap.md:6`, `docs/roadmap.md:546`, `MIRR_ARSENAL_README.md:3`, `docs/kb-lite-design.md:8` | High |
| CLI army should represent dual-domain workflows end-to-end | CLI binaries include R-SPU-related features, but MCP tool surface does not map them into first-class API tools | `src/bin/mirr-general.rs:274`, `src/bin/mirr-compile/main.rs:176`, `mcp_server/src/mrt.ts:138`..`142` | High |
| Governance should define dual-domain acceptance criteria | KB-lite and topology docs define planes/bridge roles but not compiler+R-SPU workflow obligations in MCP contract | `docs/repo-topology.md:17`, `docs/kb-lite-design.md:10`..`18` | Medium |

## Topology Diagram (Docs -> MCP -> CLI)

```text
[Governance / Positioning Docs]
  MIRR_ARSENAL_README.md (compiler governance loop)
  docs/repo-topology.md (MRT control-plane + mcp_server bridge)
  docs/kb-lite-design.md (proposal/campaign governance planes)
  docs/roadmap.md (MIRR compiler + R-SPU strategic mission)
                |
                v
[MCP Bridge: mcp_server/src/mrt.ts]
  mrt_audit ------------> mirr-audit
  mrt_brain_get --------> mirr-brain
  mrt_general_ci* ------> mirr-general (ci profiles)
  mrt_wave_* -----------> mirr-wave
  mrt_lsp_diagnostics --> mirr-lsp
  (no mrt_rspu / rspu_tool / compile-rspu endpoint)
                |
                v
[CLI/Binary Capability Layer]
  src/bin/mirr-general.rs (includes proofs-rspu tasks)
  src/bin/mirr-compile/main.rs (rspu emission path)
  src/bin/mirr-simulate.rs (MIRR/R-SPU simulation harness)
```

## Remediation Recommendations

### Code-first (implementation alignment)
1. Add first-class MCP tools for compiler/R-SPU workflows:
- `mrt_compile` with target enum including `rspu`.
- `mrt_rspu_validate` for R-SPU emission/validation pipelines.
- `mrt_rspu_proofs` to expose `proofs/rspu` checks now only indirectly reachable.

2. Extend typed dispatch contract and allowlist:
- Add new `MrtDispatchTool` variants and role gates in `mcp_server/src/mrt.ts` and `mcp_server/src/mrt_kb_lite.ts`.
- Add explicit schema-versioned request/response contracts for new tools.

3. Add MCP tests that assert dual-domain coverage:
- Contract tests for new tools in `mcp_server/tests/`.
- Negative tests proving unauthorized role rejection and bounded output behavior.

### Documentation-first (intent alignment)
1. Update MRT purpose statements in `MIRR_ARSENAL_README.md` to explicitly include both compiler and R-SPU development workflows.
2. Add a “purpose-to-surface matrix” section in `docs/repo-topology.md` mapping each declared MRT responsibility to specific MCP tools and CLI binaries.
3. Extend `docs/kb-lite-design.md` with dual-domain governance acceptance criteria (compiler and R-SPU coverage checks).
4. Add explicit MIRR vs MRT role boundary note in `docs/roadmap.md` to avoid conflating language orchestration mission with current MCP control-plane surface.

## Bottom Line
MRT is implemented as a robust governance/control-plane bridge, and the repo clearly contains compiler+R-SPU capabilities. But the MCP tool surface and governance docs do not yet fully operationalize MRT as an explicit AI-powered workflow optimization layer for both domains. The current state is partial alignment, not full alignment.