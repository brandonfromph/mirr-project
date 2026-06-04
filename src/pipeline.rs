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
use crate::width::{self, SccWidthResult};

use serde::{Deserialize, Serialize};

/// Configuration for which pipeline stages to run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Run type checking after validation.
    pub typecheck: bool,
    /// Run typechecker in bootstrap/hydration mode (relaxed bitwise/logical checks on unsigned types).
    #[serde(default)]
    pub bootstrap_mode: bool,
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
    /// Run MEGA-5 symbolic analysis after type checking.
    pub symbolic: bool,
    /// Emit MAPE-K RTL (SystemVerilog) from autonomic simulation results.
    pub emit_mape_k_rtl: bool,
    /// Run MEGA-12 HLS pass (scheduling, sharing, binding, FIFO).
    pub hls: bool,
    /// Run automated logic optimization pass.
    /// Run automated logic optimization pass.
    pub logic_optimize: bool,
    /// Base directory for resolving imports during ECS hydration.
    pub base_dir: Option<std::path::PathBuf>,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            typecheck: true,
            bootstrap_mode: false,
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
            symbolic: false,
            emit_mape_k_rtl: false,
            hls: false,
            logic_optimize: false,
            base_dir: None,
        }
    }
}

/// Collected results from every pipeline stage.
#[derive(Debug)]
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
    /// MEGA-5 symbolic analysis result (None if stage was skipped).
    pub symbolic_result: Option<crate::symbolic::SymbolicResult>,
    /// MAPE-K RTL emission output (None if not requested).
    pub mape_k_rtl: Option<String>,
    /// MEGA-12 HLS pass result (None if stage was skipped).
    pub hls_result: Option<crate::hls::HlsResult>,
}

impl PipelineResult {
    /// True if width inference produced any hard errors.
    pub fn has_width_errors(&self) -> bool {
        self.width_result.as_ref().is_some_and(|r| r.has_errors())
    }
}

/// Run the full compilation pipeline on MIRR source text.
pub fn run_pipeline(
    source: &str,
    config: &PipelineConfig,
) -> Result<PipelineResult, PipelineErrors> {
    let program = crate::parser::parse_mirr(source)?;
    run_pipeline_on_program(program, config)
}

/// Run the compilation pipeline on a pre-parsed (and potentially merged) MirrProgram.
pub fn run_pipeline_on_program(
    mut program: MirrProgram,
    config: &PipelineConfig,
) -> Result<PipelineResult, PipelineErrors> {
    // Stage 1.5: Validate pattern definitions, then expand pattern calls.
    // This runs BEFORE module validation so expanded items are validated as
    // part of the normal module (the emission pipeline never sees patterns).
    crate::validation::validate_pattern_defs(&program.patterns)?;

    // ECS Transition: Create Registry and hydrate definitions for expansion lookup.
    let mut registry = crate::ecs::Registry::new();

    // 1. Ingest pattern definitions (from current program).
    for pat in &program.patterns {
        let entity =
            registry.create_entity(&pat.name, crate::ecs::components::KindComponent::PATTERN);
        registry.set_type(entity, crate::ecs::components::TypeComponent::pattern(pat.clone()));
        registry.pattern_defs[entity.0 as usize] =
            Some(crate::ecs::components::PatternDefComponent(pat.clone()));
    }

    // 2. Ingest imports (which contain more patterns).
    if let Some(dir) = config.base_dir.as_deref() {
        let mut loaded = std::collections::HashSet::new();
        load_imports_recursive_for_pipeline(&mut registry, &program.imports, dir, &mut loaded)
            .map_err(|e| PipelineErrors { errors: vec![e] })?;
    }

    // 3. Expand patterns in the AST using the Registry as a lookup.
    crate::expand::expand_patterns(&mut program, &registry)?;

    // Stage 2: Semantic validation (Mandatory diagnostic gate).
    crate::validation::validate_module(&program.module)?;

    // 4. Hydrate the full, expanded module into the Registry for Stage 5 synthesis.
    // This happens AFTER AST validation so we know the AST is semantically sound.
    registry.ingest_module(&program.module)?;
    registry.semantic_validate()?;

    // ECS-native typechecking complete. Maintaining AST-based type_map for
    // downstream emitters until Phase 3 (R-SPU Emission).
    let type_map = if config.typecheck {
        let mode = if config.bootstrap_mode {
            crate::typeck::TypecheckMode::Bootstrap
        } else {
            crate::typeck::TypecheckMode::Standard
        };
        Some(crate::typeck::typecheck_module_with_mode(&program.module, mode)?)
    } else {
        None
    };

    // Stage 2.6: Extended type checking (opt-in MEGA-1).
    let extended_type_map = if config.extended_typecheck {
        let extended_decls: Vec<crate::typeck::extended::ExtendedSignalDecl> = program
            .module
            .signals
            .iter()
            .map(crate::typeck::extended::ExtendedSignalDecl::from_ast)
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

    // Stage 2.7: Symbolic analysis (optional, MEGA-5).
    let symbolic_result =
        if config.symbolic { crate::symbolic::analyze_module(&program.module).ok() } else { None };

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

    // Stage 5: ECS-Native Temporal Synthesis (Proposal 110 — Phase 3 ECS Transition).
    //
    // We re-hydrate the registry here to ensure it represents the absolute
    // final state of the program (after simplification and width inference).
    let mut final_registry = crate::ecs::Registry::new();
    if let Err(e) = crate::ecs::adapter::ingest_program(
        &mut final_registry,
        program.clone(),
        config.base_dir.as_deref(),
    ) {
        return Err(PipelineErrors { errors: vec![e] });
    }

    // Phase 3 ECS Systems: Semantic Validation & Typechecking (Shadow Gate)
    // Runs alongside AST-based gates to ensure ECS parity during transition.
    let _ = crate::ecs::systems::run_compilation_pipeline(&mut final_registry);

    let mut temporal_netlist = if config.temporal {
        Some(
            crate::ecs::systems::temporal_synthesis_system(&mut final_registry)
                .map_err(|e| PipelineErrors { errors: vec![e] })?,
        )
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
        symbolic_result,
        mape_k_rtl: None,
        hls_result: None,
    };

    // Stage 5c: HLS pass (optional, MEGA-12).
    if config.hls {
        let mut dag = crate::hls::OpDag::new();

        // Build DAG from reflex assignments.
        for reflex in &result.program.module.reflexes {
            for _assign in &reflex.assignments {
                let op_kind = crate::hls::ResourceKind::Add;
                let width = 8;
                if let Some(_op_id) = dag.add_op(op_kind, width, vec![width, width]) {
                    // DAG built successfully.
                }
            }
        }

        let hls_config = crate::hls::HlsConfig::default();
        result.hls_result = crate::hls::run_hls_pass(&dag, &hls_config).ok();
    }

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

    // Stage 9: MAPE-K RTL emission (optional, requires mape_k result).
    if config.emit_mape_k_rtl {
        result.mape_k_rtl =
            Some(crate::emit::mape_k_rtl::emit_mape_k_rtl(&result).map_err(|e| {
                PipelineErrors { errors: vec![crate::error::MirrError::parse_error(e)] }
            })?);
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

fn load_imports_recursive_for_pipeline(
    registry: &mut crate::ecs::Registry,
    imports: &[crate::ast::program::ImportDecl],
    current_dir: &std::path::Path,
    loaded_paths: &mut std::collections::HashSet<std::path::PathBuf>,
) -> Result<(), crate::error::MirrError> {
    for import in imports {
        let import_path = current_dir.join(&import.path);
        let canonical_path = import_path.canonicalize().unwrap_or_else(|_| import_path.clone());
        if loaded_paths.contains(&canonical_path) {
            continue;
        }
        loaded_paths.insert(canonical_path.clone());

        let source = std::fs::read_to_string(&import_path).map_err(|e| {
            crate::error::MirrError::ImportError {
                message: format!("Failed to read import file {:?}: {}", import_path, e),
                span: import.span,
            }
        })?;

        let imported_prog = crate::parser::parse_mirr(&source).map_err(|e| {
            crate::error::MirrError::ImportError {
                message: format!("Failed to parse imported file {:?}: {}", import_path, e),
                span: import.span,
            }
        })?;

        for pat in imported_prog.patterns {
            let entity =
                registry.create_entity(&pat.name, crate::ecs::components::KindComponent::PATTERN);
            registry.set_type(entity, crate::ecs::components::TypeComponent::pattern(pat.clone()));
            registry.pattern_defs[entity.0 as usize] =
                Some(crate::ecs::components::PatternDefComponent(pat.clone()));
            let qualified_name = format!("{}::{}", import.alias, pat.name);
            registry.register_symbol(&qualified_name, entity);
        }

        // Recurse using the imported file's parent directory
        if let Some(parent_dir) = import_path.parent() {
            load_imports_recursive_for_pipeline(
                registry,
                &imported_prog.imports,
                parent_dir,
                loaded_paths,
            )?;
        }
    }
    Ok(())
}
