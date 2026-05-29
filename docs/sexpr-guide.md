---
title: S-Expression IR Guide
nav_order: 8
---

# S-Expression IR Guide

The S-expression intermediate representation (IR) provides a homoiconic
encoding of MIRR programs. Every AST node maps to a unique S-expression
form, enabling code-as-data transformations, hygienic macros, and
bounded compile-time evaluation.

---

## Table of Contents

1. [What is the S-expression IR?](#what-is-the-s-expression-ir)
2. [When to use --emit sexpr](#when-to-use---emit-sexpr)
3. [The 8 SExpr variants](#the-8-sexpr-variants)
4. [Tagged list conventions](#tagged-list-conventions)
5. [Round-trip property](#round-trip-property)
6. [Bounded evaluation model](#bounded-evaluation-model)
7. [Hygienic macro expansion](#hygienic-macro-expansion)
8. [Reader macros](#reader-macros)
9. [Example output](#example-output)
10. [Error codes (E8xx)](#error-codes-e8xx)

---

## What is the S-expression IR?

S-expressions (symbolic expressions) are a parenthesized prefix notation
originating from Lisp. In MIRR, the S-expression IR is a faithful
encoding of the compiler's abstract syntax tree (AST) as nested lists
and atoms. This gives you a human-readable, machine-parseable view of
exactly what the compiler sees after parsing.

The S-expression IR serves three purposes:

1. **Compile-time metaprogramming** -- code-as-data transformations via
   the eval/apply core and hygienic macros.
2. **Debugging and tooling** -- a serializable, human-readable IR that
   external tools can consume and produce.
3. **Self-hosting validation** -- a verified round-trip invariant between
   the AST and the S-expression encoding ensures the encoding is lossless.

The implementation lives in `src/sexpr/` (7 submodules, approximately
2,100 lines).

---

## When to use --emit sexpr

Use the S-expression output when you need to:

- **Inspect the AST** after parsing. The S-expression form shows every
  signal, guard, reflex, property, and pattern call in a structured tree.
- **Build external tools** that read or transform MIRR programs. Parse
  the S-expression text, manipulate the tree, and convert it back.
- **Debug pattern expansion**. The `pattern-origins` section records
  which pattern call generated each expanded element.
- **Verify round-trip correctness**. Convert your program to S-expressions
  and back to confirm the encoding preserves all information.

```bash
cargo run --bin mirr-compile -- --emit sexpr my_module.mirr
```

---

## The 8 SExpr variants

The core `SExpr` enum (defined in `src/sexpr/types.rs`) has 8 variants.
Four are atoms (leaf values), four are compound forms:

### Atoms

| Variant | Syntax | Purpose |
|---------|--------|---------|
| `Symbol(String)` | `signal`, `guard`, `always` | Identifiers, keywords, operators |
| `Integer(u64)` | `42`, `1000` | Widths, cycle counts, delay values |
| `Bool(bool)` | `true`, `false` | Boolean literals |
| `Str(String)` | `"airway_pressure"` | Signal names, module names, pattern names |

### Compound forms

| Variant | Syntax | Purpose |
|---------|--------|---------|
| `List(Vec<SExpr>)` | `(signal "name" input bool)` | Tagged lists (the primary structural form) |
| `Quote(Box<SExpr>)` | `'(expr)` | Returns the expression unevaluated |
| `Quasiquote(Box<SExpr>)` | `` `(expr) `` | Template with unquote splices |
| `Unquote(Box<SExpr>)` | `,expr` | Evaluated and spliced into enclosing quasiquote |

Every S-expression tree is bounded: lists by `MAX_SEXPR_NODES` (4,096),
strings by `MAX_SEXPR_STRING_LEN` (1 MB).

---

## Tagged list conventions

MIRR S-expressions use **tagged lists** where the first element (the
head) is a symbol that identifies the form. The remaining elements are
the form's fields.

```
(signal "airway_pressure" input (unsigned 16))
 ^^^^^^  ^^^^^^^^^^^^^^^^^  ^^^^^  ^^^^^^^^^^^
  head     name (string)     kind   type
```

Common head tags:

| Head symbol | Structure |
|-------------|-----------|
| `program` | `(program (patterns ...) (module ...))` |
| `module` | `(module "name" (signals ...) (guards ...) (reflexes ...) ...)` |
| `signal` | `(signal "name" input/output/internal type)` |
| `guard` | `(guard "name" condition cycles)` |
| `reflex` | `(reflex "name" (on "guard1" ...) (assign "target" value) ...)` |
| `property` | `(property "name" assert/cover/assume formula)` |
| `pattern-def` | `(pattern-def "name" (params ...) (reflect ...))` |
| `pattern-call` | `(pattern-call "name" arg1 arg2 ...)` |

---

## Round-trip property

The bidirectional converter (`src/sexpr/convert.rs`) guarantees:

```
parse_mirr(source) == sexpr_to_ast(ast_to_sexpr(parse_mirr(source)))
```

This means you can convert any parsed MIRR program to S-expressions,
print it to text, parse the text back into S-expressions, convert back
to AST, and get an identical program.

The two key functions are:

- `ast_to_sexpr(program)` -- converts a `MirrProgram` AST to an `SExpr`
  tree. This is a total function (never fails).
- `sexpr_to_ast(sexpr)` -- converts an `SExpr` tree back to a
  `MirrProgram`. Returns `Result<MirrProgram, MirrError>` because the
  S-expression may be malformed.

The round-trip property is verified by self-hosting tests in
`tests/self_hosting_ir_schema_tests.rs` and
`tests/self_hosting_parity_tests.rs`.

### Using the round-trip in Rust

```rust
use mirr::sexpr::{ast_to_sexpr, sexpr_to_ast, print_sexpr, parse_sexpr};

// AST -> S-expression -> text
let sexpr = ast_to_sexpr(&program);
let text = print_sexpr(&sexpr);

// text -> S-expression -> AST
let parsed_sexpr = parse_sexpr(&text)?;
let recovered_program = sexpr_to_ast(&parsed_sexpr)?;

assert_eq!(program, recovered_program);
```

---

## Bounded evaluation model

The eval/apply core (`src/sexpr/eval.rs`) implements a bounded
interpreter for compile-time metaprogramming. It is deliberately
not a general-purpose Lisp interpreter -- it omits lambda/closures
to avoid Turing-completeness, which would violate the NASA Power-of-10
termination guarantee.

### Resource bounds

| Constant | Value | Purpose |
|----------|-------|---------|
| `MAX_EVAL_STEPS` | 10,000 | Total evaluation steps (fuel counter). Hard error at 0. |
| `MAX_EVAL_DEPTH` | 32 | Maximum nesting depth for evaluation frames. |
| `MAX_SEXPR_DEPTH` | 64 | Maximum nesting depth for parsing/printing. |
| `MAX_SEXPR_NODES` | 4,096 | Maximum nodes in a single S-expression tree. |
| `MAX_SEXPR_STRING_LEN` | 1,048,576 | Maximum input string length (1 MB). |
| `MAX_MACRO_EXPAND_DEPTH` | 8 | Maximum depth for hygienic macro expansion. |
| `MAX_READER_MACROS` | 32 | Maximum registered reader macros. |

### Evaluation state

The evaluator uses an explicit `EvalState` with a step counter, depth
tracker, and flat environment (no closures):

```rust
use mirr::sexpr::{eval, EvalState, parse_sexpr};

let expr = parse_sexpr("(if true 42 99)")?;
let mut state = EvalState::new();
let result = eval(&expr, &mut state)?;
// result == SExpr::Integer(42)
```

### Supported special forms

The evaluator supports these forms:

| Form | Syntax | Description |
|------|--------|-------------|
| `quote` | `(quote expr)` | Return expression unevaluated |
| `if` | `(if cond then else)` | Conditional (bool, nonzero int, or nonempty list = true) |
| `list` | `(list a b c)` | Construct a list from evaluated arguments |
| `car` | `(car lst)` | First element of a list |
| `cdr` | `(cdr lst)` | All elements after the first |
| `cons` | `(cons head tail)` | Prepend an element to a list |
| `eq?` | `(eq? a b)` | Structural equality test |
| `symbol?` | `(symbol? x)` | Type predicate: is this a symbol? |
| `list?` | `(list? x)` | Type predicate: is this a list? |
| `integer?` | `(integer? x)` | Type predicate: is this an integer? |
| `bool?` | `(bool? x)` | Type predicate: is this a boolean? |
| `match-type` | `(match-type type clause1 clause2 ...)` | Pattern-match on type structure |

---

## Hygienic macro expansion

The macro expander (`src/sexpr/macro_expand.rs`) supports
quasiquote/unquote-based template instantiation with hygienic name
resolution. This prevents name collisions when expanding templates.

Internal names generated during expansion are suffixed with
`__hyg{expansion_id}` to ensure uniqueness.

```rust
use mirr::sexpr::MacroExpander;

let mut expander = MacroExpander::new();
let expanded = expander.expand_hygienic(
    &template,
    &param_names,
    &bindings,
    0,  // initial depth
)?;
```

Expansion is bounded by `MAX_MACRO_EXPAND_DEPTH` (8 levels) and
`MAX_SEXPR_NODES` (4,096 nodes).

---

## Reader macros

Reader macros (`src/sexpr/reader.rs`) transform notation during
S-expression parsing (read time), before evaluation. Three built-in
reader macros provide hardware-domain shorthand:

| Macro | Input | Output |
|-------|-------|--------|
| `#freq` | `100MHz` | `(frequency 100000000)` |
| `#delay` | `5` | `(temporal-delay 5)` |
| `#range` | `0..255` | `(refinement-range 0 255)` |

The `#freq` macro supports Hz, KHz/kHz, MHz, and GHz suffixes.

Custom reader macros can be registered up to the `MAX_READER_MACROS`
(32) limit.

---

## Example output

Given this MIRR source:

```
module neonatal_respirator {
    signals {
        respirator_enable: in bool
        airway_pressure:   in u16
        clamp_valve:       out bool
    }

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
```

The `--emit sexpr` output is:

```lisp
(program
  (patterns)
  (module
    "neonatal_respirator"
    (signals
      (signal "respirator_enable" input bool)
      (signal "airway_pressure" input (unsigned 16))
      (signal "clamp_valve" output bool))
    (guards
      (guard "sustained_pressure_drop"
        (< (signal "airway_pressure") 50)
        1000))
    (reflexes
      (reflex "emergency_clamp"
        (on "sustained_pressure_drop")
        (assign "clamp_valve" true)))
    (properties)
    (pattern-calls)
    (pattern-origins)))
```

Notice:
- Signal types are encoded as `bool` or `(unsigned 16)` / `(signed 32)`.
- Guard conditions use prefix notation: `(< (signal "airway_pressure") 50)`.
- The `for 1000 cycles` clause becomes the integer `1000` as the guard's
  third field.
- Empty sections (`patterns`, `properties`, etc.) are present but empty.

---

## Error codes (E8xx)

All S-expression errors use the E8xx range. The error is returned as a
`MirrError::SExprError` with a message string containing the code.

| Code | Meaning |
|------|---------|
| E800 | Invalid token (e.g., bare `#` without `t` or `f`), or unexpected trailing tokens |
| E801 | Empty input, unterminated string literal, or unexpected end of input |
| E802 | Unbalanced parentheses (missing `)` or unexpected `)`) |
| E803 | Nesting depth exceeds `MAX_SEXPR_DEPTH` (64) |
| E804 | Node/token count exceeds `MAX_SEXPR_NODES` (4,096) or input exceeds `MAX_SEXPR_STRING_LEN` |
| E805 | Structural error: unexpected form, unknown operator, or expected symbol as list head |
| E806 | Semantic error: missing required fields, wrong types, unknown parameter kinds |
| E807 | Signal kind or type error in S-expression (unknown kind, invalid type form) |
| E808 | Expression nesting exceeds max depth, or invalid reader macro argument |
| E809 | AST to S-expression conversion error |
| E810 | Round-trip validation failure |
| E811 | Evaluation depth exceeds `MAX_EVAL_DEPTH` (32) |
| E812 | Evaluation steps exceed `MAX_EVAL_STEPS` (10,000) |
| E813 | Undefined symbol during evaluation |
| E814 | Macro expansion depth exceeded (`MAX_MACRO_EXPAND_DEPTH` = 8) |
| E815 | Reader macro error: too many registered macros, or unknown reader macro name |

---

## See Also

- [Tutorial](tutorial) -- Getting started with MIRR
- [Error Codes](error_codes) -- Full list of compiler diagnostics
- [Contributing](contributing) -- Coding standards and campaign workflow
