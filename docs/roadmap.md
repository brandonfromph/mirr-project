---
title: Roadmap
nav_order: 4
---

# R-SPU Compiler & EDA Roadmap

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

{: .tip }
> Phases 0--4, 6, 7a, 7b, 7c complete. Phase 5 partial.
> The compiler is operational with ~1157 passing tests, zero unsafe code,
> and zero clippy warnings. Synthesis validated through Yosys (11/11 examples).
> Formal proofs: 27 Rocq theorems (14 fully mechanized, 13 axiomatized).
> Code metrics: 22,315 source lines + 18,200 test lines = 40,515 total across 127 files.
> Error codes: 162 unique codes (E100--E705).

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

- **Scope (Phase 4b — FIRWINE Complete, Completed):**
  - **`Expr::Prev { signal, delay }`** — new IR variant for temporal back-references (shift registers, accumulators). Delay ≥ 1 enforced by semantic validation. Wired through executor, semantic validator, all width pass stages, simplifier, and MAPE-K harness.
  - **Width Dependency Graph** (`src/width/graph.rs`) — directed graph tracking signal-to-signal width dependencies through all reflex assignment RHS expressions, including `Prev` back-edges that create cycles.
  - **Iterative Tarjan's SCC** (`src/width/scc.rs`) — finds strongly connected components; O(V+E), bounded by `MAX_SIGNALS = 1024`. All traversal via explicit work stack, zero recursion (NASA P10).
  - **SCC classification:** Expansive (cycle path contains `Add`, `Mul`, or `Shl` — values can grow), Nonexpansive (cycle contains only `Prev`, bitwise ops, comparisons — values circulate but don't grow).
  - **Nonexpansive solver** (`src/width/scc_solver.rs`) — Floyd-Warshall fixpoint: all signals in the SCC converge to the maximum declared width among members. Bounded by `MAX_SCC_SIZE² = 4096` iterations.
  - **Expansive solver** — guard-bound inference: extracts upper bound from guard cycle-count annotations, computes `min_bits_for(bound)`. Hard error (reject compilation) if no bound is provable — no silent u64 default.
  - **Unique Least Solution verification** (`src/width/verify.rs`) — confirms each solved width is not over-wide relative to declared signal types; emits COMPILER BUG diagnostic if minimality is violated.
  - **`scc_member_names: HashSet<String>`** in `SccWidthResult` — correct Phase 4a/4b diagnostic split. Phase 4a correctly flags `prev(counter)+1` as truncation; Phase 4b suppresses that error for SCC-member targets since it owns their width determination.
  - **`--scc` flag** added to `mirr-width` CLI — activates the full Phase 4b pipeline.
  - 4 new modules (`graph.rs`, `scc.rs`, `scc_solver.rs`, `verify.rs`), 10 files total in `src/width/`, 38 new integration tests across 9 categories (105 total for Phase 4).

- **Scope (Phase 4c — Semantic Guard Wrappers, Future):**
  - Pre-synthesis assertions defining "Physiological Plausibility" for input signals — maximum rates of change, absolute physical bounds (e.g., human heart rate cannot exceed 300 bpm). Protects against valid-but-incorrect data from sensor failure (e.g., disconnected sensor reading as valid "0"). Extends FIRWINE beyond arithmetic overflow into semantic data integrity.

**Result artifact:** CLI tool `mirr-width` that computes widths, reports unsafe truncations, detects cyclic width dependencies via SCC analysis, and emits a fully width-annotated IR/netlist.

---

## Phase 5 – MAPE-K Simulation Harness (Partial — Phase 5a Complete, 5b Not Started)

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

## Phase 6 – Integration and Visualization (Completed)

- **Goal:** Integrate all previous tools into a cohesive, auditable "mini-EDA" flow.
- **Scope:**
  - End-to-end pipeline: parse MIRR source → validate → simplify logic → assign bit-widths → temporal lowering → emit output.
  - `PipelineConfig` with toggles for simplification, width inference, and temporal compilation.
  - Generate Graphviz `.dot` files from IR/netlist for visualization and debugging (with `--dot-detail expr` for full AST subgraphs).
  - Emit SystemVerilog RTL, JSON netlist, and DOT graph from the unified pipeline.
  - Single driver binary `mirr-compile` with `--emit verilog|dot|json` flags.
  - Bootstrap runner (`mirr-parse`) with parity checks against the unified pipeline.
  - 1041 tests passing, zero clippy warnings, zero unsafe code.

**Result artifact:** Unified driver binary `mirr-compile` that performs a full compile-analyze run with multiple output formats.

---

## Phase 7a – Safety Properties & SVA Assertion Emission (Completed)

- **Goal:** Add a `property` keyword to the MIRR DSL that compiles to SystemVerilog Assertions (SVA), enabling formal verification of safety invariants without affecting generated hardware.

- **Scope (Completed):**
  - **AST:** `PropertyFormula` enum (`Always(Expr)`, `Never(Expr)`, `AlwaysImplies { antecedent, consequent }`) and `PropertyDecl` struct in `src/ast/property.rs`. `Module` extended with `properties: Vec<PropertyDecl>` (`#[serde(default)]` for backward compatibility).
  - **Parser:** `property` keyword dispatch in `module_parser.rs`. Multi-line block syntax matching `guard`/`reflex` pattern. `->` implication handled at property parser level (string split), not in tokenizer or expression parser — avoids ripple to simplifier, width inference, etc.
  - **Validation:** Duplicate property name detection. Signal reference validation — all signals referenced in property formulas must be declared in the module.
  - **Pipeline:** Property expressions simplified through the existing `simplify_expr_with_stats` pass.
  - **SVA emission:** `assert property (@(posedge clk) ...)` blocks in SystemVerilog output. `disable iff (!rst_n)` emitted only when `rst_n` is declared as an input signal. Three SVA forms: `always(P)` → `assert P`, `never(P)` → `assert !(P)`, `always(P -> Q)` → `assert P |-> Q`.
  - **JSON emission:** `"properties"` array in JSON netlist with `name`, `kind`, `formula_text` fields.
  - **DOT emission:** Property nodes (`shape=note`, lightblue fill) with dotted blue edges from referenced signals.
  - **CLI:** `--emit sva` for standalone SVA output (no module wrapper). `emit_sva_only()` public API.
  - 40 new tests (520 total). Zero clippy warnings, zero unsafe code.

**Result artifact:** MIRR programs can now declare safety properties that compile to industry-standard SVA assertions, usable by any SystemVerilog simulation or formal verification tool.

---

## Phase 7b – Homoiconic Pattern System (Completed)

- **Goal:** Add compile-time pattern expansion to MIRR via the `def`/`reflect` keywords, enabling reusable hardware structure definitions with DO-178C traceability.

- **Scope (Completed):**
  - `def` keyword for pattern definitions with signal (`signal in u16`, `signal out bool`) and constant (`u16`, `u32`, `bool`) parameters.
  - `reflect` block contains a template body with `${param}` interpolation markers. Body is stored as raw text, substituted at compile time, then re-parsed by the existing module parser.
  - Name prefixing scheme: `{pattern_name}_{call_index}_{original_name}` for collision-proof naming across multiple calls.
  - Origin tagging: `origin: Option<String>` on `Guard`, `Reflex`, `SignalDecl`, `PropertyDecl` for DO-178C traceability. Verilog emitter outputs `// Pattern: monitor_sensor_0` comments. DOT emitter adds tooltips. JSON omits the field when `None` (serde `skip_serializing_if`).
  - Internal signal scoping: pattern-internal signals (`SignalKind::Internal`) cannot be referenced outside their expansion. Cross-expansion references also blocked.
  - Bounded expansion: `MAX_EXPANSION_DEPTH=4`, `MAX_EXPANDED_ITEMS=256`, `MAX_PARAMS=32`, `MAX_ARGS=32`, `MAX_REFLECT_LINES=512`, `MAX_BRACE_DEPTH=16`.
  - Pipeline ordering: parse → `validate_pattern_defs` → `expand_patterns` → validate_module → simplify → width → temporal → emit.
  - New modules: `src/ast/pattern.rs`, `src/parser/pattern_parser.rs`, `src/expand/mod.rs`.
  - 112 pattern tests across parser, validation, expansion, scoping, emission, and pipeline integration categories. **1041 total tests, zero clippy warnings.**

**Result artifact:** Reusable pattern definitions that expand at compile time into validated, origin-tagged hardware structures.

---

## Phase 7c — Advanced Type System (Complete)

- **Goal:** Extend MIRR's strict static type system with expressive type-level features that encode hardware constraints directly in the type language, enabling the compiler to reject physically invalid designs before synthesis.

- **Scope:**
  - Dependent types for encoding hardware constraints (e.g., FIFO depth parameterized by pipeline stage count).
  - Linear types for resource tracking: enforce one-writer, one-reader signal semantics at the type level, preventing accidental multi-driver nets.
  - Refinement types with compile-time range checking (e.g., `u8{0..=200}` for physiological sensor bounds).
  - Effect system distinguishing pure combinational logic from stateful sequential logic — prevents accidental register inference in combinational contexts.
  - Type-level natural numbers for dimension-safe signal arrays and bus widths, eliminating width mismatch classes that FIRWINE currently catches at constraint-solving time.
  - Substructural type qualifiers for clock domain crossing safety — signals tagged with their clock domain cannot be mixed without explicit synchronizer instantiation.
  - Phantom types for unit-of-measure tracking (voltage, current, temperature) through signal processing chains.
  - Extended Rocq proofs covering type system soundness: progress and preservation theorems for the core type language.
  - Integration with Phase 4's FIRWINE solver: refinement type bounds feed directly into width constraint generation.
  - Backward compatibility layer: existing MIRR programs without advanced type annotations remain valid (all new features opt-in).

**Campaign 025 progress (FOUNDATION-001 + MEGA-1, Phase A--C):**
- Parser-level type annotations: `Linearity`, `EffectQualifier`, `Refinement`, `TypeAnnotations` in `src/ast/types.rs`.
- Token-stream signal parser: `tokenize_signal_decl()` in `src/parser/mod.rs` handling `linear`, `stateful`, `pure`, `where`, `@clock`, `#Tag` syntax.
- Extended type checker: `src/typeck/extended.rs` (2138 lines) implementing refinement bounds, linear tracking, effect qualification, clock domain verification, phantom tags, session types. Error codes E610--E625.
- Parser error codes E170--E183 for type annotation diagnostics.
- Sections D--H completed by campaign MEGA-1b (proposal 027). Extended type checker wired into pipeline. Phase 7c closed.

**Result artifact:** Type checker module (`src/typeck/`) extended with dependent, linear, and refinement type support, with Rocq-mechanized soundness proofs in `proofs/types/`.

---

### Phase 7d — S-Expression IR (MEGA-2)

- **Status:** Complete
- **Proposal:** 026-MEGA2-LISP-CORE-2026-03-10
- **New module:** `src/sexpr/` (8 files, ~2,300 lines)
- **New emit backend:** `--emit sexpr` (S-expression output)
- **Error codes:** E800–E815 (S-expression errors)
- **Tests:** ~120 new tests across 3 test files
- **Key feature:** Bidirectional AST ↔ S-expression conversion with round-trip invariant

---

## Phase 7d — Metaprogramming & Code Generation (Complete)

- **Goal:** Provide structured compile-time program transformation capabilities, extending Phase 7b's `def`/`reflect` pattern system into a full metaprogramming framework for parametric hardware generation.

- **Scope:**
  - Structured intermediate representation for compile-time program transformation — typed AST manipulation rather than text substitution.
  - Hygienic macro system extending the `def`/`reflect` pattern system from Phase 7b, with guaranteed name hygiene across expansion boundaries.
  - Reader extensions for domain-specific notations: timing constraint literals (`50ns`, `1.2GHz`), physical unit annotations, and signal range specifications.
  - Compile-time evaluation engine for template instantiation with bounded execution (NASA P10 compliant: no unbounded recursion, configurable `MAX_EVAL_STEPS`).
  - Parametric module generation: N-way Triple Modular Redundancy (TMR), configurable pipeline depths, variable-width crossbar switches — all from single parameterized definitions.
  - Compile-time assertion system: `static_assert` for validating parameter constraints before expansion (e.g., TMR voter width must match data path width).
  - Introspection API: patterns can query signal properties (width, direction, clock domain) of their arguments at expansion time.
  - Code generation audit trail: every generated construct carries provenance metadata linking back to its macro definition and call site (extends Phase 7b's origin tagging).
  - Integration with Phase 7c type system: macro parameters carry type constraints, expansion is type-checked before and after.
  - Bounded expansion guarantees inherited from Phase 7b: `MAX_EXPANSION_DEPTH`, `MAX_EXPANDED_ITEMS` enforced across all metaprogramming constructs.

**Result artifact:** Extended pattern expansion engine (`src/expand/`) with hygienic macros, compile-time evaluation, and parametric module generation. CLI flag `--expand-only` for inspecting expanded output before compilation.

---

## Phase 7e — R-SPU Instruction Set Architecture (Complete)

- **Goal:** Define the formal Instruction Set Architecture (ISA) for the R-SPU execution model, establishing MIRR as the native instruction language that compiles to R-SPU microcode.

- **Scope:**
  - Formal ISA specification document with precise encoding formats, instruction semantics, and operational modes.
  - MIRR as native instruction language: compile MIRR behavioral descriptions directly to R-SPU microcode sequences.
  - Tagged-word architecture: every data word carries type metadata and provenance tags in hardware, enabling runtime type safety without software overhead.
  - Hardware type dispatch: instruction decoder inspects type tags for zero-overhead polymorphic operation selection (e.g., fixed-point vs. integer arithmetic resolved by tag, not by opcode).
  - Native temporal instruction support: `DELAY`, `GUARD_CHECK`, `PROPERTY_EVAL` as first-class ISA operations with dedicated execution units.
  - Microcode generation from MIRR behavioral descriptions: the compiler emits microcode sequences that the R-SPU control unit executes directly.
  - Dual-mode execution: deterministic reflex mode (cycle-accurate, bounded latency) and cognitive host mode (interrupt-driven, variable latency) selectable per instruction stream.
  - Register file design: split register banks for temporal state (shift register shadows), combinational intermediates, and persistent configuration.
  - Exception model: hardware traps for type tag violations, temporal deadline misses, and safety property failures — all routed to the MAPE-K Analyze stage.
  - ISA simulation model in Rust: cycle-accurate software simulator for validating ISA semantics before RTL implementation in Phase 8.
  - Instruction encoding density analysis: minimize code size for the on-chip instruction memory budget.

**Result artifact:** Formal ISA specification (`docs/rspu_isa_spec.md`), Rust-based cycle-accurate ISA simulator (`src/emit/rspu_sim.rs`), and microcode assembler integrated into `mirr-compile --emit rspu`.

**Delivered by MEGA-3 (2026-03-10):**
  - Binary encoding: 32-bit fixed-width instruction words in 4 formats (R-type, I-type, G-type, S-type).
  - Tagged-word register file: 256 registers with 4-bit type tags (`Bool`, `Unsigned`, `Signed`, `Uninitialized`) and 4-bit provenance tracking.
  - Exception model with dual-mode execution: bounded trap depth (`MAX_EXCEPTION_DEPTH = 8`), handler table (`MAX_TRAP_HANDLERS = 16`), reflex-mode fail-safe (unhandled exception triggers `EMERGENCY_STOP`), host-mode trap-to-software.
  - Cycle-accurate ISA simulator (`src/emit/rspu_sim.rs`) with bounded execution (`MAX_SIM_CYCLES`).
  - ISA extended from 20 to 30 instructions: `TRAP`, `TRAP_IF`, `HALT`, `MODE_SWITCH`, `NOP`, `FENCE`, `TAG_LOAD`, `TAG_CHECK`, `TAG_READ`, `DEADLINE_SET`.
  - Rocq formal proofs for encoding bijectivity and tagged-word invariants.
  - Error codes E706–E715 activated for encoding, tagging, exceptions, simulation, and deadline enforcement.

---

## Phase 7f — Proof-Carrying Code Infrastructure (Not Started)

- **Goal:** Implement a proof-carrying code framework where compiled R-SPU programs carry machine-checkable safety certificates alongside their executable code, enabling hardware-verified trust.

- **Scope:**
  - Proof-carrying code model: every compiled R-SPU binary includes a compact safety certificate attesting to specific verified properties.
  - Certificate format design: compact proof witnesses checkable in bounded clock cycles by a hardware verification unit.
  - Safety properties encoded in certificates: type safety (no tag violations), temporal safety (all deadlines met), width safety (no truncations), and resource safety (no multi-driver conflicts).
  - Hardware proof verification unit specification: a dedicated on-chip module that validates certificates before permitting code execution on the R-SPU core.
  - Integration with Rocq extraction: verified proof checkers extracted from Rocq proofs into synthesizable hardware descriptions.
  - Total function enforcement: the MIRR compiler must prove termination for all R-SPU reflex-domain code — non-terminating programs are rejected at compile time.
  - Termination analysis via structural recursion bounds and loop iteration limits (extends NASA P10 bounded-iteration requirement to a formal proof obligation).
  - Certificate chaining: when modules compose, their certificates compose — system-level safety follows from component-level certificates.
  - Revocation and versioning: certificates reference compiler version and proof context, enabling invalidation when compiler bugs are discovered.
  - Bounded verification cost: certificate checking must complete within a configurable cycle budget (hard real-time requirement for safety-critical deployment).

**Result artifact:** Certificate generation pass in the compiler (`src/certify/`), certificate format specification, and Rocq-extracted hardware proof checker RTL.

---

## Phase 7g — Symbolic Evaluation Engine (Not Started)

- **Goal:** Build a hardware-acceleratable symbolic evaluation engine for runtime signal classification, constraint resolution, and logic optimization within the R-SPU execution model.

- **Scope:**
  - Hardware-accelerated pattern matching engine for classifying signal waveforms against known templates (e.g., QRS complex detection, sensor drift signatures).
  - Constraint resolution engine for runtime sensor calibration: solve systems of inequalities derived from physical sensor models within bounded cycle budgets.
  - Symbolic signal processing primitives: discrete differentiation, integration approximations, and moving-window statistics computed symbolically for precision analysis.
  - Abstract interpretation passes realized as hardware acceleration targets: interval analysis, sign analysis, and range propagation for runtime signal characterization.
  - Term rewriting engine for runtime logic optimization: dynamically simplify monitoring expressions as runtime knowledge narrows signal ranges.
  - Bounded rewriting depth: all symbolic evaluation operations have configurable maximum step counts (NASA P10 compliance).
  - Integration with Phase 3's SmaRTLy simplifier: the symbolic engine reuses algebraic simplification rules in a runtime context.
  - Signal fingerprinting: compute compact signatures of signal behavior over configurable time windows for anomaly detection.
  - Configurable precision modes: trade symbolic precision for evaluation speed based on current safety criticality level.
  - Software simulation model in Rust before hardware realization: all symbolic primitives implemented and tested as software, then mapped to RTL in Phase 7h.

**Result artifact:** Symbolic evaluation library (`src/symbolic/`) with Rust simulation model, hardware mapping annotations, and integration into the MAPE-K Analyze stage.

---

## Phase 7h — MAPE-K Hardware Realization (Not Started)

- **Goal:** Port Phase 5's MAPE-K simulation harness to synthesizable RTL, creating a hardware-realized autonomic control loop with sub-microsecond response times.

- **Scope:**
  - Translate the Phase 5a Rust MAPE-K simulation model to synthesizable SystemVerilog RTL via the MIRR compilation pipeline.
  - Hardware LTL checker with configurable property depth: evaluate temporal safety assertions within the current clock cycle using dedicated comparison and shift-register logic.
  - On-chip knowledge base: pre-verified configuration library stored in on-chip SRAM or block RAM, indexed by fault signature for constant-time lookup.
  - DPR controller with Cement2 temporal synchrony: manage partial bitstream loading into reconfigurable tiles without temporal discontinuity — no missed events during reconfiguration.
  - Safety clamp integration: single-cycle hazard response implemented in the static shell — clamps engage independently of and prior to any DPR activity.
  - Monitor stage hardware: shadow register chain with hardware pre-processing (threshold comparators, rate-of-change detectors) feeding the Analyze stage directly.
  - Analyze stage hardware: LTL checker array with priority encoder for multi-property evaluation and fault classification.
  - Plan stage hardware: lookup table indexed by fault classification, returning pre-verified bitstream identifiers and safety clamp configurations.
  - Execute stage hardware: DPR sequencer with Cement2 guards ensuring reconfigurable tile transitions complete atomically with respect to the temporal domain.
  - Formal verification of the hardware MAPE-K loop: Rocq proofs that the hardware realization preserves the safety properties validated in Phase 5's simulation.
  - Area and timing budgets: LTL checker must meet target frequency with < 5% area overhead relative to the monitored design.

**Result artifact:** Synthesizable MAPE-K RTL modules (`src/emit/mape_k_rtl.rs`), Yosys synthesis validation, and Rocq proofs of behavioral equivalence with the Phase 5 simulation model.

---

## Phase 7i — Verified Compilation Chain (Not Started)

- **Goal:** Establish an end-to-end formally verified compilation chain from MIRR source code to gate-level netlist, providing mathematical proof that generated hardware preserves source-level semantics.

- **Scope:**
  - Rocq-verified compiler passes: extend Phase 4's width inference proofs to cover ALL compiler passes — parsing, type checking, simplification, temporal lowering, and code emission.
  - CompCert-inspired verified compilation methodology: each compiler pass accompanied by a simulation relation proof showing output preserves input semantics.
  - Proof that generated RTL preserves MIRR source semantics: formal simulation relation between MIRR behavioral specification and emitted SystemVerilog.
  - End-to-end formal chain: source → typed IR → simplified IR → temporal IR → RTL → gate-level netlist, with verified preservation at each boundary.
  - Verified constant folding: proof that compile-time evaluation produces the same result as runtime evaluation for all constant expressions.
  - Verified width inference: extend existing Rocq proofs (27 theorems) to full coverage of the width constraint system including SCC cycles.
  - Verified temporal lowering: proof that Cement2 shift-register synthesis preserves the temporal semantics of `delay(k)` guards.
  - Verified simplification: proof that SmaRTLy-inspired algebraic rules preserve logical equivalence (all 33 rules individually proven).
  - Certification artifacts for DO-178C Level A (aerospace) and IEC 62304 Class C (medical device software): compiler qualification data package derived from Rocq proofs.
  - Regression proof infrastructure: CI pipeline runs Rocq proof checking on every commit, ensuring proofs remain valid as the compiler evolves.
  - Proof maintenance tooling: when a compiler pass changes, identify which proofs are invalidated and require update.

**Result artifact:** Fully verified compilation chain with Rocq proofs covering all passes (`proofs/` expanded to cover full pipeline), DO-178C/IEC 62304 certification data package, and CI-integrated proof regression suite.

---

## Phase 8a — R-SPU Core Architecture (Not Started)

- **Goal:** Design and implement the R-SPU processor core with reflexive capabilities as synthesizable RTL.
- **Scope:**
  - Word architecture: 64-bit data words with type metadata tags for runtime type safety.
  - Datapath design: ALU, FPU, dedicated evaluation units for signal processing.
  - Register file with hardware type checking and provenance tracking.
  - Memory subsystem with metadata support and bounded allocation.
  - 5-stage pipeline with proof verification interlock stage.
  - I/O subsystem: sensor interface with ADC/DAC integration points.
  - Dual clock domains: deterministic reflex domain (synchronous) and cognitive host domain (asynchronous).
  - Static shell (safety clamps, MAPE-K controller, LTL checker) — never reconfigured.
  - Reconfigurable tiles receiving partial bitstreams via DPR controller.
  - Mixed-signal interface layer for analog sensor front-end.
  - Cement2 temporal primitives realized in silicon (shift registers, guard units).
  - Exception handling: hardware-enforced safety modes on invariant violation.
  - Power management: dynamic voltage/frequency scaling per tile.

**Result artifact:** Complete RTL implementation of the R-SPU processor core, validated through Yosys synthesis and formal verification.

---

## Phase 8b — Multi-Core Fabric & Tape-Out Preparation (Not Started)

- **Goal:** Scale R-SPU to multi-core fabric and prepare for physical implementation.
- **Scope:**
  - Network-on-Chip (NoC) interconnect for multi-R-SPU communication.
  - Distributed knowledge base across cores with consistency protocol.
  - Fault tolerance: redundancy and voting mechanisms for safety-critical multi-core.
  - Consensus protocol for distributed property verification across cores.
  - Thermal-aware placement with on-die sensor feedback.
  - Host interface: PCIe/AXI bridge for host CPU to dispatch MIRR programs to R-SPU fabric.
  - FPGA prototype: validated on Xilinx Ultrascale+ and/or Lattice Nexus development boards.
  - Pre-silicon verification: gate-level simulation, formal equivalence checking, timing closure.
  - Tape-out preparation: GDS-II generation, DRC (Design Rule Check), LVS (Layout vs Schematic).
  - Power/performance/area (PPA) characterization from post-synthesis and post-layout data.

**Result artifact:** Multi-core R-SPU fabric tape-out-ready design with FPGA prototype validation.

---

## Phase 9 — Production Deployment & Certification (Not Started)

{: .note }
> Phase 9 is a forward-looking design goal. It describes the
> intended trajectory of the project, not current capabilities.

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

---

## Repository Structure

```text
.
├── src/
│   ├── main.rs                    # CLI entrypoint
│   ├── lib.rs                     # Public API / module exports
│   ├── error.rs                   # Shared error types
│   ├── bootstrap_runner.rs        # Self-host bootstrap pipeline
│   ├── simplify.rs                # Logic simplifier — 33 algebraic rules (Phase 3)
│   ├── pipeline.rs                # Unified compilation pipeline (Phase 6)
│   ├── mirr_executor.rs           # Signal evaluator used by MAPE-K harness (Phase 5)
│   ├── diagnostic.rs              # Diagnostic formatting and error reporting (DIAG-001)
│   ├── suggest.rs                 # Did-you-mean suggestions for misspelled identifiers (DIAG-001)
│   ├── bin/
│   │   ├── mirr-compile.rs        # Unified pipeline CLI: --emit verilog|dot|json|sva (Phase 6)
│   │   ├── mirr-simplify.rs       # Standalone simplifier CLI
│   │   ├── mirr-width.rs          # Bit-width inference CLI; --scc enables Phase 4b
│   │   ├── mirr-simulate.rs       # MAPE-K simulation harness CLI (Phase 5)
│   │   └── generate_mirr_stress.rs
│   ├── ast/
│   │   ├── mod.rs                 # Module declarations and re-exports
│   │   ├── types.rs
│   │   ├── expr.rs                # Includes Expr::Prev for temporal back-references
│   │   ├── program.rs             # MirrProgram, Module (includes properties field)
│   │   ├── property.rs            # PropertyDecl, PropertyFormula (Phase 7a)
│   │   └── pattern.rs             # PatternDef, PatternCall, PatternOrigin (Phase 7b)
│   ├── lexer/
│   │   └── tokenizer.rs
│   ├── parser/
│   │   ├── expr_parser.rs
│   │   ├── module_parser.rs       # Includes property parsing (Phase 7a)
│   │   └── pattern_parser.rs      # Pattern definition and call parsing (Phase 7b)
│   ├── validation/
│   │   └── semantic.rs            # Includes property validation (Phase 7a)
│   ├── typeck/                    # Type checker module (TYPE-001, MEGA-1)
│   │   ├── mod.rs
│   │   └── extended.rs            # MEGA-1 extended type checker (E610--E625)
│   ├── emit/                      # Output emitters — 7 backends (Phase 6+)
│   │   ├── mod.rs
│   │   ├── verilog.rs             # SystemVerilog RTL + SVA assertions (Phase 7a)
│   │   ├── dot.rs                 # Graphviz DOT with property nodes (Phase 7a)
│   │   ├── json_netlist.rs        # JSON netlist with properties key (Phase 7a)
│   │   ├── rspu.rs                # R-SPU native code emission
│   │   ├── rspu_isa.rs            # R-SPU instruction set definitions
│   │   ├── rspu_regalloc.rs       # R-SPU register allocator
│   │   ├── firrtl.rs              # FIRRTL intermediate representation emission
│   │   ├── fpga_scaffold.rs       # FPGA scaffold generation (FPGA-001)
│   │   ├── fpga_target.rs         # FPGA target definitions (FPGA-002)
│   │   ├── dsp.rs                 # DSP block emission
│   │   └── testbench.rs           # Testbench generation
│   ├── temporal/
│   │   ├── compiler.rs
│   │   ├── emit.rs
│   │   └── low_level_ir.rs
│   ├── width/                     # FIRWINE bit-width inference (Phase 4)
│   │   ├── mod.rs                 # Pipeline entry; SccWidthResult and has_errors()
│   │   ├── types.rs               # FlatNode, WidthExpr, SccInfo, WidthStats
│   │   ├── flatten.rs             # Expression flattening including Prev back-edges
│   │   ├── constraint.rs          # Constraint generation from flat nodes
│   │   ├── solver.rs              # Acyclic constraint propagation (Phase 4a)
│   │   ├── display.rs             # Human-readable width and SCC reports
│   │   ├── graph.rs               # Width dependency graph construction (Phase 4b)
│   │   ├── scc.rs                 # Iterative Tarjan's SCC detection (Phase 4b)
│   │   ├── scc_solver.rs          # Expansive/nonexpansive SCC solvers (Phase 4b)
│   │   └── verify.rs              # Unique least solution verification (Phase 4b)
│   └── mape_k/                    # MAPE-K adaptive simulation harness (Phase 5)
│       ├── mod.rs
│       ├── monitor.rs
│       ├── analyzer.rs
│       ├── planner.rs
│       └── sensor.rs
├── src/expand/                    # Pattern expansion engine (Phase 7b)
│   └── mod.rs                     # expand_patterns(), name prefixing, scoping validation
├── tests/
│   ├── *_tests.rs                 # Unit/integration suites (1041 tests)
│   ├── property_tests.rs          # Property/SVA tests (Phase 7a)
│   ├── pattern_tests.rs           # Pattern system tests (Phase 7b)
│   ├── pattern_coverage_tests.rs  # Pattern coverage gap tests (Phase 7b)
│   ├── synth_yosys_tests.rs       # End-to-end Yosys synthesis tests (SYNTH-001)
│   └── fixtures/
├── proofs/                        # Rocq (Coq) formal proofs (ROCQ-001)
│   └── width/                     # FIRWINE width inference proofs
│       ├── Types.v, MinBits.v, Constraint.v, Flatten.v
│       ├── Solver.v, Monotone.v, Truncation.v, Integration.v
│       └── SCC/                   # SCC-specific proofs
│           ├── Tarjan.v, Classify.v, Nonexpansive.v
├── compiler_mirr/                 # MIRR-in-MIRR self-hosting port (incremental)
├── stdlib/mirr_core/              # Standard library primitives in MIRR
├── examples/
│   └── neonatal_respirator.mirr
├── docs/
│   ├── roadmap.md                 # This file
│   ├── mirr_spec.md
│   └── ...
└── mcp_server/                    # TypeScript MCP stdio server
```

### Where to make changes

| Area | Files |
|---|---|
| Syntax / tokens | `src/lexer/*`, `src/parser/*` |
| AST / data model | `src/ast/*`; temporal back-refs: `Expr::Prev` in `expr.rs`; properties: `property.rs` |
| Semantic rules | `src/validation/semantic.rs` |
| Temporal lowering | `src/temporal/*` |
| Logic simplification | `src/simplify.rs` |
| Bit-width inference (acyclic) | `src/width/solver.rs`, `constraint.rs`, `flatten.rs` |
| Bit-width inference (cyclic SCC) | `src/width/graph.rs`, `scc.rs`, `scc_solver.rs`, `verify.rs` |
| Output emission | `src/emit/verilog.rs`, `dot.rs`, `json_netlist.rs`, `rspu.rs`, `firrtl.rs`, `fpga_scaffold.rs`, `fpga_target.rs`, `dsp.rs`, `testbench.rs` |
| R-SPU backend | `src/emit/rspu.rs`, `rspu_isa.rs`, `rspu_regalloc.rs` |
| FIRRTL emission | `src/emit/firrtl.rs` |
| FPGA targets | `src/emit/fpga_scaffold.rs`, `src/emit/fpga_target.rs` |
| DSP emission | `src/emit/dsp.rs` |
| Testbench generation | `src/emit/testbench.rs` |
| Type checking | `src/typeck/mod.rs` |
| Type checking (extended) | `src/typeck/extended.rs` |
| Unified pipeline | `src/pipeline.rs` |
| Safety properties / SVA | `src/ast/property.rs`, `src/emit/verilog.rs` (SVA), `src/validation/semantic.rs` |
| Pattern expansion | `src/ast/pattern.rs`, `src/parser/pattern_parser.rs`, `src/expand/mod.rs` |
| MAPE-K harness | `src/mape_k/`, `src/mirr_executor.rs` |
| Self-hosting pipeline | `src/bootstrap_runner.rs` |
| Rocq formal proofs | `proofs/width/*.v`, `proofs/width/SCC/*.v` |

---

## See Also

- [Tutorial](tutorial) — Learn the language from scratch
- [Error Codes](error_codes) — Error codes introduced per phase
- [Type System](type-system) — TYPE campaign results
- [R-SPU Reference](rspu-reference) — Phase 8 deliverable
- [Migration Guide](migration-guide) — Breaking changes per version
- [Glossary](glossary) — Project terminology and acronyms
- [Contributing](contributing) — Coding standards, campaign workflow, error allocation