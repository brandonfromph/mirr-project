---
title: Consumer Contracts
status: Active
---

# Consumer Contracts

This document defines repo-wide consumer contracts used by architecture proposals.

| Consumer | Contract |
|---|---|
| crates/mirr-wasm | Public exported API remains backward-compatible unless explicitly versioned |
| crates/lra-cli | Compile path must call compiler library entrypoints, not shell-out wrappers |
| mcp_server | Tool routing is explicit, typed, and allowlisted |
| vscode-mirr | Package contract text must match actual capability |
| demos/proofs/fuzz/scripts | Must have explicit compatibility evidence in architecture waves |

## Ownership
- Compiler boundary: compiler maintainers
- Consumer boundary: consumer maintainers
- Proposal conformance: proposal reviewers
