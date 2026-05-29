# PROPOSAL 111: RSPU ARCHITECTURE UPGRADE & MASSIVE COMPILER STRESS-TEST PIPELINE

**Proposal #:** 111
**Campaign ID:** RSPU-UPGRADE-STRESS-TEST
**Status:** PROPOSED — AWAITING TDD BOOTSTRAP
**Scope Class:** INTEGRATION (RSPU Core + ECS Composite Realization + Open-Source RTL Simulation)
**Date:** 2026-05-23

---

## Executive Summary

The current RSPU multi-core module represents a stable baseline, but it has not yet been subjected to massive memory and scale stress testing using our modern, hand-rolled Structure-of-Arrays (SoA) ECS expression system. Furthermore, to validate safety-critical compilation, we must transition the RSPU project to a strict Test-Driven Development (TDD) harness using industry-standard open-source EDA tools.

This proposal upgrades the RSPU multi-core design to leverage composite `ArrayLiteral` and `StructLiteral` elements for core-to-core interconnects, and establishes a massive TDD stress test that:
1. **Floods the ECS Registry** with over `100,000` flattened signal, guard, and assignment entities to validate cache stability and resize performance.
2. **Performs complete multi-file compilation** of the 16-core RSPU interconnect crossbar and ALU.
3. **Validates hardware clock realization** of unrolled structural loops.
4. **Triggers open-source RTL compilation & simulation** using Icarus Verilog (`iverilog` / `vvp`) directly inside the integration test suite.

---

## Architectural Mandates & Philosophy

1. **Test-Driven Development (TDD)**: All compilation improvements, type-checking constraints, and unrolled logic must be driven by strict test suite gates.
2. **NASA JPL Power-of-10 Compliance**:
   - Zero dynamic allocations after workspace initialization.
   - Strictly bounded expression depth limit checks (`MAX_EXPR_NODES = 512`).
   - No recursion during verification traversals.
3. **Zero Deletions**: Existing RSPU reference modules (such as `rspu_top.mirr`, `alu.mirr`, and `ram.mirr`) are extended and upgraded, never deleted.

---

## Proposed Changes

```mermaid
graph TD
    RSPU_Top["RSPU 16-Core Top (rspu_top.mirr)"] --> Core_ALU["Core ALU (core/alu.mirr)"]
    RSPU_Top --> Core_RAM["Core RAM (core/ram.mirr)"]
    RSPU_Top --> Crossbar["Interconnect (interconnect/crossbar.mirr)"]
    
    SubGraph_ECS["ECS Compiler Ingestion & Lowering"]
        Registry_Ingest["SoA Registry Ingestion"]
        Semantic_Val["Semantic Validation (Iterations & Depth limits)"]
        Typecheck["Typechecking & Domain refinement checks"]
    end

    SubGraph_Simulation["Verification Staging"]
        Verilog_Emit["Verilog Emitter (emit/verilog)"]
        Iverilog["Icarus Verilog Compiler (iverilog -g2012)"]
        VVP["RTL Parity Simulation (vvp)"]
    end

    RSPU_Top --> Registry_Ingest
    Registry_Ingest --> Semantic_Val
    Semantic_Val --> Typecheck
    Typecheck --> Verilog_Emit
    Verilog_Emit --> Iverilog
    Iverilog --> VVP
```

### 1. Hardware Module Upgrades (RSPU Core & Interconnect)
- **`rspu_chip/core/alu.mirr`**: Upgrade to support multi-issue vector operations by packaging inputs and outputs into flat `StructLiteral` definitions and array signals.
- **`rspu_chip/interconnect/crossbar.mirr`**: Refactor port indexing and priority selectors to use `UnfoldIndex` loops, allowing the compiler to unroll parallel 16-channel routing networks cleanly.

### 2. The TDD Stress-Test Harness
We will introduce a giant TDD integration test suite:
- **`tests/rspu_massive_stress_tests.rs`**:
  - Dynamically synthesizes a high-scale 16-core unrolled processor configuration inside the ECS `Registry`.
  - Asserts that `next_id()` and option vectors resize efficiently to $\ge 100,000$ entities without desynchronization.
  - Verifies that compilation completes with zero type checking or bounds violations.
  - Emits the completed design to SystemVerilog, compiles it using `iverilog`, lints the generated files, and simulates them with `vvp` to prove physical parity.

---

## Verification & Execution Plan

### Automated Test Gates
We will run the TDD stress test target:
```sh
cargo nextest run --test rspu_massive_stress_tests
```

### Validation Bounds
- **Registry Entity Count**: $\ge 100,000$ flattened components.
- **Expression Complexity**: Maximum expression tree depth $\le 512$ nodes (`MAX_EXPR_NODES`).
- **External Compilation Exit Status**: Zero warnings or compile-time failures from `iverilog`.
