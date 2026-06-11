#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};

#[path = "mirr-compile/main.rs"]
mod compile;

#[path = "mirr-lsp.rs"]
mod lsp;

#[path = "mirr-proof-audit.rs"]
mod proof_audit;

#[path = "mirr-audit.rs"]
mod audit;

#[path = "mirr-general.rs"]
mod general;

#[path = "generate_mirr_stress.rs"]
mod generate_mirr_stress;

#[path = "mirr-brain.rs"]
mod mirr_brain;

#[derive(Parser, Debug)]
#[command(
    name = "mirr",
    author,
    version,
    about = "Unified MIRR Router",
    long_about = "The unified CLI for the MIRR hardware compilation ecosystem."
)]
struct MirrCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile MIRR source code to RTL, ASM, or verification outputs
    Compile(compile::Cli),

    /// Run the MIRR Language Server Protocol (LSP) server
    Lsp(lsp::Args),

    /// Audit missing proofs for the MIRR compiler core
    ProofAudit(proof_audit::Args),

    /// MIRR Zero-Debt Compliance Engine
    Audit(audit::Args),

    /// MIRR General Orchestrator (CI/CD)
    General {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Generate MIRR stress test code
    GenerateStress(generate_mirr_stress::Args),

    /// MIRR Knowledge Core
    Brain(mirr_brain::Args),

    /// Knowledge Base Commands (Native Embeddings)
    #[command(disable_help_flag = true)]
    Kb {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Index Knowledge Base
    #[command(disable_help_flag = true)]
    KbIndex {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Hydrate Knowledge Base
    #[command(disable_help_flag = true)]
    KbHydrate {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = MirrCli::parse();

    match cli.command {
        Commands::Compile(args) => {
            compile::run(args)?;
        }
        Commands::Lsp(args) => {
            lsp::run(args);
        }
        Commands::ProofAudit(args) => {
            proof_audit::run(args)?;
        }
        Commands::Audit(args) => {
            audit::run(args)?;
        }
        Commands::General { args } => {
            general::run(args)?;
        }
        Commands::GenerateStress(args) => {
            generate_mirr_stress::run(args)?;
        }
        Commands::Brain(args) => {
            mirr_brain::run(args)?;
        }
        Commands::Kb { args } => {
            let exe_dir = std::env::current_exe().unwrap();
            let bin_dir = exe_dir.parent().unwrap();
            let mut cmd = std::process::Command::new(bin_dir.join("mirr-kb-native"));
            cmd.args(args);
            let status = cmd.status().expect("Failed to execute mirr-kb-native");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Commands::KbIndex { args } => {
            let exe_dir = std::env::current_exe().unwrap();
            let bin_dir = exe_dir.parent().unwrap();
            let mut cmd = std::process::Command::new(bin_dir.join("mirr-kb-index"));
            cmd.args(args);
            let status = cmd.status().expect("Failed to execute mirr-kb-index");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        Commands::KbHydrate { args } => {
            let exe_dir = std::env::current_exe().unwrap();
            let bin_dir = exe_dir.parent().unwrap();
            let mut cmd = std::process::Command::new(bin_dir.join("mirr-kb-hydrate"));
            cmd.args(args);
            let status = cmd.status().expect("Failed to execute mirr-kb-hydrate");
            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }
        }
    }

    Ok(())
}
