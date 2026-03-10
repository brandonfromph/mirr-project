//! Bounded S-expression text parser.
//!
//! Parses S-expression text into `SExpr` values with explicit bounds
//! on depth, node count, and input size. No recursion — uses an
//! explicit stack per NASA Power-of-10 compliance.

#![forbid(unsafe_code)]

use crate::error::MirrError;
use crate::sexpr::types::SExpr;
use crate::sexpr::{MAX_SEXPR_DEPTH, MAX_SEXPR_NODES, MAX_SEXPR_STRING_LEN};

/// Token produced by the S-expression tokenizer.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    Quote,
    Backtick,
    Comma,
    Symbol(String),
    Integer(u64),
    Bool(bool),
    Str(String),
}

/// Tokenize an S-expression input string into a bounded token stream.
///
/// Maximum tokens = `MAX_SEXPR_NODES * 3` (worst case: every node bracketed).
fn tokenize(input: &str) -> Result<Vec<Token>, MirrError> {
    let max_tokens = MAX_SEXPR_NODES * 3;
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if tokens.len() >= max_tokens {
            return Err(sexpr_err("[E804] Token stream exceeds maximum size"));
        }

        match bytes[i] {
            b' ' | b'\t' | b'\n' | b'\r' => {
                i += 1;
            }
            b';' => {
                // Line comment: skip to end of line.
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'(' => {
                tokens.push(Token::OpenParen);
                i += 1;
            }
            b')' => {
                tokens.push(Token::CloseParen);
                i += 1;
            }
            b'\'' => {
                tokens.push(Token::Quote);
                i += 1;
            }
            b'`' => {
                tokens.push(Token::Backtick);
                i += 1;
            }
            b',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            b'"' => {
                // String literal.
                i += 1;
                let start = i;
                while i < bytes.len() && bytes[i] != b'"' {
                    if bytes[i] == b'\\' {
                        i += 1; // skip escaped char
                    }
                    i += 1;
                }
                if i >= bytes.len() {
                    return Err(sexpr_err("[E801] Unterminated string literal"));
                }
                let s = String::from_utf8_lossy(&bytes[start..i]).to_string();
                tokens.push(Token::Str(s));
                i += 1; // skip closing quote
            }
            b'#' => {
                // Boolean: #t / #f
                if i + 1 < bytes.len() && bytes[i + 1] == b't' {
                    tokens.push(Token::Bool(true));
                    i += 2;
                } else if i + 1 < bytes.len() && bytes[i + 1] == b'f' {
                    tokens.push(Token::Bool(false));
                    i += 2;
                } else {
                    return Err(sexpr_err("[E800] Invalid '#' token — expected #t or #f"));
                }
            }
            _ => {
                // Symbol or integer.
                let start = i;
                while i < bytes.len()
                    && !matches!(
                        bytes[i],
                        b' ' | b'\t'
                            | b'\n'
                            | b'\r'
                            | b'('
                            | b')'
                            | b';'
                            | b'"'
                            | b'\''
                            | b'`'
                            | b','
                    )
                {
                    i += 1;
                }
                let word = String::from_utf8_lossy(&bytes[start..i]).to_string();
                if word == "true" {
                    tokens.push(Token::Bool(true));
                } else if word == "false" {
                    tokens.push(Token::Bool(false));
                } else if let Ok(n) = word.parse::<u64>() {
                    tokens.push(Token::Integer(n));
                } else if word.starts_with("0x") || word.starts_with("0X") {
                    match u64::from_str_radix(&word[2..], 16) {
                        Ok(n) => tokens.push(Token::Integer(n)),
                        Err(_) => tokens.push(Token::Symbol(word)),
                    }
                } else {
                    tokens.push(Token::Symbol(word));
                }
            }
        }
    }
    Ok(tokens)
}

/// Parse an S-expression from text.
///
/// Bounded by `MAX_SEXPR_STRING_LEN`, `MAX_SEXPR_DEPTH`, and `MAX_SEXPR_NODES`.
/// Uses an explicit stack — no recursion.
pub fn parse_sexpr(input: &str) -> Result<SExpr, MirrError> {
    if input.len() > MAX_SEXPR_STRING_LEN {
        return Err(sexpr_err("[E804] Input exceeds MAX_SEXPR_STRING_LEN"));
    }

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(sexpr_err("[E801] Empty S-expression input"));
    }

    let tokens = tokenize(trimmed)?;
    if tokens.is_empty() {
        return Err(sexpr_err("[E801] No tokens in S-expression input"));
    }

    let mut pos = 0;
    let mut node_count = 0usize;
    let result = parse_one(&tokens, &mut pos, 0, &mut node_count)?;

    // Check for trailing tokens.
    if pos < tokens.len() {
        return Err(sexpr_err("[E800] Unexpected tokens after S-expression"));
    }

    Ok(result)
}

/// Parse a single S-expression from the token stream.
///
/// Uses an explicit stack for list parsing (no recursion).
fn parse_one(
    tokens: &[Token],
    pos: &mut usize,
    depth: usize,
    node_count: &mut usize,
) -> Result<SExpr, MirrError> {
    if depth >= MAX_SEXPR_DEPTH {
        return Err(sexpr_err("[E803] S-expression nesting exceeds MAX_SEXPR_DEPTH"));
    }
    if *pos >= tokens.len() {
        return Err(sexpr_err("[E801] Unexpected end of S-expression input"));
    }
    *node_count += 1;
    if *node_count > MAX_SEXPR_NODES {
        return Err(sexpr_err("[E804] S-expression tree exceeds MAX_SEXPR_NODES"));
    }

    match &tokens[*pos] {
        Token::OpenParen => {
            *pos += 1;
            let mut items = Vec::new();
            // Bounded: at most MAX_SEXPR_NODES items.
            let mut list_iters = 0usize;
            while *pos < tokens.len() && tokens[*pos] != Token::CloseParen {
                list_iters += 1;
                if list_iters > MAX_SEXPR_NODES {
                    return Err(sexpr_err("[E804] List exceeds MAX_SEXPR_NODES elements"));
                }
                let item = parse_one(tokens, pos, depth + 1, node_count)?;
                items.push(item);
            }
            if *pos >= tokens.len() {
                return Err(sexpr_err("[E802] Unbalanced parentheses — missing ')'"));
            }
            *pos += 1; // consume ')'
            Ok(SExpr::List(items))
        }
        Token::CloseParen => Err(sexpr_err("[E802] Unexpected ')' — unbalanced parentheses")),
        Token::Quote => {
            *pos += 1;
            let inner = parse_one(tokens, pos, depth + 1, node_count)?;
            Ok(SExpr::Quote(Box::new(inner)))
        }
        Token::Backtick => {
            *pos += 1;
            let inner = parse_one(tokens, pos, depth + 1, node_count)?;
            Ok(SExpr::Quasiquote(Box::new(inner)))
        }
        Token::Comma => {
            *pos += 1;
            let inner = parse_one(tokens, pos, depth + 1, node_count)?;
            Ok(SExpr::Unquote(Box::new(inner)))
        }
        Token::Symbol(s) => {
            let result = SExpr::Symbol(s.clone());
            *pos += 1;
            Ok(result)
        }
        Token::Integer(n) => {
            let result = SExpr::Integer(*n);
            *pos += 1;
            Ok(result)
        }
        Token::Bool(b) => {
            let result = SExpr::Bool(*b);
            *pos += 1;
            Ok(result)
        }
        Token::Str(s) => {
            let result = SExpr::Str(s.clone());
            *pos += 1;
            Ok(result)
        }
    }
}

/// Construct an S-expression error (E8xx range).
pub(crate) fn sexpr_err(msg: impl Into<String>) -> MirrError {
    MirrError::SExprError { message: msg.into(), span: None }
}
