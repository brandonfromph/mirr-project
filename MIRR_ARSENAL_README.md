# 🏛️ MIRR Arsenal: The Presidential Command Suite

The **MIRR Arsenal** is a sovereign collection of high-agency CLI tools designed to enforce Correctness-Driven Development (CDD) and manage the MIRR compiler's evolution with mathematical rigor.

## 🪖 The Army of CLIs

The Arsenal is composed of specialized, deterministic agents ("The Army") that work in concert to maintain the **Presidential Loop**.

### 1. `mirr-audit` (The Eyes)
Performs high-speed scans of the codebase to detect:
*   **Zero-Debt Violations**: Identifies deprecated aliases, dead code, and backward-compatibility shims.
*   **Refinement Gaps**: Detects the "Semantic Gap" between proposed changes in Markdown and their implementation in Rust.
*   **Red Lines**: Flags unauthorized IO or process usage that violates safety-critical constraints.

### 2. `mirr-brain` (The Knowledge Core)
The project's "Long-Term Memory." It stores and retrieves:
*   **Architectural Invariants**: Global laws such as reserved error code ranges (E1xx-E8xx).
*   **System Limits**: Physical constraints like R-SPU register counts and max path depths.
*   **Signed Proofs**: Hashes of approved proposals to ensure execution integrity.

### 3. `mirr-general` (The Orchestrator)
The central command node that manages the lifecycle of a change:
*   **Audit**: Consults the Brain and runs the Auditor to verify workspace health.
*   **Wave Execution**: Coordinates `mirr-wave` to apply atomic, verified changes.
*   **CI Enforcement**: Runs the full NASA-grade gate sequence (`cargo fmt`, `cargo check`, `cargo clippy`, `cargo test`), with `nextest` profiles when configured.

### 4. `mirr-wave` (The Executive)
The mechanical arm of the Arsenal. It parses execution plans from signed proposals and applies surgical text replacements. It is designed to be **Sub-Turing**: it makes zero choices and aborts if any ambiguity is detected.

## 🔄 The Presidential Loop

The Arsenal enforces a strict, linear workflow for every modification:

1.  **Audit**: Scan the repo for debt and refinement gaps.
2.  **Propose**: Create a formal `.md` proposal with a detailed breakage map.
3.  **Sign**: The "President" (human or lead agent) signs the proposal, hashing it into the Brain.
4.  **Connect MCP**: Confirm `.vscode/mcp.json` maps `mirr-local` to `node mcp_server/start.js --stdio-direct --workspace-root .`, then run `npm.cmd --prefix mcp_server run mcp:health` to verify the stdio bridge health probe.
5.  **Execute**: `mirr-general` executes the wave atomically.
6.  **Verify**: A full CI gate validates the new state against the workspace safety test suite.

## 🛠️ Usage

Ensure you have the Arsenal loaded in your environment:

```powershell
# Load the PowerShell Module
. scripts/MirrArsenal.ps1

# Verify MCP bridge build + health probe
npm.cmd --prefix mcp_server run build
npm.cmd --prefix mcp_server run mcp:health

# Run a full workspace audit
Invoke-MirrAudit

# Propose a new campaign
New-MirrProposal -ID "PHASE-0" -Title "Trinity Foundation"

# Execute a signed wave
Invoke-MirrWave -ID "PHASE-0"
```

---
*MIRR Arsenal: Safety-Critical Governance for Autonomous Intelligence.*
