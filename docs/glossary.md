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
| **DPR** | Dynamic Partial Reconfiguration — loading partial FPGA bitstreams at runtime |
| **DSL** | Domain-Specific Language |
| **EDA** | Electronic Design Automation |
| **EQY** | Yosys Equivalence Checker tool |
| **FIRRTL** | Flexible Intermediate Representation for RTL — Chisel's IR, used as a MIRR emission target |
| **FIRWINE** | Formally verified width inference procedure (Wang et al. 2026) |
| **FPGA** | Field-Programmable Gate Array |
| **Guard** | MIRR temporal condition monitoring signals over time; triggers when condition holds for N cycles |
| **HLS** | High-Level Synthesis |
| **ISA** | Instruction Set Architecture |
| **LPF** | Lattice Preference File — ECP5 constraint format |
| **LTL** | Linear Temporal Logic — formal language for specifying temporal properties |
| **MAPE-K** | Monitor–Analyze–Plan–Execute–Knowledge — autonomic computing feedback loop |
| **MIRR** | The hardware rule language this project implements |
| **MinGW** | Minimalist GNU for Windows — used by oss-cad-suite on Windows |
| **NBTI** | Negative Bias Temperature Instability — silion aging mechanism |
| **NASA P10** | NASA Power-of-10 coding rules for safety-critical software |
| **nextpnr** | Open-source FPGA place-and-route tool (supports iCE40, ECP5, Nexus) |
| **oss-cad-suite** | YosysHQ open-source EDA toolchain distribution |
| **PCF** | Physical Constraints File — iCE40 pin assignment format |
| **PDC** | Physical Design Constraints — Lattice Nexus constraint format |
| **R-SPU** | Reflex Signal Processing Unit — the custom processor MIRR targets |
| **Reflex** | MIRR action that fires when a guard triggers; the only way to drive outputs |
| **Rocq** | Interactive theorem prover (formerly Coq); used for FIRWINE proofs |
| **RTL** | Register Transfer Level — hardware description abstraction |
| **SCC** | Strongly Connected Component — cycle in the width dependency graph |
| **SDC** | Synopsys Design Constraints — timing constraint format |
| **Signal** | MIRR named data path carrying a typed value every clock cycle |
| **SmaRTLy** | Inference-driven RTL logic optimization (Li et al. 2025) |
| **SVA** | SystemVerilog Assertions — property specification language |
| **sby** | SymbiYosys — formal verification front-end for Yosys |
| **XDC** | Xilinx Design Constraints — Vivado constraint format |
| **Yosys** | Open-source synthesis tool |
| **zext** | Zero-extension — explicit widening of an unsigned value |

---

## See Also

- [Roadmap](roadmap) — Full project context
- [Contributing](contributing) — Coding standards and workflow
