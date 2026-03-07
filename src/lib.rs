//! MIRR compiler — a safety-critical DSL for hardware-software co-design.
//!
//! MIRR compiles temporal behavioral specifications into SystemVerilog RTL,
//! JSON netlists, and Graphviz DOT graphs. The pipeline:
//! parse → validate → expand patterns → simplify → width inference → temporal compile → emit.
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
pub mod emit;
pub mod error;
pub mod expand;
pub mod lexer;
pub mod mape_k;
pub mod mirr_driver;
pub mod mirr_executor;
pub mod mirr_runtime;
pub mod parser;
pub mod pipeline;
pub mod simplify; // Adding the simplify module
pub mod temporal;
pub mod validation;
pub mod width;

// Top-level re-exports for ergonomic access.
pub use ast::pattern::{PatternDef, PatternOrigin};
pub use ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
pub use ast::{MirrAstJson, MirrProgram};
pub use bootstrap_runner::{BootstrapOpts, BootstrapResult, BootstrapRunner, StageResult};
pub use emit::json_netlist::JsonNetlist;
pub use error::MirrError;
pub use mape_k::{MapeKSimulator, SimConfig, SimResult};
pub use parser::parse_mirr;
pub use pipeline::{run_pipeline, PipelineConfig, PipelineResult};
pub use simplify::SimplifyStats;
pub use temporal::{
    low_level_ir::TemporalNetlist, low_level_ir::TemporalNetlistJson, TemporalGuardCompiler,
};
pub use validation::validate_module;
pub use width::types::{WidthDiag, WidthExpr, WidthStats};
