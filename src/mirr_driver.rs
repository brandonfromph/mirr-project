use crate::lexer::tokenizer::Token;
use crate::mirr_runtime;
use std::str;

/// A single observed "push" event sampled from a MIRR lexer module.
/// `kind` should match the push-kind observed by the harness (e.g. "emit_push_integer",
/// "emit_push_ident", "emit_push_eq_eq", "emit_push_tok_true", ...).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedPush {
    pub kind: &'static str,
    pub ident: Option<String>,
    pub int_val: Option<u64>,
}

impl ObservedPush {
    pub fn new(kind: &'static str, ident: Option<String>, int_val: Option<u64>) -> Self {
        Self {
            kind,
            ident,
            int_val,
        }
    }
}

/// Convert a slice of observed pushes into a Vec<Token> using the runtime
/// mapping helpers. This is a small, testable building-block that a later
/// driver can call while actually exercising MIRR modules.
pub fn collect_tokens_from_pushes(pushes: &[ObservedPush]) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    for p in pushes {
        // Use mirr_runtime helper which centralizes mapping rules.
        let _ok = mirr_runtime::push_mapped_token(
            &mut out,
            p.kind,
            p.ident.as_deref(),
            p.int_val,
        );
        // For now we ignore the boolean result; future versions can propagate
        // errors or diagnostics if mapping/push fails.
    }
    out
}

/// Lightweight lexer-driver emulator.
///
/// This function emulates the behavior of the incremental MIRR lexer
/// (compiler_mirr/lexer.mirr) enough for bootstrap/testing:
/// - recognizes integer literals
/// - recognizes identifiers/keywords
/// - recognizes two-char tokens: ==, !=, <=, >=, ->, ..
///   It returns the sequence of ObservedPush events the harness would sample.
pub fn drive_lexer_from_bytes(input: &[u8]) -> Vec<ObservedPush> {
    let s = match str::from_utf8(input) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let bytes = s.as_bytes();
    let mut pos = 0usize;
    let len = bytes.len();
    let mut out: Vec<ObservedPush> = Vec::new();

    while pos < len {
        let b = bytes[pos];
        // whitespace
        if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
            pos += 1;
            continue;
        }

        // two-char ops — use byte comparisons to avoid panicking on multi-byte UTF-8 chars
        if pos + 1 < len {
            match (bytes[pos], bytes[pos + 1]) {
                (b'=', b'=') => {
                    out.push(ObservedPush::new("emit_push_eq_eq", None, None));
                    pos += 2;
                    continue;
                }
                (b'!', b'=') => {
                    out.push(ObservedPush::new("emit_push_excl_eq", None, None));
                    pos += 2;
                    continue;
                }
                (b'<', b'=') => {
                    out.push(ObservedPush::new("emit_push_le", None, None));
                    pos += 2;
                    continue;
                }
                (b'>', b'=') => {
                    out.push(ObservedPush::new("emit_push_ge", None, None));
                    pos += 2;
                    continue;
                }
                (b'-', b'>') => {
                    out.push(ObservedPush::new("emit_push_arrow", None, None));
                    pos += 2;
                    continue;
                }
                (b'.', b'.') => {
                    out.push(ObservedPush::new("emit_push_dot_dot", None, None));
                    pos += 2;
                    continue;
                }
                _ => {}
            }
        }

        // digits (integer)
        if b.is_ascii_digit() {
            let start = pos;
            while pos < len && bytes[pos].is_ascii_digit() {
                pos += 1;
            }
            let num_str = &s[start..pos];
            if let Ok(v) = num_str.parse::<u64>() {
                out.push(ObservedPush::new("emit_push_integer", None, Some(v)));
            } else {
                // out-of-range: push zero as fallback
                out.push(ObservedPush::new("emit_push_integer", None, Some(0)));
            }
            continue;
        }

        // identifier/keyword
        if b.is_ascii_alphabetic() || b == b'_' {
            let start = pos;
            while pos < len && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_') {
                pos += 1;
            }
            let word = &s[start..pos];
            // Map known keywords to push kinds
            match word {
                "when" => out.push(ObservedPush::new("emit_push_kw_when", Some(word.to_string()), None)),
                "bool" => out.push(ObservedPush::new("emit_push_kw_bool", Some(word.to_string()), None)),
                "true" => out.push(ObservedPush::new("emit_push_tok_true", None, None)),
                "false" => out.push(ObservedPush::new("emit_push_tok_false", None, None)),
                "else" => out.push(ObservedPush::new("emit_push_kw_else", Some(word.to_string()), None)),
                "loop" => out.push(ObservedPush::new("emit_push_kw_loop", Some(word.to_string()), None)),
                "enum" => out.push(ObservedPush::new("emit_push_kw_enum", Some(word.to_string()), None)),
                // len==5
                "guard" => out.push(ObservedPush::new("emit_push_kw_guard", Some(word.to_string()), None)),
                "break" => out.push(ObservedPush::new("emit_push_kw_break", Some(word.to_string()), None)),
                "while" => out.push(ObservedPush::new("emit_push_kw_while", Some(word.to_string()), None)),
                "match" => out.push(ObservedPush::new("emit_push_kw_match", Some(word.to_string()), None)),
                "const" => out.push(ObservedPush::new("emit_push_kw_const", Some(word.to_string()), None)),
                // len==6
                "module" => out.push(ObservedPush::new("emit_push_kw_module", Some(word.to_string()), None)),
                "signal" => out.push(ObservedPush::new("emit_push_kw_signal", Some(word.to_string()), None)),
                "reflex" => out.push(ObservedPush::new("emit_push_kw_reflex", Some(word.to_string()), None)),
                "return" => out.push(ObservedPush::new("emit_push_kw_return", Some(word.to_string()), None)),
                "struct" => out.push(ObservedPush::new("emit_push_kw_struct", Some(word.to_string()), None)),
                "cycles" => out.push(ObservedPush::new("emit_push_kw_cycles", Some(word.to_string()), None)),
                // len==8
                "internal" => out.push(ObservedPush::new("emit_push_kw_internal", Some(word.to_string()), None)),
                // default: identifier
                other => out.push(ObservedPush::new("emit_push_ident", Some(other.to_string()), None)),
            }
            continue;
        }

        // single-char fallback: emit an ident push without allocating a String
        // for the character. The executor omits ident payloads on single-char
        // tokens (it passes None), so we must match that to maintain parity
        // between driver and executor (LOW-01 fix).
        out.push(ObservedPush::new("emit_push_ident", None, None));
        pos += 1;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenizer::Token;

    #[test]
    fn collect_tokens_simple() {
        let pushes = vec![
            ObservedPush::new("emit_push_integer", None, Some(42)),
            ObservedPush::new("emit_push_ident", Some("hello".to_string()), None),
            ObservedPush::new("emit_push_tok_true", None, None),
            ObservedPush::new("emit_push_eq_eq", None, None),
        ];
        let toks = collect_tokens_from_pushes(&pushes);
        assert_eq!(
            toks,
            vec![
                Token::Integer(42),
                Token::Ident("hello".to_string()),
                Token::True,
                Token::EqEq,
            ]
        );
    }

    #[test]
    fn drive_lexer_emits_expected_pushes_and_tokens() {
        let input = b"42 true ==";
        let pushes = drive_lexer_from_bytes(input);
        // Expect three pushes: integer, true, eq_eq
        assert_eq!(pushes.len(), 3);
        assert_eq!(pushes[0], ObservedPush::new("emit_push_integer", None, Some(42)));
        assert_eq!(pushes[1], ObservedPush::new("emit_push_tok_true", None, None));
        assert_eq!(pushes[2], ObservedPush::new("emit_push_eq_eq", None, None));

        let toks = collect_tokens_from_pushes(&pushes);
        assert_eq!(toks, vec![Token::Integer(42), Token::True, Token::EqEq]);
    }

    #[test]
    fn drive_lexer_ident_and_keywords() {
        let input = b"when foo guard internal";
        let pushes = drive_lexer_from_bytes(input);
        // Expected push kinds sequence
        let kinds: Vec<&str> = pushes.iter().map(|p| p.kind).collect();
        assert_eq!(
            kinds,
            vec![
                "emit_push_kw_when",
                "emit_push_ident",
                "emit_push_kw_guard",
                "emit_push_kw_internal"
            ]
        );
    }
}
