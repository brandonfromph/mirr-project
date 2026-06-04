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
pub mod cert;
pub mod cross_surface_stress;
pub mod diagnostic;
pub mod diagnostic_builder;
pub mod ecs;
pub mod emit;
pub mod error;
pub mod error_codes;
pub mod expand;
pub mod hls;
pub mod lexer;
pub mod lsp;
pub mod lsp_bridge;
pub mod lsp_incremental;
pub mod mape_k;
pub mod mirr_daemon;
pub mod mirr_daemon_security;
pub mod mirr_driver;
pub mod mirr_executor;
pub mod mirr_runtime;
pub mod mrt_auth;
pub mod mrt_host;
pub mod mrt_schema;
pub mod parser;
pub mod pipeline;
pub mod sat;
pub mod sexpr;
pub mod simplify;
pub mod span;
pub mod suggest;
pub mod symbolic;
pub mod temporal;
pub mod toolchain;
pub mod totality;
pub mod typeck;
pub mod validation;
pub mod width;
pub mod workspace;
pub mod zero_debt_closeout;

// Top-level re-exports for ergonomic access.
pub use ast::pattern::{PatternDef, PatternOrigin};
pub use ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
pub use ast::{MirrAstJson, MirrProgram};
pub use bootstrap_runner::{BootstrapOpts, BootstrapResult, BootstrapRunner, StageResult};
pub use diagnostic_builder::{emit as emit_diag, emit_at, MirrDiagnostic};
pub use emit::json_netlist::JsonNetlist;
pub use emit::rspu_encoding::{decode, emit_binary, encode, EncodedInstruction};
pub use emit::rspu_isa::{RspuInstruction, RspuProgram};
pub use emit::rspu_sim::RspuSimulator;
pub use emit::rspu_tagged::{RegisterFile, TaggedWord, TypeTag};
pub use error::MirrError;
pub use error::PipelineErrors;
pub use error_codes::{mirrcode, ErrorCode};
pub use mape_k::{MapeKResult, MapeKSimulator, SimConfig};
pub use parser::parse_mirr;
pub use pipeline::{run_pipeline, PipelineConfig, PipelineResult};
pub use simplify::SimplifyStats;
pub use temporal::{
    low_level_ir::TemporalNetlist, low_level_ir::TemporalNetlistJson, TemporalGuardCompiler,
};
pub use typeck::extended::typecheck_extended;
pub use typeck::{typecheck_module, typecheck_module_with_mode, TypecheckMode};
pub use validation::validate_module;
pub use width::types::{WidthDiag, WidthExpr, WidthStats};

// MEGA-4: Totality Engine re-exports.
pub use cert::{verify_certificate, ProofCertificate, TerminationStrategy};
pub use totality::{run_totality_check, TotalityResult};
pub use workspace::{
    Workspace, WorkspaceArtifactSummary, WorkspaceConfig, WorkspaceError, WorkspaceSnapshot,
};
