#![allow(dead_code)]
//! Bidirectional AST <-> S-expression conversion.
//!
//! For all parseable MIRR programs:
//! `parse_mirr(source) == sexpr_to_ast(ast_to_sexpr(parse_mirr(source)))`

#![forbid(unsafe_code)]

mod from_sexpr;
mod parse_expr;
mod to_sexpr;

/// Maximum nesting depth for expression conversion/parsing (NASA Power-of-10).
const MAX_CONVERT_DEPTH: usize = 64;

pub use from_sexpr::sexpr_to_registry;
pub use to_sexpr::registry_to_sexpr;
