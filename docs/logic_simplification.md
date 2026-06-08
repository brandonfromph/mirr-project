# MIRR Logic Simplification & Constant Folding

## Overview

The simplification engine reduces boolean, comparison, and arithmetic expressions in MIRR before temporal lowering and hardware mapping. 

Following the Phase 3 ECS Migration, MIRR now operates two distinct simplification engines:
1. **ECS Parallel Constant Folding:** The modern production engine running natively over the flat Entity Component System arrays.
2. **AST Iterative Simplifier:** The legacy bootstrap engine (`simplify_expr()`) that processes the Abstract Syntax Tree.

---

## 1. ECS Parallel Constant Folding (Phase 3 Native)

The primary simplifier runs natively inside the ECS `Registry`. Because the ECS stores expressions as flat contiguous arrays (Data-Oriented Design), the simplification pass uses **Rayon** for blazing-fast multi-threaded constant folding.

### Architecture
Located in `src/ecs/systems.rs`:
```rust
pub fn parallel_constant_folding_system(registry: &mut Registry)
```

- **Execution:** Runs as an ECS System over all entities bounded by `registry.next_id`.
- **Parallelism:** Uses `rayon::prelude::*` to iterate over `binary_ops` and `literals` component arrays concurrently.
- **In-Place Mutation:** Directly transforms `BinaryComponent` entities into `LiteralComponent` entities when both children resolve to constants.
- **NASA P10 Compliance:** Strict linear iteration bounded by active entity count. No recursive descent or unbounded stack usage.

---

## 2. AST Iterative Simplification (Legacy / Bootstrap)

The original Phase 2 simplifier operates directly on the `Expr` tree. It remains in the codebase primarily for the self-hosting bootstrap runner and CLI analysis (`mirr-simplify`).

### Architecture
- **Engine:** Iterative post-order traversal with an explicit stack (`Vec<WorkItem>`).
- **Depth Bound:** `MAX_SIMPLIFY_DEPTH = 128` (prevents stack overflow).
- **Fixpoint:** Repeats until no rules fire, bounded by `MAX_PASSES = 8`.
- **Entry points:**
  - `simplify_expr(expr) -> Expr`
  - `simplify_expr_with_stats(expr) -> (Expr, SimplifyStats)`

---

## 3. Supported Rules

Both the ECS and AST simplifiers execute the following formal rules:

### Boolean Identity / Annihilation (8 rules)
| Rule | Input | Output |
|------|-------|--------|
| AND identity | `X && true` | `X` |
| AND annihilation | `X && false` | `false` |
| OR identity | `X \|\| false` | `X` |
| OR annihilation | `X \|\| true` | `true` |
| XOR identity | `X ^ false` | `X` |
| XOR complement | `X ^ true` | `!X` |
| Double negation | `!!X` | `X` |
| Literal negation | `!true` / `!false` | `false` / `true` |

### Boolean Idempotence / Absorption (5 rules)
| Rule | Input | Output |
|------|-------|--------|
| AND idempotent | `a && a` | `a` |
| OR idempotent | `a \|\| a` | `a` |
| XOR self-cancel | `a ^ a` | `false` |
| Contradiction | `a && !a` | `false` |
| Tautology | `a \|\| !a` | `true` |

### Comparison Constant Folding (6 rules)
When both operands are integer literals, all comparison operators (`<`, `<=`, `>`, `>=`, `==`, `!=`) are folded to boolean constants.

### Arithmetic Identity / Annihilation (9 rules)
| Rule | Input | Output |
|------|-------|--------|
| Add identity | `x + 0`, `0 + x` | `x` |
| Sub identity | `x - 0` | `x` |
| Mul identity | `x * 1`, `1 * x` | `x` |
| Mul annihilation | `x * 0`, `0 * x` | `0` |
| Shift identity | `x << 0`, `x >> 0` | `x` |

### Arithmetic Constant Folding (5 rules)
When both operands are integer literals, `+`, `-`, `*`, `<<`, `>>` are folded at compile time. Shift amounts are strictly clamped to 63 to prevent undefined behavior in hardware.

---

## 4. CLI Tool

```bash
mirr-simplify <expr.json | program.mirr> [--stats]
```

- `.json` mode: Simplify a bare `Expr` JSON and print the result.
- `.mirr` mode: Parse the program, simplify all guard conditions, and print gate count statistics.

## 5. Extensibility Roadmap

The flat ECS data layout allows for future high-performance integration of:
- GPU-accelerated massive SIMD parallel evaluation
- Dead-gate elimination passes across component arrays
- SAT-solver extraction for automated equivalence checking
