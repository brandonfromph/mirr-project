//! Parser module for MIRR source code.
//!
//! Re-exports the module parser, expression parser, and pattern parser.

#![forbid(unsafe_code)]

pub mod expr_parser;
pub mod module_parser;
pub mod pattern_parser;

pub use expr_parser::parse_expression;
pub use module_parser::parse_mirr;
pub use pattern_parser::{is_pattern_call_line, parse_pattern_call, parse_pattern_def};

/// Skip empty lines and comment lines in a line array.
/// Used by module_parser and pattern_parser.
pub(crate) fn skip_empty_and_comments(lines: &[&str], index: &mut usize) {
    while *index < lines.len() {
        let line = lines[*index].trim();
        if line.is_empty() || line.starts_with("//") {
            *index += 1;
        } else {
            break;
        }
    }
}
