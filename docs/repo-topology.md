---
title: Repository Topology
status: Active
---

# Repository Topology

Canonical topology for first-class consumers in this workspace.

## Core
- src: MIRR compiler core
- tests: compiler and integration suites
- compiler_mirr: self-hosting sources

## Control Plane
- MRT / Presidential Arsenal: mirr-audit, mirr-brain, mirr-wave, mirr-general, mirr-lsp, KB-lite governance plane
MRT is the official name of the full control-plane toolchain; mcp_server is the TypeScript interface bridge into MRT.

## First-Class Consumers
- crates/mirr-wasm: WASM compiler bindings
- crates/mirr-arsenal-wasm: Arsenal validation bridge
- crates/lra-cli: Arsenal-facing CLI surface
- mcp_server: TypeScript interface bridge into MRT for MCP access, not a core compiler or toolchain logic owner
- vscode-mirr: VS Code package surface
- demos: demo packages
- proofs: formal proof projects
- fuzz: fuzz harnesses
- scripts: governance and automation scripts

## Contract Rule
Any architecture-scope proposal must declare impact on each first-class consumer or mark it out-of-scope with rationale.
