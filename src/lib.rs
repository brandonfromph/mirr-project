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
pub mod error;
pub mod lexer;
pub mod simplify; // Adding the simplify module
pub mod parser;
pub mod validation;
pub mod temporal;
pub mod mirr_runtime;
pub mod mirr_driver;
pub mod mirr_executor;

// Top-level re-exports for ergonomic access.
pub use ast::{MirrAstJson, MirrProgram};
pub use bootstrap_runner::{BootstrapOpts, BootstrapResult, BootstrapRunner, StageResult};
pub use error::MirrError;
pub use parser::parse_mirr;
pub use validation::validate_module;
pub use simplify::SimplifyStats;
pub use temporal::{TemporalGuardCompiler, low_level_ir::TemporalNetlist, low_level_ir::TemporalNetlistJson};
