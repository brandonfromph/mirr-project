//! Phase 6: Unified compilation pipeline.
//!
//! Orchestrates all compilation stages in sequence:
//!   parse -> validate -> typecheck -> simplify -> width_infer -> temporal_lower
//!
//! Each stage is gated by a config flag so partial pipelines can run
//! (e.g., parse+simplify only).

#![forbid(unsafe_code)]

use crate::ast::MirrProgram;
use crate::emit::rspu_isa::RspuProgram;
use crate::error::PipelineErrors;
use crate::simplify::SimplifyStats;
use crate::temporal::low_level_ir::TemporalNetlist;
use crate::temporal::TemporalGuardCompiler;
use crate::width::{self, SccWidthResult};

/// Configuration for which pipeline stages to run.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Run type checking after validation.
    pub typecheck: bool,
    /// Run Phase 3 simplification.
    pub simplify: bool,
    /// Run Phase 4 width inference.
    pub width: bool,
    /// Run Phase 2 temporal lowering.
    pub temporal: bool,
    /// Run R-SPU instruction emission (requires temporal).
    pub rspu: bool,
    /// Run MEGA-1 extended type checking (opt-in).
    pub extended_typecheck: bool,
    /// Run R-SPU ISA simulator after emission (requires rspu).
    pub simulate: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            typecheck: true,
            simplify: true,
            width: true,
            temporal: true,
            rspu: false,
            extended_typecheck: false,
            simulate: false,
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
    /// R-SPU program (None if stage was skipped).
    pub rspu_program: Option<RspuProgram>,
    /// Expression type map from the type checker (None if stage was skipped).
    pub type_map: Option<crate::typeck::TypeMap>,
    /// Extended type map from MEGA-1 checker (None if stage was skipped).
    pub extended_type_map: Option<crate::typeck::extended::ExtendedTypeMap>,
    /// ISA simulation result (None if stage was skipped).
    pub sim_result: Option<crate::emit::rspu_sim::SimResult>,
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
pub fn run_pipeline(
    source: &str,
    config: &PipelineConfig,
) -> Result<PipelineResult, PipelineErrors> {
    // Stage 1: Parse.
    let mut program = crate::parser::parse_mirr(source)?;

    // Stage 1.5: Validate pattern definitions, then expand pattern calls.
    // This runs BEFORE module validation so expanded items are validated as
    // part of the normal module (the emission pipeline never sees patterns).
    crate::validation::validate_pattern_defs(&program.patterns)?;
    crate::expand::expand_patterns(&mut program)?;

    // Stage 2: Validate.
    crate::validation::validate_module(&program.module)?;

    // Stage 2.5: Type-check (optional).
    let type_map = if config.typecheck {
        Some(crate::typeck::typecheck_module(&program.module)?)
    } else {
        None
    };

    // Stage 2.6: Extended type checking (opt-in MEGA-1).
    let extended_type_map = if config.extended_typecheck {
        let extended_decls: Vec<crate::typeck::extended::ExtendedSignalDecl> = program
            .module
            .signals
            .iter()
            .map(crate::typeck::extended::ExtendedSignalDecl::from_legacy)
            .collect();
        let ext_result = crate::typeck::extended::typecheck_extended(
            &program.module,
            &extended_decls,
            &[],
            &[],
            &[],
        );
        if !ext_result.errors.is_empty() {
            return Err(ext_result.errors);
        }
        Some(ext_result.type_map)
    } else {
        None
    };

    // Stage 3: Simplify (optional).
    let simplify_stats = if config.simplify { Some(simplify_program(&mut program)) } else { None };

    // Stage 4: Width inference (optional). Always includes SCC.
    let width_result = if config.width {
        Some(width::infer_program_widths_with_scc(&program, type_map.as_ref()))
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

    let mut result = PipelineResult {
        program,
        simplify_stats,
        width_result,
        temporal_netlist,
        rspu_program: None,
        type_map,
        extended_type_map,
        sim_result: None,
    };

    // Stage 6: R-SPU emission (optional, requires temporal).
    if config.rspu {
        result.rspu_program = Some(crate::emit::rspu::emit_rspu(&result)?);
    }

    // Stage 7: ISA simulation (optional, requires rspu program).
    if config.simulate {
        if let Some(ref prog) = result.rspu_program {
            use crate::emit::rspu_isa::MAX_SIM_CYCLES;
            use crate::emit::rspu_sim::RspuSimulator;
            let mut sim = RspuSimulator::new();
            let sim_out =
                sim.run(prog, MAX_SIM_CYCLES).map_err(crate::error::PipelineErrors::from)?;
            result.sim_result = Some(sim_out);
        }
    }

    Ok(result)
}

/// Run Phase 3 simplification on all expressions in the program.
///
/// Returns aggregate stats. Bounded: iterates over guards + reflexes.
fn simplify_program(program: &mut MirrProgram) -> SimplifyStats {
    let mut total = SimplifyStats { rules_applied: 0, nodes_before: 0, nodes_after: 0 };

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

    simplify_properties(&mut program.module.properties, &mut total);

    total
}

/// Simplify expressions inside property formulas.
fn simplify_properties(
    properties: &mut [crate::ast::property::PropertyDecl],
    total: &mut SimplifyStats,
) {
    for p in properties.iter_mut() {
        for expr in p.formula.exprs_mut() {
            simplify_one(expr, total);
        }
    }
}

fn simplify_one(expr: &mut crate::ast::Expr, total: &mut SimplifyStats) {
    let (simplified, stats) = crate::simplify::simplify_expr_with_stats(expr.clone());
    *expr = simplified;
    total.rules_applied += stats.rules_applied;
    total.nodes_before += stats.nodes_before;
    total.nodes_after += stats.nodes_after;
}
