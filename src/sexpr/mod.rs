//! S-expression intermediate representation for MIRR.
//!
//! Provides homoiconic encoding of MIRR programs as S-expressions,
//! enabling code-as-data transformations, hygienic macros, and
//! bounded compile-time evaluation.
//!
//! ## Module Structure
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `types` | `SExpr` enum and helper constructors |
//! | `parser` | Bounded S-expression text parser |
//! | `printer` | Bounded pretty-printer |
//! | `convert` | Bidirectional AST ↔ S-expression conversion |
//! | `eval` | Bounded eval/apply core (compile-time only) |
//! | `macro_expand` | Hygienic macro expander |
//! | `reader` | Reader macro registry |

#![forbid(unsafe_code)]

pub mod convert;
pub mod eval;
pub mod macro_expand;
pub mod parser;
pub mod printer;
pub mod reader;
pub mod types;

// Re-exports for convenience.
pub use convert::{ast_to_sexpr, sexpr_to_ast};
pub use eval::{eval, EvalState};
pub use macro_expand::MacroExpander;
pub use parser::parse_sexpr;
pub use printer::print_sexpr;
pub use reader::ReaderMacroRegistry;
pub use types::SExpr;

/// Maximum nesting depth for S-expression parsing/printing.
/// NASA Power-of-10: all recursion replaced with bounded iteration.
pub const MAX_SEXPR_DEPTH: usize = 64;

/// Maximum number of nodes in a single S-expression tree.
/// NASA Power-of-10: all collections bounded.
pub const MAX_SEXPR_NODES: usize = 4096;

/// Maximum input string length for S-expression parsing.
pub const MAX_SEXPR_STRING_LEN: usize = 1_048_576; // 1 MB

/// Maximum evaluation depth for the eval/apply core.
pub const MAX_EVAL_DEPTH: usize = 32;

/// Maximum evaluation steps for the eval/apply core.
pub const MAX_EVAL_STEPS: usize = 10_000;

/// Maximum depth for hygienic macro expansion.
pub const MAX_MACRO_EXPAND_DEPTH: usize = 8;

/// Maximum number of registered reader macros.
pub const MAX_READER_MACROS: usize = 32;
