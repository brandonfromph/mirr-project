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
    /// Run SAT-based simplification after heuristic simplification.
    pub sat_simplify: bool,
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
    /// Run MAPE-K autonomic simulation (requires temporal).
    pub mape_k: bool,
    /// MAPE-K partition configuration (None = unified/default).
    pub mape_k_partition: Option<crate::mape_k::partition::PartitionConfig>,
    /// MAPE-K tick count override (None = use default 1024).
    pub mape_k_ticks: Option<u32>,
    /// Run register retiming optimization after temporal lowering.
    pub retiming: bool,
    /// Run MEGA-4 totality check after R-SPU emission.
    pub totality: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            typecheck: true,
            simplify: true,
            sat_simplify: false,
            width: true,
            temporal: true,
            rspu: false,
            extended_typecheck: false,
            simulate: false,
            mape_k: false,
            mape_k_partition: None,
            mape_k_ticks: None,
            retiming: false,
            totality: false,
        }
    }
}

/// Collected results from every pipeline stage.
pub struct PipelineResult {
    /// The parsed (and possibly simplified) program.
    pub program: MirrProgram,
    /// Simplification stats (None if stage was skipped).
    pub simplify_stats: Option<SimplifyStats>,
    /// SAT simplification stats (None if stage was skipped).
    pub sat_stats: Option<SatSimplifyPipelineStats>,
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
    /// MAPE-K simulation result (None if stage was skipped).
    pub mape_k_result: Option<crate::mape_k::MapeKResult>,
    /// Retiming optimization stats (None if stage was skipped).
    pub retiming_stats: Option<crate::temporal::retiming::RetimingStats>,
    /// MEGA-4 totality check result (None if stage was skipped).
    pub totality_result: Option<crate::totality::TotalityResult>,
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

    // Stage 3b: SAT-based simplification (optional, runs after heuristic).
    let sat_stats =
        if config.sat_simplify { Some(sat_simplify_program(&mut program)) } else { None };

    // Stage 4: Width inference (optional). Always includes SCC.
    let width_result = if config.width {
        Some(width::infer_program_widths_with_scc(&program, type_map.as_ref()))
    } else {
        None
    };

    // Stage 5: Temporal lowering (optional).
    let mut temporal_netlist = if config.temporal {
        let mut compiler = TemporalGuardCompiler::new();
        Some(compiler.compile_temporal_guards(&program.module)?)
    } else {
        None
    };

    // Stage 5b: Retiming optimization (optional, requires temporal).
    let retiming_stats = if config.retiming {
        if let Some(ref mut netlist) = temporal_netlist {
            let rconf = crate::temporal::retiming::RetimingConfig { enabled: true, max_passes: 4 };
            Some(crate::temporal::retiming::retime(netlist, &rconf))
        } else {
            None
        }
    } else {
        None
    };

    let mut result = PipelineResult {
        program,
        simplify_stats,
        sat_stats,
        width_result,
        temporal_netlist,
        rspu_program: None,
        type_map,
        extended_type_map,
        sim_result: None,
        mape_k_result: None,
        retiming_stats,
        totality_result: None,
    };

    // Stage 6: R-SPU emission (optional, requires temporal).
    if config.rspu {
        result.rspu_program = Some(crate::emit::rspu::emit_rspu(&result)?);
    }

    // Stage 6.5: Totality check (optional, requires rspu program).
    if config.totality {
        let totality = crate::totality::run_totality_check(&result.program.module);
        if let Some(ref mut prog) = result.rspu_program {
            if let Ok(binary) = crate::emit::rspu_encoding::emit_binary(prog) {
                let cert =
                    crate::cert::build_certificate(&totality, &binary, &result.program.module);
                if let Ok(bytes) = crate::cert::serialize_certificate(&cert) {
                    prog.certificate = Some(bytes);
                }
            }
        }
        result.totality_result = Some(totality);
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

    // Stage 8: MAPE-K autonomic simulation (optional).
    if config.mape_k {
        match crate::mape_k::bridge::bridge_from_pipeline(&result) {
            Ok(sim_config) => {
                let mut sim = crate::mape_k::MapeKSimulator::new(sim_config);
                let ticks = config.mape_k_ticks.map(|t| t.min(10_000) as u64).unwrap_or(1024);
                result.mape_k_result = Some(sim.run(ticks));
            }
            Err(_bridge_errors) => {
                // Bridge conversion failed — skip MAPE-K silently.
                // Errors are not fatal; the pipeline continues.
            }
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

/// Aggregate SAT simplification statistics for the full pipeline.
#[derive(Debug, Clone, Default)]
pub struct SatSimplifyPipelineStats {
    /// Total SAT equivalence checks performed.
    pub checks_performed: usize,
    /// Total checks that confirmed equivalence.
    pub equivalences_confirmed: usize,
    /// Whether any check hit solver bounds.
    pub had_unknown: bool,
}

/// Run SAT-based simplification on all guard conditions and reflex assignments.
///
/// Bounded: delegates to `sat::simplify_with_sat` which has internal bounds.
fn sat_simplify_program(program: &mut MirrProgram) -> SatSimplifyPipelineStats {
    let mut total = SatSimplifyPipelineStats::default();

    for g in &mut program.module.guards {
        let result = crate::sat::simplify_with_sat(g.condition.clone());
        g.condition = result.expr;
        total.checks_performed += result.checks_performed;
        total.equivalences_confirmed += result.equivalences_confirmed;
        total.had_unknown |= result.had_unknown;
    }

    for r in &mut program.module.reflexes {
        for a in &mut r.assignments {
            let result = crate::sat::simplify_with_sat(a.value.clone());
            a.value = result.expr;
            total.checks_performed += result.checks_performed;
            total.equivalences_confirmed += result.equivalences_confirmed;
            total.had_unknown |= result.had_unknown;
        }
    }

    total
}
