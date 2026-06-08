//! CLI tool for MIRR logic simplification.
//!
//! Supports two modes:
//!   1. Bare Expr JSON: `mirr-simplify <expr.json>`
//!   2. Full .mirr file: `mirr-simplify <program.mirr> [--stats]`
//!
//! In .mirr mode, simplifies all guard conditions and reflex RHS expressions,
//! printing a summary of rule applications and node count reduction.

#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::process;

use mirrc::ast::Expr;
use mirrc::diagnostic::{render_diagnostic, Diagnostic};
use mirrc::error::MirrError;
use mirrc::parse_mirr;
use mirrc::simplify::{simplify_expr_with_stats, SimplifyStats};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage(args.is_empty());
        process::exit(if args.is_empty() { 1 } else { 0 });
    }

    let show_stats = args.iter().any(|a| a == "--stats");
    let input_path = args.iter().find(|a| !a.starts_with('-'));

    let input_path = match input_path {
        Some(p) => p.clone(),
        None => {
            fatal_diagnostic(
                Diagnostic::error("no input file specified")
                    .with_help("Pass a .json or .mirr file, or run with --help."),
            );
        }
    };

    let content = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            fatal_diagnostic(
                Diagnostic::error(format!("cannot read '{}'", input_path))
                    .with_note(e.to_string())
                    .with_help("Check the file path and permissions."),
            );
        }
    };

    if input_path.ends_with(".mirr") {
        run_mirr_mode(&content, show_stats, &input_path);
    } else {
        run_json_mode(&content, show_stats);
    }
}

fn run_json_mode(content: &str, show_stats: bool) {
    let expr: Expr = match serde_json::from_str(content) {
        Ok(e) => e,
        Err(e) => {
            fatal_diagnostic(
                Diagnostic::error("invalid Expr JSON")
                    .with_note(e.to_string())
                    .with_help("Ensure the input file contains a serialized Expr value."),
            );
        }
    };

    let (simplified, stats) = simplify_expr_with_stats(expr);

    let output = serde_json::to_string_pretty(&simplified).expect("Failed to serialize output");
    println!("{}", output);

    if show_stats {
        print_stats("expr", &stats);
    }
}

fn run_mirr_mode(content: &str, show_stats: bool, input_path: &str) {
    let program = match parse_mirr(content) {
        Ok(p) => p,
        Err(e) => {
            fatal_rendered_error(&e, content, input_path);
        }
    };

    let mut total_stats = SimplifyStats { rules_applied: 0, nodes_before: 0, nodes_after: 0 };

    // Simplify guard conditions.
    let mut guard_count = 0;
    for g in &program.module.guards {
        let (_, stats) = simplify_expr_with_stats(g.condition.clone());
        total_stats.rules_applied += stats.rules_applied;
        total_stats.nodes_before += stats.nodes_before;
        total_stats.nodes_after += stats.nodes_after;
        guard_count += 1;
    }

    // Simplify reflex assignment RHS expressions.
    let mut reflex_assign_count = 0;
    for r in &program.module.reflexes {
        for a in &r.assignments {
            let (_, stats) = simplify_expr_with_stats(a.value.clone());
            total_stats.rules_applied += stats.rules_applied;
            total_stats.nodes_before += stats.nodes_before;
            total_stats.nodes_after += stats.nodes_after;
            reflex_assign_count += 1;
        }
    }

    println!("MIRR Simplification Summary: {}", program.module.name);
    println!("  Guards simplified:     {}", guard_count);
    println!("  Assignments simplified: {}", reflex_assign_count);
    println!("  Total rules applied:   {}", total_stats.rules_applied);
    println!(
        "  Node count:            {} -> {} ({} reduced)",
        total_stats.nodes_before,
        total_stats.nodes_after,
        total_stats.nodes_before.saturating_sub(total_stats.nodes_after),
    );

    if show_stats {
        print_stats("total", &total_stats);
    }
}

fn print_stats(label: &str, stats: &SimplifyStats) {
    eprintln!(
        "[stats:{}] rules={} nodes_before={} nodes_after={}",
        label, stats.rules_applied, stats.nodes_before, stats.nodes_after,
    );
}

fn print_usage(is_error: bool) {
    if is_error {
        let diag = Diagnostic::error("invalid CLI invocation")
            .with_help("Usage: mirr-simplify <expr.json | program.mirr> [--stats]")
            .with_note(
                "Modes: .json simplifies a bare Expr JSON; .mirr simplifies guard/reflex expressions.",
            )
            .with_note("Flags: --stats prints before/after node counts and rules applied.");
        eprint!("{}", render_diagnostic(&diag, "", ""));
        process::exit(1);
    } else {
        eprintln!("Usage: mirr-simplify <expr.json | program.mirr> [--stats]");
        eprintln!("  .json file  — simplify a bare Expr JSON and print the result");
        eprintln!("  .mirr file  — simplify all guard/reflex expressions, print summary");
        eprintln!("  --stats     Print before/after node counts and rules applied");
    }
}

fn fatal_rendered_error(error: &MirrError, source: &str, file_path: &str) -> ! {
    eprint!("{}", render_diagnostic(&error.to_diagnostic(), source, file_path));
    process::exit(1);
}

fn fatal_diagnostic(diag: Diagnostic) -> ! {
    eprint!("{}", render_diagnostic(&diag, "", ""));
    process::exit(1);
}
