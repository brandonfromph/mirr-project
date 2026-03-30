//! # 🏛️ MRT COMPLIANCE MANDATE (Proposal 090)
//!
//! This file is part of the MIRR Runtime Tooling (MRT / Presidential Arsenal).
//! All modifications MUST adhere to the following standards:
//! 1. CDD LIFECYCLE: Audit → Propose → Sign → Execute → CI.
//! 2. ZERO-DEBT INVARIANT: No wrappers (D1), no dead code (D3), no stale comments (D7).
//! 3. KB STANDARD: All operational telemetry MUST be stashed in `mirr-brain`.
//! 4. NO VIBE-CODING: Surgical edits via `mirr-wave` only.

//! # 🎖ï¸ mirr-general: The Orchestrator
//!
//! `mirr-general` is the central command node of the Presidential Arsenal.
//! It coordinates the other specialized CLIs to manage the lifecycle of a change.

use clap::{Parser, Subcommand};
use serde::Serialize;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "mirr-general", version, about = "MIRR Arsenal Orchestrator")]
struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Output format (text or json)
    #[arg(short, long, default_value = "text")]
    format: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a full audit and consult the brain.
    Audit,
    /// Execute a signed wave by ID.
    Wave {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        file: String,
    },
    /// Run the full CI gate.
    Ci,
}

#[derive(Serialize, Debug)]
struct GeneralResponse {
    pub status: String,
    pub mission: String,
    pub details: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut response = GeneralResponse {
        status: "STARTED".to_string(),
        mission: String::new(),
        details: Vec::new(),
    };

    match args.command {
        Commands::Audit => {
            let start = chrono::Utc::now();
            response.mission = "WORKSPACE_AUDIT".to_string();
            response.details.push("Consulting The Brain...".to_string());
            let status = Command::new("cargo")
                .args(["run", "--bin", "mirr-brain", "--", "laws"])
                .status()?;

            if status.success() {
                response.details.push("Initiating Refinement Audit...".to_string());
                let audit_status = Command::new("cargo")
                    .args([
                        "run",
                        "--bin",
                        "mirr-audit",
                        "--",
                        "--mode",
                        "refinement",
                        "--stash-key",
                        "last_audit",
                    ])
                    .status()?;

                if audit_status.success() {
                    response.status = "SUCCESS".to_string();
                    let duration = chrono::Utc::now().signed_duration_since(start);
                    response
                        .details
                        .push(format!("Audit Latency: {}ms", duration.num_milliseconds()));
                } else {
                    response.status = "FAILED".to_string();
                    response.details.push("Audit failed.".to_string());
                }
            } else {
                response.status = "FAILED".to_string();
                response.details.push("Could not consult the Brain.".to_string());
            }
        }
        Commands::Wave { id, file } => {
            response.mission = format!("WAVE_EXECUTION: {}", id);
            response.details.push("Consulting The Brain for Wave Integrity...".to_string());

            let wave_status = Command::new("cargo")
                .args(["run", "--bin", "mirr-wave", "--", "-i", &id, "-f", &file, "--stash"])
                .status()?;

            if wave_status.success() {
                response.status = "SUCCESS".to_string();
            } else {
                response.status = "FAILED".to_string();
                response.details.push("Wave execution failed.".to_string());
            }
        }
        Commands::Ci => {
            response.mission = "CI_GATE".to_string();
            response.details.push("Running NASA-Grade CI...".to_string());
            let status =
                Command::new("cargo").args(["nextest", "run", "--test-threads", "4"]).status()?;

            if status.success() {
                response.status = "SUCCESS".to_string();
            } else {
                response.status = "FAILED".to_string();
                response.details.push("CI Gate failed.".to_string());
            }
        }
    }

    if args.format == "json" {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        println!("--- [GENERAL] Mission: {} ---", response.mission);
        println!("Status: {}", response.status);
        for detail in &response.details {
            println!("  > {}", detail);
        }
        if response.status == "SUCCESS" {
            println!("--- [GENERAL] Mission Success ---");
        } else {
            println!("--- [GENERAL] Mission Failed ---");
        }
    }

    Ok(())
}
