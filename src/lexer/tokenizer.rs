//! Hand-written tokenizer for the MIRR language.
//!
//! Scans MIRR source bytes into a sequence of `Token` values. Bounded iteration
//! (NASA P10 rule #1): the main loop is bounded by input length.

#![forbid(unsafe_code)]

use crate::error::MirrError;

/// Token produced by the expression tokenizer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// An identifier (signal name, keyword, etc.).
    Ident(String),
    /// An integer literal.
    Integer(u64),
    /// The `true` keyword.
    True,
    /// The `false` keyword.
    False,
    /// `!` — logical/bitwise NOT.
    Bang,
    /// `&&` — logical AND.
    AmpAmp,
    /// `||` — logical OR.
    PipePipe,
    /// `^` — bitwise XOR.
    Caret,
    /// `+` — addition.
    Plus,
    /// `-` — subtraction or unary negation.
    Minus,
    /// `*` — multiplication.
    Star,
    /// `<<` — left shift.
    LtLt,
    /// `>>` — right shift.
    GtGt,
    /// `<` — less than.
    Lt,
    /// `<=` — less than or equal.
    Le,
    /// `>` — greater than.
    Gt,
    /// `>=` — greater than or equal.
    Ge,
    /// `==` — equality.
    EqEq,
    /// `!=` — inequality.
    BangEq,
    /// `(` — left parenthesis.
    LParen,
    /// `)` — right parenthesis.
    RParen,
    /// `{` — left brace.
    LBrace,
    /// `}` — right brace.
    RBrace,
    /// `[` — left bracket.
    LBracket,
    /// `]` — right bracket.
    RBracket,
    /// `,` — element separator.
    Comma,
    /// `:` — key/value separator in struct literals.
    Colon,
    /// `.` — field access separator.
    Dot,
}

/// Tokenize an expression string into a sequence of tokens.
pub fn tokenize_expr(input: &str) -> Result<Vec<Token>, MirrError> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos = 0usize;
    let estimated_tokens = len.saturating_div(2).max(8);
    let mut tokens = Vec::with_capacity(estimated_tokens);

    // Bounded iteration: each loop iteration advances pos by at least 1.
    while pos < len {
        let b = bytes[pos];

        // Skip whitespace.
        if is_whitespace_byte(b) {
            pos += 1;
            continue;
        }

        // Two-character operators (check before single-char).
        // Safety: only attempt the str slice if both positions are on ASCII
        // (single-byte) chars, so we never panic on multi-byte UTF-8 boundaries.
        if pos + 1 < len && b.is_ascii() && bytes[pos + 1].is_ascii() {
            let pair = &input[pos..pos + 2];
            if let Some(tok) = match_two_char_operator(pair) {
                tokens.push(tok);
                pos += 2;
                continue;
            }
        }

        // Single-character operators.
        if let Some(tok) = match_single_char_operator(b) {
            tokens.push(tok);
            pos += 1;
            continue;
        }

        // Integer literal with bounds checking.
        if b.is_ascii_digit() {
            let start = pos;
            let mut is_hex = false;

            if b == b'0' && pos + 1 < len && (bytes[pos + 1] == b'x' || bytes[pos + 1] == b'X') {
                is_hex = true;
                pos += 2;
                while pos < len && bytes[pos].is_ascii_hexdigit() {
                    pos += 1;
                }
            } else {
                while pos < len && bytes[pos].is_ascii_digit() {
                    pos += 1;
                }
            }

            let num_str = &input[start..pos];
            let value: u64 = if is_hex {
                u64::from_str_radix(&num_str[2..], 16).map_err(|_| {
                    MirrError::parse_error(format!(
                        "{} Hex literal too large or invalid: '{num_str}'.",
                        crate::error_codes::ec(180)
                    ))
                })?
            } else {
                num_str.parse().map_err(|_| {
                    MirrError::parse_error(format!(
                        "{} Integer literal too large: '{num_str}'.",
                        crate::error_codes::ec(180)
                    ))
                })?
            };
            tokens.push(Token::Integer(value));
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
            tokens.push(tok);
            continue;
        }

        // Safety: reconstruct the character from the byte rather than slicing
        // the original &str, which would panic on multi-byte UTF-8 boundaries
        // (e.g., em dash U+2014 is 3 bytes: 0xE2 0x80 0x94).
        let ch_display =
            if b.is_ascii() { (b as char).to_string() } else { format!("0x{:02X}", b) };
        return Err(MirrError::parse_error(format!(
            "{} Unexpected character '{}' in expression.",
            crate::error_codes::ec(181),
            ch_display
        )));
    }

    Ok(tokens)
}

/// Returns true if byte is ASCII whitespace.
#[inline]
fn is_whitespace_byte(b: u8) -> bool {
    // Check for space, tab, newline, carriage return
    b == b' ' || b == b'\t' || b == b'\n' || b == b'\r'
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

/// Match a two-character operator token.
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

/// Match a single-character operator token.
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
        b'{' => Some(Token::LBrace),
        b'}' => Some(Token::RBrace),
        b'[' => Some(Token::LBracket),
        b']' => Some(Token::RBracket),
        b',' => Some(Token::Comma),
        b':' => Some(Token::Colon),
        b'.' => Some(Token::Dot),
        _ => None,
    }
}
