//! Parser module for MIRR source code.
//!
//! Re-exports the module parser, expression parser, and pattern parser.

#![forbid(unsafe_code)]

pub mod expr_parser;
pub mod module_parser;
pub mod pattern_parser;

pub use expr_parser::parse_expression;
pub use module_parser::parse_mirr;
pub use pattern_parser::{is_pattern_call_line, parse_pattern_call, parse_pattern_def};

/// Skip empty lines and comment lines in a line array.
/// Used by module_parser and pattern_parser.
pub(crate) fn skip_empty_and_comments(lines: &[&str], index: &mut usize) {
    while *index < lines.len() {
        let line = lines[*index].trim();
        if line.is_empty() || line.starts_with("//") {
            *index += 1;
        } else {
            break;
        }
    }
}

/// Parse a signal type string into a `SignalType`.
///
/// Recognizes `"bool"`, `"u<N>"` (unsigned), and `"i<N>"` (signed).
/// Returns `None` for unrecognized type strings.
/// Width parsing failures (non-numeric suffix) also return `None`.
///
/// Callers are responsible for producing error messages with appropriate
/// error codes (E116-E118 for module signals, E416-E417 for patterns).
pub(crate) fn parse_signal_type_str(ty_str: &str) -> Option<crate::ast::types::SignalType> {
    use crate::ast::types::SignalType;
    if ty_str == "bool" {
        return Some(SignalType::Bool);
    }
    if let Some(suffix) = ty_str.strip_prefix('u') {
        return suffix.parse::<u32>().ok().map(SignalType::Unsigned);
    }
    if let Some(suffix) = ty_str.strip_prefix('i') {
        return suffix.parse::<u32>().ok().map(SignalType::Signed);
    }
    None
}

// =========================================================================
// MEGA-1 extended signal declaration tokenizer
//
// Replaces the previous `split_whitespace()` + 2-token approach with a
// structured tokenizer that handles the full MEGA-1 grammar:
//
//   <kind> [linear] [stateful|pure] <base_type> [where <refinement>] [@clock] [#phantom]
//
// Backward compatible: plain `<kind> <type>` declarations still parse
// identically — the tokenizer simply produces default (empty) annotations.
//
// Error codes: E177–E179, E182–E183, E190–E197 for extended annotation errors.
// =========================================================================

use crate::ast::types::{EffectQualifier, Linearity, Refinement, SignalKind, TypeAnnotations};
use crate::error::MirrError;

/// Maximum whitespace tokens examined in a signal type declaration.
/// NASA Power-of-10 rule: all loops bounded.
const MAX_SIGNAL_DECL_TOKENS: usize = 64;

/// Parsed result of [`tokenize_signal_decl`].
///
/// Contains the signal kind, base type, and all MEGA-1 annotations
/// extracted from the declaration's right-hand side.
#[derive(Debug, Clone)]
pub(crate) struct TokenizedSignalDecl {
    pub kind: SignalKind,
    pub ty: crate::ast::types::SignalType,
    pub annotations: TypeAnnotations,
}

/// Tokenize and parse the RHS of a signal declaration (after `:`; semicolon already stripped).
///
/// # Grammar
///
/// ```text
/// <kind> [linear] [stateful|pure] <base_type> [where <refinement>] [@<clock>] [#<phantom>]
/// ```
///
/// Where:
/// - `<kind>` is `in`, `out`, or `internal`
/// - `[linear]` is the optional linearity qualifier
/// - `[stateful|pure]` is the optional effect annotation
/// - `<base_type>` is `bool`, `u<N>`, or `i<N>`
/// - `[where <refinement>]` is `where lo..hi` or `where <predicate_expr>`
/// - `[@<clock>]` is `@<identifier>` for clock domain
/// - `[#<phantom>]` is `#<Identifier>` (uppercase) for phantom tag
///
/// # Backward compatibility
///
/// Plain 2-token declarations (`<kind> <type>`) parse identically to the
/// previous `split_whitespace()` approach; the resulting `annotations` will
/// have all default (empty) values.
///
/// # Bounding
///
/// At most [`MAX_SIGNAL_DECL_TOKENS`] whitespace tokens are examined.
pub(crate) fn tokenize_signal_decl(rest: &str) -> Result<TokenizedSignalDecl, MirrError> {
    let tokens: Vec<&str> = rest.split_whitespace().collect();

    if tokens.len() > MAX_SIGNAL_DECL_TOKENS {
        return Err(MirrError::new("[E183] Signal declaration exceeds maximum token count."));
    }
    if tokens.is_empty() {
        return Err(MirrError::new("[E112] Signal kind (in/out/internal) is missing."));
    }

    // 1. Parse kind (required, first token).
    let kind = parse_signal_kind(tokens[0])?;

    // 2. Parse qualifiers + base type + suffixes from remaining tokens.
    let (ty, annotations) = parse_qualified_type(&tokens, 1)?;

    Ok(TokenizedSignalDecl { kind, ty, annotations })
}

/// Parse only the type portion (no kind prefix) with optional MEGA-1 annotations.
///
/// Used for pattern constant parameters where the declaration is `name: <type> [annotations]`
/// without a preceding `signal <kind>`.
///
/// # Grammar
///
/// ```text
/// [linear] [stateful|pure] <base_type> [where <refinement>] [@<clock>] [#<phantom>]
/// ```
///
/// # Backward compatibility
///
/// A bare `<base_type>` token (e.g. `"u16"`) produces default annotations,
/// identical to the previous `parse_signal_type_str()` call.
pub(crate) fn parse_type_with_annotations(
    type_str: &str,
) -> Result<(crate::ast::types::SignalType, TypeAnnotations), MirrError> {
    let tokens: Vec<&str> = type_str.split_whitespace().collect();

    if tokens.len() > MAX_SIGNAL_DECL_TOKENS {
        return Err(MirrError::new("[E183] Type annotation exceeds maximum token count."));
    }

    parse_qualified_type(&tokens, 0)
}

/// Parse a signal kind string into a [`SignalKind`].
fn parse_signal_kind(s: &str) -> Result<SignalKind, MirrError> {
    match s {
        "in" => Ok(SignalKind::Input),
        "out" => Ok(SignalKind::Output),
        "internal" => Ok(SignalKind::Internal),
        other => Err(MirrError::new(format!(
            "[E115] Unknown signal kind: {other}. Expected 'in', 'out', or 'internal'.",
        ))),
    }
}

/// Internal: parse `[qualifiers] <base_type> [suffixes]` from a token slice.
///
/// `tokens[start..]` is examined. Iteration is bounded by the slice length
/// (which is capped at [`MAX_SIGNAL_DECL_TOKENS`] by the callers).
///
/// # Phases
///
/// 1. **Qualifiers** — consume `linear`, `stateful`, `pure` (at most 2).
/// 2. **Base type** — consume one token; parse via [`parse_signal_type_str`].
/// 3. **Suffixes** — consume `where <refinement>`, `@<clock>`, `#<phantom>` in any order.
fn parse_qualified_type(
    tokens: &[&str],
    start: usize,
) -> Result<(crate::ast::types::SignalType, TypeAnnotations), MirrError> {
    let mut pos = start;
    let mut annotations = TypeAnnotations::default();

    // --- Phase 1: Optional qualifiers (linear, stateful, pure) ---
    // Bounded: at most 2 qualifiers (one linearity + one effect).
    const MAX_QUALIFIERS: usize = 2;
    let mut qualifier_count: usize = 0;

    while pos < tokens.len() && qualifier_count < MAX_QUALIFIERS {
        match tokens[pos] {
            "linear" => {
                if annotations.linearity == Linearity::Linear {
                    return Err(MirrError::new(
                        "[E190] Duplicate 'linear' qualifier in signal declaration.",
                    ));
                }
                annotations.linearity = Linearity::Linear;
                pos += 1;
                qualifier_count += 1;
            }
            "stateful" => {
                if annotations.effect != EffectQualifier::Unspecified {
                    return Err(MirrError::new(
                        "[E191] Conflicting effect qualifiers: only one of 'stateful' or 'pure' is allowed.",
                    ));
                }
                annotations.effect = EffectQualifier::Stateful;
                pos += 1;
                qualifier_count += 1;
            }
            "pure" => {
                if annotations.effect != EffectQualifier::Unspecified {
                    return Err(MirrError::new(
                        "[E191] Conflicting effect qualifiers: only one of 'stateful' or 'pure' is allowed.",
                    ));
                }
                annotations.effect = EffectQualifier::Pure;
                pos += 1;
                qualifier_count += 1;
            }
            _ => break, // Not a qualifier — must be the base type.
        }
    }

    // --- Phase 2: Base type (required) ---
    if pos >= tokens.len() {
        return Err(MirrError::new(
            "[E192] Missing base type after qualifiers. Expected 'bool', 'uN', or 'iN'.",
        ));
    }

    let ty_str = tokens[pos];
    let ty = parse_signal_type_str(ty_str).ok_or_else(|| {
        MirrError::new(format!(
            "[E118] Unknown signal type: {ty_str}. Expected 'bool', 'uN', or 'iN'.",
        ))
    })?;
    pos += 1;

    // --- Phase 3: Optional suffix annotations ---
    // Accepted in any order: `where <refinement>`, `@<clock>`, `#<phantom>`.
    // Bounded: each suffix consumed at most once; iteration bounded by token count.
    while pos < tokens.len() {
        let token = tokens[pos];

        if token == "where" {
            // --- Refinement clause ---
            if annotations.refinement.is_some() {
                return Err(MirrError::new(
                    "[E196] Duplicate 'where' clause in signal declaration.",
                ));
            }
            pos += 1;

            // Collect tokens until we hit one starting with '@' or '#', or run out.
            let ref_start = pos;
            while pos < tokens.len()
                && !tokens[pos].starts_with('@')
                && !tokens[pos].starts_with('#')
            {
                pos += 1;
            }

            if pos == ref_start {
                return Err(MirrError::new(
                    "[E193] Empty refinement clause after 'where'. Expected range 'N..M' or predicate.",
                ));
            }

            let ref_str: String = tokens[ref_start..pos].join(" ");
            annotations.refinement = Some(parse_refinement_clause(&ref_str)?);
        } else if let Some(domain) = token.strip_prefix('@') {
            // --- Clock domain ---
            if annotations.clock_domain.is_some() {
                return Err(MirrError::new(
                    "[E197] Duplicate clock domain annotation in signal declaration.",
                ));
            }
            if domain.is_empty() {
                return Err(MirrError::new(
                    "[E195] Empty clock domain: expected identifier after '@'.",
                ));
            }
            if !is_valid_identifier(domain) {
                return Err(MirrError::new(format!(
                    "[E177] Invalid clock domain name '{domain}': must be alphanumeric/underscore identifier.",
                )));
            }
            annotations.clock_domain = Some(domain.to_string());
            pos += 1;
        } else if let Some(tag) = token.strip_prefix('#') {
            // --- Phantom tag ---
            if annotations.phantom_tag.is_some() {
                return Err(MirrError::new(
                    "[E182] Duplicate phantom tag annotation in signal declaration.",
                ));
            }
            if tag.is_empty() {
                return Err(MirrError::new(
                    "[E178] Empty phantom tag: expected identifier after '#'.",
                ));
            }
            if !tag.starts_with(|c: char| c.is_ascii_uppercase()) {
                return Err(MirrError::new(format!(
                    "[E179] Invalid phantom tag '{tag}': must start with uppercase letter.",
                )));
            }
            if !is_valid_identifier(tag) {
                return Err(MirrError::new(format!(
                    "[E179] Invalid phantom tag '{tag}': must be alphanumeric/underscore identifier starting with uppercase.",
                )));
            }
            annotations.phantom_tag = Some(tag.to_string());
            pos += 1;
        } else {
            return Err(MirrError::new(format!(
                "[E183] Unexpected token '{token}' after signal type. Expected 'where', '@clock', or '#Tag'.",
            )));
        }
    }

    Ok((ty, annotations))
}

/// Parse a refinement clause string into a [`Refinement`].
///
/// Accepts two forms:
/// - **Range**: `"0..1023"` or `"0..=1023"` — both are inclusive on both ends
///   (hardware convention).
/// - **Predicate**: `"value < 1024"` — stored as raw expression string for
///   later semantic analysis.
fn parse_refinement_clause(ref_str: &str) -> Result<Refinement, MirrError> {
    let trimmed = ref_str.trim();

    // Try `lo..=hi` first (explicit inclusive range notation).
    if let Some((lo_str, hi_str)) = trimmed.split_once("..=") {
        let lo = lo_str.trim().parse::<u64>().map_err(|_| {
            MirrError::new(format!(
                "[E193] Malformed range refinement: '{lo_str}' is not a valid integer.",
            ))
        })?;
        let hi = hi_str.trim().parse::<u64>().map_err(|_| {
            MirrError::new(format!(
                "[E193] Malformed range refinement: '{hi_str}' is not a valid integer.",
            ))
        })?;
        if lo > hi {
            return Err(MirrError::new(format!(
                "[E194] Invalid range in refinement: lo ({lo}) must be <= hi ({hi}).",
            )));
        }
        return Ok(Refinement::Range { lo, hi });
    }

    // Try `lo..hi` (hardware-convention inclusive range).
    if let Some((lo_str, hi_str)) = trimmed.split_once("..") {
        let lo = lo_str.trim().parse::<u64>().map_err(|_| {
            MirrError::new(format!(
                "[E193] Malformed range refinement: '{lo_str}' is not a valid integer.",
            ))
        })?;
        let hi = hi_str.trim().parse::<u64>().map_err(|_| {
            MirrError::new(format!(
                "[E193] Malformed range refinement: '{hi_str}' is not a valid integer.",
            ))
        })?;
        if lo > hi {
            return Err(MirrError::new(format!(
                "[E194] Invalid range in refinement: lo ({lo}) must be <= hi ({hi}).",
            )));
        }
        return Ok(Refinement::Range { lo, hi });
    }

    // Predicate form: store the raw expression string.
    if trimmed.is_empty() {
        return Err(MirrError::new(
            "[E193] Empty refinement clause. Expected range 'N..M' or predicate expression.",
        ));
    }

    Ok(Refinement::Predicate(trimmed.to_string()))
}

/// Check if a string is a valid MIRR identifier (ASCII alphanumeric + underscore,
/// not starting with a digit).
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
