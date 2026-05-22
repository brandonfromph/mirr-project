# ECS Temporal Synthesis Stress Test Plan

**Goal**: Verify the robustness, safety, and performance of the ECS-native `TemporalCompiler::lower_guard_to_ecs` implementation under extreme conditions.

## 1. Safety & Bounds Verification (NASA P10 Compliance)
*   **Test: Deep Nesting Limit**
    *   **Input**: A compound guard with 65 nested AND operations.
    *   **Expectation**: Returns `Err` with "exceeds maximum nesting depth" ([E301]).
    *   **Purpose**: Verify `MAX_COMPILE_GUARD_DEPTH` enforcement.
*   **Test: Iteration Bound**
    *   **Input**: A pathological condition tree that might cause infinite loops if the stack logic is flawed.
    *   **Expectation**: Termination within `MAX_COMPILE_GUARD_DEPTH * 4` iterations.

## 2. Adaptive Strategy Verification
*   **Test: Threshold Boundary (ShiftRegister -> Counter)**
    *   **Scenario A**: Guard with 16 cycles. Expect `CompiledGuard::ShiftRegister`.
    *   **Scenario B**: Guard with 17 cycles. Expect `CompiledGuard::Counter`.
    *   **Scenario C**: Guard with `prev(5)` and 12 cycles (total 17). Expect `CompiledGuard::Counter`.

## 3. Robustness (Negative Testing)
*   **Test: Missing Name Component**
    *   **Expectation**: Error "missing NameComponent".
*   **Test: Missing Cycles Component**
    *   **Expectation**: Error "missing CyclesComponent".
*   **Test: Missing Condition Component**
    *   **Expectation**: Error "missing ConditionComponent".
*   **Test: Dangling Entity Reference**
    *   **Input**: `ConditionComponent` points to an entity that has no `signal_refs`, `prev_ops`, `unary_ops`, or `binary_ops`.
    *   **Expectation**: Error "unsupported condition expression form".

## 4. Scale & Performance
*   **Test: Large Flat Guard**
    *   **Input**: A guard with 1000 sub-conditions (if depth allowed, otherwise large breadth).
    *   **Expectation**: Successful synthesis within acceptable time (< 100ms).
*   **Test: Massive Registry**
    *   **Input**: 5000 independent guards in one Registry.
    *   **Expectation**: Successful bulk synthesis via `temporal_synthesis_system`.

## 5. Circular Reference Detection
*   **Input**: A Registry where Entity A's binary op references Entity B, and Entity B's binary op references Entity A.
*   **Expectation**: Error (likely depth limit exceeded or explicit cycle detection).
