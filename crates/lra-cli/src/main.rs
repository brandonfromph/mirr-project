#![forbid(unsafe_code)]
#![deny(warnings)]

use clap::{Parser, Subcommand};

mod badge;
mod build;
mod build_docs;
mod init;
mod serve;
mod validate;

#[derive(Parser)]
#[command(name = "lra", version, about = "Living Research Artifact CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scaffold a new LRA project
    Init {
        /// Project name (creates a directory)
        name: String,
    },
    /// Validate an LRA against the spec
    Validate {
        /// Path to index.html (default: ./index.html)
        #[arg(default_value = "index.html")]
        path: String,
    },
    /// Start a local dev server with live reload
    Serve {
        /// Port number
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Print the compliance badge URL
    Badge {
        /// Path to index.html (default: ./index.html)
        #[arg(default_value = "index.html")]
        path: String,
    },
    /// Build Markdown into LRA-compliant HTML
    Build {
        /// Path to input Markdown file
        #[arg(default_value = "paper.md")]
        input: String,
        /// Output HTML file
        #[arg(short, long, default_value = "index.html")]
        output: String,
    },
    /// Build a docs directory of Markdown files into static HTML pages
    BuildDocs {
        /// Input directory containing .md files
        #[arg(default_value = "docs")]
        input_dir: String,
        /// Output directory for .html files
        #[arg(short, long, default_value = "_site")]
        output_dir: String,
        /// Path to CSS file (relative to output)
        #[arg(long, default_value = "style.css")]
        css: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let code = match cli.command {
        Command::Init { name } => init::run(&name),
        Command::Validate { path } => validate::run(&path),
        Command::Serve { port } => serve::run(port),
        Command::Badge { path } => badge::run(&path),
        Command::Build { input, output } => build::run(&input, &output),
        Command::BuildDocs { input_dir, output_dir, css } => {
            build_docs::run(&input_dir, &output_dir, &css)
        }
    };
    std::process::exit(code);
}
