//! Phase 6: Unified compilation pipeline.
//!
//! Orchestrates all compilation stages in sequence:
//!   parse -> validate -> simplify -> width_infer -> temporal_lower
//!
//! Each stage is gated by a config flag so partial pipelines can run
//! (e.g., parse+simplify only).

#![forbid(unsafe_code)]

use crate::ast::MirrProgram;
use crate::error::MirrError;
use crate::simplify::SimplifyStats;
use crate::temporal::low_level_ir::TemporalNetlist;
use crate::temporal::TemporalGuardCompiler;
use crate::width::{self, SccWidthResult};

/// Configuration for which pipeline stages to run.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Run Phase 3 simplification.
    pub simplify: bool,
    /// Run Phase 4 width inference.
    pub width: bool,
    /// Run Phase 2 temporal lowering.
    pub temporal: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            simplify: true,
            width: true,
            temporal: true,
        }
    }
}

/// Collected results from every pipeline stage.
pub struct PipelineResult {
    /// The parsed (and possibly simplified) program.
    pub program: MirrProgram,
    /// Simplification stats (None if stage was skipped).
    pub simplify_stats: Option<SimplifyStats>,
    /// Width inference results (None if stage was skipped).
    pub width_result: Option<SccWidthResult>,
    /// Temporal netlist (None if stage was skipped).
    pub temporal_netlist: Option<TemporalNetlist>,
}

impl PipelineResult {
    /// True if width inference produced any hard errors.
    pub fn has_width_errors(&self) -> bool {
        self.width_result.as_ref().is_some_and(|r| r.has_errors())
    }
}

/// Run the full compilation pipeline on MIRR source text.
///
/// Bounded: each stage is individually bounded by its own limits.
pub fn run_pipeline(source: &str, config: &PipelineConfig) -> Result<PipelineResult, MirrError> {
    // Stage 1: Parse.
    let mut program = crate::parser::parse_mirr(source)?;

    // Stage 2: Validate.
    crate::validation::validate_module(&program.module)?;

    // Stage 3: Simplify (optional).
    let simplify_stats = if config.simplify {
        Some(simplify_program(&mut program))
    } else {
        None
    };

    // Stage 4: Width inference (optional). Always includes SCC.
    let width_result = if config.width {
        Some(width::infer_program_widths_with_scc(&program))
    } else {
        None
    };

    // Stage 5: Temporal lowering (optional).
    let temporal_netlist = if config.temporal {
        let mut compiler = TemporalGuardCompiler::new();
        Some(compiler.compile_temporal_guards(&program.module)?)
    } else {
        None
    };

    Ok(PipelineResult {
        program,
        simplify_stats,
        width_result,
        temporal_netlist,
    })
}

/// Run Phase 3 simplification on all expressions in the program.
///
/// Returns aggregate stats. Bounded: iterates over guards + reflexes.
fn simplify_program(program: &mut MirrProgram) -> SimplifyStats {
    let mut total = SimplifyStats {
        rules_applied: 0,
        nodes_before: 0,
        nodes_after: 0,
    };

    for g in &mut program.module.guards {
        let (simplified, stats) = crate::simplify::simplify_expr_with_stats(g.condition.clone());
        g.condition = simplified;
        total.rules_applied += stats.rules_applied;
        total.nodes_before += stats.nodes_before;
        total.nodes_after += stats.nodes_after;
    }

    for r in &mut program.module.reflexes {
        for a in &mut r.assignments {
            let (simplified, stats) = crate::simplify::simplify_expr_with_stats(a.value.clone());
            a.value = simplified;
            total.rules_applied += stats.rules_applied;
            total.nodes_before += stats.nodes_before;
            total.nodes_after += stats.nodes_after;
        }
    }

    total
}
