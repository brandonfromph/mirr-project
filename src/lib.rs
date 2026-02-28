#![forbid(unsafe_code)]
#![deny(warnings)]

// ---------------------------------------------------------------------------
// MIRR Compiler Library — Public API
// ---------------------------------------------------------------------------
// NASA/JPL Rule: Minimal public surface area. External consumers and main.rs
// import only what they need through clean re-exports.
// ---------------------------------------------------------------------------

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod validation;

// Top-level re-exports for ergonomic access.
pub use ast::MirrProgram;
pub use error::MirrError;
pub use parser::parse_mirr;
pub use validation::validate_module;