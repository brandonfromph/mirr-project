# Shift-Left QA: The Parity Testing Pattern

The MIRR project employs a "Shift-Left QA" strategy to ensure that architectural refactorings—such as the transition from AST-based synthesis to ECS-native synthesis—never introduce regressions.

## 1. The Parity Testing Pattern

Parity testing is a methodology where two different implementation paths (the "Legacy" path and the "Modern" path) are executed on the same input, and their outputs are compared for bit-for-bit or semantic identity.

### Why Use It?
*   **Verification of Correctness**: Ensures the new architecture produces identical hardware to the battle-tested old one.
*   **Zero-Debt Invariant**: Prevents "invisible" regressions during architectural migrations.
*   **Safety-Critical Compliance**: Provides a formal record of equivalence for safety audits.

## 2. Implementation in MIRR

In the temporal synthesis pass, we implemented this in `tests/temporal_parity_tests.rs`.

### The Workflow:
1.  **Define Input**: Create a complex hardware scenario (e.g., a compound AND/OR guard with high cycle count).
2.  **Path A (Legacy)**:
    *   Reify the ECS entity back into a legacy AST `Guard`.
    *   Run `TemporalCompiler::compile_module` (the AST path).
3.  **Path B (Modern)**:
    *   Run `TemporalCompiler::lower_guard_to_ecs` (the direct ECS path).
4.  **Verification**:
    *   Compare the resulting `CompiledGuard` structures.
    *   Ensure generated signals, delay logic, and internal naming are identical.

## 3. Global Regression

This pattern is part of the global `nextest` suite. Any change to the synthesis logic must pass the parity test before being committed. 

> [!TIP]
> When implementing a new compiler pass, always start by creating a parity test against the previous stable baseline. This "seam" becomes your safety net.
