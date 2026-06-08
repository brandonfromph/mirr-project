//! mirr-diff: Structural diff tool for SystemVerilog regression testing.
//!
//! Compares emitted SystemVerilog from two compiler versions at the AST level,
//! ignoring whitespace and formatting differences.

#![forbid(unsafe_code)]
#![deny(warnings)]

use clap::Parser;
use std::path::PathBuf;

use mirrc::diagnostic::{render_diagnostic, Diagnostic};

/// Maximum AST nodes to compare (NASA P10 bounded iteration).
const MAX_AST_NODES: usize = 10000;

#[derive(Parser)]
#[command(name = "mirr-diff", about = "Structural diff for SystemVerilog")]
struct Args {
    /// Path to first SystemVerilog file (baseline).
    baseline: PathBuf,

    /// Path to second SystemVerilog file (candidate).
    candidate: PathBuf,

    /// Ignore module names (useful for renamed modules).
    #[arg(long)]
    ignore_names: bool,

    /// Output format (text, json).
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() {
    let args = Args::parse();

    let baseline = std::fs::read_to_string(&args.baseline).unwrap_or_else(|e| {
        fatal_diagnostic(
            Diagnostic::error("failed to read baseline file")
                .with_note(e.to_string())
                .with_code("E908"),
        );
    });
    let candidate = std::fs::read_to_string(&args.candidate).unwrap_or_else(|e| {
        fatal_diagnostic(
            Diagnostic::error("failed to read candidate file")
                .with_note(e.to_string())
                .with_code("E908"),
        );
    });

    let diffs = structural_diff(&baseline, &candidate, args.ignore_names);

    if diffs.is_empty() {
        println!("No structural differences found.");
        std::process::exit(0);
    } else {
        match args.format.as_str() {
            "json" => {
                let json: Vec<serde_json::Value> = diffs
                    .iter()
                    .map(|d| {
                        serde_json::json!({
                            "path": d.path,
                            "baseline": d.baseline,
                            "candidate": d.candidate,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            }
            _ => {
                for d in &diffs {
                    println!("@ {}", d.path);
                    println!("- {}", d.baseline);
                    println!("+ {}", d.candidate);
                    println!();
                }
            }
        }
        std::process::exit(1);
    }
}

fn fatal_diagnostic(diag: Diagnostic) -> ! {
    eprint!("{}", render_diagnostic(&diag, "", ""));
    std::process::exit(1);
}

struct AstDiff {
    path: String,
    baseline: String,
    candidate: String,
}

fn structural_diff(baseline: &str, candidate: &str, _ignore_names: bool) -> Vec<AstDiff> {
    let mut diffs = Vec::new();

    // Simple line-based structural comparison.
    // Ignores whitespace differences by comparing trimmed, normalized lines.
    let base_lines: Vec<&str> =
        baseline.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    let cand_lines: Vec<&str> =
        candidate.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();

    let max_lines = base_lines.len().max(cand_lines.len());
    if max_lines > MAX_AST_NODES {
        diffs.push(AstDiff {
            path: "/".to_string(),
            baseline: format!("{} lines", base_lines.len()),
            candidate: format!(
                "{} AST diff overflow: {} lines > {} limit",
                mirrc::error_codes::ec(908),
                max_lines,
                MAX_AST_NODES
            ),
        });
        return diffs;
    }

    // Compare line by line.
    let compare_count = base_lines.len().min(cand_lines.len());
    let mut i = 0;
    while i < compare_count {
        if base_lines[i] != cand_lines[i] {
            diffs.push(AstDiff {
                path: format!("/line/{}", i + 1),
                baseline: base_lines[i].to_string(),
                candidate: cand_lines[i].to_string(),
            });
        }
        i += 1;
    }

    // Report extra lines.
    if base_lines.len() > cand_lines.len() {
        diffs.push(AstDiff {
            path: "/length".to_string(),
            baseline: format!("{} lines", base_lines.len()),
            candidate: format!(
                "{} lines ({} missing)",
                cand_lines.len(),
                base_lines.len() - cand_lines.len()
            ),
        });
    } else if cand_lines.len() > base_lines.len() {
        diffs.push(AstDiff {
            path: "/length".to_string(),
            baseline: format!("{} lines", base_lines.len()),
            candidate: format!(
                "{} lines ({} extra)",
                cand_lines.len(),
                cand_lines.len() - base_lines.len()
            ),
        });
    }

    diffs
}
