---
title: Consumer Contracts
status: Active
---

# Consumer Contracts

This document defines repo-wide consumer contracts used by architecture proposals.

| Consumer | Contract |
|---|---|
| crates/mirr-wasm | Public exported API remains backward-compatible unless explicitly versioned |
| crates/mirr-arsenal-wasm | Compile-contract output remains deterministic and schema-stable |
| crates/lra-cli | Compile path must call compiler library entrypoints, not shell-out wrappers |
| mcp_server | Tool routing is explicit, typed, allowlisted, and keeps MRT tool names stable; interface/bridge layer only and must not own core compiler logic |
| vscode-mirr | Package contract text must match actual capability |
| demos/proofs/fuzz/scripts | Must have explicit compatibility evidence in architecture waves |

## Ownership
- Compiler boundary: elvie (primary), compiler maintainers (backup), compiler reviewers (escalation)
- Consumer boundary: elvie (primary), consumer maintainers (backup), architecture reviewers (escalation)
- Proposal conformance: proposal reviewers (primary), repository governance maintainers (backup), campaign owner (escalation)
