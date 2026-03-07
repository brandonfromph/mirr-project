//! CLI tool for MIRR logic simplification.
//!
//! Supports two modes:
//!   1. Bare Expr JSON: `mirr-simplify <expr.json>`
//!   2. Full .mirr file: `mirr-simplify <program.mirr> [--stats]`
//!
//! In .mirr mode, simplifies all guard conditions and reflex RHS expressions,
//! printing a summary of rule applications and node count reduction.

use std::env;
use std::fs;
use std::process;

use nasa_rust_project::ast::Expr;
use nasa_rust_project::simplify::{simplify_expr_with_stats, SimplifyStats};
use nasa_rust_project::parse_mirr;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!("Usage: mirr-simplify <expr.json | program.mirr> [--stats]");
        eprintln!();
        eprintln!("Modes:");
        eprintln!("  .json file  — simplify a bare Expr JSON and print the result");
        eprintln!("  .mirr file  — simplify all guard/reflex expressions, print summary");
        eprintln!();
        eprintln!("Flags:");
        eprintln!("  --stats     Print before/after node counts and rules applied");
        process::exit(if args.is_empty() { 1 } else { 0 });
    }

    let show_stats = args.iter().any(|a| a == "--stats");
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
        run_mirr_mode(&content, show_stats);
    } else {
        run_json_mode(&content, show_stats);
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

    let (simplified, stats) = simplify_expr_with_stats(expr);

    let output = serde_json::to_string_pretty(&simplified).expect("Failed to serialize output");
    println!("{}", output);

    if show_stats {
        print_stats("expr", &stats);
    }
}

fn run_mirr_mode(content: &str, show_stats: bool) {
    let program = match parse_mirr(content) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };

    let mut total_stats = SimplifyStats {
        rules_applied: 0,
        nodes_before: 0,
        nodes_after: 0,
    };

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
