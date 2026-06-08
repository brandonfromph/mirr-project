//! mirr-explain: Compilation trace tool for MIRR developers.
//!
//! Given a signal, guard, or reflex name, produces a full compilation trace:
//! - Width inference iterations
//! - Temporal automaton produced
//! - R-SPU instructions emitted
//! - Cross-references to other constructs

#![forbid(unsafe_code)]
#![deny(warnings)]

use clap::Parser;
use std::path::PathBuf;

use mirrc::diagnostic::{render_diagnostic, Diagnostic};

#[derive(Parser)]
#[command(name = "mirr-explain", about = "Compilation trace tool for MIRR")]
struct Args {
    /// Path to MIRR source file.
    #[arg(short, long)]
    source: PathBuf,

    /// Name of construct to explain (signal, guard, or reflex).
    #[arg(short, long)]
    name: String,

    /// Verbosity level (0=summary, 1=normal, 2=verbose, 3=debug).
    #[arg(short, long, default_value_t = 1)]
    verbosity: u8,

    /// Output format (text, json, markdown).
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    let args = Args::parse();

    let source = std::fs::read_to_string(&args.source).unwrap_or_else(|e| {
        fatal_diagnostic(
            Diagnostic::error(format!("failed to read {}", args.source.display()))
                .with_note(e.to_string())
                .with_code("E907"),
        );
    });

    let program = mirrc::parse_mirr(&source).unwrap_or_else(|e| {
        fatal_diagnostic(
            Diagnostic::error("failed to parse MIRR source")
                .with_note(e.to_string())
                .with_code("E907"),
        );
    });

    // Find the construct by name.
    let found_signal = program.module.signals.iter().find(|s| s.name == args.name);
    let found_guard = program.module.guards.iter().find(|g| g.name == args.name);
    let found_reflex = program.module.reflexes.iter().find(|r| r.name == args.name);

    if found_signal.is_none() && found_guard.is_none() && found_reflex.is_none() {
        fatal_diagnostic(
            Diagnostic::error(format!("explain target not found: {}", args.name))
                .with_help("Use a signal, guard, or reflex name from the input file.")
                .with_code("E907"),
        );
    }

    let mut output = String::new();

    if let Some(sig) = found_signal {
        output.push_str(&format!("=== signal `{}` ===\n\n", sig.name));
        output.push_str(&format!("Kind: {:?}\n", sig.kind));
        output.push_str(&format!("Type: {}\n", sig.ty.core));
        if args.verbosity >= 2 {
            output.push_str(&format!("Span: {:?}\n", sig.span));
        }
    }

    if let Some(guard) = found_guard {
        output.push_str(&format!("=== guard `{}` ===\n\n", guard.name));
        output.push_str(&format!(
            "Condition: {}\n",
            mirrc::emit::expr_text(&guard.condition)
        ));
        output.push_str(&format!("Cycles: {}\n", guard.cycles));
        if args.verbosity >= 2 {
            output.push_str(&format!("Span: {:?}\n", guard.span));
        }
    }

    if let Some(reflex) = found_reflex {
        output.push_str(&format!("=== reflex `{}` ===\n\n", reflex.name));
        output.push_str(&format!("Guards: {}\n", reflex.guard_names.join(", ")));
        output.push_str("Assignments:\n");
        for assign in &reflex.assignments {
            output.push_str(&format!(
                "  {} = {}\n",
                assign.target,
                mirrc::emit::expr_text(&assign.value)
            ));
        }
        if args.verbosity >= 2 {
            output.push_str(&format!("Span: {:?}\n", reflex.span));
        }
    }

    // Cross-references.
    if args.verbosity >= 1 {
        output.push_str("\n## Cross-references\n\n");
        if found_signal.is_some() {
            let refs: Vec<&str> = program
                .module
                .guards
                .iter()
                .filter(|g| {
                    // Simple check: does the condition text contain the signal name.
                    let text = mirrc::emit::expr_text(&g.condition);
                    text.contains(&args.name)
                })
                .map(|g| g.name.as_str())
                .collect();
            if !refs.is_empty() {
                output.push_str(&format!("Used by guards: {}\n", refs.join(", ")));
            }
        }
    }

    match args.format.as_str() {
        "json" => {
            let json = serde_json::json!({
                "construct_type": if found_signal.is_some() { "signal" }
                    else if found_guard.is_some() { "guard" }
                    else { "reflex" },
                "name": args.name,
                "output": output,
            });
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
        "markdown" => {
            println!(
                "# {} `{}`\n\n{}",
                if found_signal.is_some() {
                    "signal"
                } else if found_guard.is_some() {
                    "guard"
                } else {
                    "reflex"
                },
                args.name,
                output
            );
        }
        _ => {
            print!("{output}");
        }
    }
}

fn fatal_diagnostic(diag: Diagnostic) -> ! {
    eprint!("{}", render_diagnostic(&diag, "", ""));
    std::process::exit(1);
}
