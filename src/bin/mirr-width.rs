//! CLI tool for MIRR bit-width inference (Phase 4).
//!
//! Supports two modes:
//!   1. Bare Expr JSON: `mirr-width <expr.json>` with signal declarations
//!   2. Full .mirr file: `mirr-width <program.mirr> [--stats]`
//!
//! In .mirr mode, runs Phase 3 simplification first (automatically), then
//! infers widths for all guard conditions and reflex RHS expressions, checking
//! for unsafe truncations.

#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::process;

use nasa_rust_project::ast::Expr;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::simplify::simplify_expr;
use nasa_rust_project::width;
use nasa_rust_project::width::types::DiagSeverity;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage(args.is_empty());
        return;
    }

    let show_stats = args.iter().any(|a| a == "--stats");
    let scc_mode = args.iter().any(|a| a == "--scc");
    let input_path = args.iter().find(|a| !a.starts_with('-'));

    let input_path = match input_path {
        Some(p) => p.clone(),
        None => {
            eprintln!("Error: no input file specified");
            process::exit(1);
        }
    };

    let content = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: cannot read '{}': {}", input_path, e);
            process::exit(1);
        }
    };

    if input_path.ends_with(".mirr") {
        if scc_mode {
            run_scc_mode(&content, show_stats);
        } else {
            run_mirr_mode(&content, show_stats);
        }
    } else {
        run_json_mode(&content, show_stats);
    }
}

fn print_usage(is_error: bool) {
    eprintln!("Usage: mirr-width <expr.json | program.mirr> [--stats] [--scc]");
    eprintln!();
    eprintln!("Modes:");
    eprintln!("  .json file  -- infer widths for a bare Expr JSON (no signals)");
    eprintln!("  .mirr file  -- full program width analysis with truncation checks");
    eprintln!();
    eprintln!("Flags:");
    eprintln!("  --stats     Print detailed inference statistics");
    eprintln!("  --scc       Enable Phase 4b SCC analysis (requires .mirr input)");
    if is_error {
        process::exit(1);
    }
}

fn run_json_mode(content: &str, show_stats: bool) {
    let expr: Expr = match serde_json::from_str(content) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error: invalid Expr JSON: {}", e);
            process::exit(1);
        }
    };

    // Run simplification first.
    let simplified = simplify_expr(expr);

    // Infer widths with no signal declarations (literals-only mode).
    let result = width::infer_widths(&simplified, &[]);

    if let Some(we) = &result.expr {
        println!("{}", width::display::format_width_expr(we));
    }

    print_diagnostics(&result.diagnostics);

    if show_stats {
        eprintln!("[stats] {}", width::display::format_stats(&result.stats));
    }

    if result.has_errors() {
        process::exit(1);
    }
}

fn run_mirr_mode(content: &str, show_stats: bool) {
    let mut program = match parse_mirr(content) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };

    // Run Phase 3 simplification on all expressions first.
    simplify_program(&mut program);

    // Run full width inference.
    let result = width::infer_program_widths(&program);

    // Print summary.
    println!("MIRR Width Analysis: {}", program.module.name);
    println!("  Guards analyzed:      {}", result.guard_results.len());
    println!("  Assignments analyzed: {}", result.assignment_results.len());

    // Print per-guard results.
    for (name, r) in &result.guard_results {
        if let Some(we) = &r.expr {
            println!("  guard '{}': {}", name, we.width());
        }
    }

    // Print all diagnostics.
    let all_diags = result.all_diagnostics();
    let error_count = all_diags.iter().filter(|d| d.severity == DiagSeverity::Error).count();
    let info_count = all_diags.iter().filter(|d| d.severity == DiagSeverity::Info).count();

    println!("  Diagnostics: {} error(s), {} info(s)", error_count, info_count,);

    for d in &all_diags {
        println!("  {}", d);
    }

    if show_stats {
        eprintln!("[stats] {}", width::display::format_stats(&result.stats));
    }

    if result.has_errors() {
        process::exit(1);
    }
}

fn run_scc_mode(content: &str, show_stats: bool) {
    let mut program = match parse_mirr(content) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };

    // Run Phase 3 simplification on all expressions first.
    simplify_program(&mut program);

    // Run full Phase 4b SCC-based width inference.
    let result = width::infer_program_widths_with_scc(&program, None);

    // Print Phase 4a summary.
    println!("MIRR Width Analysis (Phase 4b): {}", program.module.name);
    println!("  Guards analyzed:      {}", result.phase4a.guard_results.len());
    println!("  Assignments analyzed: {}", result.phase4a.assignment_results.len());

    // Print SCC report.
    let signal_names: Vec<String> = program.module.signals.iter().map(|s| s.name.clone()).collect();
    print!("{}", width::display::format_scc_report(&result.sccs, &signal_names));

    // Print SCC diagnostics.
    for d in &result.scc_diagnostics {
        println!("  {}", d);
    }

    // Print verification result.
    if result.verification.is_minimal {
        println!("  Least solution: VERIFIED");
    } else {
        println!("  Least solution: FAILED (see diagnostics)");
    }

    if show_stats {
        eprintln!("[stats] {}", width::display::format_stats(&result.stats));
    }

    if result.has_errors() {
        process::exit(1);
    }
}

/// Run Phase 3 simplification on all expressions in the program.
///
/// Bounded: iterates over guards + reflexes (finite, from parsed program).
fn simplify_program(program: &mut nasa_rust_project::MirrProgram) {
    for g in &mut program.module.guards {
        let simplified = simplify_expr(g.condition.clone());
        g.condition = simplified;
    }
    for r in &mut program.module.reflexes {
        for a in &mut r.assignments {
            let simplified = simplify_expr(a.value.clone());
            a.value = simplified;
        }
    }
}

fn print_diagnostics(diags: &[width::types::WidthDiag]) {
    for d in diags {
        println!("  {}", d);
    }
}
