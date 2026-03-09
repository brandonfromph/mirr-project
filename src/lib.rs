//! MIRR compiler — a safety-critical DSL for hardware-software co-design.
//!
//! MIRR compiles temporal behavioral specifications into SystemVerilog RTL,
//! FIRRTL, JSON netlists, Graphviz DOT graphs, and R-SPU assembly. The pipeline:
//!
//! 1. **Parse** — lexer + recursive-descent parser → AST
//! 2. **Validate patterns** — pattern arity and parameter checks
//! 3. **Expand patterns** — inline pattern instantiations
//! 4. **Validate module** — semantic checks (E2xx)
//! 5. **Typecheck** — signedness consistency (E6xx, optional)
//! 6. **Simplify** — constant folding and identity reduction
//! 7. **Width inference** — SCC-based constraint solving (E5xx)
//! 8. **Temporal compile** — guards → shift registers / counters
//! 9. **Emit** — Verilog, FIRRTL, JSON, DOT, or R-SPU assembly
//!
//! See the [README](https://github.com/brandonfromph/mirr-project) for language documentation.

#![forbid(unsafe_code)]
#![deny(warnings)]

// ---------------------------------------------------------------------------
// MIRR Compiler Library — Public API
// ---------------------------------------------------------------------------
// NASA/JPL Rule: Minimal public surface area. External consumers and main.rs
// import only what they need through clean re-exports.
// ---------------------------------------------------------------------------

pub mod ast;
pub mod bootstrap_runner;
pub mod diagnostic;
pub mod emit;
pub mod error;
pub mod expand;
pub mod lexer;
pub mod lsp;
pub mod mape_k;
pub mod mirr_driver;
pub mod mirr_executor;
pub mod mirr_runtime;
pub mod parser;
pub mod pipeline;
pub mod simplify;
pub mod span;
pub mod suggest;
pub mod temporal;
pub mod toolchain;
pub mod typeck;
pub mod validation;
pub mod width;

// Top-level re-exports for ergonomic access.
pub use ast::pattern::{PatternDef, PatternOrigin};
pub use ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
pub use ast::{MirrAstJson, MirrProgram};
pub use bootstrap_runner::{BootstrapOpts, BootstrapResult, BootstrapRunner, StageResult};
pub use emit::json_netlist::JsonNetlist;
pub use emit::rspu_isa::{RspuInstruction, RspuProgram};
pub use error::MirrError;
pub use error::PipelineErrors;
pub use mape_k::{MapeKSimulator, SimConfig, SimResult};
pub use parser::parse_mirr;
pub use pipeline::{run_pipeline, PipelineConfig, PipelineResult};
pub use simplify::SimplifyStats;
pub use temporal::{
    low_level_ir::TemporalNetlist, low_level_ir::TemporalNetlistJson, TemporalGuardCompiler,
};
pub use typeck::typecheck_module;
pub use validation::validate_module;
pub use width::types::{WidthDiag, WidthExpr, WidthStats};
