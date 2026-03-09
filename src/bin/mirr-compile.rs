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
            "--stats" => show_stats = true,
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
                    "Valid targets: generic, xilinx-7, xilinx-us, intel-cyclone, lattice-ice40"
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
            let diagnostic = e.to_diagnostic();
            let rendered =
                nasa_rust_project::diagnostic::render_diagnostic(&diagnostic, &source, &input_path);
            eprint!("{}", rendered);
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
            emit::verilog::emit_sv_with_target(&result, &fpga_target, dsp_threshold)
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
        other => {
            eprintln!(
                "Unknown emit format: '{other}'. Use dot, verilog, json, sva, firrtl, rspu, testbench, scaffold, or build-script."
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
            FpgaTarget::LatticeIce40 | FpgaTarget::Generic => "sh",
            _ => "tcl",
        };
        let build_path = derive_path(&input_path, &format!("_build.{build_ext}"));
        if let Err(e) = std::fs::write(&build_path, &build) {
            eprintln!("Error writing build script '{build_path}': {e}");
        } else {
            eprintln!("Build script written to {build_path}");
        }
    }

    // Emit synchronizer chain info if non-default.
    if sync_stages != 2 && (format == "verilog" || format == "sv") {
        eprintln!("  Sync stages: {sync_stages}");
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
    println!("Options:");
    println!("  --emit FORMAT       Output format: dot, verilog, json, sva, firrtl, rspu,");
    println!("                      testbench, scaffold, build-script (default: dot)");
    println!("  --output FILE, -o   Write output to FILE (default: stdout)");
    println!("  --target FAMILY     FPGA target: generic, xilinx-7, xilinx-us, intel-cyclone,");
    println!("                      lattice-ice40 (default: generic)");
    println!("  --sync-stages N     Input synchronizer stages, 0 to disable (default: 2)");
    println!("  --dsp-threshold N   Min operand bits for DSP inference, 0 to disable (default: 9)");
    println!("  --testbench         Also emit a self-checking testbench (with --emit verilog)");
    println!("  --scaffold          Also emit constraint template and build script");
    println!("  --dot-detail expr   Show full AST trees in DOT output");
    println!("  --stats             Print detailed pipeline statistics");
    println!("  --help, -h          Show this help");
    println!();
    println!("Examples:");
    println!("  mirr-compile program.mirr --emit verilog -o out.sv");
    println!("  mirr-compile program.mirr --emit verilog --target xilinx-7 --testbench --scaffold");
    println!("  mirr-compile program.mirr --emit testbench");
    println!("  mirr-compile program.mirr --emit json | jq .");
    println!("  mirr-compile program.mirr --emit dot | dot -Tpng -o graph.png");
    println!("  mirr-compile program.mirr --emit rspu");
}
