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

use std::io;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();

    if let Err(e) = nasa_rust_project::lsp::server::run(&mut input, &mut output) {
        eprintln!("mirr-lsp: {e}");
        std::process::exit(1);
    }
}
