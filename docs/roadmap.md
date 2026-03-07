# R‑SPU Compiler & EDA Roadmap

This roadmap breaks the Reflexive Processing Unit (R‑SPU) concept into **small, Rust‑based subprojects**. Each step is intended to be realistically completable and to produce visible results.

---

## The Problem This Solves: The Autonomic Observability Gap

Modern processors operate on a dangerous assumption: that a delay in calculation is merely a performance issue. In safety-critical domains — medical robotics, aerospace controls, autonomous vehicles — a stalled thread means a failed respirator or a severed artery.

Standard heterogeneous architectures are fundamentally unsafe for such automation because of the **Autonomic Observability Gap**: the inability of a standard silicon device to perceive its own operational context — thermal gradients, voltage droops, sensor degradation, or changing workload characteristics.

A concrete example: Negative Bias Temperature Instability (NBTI) causes physical aging in silicon transistors. As demonstrated by Lin et al. (2016), hardware cannot detect its own timing degradation until a system-level failure occurs. Circuit-level mitigations like Adaptive Hold Logic (AHL) can pause the clock, but they lack the high-level semantic awareness required for clinical safety — they protect the clock, not the application.

In traditional architectures, adaptation is relegated to the software layer. The latency inherent in this software-mediated control loop — from hardware capture to software analysis and back to hardware reconfiguration — is often in the millisecond range. For safety-critical applications, microseconds determine safety outcomes.

The R‑SPU resolves this by bifurcating the computing substrate into:
- **Cognitive Host Domain** — non-deterministic, high-level planning (Linux/software layer)
- **Deterministic Reflex Domain** — cycle-accurate, formally verified hardware layer

MIRR is the orchestration language that bridges both. It compiles high-level behavioral intent directly into Linear Temporal Logic (LTL) safety guards and SystemVerilog RTL, enforcing nanosecond-level Hardware-Software Co-Design precision.

The three backend engines MIRR orchestrates:
- **Cement2** — Temporal Hardware Transactions (replaces brittle global counters)
- **SmaRTLy** — Inference-driven Logic Optimization (semantic redundancy elimination)
- **FIRWINE** — Formally Verified Width Inference (prevents data corruption at compile time)

---

## What Makes MIRR Unique: Three Roles in One Language

MIRR is not merely a language or a compiler. It serves three distinct but interconnected roles simultaneously — which is what makes it fundamentally different from every existing tool:

**1. Design Language**
The engineer writes MIRR code to describe desired hardware behavior. Time is a first-class citizen — delays and temporal relationships are part of the language itself, not library calls.

**2. Compiler Toolchain**
The MIRR compiler translates design intent into Verilog RTL, automatically leveraging:
- Cement2 for synthesizing temporal constructs into efficient hardware
- SmaRTLy for optimizing logic and eliminating redundancy
- FIRWINE for formally verifying bit-width safety

**3. Runtime Instruction Language**
Once the R-SPU chip is fabricated, the same MIRR language is used to write software that runs on it. The chip understands MIRR natively, creating a seamless continuum from design-time specification to runtime operation.

This unified model is what makes MIRR fundamentally different from:
- C++ — software-only, no native hardware timing or physical bit-width concepts
- Verilog — hardware-only, no high-level behavioral abstractions
- Chisel — design-only, no runtime instruction semantics
- MIRR bridges all three, enabling true Hardware-Software Co-Design

---

## Why MIRR v2.2 Failed: The Lessons Built Into This Design

Understanding MIRR v2.2's architectural failures explains every design decision in this project.

**Failure 1 — `at(t)`: The Absolute Timing Trap**
MIRR v2.2 used absolute time: `at(system_time + 50) { fire_signal(); }`. The compiler synthesized one dedicated 64-bit comparator per temporal event, connected to a central chip-wide 64-bit counter bus. In a design with 10,000 temporal events: 10,000 comparators, massive routing bottleneck, timing closure impossible above 1 GHz. If a memory stall caused the counter to advance past the target time, the event was silently missed.

→ **This project's response:** Cement2's `p.delay(k)` uses shift registers of length k instead. Gate count drops from O(N × 64) to O(N × k). No global bus, no routing congestion, no missed events.

**Failure 2 — `mutate`: The MUX Forest Problem**
The `mutate` primitive synthesized runtime datapath switching into multiplexers naively. Nested mutate blocks generated MUX trees without checking for logical redundancy — it never analyzed whether cases shared common sub-expressions or whether control signals were logically correlated. Deep logic cones severely limited maximum operating frequency.

→ **This project's response:** SmaRTLy performs Semantic Logic Inference — MiniSAT solver determines if a MUX actually affects the output. If not, it is removed. 47% area reduction on industrial benchmarks.

**Failure 3 — `reflect`: Passive Telemetry Latency**
The `reflect` primitive used a Shadow Register Chain (scan chain) to capture register state non-intrusively. However, data had to be streamed out via JTAG/UART to host software for analysis. Round-trip latency: milliseconds. Too slow to suppress transient faults or adapt to rapid signal degradation.

→ **This project's response:** MAPE-K with hardware-accelerated LTL Checker evaluates safety invariants within the current clock cycle. Sub-millisecond response, no software round-trip required.

---

## Phase 0 – Foundation (Completed)

- **Goal:** Establish a robust, safety-critical Rust toolchain with strict NASA/JPL coding standards.
- **Tasks:**
  - Enforce `#![forbid(unsafe_code)]` and `#![deny(warnings)]` across all crates.
  - Ensure all code passes `cargo fmt` and `cargo clippy` with zero warnings.
  - Implement comprehensive unit tests for all core modules.
  - Integrate performance optimizations: arena allocation, lookup tables, SIMD, and memory pooling.

**Result:** A clean, strictly checked Rust environment for safety-critical EDA tooling, with production-grade performance and reliability.

---

## Phase 1 – Mini MIRR: Reflex-Oriented DSL (Completed)

- **Goal:** Define and parse a minimal, text-based DSL for reflexive hardware behaviors, inspired by MIRR.
- **Scope:**
  - Support declarations for inputs, outputs, and internal signals.
  - Parse combinational expressions (`&`, `|`, `^`, `!`).
  - Parse temporal guards (`when signal for N cycles`).
  - Implement a hand-written lexer and parser in Rust.
  - Build a strongly-typed AST and IR.
  - Provide clear, actionable error messages for syntax and type errors.

- **Philosophical foundation carried forward:**
  MIRR enforces a **Strict Static Typing System** inherited from MIRR v2.2's core insight: in hardware, types represent physical wires, not memory abstractions. Implicit casting is forbidden. A `u8` signal cannot be assigned to a `u16` register without an explicit zero-extension (`zext`) operation. This forces designers to be explicitly aware of the physical dimensions of every data path — and lays the philosophical groundwork for FIRWINE's formal width inference in Phase 4.

**Result artifact:** CLI tool `mirr-parse` that reads `.mirr` files and prints structured JSON or pretty IR. Fully self-contained and shippable.

---

## Phase 2 – Temporal Guard Compiler (Cement2-Inspired, Completed)

- **Goal:** Compile temporal guards from the MIRR DSL into a low-level, hardware-mappable IR using shift registers and counters — replacing MIRR v2.2's absolute timing model with Cement2's distributed transactional model.

**Why Cement2 matters:** Reduces gate count from O(N × 64) to O(N × k), eliminates global counter bus routing bottleneck, handles non-deterministic latency without missed events. Benchmark: 377 MHz timing closure on RISC-V soft-core.

- **Scope (Completed):**
  - `p.delay(k)` — Latency-Sensitive Guard: shift register of length k, one valid bit propagates per clock cycle.
  - Shift-register chains for short delays (≤16 cycles).
  - Counter-comparator structures for long delays (adaptive synthesis).
  - Emit a gate-level netlist and structured JSON conforming to project schemas.
  - Robust error handling for unsupported or unsafe guard forms.

- **Scope (Future — Phase 2b):**
  - `p.dyndelay(k)` — Latency-Insensitive Guard: FSM with sticky valid bit for non-deterministic components (DRAM, off-chip sensors). Guard becomes valid at cycle T+k and remains valid until the target rule fires.
  - ASAP Scheduling: dependency graph of all rules, infer earliest possible fire time per action, maximizing throughput.
  - Temporal Partitioning: auto-detect multi-cycle actions, slice into atomic intra-cycle stages. Relieves designer from manually inserting pipeline registers.
  - Automatic Retiming: bottom-up timing inference using SDC formulation (Cong & Zhang 2006), auto-insert pipeline registers to meet target frequency.
  - Hybrid Temporal Synthesis: for macroscopic delays (milliseconds+), auto-switch from shift registers to Synchronous Counter-Comparator blocks via hardware cost function. Short delays retain shift-register topology for jitter-free reflex domain performance. Target: O(N) area reduction for long-interval timers.
  - **Target benchmark:** 377 MHz timing closure on RISC-V soft-core (matching Cement2 paper).

**Result artifact:** CLI tool `mirr-temporal` that shows how each high-level guard maps to concrete hardware primitives, with JSON/DOT output for downstream tools.

---

## Phase 3 – Logic Simplifier (SmaRTLy-Inspired, Completed)

- **Goal:** Build a robust, resource-bounded logic simplification engine for the combinational logic in MIRR IR/netlists — addressing the MUX Forest problem from MIRR v2.2's `mutate` primitive.

**Why SmaRTLy matters:** Traditional tools like Yosys detect only structural equivalence. SmaRTLy detects semantic equivalence — if S_2 = S_1 ∨ X and S_1 is High, S_2 is necessarily High, making the second MUX redundant. Yosys misses this entirely. Benchmarks: 8.95% AIG reduction on RISC-V, 47.2% on industrial designs.

- **Current Status (Completed):**
  - All boolean, arithmetic, and comparison simplification rules implemented and tested (33 algebraic rules).
  - Boolean identity/annihilation, idempotence, absorption.
  - Arithmetic identity/annihilation and constant folding with wrapping semantics.
  - Comparison constant folding.
  - Iterative post-order traversal engine (bounded, NASA P10 compliant, no recursion).
  - Fixpoint iteration (bounded by MAX_PASSES) catches cascading reductions.
  - SimplifyStats API reports rules applied and before/after node counts.
  - CLI tool `mirr-simplify` with `--stats` flag, Expr JSON and `.mirr` file modes.
  - Wired into temporal lowering pipeline as pre-lowering pass.
  - 58 unit and integration tests with full rule coverage.

- **Scope (Future — Phase 3b: Full SmaRTLy):**
  - MiniSAT integration: convert logic cone around each MUX into a Boolean Satisfiability problem. If unsatisfiable ("no input vector affects this MUX output"), remove it.
  - Sub-Graph Pruning via Ancestry Theorem: a signal S can only affect signal T if S is an ancestor of T, T is an ancestor of S, or they share a common ancestor. Prunes ~80% of circuit from each SAT problem instance.
  - Symmetry Theorem: further refine pruning by identifying irrelevant logic cones.
  - Fast Inference Rules before SAT: OR-gate propagation and constant folding to resolve large portions before invoking the expensive solver.
  - ADD-based structural rebuilding: Algebraic Decision Diagrams for optimal MUX tree variable ordering. Cost function (tree height + cell count) confirms rebuilt structure is superior to original.
  - **Target benchmarks:** 8.95% AIG reduction on RISC-V (early milestone), 47.2% on industrial designs (full milestone).

**Result artifact:** CLI tool `mirr-simplify` that reads a netlist/IR and prints a reduced version, with statistics on gate count reduction.

---

## Phase 4 – Bit-Width Inference (FIRWINE-Inspired, Completed)

- **Goal:** Implement a robust bit-width inference and checking pass for arithmetic IR — preventing the bit-width mismatch failures of early R-SPUs where assigning a 12-bit sensor value to a 10-bit register caused silent data corruption.

**Why FIRWINE matters:** Width inference is modeled as a system of inequality constraints (Φ_W-constraints). Wang et al. (2026) proved the Unique Least Solution theorem: if η₁ and η₂ are both valid solutions, their component-wise minimum is also valid. The compiler doesn't search for a good solution — it calculates the mathematically optimal minimum width that guarantees zero data loss while minimizing silicon area.

- **Scope (Phase 4a — Core, Completed):**
  - Extend IR to support integer operations (add, sub, mul, shifts).
  - Flatten Expr trees to post-order FlatNode arrays for iterative processing.
  - Encode and solve width constraints via monotonic iterative propagation (bounded by MAX_PROPAGATION_ROUNDS).
  - Termination proof: widths only increase, maximum value is 64, total increases ≤ 64 x node_count.
  - Hard-error on width > 64 bits with clear message instructing restructuring.
  - Informational note on unsigned subtraction (common in hardware, worth flagging).
  - Detect unsafe truncations at compile time with exact diagnostic messages pinning signal name and bit widths.
  - Width inference across all expressions: guard conditions (validate to boolean) + all reflex RHS.
  - New `src/width/` module — 6 files, FIRWINE-inspired constraint solver with 9 constraint kinds.
  - `mirr-width` CLI runs simplification automatically before inference.
  - 67 integration tests across 13 categories, every diagnostic message pinned by exact text.

- **Scope (Phase 4b — FIRWINE Complete):**
  - **Strongly Connected Component (SCC) detection** in width constraint graph — required for loops like `x <= x + 1` (accumulators, counters, state machines, shift registers).
  - **Expansive SCCs** (values can grow — counters, accumulators): Branch-and-Bound algorithm to find smallest width satisfying loop constraints without overflow.
  - **Nonexpansive SCCs** (values circulate but don't grow — state machine encoding, shift registers): modified Floyd-Warshall algorithm for fixed-point width where constraints stabilize.
  - **Unique Least Solution verification:** confirm solver finds the mathematically optimal minimum, not just any valid width.
  - **Semantic Guard Wrappers:** pre-synthesis assertions defining "Physiological Plausibility" for input signals — maximum rates of change, absolute physical bounds (e.g., human heart rate cannot exceed 300 bpm). Protects against valid-but-incorrect data from sensor failure (e.g., disconnected sensor reading as valid "0"). Extends FIRWINE beyond arithmetic overflow into semantic data integrity.

**Result artifact:** CLI tool `mirr-width` that computes widths, reports unsafe truncations, and emits a fully width-annotated IR/netlist.

---

## Phase 5 – MAPE-K Simulation Harness (Not Started)

- **Goal:** Simulate the Monitor–Analyze–Plan–Execute–Knowledge (MAPE-K) loop for clinical and safety-critical scenarios — transitioning R-SPU from "hardware as a resource" to "hardware as an agent."

**MAPE-K loop architecture:**
- **Monitor:** Shadow register chains + Embedded Trace Buffers capture KPIs (localized voltage, die temperature, bus throughput, logic toggle rates). Pre-processed by hardware monitors for threshold violations — not buffered for software analysis.
- **Analyze:** Hardware-accelerated LTL Checker evaluates real-time signals against Temporal Assertions within the current clock cycle. Triggers immediate transition to pre-verified safe state on invariant violation.
- **Plan:** Query the **Knowledge Base of pre-synthesized bitstreams** — configurations optimized offline by SmaRTLy for specific fault scenarios. Never synthesizes new logic on the fly. Selects from a verified library.
- **Execute:** DPR loads selected partial bitstream into target Reconfigurable Tile. Cement2 ensures temporal synchrony during transition — no glitches, no missed events.
- **Knowledge:** Stores FIRWINE proofs and safety rules as the formal foundation for all planning decisions.

- **Scope:**
  - Model a sensor pipeline (respiratory rate, ECG) as a Rust component graph.
  - Monitor: sample sensor data and noise from a stochastic model.
  - Analyze: check LTL-like invariants over recent history.
  - Plan: select from pre-defined configurations in a pre-synthesized bitstream library.
  - Execute: dynamically reconfigure the pipeline at runtime.
  - Log all adaptation decisions and state transitions for auditability.

- **Case study 1 — Autonomic Epilepsy Monitor (Sayeed et al. 2024):**
  - Low Risk (Sleep) → lightweight SVM classifier via DPR (small logic area, minimal power).
  - Anomaly Suspected → DPR swap to high-precision CNN.
  - Target: >98% CNN sensitivity during critical moments, low-power SVM baseline.

- **Case study 2 — Self-Healing Respiratory Monitor, Neonatal (Kwon et al. 2021):**
  - Continuous SNR monitoring → LTL compliance check → Kalman Filter variant selection on drift detection.
  - Adapts to degrading sensor in real-time, prevents alarm fatigue without nurse intervention.

- **Future (Phase 5b — Split MAPE-K):**
  - Monitor & Execute in FPGA fabric (nanosecond response).
  - Analyze & Plan offloaded to paired ARM processor (complex LTL evaluation, bitstream selection).
  - **Dual-Layer Reflex System:**
    - Immediate Layer (Static): safety clamps ("halt", "reset", "safe-mode") triggerable in a single clock cycle.
    - Adaptive Layer (Dynamic): DPR reserved for "Plan" phase after clamp engages.
    - Decouples reaction time from FPGA configuration bus bandwidth.

**Result artifact:** Rust binary that runs a time-stepped simulation and logs all adaptation and reconfiguration events.

---

## Phase 6 – Integration and Visualization (Not Started)

- **Goal:** Integrate all previous tools into a cohesive, auditable "mini-EDA" flow.
- **Scope:**
  - End-to-end pipeline: parse MIRR source → simplify logic → assign bit-widths → emit netlist and temporal guards.
  - Generate Graphviz `.dot` files from IR/netlist for visualization and debugging.
  - Emit Verilog/VHDL/SystemVerilog RTL from the IR.
  - Single driver binary or workspace for the entire compile-and-analyze pipeline.

**Result artifact:** Unified driver binary (or cargo workspace) that performs a full compile-analyze run, suitable as the engine for future R-SPU toolchains.

---

## Phase 7 – Myth-Inspired Language & Formal Verification (Not Started)

- **Goal:** Evolve the MIRR DSL into a highly expressive language with formal correctness guarantees — and fully establish MIRR's third role as a Runtime Instruction Language the fabricated R-SPU chip understands natively.

- **Scope:**
  - **Advanced Type System:** dependent types, linear types, effect systems. Extends Strict Static Typing from Phase 1 — no implicit casting, every bit-width explicit, every `zext` visible.
  - **Higher-Order Functions:** function composition and higher-order constructs for complex signal processing.
  - **Metaprogramming:** template system and compile-time code generation for hardware specialization.
  - **Formal Verification in Rocq:** implement width inference proofs in Rocq (formerly Coq) interactive theorem prover, matching the FIRWINE approach (Wang et al. 2026). Auto-extract verified executable from proofs. Formally proven compiler correctness — eliminates compiler-induced bugs at the mathematical level.
  - **Hardware Synthesis:** generate optimized Verilog/VHDL/SystemVerilog from high-level specifications.
  - **Performance Modeling:** predict timing, power, and area characteristics before synthesis.
  - **Runtime Instruction Language:** same MIRR language used to write software that runs on the fabricated R-SPU natively. Seamless continuum from design-time specification to runtime operation.

**Result artifact:** Production-grade compiler for "Million Dollar Labs" style rapid prototyping of safety-critical embedded systems.

---

## Phase 8 – R-SPU Architecture Design & RTL Implementation (Not Started)

- **Goal:** Design and implement the Reflexive Processing Unit (R-SPU) hardware architecture.
- **Scope:**
  - **R-SPU Core Design:** RTL specification for the R-SPU processor core with reflexive capabilities.
  - **FPGA Fabric Partitioning (Critical Architectural Constraint):** Static Shell (MAPE-K controller, Knowledge Base, LTL Checker) is NEVER reconfigured. Reconfigurable Tiles receive partial bitstreams from DPR Controller. Static Shell must remain stable for the system to maintain safety guarantees during dynamic reconfiguration.
  - **Memory Architecture:** specialized memory hierarchies for temporal signal processing.
  - **I/O Subsystem:** adaptive I/O interfaces for real-time sensor data.
  - **Reconfiguration Engine:** DPR Controller manages partial bitstream loading into Reconfigurable Tiles. Cement2 ensures temporal synchrony during tile transitions.
  - **Safety Mechanisms:** hardware fault detection, error correction, fail-safe modes. Safety Clamps (Critical Override) in static logic for immediate single-cycle hazard response. DPR security patterns (Sunkavilli et al. 2022) for FPGA design obfuscation.
  - **LTL Assertion Layer:** hardware-accelerated LTL Checker evaluating safety invariants within each clock cycle. Intentionally higher silicon cost than AHL — necessary trade-off for clinical integrity. Standard methods protect the clock; the R-SPU's LTL layer protects the application.
  - **Power Management:** dynamic voltage/frequency scaling for energy efficiency.

**Result artifact:** Complete RTL implementation of the R-SPU processor, ready for FPGA synthesis and ASIC design.

---

## Phase 9 – R-SPU Fabric & Multi-Core Integration (Not Started)

- **Goal:** Scale the R-SPU design to multi-core fabric architectures for complex, safety-critical embedded systems.
- **Scope:**
  - Interconnect Fabric, Distributed Memory, Load Balancing, Fault Tolerance, Thermal Management, Security.

**Result artifact:** Multi-core R-SPU fabric, ready for deployment in safety-critical embedded systems.

---

## Phase 10 – Production Deployment & Certification (Not Started)

- **Goal:** Deploy R-SPU systems in real-world, safety-critical applications with full certification.
- **Scope:**
  - Medical: ventilators, patient monitors, neonatal respiratory monitors, epilepsy wearables, implantable medical robotics.
  - Aerospace: flight control systems, satellite subsystems.
  - Automotive: ADAS, autonomous vehicle controls (Guo et al. 2024).
  - Certification: DO-178C (aerospace), IEC 62304 (medical), ISO 26262 (automotive).
  - Field testing, validation, volume manufacturing.

**Result artifact:** Certified, production-ready R-SPU systems deployed in safety-critical applications worldwide.

---

## The Ultimate Goal

A complete ecosystem where domain experts specify safety-critical system behavior at a high level, and the toolchain automatically generates optimized, formally verified R-SPU hardware implementations deployable with NASA-level confidence in life-critical applications.

The novel claim: integrating Cement2, SmaRTLy, and FIRWINE into a unified language and architecture (MIRR/R-SPU) is transformative. Each technology is already proven. MIRR is the orchestration layer that makes them work together — and the only language that simultaneously serves as design language, compiler toolchain, and runtime instruction language.

---

## Key Benchmarks to Target

| Technology | Metric | Source |
|---|---|---|
| Cement2 | 377 MHz timing closure on RISC-V soft-core | Xiao et al. 2025 |
| SmaRTLy | 8.95% AIG reduction on RISC-V (early milestone) | Li et al. 2025 |
| SmaRTLy | 47.2% AIG reduction vs Yosys (industrial, full milestone) | Li et al. 2025 |
| FIRWINE | Unique Least Solution — formally proven optimal | Wang et al. 2026 |
| R-SPU LTL | Sub-cycle fault detection (nanosecond response) | Architecture goal |
| R-SPU DPR | Millisecond reconfiguration + static clamp in 1 cycle | Architecture goal |

---

## Research Foundation

Performance claims (377 MHz, 47% area reduction) are drawn from the original papers as evidence that the underlying components are already proven. The novel contribution of this project is their integration into a unified language experience under MIRR.

**Core Technologies:**
- Xiao, Y. et al. (2025). Cement2: Temporal hardware transactions for FPGA programming. arXiv:2511.15073
- Li, C. et al. (2025). SmaRTLy: RTL optimization with logic inferencing and structural rebuilding. arXiv:2510.17251
- Wang, K. et al. (2026). FIRWINE: A formally verified procedure for width inference in FIRRTL. arXiv:2601.12813
- Arcaini, P. et al. (2015). Modeling and analyzing MAPE-K feedback loops for self-adaptation. SEAMS 2015.

**Foundational Theory:**
- Pnueli, A. (1977). The temporal logic of programs. 18th Annual Symposium on Foundations of Computer Science. IEEE. — The foundational paper from which all LTL in this project traces its lineage.
- Cong, J., & Zhang, Z. (2006). An efficient and versatile scheduling algorithm based on SDC formulation. DAC 2006.

**Hardware Reliability & Aging:**
- Lin, I.-C. et al. (2016). Aging-aware reliable multiplier design with adaptive hold logic. IEEE Trans. VLSI Systems, 24(3), 844–853.

**Target Application Domains:**
- Guo, J.-I., & Chen, Y.-L. (2024). ConcentrateNet: Multi-scale object detection for ADAS. Sensors, 24(5), 1682.
- Wu, Y.-C. et al. (2020). 28nm fully integrated genome analysis accelerator. IEEE Trans. Biomedical Circuits and Systems, 14(6), 1262–1274.
- Sayeed, M.A. et al. (2024). Real-time multi-channel epileptic seizure detection. Sensors, 24(22), 7175.
- Kwon, S. et al. (2021). Non-contact respiratory monitoring using an RGB camera. Sensors, 21(16), 5429.
- Sunkavilli, S. et al. (2022). DPReDO: DPR-enabled design obfuscation for FPGA security. IEEE SOCC 2022.