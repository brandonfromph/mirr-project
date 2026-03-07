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

Capture the full error output.

## Step 2 — Decode the Error

MIRR errors follow the format `[Exxx] <Category> error: <message>`.

| Code | Category | Where to look |
|------|----------|---------------|
| `[E100]` | Parse error | `src/parser/module_parser.rs`, `src/parser/expr_parser.rs`, `src/lexer/tokenizer.rs` |
| `[E200]` | Semantic error | `src/validation/semantic.rs`, `src/expand/mod.rs` |
| `[E300]` | Temporal error | `src/temporal/compiler.rs`, `src/temporal/emit.rs` |
| `[E400]` | Pattern error | `src/parser/pattern_parser.rs`, `src/expand/mod.rs`, `src/validation/semantic.rs` |

## Step 3 — Locate the Source Line

Read the `.mirr` file the user provided. Map the error message to the specific line that triggers it:

- **Parse errors**: Usually point to a specific declaration (signal, guard, reflex, property) with the name in the message. Find that declaration.
- **Semantic errors**: Name the offending construct (e.g., `"Guard 'g' references undeclared signal 'x'"`). Find the guard named `g` and its `when` clause.
- **Pattern errors**: Name the pattern definition or call site. Find `def <name>` or `<name>(...)` lines.
- **Temporal errors**: Name the guard that can't be lowered. Find the guard declaration.

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
| `"Unknown signal type: X"` | Invalid type | Use `bool`, `u8`, `u16`, `u32`, `u64` |
| `"Guard 'X' missing 'when' clause"` | Wrong guard body structure | Add `when <expr>` line |
| `"Guard 'X' missing 'for' clause"` | Missing cycle count | Add `for N cycles;` line |
| `"Reflex 'X' references undeclared guard 'Y'"` | Typo in guard name | Check `on <guard_name>` matches a guard |
| `"Reflex 'X' assigns to input signal 'Y'"` | Writing to an input | Change signal to `out` or `internal` |
| `"Duplicate signal/guard/reflex name"` | Name reuse | Rename one of the duplicates |
| `"prev('X') with delay 0"` | Prev with zero delay | Use delay >= 1 |
| `"formula must start with 'always' or 'never'"` | Wrong property keyword | Properties use `always (...)`, `never (...)`, or `always (P -> Q)` |

## MIRR Syntax Quick Reference

```
module <name> {
    signal <name>: in|out|internal bool|u8|u16|u32|u64;

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
    }
}
```
