//! mirr-compile — Unified MIRR compilation driver (Phase 6).
//!
//! End-to-end pipeline: parse -> validate -> simplify -> width -> temporal -> emit.
//!
//! Usage:
//!   mirr-compile <file.mirr> [--emit dot|verilog|json] [--output FILE] [--stats]
//!   mirr-compile <file.mirr> --emit dot --dot-detail expr [--output FILE]

#![forbid(unsafe_code)]

use std::process;

use nasa_rust_project::pipeline::{PipelineConfig, run_pipeline};
use nasa_rust_project::emit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut input_path: Option<String> = None;
    let mut emit_format: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut show_stats = false;
    let mut show_help = false;
    let mut dot_detail_expr = false;

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

    let source = match std::fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: cannot read '{input_path}': {e}");
            process::exit(1);
        }
    };

    // Run full pipeline.
    let config = PipelineConfig::default();
    let result = match run_pipeline(&source, &config) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
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
        "verilog" | "sv" => emit::verilog::emit_sv(&result),
        "json" => {
            match emit::json_netlist::emit_json(&result) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error serializing JSON: {e}");
                    process::exit(1);
                }
            }
        }
        other => {
            eprintln!("Unknown emit format: '{other}'. Use dot, verilog, or json.");
            process::exit(1);
        }
    };

    // Write output.
    match output_path {
        Some(path) => {
            if let Err(e) = std::fs::write(&path, &output) {
                eprintln!("Error writing '{path}': {e}");
                process::exit(1);
            }
            eprintln!("Output written to {path}");
        }
        None => {
            print!("{output}");
        }
    }
}

fn print_summary(
    result: &nasa_rust_project::pipeline::PipelineResult,
    show_stats: bool,
) {
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
        eprintln!(
            "  Temporal: {} guards, {} signals",
            tn.guards.len(),
            tn.signals.len(),
        );
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
    println!("  --emit FORMAT       Output format: dot, verilog, json (default: dot)");
    println!("  --output FILE, -o   Write output to FILE (default: stdout)");
    println!("  --dot-detail expr   Show full AST trees in DOT output");
    println!("  --stats             Print detailed pipeline statistics");
    println!("  --help, -h          Show this help");
    println!();
    println!("Examples:");
    println!("  mirr-compile program.mirr --emit verilog -o out.sv");
    println!("  mirr-compile program.mirr --emit json | jq .");
    println!("  mirr-compile program.mirr --emit dot | dot -Tpng -o graph.png");
}
