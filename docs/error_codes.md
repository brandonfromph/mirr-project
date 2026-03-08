---
title: Error Codes
nav_order: 2
---

# MIRR Error Code Reference

> **Status:** Active
> **Last updated:** 2026-03-08

All MIRR compiler diagnostics carry a bracketed error code in the format `[Ennn]`.
The prefix classifies the error; the full code maps to a single creation site.

## Error Code Scheme

| Prefix | Range   | Category              | Variant                     |
|--------|---------|-----------------------|-----------------------------|
| E1xx   | 100–199 | Parse / lexical       | `MirrError::ParseError`     |
| E2xx   | 200–299 | Semantic analysis     | `MirrError::SemanticError`  |
| E3xx   | 300–399 | Temporal compilation  | `MirrError::TemporalCompilationError` |
| E4xx   | 400–499 | Pattern expansion     | `MirrError::PatternError`   |
| E5xx   | 500–599 | Width inference        | `WidthDiag` (subsystem-local) |
| E6xx   | 600–699 | Type checking          | `MirrError::TypeError`      |
| E7xx   | 700–799 | R-SPU emission          | `MirrError::RspuError`      |

## Parse Errors (E1xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E100 | *(category prefix for all parse errors)* | `src/error.rs` |
| — | `MIRR source is empty.` | `src/parser/module_parser.rs` |
| — | `Expected 'module' declaration but found end of file.` | `src/parser/module_parser.rs` |
| — | `Expected 'module' declaration, found: {header}` | `src/parser/module_parser.rs` |
| — | `Module name cannot be empty.` | `src/parser/module_parser.rs` |
| — | `Module '{name}' was not closed with '}'.` | `src/parser/module_parser.rs` |
| — | `Unexpected line inside module '{name}': {line}` | `src/parser/module_parser.rs` |
| — | `Malformed signal declaration.` | `src/parser/module_parser.rs` |
| — | `Signal declaration must end with ';'.` | `src/parser/module_parser.rs` |
| — | `Signal declaration must contain ':'.` | `src/parser/module_parser.rs` |
| — | `Signal name cannot be empty.` | `src/parser/module_parser.rs` |
| — | `Signal kind (in/out/internal) is missing.` | `src/parser/module_parser.rs` |
| — | `Signal type (bool/uN) is missing.` | `src/parser/module_parser.rs` |
| — | `Unknown signal kind: {other}.` | `src/parser/module_parser.rs` |
| — | `Unknown signal type: {ty_str}.` | `src/parser/module_parser.rs` |
| — | `Guard name cannot be empty.` | `src/parser/module_parser.rs` |
| — | `Guard '{name}' missing 'when' clause.` | `src/parser/module_parser.rs` |
| — | `Guard '{name}' missing 'for' clause.` | `src/parser/module_parser.rs` |
| — | `Invalid cycle count in guard '{name}': {str}` | `src/parser/module_parser.rs` |
| — | `Reflex name cannot be empty.` | `src/parser/module_parser.rs` |
| — | `Reflex '{name}' missing 'on' clause.` | `src/parser/module_parser.rs` |
| — | `Reflex '{name}' has no guard names in 'on' clause.` | `src/parser/module_parser.rs` |
| — | `Property name cannot be empty.` | `src/parser/module_parser.rs` |
| — | `Property '{name}' missing formula (always/never).` | `src/parser/module_parser.rs` |
| — | `Property '{name}' formula must start with 'always' or 'never'.` | `src/parser/module_parser.rs` |
| — | `Empty expression.` | `src/parser/expr_parser.rs` |
| — | `Unbalanced parentheses in expression.` | `src/parser/expr_parser.rs` |
| — | `Expression depth exceeds limit of {N}.` | `src/parser/expr_parser.rs` |
| — | `Unexpected end of expression.` | `src/parser/expr_parser.rs` |
| — | `Integer literal too large: '{str}'.` | `src/lexer/tokenizer.rs` |
| — | `Unexpected character '{c}' in expression.` | `src/lexer/tokenizer.rs` |

## Semantic Errors (E2xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E201 | `[E201] Duplicate signal name: '{name}'.` | `src/validation/semantic.rs` |
| E202 | `[E202] Duplicate guard name: '{name}'.` | `src/validation/semantic.rs` |
| E203 | `[E203] Duplicate reflex name: '{name}'.` | `src/validation/semantic.rs` |
| E204 | `[E204] Guard '{name}' references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| E205 | `[E205] Reflex '{name}' references undeclared guard '{guard}'.` | `src/validation/semantic.rs` |
| E206 | `[E206] Reflex '{name}' assigns to input signal '{sig}', which is not writable.` | `src/validation/semantic.rs` |
| E207 | `[E207] Reflex '{name}' assigns to undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| E208 | `[E208] Reflex '{name}' assignment references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| E209 | `[E209] '{context}' contains prev('{sig}') with delay 0; delay must be >= 1.` | `src/validation/semantic.rs` |
| E210 | `[E210] Duplicate property name: '{name}'.` | `src/validation/semantic.rs` |
| E211 | `[E211] Property '{name}' references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| E212 | `[E212] signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` (hand-written reflex target) |
| E213 | `[E213] signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` (hand-written expression) |
| E214 | `[E214] signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` (cross-expansion target) |
| E215 | `[E215] signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` (cross-expansion expression) |
| E216 | `[E216] Signal '{sig}' has multiple writers: reflex '{r1}' and reflex '{r2}'.` | `src/validation/semantic.rs` |

## Temporal Errors (E3xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E300 | *(category prefix for all temporal errors)* | `src/error.rs` |
| — | `guard '{name}': condition cannot be lowered to hardware -- unsupported form` | `src/temporal/compiler.rs` |
| — | `guard '{name}': condition cannot be lowered to hardware -- {reason}` | `src/temporal/compiler.rs` |
| — | `JSON serialization failed: {e}` | `src/temporal/emit.rs` |

## Pattern Errors (E4xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E400 | *(category prefix for all pattern errors)* | `src/error.rs` |
| — | `Too many pattern definitions (max {N}).` | `src/parser/module_parser.rs` |
| — | `Duplicate pattern definition: '{name}'.` | `src/validation/semantic.rs` |
| — | `Pattern '{name}' has duplicate parameter name: '{param}'.` | `src/validation/semantic.rs` |
| — | `Pattern '{name}' has {N} parameters (max {M}).` | `src/validation/semantic.rs` |
| — | `Pattern '{name}' has empty reflect body.` | `src/validation/semantic.rs` |
| — | `Pattern call references undefined pattern '{name}'.` | `src/expand/mod.rs` |
| — | `Pattern '{name}' expects {N} arguments, got {M}.` | `src/expand/mod.rs` |
| — | *(plus ~25 additional parse-level pattern messages)* | `src/parser/pattern_parser.rs` |

## Removed Variants

The following variants were removed as dead code (never constructed):

- `LexicalError` — lexer errors are correctly classified under `ParseError` (E1xx).
- `TemporalCausalityViolation` — forward-declared for planned causality analysis; never implemented.

## Width Inference Errors (E5xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E500 | `expression tree exceeds maximum node count (512)` | `src/width/mod.rs` |
| E501 | `signal '{name}' has no declared width` | `src/width/constraint.rs` |
| E502 | `prev signal '{name}' has no declared width` | `src/width/constraint.rs` |
| E503 | `node {id} ({desc}) has unresolved width` | `src/width/solver.rs` |
| E504 | `node {id} ({desc}) requires {n} bits, exceeding maximum of 64` | `src/width/solver.rs` |
| E505 | `assignment to '{target}' truncates {signed\|unsigned} {n} bits to {m} bits` | `src/width/solver.rs` |
| E506 | `SCC detection exceeded iteration budget` | `src/width/scc.rs` |
| E507 | `SCC with {n} signals exceeds maximum size of {max}; signals include: {names}` | `src/width/scc.rs` |
| E508 | `nonexpansive SCC solver exceeded iteration budget` | `src/width/scc_solver.rs` |
| E509 | `signal '{name}' in nonexpansive SCC has no width anchor (add an explicit type annotation)` | `src/width/scc_solver.rs` |
| E510 | `signal '{name}' is in an expansive SCC but has no provable width bound. Add an explicit type annotation or a bounded temporal guard.` | `src/width/scc_solver.rs` |
| E511 | `COMPILER BUG: signal '{name}' solved width {i\|u}{n} is less than declared {i\|u}{m}` | `src/width/verify.rs` |

## Type Errors (E6xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E601 | `[E601] Guard '{name}' condition must be bool, got {ty}.` | `src/typeck/mod.rs` |
| E602 | `[E602] Assignment to '{target}' ({target_ty}): expression type {expr_ty} is not compatible.` | `src/typeck/mod.rs` |
| E603 | `[E603] Operator '{op}' requires numeric operands, got {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E603 | `[E603] Operator '{op}' cannot mix signed and unsigned operands: {left} and {right}.` | `src/typeck/mod.rs` |
| E603 | `[E603] Operator '-' (negate) cannot be applied to bool.` | `src/typeck/mod.rs` |
| E604 | `[E604] Operator '{op}' requires bool operands, got {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E605 | `[E605] Ordering operator '{op}' cannot compare {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E606 | `[E606] Equality operator '{op}' cannot compare {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E607 | `[E607] Operator '^' (xor) requires matching types, got {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |

**Note (TYPE-002):** E603/E605/E606/E607 also trigger for cross-category signed/unsigned
operations. Signed types (`i1`–`i64`) participate in the same error code scheme as unsigned
types (`u1`–`u64`). No implicit signed↔unsigned conversion is allowed.

## R-SPU Emission Errors (E7xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E700 | *(category prefix for all R-SPU errors)* | `src/error.rs` |
| E701 | `[E701] R-SPU register allocation failed: too many {kind} signals ({count} > {max}).` | `src/emit/rspu_regalloc.rs` |
| E701 | `[E701] R-SPU temporary registers exhausted.` | `src/emit/rspu.rs` |
| E702 | `[E702] R-SPU instruction budget exceeded: {count} instructions > {max}.` | `src/emit/rspu.rs` |
| E702 | `[E702] R-SPU expression exceeds maximum node count.` | `src/emit/rspu.rs` |
| E703 | `[E703] R-SPU guard resource exhausted: {count} guards > {max}.` | `src/emit/rspu.rs` |

---

## See Also

- [Tutorial](tutorial) — Lesson 9: Common errors
- [Type System](type-system) — Type rules behind E6xx errors
- [R-SPU Reference](rspu-reference) — Resource limits behind E7xx errors
- [Migration Guide](migration-guide) — New error codes added per version
