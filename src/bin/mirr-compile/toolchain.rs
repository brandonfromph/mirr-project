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
    let sv_path = super::derive_path(input_path, "_synth.sv");
    if let Err(e) = std::fs::write(&sv_path, &sv_content) {
        eprintln!("  [toolchain] failed to write synthesis SV '{sv_path}'\n    help: {}", e);
        return;
    }

    if optimize {
        if registry.is_available(Tool::Yosys) {
            eprintln!("  [optimize] Running logic optimization with ABC...");
            match mirrc::toolchain::optimize::run_logic_optimization(
                &registry,
                std::path::Path::new(&sv_path),
                &result.program.module.name,
                std::path::Path::new("."),
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

    // Write SVA bind file for formal verification
    let bind_content = emit::verilog::emit_sva_bind_file(result);
    let bind_path = if !bind_content.is_empty() {
        let p = super::derive_path(input_path, "_sva_bind.sv");
        if let Err(e) = std::fs::write(&p, &bind_content) {
            eprintln!("  [toolchain] failed to write SVA bind file '{p}'\n    help: {}", e);
        }
        Some(p)
    } else {
        None
    };

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
                &result.program.module.name,
                std::path::Path::new(&sv_path),
                bind_path.as_ref().map(|p| std::path::Path::new(p.as_str())),
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
}
