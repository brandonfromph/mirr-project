# MIRR Compiler — File Tree & Module Map

> Auto-generated 2026-03-17. 509K total lines, 293 Rust files, 3,469 tests passing.

> Snapshot note: this file is a dated module-tree snapshot, not the canonical topology source. For the current repository map, use [REPO_UNDERSTANDING.md](../REPO_UNDERSTANDING.md).

## Repository Statistics

| Category | Files | Lines |
|---|---|---|
| Rust source (`src/`) | 183 | 35,892 |
| Rust tests (`tests/`) | 110 | 59,073 |
| Rocq proofs (`proofs/`) | 16 | 2,536 |
| LaTeX paper (`paper/`) | 23 | 6,179 |
| Documentation (`docs/`) | 49 | 9,802 |
| Static site (`_site/`) | 20 | 9,828 |
| Everything (all tracked) | 293+ | **508,997** |

## Source Tree (`src/`) — 35,892 lines

```
src/
├── lib.rs                          82    Crate root, #![deny(warnings)]
├── main.rs                        229    CLI entry point
├── error.rs                       453    MirrError enum, error formatting
├── span.rs                         96    Source span tracking
├── pipeline.rs                    368    Compilation pipeline orchestration
├── mirr_driver.rs                 260    Driver for multi-file compilation
├── mirr_runtime.rs                318    Runtime support for MIRR execution
├── simplify.rs                    414    Expression simplification engine
├── suggest.rs                     134    Typo correction suggestions
│
├── ast/                           779    Abstract Syntax Tree
│   ├── mod.rs                            Module, Signal, Assignment, Property
│   ├── expr.rs                           Expr enum (14 variants incl. composites)
│   └── types.rs                          SignalType (8 variants), BinaryOp, UnaryOp
│
├── lexer/                         197    Lexical analysis
│   └── mod.rs                            Token types, lexer state machine
│
├── parser/                      1,802    Recursive-descent parser
│   ├── mod.rs                            Top-level parse(), error recovery
│   └── module_parser/                    Module-level parsing (signals, reflexes)
│
├── typeck/                      2,816    Type checking (Phase 2)
│   ├── mod.rs                            infer_type(), check_binary/unary, E226
│   └── extended/                         Pattern checks, width constraints
│
├── validation/                    589    Semantic validation
│   └── semantic.rs                       Signal ref checks, prev() validation
│
├── totality/                      709    Totality checking (Phase 2b)
│   ├── mod.rs                            Reflex completeness analysis
│   ├── checks.rs                         Guard coverage algorithms
│   └── types.rs                          TotalityCoverage, GuardSet
│
├── temporal/                    1,958    Temporal logic compilation (Phase 3)
│   ├── mod.rs                            Temporal guard lowering
│   └── low_level_ir/                     LTL → automaton compilation
│
├── width/                       2,232    Width inference engine (Phase 4)
│   ├── mod.rs                            Constraint solver, SCC analysis
│   ├── graph.rs                          Dependency graph construction
│   └── types.rs                          Width, WidthConstraint, MAX_*
│
├── expand/                        920    Macro/template expansion
│   ├── mod.rs                            Fragment expansion pipeline
│   ├── rename.rs                         Signal renaming in expanded frags
│   └── scoping.rs                        Scope validation for expansions
│
├── simplify.rs                    414    Bottom-up expression simplifier
│
├── sat/                         1,034    SAT-based reasoning
│   ├── mod.rs                            Propositional satisfiability
│   └── simplify_sat.rs                   SAT-guided simplification
│
├── symbolic/                    1,345    Symbolic reasoning engine (MEGA-5)
│   └── mod.rs                            Interval arithmetic, pattern matching
│
├── sexpr/                       2,733    S-expression IR
│   ├── mod.rs                            SExpr type, parser
│   └── convert/                          AST ↔ S-expr round-trip
│       ├── to_sexpr.rs                   AST → SExpr (iterative work-stack)
│       ├── from_sexpr.rs                 SExpr → AST reconstruction
│       └── parse_expr.rs                 Expression parsing from S-exprs
│
├── emit/                        7,569    Code generation backends
│   ├── mod.rs                            Backend trait, format dispatch
│   ├── verilog/                          SystemVerilog emission
│   │   └── mod.rs                        SV modules, always blocks
│   ├── firrtl.rs                         FIRRTL backend (iterative)
│   ├── rspu_asm.rs                       R-SPU assembly emission
│   ├── rspu_encoding/                    R-SPU binary encoding
│   ├── rspu_sim/                         R-SPU cycle-accurate simulator
│   ├── rspu_tagged.rs                    Tagged-word R-SPU emission
│   ├── testbench.rs                      SystemVerilog testbench generation
│   ├── dsp.rs                            DSP inference & annotation
│   ├── json_emit.rs                      JSON IR export
│   ├── dot.rs                            Graphviz DOT export
│   ├── scaffold.rs                       Scaffold/skeleton generation
│   ├── mape_k_rtl/                       MAPE-K RTL monitor emission
│   └── formal.rs                         Formal verification harnesses
│
├── mape_k/                      2,911    MAPE-K autonomic loop
│   ├── mod.rs                            Monitor-Analyze-Plan-Execute-Knowledge
│   └── bridge/                           RTL ↔ software bridge
│
├── diagnostic/                    511    Diagnostic reporting
│   └── mod.rs                            Error/warning formatting, codes
│
├── cert/                          677    Compilation certificates
│   └── mod.rs                            Hash-based build provenance
│
├── toolchain/                   1,392    External tool integration
│   └── mod.rs                            Yosys, Verilator, Icarus, GHDL
│
├── lsp/                           568    Language Server Protocol
│   └── mod.rs                            Diagnostics, hover, completions
│
├── bootstrap_runner/              619    Self-hosted test runner
│   └── mod.rs                            Execute .mirr test suites
│
├── mirr_executor/                 660    Interpreter/executor
│   └── mod.rs                            Step-based MIRR execution
│
└── bin/
    └── mirr-compile/            1,517    Standalone compiler binary
        └── main.rs                       Multi-format batch compilation
```

## Test Suite (`tests/`) — 59,073 lines, 110 files

```
tests/
├── mega1_typeck_verification_tests.rs      4,171   MEGA-1 type system
├── typeck_extended_deep_tests.rs           2,766   Extended type checking
├── mape_k_integration_tests.rs             2,171   MAPE-K integration
├── bootstrap_runner_extended_tests.rs      2,125   Bootstrap runner
├── mega2_sexpr_verification_tests.rs       2,070   MEGA-2 S-expr round-trip
├── pattern_coverage_tests.rs               2,058   Pattern completeness
├── emit_verilog_extended_tests.rs          2,052   SystemVerilog backend
├── sexpr_convert_tests.rs                  1,810   S-expr conversion
├── mape_k_bridge_tests.rs                  1,708   MAPE-K bridge
├── rspu_encoding_extended_tests.rs         1,578   R-SPU encoding
├── rspu_sim_extended_tests.rs              1,566   R-SPU simulator
├── mega4_totality_verification_tests.rs    1,499   MEGA-4 totality
├── mega3_rspu_verification_tests.rs        1,380   MEGA-3 R-SPU
├── pattern_tests.rs                        1,348   Pattern matching
├── parser_module_extended_tests.rs         1,232   Module parser
├── formal_orchestration_tests.rs           1,080   Formal verification
├── diagnostic_extended_tests.rs            1,056   Diagnostics
├── mega5_symbolic_verification_tests.rs    1,027   MEGA-5 symbolic
├── expand_extended_tests.rs                  976   Expansion
├── width_solver_tests.rs                     906   Width solver
├── ... (90 more test files)
├── eda/                                          EDA tool integration
│   └── run_eda_tests.sh                          Yosys/Verilator/eqy
├── fixtures/                                     Test data
│   ├── ast/                                      AST snapshots
│   ├── netlist/                                  Netlist references
│   ├── parse/                                    Parse fixtures
│   ├── semantic/                                 Semantic fixtures
│   └── tokens/                                   Lexer fixtures
├── sim/                                          Simulation tests
└── ui/                                           UI regression tests
    ├── parse/                                    Parse error snapshots
    └── semantic/                                 Semantic error snapshots
```

## Rocq Proofs (`proofs/`) — 2,536 lines, 16 files

```
proofs/
├── width/                          11 files
│   ├── Solver.v                   475   Width solver correctness
│   ├── Constraints.v                    Constraint system proofs
│   ├── BitOps.v                         Bitwise operation lemmas
│   ├── SCC/                             SCC algorithm proofs
│   └── ...
└── rspu/                            5 files
    ├── Encoding.v                       Instruction encoding proofs
    ├── TaggedWord.v                     Tagged word field proofs
    └── test_fields.v                    Field extraction tests
```

## Paper & Living Document (`paper/`) — 6,179 lines

```
paper/
├── living-doc/
│   ├── main.tex                   222   Master document
│   ├── metrics.tex                133   Metrics & counters
│   ├── ch-width.tex               239   Width inference chapter
│   ├── ch-vision.tex              101   Vision & architecture
│   └── ...                              Additional chapters
└── demos/
    ├── mirr_wasm_bg.wasm          596K  WASM compiler binary
    └── ...                              Playground assets
```

## Infrastructure

```
.claude/commands/                        6 skill definitions
.github/
├── skills/                              6 SKILL.md specs
└── workflows/ci.yml                     CI pipeline
crates/
├── lra-cli/                             LRA scaffolding tool
└── mirr-wasm/                           WASM compilation target
mcp_server/                              MCP language server
_site/                              20   Static documentation site
fuzz/                                    Fuzzing harnesses
vscode-mirr/                             VS Code extension
```

## Compilation Pipeline Flow

```
.mirr source
  │
  ├─ lexer/        → Token stream
  ├─ parser/       → AST (ast/)
  ├─ typeck/       → Type-checked AST
  ├─ validation/   → Validated AST
  ├─ totality/     → Completeness-checked AST
  ├─ temporal/     → Temporal-lowered AST
  ├─ width/        → Width-inferred AST
  ├─ expand/       → Expanded AST
  ├─ simplify      → Simplified AST
  │
  └─ emit/
     ├─ verilog/       → .sv
     ├─ firrtl         → .fir
     ├─ rspu_asm       → .rasm
     ├─ rspu_encoding  → binary
     ├─ testbench      → _tb.sv
     ├─ json_emit      → .json
     ├─ dot            → .dot
     ├─ scaffold       → skeleton
     ├─ formal         → .sv + .sby
     └─ mape_k_rtl     → monitors
```
