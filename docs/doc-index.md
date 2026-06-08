---
title: Documentation Index
nav_order: 1
---

# MIRR Documentation Index

Canonical index for all project documentation. Updated each campaign.

---

## Status Legend

| Status | Meaning |
|--------|---------|
| **Active** | Current, maintained documentation |
| **Frozen** | Historical, will not be updated |
| **Deprecated** | Superseded by newer docs; kept for reference |
| **Stub** | Placeholder awaiting content |

---

## Core Documentation

| Document | Status | Description |
|----------|--------|-------------|
| [Architecture](ARCHITECTURE) | Active | Canonical source of truth for the codebase tree |
| [Roadmap](roadmap) | Active | Project phases, architecture, research foundation |
| [Presidential Arsenal Roadmap](presidential-arsenal-roadmap) | Active | Mega-campaign execution plans and strategies |
| [Tutorial](tutorial) | Active | Learn MIRR from scratch |
| [Type System](type-system) | Active | Type checker reference (E601–E625) |
| [Error Codes](error_codes) | Active | Error codes introduced per phase |
| [Logic Simplification](logic_simplification) | Active | Phase 3 simplifier architecture and rules |
| [Benchmarks](benchmarks) | Active | Criterion benchmark tiers and usage |
| [Glossary](glossary) | Active | Project terminology and acronyms (~55 terms) |
| [Contributing](contributing) | Active | Coding standards, campaign workflow, error allocation |
| [Testing Guide](testing-guide) | Active | Testing architecture and test suite organization |
| [Web Rules](web-rules) | Active | Web development and aesthetic UI guidelines |
| [File Tree](file-tree) | Frozen | Historical snapshot of the Phase 7 repository tree |
| [Consumer Contracts](consumer-contracts) | Active | Interface boundaries and API guarantees for end users |
| [FPGA Targets Guide](fpga-targets-guide) | Active | FPGA toolchain, synthesis, and target configuration |
| [MAPE-K Guide](mape-k-guide) | Active | Autonomic feedback loop simulator and LTL monitoring |
| [S-Expression Guide](sexpr-guide) | Active | Homoiconic S-expression IR, round-trip invariant |
| [ECS Migration Guide](ECS_MIGRATION_GUIDE) | Active | Guide for migrating the compiler to the ECS architecture |

## Architecture References

| Document | Status | Description |
|----------|--------|-------------|
| [R-SPU ISA v2 Specification](rspu_isa_spec) | Active | R-SPU instruction set architecture v2 specification |
| [Migration Guide](migration-guide) | Active | Breaking changes per version |
| [Phase 2 Temporal Guard Compiler](phase2_temporal_guard_compiler) | Frozen | Original Phase 2 design notes |

## Self-Hosting

| Document | Status | Description |
|----------|--------|-------------|
| [Self-Hosting Milestone](self_hosting_milestone) | Frozen | MIRR-in-MIRR progress tracker |
| [Self-Hosting Core Spec](self_hosting_core_spec) | Frozen | Core language spec for self-hosting |
| [Self-Hosting IR Contract](self_hosting_ir_contract) | Frozen | IR contract for bootstrap pipeline |

## Legacy

| Document | Status | Description |
|----------|--------|-------------|
| [MIRR Spec](mirr_spec) | Deprecated | Phase 1 minimal core only. See Tutorial and Type System. |

## Papers

| Document | Status | Description |
|----------|--------|-------------|
| `paper/dac2027-mirr.tex` | Frozen | DAC 2027 submission (tag: `dac2027-submission`) |
| `paper/living-doc/` | Active | Living documentation (no page limit, updated every campaign) |

---

## Proposal Archive

All proposals live in `proposals/` and follow the campaign workflow.

| # | Campaign | Date | Status |
|---|----------|------|--------|
| 001 | SEM-001 | 2026-03-08 | Executed |
| 002 | TYPE-001 | 2026-03-08 | Executed |
| 003 | TYPE-002 | 2026-03-08 | Executed |
| 004 | TYPE-003 | 2026-03-08 | Executed |
| 005 | TYPE-004 | 2026-03-08 | Executed |
| 006 | ROCQ-001 | 2026-03-08 | Executed |
| 007 | TYPE-005 + RSPU-001 | 2026-03-08 | Executed |
| 008 | DOC-001 | 2026-03-08 | Executed |
| 009 | SITE-001 | 2026-03-09 | Executed |
| 010 | SITE-002 | 2026-03-09 | Executed |
| 011 | SPAN-001 + LSP-001 | 2026-03-09 | Executed |
| 012 | ERR-001 + VSCODE-001 | 2026-03-09 | Executed |
| 013 | FPGA-001 | 2026-03-09 | Executed |
| 014 | FPGA-002 | 2026-03-09 | Executed |
| 015 | ERR-002 | 2026-03-09 | Executed |
| 016 | SAFE-001 | 2026-03-09 | Executed |
| 017 | DEBT-001 | 2026-03-09 | Executed |
| 018 | DEBT-002 | 2026-03-09 | Executed |
| 019 | DOC-001 | 2026-03-09 | Executed |
| 020 | DOC-002 | 2026-03-09 | Executed |
| 021 | SYNTH-001 | 2026-03-09 | Executed |
| 022 | PAPER-001 | 2026-03-09 | Executed |
| 023 | PHASE7-FOUNDATION | 2026-03-10 | Executed |
| 024 | MEGA-1a | 2026-03-10 | Executed |
| 025 | MEGA-1b | 2026-03-11 | Executed |
| 026 | MEGA-2 | 2026-03-11 | Executed |
| 027 | MEGA-3 | 2026-03-11 | Executed |
| 028 | AUDIT-001 | 2026-03-11 | Executed |
| 029 | STD-001 | 2026-03-12 | Executed |
| 030 | LRA-001 | 2026-03-11 | Executed |
| 031 | LRA-002 | 2026-03-11 | Executed |
| 032 | STD-001-LRA | 2026-03-12 | Executed |
| 033 | LRA-PHASE1 | 2026-03-12 | Executed |
| 034 | LRA-PHASE2 | 2026-03-12 | Executed |
| 035 | LRA-PHASE3-PHASE4 | 2026-03-12 | Executed |
| 036 | CIGREEN-001 | 2026-03-12 | Executed |
| 037 | HARDEN-001 | 2026-03-12 | Executed |
| 038 | PAPER-001 | 2026-03-12 | Executed |
| 039 | CIRUST-001 | 2026-03-12 | Executed |
| 040 | PAGES-001 | 2026-03-12 | Executed |
| 041 | ULTRA-FORMAL-001 | 2026-03-12 | Executed |
| 042 | PAPER-001 | 2026-03-13 | Executed |
| 043 | TITAN-CONVERGENCE | 2026-03-13 | Executed |
| 044 | OUROBOROS-DESIGN | 2026-03-13 | Executed |
| 045 | MEGA-CONVERGENCE | 2026-03-14 | Executed |
| 046 | POLISH-AND-SHIP | 2026-03-14 | Executed |
| 047 | POLISH-2 | 2026-03-14 | Executed |
| 048 | LRA-PHASE5 | 2026-03-15 | Executed |
| 049 | LRA-PHASE6 | 2026-03-15 | Executed |
| 050 | LRA-PHASE7 | 2026-03-15 | Executed |
| 051 | LRA-PHASE8 | 2026-03-15 | Executed |
| 052 | LRA-PHASE9 | 2026-03-15 | Executed |
| 053 | MEGA4-TOTALITY-ENGINE | 2026-03-15 | Executed |
| 054 | MEGA5-SYMBOLIC-EVAL | 2026-03-15 | Executed |
| 055 | MEGA-VERIFY | 2026-03-15 | Executed |
| 056 | MEGA6-MAPE-K-SILICON | 2026-03-16 | Executed |
| 057 | MEGA7-RUST-LEVEL-ERRORS | 2026-03-17 | Proposed |
| 058 | MEGA10-BOUNDED-DATA | 2026-03-17 | Proposed |
| 059 | MEGA11-14-UNIFIED | 2026-03-17 | Proposed |
| 060 | MEGA12-BOUNDED-HLS | 2026-03-19 | Proposed |
| 061 | MEGA12-DATAFLOW-OPT | 2026-03-19 | Proposed |
| 062 | MEGA13-SELF-HOST | 2026-03-19 | Proposed |
| 063 | WASM-BUILDER-AUDIT | 2026-03-20 | Executed |
| 064 | MAJOR-REPO-HARDENING | 2026-03-20 | Proposed |

---

## See Also

- [Home](home) — Project landing page
- [Roadmap](roadmap) — Full project roadmap
