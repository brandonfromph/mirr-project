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
pub(crate) const MAX_PARAMS: usize = 128;

/// Maximum number of arguments in a pattern call.
const MAX_ARGS: usize = 128;

/// Maximum number of lines in a reflect body.
pub(crate) const MAX_REFLECT_LINES: usize = 8192;

/// Maximum brace nesting depth inside a reflect body.
const MAX_BRACE_DEPTH: usize = 16;

/// MIRR keywords that cannot be pattern call names.
const KEYWORDS: &[&str] = &[
    "signal", "guard", "reflex", "property", "module", "def", "reflect", "when", "for", "on",
    "always", "never", "true", "false", "in", "out", "internal", "cycles", "bool", "and",
    // MEGA-1 type annotation keywords:
    "linear", "stateful", "pure", "where", "calls",
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
        return Err(pattern_err(format!(
            "{} Unexpected end of file in pattern definition.",
            crate::error_codes::ec(401)
        )));
    }

    // Collect the full header (may span multiple lines until we see `{` after `)`)
    let header = collect_def_header(lines, index)?;

    // Extract name and param string from header.
    let after_def = header.strip_prefix("def ").ok_or_else(|| {
        pattern_err(format!("{} Malformed pattern definition.", crate::error_codes::ec(402)))
    })?;

    let open_paren = after_def.find('(').ok_or_else(|| {
        pattern_err(format!("{} Pattern definition missing '('.", crate::error_codes::ec(403)))
    })?;

    let name = after_def[..open_paren].trim();
    if name.is_empty() {
        return Err(pattern_err(format!(
            "{} Pattern name cannot be empty.",
            crate::error_codes::ec(404)
        )));
    }

    // Find the matching close paren.
    let close_paren = after_def.rfind(')').ok_or_else(|| {
        pattern_err(format!("{} Pattern '{name}' missing closing ')'", crate::error_codes::ec(405)))
    })?;

    let param_str = &after_def[open_paren + 1..close_paren];
    let params = parse_pattern_params(param_str, name)?;

    // Now we should be inside the def body. Look for `reflect {`.
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(pattern_err(format!(
            "{} Pattern '{name}' missing 'reflect' block.",
            crate::error_codes::ec(406)
        )));
    }

    let reflect_line = lines[*index].trim();
    if !reflect_line.starts_with("reflect") {
        return Err(pattern_err(format!(
            "{} Pattern '{name}' expected 'reflect' block, found: {reflect_line}",
            crate::error_codes::ec(407)
        )));
    }

    // Check for opening brace on the reflect line or next line.
    if !reflect_line.contains('{') {
        *index += 1;
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() || !lines[*index].trim().starts_with('{') {
            return Err(pattern_err(format!(
                "{} Pattern '{name}' reflect block missing opening '{{'.",
                crate::error_codes::ec(408)
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
            header.push('\n');
        }
        header.push_str(line);

        // Check if we've seen closing parenthesis followed by opening brace
        let clean_check = header.replace(|c: char| c.is_whitespace(), "");
        if clean_check.contains("){") {
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

    Err(pattern_err(format!(
        "{} Pattern definition header not closed with ') {{'.",
        crate::error_codes::ec(409)
    )))
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

    let mut clean_param_str = String::with_capacity(param_str.len());
    for line in param_str.lines() {
        let mut line_part = line;
        if let Some(pos) = line.find("//") {
            line_part = &line[..pos];
        }
        clean_param_str.push_str(line_part);
        clean_param_str.push(' ');
    }
    let trimmed_clean = clean_param_str.trim();

    let parts: Vec<&str> = trimmed_clean.split(',').collect();
    if parts.len() > MAX_PARAMS {
        return Err(pattern_err(format!(
            "{} Pattern '{name}' has too many parameters (max {MAX_PARAMS}).",
            crate::error_codes::ec(410)
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
    let (pname, type_part) = match param_str.split_once(':') {
        Some((n, t)) => (n.trim(), Some(t.trim())),
        None => (param_str.trim(), None),
    };

    if pname.is_empty() {
        return Err(pattern_err(format!(
            "{} Pattern '{def_name}' has parameter with empty name.",
            crate::error_codes::ec(412)
        )));
    }

    let type_str = type_part.unwrap_or("signal bool");

    // Check if it's a signal parameter (starts with "signal").
    if let Some(after_signal) = type_str.strip_prefix("signal") {
        let rest = after_signal.trim();
        // Special case: if it was untyped, we've defaulted it to "signal bool".
        // If it was typed "name: signal", rest is empty. Default to bool.
        let rest = if rest.is_empty() { "bool" } else { rest };

        // Delegate to the shared MEGA-1 tokenizer.
        let parsed = crate::parser::tokenize_signal_decl(rest).map_err(|e| {
            pattern_err(format!(
                "{} Pattern '{def_name}' signal parameter '{pname}': {}",
                crate::error_codes::ec(413),
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
        let (ty, annotations) =
            crate::parser::parse_type_with_annotations(type_str).map_err(|e| {
                pattern_err(format!(
                    "{} Pattern '{def_name}' constant parameter '{pname}': {}",
                    crate::error_codes::ec(414),
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
                        return Err(pattern_err(format!("{} Pattern '{name}' reflect body exceeds maximum brace depth ({MAX_BRACE_DEPTH}).", crate::error_codes::ec(418))));
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

    Err(pattern_err(format!(
        "{} Pattern '{name}' reflect block not closed with '}}'.",
        crate::error_codes::ec(419)
    )))
}

// ---------------------------------------------------------------------------
// Pattern call parser
// ---------------------------------------------------------------------------

/// Check if a line looks like the start of a pattern call: `identifier(`
///
/// Returns true if the line starts with an identifier followed by '(' and is not a keyword.
pub fn is_pattern_call_start(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() {
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

    // Check it's a valid identifier (alphanumeric + underscore + colon).
    for c in ident.chars() {
        if !(c.is_ascii_alphanumeric() || c == '_' || c == ':') {
            return false;
        }
    }

    // Must not be a MIRR keyword.
    let is_kw = KEYWORDS.contains(&ident);
    if is_kw {
        return false;
    }

    true
}

/// Check if a line looks like a single-line pattern call: `identifier(args);`
///
/// Returns true if the line matches the pattern and the identifier is not a MIRR keyword.
pub fn is_pattern_call_line(line: &str) -> bool {
    let trimmed = line.trim();

    // Must end with ");".
    if !trimmed.ends_with(");") {
        return false;
    }

    is_pattern_call_start(line)
}

/// Parse a pattern call (may span multiple lines): `name(arg1, arg2, ...);`
///
/// Bounded: MAX_ARGS arguments, MAX_HEADER_LINES lines.
pub fn parse_pattern_call(lines: &[&str], index: &mut usize) -> Result<PatternCall, MirrError> {
    // Collect the full call (may span multiple lines until we see `);`)
    let full_call = collect_call_header(lines, index)?;
    parse_pattern_call_str(&full_call)
}

/// Parse a pattern call from a single-line string.
pub fn parse_pattern_call_single(line: &str) -> Result<PatternCall, MirrError> {
    parse_pattern_call_str(line)
}

/// Internal helper: parse a pattern call from a joined string.
fn parse_pattern_call_str(full_call: &str) -> Result<PatternCall, MirrError> {
    let trimmed = full_call.trim();

    // Strip trailing ";".
    let without_semi = trimmed
        .strip_suffix(';')
        .ok_or_else(|| {
            pattern_err(format!("{} Pattern call must end with ';'.", crate::error_codes::ec(420)))
        })?
        .trim();

    // Find the opening paren.
    let open = without_semi.find('(').ok_or_else(|| {
        pattern_err(format!("{} Pattern call missing '('.", crate::error_codes::ec(421)))
    })?;

    let pattern_name = without_semi[..open].trim();
    if pattern_name.is_empty() {
        return Err(pattern_err(format!(
            "{} Pattern call has empty name.",
            crate::error_codes::ec(422)
        )));
    }

    // Find the closing paren.
    let close = without_semi.rfind(')').ok_or_else(|| {
        pattern_err(format!(
            "{} Pattern call '{pattern_name}' missing closing ')'.",
            crate::error_codes::ec(423)
        ))
    })?;

    let args_str = &without_semi[open + 1..close];
    let arguments = parse_call_args(args_str, pattern_name)?;

    Ok(PatternCall { pattern_name: pattern_name.to_string(), arguments, span: None })
}

/// Collect a pattern call header, which may span multiple lines.
///
/// Joins lines until we see `);`. Returns the joined string.
/// Bounded: at most 64 lines.
fn collect_call_header(lines: &[&str], index: &mut usize) -> Result<String, MirrError> {
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

        // Check if we've seen `);` marking end of call.
        if header.ends_with(");") || header.contains(");") {
            return Ok(header);
        }
    }

    Err(pattern_err(format!(
        "{} Pattern call header not closed with ');'.",
        crate::error_codes::ec(420)
    )))
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
            "{} Pattern call '{call_name}' has too many arguments (max {MAX_ARGS}).",
            crate::error_codes::ec(424)
        )));
    }

    let mut args = Vec::with_capacity(parts.len());
    for part in &parts {
        let arg_str = part.trim();
        if arg_str.is_empty() {
            return Err(pattern_err(format!(
                "{} Pattern call '{call_name}' has empty argument.",
                crate::error_codes::ec(425)
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
