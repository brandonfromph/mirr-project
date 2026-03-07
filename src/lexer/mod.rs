//! Lexical analysis module for MIRR source code.
//!
//! Re-exports the tokenizer and `Token` enum.

pub mod tokenizer;

pub use tokenizer::{tokenize_expr, Token};
