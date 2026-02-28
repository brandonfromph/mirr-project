// ---------------------------------------------------------------------------
// Parser module — public interface
// ---------------------------------------------------------------------------

pub mod expr_parser;
pub mod module_parser;

pub use expr_parser::parse_expression;
pub use module_parser::parse_mirr;