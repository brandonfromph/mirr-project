# PROPOSAL 096: REPO-WIDE FOUNDATIONAL INTEGRATION

**Proposal #:** 096
**Campaign ID:** REPO-WIDE-FOUNDATIONAL-INTEGRATION
**Status:** EXECUTED
**Scope Class:** ARCHITECTURE (10+ files, multi-subsystem, multi-consumer)
**Date:** 2026-04-02

## Executive Summary

Proposal 096 is a foundational campaign for the repository as a system. It defines permanent contracts and concrete implementation anchors across first-class consumers so future campaigns do not re-audit topology, consumer relationships, and KB boundaries.

This proposal is not a cleanup campaign and not a docs polish campaign. It establishes load-bearing interfaces and wave-locked transitions:
- WASM parity is anchored to the compiler library surface (`run_pipeline`), with `mirr-compile` used as a parity reference surface.
- LRA compile moves from shell-out to direct compiler library calls in a pinned wave.
- MRT moves from ad-hoc cargo wrappers to a typed interface contract.
- KB-lite is anchored to the existing `.kb-data` store and concrete scripts.

## Philosophy Gate

This campaign passes the MIRR philosophy gate.

1. The generative power of three is preserved. No new MIRR language construct is introduced.
2. NASA Power-of-10 is preserved. No recursion or unbounded iteration is introduced.
3. Hardware synthesizability is preserved. No unsupported RTL construct is introduced.
4. Properties remain verification-only. No hardware semantics are assigned to property declarations.
5. Zero-Debt is enforced through explicit contract ownership, wave-locked adapter removal, and no-deletion-by-default governance.

## Scope Detection

This is an ARCHITECTURE campaign because it spans:
- repository topology and contract authority,
- first-class consumer behavior (WASM, LRA, MRT/MCP, VS Code, demos),
- compiler API boundary usage across non-compiler crates,
- KB-lite operating boundary grounded in existing repository state.

## Current State Assessment

| Area | Status | Evidence |
|------|--------|----------|
| Compiler feature surface (post-095) | Complete, larger than WASM exports | [src/bin/mirr-compile/main.rs#L254](../src/bin/mirr-compile/main.rs#L254), [src/bin/mirr-compile/main.rs#L263](../src/bin/mirr-compile/main.rs#L263), [src/bin/mirr-compile/main.rs#L324](../src/bin/mirr-compile/main.rs#L324) |
| WASM exported API (14 functions) | Partial parity | [crates/mirr-wasm/src/lib.rs#L168](../crates/mirr-wasm/src/lib.rs#L168), [crates/mirr-wasm/src/lib.rs#L626](../crates/mirr-wasm/src/lib.rs#L626) |
| Arsenal WASM consumer surface | Present as first-party crate, contract not yet gated in acceptance bundle | [crates/mirr-arsenal-wasm/Cargo.toml#L1](../crates/mirr-arsenal-wasm/Cargo.toml#L1) |
| LRA compile path | Wrapper-based shell-out | [crates/lra-cli/src/main.rs#L197](../crates/lra-cli/src/main.rs#L197), [crates/lra-cli/src/main.rs#L200](../crates/lra-cli/src/main.rs#L200) |
| Compiler library entrypoint availability | Present and re-exported | [src/lib.rs#L71](../src/lib.rs#L71) |
| MRT adapter implementation | Cargo-wrapper execution in tool handler | [mcp_server/src/mrt.ts#L8](../mcp_server/src/mrt.ts#L8), [mcp_server/src/mrt.ts#L74](../mcp_server/src/mrt.ts#L74), [mcp_server/src/mrt.ts#L83](../mcp_server/src/mrt.ts#L83) |
| VS Code extension contract clarity | Syntax/theme surface only, contract text under-specified | [vscode-mirr/package.json#L4](../vscode-mirr/package.json#L4), [vscode-mirr/package.json#L20](../vscode-mirr/package.json#L20) |
| KB data substrate | Present on disk and already populated | [.kb-data](../.kb-data), [.kb-data/knowledge.lance](../.kb-data/knowledge.lance), [.kb-data/graph.db](../.kb-data/graph.db) |
| Demos/proofs/fuzz/scripts integration gates | Present in repository, not yet wired into consumer-wave gating | [demos](../demos), [proofs](../proofs), [fuzz](../fuzz), [scripts](../scripts) |

## Debt Audit

| # | Prohibition | Findings in scope | Action |
|---|-------------|-------------------|--------|
| D1 | No wrapper functions | Found: LRA compile shells out to `cargo run --bin mirr-compile`; MRT tools shell out through `execSync(cargo run ...)` | Replace with direct library call in LRA (Wave 2) and typed MCP tool runner in MRT (Wave 3) |
| D2 | No deprecated aliases | Found: deprecated warning path still used by active compile command in LRA | Remove deprecated shell-out path at Wave 2 closeout |
| D3 | No dead code | Found: parity-related surfaces exist but are not mapped into consumer contracts | Bind each surface to explicit parity contract and acceptance gate |
| D4 | No redundant abstractions | Found: duplicate "wrapper" semantics in LRA and MRT for compiler invocation | Consolidate invocation through one explicit API contract per consumer |
| D5 | No backward-compat shims | None required for 096 scope | N/A |
| D6 | No duplicate logic | Found: multiple consumer paths re-encode compilation behavior inconsistently | Introduce common compiler API usage pattern via `run_pipeline` |
| D7 | No misleading comments | Found: consumer maturity statements exceed implementation in some surfaces | Align text with concrete line-level capabilities and limits |

## Risk Analysis

| # | Risk | Severity | Mitigation |
|---|------|----------|------------|
| R1 | WASM parity drift against post-095 compiler features | High | Function-by-function parity table with explicit signature deltas and acceptance gates |
| R2 | LRA direct integration introduces compile regressions | High | Pin replacement to Wave 2, keep tests green (`cargo test -p lra-cli`, `cargo test --all`) |
| R3 | MRT remains a cargo-wrapper layer with unclear contracts | High | Replace wrapper calls with a typed interface contract and allowed tool map |
| R4 | KB-lite scope balloons into platform redesign | Medium | Anchor strictly to existing `.kb-data` artifacts plus two existing scripts |
| R5 | VS Code package overstates service capabilities | Medium | Pin one concrete manifest edit to declare truthful contract |

## Constraints

| # | Constraint | Source | Enforced by |
|---|-----------|--------|-------------|
| C1 | No unsafe code | NASA Power-of-10 | Existing crate policy and CI checks |
| C2 | Zero warnings | CI | `cargo clippy --all-targets -- -D warnings` |
| C3 | Bounded algorithms only | NASA Power-of-10 | Fixed caps and finite loops only |
| C4 | Backward compatibility by default | Repo policy | Existing and added parity/consumer tests |
| C5 | No deletion as default response | Governance | Contractized transitions, explicit wave exits |
| C6 | Foundational over cleanup framing | Campaign requirement | Keep implementation anchors and permanent contracts in proposal body |

## Proposal

### Section 0: Repo Governance Contracts

This section is the durable contract layer for repository-wide work.

#### 0.A: Repo Topology Authority Contract

A single canonical topology model defines first-party projects and relationships. All derivative docs and instructions must reference, not redefine, that model.

#### 0.B: First-Class Consumer Contract

WASM, LRA, MRT/MCP, VS Code, demos, paper demo mirrors (`paper/demos`, `docs/paper/demos`), proofs, fuzz, and scripts are first-class consumers. Compiler changes must declare downstream impact on these surfaces.

#### 0.C: KB-Lite Contract (Interim Authority Assigned)

KB-lite is defined as a concrete repository substrate and retrieval scope anchored to existing `.kb-data` artifacts and script-level governance checks. Contract authority is assigned to repository governance maintainers for Proposal 096 execution and closeout.

#### 0.D: No-Surprise Compatibility Contract

No consumer should experience undeclared breakage in emitted formats, diagnostics, command behavior, or entrypoints.

Declared-exception rule:
- A known failing baseline may be carried only as an explicitly declared temporary exception with: artifact evidence, owning wave, and closure gate.
- Undeclared failures remain merge-blocking.
- Declared exceptions are treated as temporary compatibility debt, not as compatibility pass, and must not be counted as no-surprise pass conditions.

Compatibility policy for new surfaces in 096:
- New WASM and LRA interfaces introduced by 096 are additive-first.
- Existing exported function names and current command semantics remain valid for one full campaign cycle after new interfaces land.
- Removal or signature-tightening requires a follow-on proposal with explicit migration notes and parity evidence.

#### 0.E: No-Deletion Default Contract

Deletion is not the default mechanism. If a path is removed, it must be justified as the least-risk route after compatibility evidence is recorded.

#### Contract Ownership and Verification

Proposal 096 signoff owner roster (accountable approvers):
- Compiler/API: elvie (primary), Code-Reviewer-Alpha (secondary)
- WASM + Arsenal surfaces: elvie (primary), Architect-Reviewer-Alpha (secondary)
- LRA surface: elvie (primary), Code-Reviewer-Beta (secondary)
- MRT/MCP surface: elvie (primary), Architect-Reviewer-Beta (secondary)
- Docs/contracts: elvie (primary), Documenter reviewer role (secondary)
- VS Code surface: elvie (primary), Code-Reviewer-Alpha (secondary)
- Demos + paper demo mirrors: elvie (primary), Architect-Reviewer-Alpha (secondary)
- Proofs surface: elvie (primary), Architect-Reviewer-Beta (secondary)
- Fuzz surface: elvie (primary), Code-Reviewer-Beta (secondary)
- Scripts/EDA surface: elvie (primary), Code-Reviewer-Alpha (secondary)

| Contract | Owner | Verifier | Failure mode |
|---|---|---|---|
| 0.A Repo Topology Authority | Compiler/API owner + docs/contracts owner | Proposal reviewers | Conflicting topology narratives |
| 0.B First-Class Consumer Matrix | Named owner per consumer surface | Parity gate suite | Drift across consumer behaviors |
| 0.C KB-Lite | Docs/contracts owner | Proposal reviewers | Contract must remain aligned with KB-lite scope and evidence commands |
| 0.D No-Surprise Compatibility | Named release/consumer owner | Consumer acceptance bundle | Silent behavior drift |
| 0.E No-Deletion Default | Compiler/API owner + docs/contracts owner | Proposal review | Cleanup-driven breakage |

#### Contract Verification Rules

| Contract | Verification rule | Pass condition | Failure action |
|---|---|---|---|
| 0.A | Canonical topology alignment check | Shared topology references match canonical source | Block merge |
| 0.B | Per-consumer acceptance execution | All first-class surfaces have pass/fail evidence | Block merge |
| 0.C | Explicit owner and executable KB evidence commands | Owner is declared and KB evidence commands are present and runnable | Block merge until owner and commands are present |
| 0.D | Protocol and parity checks | No undeclared behavior drift | Block merge and require delta note |
| 0.E | Diff audit for deletion bias | No unjustified deletion-first approach | Reject wave |

#### Contract Dependency DAG

```text
Wave 0: Contract acceptance + parity baseline capture
  -> Wave 1: WASM parity closure
    -> Wave 2: LRA direct compiler API transition
      -> Wave 3: MRT typed interface + KB-lite script scope
        -> Wave 4A: Demos/scripts + paper mirrors
          -> Parallel Wave 4B: Proofs, Wave 4C: Fuzz, Wave 4D: Arsenal/integration
            -> Wave 5: Public contract sync and closeout
```

## Section A: Topology and Contract Authority

| # | File:Line | Current | Proposed | Rationale |
|---|-----------|---------|----------|-----------|
| A1 | `AGENTS.md:1` | Repo-scale guidance exists but drifts by repetition | Keep one authority model and reference it from other docs | Prevent topology drift across campaigns |
| A2 | `docs/repo-topology.md:1` | Canonical topology not yet established in this wave | Create/refresh canonical topology doc used by all campaign docs | Load-bearing contract for future work |
| A3 | `docs/consumer-contracts.md:1` | Consumer contracts distributed across files | Keep one consumer matrix and link from README/home/index | Stable surface for downstream campaigns |

## Section B: First-Class Consumer Integration (Implementation-Anchored)

### B1. WASM Parity Matrix for All 14 Exported Functions

Evidence baseline:
- WASM export set: [crates/mirr-wasm/src/lib.rs#L168](../crates/mirr-wasm/src/lib.rs#L168) through [crates/mirr-wasm/src/lib.rs#L626](../crates/mirr-wasm/src/lib.rs#L626)
- Compiler feature surface: [src/bin/mirr-compile/main.rs#L254](../src/bin/mirr-compile/main.rs#L254) through [src/bin/mirr-compile/main.rs#L324](../src/bin/mirr-compile/main.rs#L324)

| Export function | Parity status vs post-095 compiler surface | Required delta to expose parity |
|---|---|---|
| `compile_verilog` | **Partial** (no target/dsp/sva options) | Add `compile_verilog_with_options(source, target, dsp_threshold, strip_sva)` and keep current signature as default |
| `compile_firrtl` | **Current** | No change required |
| `compile_sexpr` | **Current** | No change required |
| `compile_dot` | **Partial** (no `--dot-detail expr` path) | Add `compile_dot_with_detail(source, detail_expr)` |
| `compile_rspu` | **Partial** (no target-select parity for riscv/arm/cert) | Add `compile_target(source, target)` and `compile_cert(source)` |
| `compile_verilog_sat` | **Current (WASM-specific extension)** | Keep as documented WASM extension |
| `compile_graph_data` | **Current (WASM visualization helper)** | Keep as helper; explicitly non-CLI parity surface |
| `infer_widths` | **Partial** (returns JSON string but not compiler-named surface) | Add `compile_json_netlist(source)` alias and align error payload shape |
| `simulate_waveform` | **Partial** (synthetic signal model) | Add `simulate_waveform_rspu(source, cycles)` path using pipeline simulation outputs |
| `simulate_rspu` | **Partial** (text blob only) | Return structured JSON payload with current text as optional field |
| `simulate_mapek` | **Partial** (missing explicit mape-k-rtl parity endpoint) | Add `compile_mapek_rtl(source)` endpoint |
| `mirr_version` | **Current** | No change required |
| `compile_pipeline_stages` | **Partial** (boolean-only stage result) | Include stage diagnostics and emitted-target availability map |
| `proof_status` | **Non-parity metadata endpoint** | Keep as metadata endpoint; explicitly outside `mirr-compile` parity contract |

New parity-closing WASM signatures (Wave 1-2 scope):

Boundary rule:
- Preserve a stable WASM adapter boundary; do not mirror every CLI-specific target as a separate mandatory endpoint.
- Prefer one generic target-aware entrypoint where possible.

```rust
#[wasm_bindgen]
pub fn compile_verilog_with_options(
    source: &str,
    target: &str,
    dsp_threshold: u32,
    strip_sva: bool,
) -> String;

#[wasm_bindgen]
pub fn compile_dot_with_detail(source: &str, detail_expr: bool) -> String;

#[wasm_bindgen]
pub fn compile_json_netlist(source: &str) -> String;

#[wasm_bindgen]
pub fn compile_target(source: &str, target: &str) -> String;

#[wasm_bindgen]
pub fn compile_mapek_rtl(source: &str) -> String;

#[wasm_bindgen]
pub fn compile_cert(source: &str) -> String;
```

### B2. LRA Compile Path: Replace Shell-Out at Pinned Site (Wave 2)

Pinned replacement site:
- [crates/lra-cli/src/main.rs#L197](../crates/lra-cli/src/main.rs#L197)
- [crates/lra-cli/src/main.rs#L198](../crates/lra-cli/src/main.rs#L198)
- [crates/lra-cli/src/main.rs#L200](../crates/lra-cli/src/main.rs#L200)

Current site to replace:

```rust
Command::Compile { source, target, output } => {
    legacy::warn_deprecated("compile");
    std::process::Command::new("cargo")...
}
```

Replacement call site and signature:

```rust
Command::Compile { source, target, output } => {
    compile_via_library(&source, &target, output.as_deref())
}

fn compile_via_library(source: &str, target: &str, output: Option<&str>) -> i32
```

Compiler crate entrypoint used by the replacement:
- `nasa_rust_project::run_pipeline` from [src/lib.rs#L71](../src/lib.rs#L71)

Wave lock:
- Adapter transition is pinned to **Wave 2**.
- Shell-out path is not permitted after Wave 2 closeout.

### B3. MRT: From Cargo Wrapper Calls to Typed Interface Contract

Current wrapper anchors:
- [mcp_server/src/mrt.ts#L8](../mcp_server/src/mrt.ts#L8)
- [mcp_server/src/mrt.ts#L74](../mcp_server/src/mrt.ts#L74)
- [mcp_server/src/mrt.ts#L79](../mcp_server/src/mrt.ts#L79)
- [mcp_server/src/mrt.ts#L83](../mcp_server/src/mrt.ts#L83)

Current behavior executes `cargo run --bin ...` from tool handlers.

Required implementation delta:
1. Introduce typed interface runner in `mcp_server/src/mrt.ts`:

```ts
async function callMrtInterface(
  tool: "mirr-audit" | "mirr-brain" | "mirr-general",
  args: string[]
): Promise<string>
```

Typed MRT contract requirements:
- Request schema version field: `schema_version: "1"`.
- Tool input/output payloads must be JSON objects with stable keys per tool.
- Error responses must include `code` and `message` fields.
- Any schema break requires explicit version bump and migration note in proposal evidence.

2. Route `mrt_audit`, `mrt_brain_get`, and `mrt_general_ci` through this typed runner.
3. Keep binary allowlist explicit and reject unknown tool names.
4. Remove malformed trailing duplicate tail in `mcp_server/src/mrt.ts` as part of Wave 3 integrity hardening.

### B4. VS Code Contract: One Concrete Manifest Change

Concrete line-level change anchored in existing manifest:
- Current: [vscode-mirr/package.json#L4](../vscode-mirr/package.json#L4)

Proposed line change:
- Replace description with explicit contract text:
  - From: `"Syntax highlighting, file icon, and color theme for the MIRR safety-critical HDL"`
  - To: `"Syntax highlighting, icon, and theme for MIRR (no LSP or compiler service in this package)."`

This makes the extension contract truthful and removes ambiguity about runtime compiler service behavior.

### LRA Command Utility Matrix (retained)

| Command group | Current utility in repo integration | Contract status | Action (no deletion) |
|---|---|---|---|
| `lra author <init|build|serve|build-docs>` | Directly useful for content and docs flows | Keep as first-class | Wire into docs and artifact pipeline contracts |
| `lra verify <validate|hash|verify|verify-receipt>` | Directly useful for integrity and release receipts | Keep as first-class | Tie to proposal/artifact verification gates |
| `lra network <search|deps|health|status|crawl>` | Useful for ecosystem observability | Keep as optional tier | Keep documented and non-blocking |
| `lra arsenal <compile|receipt|sign|keygen>` | Compiler and artifact bridge | Keep as first-class | Wire to direct compiler API behavior |
| High-resolution commands (`badge`, etc.) | Lower frequency but valid | Keep as retained capability | Keep measurable usage, no deletion |

#### LRA Adapter Exit Criteria (Pinned)

| Item | Requirement |
|---|---|
| Owner | `crates/lra-cli` maintainers + compiler API maintainers |
| Pinned wave | **Wave 2** |
| Transition end-state | `lra compile` calls compiler library entrypoint directly |
| Removal trigger | `cargo test -p lra-cli` and `cargo test --all` pass with shell-out path removed |
| Failure condition | If shell-out path remains after Wave 2, Proposal 096 cannot close |

## Backward Compatibility Matrix

Baseline evidence captured:
- [proposals/evidence/096/cargo-test-all-baseline.log](../proposals/evidence/096/cargo-test-all-baseline.log)
- [proposals/evidence/096/mcp-stdio-baseline.log](../proposals/evidence/096/mcp-stdio-baseline.log)
- Baseline aggregate: 3350 passed, 0 failed, 1 ignored

| Surface | Candidate tests | Baseline expectation | Post-change expectation |
|---|---|---|---|
| Compiler parity and self-hosting | `tests/self_hosting_parity_tests.rs`, `tests/bootstrap_parity_tests.rs` | Pass | Pass (no count decrease) |
| LRA compile behavior | `crates/lra-cli/tests/build_test.rs`, `crates/lra-cli/tests/validate_test.rs` | Pass | Pass (same assertions) |
| MAPE-K and emitted output paths touched by new wasm parity endpoints | `tests/mape_k_tests.rs`, `tests/mape_k_rtl_core_tests.rs`, `tests/mape_k_integration_core_tests.rs` | Pass | Pass (no diagnostic regressions) |
| MCP bridge behavior | `mcp_server/tests/stdio_proxy_test.js` | Known failing baseline (invalid_api_key path in current stdio test) | Pass after Wave 3 typed-dispatch and auth alignment |
| Workspace-wide regression gate | `cargo test --all` | 3350 passed, 0 failed, 1 ignored | >=3350 passed, 0 failed, and no unplanned count regression |

## Section C: KB-Lite (Grounded in Existing `.kb-data`)

### C1. Existing Tech Stack and File Locations

Observed repository data substrate:
- `.kb-data/graph.db` (SQLite graph store, currently 1,589,248 bytes)
- `.kb-data/knowledge.lance/data` (77 lance data files)
- `.kb-data/knowledge.lance/_transactions` (76 files)
- `.kb-data/knowledge.lance/_versions` (76 files)

Contractized KB-lite stack for 096:
1. Data plane: `.kb-data/knowledge.lance` + `.kb-data/graph.db`
2. Governance plane: `scripts/validate_proposals.py` + `scripts/repo_metrics.py`
3. Interface plane: `mcp_server/src/server.ts` file/search/read endpoints

### C2. Exact Implementation Scope (No Vague Boundary Language)

| Scope item | File location | Exact implementation in 096 |
|---|---|---|
| KB-lite health summary in metrics output | `scripts/repo_metrics.py:72` | Add `.kb-data` presence and counts (`graph_db_bytes`, `lance_data_files`, `lance_txn_files`, `lance_version_files`) to JSON output |
| KB-lite prerequisite check for proposal workflow | `scripts/validate_proposals.py:162` | Add `--kb-lite-strict` check that `.kb-data/graph.db` and `.kb-data/knowledge.lance` exist for 096 wave closeout |
| KB-lite usage contract docs | `docs/kb-lite-design.md` | Define this exact stack and explicit non-goals |

Out of scope for 096:
- No re-platform to a new KB engine.
- No always-on daemon requirement introduced by this proposal.
- No new memory-gate threshold policy in foundational contract text.

## Section D: Public Contract Sync

| # | File:Line | Current | Proposed | Rationale |
|---|---|---|---|---|
| D1 | `README.md:35` | Compiler-first narrative dominates | Add first-class consumer contract entrypoint | Public topology should match actual workspace |
| D2 | `docs/home.md:1` | Landing text under-specifies consumers | Add consumer matrix entrypoint and ownership note | Prevent onboarding drift |
| D3 | `docs/doc-index.md:1` | Topic index not contract-first | Add links to topology, consumer contracts, kb-lite contract | Downstream campaigns need stable references |

## Execution Plan

| Wave | Scope | Files | Depends on | Gate |
|---|---|---|---|---|
| 0 | Contract acceptance + baseline parity capture | Proposal + evidence logs | None | `cargo check --all-targets` + `cargo test --all > proposals/evidence/096/cargo-test-all-baseline.log` |
| 1 | WASM parity closure (function surface) | `crates/mirr-wasm/src/lib.rs` | Wave 0 | `cargo check -p mirr-wasm` + `cargo test -p mirr-wasm` + `cargo test --test toolchain_tests` |
| 2 | LRA direct library compile transition (pinned) | `crates/lra-cli/src/main.rs`, `crates/lra-cli/Cargo.toml` | Wave 1 | `cargo test -p lra-cli` |
| 3 | MRT typed interface transition + KB-lite script scope | `mcp_server/src/mrt.ts`, `scripts/repo_metrics.py`, `scripts/validate_proposals.py` | Wave 2 | `npm --prefix mcp_server test` + `node mcp_server/tests/stdio_proxy_test.js` |
| 4A | Demos + scripts compatibility sweep | `demos/`, `paper/demos`, `docs/paper/demos`, `scripts/` | Wave 3 | `npm --prefix demos pack --dry-run` + `npm --prefix paper/demos pack --dry-run` + `npm --prefix docs/paper/demos pack --dry-run` + `tests/eda/run_eda_tests.sh` |
| 4B | Proofs compatibility sweep | `proofs/` | Wave 4A | `make -C proofs/rspu` |
| 4C | Fuzz harness compatibility sweep | `fuzz/` | Wave 4A | `cargo check --manifest-path fuzz/Cargo.toml` |
| 4D | Arsenal + cross-regression sweep | `crates/mirr-arsenal-wasm`, regression tests | Wave 4A | `cargo check --manifest-path crates/mirr-arsenal-wasm/Cargo.toml` + `cargo test --test self_hosting_parity_tests` + `cargo test --test mape_k_integration_core_tests` |
| 5 | Public contract sync and closeout | README/docs/topology docs | Waves 1-4 | `cargo test --all` |

## Breakage Map

| Wave | Expected temporary breakage | Resolution condition |
|---|---|---|
| 1 | WASM parity endpoints may be compile-incomplete while signatures are introduced | `cargo check -p mirr-wasm`, `cargo test -p mirr-wasm`, and `cargo test --test toolchain_tests` all pass |
| 2 | LRA compile command behavior may diverge while shell-out is removed | `cargo test -p lra-cli` passes and `lra compile` path no longer shells out |
| 3 | MRT auth/tool routing may fail while typed dispatch is introduced | Declared temporary exception allowed only for baseline capture; closure requires `npm --prefix mcp_server test` and `node mcp_server/tests/stdio_proxy_test.js` to pass with typed dispatch active |
| 4A | Demos/scripts drift can appear in packaging and EDA automation | `npm --prefix demos pack --dry-run`, `npm --prefix paper/demos pack --dry-run`, `npm --prefix docs/paper/demos pack --dry-run`, and `tests/eda/run_eda_tests.sh` all pass |
| 4B | Proof drift can appear in formal artifact build | `make -C proofs/rspu` passes |
| 4C | Fuzz harness drift can appear as compile-level breakage | `cargo check --manifest-path fuzz/Cargo.toml` passes |
| 4D | Arsenal and integration drift can appear in cross-regression tests | `cargo check --manifest-path crates/mirr-arsenal-wasm/Cargo.toml`, `cargo test --test self_hosting_parity_tests`, and `cargo test --test mape_k_integration_core_tests` all pass |
| 5 | Docs and contract references may lag final implementation state | README/docs sync completed and `cargo test --all` passes |

## Consumer Evidence Commands

| Consumer | Command |
|---|---|
| WASM | `cargo check -p mirr-wasm` |
| Arsenal WASM | `cargo check --manifest-path crates/mirr-arsenal-wasm/Cargo.toml` |
| LRA | `cargo test -p lra-cli` |
| MRT/MCP | `npm --prefix mcp_server test` + `node mcp_server/tests/stdio_proxy_test.js` |
| VS Code package behavioral boundary contract | `node -e "const p=require('./vscode-mirr/package.json'); if(!p.description||!p.contributes||p.main||p.activationEvents){process.exit(1)}"` |
| Demos | `npm --prefix demos pack --dry-run` |
| Paper demos mirror | `npm --prefix paper/demos pack --dry-run` |
| Docs/paper demos mirror | `npm --prefix docs/paper/demos pack --dry-run` |
| Proofs | `make -C proofs/rspu` |
| Fuzz harnesses | `cargo check --manifest-path fuzz/Cargo.toml` |
| Scripts/EDA | `tests/eda/run_eda_tests.sh` |

### Baseline Capture Artifact

MCP baseline evidence is recorded at:
- `proposals/evidence/096/mcp-stdio-baseline.log`

### Consumer Acceptance Bundle

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps
cargo check --all-targets
cargo test --all
cargo test -p lra-cli
cargo check -p mirr-wasm
cargo test -p mirr-wasm
cargo check --manifest-path crates/mirr-arsenal-wasm/Cargo.toml
npm --prefix mcp_server test
node mcp_server/tests/stdio_proxy_test.js
tests/eda/run_eda_tests.sh
npm --prefix demos pack --dry-run
npm --prefix paper/demos pack --dry-run
npm --prefix docs/paper/demos pack --dry-run
make -C proofs/rspu
cargo check --manifest-path fuzz/Cargo.toml
node -e "const p=require('./vscode-mirr/package.json');if(!p.description||!p.contributes||p.main||p.activationEvents){process.exit(1)}"
# Proposal-scope strict gate (required for 096 closeout)
python scripts/validate_proposals.py --strict --kb-lite-strict --files proposals/096-REPO-WIDE-FOUNDATIONAL-INTEGRATION-2026-04-02.md
python scripts/repo_metrics.py --json
```

Validator realism note:
- 096 closeout requires all of the following: zero validator issues in proposal-scoped strict mode and successful KB-lite strict prerequisite checks.
- CI policy encoding for 096: the blocking acceptance bundle above must be wired as required CI status checks for merge.

## File Manifest

### Edited Files

| File | Change summary |
|---|---|
| `proposals/096-REPO-WIDE-FOUNDATIONAL-INTEGRATION-2026-04-02.md` | Foundational repo integration proposal with wave-locked consumer contracts, parity matrix, and acceptance gates |
| `crates/mirr-wasm/src/lib.rs` | Add parity-closing exported endpoints and preserve existing endpoint behavior |
| `crates/lra-cli/src/main.rs` | Replace compile shell-out path with direct compiler library entrypoint |
| `crates/lra-cli/Cargo.toml` | Ensure compiler library dependency surface is explicit for direct compile path |
| `mcp_server/src/mrt.ts` | Replace wrapper cargo invocations with typed tool dispatch function |
| `scripts/repo_metrics.py` | Add KB-lite presence and count metrics to JSON summary |
| `scripts/validate_proposals.py` | Add `--kb-lite-strict` and `--files` scoped strict checks for 096 closeout |
| `vscode-mirr/package.json` | Make extension capability contract explicit and truthful |
| `README.md` | Add first-class consumer contract references |
| `docs/home.md` | Add consumer matrix entrypoint references |
| `docs/doc-index.md` | Add links to topology and consumer contract docs |
| `docs/repo-topology.md` | Canonical topology authority content consumed by downstream docs |
| `docs/consumer-contracts.md` | First-class consumer ownership and compatibility contract matrix |
| `docs/kb-lite-design.md` | KB-lite scope, evidence commands, and explicit non-goals |

### New Files

| File | Description |
|---|---|
| None | Referenced contract docs already exist and are now indexed for proposal traceability |

## Foundational Claim

Proposal 095 established foundational compiler-internal contracts. Proposal 096 establishes foundational repository-system contracts with explicit ownership and executable evidence gates for each first-class consumer surface.

The durable outcome is not "cleaner docs"; it is a stable repository architecture surface:
- explicit topology authority,
- explicit first-class consumer contracts,
- explicit wave-locked adapter transitions,
- explicit KB-lite implementation scope grounded in current repository data.

Future campaigns can now reference these contracts directly instead of re-discovering the same repo-system invariants each cycle.

