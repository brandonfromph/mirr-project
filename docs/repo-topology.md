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

## First-Class Consumers
- crates/mirr-wasm: WASM compiler bindings
- crates/lra-cli: LRA command surface
- mcp_server: MRT and MCP bridge
- vscode-mirr: VS Code package surface
- demos: demo packages
- proofs: formal proof projects
- fuzz: fuzz harnesses
- scripts: governance and automation scripts

## Contract Rule
Any architecture-scope proposal must declare impact on each first-class consumer or mark it out-of-scope with rationale.
