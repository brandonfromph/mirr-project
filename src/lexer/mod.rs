//! Lexical analysis module for MIRR source code.
//!
//! Re-exports the tokenizer and `Token` enum.

#![forbid(unsafe_code)]

pub mod tokenizer;

pub use tokenizer::{tokenize_expr, Token};
