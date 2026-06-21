//! Toolchain operations: formal, lint, simulate, pnr, timing, eqy.

#![forbid(unsafe_code)]

use mirrc::emit;
use mirrc::emit::fpga_target::FpgaTarget;

/// Run toolchain operations (formal, lint, simulate, pnr, timing, eqy).
///
/// This is the foundation for future toolchain integration. Each operation
/// checks whether its required tool is available and prints a clear message
/// if not.
#[allow(clippy::too_many_arguments)]
pub(super) fn run_toolchain_operations(
    result: &mirrc::pipeline::PipelineResult,
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
    optimize: bool,
    tapeout: bool,
    _toolchain_path: Option<&str>,
    link: &[String],
) {
    use mirrc::toolchain::{Tool, ToolRegistry};

    eprintln!();
    eprintln!("=== Toolchain Operations ===");

    // Probe relevant tools
    let mut registry = ToolRegistry::new();

    if formal {
        registry.probe(Tool::Sby);
    }
    if optimize {
        registry.probe(Tool::Yosys);
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
    if tapeout {
        registry.probe(Tool::Openlane);
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

    let ecs = result.ecs_registry.as_ref().expect("ECS registry required");
    let module_name = ecs.get_module_name().unwrap_or_else(|| "unknown_module".to_string());

    // Generate synthesis-clean SV for toolchain operations
    let t = if *fpga_target == FpgaTarget::Generic { None } else { Some(*fpga_target) };
    let sv_content = emit::verilog::emit_sv_synthesis(result, t, dsp_threshold);
    let sv_path = super::derive_path(input_path, "_synth.sv");
    if let Err(e) = std::fs::write(&sv_path, &sv_content) {
        eprintln!("  [toolchain] failed to write synthesis SV '{sv_path}'\n    help: {}", e);
        return;
    }

    let prov_content =
        emit::provenance::emit_provenance_graph(result).unwrap_or_else(|_| "{}".to_string());
    let prov_path = super::derive_path(input_path, "_provenance.json");
    if let Err(e) = std::fs::write(&prov_path, &prov_content) {
        eprintln!(
            "  [toolchain] WARNING: failed to write provenance graph '{prov_path}'\n    help: {}",
            e
        );
    }

    if optimize {
        if registry.is_available(Tool::Yosys) {
            eprintln!("  [optimize] Running logic optimization with ABC...");
            match mirrc::toolchain::optimize::run_logic_optimization(
                &registry,
                std::path::Path::new(&sv_path),
                &module_name,
                std::path::Path::new("."),
                link,
            ) {
                Ok(res) => {
                    if res.success {
                        eprintln!("  [optimize] PASSED");
                        eprintln!("  [optimize] Optimized SV written to {}", res.optimized_path);
                    } else {
                        eprintln!("  [optimize] FAILED:\n{}", res.stderr);
                    }
                }
                Err(e) => eprintln!("  [optimize] failed: {e}"),
            }
        } else {
            eprintln!("  [optimize] SKIPPED — yosys not found in PATH");
        }
    }

    let mut formal_sv_path = None;

    if formal {
        let formal_content = emit::verilog::emit_sv_full(result, t, dsp_threshold, false);
        let path = super::derive_path(input_path, "_formal.sv");
        if let Err(e) = std::fs::write(&path, &formal_content) {
            eprintln!("  [toolchain] failed to write formal SV '{path}'\n    help: {}", e);
        } else {
            formal_sv_path = Some(path);
        }
    }

    // Formal verification
    if formal {
        if registry.is_available(Tool::Sby) {
            let engine = mirrc::toolchain::sby::SbyEngine::from_str_name(formal_engine)
                .unwrap_or(mirrc::toolchain::sby::SbyEngine::Bitwuzla);
            let config = mirrc::toolchain::sby::SbyConfig {
                bmc_depth: formal_depth,
                prove: formal_prove,
                cover: false,
                engine,
                extra_files: link.to_vec(),
            };
            let sby_content = mirrc::toolchain::sby::generate_sby_config(
                &module_name,
                std::path::Path::new(formal_sv_path.as_ref().unwrap_or(&sv_path)),
                None,
                &config,
            );
            let sby_path = super::derive_path(input_path, ".sby");
            if let Err(e) = std::fs::write(&sby_path, &sby_content) {
                eprintln!("  [formal] failed to write sby config '{sby_path}'\n    help: {}", e);
            } else {
                eprintln!("  [formal] Config written to {sby_path}");
                eprintln!("  [formal] Engine: {formal_engine}, depth: {formal_depth}, prove: {formal_prove}");
                // Run sby
                match mirrc::toolchain::sby::run_formal(
                    std::path::Path::new(&sby_path),
                    std::path::Path::new("."),
                    &registry,
                ) {
                    Ok(res) => {
                        if res.passed {
                            eprintln!("  [formal] PASSED");
                        } else {
                            eprintln!("  [formal] FAILED (exit code: {:?})", res.exit_code);

                            // Invoke Automated Trace Analyzer
                            if let Some(graph) =
                                mirrc::emit::provenance::build_provenance_graph(result)
                            {
                                let task_name = if formal_prove { "prove" } else { "bmc" };
                                let base_dir = std::path::Path::new(&sby_path).with_extension("");
                                let trace_dir = format!("{}_{}", base_dir.display(), task_name);
                                let trace_path = std::path::Path::new(&trace_dir)
                                    .join("engine_0")
                                    .join("trace.vcd");

                                // Extract the exact property name from the SBY output.
                                let mut failed_property = String::new();
                                for line in res.stdout.lines() {
                                    if line.contains("failed assertion ") {
                                        if let Some(pos) = line.find("failed assertion ") {
                                            let after = &line[pos + 17..];
                                            if let Some(space) = after.find(' ') {
                                                failed_property = after[..space].to_string();
                                                break;
                                            }
                                        }
                                    }
                                }

                                if failed_property.is_empty() {
                                    failed_property = format!("{}.unknown_property", module_name);
                                } else if let Some(dot_idx) = failed_property.find('.') {
                                    // Strip the module prefix because our provenance graph doesn't prepend it for properties yet.
                                    failed_property = failed_property[dot_idx + 1..].to_string();
                                }

                                let report = mirrc::diagnostic::formal_trace::analyze_failure(
                                    &graph,
                                    &failed_property,
                                    Some(&trace_path),
                                    &result.file_table,
                                );

                                for diag in report.to_diagnostics() {
                                    // Fetch source content if we have a span
                                    let (source, path) = if let Some(s) = diag.span {
                                        if let Some(file_id) = s.file_id {
                                            if let Some(p_str) = result.file_table.get(file_id) {
                                                let src = std::fs::read_to_string(p_str)
                                                    .unwrap_or_default();
                                                (src, p_str.to_string())
                                            } else {
                                                (String::new(), "unknown".to_string())
                                            }
                                        } else {
                                            let src = std::fs::read_to_string(input_path)
                                                .unwrap_or_default();
                                            (src, input_path.to_string())
                                        }
                                    } else {
                                        (String::new(), "unknown".to_string())
                                    };

                                    let rendered =
                                        mirrc::diagnostic::render_diagnostic(&diag, &source, &path);
                                    eprint!("{}", rendered);
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("  [formal] failed: {e}"),
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
            match mirrc::toolchain::verilator::run_lint(
                std::path::Path::new(&sv_path),
                std::path::Path::new("."),
                &registry,
                link,
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
                Err(e) => eprintln!("  [lint] failed: {e}"),
            }
        } else {
            eprintln!("  [lint] SKIPPED — verilator not found in PATH");
        }
    }

    // Simulate
    if simulate {
        if registry.is_available(Tool::Verilator) {
            eprintln!("  [simulate] Running Verilator simulation...");
            match mirrc::toolchain::verilator::run_simulation(
                std::path::Path::new(&sv_path),
                &module_name,
                std::path::Path::new("."),
                &registry,
                link,
            ) {
                Ok(res) => {
                    if res.passed {
                        eprintln!("  [simulate] PASSED (cycles: {:?})", res.cycles);
                    } else {
                        eprintln!("  [simulate] FAILED");
                    }
                }
                Err(e) => eprintln!("  [simulate] failed: {e}"),
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
                    super::derive_path(input_path, "")
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

    // ASIC Tape-out (OpenLANE)
    if tapeout {
        eprintln!("  [tapeout] Generating OpenLANE ASIC physical design package...");

        // 1. Generate SDC
        let sdc_content = mirrc::toolchain::sdc::generate_sdc_config(ecs.target_config.as_ref());
        let sdc_path = "constraints.sdc";
        if let Err(e) = std::fs::write(sdc_path, &sdc_content) {
            eprintln!("  [tapeout] FAILED — unable to write SDC constraints: {}", e);
            return;
        }

        // 2. Generate config.json
        let mut verilog_files = vec![sv_path.clone()];
        verilog_files.extend_from_slice(link);

        let config = mirrc::toolchain::openlane::OpenLaneConfig {
            design_name: module_name.clone(),
            verilog_files,
            clock_port: "clk".to_string(),
            clock_period: 10.0,
            sdc_file: std::fs::canonicalize(sdc_path)
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| sdc_path.to_string()),
            pdk: "sky130A".to_string(),
            std_cell_library: "sky130_fd_sc_hd".to_string(),
            pl_target_density: 0.65,
            run_synth: false,
        };

        let config_json = serde_json::to_string_pretty(&config).unwrap_or_default();
        if let Err(e) = std::fs::write("config.json", &config_json) {
            eprintln!("  [tapeout] FAILED — unable to write config.json: {}", e);
            return;
        }

        eprintln!("  [tapeout] PASSED — Tape-out package generated successfully.");
        eprintln!("  [tapeout] Output files ready for foundry / CI pipeline:");
        eprintln!("    - {}", sv_path);
        eprintln!("    - constraints.sdc");
        eprintln!("    - config.json");

        if registry.is_available(Tool::Openlane) {
            eprintln!(
                "  [tapeout] Docker detected. Running local OpenLANE physical design flow..."
            );
            match mirrc::toolchain::openlane::run_openlane_flow(
                &registry,
                std::path::Path::new("."),
                &config,
            ) {
                Ok(res) => {
                    if res.success {
                        eprintln!("  [tapeout] PASSED — OpenLANE flow complete.");
                        if let Some(gds) = res.gds_path {
                            eprintln!("  [tapeout] GDSII Layout: {}", gds);
                        }
                        if res.setup_slack_ns < 0.0 {
                            eprintln!(
                                "  [tapeout] WARNING — Static Timing Analysis failed. WNS: {}ns.",
                                res.setup_slack_ns
                            );
                        }
                    } else if res.routing_violations > 0 {
                        eprintln!(
                            "  [tapeout] FAILED — Placement density too high (Current: 0.65). Try lowering --pl-target-density."
                        );
                    } else {
                        eprintln!("  [tapeout] FAILED — OpenLANE error:\n{}", res.stderr);
                    }
                }
                Err(e) => eprintln!("  [tapeout] FAILED: {}", e),
            }
        } else {
            eprintln!("  [tapeout] SKIPPED local execution — Docker not found. This is normal!");
            eprintln!("  [tapeout] (ASIC Place & Route of 64 cores requires 16GB+ RAM. Drop the generated files into a CI runner to produce the .gds file.)");
        }
    }
}
