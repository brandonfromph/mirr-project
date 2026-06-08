---
title: Glossary
nav_order: 11
---

# Glossary

Terminology and acronyms used throughout the MIRR project.

---

| Term | Definition |
|------|-----------|
| **AHL** | Adaptive Hold Logic — circuit-level mitigation that pauses the clock on timing violations |
| **BMC** | Bounded Model Checking — formal verification technique checking properties up to a bounded depth |
| **Cement2** | Temporal hardware transaction model replacing absolute timing with shift-register delays |
| **Claims** | Verifiable statements in an LRA paper backed by evidence |
| **DAC** | Design Automation Conference — target venue for the MIRR living research paper |
| **DPR** | Dynamic Partial Reconfiguration — loading partial FPGA bitstreams at runtime |
| **DSL** | Domain-Specific Language |
| **Dual-Mode Execution** | R-SPU execution mode switching between normal and exception handling |
| **EDA** | Electronic Design Automation |
| **ECS** | Entity Component System — data-oriented architecture introduced in Phase 3 for cache-friendly flat array iteration |
| **EntityId** | A 32-bit handle representing an AST node or hardware primitive in the ECS Registry |
| **EQY** | Yosys Equivalence Checker tool |
| **Exception Code** | Numeric identifier for R-SPU trap conditions (E7xx range) |
| **FIRRTL** | Flexible Intermediate Representation for RTL — Chisel's IR, used as a MIRR emission target |
| **FIRWINE** | Formally verified width inference procedure (Wang et al. 2026) |
| **FPGA** | Field-Programmable Gate Array |
| **Guard** | MIRR temporal condition monitoring signals over time; triggers when condition holds for N cycles |
| **HLS** | High-Level Synthesis |
| **Homoiconic** | Property where code and data share the same representation (S-expressions) |
| **ISA** | Instruction Set Architecture |
| **JSON-RPC** | Remote procedure call protocol used by the LRA Service Worker |
| **Living Research Artifact (LRA)** | A paper that bundles code, proofs, and GUI into a single Apache-licensed repository |
| **LPF** | Lattice Preference File — ECP5 constraint format |
| **LRA-1.0** | The specification standard for Living Research Artifacts |
| **LTL** | Linear Temporal Logic — formal language for specifying temporal properties |
| **Macro Hygiene** | Ensuring macro-expanded identifiers do not capture surrounding bindings |
| **MAPE-K** | Monitor–Analyze–Plan–Execute–Knowledge — autonomic computing feedback loop |
| **MIRR** | The hardware rule language this project implements |
| **MinGW** | Minimalist GNU for Windows — used by oss-cad-suite on Windows |
| **NBTI** | Negative Bias Temperature Instability — silicon aging mechanism |
| **NASA P10** | NASA Power-of-10 coding rules for safety-critical software |
| **nextpnr** | Open-source FPGA place-and-route tool (supports iCE40, ECP5, Nexus) |
| **oss-cad-suite** | YosysHQ open-source EDA toolchain distribution |
| **PCF** | Physical Constraints File — iCE40 pin assignment format |
| **PDC** | Physical Design Constraints — Lattice Nexus constraint format |
| **Provenance** | Type-level tracking of a signal's origin through tagged words |
| **Quasiquote** | S-expression template with unquote splicing for code generation |
| **R-SPU** | Reflex Signal Processing Unit — the custom processor MIRR targets |
| **Reader Macro** | User-defined syntax extension in the S-expression parser |
| **Registry** | The central data store in the ECS containing all components as contiguous arrays |
| **Reflex** | MIRR action that fires when a guard triggers; the only way to drive outputs |
| **Rocq** | Interactive theorem prover (formerly Coq); used for FIRWINE proofs |
| **RTL** | Register Transfer Level — hardware description abstraction |
| **S-expression** | Symbolic expression — the homoiconic IR used by MIRR |
| **SCC** | Strongly Connected Component — cycle in the width dependency graph |
| **SDC** | Synopsys Design Constraints — timing constraint format |
| **Service Worker** | Browser-side proxy that enables offline operation and the LRA protocol |
| **Signal** | MIRR named data path carrying a typed value every clock cycle |
| **SmaRTLy** | Inference-driven RTL logic optimization (Li et al. 2025) |
| **SVA** | SystemVerilog Assertions — property specification language |
| **Tagged Architecture** | R-SPU ISA v2 feature where values carry type metadata at runtime |
| **Tagged Word** | R-SPU machine word carrying type and provenance metadata |
| **sby** | SymbiYosys — formal verification front-end for Yosys |
| **WASM** | WebAssembly — portable binary format; used for running MIRR compiler in the browser |
| **XDC** | Xilinx Design Constraints — Vivado constraint format |
| **Yosys** | Open-source synthesis tool |
| **zext** | Zero-extension — explicit widening of an unsigned value |

---

## See Also

- [Roadmap](roadmap) — Full project context
- [Contributing](contributing) — Coding standards and workflow
