# MRT Project Mandates

*A safety-critical HDL compiler where the code literally cannot be wrong or people die.*

**Quality** — Read actual file content, never trust status claims. Verify artifacts.


### MRT CLI tools (Available via repo scripts)
*   `mrt_audit` : Remote repo-wide audit tool.
*   `mrt_brain_get` : Remote knowledge retrieval.
*   `mrt_general_ci` : Remote CI execution.
*   `mrt_semantic_hover` : Query symbols for types, kinds, and docs.
*   `cargo run --bin mirr-audit ...` : Local audit engine.
*   `cargo run --bin mirr-brain ...` : Brain persistence store.
*   `cargo run --bin mirr-general ...` : Orchestrator for wave/CI operations.

---

## 🛠️ Arsenal Command Suite (Full Manifest)

### 1. High-Level Missions (Unified Layer)
*   `lra author <init|build|serve|build-docs>` : Unified content creation.
*   `lra verify <validate|hash|verify|verify-receipt>` : Cryptographic integrity.
*   `lra network <search|deps|health|status|crawl>` : Artifact ecosystem discovery.
*   `lra arsenal <compile|receipt|sign|keygen>` : Deployment and certification.

### 2. Core MRT (The Muscle)
*   `cargo run --bin mirr-audit -- --mode <workspace|refinement|proposal>` : Debt Auditor.
*   `cargo run --bin mirr-wave -- -i <ID> -f <PROPOSAL_FILE> --stash` : Atomic Executor.
*   `cargo run --bin mirr-brain -- <get|store|laws>` : Knowledge Core.
*   `cargo run --bin mirr-general -- <audit|wave|ci>` : Arsenal Orchestrator.
*   `cargo run --bin mirr-lsp` : Semantic Intelligence Engine.

### 3. MCP Semantic Bridge (Gemini CLI Integration)
*   `mrt_audit` : Remote repo-wide audit tool.
*   `mrt_brain_get` : Remote knowledge retrieval.
*   `mrt_general_ci` : Remote CI execution.
*   `mrt_semantic_hover` : **(NEW)** Query symbols for types, kinds, and docs.

### 4. High-Resolution Commands (Specialized Tooling)
*Every command here is preserved for granular control and rapid experimentation.*
*   `lra init` : Scaffolder.
*   `lra validate` : Schema check.
*   `lra build` : Document builder.
*   `lra build-docs` : Site generator.
*   `lra serve` : Live dev server.
*   `lra hash` : SHA-256 tool.
*   `lra verify` : Signature tool.
*   `lra keygen` : Identity generator.
*   `lra search` : Network search.
*   `lra deps` : Graph printer.
*   `lra crawl` : Registry crawler.
*   `lra status` : Health check.
*   `lra health` : Node connectivity.
*   `lra receipt` : Receipt tool.
*   `lra compile` : MIRR bridge.
*   `lra sign` : Signature tool.
*   `lra verify-receipt` : Verification tool.
*   `lra badge` : Compliance visualizer.

---

## Technical Constraints

*   **ZERO DELETIONS**: Every line of original logic is preserved. New missions wrap existing code.
*   **ZERO-DEBT INVARIANT**: No dead code or orphaned stubs.
*   **ZERO-STUB INVARIANT**: No `// TODO`, `FIXME`, or `unimplemented!` markers. Every line must be wired.
*   **NO PYTHON SCRIPTS EVER** for text manipulation.
*   **KB STANDARD**: Telemetry MUST be stashed in the **Brain** (`mirr-brain`).

## Resource Map

| Resource | Path |
|---|---|
| MRT Command Center | `MIRR_ARSENAL_README.md` |
| Campaign Spec | `.claude/commands/propose-campaign.md` |
| Error Codes | `docs/error_codes.md` |
| Living Doc | `paper/living-doc/main.tex` |
| Proposals | `proposals/` |
| Archdrive | `proposals/archdrive/` |
on.
*   **KB STANDARD**: Telemetry MUST be stashed in the **Brain** (`mirr-brain`).
