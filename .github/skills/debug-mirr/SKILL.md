---
name: debug-mirr
description: 'Debug a failing .mirr file — read the compiler error, locate the source line, explain what went wrong, and suggest a fix. Use this when a MIRR program fails to compile or validate.'
argument-hint: 'Path to .mirr file or paste the error message'
user-invocable: true
---

# MIRR Debugger

When a `.mirr` file fails to compile, this skill walks through a structured diagnosis: reproduce the error, decode the error code, find the offending line, explain the root cause, and suggest a concrete fix.

## Step 1 — Reproduce

Run the file through the compiler:

```bash
cargo run --bin mirr-compile -- --emit verilog <file.mirr> 2>&1
```

Capture the full error output. If the error is in a specific backend, also try:

```bash
cargo run --bin mirr-compile -- --emit firrtl <file.mirr> 2>&1
cargo run --bin mirr-compile -- --emit testbench <file.mirr> 2>&1
```

## Step 2 — Decode the Error

MIRR errors follow the format `[Exxx] <Category> error: [Eyyy] <message>`.

The outer code (Exxx) identifies the category. The inner sub-code (Eyyy), if present, pinpoints the exact parser/pattern rule.

| Code | Category | Where to look |
|------|----------|---------------|
| `[E1xx]` | Parse error (E101-E166, E170-E181) | `src/parser/module_parser.rs`, `src/parser/expr_parser.rs`, `src/lexer/tokenizer.rs` |
| `[E2xx]` | Semantic error (E201-E216) | `src/validation/semantic.rs`, `src/expand/mod.rs` |
| `[E300]` | Temporal error | `src/temporal/compiler.rs`, `src/temporal/emit.rs` |
| `[E4xx]` | Pattern error (E400-E425) | `src/parser/pattern_parser.rs`, `src/expand/mod.rs`, `src/validation/semantic.rs` |
| `[E5xx]` | Width inference error (E500-E511) | `src/width/solver.rs`, `src/width/scc_solver.rs`, `src/width/verify.rs` |
| `[E6xx]` | Type error (E601-E609) | `src/typeck/`, `src/validation/semantic.rs` |
| `[E7xx]` | R-SPU error (E701-E705) | `src/emit/rspu.rs`, `src/emit/rspu_regalloc.rs` |

### Key Sub-Codes

| Sub-code | Meaning |
|----------|---------|
| E114 | Too many tokens remaining after parse |
| E141 | Expected `{` after guard name |
| E151 | Expected `{` after reflex name |
| E170 | Unexpected token in expression |
| E216 | Single-writer violation (two reflexes write same signal) |
| E300 | Guard condition can't be lowered to hardware |
| E605 | Signed/unsigned width mismatch |
| E608 | Mixed signedness in binary expression |
| E609 | Negate applied to boolean |

## Step 3 — Locate the Source Line

Read the `.mirr` file the user provided. Map the error message to the specific line that triggers it:

- **Parse errors**: Usually point to a specific declaration (signal, guard, reflex, property) with the name in the message. Find that declaration.
- **Semantic errors**: Name the offending construct (e.g., `"Guard 'g' references undeclared signal 'x'"`). Find the guard named `g` and its `when` clause.
- **Pattern errors**: Name the pattern definition or call site. Find `def <name>` or `<name>(...)` lines.
- **Temporal errors**: Name the guard that can't be lowered. Find the guard declaration. Common cause: signal-to-signal comparison in `when` clause.
- **Type errors**: Name the expression with mismatched types. Check for signed/unsigned mixing.

## Step 4 — Explain

Provide a clear, concise explanation:

1. **What the error means** — translate the compiler message into plain English
2. **Why it happened** — the specific MIRR rule that was violated
3. **Where in the file** — the exact line and construct

## Step 5 — Suggest Fix

Propose a minimal edit to fix the error. Show the corrected `.mirr` source. Only change what's necessary.

## Common MIRR Mistakes

| Error pattern | Cause | Fix |
|--------------|-------|-----|
| `"Signal declaration must end with ';'"` | Missing semicolon | Add `;` after type |
| `"Unknown signal kind: X"` | Typo in direction | Use `in`, `out`, or `internal` |
| `"Unknown signal type: X"` | Invalid type | Use `bool`, `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64` |
| `"Guard 'X' missing 'when' clause"` | Wrong guard body structure | Add `when <expr>` line |
| `"Guard 'X' missing 'for' clause"` | Missing cycle count | Add `for N cycles;` line |
| `"Reflex 'X' references undeclared guard 'Y'"` | Typo in guard name | Check `on <guard_name>` matches a guard |
| `"Reflex 'X' assigns to input signal 'Y'"` | Writing to an input | Change signal to `out` or `internal` |
| `"Duplicate signal/guard/reflex name"` | Name reuse | Rename one of the duplicates |
| `"prev('X') with delay 0"` | Prev with zero delay | Use delay >= 1 |
| `"formula must start with 'always' or 'never'"` | Wrong property keyword | Properties use `always (...)`, `never (...)`, `eventually within N (...)`, or `always (P followed_by N Q)` |
| `"Signed/unsigned mismatch in expression"` | Mixing signed and unsigned types | Ensure both operands are signed (`i8`-`i64`) or both unsigned (`u8`-`u64`) |
| `[E216] single-writer violation` | Two reflexes assign to same signal | Restructure so each signal is written by exactly one reflex |
| `[E300] temporal lowering failure` | Signal-to-signal comparison in guard `when` | Use boolean flags or signal-vs-literal only |

## Guard Condition Debugging

The temporal compiler can only lower these forms to hardware:
- `when signal` (boolean)
- `when !signal` (negated boolean)
- `when signal <op> literal` (comparison against constant)
- Boolean AND/OR of the above

If you see E300, check the guard's `when` clause for:
- Signal-to-signal comparison (`when a == b`) — rewrite using boolean flags
- Complex arithmetic (`when a + b > 100`) — compute in a reflex, store in an internal signal, guard on that

## MIRR Syntax Quick Reference

```
module <name> {
    signal <name>: in|out|internal bool|u8|u16|u32|u64|i8|i16|i32|i64;

    guard <name> {
        when <expression>
        for <N> cycles;
    }

    reflex <name> {
        on <guard_name> [and <guard_name>...] {
            <signal> = <expression>;
        }
    }

    property <name> {
        always (<expression>);
        never (<expression>);
        always (<expression> -> <expression>);
        never (<expression> -> <expression>);
        eventually within <N> (<expression>);
        always (<expression> followed_by <N> <expression>);
    }
}
```
