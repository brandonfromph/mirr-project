//! Language Server Protocol support for the MIRR compiler.
//!
//! Provides a synchronous LSP server that publishes diagnostics from the
//! MIRR compilation pipeline. Communicates via JSON-RPC over stdin/stdout.
//!
//! Zero external dependencies beyond `serde_json` (already in Cargo.toml).

#![forbid(unsafe_code)]

pub mod diagnostics;
pub mod server;
pub mod transport;
