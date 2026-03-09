---
title: Error Codes
nav_order: 2
---

# MIRR Error Code Reference

> **Status:** Active
> **Last updated:** 2026-03-09 (ERR-001 campaign)

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

## Parse Errors — Module (E100–E166)

| Code | Message pattern | Source |
|------|----------------|--------|
| E100 | *(category fallback prefix)* | `src/error.rs` |
| E101 | `MIRR source is empty.` | `src/parser/module_parser.rs` |
| E102 | `Expected 'module' declaration but found end of file.` | `src/parser/module_parser.rs` |
| E103 | `Expected 'module' declaration, found: {header}` | `src/parser/module_parser.rs` |
| E104 | `Malformed module declaration.` | `src/parser/module_parser.rs` |
| E105 | `Module name cannot be empty.` | `src/parser/module_parser.rs` |
| E106 | `Module '{name}' was not closed with '}'.` | `src/parser/module_parser.rs` |
| E107 | `Unexpected line inside module '{name}': {line}` | `src/parser/module_parser.rs` |
| E108 | `Malformed signal declaration.` | `src/parser/module_parser.rs` |
| E109 | `Signal declaration must end with ';'.` | `src/parser/module_parser.rs` |
| E110 | `Signal declaration must contain ':'.` | `src/parser/module_parser.rs` |
| E111 | `Signal name cannot be empty.` | `src/parser/module_parser.rs` |
| E112 | `Signal kind (in/out/internal) is missing.` | `src/parser/module_parser.rs` |
| E113 | `Signal type (bool/uN) is missing.` | `src/parser/module_parser.rs` |
| E114 | `Too many tokens in signal declaration.` | `src/parser/module_parser.rs` |
| E115 | `Unknown signal kind: {other}. Expected 'in', 'out', or 'internal'.` | `src/parser/module_parser.rs` |
| E116 | `Invalid unsigned width in type '{ty_str}'.` | `src/parser/module_parser.rs` |
| E117 | `Invalid signed width in type '{ty_str}'.` | `src/parser/module_parser.rs` |
| E118 | `Unknown signal type: {ty_str}. Expected 'bool', 'uN', or 'iN'.` | `src/parser/module_parser.rs` |
| E119 | `Unexpected end of file in guard declaration.` | `src/parser/module_parser.rs` |
| E120 | `Malformed guard declaration.` | `src/parser/module_parser.rs` |
| E121 | `Guard name cannot be empty.` | `src/parser/module_parser.rs` |
| E122 | `Guard '{name}' missing 'when' clause.` | `src/parser/module_parser.rs` |
| E123 | `Guard '{name}' expected 'when' line, found: {line}` | `src/parser/module_parser.rs` |
| E124 | `Malformed 'when' line.` | `src/parser/module_parser.rs` |
| E125 | `Guard '{name}' condition parse error: {e}` | `src/parser/module_parser.rs` |
| E126 | `Guard '{name}' missing 'for' clause.` | `src/parser/module_parser.rs` |
| E127 | `Guard '{name}' expected 'for' line, found: {line}` | `src/parser/module_parser.rs` |
| E128 | `Malformed 'for' line.` | `src/parser/module_parser.rs` |
| E129 | `Expected cycle count after 'for'.` | `src/parser/module_parser.rs` |
| E130 | `Invalid cycle count in guard '{name}': {str}` | `src/parser/module_parser.rs` |
| E131 | `Guard '{name}' not closed with '}'.` | `src/parser/module_parser.rs` |
| E132 | `Guard '{name}' expected closing '}', found: {line}` | `src/parser/module_parser.rs` |
| E133 | `Assignment missing '=': {line}` | `src/parser/module_parser.rs` |
| E134 | `Assignment target cannot be empty.` | `src/parser/module_parser.rs` |
| E135 | `Assignment to '{target}' has empty right-hand side.` | `src/parser/module_parser.rs` |
| E136 | `Error in assignment to '{target}': {e}` | `src/parser/module_parser.rs` |
| E137 | `Unexpected end of file in reflex declaration.` | `src/parser/module_parser.rs` |
| E138 | `Malformed reflex declaration.` | `src/parser/module_parser.rs` |
| E139 | `Reflex name cannot be empty.` | `src/parser/module_parser.rs` |
| E140 | `Reflex '{name}' missing 'on' clause.` | `src/parser/module_parser.rs` |
| E141 | `Reflex '{name}' expected 'on' line, found: {line}` | `src/parser/module_parser.rs` |
| E142 | `Malformed 'on' line.` | `src/parser/module_parser.rs` |
| E143 | `Reflex '{name}' has no guard names in 'on' clause.` | `src/parser/module_parser.rs` |
| E144 | `In reflex '{name}': {e}` | `src/parser/module_parser.rs` |
| E145 | `Reflex '{name}' not closed with '}'.` | `src/parser/module_parser.rs` |
| E146 | `Reflex '{name}' expected closing '}', found: {line}` | `src/parser/module_parser.rs` |
| E147 | `Unexpected end of file in property declaration.` | `src/parser/module_parser.rs` |
| E148 | `Malformed property declaration.` | `src/parser/module_parser.rs` |
| E149 | `Property name cannot be empty.` | `src/parser/module_parser.rs` |
| E150 | `Property '{name}' missing formula (always/never).` | `src/parser/module_parser.rs` |
| E151 | `Property '{name}' not closed with '}'.` | `src/parser/module_parser.rs` |
| E152 | `Property '{name}' expected closing '}', found: {line}` | `src/parser/module_parser.rs` |
| E153 | `Property '{name}' formula must start with 'always', 'never', or 'eventually'.` | `src/parser/module_parser.rs` |
| E154 | `Property '{name}': {keyword} formula must be wrapped in parentheses.` | `src/parser/module_parser.rs` |
| E155 | `Property '{name}' antecedent error: {e}` | `src/parser/module_parser.rs` |
| E156 | `Property '{name}' consequent error: {e}` | `src/parser/module_parser.rs` |
| E157 | `Property '{name}' formula error: {e}` | `src/parser/module_parser.rs` |
| E158 | `Property '{name}': expected 'eventually within N (expr)'.` | `src/parser/module_parser.rs` |
| E159 | `Property '{name}': eventually within requires parenthesized expression.` | `src/parser/module_parser.rs` |
| E160 | `Property '{name}': invalid cycle count '{str}' in eventually within.` | `src/parser/module_parser.rs` |
| E161 | `Property '{name}': eventually within requires cycles >= 1.` | `src/parser/module_parser.rs` |
| E162 | `Property '{name}': expected 'P followed_by N Q' with delay and response expression.` | `src/parser/module_parser.rs` |
| E163 | `Property '{name}': invalid delay '{str}' in followed_by.` | `src/parser/module_parser.rs` |
| E164 | `Property '{name}': followed_by requires delay >= 1.` | `src/parser/module_parser.rs` |
| E165 | `Property '{name}' trigger error: {e}` | `src/parser/module_parser.rs` |
| E166 | `Property '{name}' response error: {e}` | `src/parser/module_parser.rs` |

## Parse Errors — Expressions (E170–E181)

| Code | Message pattern | Source |
|------|----------------|--------|
| E170 | `Empty expression.` | `src/parser/expr_parser.rs` |
| E171 | `Unbalanced parentheses in expression.` | `src/parser/expr_parser.rs` |
| E172 | `Expression depth exceeds limit of {N}.` | `src/parser/expr_parser.rs` |
| E173 | `Unexpected end of expression.` | `src/parser/expr_parser.rs` |
| E174 | `Unexpected token at start of expression: {token}` | `src/parser/expr_parser.rs` |
| E175 | `Expected closing ')' in expression.` | `src/parser/expr_parser.rs` |
| E176 | `Unexpected token in expression: {token}` | `src/parser/expr_parser.rs` |
| E180 | `Integer literal too large: '{str}'.` | `src/lexer/tokenizer.rs` |
| E181 | `Unexpected character '{c}' in expression.` | `src/lexer/tokenizer.rs` |

## Semantic Errors (E2xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E201 | `Duplicate signal name: '{name}'.` | `src/validation/semantic.rs` |
| E202 | `Duplicate guard name: '{name}'.` | `src/validation/semantic.rs` |
| E203 | `Duplicate reflex name: '{name}'.` | `src/validation/semantic.rs` |
| E204 | `Guard '{name}' references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| E205 | `Reflex '{name}' references undeclared guard '{guard}'.` | `src/validation/semantic.rs` |
| E206 | `Reflex '{name}' assigns to input signal '{sig}', which is not writable.` | `src/validation/semantic.rs` |
| E207 | `Reflex '{name}' assigns to undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| E208 | `Reflex '{name}' assignment references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| E209 | `'{context}' contains prev('{sig}') with delay 0; delay must be >= 1.` | `src/validation/semantic.rs` |
| E210 | `Duplicate property name: '{name}'.` | `src/validation/semantic.rs` |
| E211 | `Property '{name}' references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| E212 | `signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` (hand-written reflex target) |
| E213 | `signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` (hand-written expression) |
| E214 | `signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` (cross-expansion target) |
| E215 | `signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` (cross-expansion expression) |
| E216 | `Signal '{sig}' has multiple writers: reflex '{r1}' and reflex '{r2}'.` | `src/validation/semantic.rs` |

**ERR-001 enhancements:** E204, E205, E207, E208, E211 now include "Did you mean '{closest}'?" when a close match exists. E201, E202, E203, E210, E216 now include "First defined at line N." when a span is available.

## Temporal Errors (E3xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E300 | *(category fallback prefix)* | `src/error.rs` |
| — | `guard '{name}': condition cannot be lowered to hardware -- unsupported form` | `src/temporal/compiler.rs` |
| — | `guard '{name}': condition cannot be lowered to hardware -- {reason}` | `src/temporal/compiler.rs` |
| — | `JSON serialization failed: {e}` | `src/temporal/emit.rs` |

## Pattern Errors (E4xx)

### Semantic-level (E400)

| Code | Message pattern | Source |
|------|----------------|--------|
| E400 | `Too many pattern definitions (max {N}).` | `src/parser/module_parser.rs` |
| — | `Duplicate pattern definition: '{name}'.` | `src/validation/semantic.rs` |
| — | `Pattern '{name}' has duplicate parameter name: '{param}'.` | `src/validation/semantic.rs` |
| — | `Pattern '{name}' has {N} parameters (max {M}).` | `src/validation/semantic.rs` |
| — | `Pattern '{name}' has empty reflect body.` | `src/validation/semantic.rs` |
| — | `Pattern call references undefined pattern '{name}'.` | `src/expand/mod.rs` |
| — | `Pattern '{name}' expects {N} arguments, got {M}.` | `src/expand/mod.rs` |

### Parse-level (E401–E425)

| Code | Message pattern | Source |
|------|----------------|--------|
| E401 | `Unexpected end of file in pattern definition.` | `src/parser/pattern_parser.rs` |
| E402 | `Malformed pattern definition.` | `src/parser/pattern_parser.rs` |
| E403 | `Pattern definition missing '('.` | `src/parser/pattern_parser.rs` |
| E404 | `Pattern name cannot be empty.` | `src/parser/pattern_parser.rs` |
| E405 | `Pattern '{name}' missing closing ')'.` | `src/parser/pattern_parser.rs` |
| E406 | `Pattern '{name}' missing 'reflect' block.` | `src/parser/pattern_parser.rs` |
| E407 | `Pattern '{name}' expected 'reflect' block, found: {line}` | `src/parser/pattern_parser.rs` |
| E408 | `Pattern '{name}' reflect block missing opening '{'.` | `src/parser/pattern_parser.rs` |
| E409 | `Pattern definition header not closed with ') {'.` | `src/parser/pattern_parser.rs` |
| E410 | `Pattern '{name}' has too many parameters (max {N}).` | `src/parser/pattern_parser.rs` |
| E411 | `Pattern '{name}' parameter missing ':': {str}` | `src/parser/pattern_parser.rs` |
| E412 | `Pattern '{name}' has parameter with empty name.` | `src/parser/pattern_parser.rs` |
| E413 | `Pattern '{name}' signal parameter '{param}' missing direction.` | `src/parser/pattern_parser.rs` |
| E414 | `Pattern '{name}' parameter '{param}': unknown signal kind '{kind}'.` | `src/parser/pattern_parser.rs` |
| E415 | `Pattern '{name}' signal parameter '{param}' missing type.` | `src/parser/pattern_parser.rs` |
| E416 | `Pattern '{name}' parameter '{param}': invalid type '{ty}'.` | `src/parser/pattern_parser.rs` |
| E417 | `Pattern '{name}' parameter '{param}': unknown type '{ty}'. Expected 'bool', 'uN', or 'iN'.` | `src/parser/pattern_parser.rs` |
| E418 | `Pattern '{name}' reflect body exceeds maximum brace depth ({N}).` | `src/parser/pattern_parser.rs` |
| E419 | `Pattern '{name}' reflect block not closed with '}'.` | `src/parser/pattern_parser.rs` |
| E420 | `Pattern call must end with ';'.` | `src/parser/pattern_parser.rs` |
| E421 | `Pattern call missing '('.` | `src/parser/pattern_parser.rs` |
| E422 | `Pattern call has empty name.` | `src/parser/pattern_parser.rs` |
| E423 | `Pattern call '{name}' missing closing ')'.` | `src/parser/pattern_parser.rs` |
| E424 | `Pattern call '{name}' has too many arguments (max {N}).` | `src/parser/pattern_parser.rs` |
| E425 | `Pattern call '{name}' has empty argument.` | `src/parser/pattern_parser.rs` |

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
| E510 | `signal '{name}' is in an expansive SCC but has no provable width bound.` | `src/width/scc_solver.rs` |
| E511 | `COMPILER BUG: signal '{name}' solved width {i\|u}{n} is less than declared {i\|u}{m}` | `src/width/verify.rs` |

{: .important }
> Error E511 indicates a compiler bug -- the solved width is narrower than
> the declared type. If you encounter this error, please file a bug report
> with the `.mirr` source that triggered it.

**ERR-001 enhancements:** All WidthDiag instances now carry `.code`, `.signal_name`, and `.help` fields for structured diagnostic rendering.

## Type Errors (E6xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E601 | `Guard '{name}' condition must be bool, got {ty}.` | `src/typeck/mod.rs` |
| E602 | `Assignment to '{target}' ({target_ty}): expression type {expr_ty} is not compatible.` | `src/typeck/mod.rs` |
| E603 | `Operator '{op}' requires numeric operands, got {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E604 | `Operator '{op}' requires bool operands, got {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E605 | `Ordering operator '{op}' cannot compare {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E606 | `Equality operator '{op}' cannot compare {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E607 | `Operator '^' (xor) requires matching types, got {left_ty} and {right_ty}.` | `src/typeck/mod.rs` |
| E608 | `Operator '{op}' cannot mix signed and unsigned operands: {left} and {right}.` | `src/typeck/mod.rs` |
| E609 | `Operator '-' (negate) cannot be applied to bool.` | `src/typeck/mod.rs` |

**ERR-001 change:** E603 was previously overloaded for three distinct error conditions. E608 (mixed signedness) and E609 (negate bool) were split out as unique codes.

## R-SPU Emission Errors (E7xx)

| Code | Message pattern | Source |
|------|----------------|--------|
| E700 | *(category fallback prefix)* | `src/error.rs` |
| E701 | `R-SPU register allocation failed: too many {kind} signals ({count} > {max}).` | `src/emit/rspu_regalloc.rs` |
| E702 | `R-SPU instruction budget exceeded: {count} instructions > {max}.` | `src/emit/rspu.rs` |
| E703 | `R-SPU guard resource exhausted: {count} guards > {max}.` | `src/emit/rspu.rs` |
| E704 | `R-SPU expression exceeds maximum node count.` | `src/emit/rspu.rs` |
| E705 | `R-SPU temporary registers exhausted.` | `src/emit/rspu.rs` |

**ERR-001 changes:** E701 was previously overloaded — temporary register exhaustion is now E705. E702 was previously overloaded — expression node count is now E704.

## Removed Variants

{: .warning }
> The `LexicalError` and `TemporalCausalityViolation` variants were removed.
> If your tooling matches on these variants, update to use `ParseError` and
> remove the causality match arm.

The following variants were removed as dead code (never constructed):

- `LexicalError` — lexer errors are correctly classified under `ParseError` (E1xx).
- `TemporalCausalityViolation` — forward-declared for planned causality analysis; never implemented.

---

## Diagnostic Rendering (ERR-001)

The `Diagnostic` struct in `src/diagnostic.rs` provides rustc-style error rendering with:

- Source line snippets with line numbers
- Caret indicators (`^^^`) pointing to the exact span
- Note and help labels for additional context
- "Did you mean?" suggestions via Levenshtein distance (`src/suggest.rs`)
- "First defined here" notes for duplicate-name errors

CLI errors are rendered through `render_diagnostic()` in `mirr-compile`. LSP diagnostics are converted via `MirrError::to_diagnostic()`.

---

## See Also

- [Tutorial](tutorial) — Lesson 9: Common errors
- [Type System](type-system) — Type rules behind E6xx errors
- [R-SPU Reference](rspu-reference) — Resource limits behind E7xx errors
- [Migration Guide](migration-guide) — New error codes added per version
