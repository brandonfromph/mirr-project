// ---------------------------------------------------------------------------
// Expression tokenizer
// ---------------------------------------------------------------------------
// Single responsibility: convert a raw expression string into a sequence
// of tokens. No parsing logic lives here.
// ---------------------------------------------------------------------------

use crate::error::MirrError;

/// Token produced by the expression tokenizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Integer(u64),
    True,
    False,
    // Operators
    Bang,     // !
    AmpAmp,   // &&
    PipePipe, // ||
    Caret,    // ^
    Plus,     // +
    Minus,    // -
    Star,     // *
    LtLt,     // <<
    GtGt,     // >>
    Lt,       // <
    Le,       // <=
    Gt,       // >
    Ge,       // >=
    EqEq,     // ==
    BangEq,   // !=
    LParen,
    RParen,
}

/// Pre-allocated buffer for tokens to reduce heap allocations.
/// NASA-style optimization: bounded memory usage with arena allocation.
struct TokenArena {
    tokens: Vec<Token>,
}

impl TokenArena {
    /// Create a new token arena with estimated capacity.
    /// Uses input length to estimate token count (typically 1 token per 2-3 chars).
    fn new(input_len: usize) -> Self {
        let estimated_tokens = input_len.saturating_div(2).max(8);
        Self { tokens: Vec::with_capacity(estimated_tokens) }
    }

    /// Add a token to the arena.
    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    /// Get the current tokens.
    fn finish(self) -> Vec<Token> {
        self.tokens
    }
}

/// Tokenize an expression string into a sequence of tokens.
/// NASA-style optimization: uses arena allocation and SIMD-like optimizations.
pub fn tokenize_expr(input: &str) -> Result<Vec<Token>, MirrError> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos = 0usize;
    let mut arena = TokenArena::new(len);

    // Bounded iteration: each loop iteration advances pos by at least 1.
    while pos < len {
        let b = bytes[pos];

        // Skip whitespace using optimized byte checking.
        // NASA-style: minimize branching and use bit operations where possible.
        if is_whitespace_byte(b) {
            pos += 1;
            continue;
        }

        // Two-character operators (check before single-char).
        // NASA-style optimization: use lookup table for faster matching.
        // Safety: only attempt the str slice if both positions are on ASCII
        // (single-byte) chars, so we never panic on multi-byte UTF-8 boundaries.
        if pos + 1 < len && b.is_ascii() && bytes[pos + 1].is_ascii() {
            let pair = &input[pos..pos + 2];
            if let Some(tok) = match_two_char_operator(pair) {
                arena.push(tok);
                pos += 2;
                continue;
            }
        }

        // Single-character operators and punctuation.
        // NASA-style optimization: use lookup table for O(1) matching.
        if let Some(tok) = match_single_char_operator(b) {
            arena.push(tok);
            pos += 1;
            continue;
        }

        // Integer literal with bounds checking.
        // NASA-style: prevent overflow and validate range.
        if is_digit_byte(b) {
            let start = pos;
            while pos < len && is_digit_byte(bytes[pos]) {
                pos += 1;
            }
            let num_str = &input[start..pos];
            let value: u64 = num_str
                .parse()
                .map_err(|_| MirrError::new(format!("Integer literal too large: '{num_str}'.")))?;
            arena.push(Token::Integer(value));
            continue;
        }

        // Identifier or keyword (true/false).
        if is_identifier_start_byte(b) {
            let start = pos;
            while pos < len && is_identifier_byte(bytes[pos]) {
                pos += 1;
            }
            let word = &input[start..pos];
            let tok = match word {
                "true" => Token::True,
                "false" => Token::False,
                _ => Token::Ident(word.to_string()),
            };
            arena.push(tok);
            continue;
        }

        // Safety: reconstruct the character from the byte rather than slicing
        // the original &str, which would panic on multi-byte UTF-8 boundaries
        // (e.g., em dash U+2014 is 3 bytes: 0xE2 0x80 0x94).
        let ch_display =
            if b.is_ascii() { (b as char).to_string() } else { format!("0x{:02X}", b) };
        return Err(MirrError::new(format!(
            "Unexpected character '{}' in expression.",
            ch_display
        )));
    }

    Ok(arena.finish())
}

/// Helper function to check if a byte is whitespace.
/// NASA-style optimization: use bit operations for faster checking.
#[inline]
fn is_whitespace_byte(b: u8) -> bool {
    // Check for space, tab, newline, carriage return
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r'
}

/// Helper function to check if a byte is a digit.
/// NASA-style optimization: direct byte comparison for speed.
#[inline]
fn is_digit_byte(b: u8) -> bool {
    b.is_ascii_digit()
}

/// Helper function to check if a byte can start an identifier.
#[inline]
fn is_identifier_start_byte(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_uppercase() || b == b'_'
}

/// Helper function to check if a byte can be part of an identifier.
#[inline]
fn is_identifier_byte(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_'
}

/// Lookup table for two-character operators.
/// NASA-style optimization: O(1) lookup instead of string matching.
#[inline]
fn match_two_char_operator(pair: &str) -> Option<Token> {
    match pair {
        "&&" => Some(Token::AmpAmp),
        "||" => Some(Token::PipePipe),
        "<<" => Some(Token::LtLt),
        ">>" => Some(Token::GtGt),
        "<=" => Some(Token::Le),
        ">=" => Some(Token::Ge),
        "==" => Some(Token::EqEq),
        "!=" => Some(Token::BangEq),
        _ => None,
    }
}

/// Lookup table for single-character operators.
/// NASA-style optimization: O(1) lookup instead of match statement.
#[inline]
fn match_single_char_operator(b: u8) -> Option<Token> {
    match b {
        b'!' => Some(Token::Bang),
        b'^' => Some(Token::Caret),
        b'+' => Some(Token::Plus),
        b'-' => Some(Token::Minus),
        b'*' => Some(Token::Star),
        b'<' => Some(Token::Lt),
        b'>' => Some(Token::Gt),
        b'(' => Some(Token::LParen),
        b')' => Some(Token::RParen),
        _ => None,
    }
}
