//! Multi-module symbol table and cross-module symbol resolution.
//!
//! This module provides infrastructure for resolving symbols across module
//! boundaries in MIRR programs. It supports:
//! - Hierarchical symbol tables for multiple modules
//! - Cross-module symbol resolution via qualified names (alias.symbol)
//! - Import management and module scoping
//! - Type-preserving symbol lookups
//!
//! Error codes: E901-E920 (see `docs/error_codes.md`).

#![forbid(unsafe_code)]

pub mod resolver;
pub mod table;

pub use resolver::*;
pub use table::*;
