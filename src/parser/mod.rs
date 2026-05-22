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
/// Recognizes `"bool"`, `"u<N>"` (unsigned), `"i<N>"` (signed), and `"fifo<T,N>"`.
/// Returns `None` for unrecognized type strings.
/// Width parsing failures (non-numeric suffix) also return `None`.
///
/// Callers are responsible for producing error messages with appropriate
/// error codes (E116-E118 for module signals, E416-E417 for patterns).
pub(crate) fn parse_signal_type_str(ty_str: &str) -> Option<crate::ast::types::SignalType> {
    use crate::ast::types::SignalType;
    // Parse array type as: <base_type>[<len>]
    if let Some(open_bracket_pos) = ty_str.find('[') {
        if ty_str.ends_with(']') {
            let element_type = &ty_str[..open_bracket_pos];
            let len_str = &ty_str[open_bracket_pos + 1..ty_str.len() - 1];
            let element = Box::new(parse_signal_type_str(element_type)?);
            let length = len_str.parse::<u64>().ok()?;
            if (1..=MAX_TYPE_NAT).contains(&length) {
                return Some(SignalType::Array { element, length });
            } else {
                return None;
            }
        }
    }

    if ty_str == "bool" {
        return Some(SignalType::Bool);
    }
    if ty_str == "u16" {
        return Some(SignalType::Unsigned(16));
    }

    if let Some(name) = ty_str.strip_prefix("struct ") {
        let name = name.trim();
        if !name.is_empty() {
            return Some(SignalType::Struct { name: name.to_string(), fields: Vec::new() });
        }
    }

    if let Some(name) = ty_str.strip_prefix("interface ") {
        let name = name.trim();
        if !name.is_empty() {
            return Some(SignalType::Bundle(name.to_string()));
        }
    }

    // Support standard uN/iN syntax
    if ty_str.starts_with('u') && ty_str[1..].chars().all(|c| c.is_ascii_digit()) {
        if let Ok(width) = ty_str[1..].parse::<u32>() {
            if width == 0 || width > 1024 {
                // Return None here, letting the caller (parse_qualified_type)
                // handle the error reporting using the E116/E117 codes.
                // Actually, parse_signal_type_str returns Option, so let's
                // just return None for invalid widths.
                return None;
            }
            return Some(SignalType::Unsigned(width));
        }
    }
    if ty_str.starts_with('i') && ty_str[1..].chars().all(|c| c.is_ascii_digit()) {
        if let Ok(width) = ty_str[1..].parse::<u32>() {
            if width == 0 || width > 1024 {
                return None;
            }
            return Some(SignalType::Signed(width));
        }
    }

    // MEGA-1 generic numeric syntax: unsigned<32>, signed<32>, fixed<total,frac>.
    if let Some(inner) = ty_str.strip_prefix("unsigned<") {
        if let Some(rest) = inner.strip_suffix('>') {
            if let Ok(width) = rest.parse::<u32>() {
                return Some(SignalType::Unsigned(width));
            }
        }
    }
    if let Some(inner) = ty_str.strip_prefix("signed<") {
        if let Some(rest) = inner.strip_suffix('>') {
            if let Ok(width) = rest.parse::<u32>() {
                return Some(SignalType::Signed(width));
            }
        }
    }

    // Backward-compatible numeric syntax: u32 / i32.
    if let Some(suffix) = ty_str.strip_prefix('u') {
        return suffix.parse::<u32>().ok().map(SignalType::Unsigned);
    }
    if let Some(suffix) = ty_str.strip_prefix('i') {
        return suffix.parse::<u32>().ok().map(SignalType::Signed);
    }
    if let Some(inner) = ty_str.strip_prefix("fixed<") {
        if let Some(rest) = inner.strip_suffix('>') {
            let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                if let (Ok(total_bits), Ok(frac_bits)) =
                    (parts[0].parse::<u32>(), parts[1].parse::<u32>())
                {
                    if total_bits == 0
                        || total_bits > crate::ast::types::MAX_FIXED_POINT_BITS
                        || frac_bits > total_bits
                    {
                        return None;
                    }
                    return Some(SignalType::FixedPoint { total_bits, frac_bits });
                }
            }
        }
    }

    // Parse fifo<T,N> where T is element type and N is depth.
    if let Some(inner) = ty_str.strip_prefix("fifo<") {
        if let Some(rest) = inner.strip_suffix('>') {
            let parts: Vec<&str> = rest.splitn(2, ',').collect();
            if parts.len() == 2 {
                let elem_str = parts[0].trim();
                let depth_str = parts[1].trim();
                let element = Box::new(parse_signal_type_str(elem_str)?);
                let depth = depth_str.parse::<u64>().ok()?;
                // Depth must be 1..=256 (MAX_FIFO_DEPTH).
                if (1..=256).contains(&depth) {
                    return Some(SignalType::Fifo { element, depth });
                }
            }
        }
        return None;
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
use crate::typeck::extended::MAX_TYPE_NAT;

/// Maximum whitespace tokens examined in a signal type declaration.
/// NASA Power-of-10 rule: all loops bounded.
const MAX_SIGNAL_DECL_TOKENS: usize = 16;

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

/// Rejoin tokens that were split inside generic type parameters (e.g. `fifo<u8, 4>`).
///
/// Without this, `split_whitespace()` turns `"fifo<u8, 4>"` into `["fifo<u8,", "4>"]`
/// and the parser fails with E118.
fn rejoin_generic_tokens(raw: Vec<&str>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0_usize;
    // NASA W2: bounded by input length.
    while i < raw.len() {
        let tok = raw[i];
        if tok.contains('<') && !tok.ends_with('>') {
            // Start of a generic — accumulate until we see '>'.
            let mut buf = String::from(tok);
            i += 1;
            while i < raw.len() {
                buf.push(' ');
                buf.push_str(raw[i]);
                if raw[i].ends_with('>') {
                    i += 1;
                    break;
                }
                i += 1;
            }
            out.push(buf);
        } else {
            out.push(tok.to_string());
            i += 1;
        }
    }
    out
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
/// - `<base_type>` is `bool`, `u<N>`, `i<N>`, or `fifo<T,N>`
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
    let raw: Vec<&str> = rest.split_whitespace().collect();
    let tokens = rejoin_generic_tokens(raw);

    if tokens.len() > MAX_SIGNAL_DECL_TOKENS {
        return Err(MirrError::parse_error(format!(
            "{} Signal declaration exceeds maximum token count.",
            crate::error_codes::ec(114)
        )));
    }
    if tokens.is_empty() {
        return Err(MirrError::parse_error(format!(
            "{} Signal type is missing after ':'.",
            crate::error_codes::ec(113)
        )));
    }

    // 1. Determine kind. If the first token is not a known kind, default to Internal
    // and treat the first token as part of the type declaration.
    let token0 = tokens[0].as_str();
    let is_type_token =
        matches!(token0, "linear" | "stateful" | "pure" | "struct" | "interface" | "bool")
            || (token0.starts_with('u')
                && token0[1..].chars().next().map_or(false, |c| c.is_ascii_digit()))
            || (token0.starts_with('i')
                && token0[1..].chars().next().map_or(false, |c| c.is_ascii_digit()))
            || token0.starts_with("unsigned")
            || token0.starts_with("signed")
            || token0.starts_with("fixed")
            || token0.contains('[');

    let (kind, start_idx) = match token0 {
        "in" => (crate::ast::types::SignalKind::Input, 1),
        "out" => (crate::ast::types::SignalKind::Output, 1),
        "internal" => (crate::ast::types::SignalKind::Internal, 1),
        _ => {
            if is_type_token {
                (crate::ast::types::SignalKind::Internal, 0)
            } else {
                // If it doesn't match a known kind AND doesn't look like a type,
                // it's an unknown type if it's the start of the type block.
                return Err(MirrError::parse_error(format!(
                    "{} Unknown signal kind or type: '{}'. Expected 'in', 'out', 'internal', 'bool', 'uN', or 'iN'.",
                    crate::error_codes::ec(118),
                    token0
                )));
            }
        }
    };

    // 2. Parse qualifiers + base type + suffixes from remaining tokens.
    let (ty, annotations) = parse_qualified_type(&tokens, start_idx)?;

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
    let raw: Vec<&str> = type_str.split_whitespace().collect();
    let tokens = rejoin_generic_tokens(raw);

    if tokens.len() > MAX_SIGNAL_DECL_TOKENS {
        return Err(MirrError::parse_error(format!(
            "{} Type annotation exceeds maximum token count.",
            crate::error_codes::ec(183)
        )));
    }

    parse_qualified_type(&tokens, 0)
}

/// Parse a signal kind string into a [`SignalKind`].
// 310: /// Internal: parse `[qualifiers] <base_type> [suffixes]` from a token slice.
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
    tokens: &[String],
    start: usize,
) -> Result<(crate::ast::types::SignalType, TypeAnnotations), MirrError> {
    let mut pos = start;
    let mut annotations = TypeAnnotations::default();

    // --- Phase 1: Optional qualifiers (linear, stateful, pure) ---
    // Bounded: at most 2 qualifiers (one linearity + one effect).
    const MAX_QUALIFIERS: usize = 2;
    let mut qualifier_count: usize = 0;

    while pos < tokens.len() && qualifier_count < MAX_QUALIFIERS {
        match tokens[pos].as_str() {
            "linear" => {
                if annotations.linearity == Linearity::Linear {
                    return Err(MirrError::parse_error(format!(
                        "{} Duplicate 'linear' qualifier in signal declaration.",
                        crate::error_codes::ec(190)
                    )));
                }
                annotations.linearity = Linearity::Linear;
                pos += 1;
                qualifier_count += 1;
            }
            "stateful" => {
                if annotations.effect != EffectQualifier::Unspecified {
                    return Err(MirrError::parse_error(format!("{} Conflicting effect qualifiers: only one of 'stateful' or 'pure' is allowed.", crate::error_codes::ec(191))));
                }
                annotations.effect = EffectQualifier::Stateful;
                pos += 1;
                qualifier_count += 1;
            }
            "pure" => {
                if annotations.effect != EffectQualifier::Unspecified {
                    return Err(MirrError::parse_error(format!("{} Conflicting effect qualifiers: only one of 'stateful' or 'pure' is allowed.", crate::error_codes::ec(191))));
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
        return Err(MirrError::parse_error(format!(
            "{} Missing base type after qualifiers. Expected 'bool', 'uN', or 'iN'.",
            crate::error_codes::ec(192)
        )));
    }

    let ty = if tokens[pos] == "struct" {
        pos += 1;
        if pos >= tokens.len() {
            return Err(MirrError::parse_error(format!(
                "{} Missing struct name after 'struct'.",
                crate::error_codes::ec(118)
            )));
        }
        let struct_name = tokens[pos].trim();
        if struct_name.is_empty() {
            return Err(MirrError::parse_error(format!(
                "{} Struct name cannot be empty.",
                crate::error_codes::ec(118)
            )));
        }
        pos += 1;
        crate::ast::types::SignalType::Struct { name: struct_name.to_string(), fields: Vec::new() }
    } else if tokens[pos] == "interface" {
        pos += 1;
        if pos >= tokens.len() {
            return Err(MirrError::parse_error(format!(
                "{} Missing interface name after 'interface'.",
                crate::error_codes::ec(118)
            )));
        }
        let interface_name = tokens[pos].trim();
        if interface_name.is_empty() {
            return Err(MirrError::parse_error(format!(
                "{} Interface name cannot be empty.",
                crate::error_codes::ec(118)
            )));
        }
        pos += 1;
        crate::ast::types::SignalType::Bundle(interface_name.to_string())
    } else {
        let ty_str = &tokens[pos];
        let ty = parse_signal_type_str(ty_str).ok_or_else(|| {
            if ty_str.starts_with('u') {
                return MirrError::parse_error(format!(
                    "{} Invalid unsigned width: {}. Must be 1-1024.",
                    crate::error_codes::ec(116),
                    ty_str
                ));
            }
            if ty_str.starts_with('i') {
                return MirrError::parse_error(format!(
                    "{} Invalid signed width: {}. Must be 1-1024.",
                    crate::error_codes::ec(117),
                    ty_str
                ));
            }
            MirrError::parse_error(format!(
                "{} Unknown signal type: {ty_str}. Expected 'bool', 'uN', or 'iN'.",
                crate::error_codes::ec(118),
            ))
        })?;
        pos += 1;
        ty
    };

    // --- Phase 3: Optional suffix annotations ---
    // Accepted in any order: `where <refinement>`, `@<clock>`, `#<phantom>`.
    // Bounded: each suffix consumed at most once; iteration bounded by token count.
    while pos < tokens.len() {
        let token = &tokens[pos];

        if token == "where" {
            // --- Refinement clause ---
            if annotations.refinement.is_some() {
                return Err(MirrError::parse_error(format!(
                    "{} Duplicate 'where' clause in signal declaration.",
                    crate::error_codes::ec(196)
                )));
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
                return Err(MirrError::parse_error(format!(
                    "{} Empty refinement clause after 'where'. Expected range 'N..M' or predicate.",
                    crate::error_codes::ec(193)
                )));
            }

            let ref_str: String = tokens[ref_start..pos].join(" ");
            annotations.refinement = Some(parse_refinement_clause(&ref_str)?);
        } else if let Some(domain) = token.strip_prefix('@') {
            // --- Clock domain ---
            if annotations.clock_domain.is_some() {
                return Err(MirrError::parse_error(format!(
                    "{} Duplicate clock domain annotation in signal declaration.",
                    crate::error_codes::ec(197)
                )));
            }
            if domain.is_empty() {
                return Err(MirrError::parse_error(format!(
                    "{} Empty clock domain: expected identifier after '@'.",
                    crate::error_codes::ec(195)
                )));
            }
            if !is_valid_identifier(domain) {
                return Err(MirrError::parse_error(format!("{} Invalid clock domain name '{domain}': must be alphanumeric/underscore identifier.", crate::error_codes::ec(177),
                )));
            }
            annotations.clock_domain = Some(domain.to_string());
            pos += 1;
        } else if let Some(tag) = token.strip_prefix('#') {
            // --- Phantom tag ---
            if annotations.phantom_tag.is_some() {
                return Err(MirrError::parse_error(format!(
                    "{} Duplicate phantom tag annotation in signal declaration.",
                    crate::error_codes::ec(182)
                )));
            }
            if tag.is_empty() {
                return Err(MirrError::parse_error(format!(
                    "{} Empty phantom tag: expected identifier after '#'.",
                    crate::error_codes::ec(178)
                )));
            }
            if !tag.starts_with(|c: char| c.is_ascii_uppercase()) {
                return Err(MirrError::parse_error(format!(
                    "{} Invalid phantom tag '{tag}': must start with uppercase letter.",
                    crate::error_codes::ec(179),
                )));
            }
            if !is_valid_identifier(tag) {
                return Err(MirrError::parse_error(format!("{} Invalid phantom tag '{tag}': must be alphanumeric/underscore identifier starting with uppercase.", crate::error_codes::ec(179),
                )));
            }
            annotations.phantom_tag = Some(tag.to_string());
            pos += 1;
        } else {
            return Err(MirrError::parse_error(format!("{} Unexpected token '{token}' after signal type. Expected 'where', '@clock', or '#Tag'.", crate::error_codes::ec(183),
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
            MirrError::parse_error(format!(
                "{} Malformed range refinement: '{lo_str}' is not a valid integer.",
                crate::error_codes::ec(193),
            ))
        })?;
        let hi = hi_str.trim().parse::<u64>().map_err(|_| {
            MirrError::parse_error(format!(
                "{} Malformed range refinement: '{hi_str}' is not a valid integer.",
                crate::error_codes::ec(193),
            ))
        })?;
        if lo > hi {
            return Err(MirrError::parse_error(format!(
                "{} Invalid range in refinement: lo ({lo}) must be <= hi ({hi}).",
                crate::error_codes::ec(194),
            )));
        }
        return Ok(Refinement::Range { lo, hi });
    }

    // Try `lo..hi` (hardware-convention inclusive range).
    if let Some((lo_str, hi_str)) = trimmed.split_once("..") {
        let lo = lo_str.trim().parse::<u64>().map_err(|_| {
            MirrError::parse_error(format!(
                "{} Malformed range refinement: '{lo_str}' is not a valid integer.",
                crate::error_codes::ec(193),
            ))
        })?;
        let hi = hi_str.trim().parse::<u64>().map_err(|_| {
            MirrError::parse_error(format!(
                "{} Malformed range refinement: '{hi_str}' is not a valid integer.",
                crate::error_codes::ec(193),
            ))
        })?;
        if lo > hi {
            return Err(MirrError::parse_error(format!(
                "{} Invalid range in refinement: lo ({lo}) must be <= hi ({hi}).",
                crate::error_codes::ec(194),
            )));
        }
        return Ok(Refinement::Range { lo, hi });
    }

    // Predicate form: store the raw expression string.
    if trimmed.is_empty() {
        return Err(MirrError::parse_error(format!(
            "{} Empty refinement clause. Expected range 'N..M' or predicate expression.",
            crate::error_codes::ec(193)
        )));
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
