//! Parser module for MIRR source code.
//!
//! Re-exports the module parser, expression parser, and pattern parser.

pub mod expr_parser;
pub mod module_parser;
pub mod pattern_parser;

pub use expr_parser::parse_expression;
pub use module_parser::parse_mirr;
pub use pattern_parser::{is_pattern_call_line, parse_pattern_call, parse_pattern_def};
