#![forbid(unsafe_code)]
#![deny(warnings)]

use clap::{Parser, Subcommand};
use mirrc::emit;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

mod badge;
mod build;
mod build_docs;
mod crawl;
mod deps;
mod hash;
mod health;
mod init;
mod keygen;
mod receipt;
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
        /// URL of the deployed paper (e.g., <https://example.github.io/paper/>)
        url: String,
    },
    /// Compile MIRR source to target format
    Compile {
        /// Path to MIRR source file
        #[arg(default_value = "main.mirr")]
        source: String,
        /// Target format (verilog, firrtl, rspu, json, sexpr, dot)
        #[arg(short, long, default_value = "verilog")]
        target: String,
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Generate a signed build certification receipt
    Receipt {
        /// Path to MIRR source file
        #[arg(short, long)]
        source: String,
        /// Path to compiled output
        #[arg(short, long)]
        output: String,
        /// Path to Ed25519 secret key
        #[arg(short, long, default_value = "lra-identity.key")]
        key: String,
        /// Output receipt path
        #[arg(long, default_value = "build-receipt.json")]
        receipt: String,
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
        Command::Compile { source, target, output } => {
            compile_via_library(&source, &target, output.as_deref())
        }
        Command::Receipt { source, output, key, receipt: receipt_path } => {
            let source_path = std::path::Path::new(&source);
            let output_path = std::path::Path::new(&output);
            let key_path = std::path::Path::new(&key);

            let mut r = match receipt::generate_receipt(source_path, output_path, "verilog") {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            };

            if key_path.exists() {
                let key_bytes = match std::fs::read(key_path) {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("[E901] Receipt generation failed: {e}");
                        std::process::exit(1);
                    }
                };
                if let Err(e) = receipt::sign_receipt(&mut r, &key_bytes) {
                    eprintln!("{e}");
                    std::process::exit(1);
                }

                // Verify the signed receipt
                if let Err(e) = receipt::verify_receipt(&r, &key_bytes[32..]) {
                    eprintln!("[E903] Receipt verification failed: {e}");
                    std::process::exit(1);
                }
                println!("Receipt signature verified");
            }

            let json = match serde_json::to_string_pretty(&r) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("[E901] Receipt generation failed: {e}");
                    std::process::exit(1);
                }
            };

            if let Err(e) = std::fs::write(&receipt_path, &json) {
                eprintln!("[E901] Receipt generation failed: {e}");
                std::process::exit(1);
            }

            println!("Receipt written to {receipt_path}");
            std::process::exit(0);
        }
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

fn compile_via_library(source: &str, target: &str, output: Option<&str>) -> i32 {
    let source_text = match std::fs::read_to_string(source) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error: cannot read '{source}': {e}");
            return 1;
        }
    };

    let mut config = PipelineConfig::default();
    if target == "rspu" {
        config.rspu = true;
    }

    let result = match run_pipeline(&source_text, &config) {
        Ok(result) => result,
        Err(errors) => {
            for err in &errors.errors {
                let diagnostic = err.to_diagnostic();
                let rendered =
                    mirrc::diagnostic::render_diagnostic(&diagnostic, &source_text, source);
                eprint!("{rendered}");
            }
            return 1;
        }
    };

    let compiled = match target {
        "verilog" | "sv" => emit::verilog::emit_sv(&result),
        "firrtl" => emit::firrtl::emit_firrtl(&result),
        "rspu" => match &result.rspu_program {
            Some(program) => program.emit_asm(),
            None => {
                eprintln!(
                    "Error: R-SPU program was not generated (pipeline may have been skipped)."
                );
                return 1;
            }
        },
        "json" => match emit::json_netlist::emit_json(&result) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error: JSON netlist serialization failed: {e}");
                return 1;
            }
        },
        "sexpr" | "s-expr" | "sexp" => emit::sexpr::emit_sexpr(&result),
        "dot" => emit::dot::emit_module_dot(&result),
        other => {
            eprintln!(
                "Unknown compile target: {}. Allowed targets: verilog, firrtl, rspu, json, sexpr, dot.",
                other
            );
            return 1;
        }
    };

    if let Some(path) = output {
        if let Err(e) = std::fs::write(path, &compiled) {
            eprintln!("Error writing '{path}': {e}");
            return 1;
        }
        println!("Output written to {path}");
    } else {
        print!("{compiled}");
    }

    0
}
