//! S-expression emission backend.
//!
//! Thin wrapper that converts a `PipelineResult` into a pretty-printed
//! S-expression string. The heavy lifting is done by `sexpr::convert`
//! and `sexpr::printer`.

#![forbid(unsafe_code)]

use crate::pipeline::PipelineResult;

use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};

/// Emit the parsed MIRR program as an S-expression.
///
/// Operates on the final typed AST (after expansion, type-checking,
/// and width inference).
pub fn emit_sexpr(result: &PipelineResult) -> Result<String, MirrError> {
    let registry = result.ecs_registry.as_ref().ok_or_else(|| {
        mirrcode(ErrorCode::SExprFallback, "ECS registry required for S-expression emission")
    })?;
    let name = registry.get_module_name().unwrap_or_else(|| "unknown".to_string());

    let mut signal_parts = Vec::new();
    for i in 0..registry.kinds.len() {
        if let (Some(name_comp), Some(kind_comp)) = (&registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::EntityKind::SIGNAL(_) = kind_comp.0 {
                signal_parts.push(format!("(signal {})", registry.resolve_name(name_comp.0)));
            }
        }
    }

    Ok(format!("(ecs-program (module {} {}))", name, signal_parts.join(" ")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::{MirrProgram, Module};

    #[test]
    fn test_emit_sexpr_empty_module() {
        let _program = MirrProgram {
            target: None,
            patterns: vec![],
            imports: vec![],
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
        let mut reg = crate::ecs::Registry::new();
        crate::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut reg, "module empty {}", None).unwrap();
        let result = PipelineResult {
            program: None,
            simplify_stats: None,
            sat_stats: None,
            width_stats: None,
            width_diagnostics: Vec::new(),
            temporal_netlist: None,
            rspu_program: None,
            extended_type_map: None,
            sim_result: None,
            mape_k_result: None,
            retiming_stats: None,

            totality_result: None,
            symbolic_result: None,
            mape_k_rtl: None,
            hls_result: None,
            ecs_registry: Some(reg),
            file_table: crate::span::FileTable::new(),
        };
        let output = emit_sexpr(&result).expect("Failed to emit sexpr");
        assert!(output.contains("module"));
        assert!(output.contains("empty"));
    }
}
