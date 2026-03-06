# MIRR Logic Simplification: Documentation

## Overview
This module provides logic simplification for boolean expressions in MIRR’s IR. It recursively applies algebraic rules to Expr trees, reducing logic complexity before hardware mapping.

## Supported Simplification Rules
- X & true = X
- X & false = false
- X | false = X
- X | true = true
- X ^ false = X
- X ^ true = !X
- !!X = X
- !true = false, !false = true

## Usage
- Use `simplify_expr(expr)` to simplify any Expr tree.
- The CLI tool `mirr-simplify` reads an Expr from JSON, simplifies it, and prints the result.

## Extensibility
- The design allows for future integration of SAT solvers or DAG-based optimizations.

## Testing
- See `tests/simplify_tests.rs` for unit tests covering all core rules.

---

This logic simplification is a key EDA step, reducing gate count and improving circuit efficiency before hardware synthesis.
