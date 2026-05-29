//! mirr-compile — Unified MIRR compilation driver (Phase 6).
//!
//!
//! End-to-end pipeline: parse -> validate -> simplify -> width -> temporal -> emit.
//!
//! Usage:
//!   mirr-compile <file.mirr> [--emit dot|verilog|json|sva|firrtl|rspu|testbench|scaffold] [--output FILE] [--stats]
//!   mirr-compile <file.mirr> --emit verilog --target xilinx-7 --testbench --scaffold
//!   mirr-compile <file.mirr> --emit dot --dot-detail expr [--output FILE]

#![forbid(unsafe_code)]

mod summary;
mod toolchain;

use clap::Parser;
use std::path::Path;
use std::process;

use nasa_rust_project::emit;
use nasa_rust_project::emit::fpga_target::FpgaTarget;
use nasa_rust_project::pipeline::PipelineConfig;
use nasa_rust_project::Workspace;

#[derive(Parser, Debug)]
#[command(
    name = "mirr-compile",
    author,
    version,
    about = "Unified MIRR compilation driver (Phase 6)"
)]
struct Cli {
    /// Path to the root MIRR file
    root_file: Option<String>,

    /// Export CLI schema as JSON for tool integration
    #[arg(long, hide = true)]
    help_json: bool,

    /// Output format: dot, verilog, sv, json, sva, firrtl, rspu, riscv, arm, testbench, scaffold, build-script, sexpr, mape-k-rtl, cert
    #[arg(short, long)]
    emit: Option<String>,

    /// Write output to FILE
    #[arg(short, long)]
    output: Option<String>,

    /// FPGA target: generic, xilinx-7, xilinx-us, intel-cyclone, lattice-ice40, lattice-ecp5, lattice-nexus
    #[arg(long, default_value = "generic")]
    target: String,

    /// Input synchronizer stages
    #[arg(long, default_value_t = 2)]
    sync_stages: u32,

    /// Min operand bits for DSP inference
    #[arg(long, default_value_t = 9)]
    dsp_threshold: u32,

    /// Also emit self-checking testbench
    #[arg(long)]
    testbench: bool,

    /// Also emit FPGA constraint template and build script
    #[arg(long)]
    scaffold: bool,

    /// Omit SVA assertions from verilog output
    #[arg(long)]
    strip_sva: bool,

    /// Write SVA properties to a separate bind file
    #[arg(long)]
    sva_file: Option<String>,

    /// Show full AST trees in DOT output
    #[arg(long)]
    dot_detail: bool,

    /// Print detailed pipeline statistics
    #[arg(long)]
    stats: bool,

    /// Enable MEGA-4 totality check and generate proof certificate
    #[arg(long)]
    totality: bool,

    /// Enable MEGA-5 symbolic interval analysis
    #[arg(long)]
    symbolic: bool,

    /// Run SymbiYosys formal verification
    #[arg(long)]
    formal: bool,

    /// BMC depth
    #[arg(long, default_value_t = 20)]
    formal_depth: u32,

    /// Also run k-induction prove
    #[arg(long)]
    formal_prove: bool,

    /// Solver: z3, yices, bitwuzla, btor
    #[arg(long, default_value = "z3")]
    formal_engine: String,

    /// Run Verilator lint-only
    #[arg(long)]
    lint: bool,

    /// Run Verilator compiled simulation
    #[arg(long)]
    simulate: bool,

    /// Run nextpnr place and route
    #[arg(long)]
    pnr: bool,

    /// Run icetime static timing analysis
    #[arg(long)]
    timing: bool,

    /// Run EQY equivalence checking
    #[arg(long)]
    eqy: bool,

    /// Override oss-cad-suite root directory
    #[arg(long)]
    toolchain_path: Option<String>,

    /// Verify a proof certificate against the compiled R-SPU program
    #[arg(long)]
    verify: Option<String>,
}

pub fn main() {
    let args = Cli::parse();

    if args.help_json {
        use clap::CommandFactory;
        fn get_cmd_manifest(cmd: &clap::Command) -> serde_json::Value {
            let mut args_list = Vec::new();
            for arg in cmd.get_arguments() {
                args_list.push(serde_json::json!({
                    "id": arg.get_id().as_str(),
                    "long": arg.get_long(),
                    "short": arg.get_short(),
                    "help": arg.get_help().map(|h| h.to_string()),
                    "required": arg.is_required_set(),
                }));
            }
            let mut subs = Vec::new();
            for sub in cmd.get_subcommands() {
                subs.push(get_cmd_manifest(sub));
            }
            serde_json::json!({
                "name": cmd.get_name(),
                "about": cmd.get_about().map(|a| a.to_string()),
                "version": cmd.get_version().map(|v| v.to_string()),
                "args": args_list,
                "subcommands": subs,
            })
        }
        let cmd = Cli::command();
        println!("{}", serde_json::to_string_pretty(&get_cmd_manifest(&cmd)).unwrap());
        process::exit(0);
    }

    let root_file = args.root_file.unwrap_or_else(|| {
        eprintln!("Error: no input file specified.\nRun with --help for usage.");
        process::exit(1);
    });

    let fpga_target = FpgaTarget::from_str_name(&args.target).unwrap_or_else(|| {
        eprintln!("Unknown FPGA target: '{}'.", args.target);
        process::exit(1);
    });

    let source = std::fs::read_to_string(&root_file).unwrap_or_else(|e| {
        eprintln!("Error: cannot read '{}': {}", root_file, e);
        process::exit(1);
    });

    let mut config = PipelineConfig { bootstrap_mode: true, ..Default::default() };
    if args.emit.as_deref() == Some("rspu")
        || args.emit.as_deref() == Some("cert")
        || args.emit.as_deref() == Some("riscv")
        || args.emit.as_deref() == Some("arm")
        || args.totality
        || args.verify.is_some()
    {
        config.rspu = true;
    }
    if args.totality || args.emit.as_deref() == Some("cert") {
        config.totality = true;
    }
    if args.symbolic {
        config.symbolic = true;
    }
    if args.emit.as_deref() == Some("mape-k-rtl") {
        config.temporal = true;
        config.mape_k = true;
        config.emit_mape_k_rtl = true;
    }

    let root_path = Path::new(&root_file);
    let workspace_root = root_path.parent().unwrap_or_else(|| Path::new("."));
    let mut workspace = Workspace::new(workspace_root);
    workspace.update_file(&root_file, source.clone());
    let snapshot = match workspace.compile_snapshot(root_path, &config) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            eprintln!("Compile error: {}", error);
            process::exit(1);
        }
    };
    let result = snapshot.pipeline.as_ref();

    if let Some(cert_path) = &args.verify {
        let cert_bytes = std::fs::read(cert_path).unwrap_or_else(|e| {
            eprintln!("Error: failed to read certificate file '{}': {}", cert_path, e);
            process::exit(1);
        });

        let cert =
            nasa_rust_project::cert::deserialize_certificate(&cert_bytes).unwrap_or_else(|e| {
                eprintln!("Error: failed to deserialize certificate: {}", e);
                process::exit(1);
            });

        let rspu_program = result.rspu_program.as_ref().unwrap_or_else(|| {
            eprintln!("Error: R-SPU program was not compiled successfully");
            process::exit(1);
        });

        let binary_words = nasa_rust_project::emit::rspu_encoding::emit_binary(rspu_program)
            .unwrap_or_else(|e| {
                eprintln!("Error: failed to encode R-SPU binary: {}", e);
                process::exit(1);
            });

        match nasa_rust_project::cert::verify_certificate(&cert, rspu_program, &binary_words) {
            Ok(()) => {
                println!("Proof certificate verification PASSED.");
                process::exit(0);
            }
            Err(e) => {
                eprintln!("Proof certificate verification FAILED: {}", e);
                process::exit(1);
            }
        }
    }

    summary::print_summary(result, args.stats);

    // Check for width errors — render through the diagnostic engine.
    if result.has_width_errors() {
        if let Some(ref wr) = result.width_result {
            let mut width_diags = Vec::new();
            for (_, diags) in &wr.phase4a.assignment_results {
                width_diags.extend(diags);
            }
            for (_, r) in &wr.phase4a.guard_results {
                width_diags.extend(&r.diagnostics);
            }
            width_diags.extend(&wr.scc_diagnostics);
            width_diags.extend(&wr.verification.diagnostics);
            for wd in &width_diags {
                let d = wd.to_diagnostic();
                let rendered =
                    nasa_rust_project::diagnostic::render_diagnostic(&d, &source, &root_file);
                eprint!("{}", rendered);
            }
        }
        eprintln!("Width errors detected — output may be incomplete.");
    }

    let format = args.emit.as_deref().unwrap_or("dot");
    let output = match format {
        "dot" => {
            if args.dot_detail {
                emit::dot::emit_expr_dot(result)
            } else {
                emit::dot::emit_module_dot(result)
            }
        }
        "verilog" | "sv" => {
            let t = if fpga_target == emit::fpga_target::FpgaTarget::Generic {
                None
            } else {
                Some(fpga_target)
            };
            if args.strip_sva {
                emit::verilog::emit_sv_synthesis(result, t, args.dsp_threshold)
            } else {
                emit::verilog::emit_sv_with_options(result, t, args.dsp_threshold)
            }
        }
        "json" => emit::json_netlist::emit_json(result).unwrap_or_else(|e| {
            eprintln!("Error serializing JSON: {e}");
            process::exit(1);
        }),
        "sva" => emit::verilog::emit_sva_only(result),
        "firrtl" => emit::firrtl::emit_firrtl(result),
        "rspu" => result.rspu_program.as_ref().map(|p| p.emit_asm()).unwrap_or_else(|| {
            eprintln!("Error: R-SPU program not generated.");
            process::exit(1);
        }),
        "riscv" => result
            .rspu_program
            .as_ref()
            .map(|p| emit::riscv::emit_riscv_asm(p).unwrap())
            .unwrap_or_else(|| {
                eprintln!("Error emitting RISC-V.");
                process::exit(1);
            }),
        "arm" => result
            .rspu_program
            .as_ref()
            .map(|p| emit::arm::emit_arm_asm(p).unwrap())
            .unwrap_or_else(|| {
                eprintln!("Error emitting ARM.");
                process::exit(1);
            }),
        "testbench" => emit::testbench::emit_testbench(result),
        "scaffold" => emit::fpga_scaffold::emit_constraints(result, &fpga_target),
        "build-script" => emit::fpga_scaffold::emit_build_script(result, &fpga_target),
        "sexpr" => emit::sexpr::emit_sexpr(result),
        "mape-k-rtl" => result.mape_k_rtl.clone().expect("MAPE-K RTL skipped"),
        "cert" => {
            let cert_bytes = result
                .rspu_program
                .as_ref()
                .and_then(|p| p.certificate.as_ref())
                .expect("Certificate missing");
            if let Some(path) = &args.output {
                std::fs::write(path, cert_bytes).unwrap();
                return;
            }
            cert_bytes.iter().map(|b| format!("{:02x}", b)).collect()
        }
        _ => {
            eprintln!("Unknown format: {format}");
            process::exit(1);
        }
    };

    if let Some(path) = &args.output {
        std::fs::write(path, &output).expect("Error writing output");
        eprintln!("Output written to {path}");
    } else {
        print!("{output}");
    }

    if (format == "verilog" || format == "sv") && args.testbench {
        let tb = emit::testbench::emit_testbench(result);
        let path = derive_path(&root_file, "_tb.sv");
        std::fs::write(&path, tb).unwrap();
        eprintln!("Testbench written to {path}");
    }

    if (format == "verilog" || format == "sv") && args.scaffold {
        let constraints = emit::fpga_scaffold::emit_constraints(result, &fpga_target);
        let ext = fpga_target.constraint_extension();
        let constr_path = derive_path(&root_file, &format!(".{ext}"));
        std::fs::write(&constr_path, constraints).unwrap();
        eprintln!("Constraints written to {constr_path}");

        let build = emit::fpga_scaffold::emit_build_script(result, &fpga_target);
        let build_ext = match fpga_target {
            FpgaTarget::LatticeIce40
            | FpgaTarget::LatticeEcp5
            | FpgaTarget::LatticeNexus
            | FpgaTarget::Generic => "sh",
            _ => "tcl",
        };
        let build_path = derive_path(&root_file, &format!("_build.{build_ext}"));
        std::fs::write(&build_path, build).unwrap();
        eprintln!("Build script written to {build_path}");
    }

    if let Some(path) = &args.sva_file {
        let sva_content = emit::verilog::emit_sva_bind_file(result);
        if !sva_content.is_empty() {
            std::fs::write(path, sva_content).unwrap();
        }
    }

    if args.sync_stages != 2 && (format == "verilog" || format == "sv") {
        eprintln!("  Sync stages: {}", args.sync_stages);
    }

    if args.formal || args.lint || args.simulate || args.pnr || args.timing || args.eqy {
        toolchain::run_toolchain_operations(
            result,
            &root_file,
            &fpga_target,
            args.dsp_threshold,
            args.formal,
            args.formal_depth,
            args.formal_prove,
            &args.formal_engine,
            args.lint,
            args.simulate,
            args.pnr,
            args.timing,
            args.eqy,
            args.toolchain_path.as_deref(),
        );
    }
}

pub(crate) fn derive_path(input_path: &str, suffix: &str) -> String {
    if let Some(dot_pos) = input_path.rfind('.') {
        format!("{}{}", &input_path[..dot_pos], suffix)
    } else {
        format!("{input_path}{suffix}")
    }
}
