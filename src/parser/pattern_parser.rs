//! Parser for MIRR pattern definitions and pattern calls.
//!
//! Handles `def name(params) { reflect { ... } }` blocks and `name(args);` call lines.
//! Bounded: MAX_PARAMS=32, MAX_ARGS=32, MAX_REFLECT_LINES=512, MAX_BRACE_DEPTH=16.

#![forbid(unsafe_code)]

use super::skip_empty_and_comments;
use crate::ast::pattern::{
    PatternArg, PatternCall, PatternDef, PatternParam, PatternParamKind, ReflectBlock,
};
#[allow(unused_imports)] // MEGA-1: SignalKind used by type annotation infrastructure
use crate::ast::types::SignalKind;
use crate::error::MirrError;

/// Maximum number of parameters in a pattern definition.
pub(crate) const MAX_PARAMS: usize = 32;

/// Maximum number of arguments in a pattern call.
const MAX_ARGS: usize = 32;

/// Maximum number of lines in a reflect body.
pub(crate) const MAX_REFLECT_LINES: usize = 512;

/// Maximum brace nesting depth inside a reflect body.
const MAX_BRACE_DEPTH: usize = 16;

/// MIRR keywords that cannot be pattern call names.
const KEYWORDS: &[&str] = &[
    "signal", "guard", "reflex", "property", "module", "def", "reflect", "when", "for", "on",
    "always", "never", "true", "false", "in", "out", "internal", "cycles", "bool", "and",
    // MEGA-1 type annotation keywords:
    "linear", "stateful", "pure", "where",
];

// ---------------------------------------------------------------------------
// Pattern definition parser
// ---------------------------------------------------------------------------

/// Parse a `def` block starting at `lines[*index]`.
///
/// Expected format:
/// ```text
/// def name(
///     param1: signal in u16,
///     param2: u16,
/// ) {
///     reflect {
///         ...
///     }
/// }
/// ```
///
/// Bounded: body lines <= MAX_REFLECT_LINES, brace depth <= MAX_BRACE_DEPTH.
pub fn parse_pattern_def(lines: &[&str], index: &mut usize) -> Result<PatternDef, MirrError> {
    if *index >= lines.len() {
        return Err(pattern_err("[E401] Unexpected end of file in pattern definition."));
    }

    // Collect the full header (may span multiple lines until we see `{` after `)`)
    let header = collect_def_header(lines, index)?;

    // Extract name and param string from header.
    let after_def = header
        .strip_prefix("def ")
        .ok_or_else(|| pattern_err("[E402] Malformed pattern definition."))?;

    let open_paren =
        after_def.find('(').ok_or_else(|| pattern_err("[E403] Pattern definition missing '('."))?;

    let name = after_def[..open_paren].trim();
    if name.is_empty() {
        return Err(pattern_err("[E404] Pattern name cannot be empty."));
    }

    // Find the matching close paren.
    let close_paren = after_def
        .rfind(')')
        .ok_or_else(|| pattern_err(format!("[E405] Pattern '{name}' missing closing ')'")))?;

    let param_str = &after_def[open_paren + 1..close_paren];
    let params = parse_pattern_params(param_str, name)?;

    // Now we should be inside the def body. Look for `reflect {`.
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(pattern_err(format!("[E406] Pattern '{name}' missing 'reflect' block.")));
    }

    let reflect_line = lines[*index].trim();
    if !reflect_line.starts_with("reflect") {
        return Err(pattern_err(format!(
            "[E407] Pattern '{name}' expected 'reflect' block, found: {reflect_line}"
        )));
    }

    // Check for opening brace on the reflect line or next line.
    if !reflect_line.contains('{') {
        *index += 1;
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() || !lines[*index].trim().starts_with('{') {
            return Err(pattern_err(format!(
                "[E408] Pattern '{name}' reflect block missing opening '{{'."
            )));
        }
    }
    *index += 1;

    // Collect raw lines until matching closing brace of reflect.
    let raw_lines = collect_reflect_body(lines, index, name)?;

    // Skip past the closing brace of the reflect block.
    // Now skip to the closing brace of the def block.
    skip_empty_and_comments(lines, index);
    if *index < lines.len() && lines[*index].trim() == "}" {
        *index += 1;
    }

    Ok(PatternDef { name: name.to_string(), params, body: ReflectBlock { raw_lines }, span: None })
}

/// Collect the full `def` header, which may span multiple lines.
///
/// Joins lines until we see `) {` or `){`. Returns the joined header string.
/// Bounded: at most 64 lines for a header.
fn collect_def_header(lines: &[&str], index: &mut usize) -> Result<String, MirrError> {
    let mut header = String::new();
    let max_header_lines = 64usize;
    let mut count = 0usize;

    while *index < lines.len() && count < max_header_lines {
        let line = lines[*index].trim();
        *index += 1;
        count += 1;

        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if !header.is_empty() {
            header.push(' ');
        }
        header.push_str(line);

        // Check if we've seen `) {` marking end of header.
        if header.contains(") {") || header.contains("){") {
            return Ok(header);
        }
    }

    // If header ends with just `)` and next non-empty line is `{`, that's also valid.
    if header.contains(')') {
        skip_empty_and_comments(lines, index);
        if *index < lines.len() && lines[*index].trim().starts_with('{') {
            *index += 1;
            return Ok(header);
        }
    }

    Err(pattern_err("[E409] Pattern definition header not closed with ') {'."))
}

/// Parse the comma-separated parameter list.
///
/// Each parameter is either:
/// - `name: signal in/out TYPE` — a signal parameter
/// - `name: TYPE` — a constant parameter
///
/// Bounded: MAX_PARAMS parameters.
fn parse_pattern_params(param_str: &str, name: &str) -> Result<Vec<PatternParam>, MirrError> {
    let trimmed = param_str.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let parts: Vec<&str> = trimmed.split(',').collect();
    if parts.len() > MAX_PARAMS {
        return Err(pattern_err(format!(
            "[E410] Pattern '{name}' has too many parameters (max {MAX_PARAMS})."
        )));
    }

    let mut params = Vec::with_capacity(parts.len());
    for part in &parts {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        let param = parse_single_param(p, name)?;
        params.push(param);
    }

    Ok(params)
}

/// Parse a single parameter declaration like `sensor: signal in u16` or `low: u16`.
///
/// With MEGA-1 extensions, also handles:
/// - `sensor: signal in linear u16 where 0..1023`
/// - `low: u16 where 0..1023`
fn parse_single_param(param_str: &str, def_name: &str) -> Result<PatternParam, MirrError> {
    let (name_part, type_part) = param_str.split_once(':').ok_or_else(|| {
        pattern_err(format!("[E411] Pattern '{def_name}' parameter missing ':': {param_str}"))
    })?;

    let pname = name_part.trim();
    if pname.is_empty() {
        return Err(pattern_err(format!(
            "[E412] Pattern '{def_name}' has parameter with empty name."
        )));
    }

    let type_str = type_part.trim();

    // Check if it's a signal parameter (starts with "signal").
    if let Some(after_signal) = type_str.strip_prefix("signal") {
        let rest = after_signal.trim();

        // Delegate to the shared MEGA-1 tokenizer which handles:
        //   <kind> [linear] [stateful|pure] <base_type> [where <refinement>] [@clock] [#phantom]
        let parsed = crate::parser::tokenize_signal_decl(rest).map_err(|e| {
            pattern_err(format!(
                "[E413] Pattern '{def_name}' signal parameter '{pname}': {}",
                e.message()
            ))
        })?;

        Ok(PatternParam {
            name: pname.to_string(),
            kind: PatternParamKind::Signal {
                kind: parsed.kind,
                ty: parsed.ty,
                annotations: parsed.annotations,
            },
        })
    } else if type_str == "pattern" {
        // Higher-order: pattern parameter.
        Ok(PatternParam { name: pname.to_string(), kind: PatternParamKind::Pattern })
    } else {
        // Constant parameter — delegate to shared type parser.
        // Handles: [qualifiers] <base_type> [where <refinement>] [@clock] [#phantom]
        let (ty, annotations) =
            crate::parser::parse_type_with_annotations(type_str).map_err(|e| {
                pattern_err(format!(
                    "[E417] Pattern '{def_name}' parameter '{pname}': {}",
                    e.message()
                ))
            })?;

        Ok(PatternParam {
            name: pname.to_string(),
            kind: PatternParamKind::Constant { ty, annotations },
        })
    }
}

/// Collect the raw lines of a reflect body (between opening and closing braces).
///
/// Tracks brace depth to handle nested blocks (guards, reflexes, properties).
/// Bounded: MAX_REFLECT_LINES lines, MAX_BRACE_DEPTH depth.
fn collect_reflect_body(
    lines: &[&str],
    index: &mut usize,
    name: &str,
) -> Result<Vec<String>, MirrError> {
    let mut raw_lines = Vec::with_capacity(64);
    let mut depth: usize = 1; // We're already inside the reflect `{`.
    let mut line_count = 0usize;

    while *index < lines.len() && line_count < MAX_REFLECT_LINES {
        let line = lines[*index];
        let trimmed = line.trim();

        // Count braces in this line.
        for ch in trimmed.chars() {
            match ch {
                '{' => {
                    depth = depth.saturating_add(1);
                    if depth > MAX_BRACE_DEPTH {
                        return Err(pattern_err(format!(
                            "[E418] Pattern '{name}' reflect body exceeds maximum brace depth ({MAX_BRACE_DEPTH})."
                        )));
                    }
                }
                '}' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        // End of reflect block.
                        *index += 1;
                        return Ok(raw_lines);
                    }
                }
                _ => {}
            }
        }

        // Store the trimmed line (skip empty lines and comments for cleaner body).
        if !trimmed.is_empty() && !trimmed.starts_with("//") {
            raw_lines.push(trimmed.to_string());
        }

        *index += 1;
        line_count += 1;
    }

    Err(pattern_err(format!("[E419] Pattern '{name}' reflect block not closed with '}}'.")))
}

// ---------------------------------------------------------------------------
// Pattern call parser
// ---------------------------------------------------------------------------

/// Check if a line looks like a pattern call: `identifier(args);`
///
/// Returns true if the line matches the pattern and the identifier is not a MIRR keyword.
pub fn is_pattern_call_line(line: &str) -> bool {
    let trimmed = line.trim();

    // Must end with ");".
    if !trimmed.ends_with(");") {
        return false;
    }

    // Must contain '('.
    let open = match trimmed.find('(') {
        Some(pos) => pos,
        None => return false,
    };

    // The identifier before '(' must be a valid non-keyword identifier.
    let ident = trimmed[..open].trim();
    if ident.is_empty() {
        return false;
    }

    // Check it's a valid identifier (alphanumeric + underscore).
    if !ident.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return false;
    }

    // Must not be a MIRR keyword.
    !KEYWORDS.contains(&ident)
}

/// Parse a pattern call line: `name(arg1, arg2, ...);`
///
/// Bounded: MAX_ARGS arguments.
pub fn parse_pattern_call(line: &str) -> Result<PatternCall, MirrError> {
    let trimmed = line.trim();

    // Strip trailing ";".
    let without_semi = trimmed
        .strip_suffix(';')
        .ok_or_else(|| pattern_err("[E420] Pattern call must end with ';'."))?
        .trim();

    // Find the opening paren.
    let open =
        without_semi.find('(').ok_or_else(|| pattern_err("[E421] Pattern call missing '('."))?;

    let pattern_name = without_semi[..open].trim();
    if pattern_name.is_empty() {
        return Err(pattern_err("[E422] Pattern call has empty name."));
    }

    // Find the closing paren.
    let close = without_semi.rfind(')').ok_or_else(|| {
        pattern_err(format!("[E423] Pattern call '{pattern_name}' missing closing ')'."))
    })?;

    let args_str = &without_semi[open + 1..close];
    let arguments = parse_call_args(args_str, pattern_name)?;

    Ok(PatternCall { pattern_name: pattern_name.to_string(), arguments, span: None })
}

/// Parse comma-separated call arguments.
///
/// Each argument is classified as:
/// - `true` / `false` -> PatternArg::ConstBool
/// - Numeric literal -> PatternArg::ConstInt
/// - Identifier -> PatternArg::SignalRef
///
/// Bounded: MAX_ARGS.
fn parse_call_args(args_str: &str, call_name: &str) -> Result<Vec<PatternArg>, MirrError> {
    let trimmed = args_str.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let parts: Vec<&str> = trimmed.split(',').collect();
    if parts.len() > MAX_ARGS {
        return Err(pattern_err(format!(
            "[E424] Pattern call '{call_name}' has too many arguments (max {MAX_ARGS})."
        )));
    }

    let mut args = Vec::with_capacity(parts.len());
    for part in &parts {
        let arg_str = part.trim();
        if arg_str.is_empty() {
            return Err(pattern_err(format!(
                "[E425] Pattern call '{call_name}' has empty argument."
            )));
        }

        let arg = if arg_str == "true" {
            PatternArg::ConstBool(true)
        } else if arg_str == "false" {
            PatternArg::ConstBool(false)
        } else if let Ok(n) = arg_str.parse::<u64>() {
            PatternArg::ConstInt(n)
        } else {
            // Treat as signal reference.
            PatternArg::SignalRef(arg_str.to_string())
        };

        args.push(arg);
    }

    Ok(args)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pattern_err(msg: impl Into<String>) -> MirrError {
    MirrError::PatternError { message: msg.into(), span: None }
}
