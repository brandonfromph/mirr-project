# MIRR Logic Simplification (Phase 3)

## Overview

The simplification engine reduces boolean, comparison, and arithmetic expressions
in MIRR's AST before temporal lowering and hardware mapping. It uses a bounded
iterative post-order traversal (NASA Power-of-10 rule #1 compliant -- no unbounded
recursion) and runs to fixpoint to catch cascading reductions.

## Architecture

```
                simplify_expr()
.mirr source ──> parse ──> AST ────────────────> temporal lowering ──> netlist
                                  ^                    |
                  Phase 3 integration wired in         v
                  src/temporal/mod.rs            Verilog / JSON / DOT
```

The simplifier is wired into the temporal compilation pipeline: guard conditions
are simplified before `ConditionKind::try_from_expr` classifies them. This allows
expressions like `sensor && true` to reduce to `sensor`, enabling correct
`SimpleSignal` classification instead of unnecessary `ComplexGuard` wrapping.

## Implementation

- **Engine:** Iterative post-order traversal with explicit stack (`Vec<WorkItem>`)
- **Depth bound:** `MAX_SIMPLIFY_DEPTH = 128` (matches parser's `MAX_EXPR_DEPTH`)
- **Fixpoint:** Repeats until no rules fire, bounded by `MAX_PASSES = 8`
- **Entry points:**
  - `simplify_expr(expr) -> Expr` -- backward-compatible, returns simplified tree
  - `simplify_expr_with_stats(expr) -> (Expr, SimplifyStats)` -- includes statistics

## Supported Rules

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
When both operands are integer literals, all comparison operators (`<`, `<=`,
`>`, `>=`, `==`, `!=`) are folded to boolean constants.

### Arithmetic Identity / Annihilation (9 rules)
| Rule | Input | Output |
|------|-------|--------|
| Add identity | `x + 0`, `0 + x` | `x` |
| Sub identity | `x - 0` | `x` |
| Mul identity | `x * 1`, `1 * x` | `x` |
| Mul annihilation | `x * 0`, `0 * x` | `0` |
| Shift identity | `x << 0`, `x >> 0` | `x` |

### Arithmetic Constant Folding (5 rules)
When both operands are integer literals, `+`, `-`, `*`, `<<`, `>>` are folded
at compile time. Uses `wrapping_*` semantics matching `eval_expr` in the executor.
Shift amounts clamped to 63 (matching CRIT-01 hardware safety fix).

## CLI Tool

```
mirr-simplify <expr.json | program.mirr> [--stats]
```

- `.json` mode: simplify a bare `Expr` JSON and print the result
- `.mirr` mode: parse the program, simplify all guard conditions and reflex
  assignment RHS expressions, print a summary with gate count statistics

## Statistics

`SimplifyStats` reports:
- `rules_applied` -- total number of rule firings across all fixpoint passes
- `nodes_before` -- AST node count before simplification
- `nodes_after` -- AST node count after simplification

## Testing

58 tests in `tests/simplify_tests.rs` covering:
- All boolean identity/annihilation rules (both operand positions)
- All idempotence/absorption rules
- All comparison constant folds
- All arithmetic identity/annihilation rules
- Arithmetic constant folding (including wrapping and shift clamping)
- Cascading/nested simplification
- Fixpoint idempotence
- Base case passthrough (signals, literals)
- Stats API correctness
- Integration: temporal pipeline simplifies guard conditions before lowering

## Extensibility

The design allows for future integration of:
- SAT solvers for equivalence checking
- DAG-based common sub-expression elimination
- Dead-gate elimination in post-lowering netlist
