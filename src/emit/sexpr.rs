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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::{MirrProgram, Module};

    #[test]
    fn test_emit_sexpr_empty_module() {
        let program = MirrProgram {
            patterns: vec![],
            module: Module {
                name: "empty".to_string(),
                signals: vec![],
                guards: vec![],
                reflexes: vec![],
                pattern_calls: vec![],
                pattern_origins: vec![],
                properties: vec![],
                span: None,
            },
        };
        let result = PipelineResult {
            program,
            simplify_stats: None,
            sat_stats: None,
            width_result: None,
            temporal_netlist: None,
            rspu_program: None,
            type_map: None,
            extended_type_map: None,
            sim_result: None,
            mape_k_result: None,
            retiming_stats: None,

            totality_result: None,
            symbolic_result: None,
            mape_k_rtl: None,
        };
        let output = emit_sexpr(&result);
        assert!(output.contains("module"));
        assert!(output.contains("empty"));
    }
}
