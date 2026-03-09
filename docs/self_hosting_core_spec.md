# MIRR-CORE Self-Hosting Subset Specification

**Version:** 1.0-draft  
**Status:** Frozen for self-hosting milestone v1  
**Purpose:** Defines the minimal language feature set required for MIRR to implement its own compiler front-end (self-hosting).

---

## 1. Introduction

MIRR-CORE is the **self-hostable subset** of the MIRR language. It contains exactly the features needed to write a MIRR compiler front-end in MIRR itself:

- Tokenizer (lexer)
- Parser (recursive descent / Pratt)
- Semantic validator
- Temporal guard lowering pass

The goal is **stage-1 self-hosting**: the Rust runtime hosts and executes MIRR compiler modules, while the MIRR compiler modules themselves process MIRR source text. In a later stage, the MIRR compiler modules will compile themselves.

---

## 2. Included Features (Frozen)

The following features are **included** and their semantics are frozen for this milestone.

### 2.1 Primitive Types

| Type     | Description                               |
|----------|-------------------------------------------|
| `bool`   | Boolean: `true` or `false`                |
| `u8`     | Unsigned 8-bit integer                    |
| `u16`    | Unsigned 16-bit integer                   |
| `u32`    | Unsigned 32-bit integer                   |
| `u64`    | Unsigned 64-bit integer                   |
| `usize`  | Unsigned platform-word-size integer       |
| `i8`     | Signed 8-bit integer                      |
| `i16`    | Signed 16-bit integer                     |
| `i32`    | Signed 32-bit integer                     |
| `i64`    | Signed 64-bit integer                     |

String handling is provided via the stdlib slice type (see Section 4).

### 2.2 Aggregate Types

| Feature   | Description                                                    |
|-----------|----------------------------------------------------------------|
| Arrays    | Fixed-size, statically-bounded: `let buf: [u8; 256]`          |
| Structs   | Named field records: `struct Token { kind: u32, len: usize }` |
| Enums     | C-style discriminated union (no heap payload in MIRR-CORE)     |

No dynamic-length collections. Sizes must be compile-time constants or bounded by `usize` parameters declared at module top level.

### 2.3 Control Flow

| Construct       | Description                                                 |
|-----------------|-------------------------------------------------------------|
| `if / else`     | Standard conditional branching                              |
| `loop`          | Unconditional loop with explicit `break` or `return` exit   |
| `for i in 0..N` | Bounded numeric iteration (upper bound must be provably finite) |
| `while cond`    | Permitted only when a clear termination argument exists     |
| `match`         | Exhaustive pattern matching on enums and integer values     |
| `return`        | Explicit function return                                    |
| `break`         | Loop exit                                                   |

**NASA-CORE rule:** Every loop must have a statically-bounded or demonstrably-finite iteration count. Unbounded loops that lack a formal termination argument are a compile-time warning and a self-hosting build blocker.

### 2.4 Functions

- Named functions with typed parameters and typed return values.
- No closures, no first-class functions in MIRR-CORE v1.
- Mutual recursion is **excluded** (see Section 3).
- Tail recursion transformable to `loop` is **excluded** from MIRR-CORE v1 — use explicit loops.

```mirr
fn tokenize(src: &str, out: &mut [Token; MAX_TOKENS]) -> usize {
    // ...
}
```

### 2.5 Modules

- A MIRR-CORE program is organized into named modules.
- A module declares:
  - Named signals (in MIRR hardware domain)
  - Named functions (in MIRR-CORE software domain)
  - Named types (structs / enums)
  - Named constants

```mirr
module lexer {
    const MAX_TOKENS: usize = 4096;

    struct Token { kind: u32, start: usize, len: usize }

    fn tokenize(src: &str, out: &mut [Token; MAX_TOKENS]) -> usize {
        // ...
    }
}
```

### 2.6 Expressions

All expressions supported in the existing MIRR Rust implementation are available:

| Category    | Operators                              |
|-------------|----------------------------------------|
| Logical     | `!`, `&&`, `\|\|`                      |
| Bitwise     | `^`, `&`, `\|`                         |
| Comparison  | `==`, `!=`, `<`, `<=`, `>`, `>=`       |
| Arithmetic  | `+`, `-`, `*`, `<<`, `>>`              |
| Grouping    | `(expr)`                               |
| Field access| `struct_val.field`                     |
| Index       | `array[i]`                             |
| Call        | `fn_name(args...)`                     |

### 2.7 Let Bindings and Mutation

```mirr
let x: u32 = 0;
let mut counter: usize = 0;
counter = counter + 1;
```

### 2.8 String Slices (Borrowed, Read-Only)

String slices `&str` are **read-only views** into a source buffer. They are not heap-allocated. Operations are provided through the stdlib (see Section 4).

```mirr
fn is_keyword(s: &str) -> bool { ... }
```

### 2.9 References

- `&T` — immutable borrow.
- `&mut T` — mutable borrow (for output buffers / accumulators).
- No raw pointers. No lifetime annotations required in MIRR-CORE v1 (simplified region model).

### 2.10 Constants

```mirr
const MAX_DEPTH: usize = 128;
const SHIFT_REGISTER_THRESHOLD: u64 = 16;
```

---

## 3. Excluded Features (MIRR-CORE v1)

The following features are **explicitly out of scope** for the self-hosting v1 milestone. They may be added in a future MIRR version.

| Feature                       | Reason for exclusion                                              |
|-------------------------------|-------------------------------------------------------------------|
| Dynamic memory allocation     | Violates NASA determinism/safety rules; no `malloc` / `Box`       |
| Recursion (general)           | Cannot guarantee stack-bounded execution; use iterative forms     |
| Closures / lambdas            | Increases execution model complexity; out of scope for v1         |
| Trait objects / vtables       | Dynamic dispatch not supported in v1                              |
| Generics / type parameters    | Basic monomorphization may appear in v2; excluded from v1         |
| Exception handling / panics   | All errors must be explicit `Result` returns                      |
| Threading / concurrency       | Single-threaded execution model for MIRR-CORE                    |
| Heap collections (Vec, etc.)  | All buffers are stack/fixed-size; deterministic memory layout     |
| Lifetimes (explicit syntax)   | Simplified region model; no `'a` annotations in v1               |
| Operator overloading          | Not supported in v1                                               |
| Macros                        | Not supported in v1                                               |

---

## 4. Required Standard Library Primitives

These primitives are needed by the MIRR compiler modules (see `stdlib/mirr_core/`):

| Primitive             | Module                    | Description                                         |
|-----------------------|---------------------------|-----------------------------------------------------|
| `str_len`             | `stdlib/mirr_core/str.mirr`   | Get length of a `&str` slice                    |
| `str_byte_at`         | `stdlib/mirr_core/str.mirr`   | Zero-copy byte access at index                  |
| `str_slice`           | `stdlib/mirr_core/str.mirr`   | Sub-slice `[start..end]` of a `&str`            |
| `str_eq`              | `stdlib/mirr_core/str.mirr`   | Equality check between two `&str` slices        |
| `TokenBuffer`         | `stdlib/mirr_core/token_buffer.mirr` | Fixed-capacity token accumulator          |
| `FixedMap`            | `stdlib/mirr_core/fixed_map.mirr` | Open-addressed fixed-capacity hash table    |
| `Diagnostic`          | `stdlib/mirr_core/diagnostics.mirr` | Structured error/warning record           |
| `DiagnosticBuilder`   | `stdlib/mirr_core/diagnostics.mirr` | Builder for diagnostic emission            |

---

## 5. Determinism and Safety Rules (NASA-CORE)

All MIRR-CORE code must satisfy:

1. **No unbounded loops.** Every loop has a statically visible upper bound or a formal argument proving termination.
2. **No allocation.** All buffers are declared with compile-time sizes or bounded parameters.
3. **All errors are explicit.** Functions that can fail return `Result<T, DiagCode>`. No panics in production paths.
4. **No hidden global mutable state.** Module-level mutable state is forbidden; pass state explicitly.
5. **Deterministic output.** For the same input bytes, the compiler must always produce exactly the same output.
6. **Bounded recursion exclusion.** Even tail-recursive forms are excluded from v1; use `loop` explicitly.

---

## 6. Grammar Summary (MIRR-CORE v1)

```
program        := module_decl+

module_decl    := 'module' ident '{' module_item* '}'

module_item    := const_decl
                | struct_decl
                | enum_decl
                | fn_decl
                | signal_decl
                | guard_decl
                | reflex_decl

const_decl     := 'const' ident ':' type '=' expr ';'

struct_decl    := 'struct' ident '{' field_list '}'
field_list     := (ident ':' type ','?)*

enum_decl      := 'enum' ident '{' variant_list '}'
variant_list   := (ident ('(' type ')')? ','?)*

fn_decl        := 'fn' ident '(' param_list ')' '->' type block
param_list     := (ident ':' type ','?)*

type           := 'bool' | 'u8' | 'u16' | 'u32' | 'u64' | 'usize'
                | 'i8' | 'i16' | 'i32' | 'i64'
                | '&' 'str'
                | '&' type
                | '&' 'mut' type
                | '[' type ';' integer ']'
                | ident

block          := '{' stmt* '}'

stmt           := let_stmt
                | assign_stmt
                | return_stmt
                | break_stmt
                | if_stmt
                | loop_stmt
                | for_stmt
                | while_stmt
                | match_stmt
                | expr_stmt

let_stmt       := 'let' 'mut'? ident ':' type '=' expr ';'
assign_stmt    := lvalue '=' expr ';'
return_stmt    := 'return' expr? ';'
break_stmt     := 'break' ';'
if_stmt        := 'if' expr block ('else' (block | if_stmt))?
loop_stmt      := 'loop' block
for_stmt       := 'for' ident 'in' expr '..' expr block
while_stmt     := 'while' expr block
match_stmt     := 'match' expr '{' arm* '}'
arm            := pattern '=>' (block | expr ','?)
expr_stmt      := expr ';'

lvalue         := ident
                | lvalue '.' ident
                | lvalue '[' expr ']'

expr           := literal
                | ident
                | expr binop expr
                | unop expr
                | expr '.' ident
                | expr '[' expr ']'
                | ident '(' arg_list ')'
                | '(' expr ')'

binop          := '&&' | '||' | '^' | '&' | '|'
                | '==' | '!=' | '<' | '<=' | '>' | '>='
                | '+' | '-' | '*' | '<<' | '>>'

unop           := '!'

literal        := 'true' | 'false' | integer | string_lit

integer        := [0-9]+
string_lit     := '"' [^"]* '"'

ident          := [a-zA-Z_][a-zA-Z0-9_]*

// MIRR hardware primitives (hardware domain, unchanged from MIRR spec)
signal_decl    := 'signal' ident ':' signal_kind type ';'
signal_kind    := 'in' | 'out' | 'internal'

guard_decl     := 'guard' ident '{' 'when' expr 'for' integer 'cycles' ';'? '}'
reflex_decl    := 'reflex' ident '{' 'on' guard_ref_list '{' assignment* '}' '}'
guard_ref_list := ident ('and' ident)*
assignment     := ident '=' expr ';'
```

---

## 7. Versioning and Stability

- This specification is **frozen** at v1 for the self-hosting milestone.
- No changes to included features or grammar without incrementing the version and updating the IR contract (`docs/self_hosting_ir_contract.md`).
- Additions to the excluded list require a documented rationale.

---

## 8. Acceptance Criteria

The self-hosting milestone is considered achieved when:

1. A MIRR source file can be tokenized by `compiler_mirr/lexer.mirr` (via bootstrap runner).
2. The token stream is parsed by `compiler_mirr/parser.mirr` into an AST matching the IR contract.
3. The AST is validated by `compiler_mirr/semantic.mirr` with diagnostics matching Rust validator output class.
4. Temporal guards are lowered by `compiler_mirr/temporal_lowering.mirr` with netlist parity to Rust backend.
5. All parity tests in `tests/self_hosting_parity_tests.rs` pass with byte-stable or semantically-equivalent output.

---

*End of MIRR-CORE Self-Hosting Core Specification v1.0-draft*