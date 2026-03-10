//! S-expression emission backend.
//!
//! Thin wrapper that converts a `PipelineResult` into a pretty-printed
//! S-expression string. The heavy lifting is done by `sexpr::convert`
//! and `sexpr::printer`.

#![forbid(unsafe_code)]

use crate::pipeline::PipelineResult;
use crate::sexpr::convert::ast_to_sexpr;
use crate::sexpr::printer::print_sexpr;

/// Emit the parsed MIRR program as an S-expression.
///
/// Operates on the final typed AST (after expansion, type-checking,
/// and width inference).
pub fn emit_sexpr(result: &PipelineResult) -> String {
    let sexpr = ast_to_sexpr(&result.program);
    print_sexpr(&sexpr)
}
