//! mirr-lsp — Language Server Protocol server for MIRR.
//!
//! Provides real-time diagnostics by running the MIRR compilation pipeline
//! on document changes and publishing errors/warnings via LSP.
//!
//! Communicates over stdin/stdout using the JSON-RPC protocol.
//! Zero async dependencies — pure synchronous I/O.
//!
//! Usage: `mirr-lsp` (launched by an editor; not intended to be run directly)

#![forbid(unsafe_code)]

use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[command(name = "lsp", about = "MIRR LSP server entrypoint")]
pub struct Args {
    /// Output mode for startup/runtime errors.
    #[arg(long, default_value = "text", value_parser = ["text", "json"])]
    pub format: String,
}

pub fn run(args: Args) {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();

    if let Err(e) = mirrc::lsp::server::run(&mut input, &mut output) {
        if args.format == "json" {
            println!(
                "{{\"ok\":false,\"binary\":\"mirr-lsp\",\"error\":\"{}\"}}",
                e.to_string().replace('"', "\\\"")
            );
        } else {
            eprintln!("mirr-lsp: {e}");
        }
        std::process::exit(1);
    }
}
