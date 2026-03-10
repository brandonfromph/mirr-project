//! mirr-compile — Unified MIRR compilation driver (Phase 6).
//!
//! End-to-end pipeline: parse -> validate -> simplify -> width -> temporal -> emit.
//!
//! Usage:
//!   mirr-compile <file.mirr> [--emit dot|verilog|json|sva|firrtl|rspu|testbench|scaffold] [--output FILE] [--stats]
//!   mirr-compile <file.mirr> --emit verilog --target xilinx-7 --testbench --scaffold
//!   mirr-compile <file.mirr> --emit dot --dot-detail expr [--output FILE]

#![forbid(unsafe_code)]

use std::process;

use nasa_rust_project::emit;
use nasa_rust_project::emit::fpga_target::FpgaTarget;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut input_path: Option<String> = None;
    let mut emit_format: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut show_stats = false;
    let mut show_help = false;
    let mut dot_detail_expr = false;
    let mut target_name: Option<String> = None;
    let mut sync_stages: u32 = 2;
    let mut dsp_threshold: u32 = nasa_rust_project::emit::dsp::DEFAULT_DSP_THRESHOLD;
    let mut emit_testbench = false;
    let mut emit_scaffold = false;
    let mut strip_sva = false;
    let mut sva_file: Option<String> = None;
    let mut formal = false;
    let mut formal_depth: u32 = 20;
    let mut formal_prove = false;
    let mut formal_engine: String = "z3".to_string();
    let mut lint = false;
    let mut simulate = false;
    let mut pnr = false;
    let mut timing = false;
    let mut eqy = false;
    let mut toolchain_path: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--emit" => {
                i += 1;
                if i < args.len() {
                    emit_format = Some(args[i].clone());
                }
            }
            "--output" | "-o" => {
                i += 1;
                if i < args.len() {
                    output_path = Some(args[i].clone());
                }
            }
            "--dot-detail" => {
                i += 1;
                if i < args.len() && args[i] == "expr" {
                    dot_detail_expr = true;
                }
            }
            "--target" => {
                i += 1;
                if i < args.len() {
                    target_name = Some(args[i].clone());
                }
            }
            "--sync-stages" => {
                i += 1;
                if i < args.len() {
                    sync_stages = args[i].parse().unwrap_or(2);
                }
            }
            "--dsp-threshold" => {
                i += 1;
                if i < args.len() {
                    dsp_threshold = args[i].parse().unwrap_or(dsp_threshold);
                }
            }
            "--testbench" => emit_testbench = true,
            "--scaffold" => emit_scaffold = true,
            "--strip-sva" => strip_sva = true,
            "--sva-file" => {
                i += 1;
                if i < args.len() {
                    sva_file = Some(args[i].clone());
                }
            }
            "--stats" => show_stats = true,
            "--formal" => formal = true,
            "--formal-depth" => {
                i += 1;
                if i < args.len() {
                    formal_depth = args[i].parse().unwrap_or(20);
                }
            }
            "--formal-prove" => formal_prove = true,
            "--formal-engine" => {
                i += 1;
                if i < args.len() {
                    formal_engine = args[i].clone();
                }
            }
            "--lint" => lint = true,
            "--simulate" => simulate = true,
            "--pnr" => pnr = true,
            "--timing" => timing = true,
            "--eqy" => eqy = true,
            "--toolchain-path" => {
                i += 1;
                if i < args.len() {
                    toolchain_path = Some(args[i].clone());
                }
            }
            "--help" | "-h" => show_help = true,
            other => {
                if other.starts_with('-') {
                    eprintln!("Unknown option: {other}");
                    process::exit(1);
                }
                input_path = Some(other.to_string());
            }
        }
        i += 1;
    }

    if show_help {
        print_help();
        return;
    }

    let input_path = match input_path {
        Some(p) => p,
        None => {
            eprintln!("Error: no input file specified.");
            eprintln!("Run with --help for usage.");
            process::exit(1);
        }
    };

    // Parse FPGA target.
    let fpga_target = match &target_name {
        Some(name) => match FpgaTarget::from_str_name(name) {
            Some(t) => t,
            None => {
                eprintln!("Unknown FPGA target: '{name}'.");
                eprintln!(
                    "Valid targets: generic, xilinx-7, xilinx-us, intel-cyclone, lattice-ice40, lattice-ecp5, lattice-nexus"
                );
                process::exit(1);
            }
        },
        None => FpgaTarget::default(),
    };

    let source = match std::fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: cannot read '{input_path}': {e}");
            process::exit(1);
        }
    };

    // Run full pipeline — enable R-SPU stage when rspu output is requested.
    let mut config = PipelineConfig::default();
    if emit_format.as_deref() == Some("rspu") {
        config.rspu = true;
    }
    let result = match run_pipeline(&source, &config) {
        Ok(r) => r,
        Err(e) => {
            for err in &e.errors {
                let diagnostic = err.to_diagnostic();
                let rendered = nasa_rust_project::diagnostic::render_diagnostic(
                    &diagnostic,
                    &source,
                    &input_path,
                );
                eprint!("{}", rendered);
            }
            let n = e.errors.len();
            if n == 1 {
                eprintln!("error: aborting due to previous error");
            } else {
                eprintln!("error: aborting due to {n} previous errors");
            }
            process::exit(1);
        }
    };

    // Print summary.
    print_summary(&result, show_stats);

    // Check for width errors.
    if result.has_width_errors() {
        eprintln!("Width errors detected — output may be incomplete.");
    }

    // Emit output.
    let format = emit_format.as_deref().unwrap_or("dot");
    let output = match format {
        "dot" => {
            if dot_detail_expr {
                emit::dot::emit_expr_dot(&result)
            } else {
                emit::dot::emit_module_dot(&result)
            }
        }
        "verilog" | "sv" => {
            let t = if fpga_target == emit::fpga_target::FpgaTarget::Generic {
                None
            } else {
                Some(fpga_target)
            };
            if strip_sva {
                emit::verilog::emit_sv_synthesis(&result, t, dsp_threshold)
            } else {
                emit::verilog::emit_sv_with_options(&result, t, dsp_threshold)
            }
        }
        "json" => match emit::json_netlist::emit_json(&result) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error serializing JSON: {e}");
                process::exit(1);
            }
        },
        "sva" => emit::verilog::emit_sva_only(&result),
        "firrtl" => emit::firrtl::emit_firrtl(&result),
        "rspu" => match &result.rspu_program {
            Some(prog) => prog.emit_asm(),
            None => {
                eprintln!(
                    "Error: R-SPU program was not generated (pipeline may have been skipped)."
                );
                process::exit(1);
            }
        },
        "testbench" => emit::testbench::emit_testbench(&result),
        "scaffold" => emit::fpga_scaffold::emit_constraints(&result, &fpga_target),
        "build-script" => emit::fpga_scaffold::emit_build_script(&result, &fpga_target),
        "sexpr" | "s-expr" | "sexp" => emit::sexpr::emit_sexpr(&result),
        other => {
            eprintln!(
                "Unknown emit format: '{other}'. Use dot, verilog, json, sva, firrtl, rspu, testbench, scaffold, build-script, or sexpr."
            );
            process::exit(1);
        }
    };

    // Write primary output.
    match &output_path {
        Some(path) => {
            if let Err(e) = std::fs::write(path, &output) {
                eprintln!("Error writing '{path}': {e}");
                process::exit(1);
            }
            eprintln!("Output written to {path}");
        }
        None => {
            print!("{output}");
        }
    }

    // Emit additional outputs if requested alongside verilog.
    if (format == "verilog" || format == "sv") && emit_testbench {
        let tb = emit::testbench::emit_testbench(&result);
        let tb_path = derive_path(&input_path, "_tb.sv");
        if let Err(e) = std::fs::write(&tb_path, &tb) {
            eprintln!("Error writing testbench '{tb_path}': {e}");
        } else {
            eprintln!("Testbench written to {tb_path}");
        }
    }

    if (format == "verilog" || format == "sv") && emit_scaffold {
        let constraints = emit::fpga_scaffold::emit_constraints(&result, &fpga_target);
        let ext = fpga_target.constraint_extension();
        let constr_path = derive_path(&input_path, &format!(".{ext}"));
        if let Err(e) = std::fs::write(&constr_path, &constraints) {
            eprintln!("Error writing constraints '{constr_path}': {e}");
        } else {
            eprintln!("Constraints written to {constr_path}");
        }

        let build = emit::fpga_scaffold::emit_build_script(&result, &fpga_target);
        let build_ext = match fpga_target {
            FpgaTarget::LatticeIce40
            | FpgaTarget::LatticeEcp5
            | FpgaTarget::LatticeNexus
            | FpgaTarget::Generic => "sh",
            _ => "tcl",
        };
        let build_path = derive_path(&input_path, &format!("_build.{build_ext}"));
        if let Err(e) = std::fs::write(&build_path, &build) {
            eprintln!("Error writing build script '{build_path}': {e}");
        } else {
            eprintln!("Build script written to {build_path}");
        }
    }

    // Write separate SVA bind file if requested.
    if let Some(ref sva_path) = sva_file {
        let sva_content = emit::verilog::emit_sva_bind_file(&result);
        if sva_content.is_empty() {
            eprintln!("No properties to write to SVA bind file.");
        } else if let Err(e) = std::fs::write(sva_path, &sva_content) {
            eprintln!("Error writing SVA bind file '{sva_path}': {e}");
        } else {
            eprintln!("SVA bind file written to {sva_path}");
        }
    }

    // Emit synchronizer chain info if non-default.
    if sync_stages != 2 && (format == "verilog" || format == "sv") {
        eprintln!("  Sync stages: {sync_stages}");
    }

    // Toolchain operations — only if any toolchain flag is set.
    if formal || lint || simulate || pnr || timing || eqy {
        run_toolchain_operations(
            &result,
            &input_path,
            &fpga_target,
            dsp_threshold,
            formal,
            formal_depth,
            formal_prove,
            &formal_engine,
            lint,
            simulate,
            pnr,
            timing,
            eqy,
            toolchain_path.as_deref(),
        );
    }
}

/// Derive an output path from the input path by replacing the extension.
fn derive_path(input_path: &str, suffix: &str) -> String {
    if let Some(dot_pos) = input_path.rfind('.') {
        format!("{}{}", &input_path[..dot_pos], suffix)
    } else {
        format!("{input_path}{suffix}")
    }
}

fn print_summary(result: &nasa_rust_project::pipeline::PipelineResult, show_stats: bool) {
    let module = &result.program.module;
    eprintln!("MIRR Compile: {}", module.name);
    eprintln!(
        "  Signals: {}  Guards: {}  Reflexes: {}",
        module.signals.len(),
        module.guards.len(),
        module.reflexes.len(),
    );

    if let Some(ss) = &result.simplify_stats {
        eprintln!(
            "  Simplify: {} rules applied, {} -> {} nodes",
            ss.rules_applied, ss.nodes_before, ss.nodes_after,
        );
    }

    if let Some(wr) = &result.width_result {
        let diag_count = wr.stats.diagnostics_count;
        let scc_count = wr.stats.scc_count;
        eprintln!("  Width: {diag_count} diagnostics, {scc_count} SCCs");
    }

    if let Some(tn) = &result.temporal_netlist {
        eprintln!("  Temporal: {} guards, {} signals", tn.guards.len(), tn.signals.len(),);
    }

    if show_stats {
        if let Some(wr) = &result.width_result {
            eprintln!(
                "  [stats] nodes_analyzed={} rounds={} sccs={} expansive={} nonexpansive={}",
                wr.stats.nodes_analyzed,
                wr.stats.propagation_rounds,
                wr.stats.scc_count,
                wr.stats.expansive_count,
                wr.stats.nonexpansive_count,
            );
        }
    }
}

fn print_help() {
    println!("mirr-compile — Unified MIRR compilation driver (Phase 6)");
    println!();
    println!("Usage:");
    println!("  mirr-compile <file.mirr> [OPTIONS]");
    println!();
    println!("Emission Options:");
    println!("  --emit FORMAT       Output format: dot, verilog, json, sva, firrtl, rspu,");
    println!("                      testbench, scaffold, build-script, sexpr (default: dot)");
    println!("  --output FILE, -o   Write output to FILE (default: stdout)");
    println!("  --target FAMILY     FPGA target: generic, xilinx-7, xilinx-us, intel-cyclone,");
    println!("                      lattice-ice40, lattice-ecp5, lattice-nexus (default: generic)");
    println!("  --sync-stages N     Input synchronizer stages, 0 to disable (default: 2)");
    println!("  --dsp-threshold N   Min operand bits for DSP inference, 0 to disable (default: 9)");
    println!("  --testbench         Also emit a self-checking testbench (with --emit verilog)");
    println!("  --scaffold          Also emit constraint template and build script");
    println!("  --strip-sva         Omit SVA assertions from verilog output (for synthesis)");
    println!("  --sva-file FILE     Write SVA properties to a separate bind file");
    println!("  --dot-detail expr   Show full AST trees in DOT output");
    println!("  --stats             Print detailed pipeline statistics");
    println!();
    println!("Toolchain Options (requires oss-cad-suite in PATH):");
    println!("  --formal            Run SymbiYosys formal verification");
    println!("  --formal-depth N    BMC depth (default: 20, max: 200)");
    println!("  --formal-prove      Also run k-induction prove");
    println!("  --formal-engine E   Solver: z3, yices, bitwuzla, btor (default: z3)");
    println!("  --lint              Run Verilator lint-only");
    println!("  --simulate          Run Verilator compiled simulation");
    println!("  --pnr               Run nextpnr place and route (Lattice targets)");
    println!("  --timing            Run icetime static timing analysis (iCE40 only)");
    println!("  --eqy               Run EQY equivalence checking");
    println!("  --toolchain-path D  Override oss-cad-suite root directory");
    println!();
    println!("  --help, -h          Show this help");
    println!();
    println!("Examples:");
    println!("  mirr-compile program.mirr --emit verilog -o out.sv");
    println!("  mirr-compile program.mirr --emit verilog --target lattice-ecp5 --scaffold");
    println!("  mirr-compile program.mirr --emit verilog --strip-sva --formal");
    println!("  mirr-compile program.mirr --emit verilog --lint");
    println!("  mirr-compile program.mirr --emit json | jq .");
    println!("  mirr-compile program.mirr --emit dot | dot -Tpng -o graph.png");
    println!("  mirr-compile program.mirr --emit rspu");
}

/// Run toolchain operations (formal, lint, simulate, pnr, timing, eqy).
///
/// This is the foundation for future toolchain integration. Each operation
/// checks whether its required tool is available and prints a clear message
/// if not.
#[allow(clippy::too_many_arguments)]
fn run_toolchain_operations(
    result: &nasa_rust_project::pipeline::PipelineResult,
    input_path: &str,
    fpga_target: &FpgaTarget,
    dsp_threshold: u32,
    formal: bool,
    formal_depth: u32,
    formal_prove: bool,
    formal_engine: &str,
    lint: bool,
    simulate: bool,
    pnr: bool,
    timing: bool,
    eqy_check: bool,
    _toolchain_path: Option<&str>,
) {
    use nasa_rust_project::toolchain::{Tool, ToolRegistry};

    eprintln!();
    eprintln!("=== Toolchain Operations ===");

    // Probe relevant tools
    let mut registry = ToolRegistry::new();

    if formal {
        registry.probe(Tool::Sby);
    }
    if lint || simulate {
        registry.probe(Tool::Verilator);
    }
    if timing {
        registry.probe(Tool::Icetime);
    }
    if eqy_check {
        registry.probe(Tool::Eqy);
    }
    if pnr {
        if let Some(bin) = fpga_target.nextpnr_binary() {
            let tool = match bin {
                "nextpnr-ice40" => Tool::NextpnrIce40,
                "nextpnr-ecp5" => Tool::NextpnrEcp5,
                "nextpnr-nexus" => Tool::NextpnrNexus,
                _ => Tool::NextpnrIce40,
            };
            registry.probe(tool);
        }
    }

    // Generate synthesis-clean SV for toolchain operations
    let t = if *fpga_target == FpgaTarget::Generic { None } else { Some(*fpga_target) };
    let sv_content = emit::verilog::emit_sv_synthesis(result, t, dsp_threshold);
    let sv_path = derive_path(input_path, "_synth.sv");
    if let Err(e) = std::fs::write(&sv_path, &sv_content) {
        eprintln!("Error writing synthesis SV '{sv_path}': {e}");
        return;
    }

    // Write SVA bind file for formal verification
    let bind_content = emit::verilog::emit_sva_bind_file(result);
    let bind_path = if !bind_content.is_empty() {
        let p = derive_path(input_path, "_sva_bind.sv");
        if let Err(e) = std::fs::write(&p, &bind_content) {
            eprintln!("Error writing SVA bind file '{p}': {e}");
        }
        Some(p)
    } else {
        None
    };

    // Formal verification
    if formal {
        if registry.is_available(Tool::Sby) {
            let engine = nasa_rust_project::toolchain::sby::SbyEngine::from_str_name(formal_engine)
                .unwrap_or(nasa_rust_project::toolchain::sby::SbyEngine::Z3);
            let config = nasa_rust_project::toolchain::sby::SbyConfig {
                bmc_depth: formal_depth,
                prove: formal_prove,
                engine,
            };
            let sby_content = nasa_rust_project::toolchain::sby::generate_sby_config(
                &result.program.module.name,
                std::path::Path::new(&sv_path),
                bind_path.as_ref().map(|p| std::path::Path::new(p.as_str())),
                &config,
            );
            let sby_path = derive_path(input_path, ".sby");
            if let Err(e) = std::fs::write(&sby_path, &sby_content) {
                eprintln!("Error writing sby config '{sby_path}': {e}");
            } else {
                eprintln!("  [formal] Config written to {sby_path}");
                eprintln!("  [formal] Engine: {formal_engine}, depth: {formal_depth}, prove: {formal_prove}");
                // Run sby
                match nasa_rust_project::toolchain::sby::run_formal(
                    std::path::Path::new(&sby_path),
                    std::path::Path::new("."),
                    &registry,
                ) {
                    Ok(res) => {
                        if res.passed {
                            eprintln!("  [formal] PASSED");
                        } else {
                            eprintln!("  [formal] FAILED (exit code: {:?})", res.exit_code);
                        }
                    }
                    Err(e) => eprintln!("  [formal] Error: {e}"),
                }
            }
        } else {
            eprintln!("  [formal] SKIPPED — sby not found in PATH");
        }
    }

    // Lint
    if lint {
        if registry.is_available(Tool::Verilator) {
            eprintln!("  [lint] Running Verilator lint...");
            match nasa_rust_project::toolchain::verilator::run_lint(
                std::path::Path::new(&sv_path),
                std::path::Path::new("."),
                &registry,
            ) {
                Ok(res) => {
                    if res.passed {
                        eprintln!("  [lint] PASSED ({} warnings)", res.warning_count);
                    } else {
                        eprintln!(
                            "  [lint] FAILED ({} errors, {} warnings)",
                            res.error_count, res.warning_count
                        );
                    }
                }
                Err(e) => eprintln!("  [lint] Error: {e}"),
            }
        } else {
            eprintln!("  [lint] SKIPPED — verilator not found in PATH");
        }
    }

    // Simulate
    if simulate {
        if registry.is_available(Tool::Verilator) {
            eprintln!("  [simulate] Running Verilator simulation...");
            match nasa_rust_project::toolchain::verilator::run_simulation(
                std::path::Path::new(&sv_path),
                &result.program.module.name,
                std::path::Path::new("."),
                &registry,
            ) {
                Ok(res) => {
                    if res.passed {
                        eprintln!("  [simulate] PASSED (cycles: {:?})", res.cycles);
                    } else {
                        eprintln!("  [simulate] FAILED");
                    }
                }
                Err(e) => eprintln!("  [simulate] Error: {e}"),
            }
        } else {
            eprintln!("  [simulate] SKIPPED — verilator not found in PATH");
        }
    }

    // Place and route
    if pnr {
        match fpga_target.nextpnr_binary() {
            Some(_) => {
                eprintln!(
                    "  [pnr] nextpnr invocation infrastructure ready for {}",
                    fpga_target.display_name()
                );
                eprintln!(
                    "  [pnr] Run build script manually: {}_build.sh",
                    derive_path(input_path, "")
                );
            }
            None => {
                eprintln!(
                    "  [pnr] SKIPPED — PnR only available for Lattice targets (ice40, ecp5, nexus)"
                );
            }
        }
    }

    // Static timing
    if timing {
        match fpga_target.icetime_device() {
            Some(_device) => {
                if registry.is_available(Tool::Icetime) {
                    eprintln!("  [timing] icetime ready for iCE40 (requires .asc file from PnR)");
                } else {
                    eprintln!("  [timing] SKIPPED — icetime not found in PATH");
                }
            }
            None => {
                eprintln!("  [timing] SKIPPED — icetime only supports iCE40 targets");
            }
        }
    }

    // Equivalence checking
    if eqy_check {
        if registry.is_available(Tool::Eqy) {
            eprintln!(
                "  [eqy] EQY ready — provide gold and gate SV files for equivalence checking"
            );
        } else {
            eprintln!("  [eqy] SKIPPED — eqy not found in PATH");
        }
    }
}
