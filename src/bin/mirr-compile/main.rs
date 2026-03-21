//! mirr-compile — Unified MIRR compilation driver (Phase 6).
//!
//! End-to-end pipeline: parse -> validate -> simplify -> width -> temporal -> emit.
//!
//! Usage:
//!   mirr-compile <file.mirr> [--emit dot|verilog|json|sva|firrtl|rspu|testbench|scaffold] [--output FILE] [--stats]
//!   mirr-compile <file.mirr> --emit verilog --target xilinx-7 --testbench --scaffold
//!   mirr-compile <file.mirr> --emit dot --dot-detail expr [--output FILE]

#![forbid(unsafe_code)]

mod help;
mod summary;
mod toolchain;

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
    let mut totality = false;
    let mut symbolic = false;

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
            "--totality" => totality = true,
            "--symbolic" => symbolic = true,
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
        help::print_help();
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
    if emit_format.as_deref() == Some("rspu")
        || emit_format.as_deref() == Some("cert")
        || emit_format.as_deref() == Some("riscv")
        || emit_format.as_deref() == Some("arm")
        || totality
    {
        config.rspu = true;
    }
    if totality || emit_format.as_deref() == Some("cert") {
        config.totality = true;
    }
    if symbolic {
        config.symbolic = true;
    }
    if emit_format.as_deref() == Some("mape-k-rtl") {
        config.temporal = true;
        config.mape_k = true;
        config.emit_mape_k_rtl = true;
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
    summary::print_summary(&result, show_stats);

    // Check for width errors — render through the diagnostic engine.
    if result.has_width_errors() {
        if let Some(ref wr) = result.width_result {
            // Collect all diagnostics from all phases.
            let mut width_diags: Vec<&nasa_rust_project::width::types::WidthDiag> = Vec::new();
            for (_, diags) in &wr.phase4a.assignment_results {
                for d in diags {
                    width_diags.push(d);
                }
            }
            for (_, r) in &wr.phase4a.guard_results {
                for d in &r.diagnostics {
                    width_diags.push(d);
                }
            }
            for d in &wr.scc_diagnostics {
                width_diags.push(d);
            }
            for d in &wr.verification.diagnostics {
                width_diags.push(d);
            }
            for wd in &width_diags {
                let d = wd.to_diagnostic();
                let rendered =
                    nasa_rust_project::diagnostic::render_diagnostic(&d, &source, &input_path);
                eprint!("{}", rendered);
            }
        }
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
        "riscv" => match &result.rspu_program {
            Some(prog) => emit::riscv::emit_riscv_asm(prog)
                .unwrap_or_else(|e| {
                    eprintln!("Error emitting RISC-V assembly: {e:?}");
                    process::exit(1);
                }),
            None => {
                eprintln!(
                    "Error: R-SPU program was not generated (required for RISC-V emission)."
                );
                process::exit(1);
            }
        },
        "arm" => match &result.rspu_program {
            Some(prog) => emit::arm::emit_arm_asm(prog)
                .unwrap_or_else(|e| {
                    eprintln!("Error emitting ARM assembly: {e:?}");
                    process::exit(1);
                }),
            None => {
                eprintln!(
                    "Error: R-SPU program was not generated (required for ARM emission)."
                );
                process::exit(1);
            }
        },
        "testbench" => emit::testbench::emit_testbench(&result),
        "scaffold" => emit::fpga_scaffold::emit_constraints(&result, &fpga_target),
        "build-script" => emit::fpga_scaffold::emit_build_script(&result, &fpga_target),
        "sexpr" | "s-expr" | "sexp" => emit::sexpr::emit_sexpr(&result),
        "mape-k-rtl" => match &result.mape_k_rtl {
            Some(rtl) => rtl.clone(),
            None => {
                eprintln!("Error: MAPE-K RTL was not generated (pipeline may have been skipped).");
                process::exit(1);
            }
        },
        "cert" => match &result.rspu_program {
            Some(prog) => match &prog.certificate {
                Some(cert_bytes) => {
                    // Binary certificate — write directly to output path.
                    if let Some(ref path) = output_path {
                        if let Err(e) = std::fs::write(path, cert_bytes) {
                            eprintln!("Error writing certificate '{path}': {e}");
                            process::exit(1);
                        }
                        eprintln!("Certificate written to {path} ({} bytes)", cert_bytes.len());
                        return;
                    }
                    // No output path: hex-encode for stdout.
                    cert_bytes.iter().fold(String::new(), |mut acc, b| {
                        use std::fmt::Write;
                        let _ = write!(acc, "{b:02x}");
                        acc
                    })
                }
                None => {
                    eprintln!("Error: totality check did not produce a certificate.");
                    eprintln!("Hint: use --totality with --emit cert.");
                    process::exit(1);
                }
            },
            None => {
                eprintln!("Error: R-SPU program was not generated (required for cert emission).");
                process::exit(1);
            }
        },
        other => {
            eprintln!(
                "Unknown emit format: '{other}'. Use dot, verilog, json, sva, firrtl, rspu, riscv, arm, testbench, scaffold, build-script, sexpr, mape-k-rtl, or cert."
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
        toolchain::run_toolchain_operations(
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
pub(crate) fn derive_path(input_path: &str, suffix: &str) -> String {
    if let Some(dot_pos) = input_path.rfind('.') {
        format!("{}{}", &input_path[..dot_pos], suffix)
    } else {
        format!("{input_path}{suffix}")
    }
}
