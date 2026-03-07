//! JSON netlist emitter for MIRR IR.
//!
//! Serializes the compiled IR (post-simplify, post-width) as a
//! machine-readable JSON artifact for downstream tools.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::ast::MirrAstJson;
use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::TemporalNetlistJson;

/// Top-level JSON netlist structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonNetlist {
    /// IR version for contract tracking.
    pub ir_version: String,
    /// The compiled program AST (post-simplification).
    pub program: MirrAstJson,
    /// Simplification statistics (null if skipped).
    pub simplify_stats: Option<SimplifyStatsJson>,
    /// Width inference statistics (null if skipped).
    pub width_stats: Option<WidthStatsJson>,
    /// Temporal netlist (null if skipped).
    pub temporal: Option<TemporalNetlistJson>,
}

/// Serializable wrapper for SimplifyStats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifyStatsJson {
    pub rules_applied: usize,
    pub nodes_before: usize,
    pub nodes_after: usize,
}

/// Serializable wrapper for WidthStats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthStatsJson {
    pub nodes_analyzed: usize,
    pub propagation_rounds: usize,
    pub diagnostics_count: usize,
    pub scc_count: usize,
    pub expansive_count: usize,
    pub nonexpansive_count: usize,
    pub has_errors: bool,
}

/// Emit a JSON netlist string from pipeline results.
pub fn emit_json(result: &PipelineResult) -> Result<String, serde_json::Error> {
    let netlist = build_netlist(result);
    serde_json::to_string_pretty(&netlist)
}

/// Build the JsonNetlist structure from pipeline results.
pub fn build_netlist(result: &PipelineResult) -> JsonNetlist {
    let program = MirrAstJson::from_program(&result.program);

    let simplify_stats = result.simplify_stats.as_ref().map(|s| SimplifyStatsJson {
        rules_applied: s.rules_applied,
        nodes_before: s.nodes_before,
        nodes_after: s.nodes_after,
    });

    let width_stats = result.width_result.as_ref().map(|w| WidthStatsJson {
        nodes_analyzed: w.stats.nodes_analyzed,
        propagation_rounds: w.stats.propagation_rounds,
        diagnostics_count: w.stats.diagnostics_count,
        scc_count: w.stats.scc_count,
        expansive_count: w.stats.expansive_count,
        nonexpansive_count: w.stats.nonexpansive_count,
        has_errors: w.has_errors(),
    });

    let temporal = result.temporal_netlist.as_ref().map(|n| {
        crate::temporal::low_level_ir::TemporalNetlistJson::from_netlist(n)
    });

    JsonNetlist {
        ir_version: "1.0".to_string(),
        program,
        simplify_stats,
        width_stats,
        temporal,
    }
}
