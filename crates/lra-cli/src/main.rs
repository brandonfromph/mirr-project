#![forbid(unsafe_code)]
#![deny(warnings)]

use clap::{Parser, Subcommand};

mod badge;
mod build;
mod build_docs;
mod crawl;
mod deps;
mod hash;
mod health;
mod init;
mod keygen;
mod registry;
mod search;
mod serve;
mod sign;
mod status;
mod util;
mod validate;
mod verify;
mod verify_receipt;

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
    /// Compute the SHA-256 content hash of an LRA paper
    Hash {
        /// Path to index.html (default: ./index.html)
        #[arg(default_value = "index.html")]
        path: String,
    },
    /// Search the LRA registry for papers
    Search {
        /// Search query (matches title, keywords, capability, authors)
        query: String,
        /// Path to lra-registry.json (default: ./lra-registry.json)
        #[arg(short, long, default_value = "lra-registry.json")]
        registry: String,
    },
    /// Show the dependency graph for the current paper
    Deps {
        /// Path to index.html (default: ./index.html)
        #[arg(default_value = "index.html")]
        path: String,
        /// Path to lra-registry.json (default: ./lra-registry.json)
        #[arg(short, long, default_value = "lra-registry.json")]
        registry: String,
    },
    /// Check headless status of a deployed LRA paper
    Health {
        /// URL of the deployed paper (e.g., https://example.github.io/paper/)
        url: String,
    },
    /// Generate an Ed25519 keypair for signing verification receipts
    Keygen,
    /// Verify a deployed LRA paper's claims and content integrity
    Verify {
        /// URL or registry hash of the target paper
        target: String,
        /// Path to lra-registry.json (default: ./lra-registry.json)
        #[arg(short, long, default_value = "lra-registry.json")]
        registry: String,
        /// Optional: write a JSON verification receipt
        #[arg(long)]
        receipt: Option<String>,
    },
    /// Sign a verification receipt with an Ed25519 keypair
    Sign {
        /// Path to the receipt JSON file
        receipt: String,
        /// Path to Ed25519 secret key (default: ./lra-identity.key)
        #[arg(short, long, default_value = "lra-identity.key")]
        key: String,
    },
    /// Show network status for all papers in the registry
    Status {
        /// Path to lra-registry.json (default: ./lra-registry.json)
        #[arg(short, long, default_value = "lra-registry.json")]
        registry: String,
    },
    /// Crawl the LRA network from a seed URL
    Crawl {
        /// Seed URL to start crawling from
        seed: String,
        /// Path to lra-registry.json (default: ./lra-registry.json)
        #[arg(short, long, default_value = "lra-registry.json")]
        registry: String,
    },
    /// Verify a signed verification receipt
    VerifyReceipt {
        /// Path to the signed receipt JSON file
        receipt: String,
        /// Optional: path to a trusted public key file
        #[arg(short, long)]
        pubkey: Option<String>,
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
        Command::Hash { path } => hash::run(&path),
        Command::Search { query, registry } => search::run(&query, &registry),
        Command::Deps { path, registry } => deps::run(&path, &registry),
        Command::Health { url } => health::run(&url),
        Command::Keygen => keygen::run(),
        Command::Verify { target, registry, receipt } => {
            verify::run(&target, &registry, receipt.as_deref())
        }
        Command::Sign { receipt, key } => sign::run(&receipt, &key),
        Command::Status { registry } => status::run(&registry),
        Command::Crawl { seed, registry } => crawl::run(&seed, &registry),
        Command::VerifyReceipt { receipt, pubkey } => {
            verify_receipt::run(&receipt, pubkey.as_deref())
        }
    };
    std::process::exit(code);
}
