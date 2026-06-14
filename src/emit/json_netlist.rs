//! JSON netlist emitter for MIRR IR.
//!
//! Serializes the compiled IR (post-simplify, post-width) as a
//! machine-readable JSON artifact for downstream tools.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::TemporalNetlistJson;

/// Top-level JSON netlist structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonNetlist {
    /// Bump when JSON netlist schema changes. See CHANGELOG.md.
    pub schema_version: String,
    /// IR version for contract tracking.
    pub ir_version: String,
    /// The ECS Registry containing all IR.
    pub ecs_registry: crate::ecs::Registry,
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
    /// Number of simplification rules applied.
    pub rules_applied: usize,
    /// Expression node count before simplification.
    pub nodes_before: usize,
    /// Expression node count after simplification.
    pub nodes_after: usize,
}

/// Serializable wrapper for WidthStats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthStatsJson {
    /// Total expression nodes analyzed.
    pub nodes_analyzed: usize,
    /// Constraint propagation iterations to fixpoint.
    pub propagation_rounds: usize,
    /// Number of width diagnostics emitted.
    pub diagnostics_count: usize,
    /// Number of strongly connected components detected.
    pub scc_count: usize,
    /// Number of expansive SCCs (contain Add/Mul/Shl).
    pub expansive_count: usize,
    /// Number of non-expansive SCCs (Prev-only or bitwise).
    pub nonexpansive_count: usize,
    /// True if any diagnostic is a hard error.
    pub has_errors: bool,
}

/// Emit a JSON netlist string from pipeline results.
pub fn emit_json(result: &PipelineResult) -> Result<String, serde_json::Error> {
    let netlist = build_netlist(result);
    serde_json::to_string_pretty(&netlist)
}

/// Build the JsonNetlist structure from pipeline results.
pub fn build_netlist(result: &PipelineResult) -> JsonNetlist {
    let ecs_registry = result.ecs_registry.clone().unwrap_or_default();

    let simplify_stats = result.simplify_stats.as_ref().map(|s| SimplifyStatsJson {
        rules_applied: s.rules_applied,
        nodes_before: s.nodes_before,
        nodes_after: s.nodes_after,
    });

    let width_stats = result.width_stats.as_ref().map(|w| WidthStatsJson {
        nodes_analyzed: w.nodes_analyzed,
        propagation_rounds: w.propagation_rounds,
        diagnostics_count: w.diagnostics_count,
        scc_count: w.scc_count,
        expansive_count: w.expansive_count,
        nonexpansive_count: w.nonexpansive_count,
        has_errors: w.diagnostics_count > 0,
    });

    let temporal = result
        .temporal_netlist
        .as_ref()
        .map(crate::temporal::low_level_ir::TemporalNetlistJson::from_netlist);

    JsonNetlist {
        schema_version: "0.3.0".to_string(),
        ir_version: crate::ast::types::IR_VERSION.to_string(),
        ecs_registry,
        simplify_stats,
        width_stats,
        temporal,
    }
}
