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
| E200 | *(category prefix for all semantic errors)* | `src/error.rs` |
| — | `Duplicate signal name: '{name}'.` | `src/validation/semantic.rs` |
| — | `Duplicate guard name: '{name}'.` | `src/validation/semantic.rs` |
| — | `Duplicate reflex name: '{name}'.` | `src/validation/semantic.rs` |
| — | `Guard '{name}' references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| — | `Reflex '{name}' references undeclared guard '{guard}'.` | `src/validation/semantic.rs` |
| — | `Reflex '{name}' assigns to input signal '{sig}', which is not writable.` | `src/validation/semantic.rs` |
| — | `Reflex '{name}' assigns to undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| — | `Reflex '{name}' assignment references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| — | `'{context}' contains prev('{sig}') with delay 0; delay must be >= 1.` | `src/validation/semantic.rs` |
| — | `Duplicate property name: '{name}'.` | `src/validation/semantic.rs` |
| — | `Property '{name}' references undeclared signal '{sig}'.` | `src/validation/semantic.rs` |
| — | `signal '{sig}' is internal to pattern '{pat}' and cannot be referenced externally` | `src/expand/mod.rs` |

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
