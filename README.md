## NASA-style Rust Project

This project is a small Rust binary crate structured with inspiration from NASA/JPL coding guidance, adapted to the Rust programming language.

Key public references:

- JPL Institutional Coding Standard for the C Programming Language (DOCID D-60411)
- “The Power of 10: Rules for Developing Safety-Critical Code” (G. J. Holzmann, NASA/JPL)
- NASA F´ (F Prime) code and style guidelines

### Rust adaptation of NASA-style rules

This project follows a conservative subset of those ideas, mapped onto Rust:

- **No `unsafe`**: The crate is compiled with `#![forbid(unsafe_code)]`.
- **Treat warnings as errors**: The crate is compiled with `#![deny(warnings)]` so all warnings must be fixed.
- **Simple, bounded control flow**: Prefer simple loops with clear bounds; avoid recursion.
- **No unchecked fallible operations**: Avoid `unwrap`, `expect`, and panics in production code; handle `Result` and `Option` explicitly.
- **Small, focused functions**: Keep functions short and single-purpose for clarity and reviewability.
- **Predictable behavior**: Avoid global mutable state and hidden side effects; prefer explicit data flow.
- **Tooling support**: Use `cargo clippy` and `cargo fmt` to maintain consistency and catch issues early.

### Building and running

1. Install Rust and Cargo from `https://rustup.rs` and the required MSVC build tools for C++.
2. From this directory, run:
   - `cargo build --release`
   - `cargo run`

### Fast local run (Windows PowerShell)

For quick parser runs without typing the full executable path:

- `./run-mirr.ps1` (runs the bundled example)
- `./run-mirr.ps1 ./examples/neonatal_respirator.mirr`

This wrapper is intentionally strict and fail-fast:

- Verifies the input file exists.
- Verifies a built binary exists in `target/debug` or `target/release`.
- Exits non-zero on errors.

### Project documentation

- See `docs/r_spu_overview.md` for a high-level summary of the Reflexive Processing Unit (R-SPU) architecture and its research context.
- See `docs/roadmap.md` for the breakdown of this project into small, buildable subprojects.

src/
├── main.rs                    # Entry point — CLI only
├── lib.rs                     # Public API re-exports
├── error.rs                   # Centralized error authority
├── ast/
│   ├── mod.rs                 # Re-exports all AST types
│   ├── types.rs               # SignalKind, SignalType, BinaryOp, UnaryOp, LiteralValue
│   ├── expr.rs                # Expr enum (expression tree)
│   └── program.rs             # SignalDecl, Guard, Assignment, Reflex, Module, MirrProgram
├── lexer/
│   ├── mod.rs                 # Re-exports
│   └── tokenizer.rs           # Token enum + tokenize_expr() with performance optimizations
├── parser/
│   ├── mod.rs                 # Re-exports parse_mirr + parse_expression
│   ├── expr_parser.rs         # Pratt parser (precedence-climbing) with early validation
│   └── module_parser.rs       # Line-based module/signal/guard/reflex parser
└── validation/
    ├── mod.rs                 # Re-exports
    └── semantic.rs            # validate_module + collect_signal_refs with pre-allocated collections

tests/
├── expr_tests.rs              # 17 expression parser tests
├── module_tests.rs            # 23 module parser + error tests
├── validation_tests.rs        # 9 semantic validation tests
└── stress_tests.rs            # 5 stress/edge-case tests
