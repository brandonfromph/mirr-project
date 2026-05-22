# ECS Migration Guide: From AST to Registry

This guide provides technical instructions for migrating legacy AST-based compiler passes to the modern **Entity Component System (ECS)** architecture.

## 1. The Core Philosophy

Legacy MIRR passes operated on tree-like AST structures (`MirrProgram -> Module -> Guard`). While intuitive, this approach is less efficient and harder to audit for safety-critical hardware.

**ECS-Native Synthesis** treats every hardware declaration as an **Entity** and its properties as **Components** stored in a Structure-of-Arrays (SoA) `Registry`.

## 2. Migration Steps

### Step 1: Component Identification
Identify which AST fields should become ECS components.
*   `Guard.condition` -> `ConditionComponent` (containing an `EntityId` to an expression tree).
*   `Guard.cycles` -> `CyclesComponent`.

### Step 2: Adaptive Factory Methods
Instead of methods taking AST nodes, implement factory methods that accept the `Registry` and an `EntityId`.

**Legacy:**
```rust
fn lower_guard(g: &Guard) -> Result<Compiled, Error>
```

**Modern:**
```rust
fn lower_guard_to_ecs(registry: &Registry, id: EntityId) -> Result<Compiled, Error>
```

### Step 3: Iterative Traversal (NASA P10)
Avoid recursion when traversing expression trees in the Registry. Use an explicit work-stack.

```rust
let mut stack = vec![id];
while let Some(current) = stack.pop() {
    // Process registry.binary_ops[current.0 as usize]
}
```

### Step 4: Feedback Loops
If your pass generates new metadata (e.g., hardware delay strategies), store it back in the Registry as a new component (e.g., `TemporalNodeComponent`).

## 3. Benefits of ECS-Native Paths
*   **Performance**: SoA layout is cache-friendly for large chips.
*   **Traceability**: Every IR node maps directly back to a Registry Entity.
*   **Auditability**: The Registry is a flat, easily inspectable source of truth.
