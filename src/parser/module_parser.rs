//! Module-level parser for MIRR source files.
//!
//! Parses the top-level `module` block and all nested declarations: signals,
//! guards, reflexes, properties, and pattern calls. Also dispatches `def` blocks
//! to the pattern parser.

use super::expr_parser::parse_expression;
use super::pattern_parser::{is_pattern_call_line, parse_pattern_call, parse_pattern_def};
use crate::ast::pattern::PatternDef;
use crate::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use crate::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use crate::ast::types::{SignalKind, SignalType};
use crate::error::MirrError;

/// Maximum number of top-level `def` blocks allowed.
const MAX_PATTERN_DEFS: usize = 64;

/// Parse a MIRR source file into an in-memory representation.
///
/// Handles zero or more top-level `def` blocks before the `module` declaration.
pub fn parse_mirr(source: &str) -> Result<MirrProgram, MirrError> {
    let lines: Vec<&str> = source.lines().collect();
    let mut index = 0usize;

    // Parse top-level `def` blocks (bounded by MAX_PATTERN_DEFS).
    let mut patterns: Vec<PatternDef> = Vec::new();
    let mut def_count = 0usize;

    loop {
        skip_empty_and_comments(&lines, &mut index);
        if index >= lines.len() {
            break;
        }
        let line = lines[index].trim();
        if line.starts_with("def ") {
            if def_count >= MAX_PATTERN_DEFS {
                return Err(MirrError::PatternError {
                    message: format!("Too many pattern definitions (max {MAX_PATTERN_DEFS})."),
                });
            }
            let pat = parse_pattern_def(&lines, &mut index)?;
            patterns.push(pat);
            def_count += 1;
        } else {
            break;
        }
    }

    skip_empty_and_comments(&lines, &mut index);

    if index >= lines.len() {
        return Err(MirrError::new("MIRR source is empty."));
    }

    let module = parse_module(&lines, &mut index)?;

    Ok(MirrProgram { patterns, module })
}

fn skip_empty_and_comments(lines: &[&str], index: &mut usize) {
    while *index < lines.len() {
        let line = lines[*index].trim();
        if line.is_empty() || line.starts_with("//") {
            *index += 1;
        } else {
            break;
        }
    }
}

fn parse_module(lines: &[&str], index: &mut usize) -> Result<Module, MirrError> {
    if *index >= lines.len() {
        return Err(MirrError::new("Expected 'module' declaration but found end of file."));
    }

    let header = lines[*index].trim();

    if !header.starts_with("module ") {
        return Err(MirrError::new(format!("Expected 'module' declaration, found: {header}")));
    }

    let after_keyword = header
        .strip_prefix("module ")
        .ok_or_else(|| MirrError::new("Malformed module declaration."))?;

    let (name_part, _) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::new("Module name cannot be empty."));
    }

    let mut module = Module {
        name: name.to_string(),
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
    };

    *index += 1;

    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }

        let line = lines[*index].trim();

        if line == "}" {
            // End of module.
            *index += 1;
            return Ok(module);
        } else if line.starts_with("signal ") {
            let signal = parse_signal(line)?;
            module.signals.push(signal);
            *index += 1;
        } else if line.starts_with("guard ") {
            let guard = parse_guard(lines, index)?;
            module.guards.push(guard);
        } else if line.starts_with("reflex ") {
            let reflex = parse_reflex(lines, index)?;
            module.reflexes.push(reflex);
        } else if line.starts_with("property ") {
            let prop = parse_property(lines, index)?;
            module.properties.push(prop);
        } else if is_pattern_call_line(line) {
            let call = parse_pattern_call(line)?;
            module.pattern_calls.push(call);
            *index += 1;
        } else {
            return Err(MirrError::new(format!(
                "Unexpected line inside module '{}': {}",
                module.name, line
            )));
        }
    }

    Err(MirrError::new(format!("Module '{}' was not closed with '}}'.", module.name)))
}

fn parse_signal(line: &str) -> Result<SignalDecl, MirrError> {
    let after_keyword = line
        .strip_prefix("signal ")
        .ok_or_else(|| MirrError::new("Malformed signal declaration."))?;

    let trimmed = after_keyword.trim();
    let without_semicolon = trimmed
        .strip_suffix(';')
        .ok_or_else(|| MirrError::new("Signal declaration must end with ';'."))?;

    let (name_part, rest) = without_semicolon
        .split_once(':')
        .ok_or_else(|| MirrError::new("Signal declaration must contain ':'."))?;

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::new("Signal name cannot be empty."));
    }

    let rest = rest.trim();
    let mut parts = rest.split_whitespace();

    let kind_str =
        parts.next().ok_or_else(|| MirrError::new("Signal kind (in/out/internal) is missing."))?;
    let ty_str = parts.next().ok_or_else(|| MirrError::new("Signal type (bool/uN) is missing."))?;

    if parts.next().is_some() {
        return Err(MirrError::new("Too many tokens in signal declaration."));
    }

    let kind = match kind_str {
        "in" => SignalKind::Input,
        "out" => SignalKind::Output,
        "internal" => SignalKind::Internal,
        other => {
            return Err(MirrError::new(format!(
                "Unknown signal kind: {other}. Expected 'in', 'out', or 'internal'."
            )));
        }
    };

    let ty = if ty_str == "bool" {
        SignalType::Bool
    } else if let Some(width_str) = ty_str.strip_prefix('u') {
        let width: u32 = width_str.parse().map_err(|_| {
            MirrError::new(format!(
                "Invalid unsigned width in type '{ty_str}'. Expected something like 'u16'."
            ))
        })?;
        SignalType::Unsigned(width)
    } else if let Some(width_str) = ty_str.strip_prefix('i') {
        let width: u32 = width_str.parse().map_err(|_| {
            MirrError::new(format!(
                "Invalid signed width in type '{ty_str}'. Expected something like 'i16'."
            ))
        })?;
        SignalType::Signed(width)
    } else {
        return Err(MirrError::new(format!(
            "Unknown signal type: {ty_str}. Expected 'bool', 'uN', or 'iN'."
        )));
    };

    Ok(SignalDecl { name: name.to_string(), kind, ty, origin: None })
}

fn parse_guard(lines: &[&str], index: &mut usize) -> Result<Guard, MirrError> {
    if *index >= lines.len() {
        return Err(MirrError::new("Unexpected end of file in guard declaration."));
    }

    let header = lines[*index].trim();
    let after_keyword = header
        .strip_prefix("guard ")
        .ok_or_else(|| MirrError::new("Malformed guard declaration."))?;

    let (name_part, _) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::new("Guard name cannot be empty."));
    }

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::new(format!("Guard '{name}' missing 'when' clause.")));
    }

    let when_line = lines[*index].trim();
    if !when_line.starts_with("when ") {
        return Err(MirrError::new(format!(
            "Guard '{name}' expected 'when' line, found: {when_line}"
        )));
    }

    let condition_str = when_line
        .strip_prefix("when ")
        .ok_or_else(|| MirrError::new("Malformed 'when' line."))?
        .trim();

    let condition = parse_expression(condition_str)
        .map_err(|e| MirrError::new(format!("Guard '{name}' condition parse error: {e}")))?;

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::new(format!("Guard '{name}' missing 'for' clause.")));
    }

    let for_line = lines[*index].trim();
    if !for_line.starts_with("for ") {
        return Err(MirrError::new(format!(
            "Guard '{name}' expected 'for' line, found: {for_line}"
        )));
    }

    let after_for = for_line
        .strip_prefix("for ")
        .ok_or_else(|| MirrError::new("Malformed 'for' line."))?
        .trim_start();

    let mut for_parts = after_for.split_whitespace();
    let cycles_str =
        for_parts.next().ok_or_else(|| MirrError::new("Expected cycle count after 'for'."))?;

    let cycles: u64 = cycles_str.trim().parse().map_err(|_| {
        MirrError::new(format!("Invalid cycle count in guard '{name}': {cycles_str}"))
    })?;

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::new(format!("Guard '{name}' not closed with '}}'.")));
    }

    let closing = lines[*index].trim();
    if closing != "}" {
        return Err(MirrError::new(format!(
            "Guard '{name}' expected closing '}}', found: {closing}"
        )));
    }

    *index += 1;

    Ok(Guard { name: name.to_string(), condition, cycles, origin: None })
}

/// Parse a single assignment line like `clamp_valve = true;` into an
/// Assignment struct with a parsed expression on the RHS.
fn parse_assignment(line: &str) -> Result<Assignment, MirrError> {
    // Strip inline comments before processing.
    let line = if let Some(pos) = line.find("//") { line[..pos].trim_end() } else { line };
    let stripped = line.strip_suffix(';').unwrap_or(line).trim();

    let (lhs, rhs) = stripped
        .split_once('=')
        .ok_or_else(|| MirrError::new(format!("Assignment missing '=': {stripped}")))?;

    let target = lhs.trim();
    if target.is_empty() {
        return Err(MirrError::new("Assignment target cannot be empty."));
    }

    let rhs_str = rhs.trim();
    if rhs_str.is_empty() {
        return Err(MirrError::new(format!("Assignment to '{target}' has empty right-hand side.")));
    }

    let value = parse_expression(rhs_str)
        .map_err(|e| MirrError::new(format!("Error in assignment to '{target}': {e}")))?;

    Ok(Assignment { target: target.to_string(), value })
}

fn parse_reflex(lines: &[&str], index: &mut usize) -> Result<Reflex, MirrError> {
    if *index >= lines.len() {
        return Err(MirrError::new("Unexpected end of file in reflex declaration."));
    }

    let header = lines[*index].trim();
    let after_keyword = header
        .strip_prefix("reflex ")
        .ok_or_else(|| MirrError::new("Malformed reflex declaration."))?;

    let (name_part, _) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::new("Reflex name cannot be empty."));
    }

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::new(format!("Reflex '{name}' missing 'on' clause.")));
    }

    let on_line = lines[*index].trim();
    if !on_line.starts_with("on ") {
        return Err(MirrError::new(format!(
            "Reflex '{name}' expected 'on' line, found: {on_line}"
        )));
    }

    let after_on =
        on_line.strip_prefix("on ").ok_or_else(|| MirrError::new("Malformed 'on' line."))?;

    let (guards_part, _) = match after_on.split_once('{') {
        Some(parts) => parts,
        None => (after_on, ""),
    };

    let mut guard_names = Vec::new();
    for part in guards_part.split("and") {
        let g = part.trim();
        if !g.is_empty() {
            guard_names.push(g.to_string());
        }
    }

    if guard_names.is_empty() {
        return Err(MirrError::new(format!("Reflex '{name}' has no guard names in 'on' clause.")));
    }

    *index += 1;
    skip_empty_and_comments(lines, index);

    let mut assignments = Vec::new();

    while *index < lines.len() {
        let line = lines[*index].trim();

        if line.is_empty() || line.starts_with("//") {
            *index += 1;
            continue;
        }

        if line == "}" {
            // End of inner block (assignments).
            *index += 1;
            break;
        }

        let assignment = parse_assignment(line)
            .map_err(|e| MirrError::new(format!("In reflex '{name}': {e}")))?;
        assignments.push(assignment);

        *index += 1;
    }

    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::new(format!("Reflex '{name}' not closed with '}}'.")));
    }

    let closing = lines[*index].trim();
    if closing != "}" {
        return Err(MirrError::new(format!(
            "Reflex '{name}' expected closing '}}', found: {closing}"
        )));
    }

    *index += 1;

    Ok(Reflex { name: name.to_string(), guard_names, assignments, origin: None })
}

fn parse_property(lines: &[&str], index: &mut usize) -> Result<PropertyDecl, MirrError> {
    if *index >= lines.len() {
        return Err(MirrError::new("Unexpected end of file in property declaration."));
    }

    let header = lines[*index].trim();
    let after_keyword = header
        .strip_prefix("property ")
        .ok_or_else(|| MirrError::new("Malformed property declaration."))?;

    let (name_part, _) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::new("Property name cannot be empty."));
    }

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::new(format!("Property '{name}' missing formula (always/never).")));
    }

    let formula_line = lines[*index].trim();
    let (directive, formula) = parse_property_formula(formula_line, name)?;

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::new(format!("Property '{name}' not closed with '}}'.")));
    }

    let closing = lines[*index].trim();
    if closing != "}" {
        return Err(MirrError::new(format!(
            "Property '{name}' expected closing '}}', found: {closing}"
        )));
    }

    *index += 1;

    Ok(PropertyDecl { name: name.to_string(), directive, formula, origin: None })
}

fn parse_property_formula(
    line: &str,
    name: &str,
) -> Result<(PropertyDirective, PropertyFormula), MirrError> {
    let stripped = line.strip_suffix(';').unwrap_or(line).trim();

    // Detect directive prefix: cover / assume
    let (directive, rest) = if let Some(after) = stripped.strip_prefix("cover") {
        let after = after.trim();
        // "cover" must be followed by a formula keyword or parens
        if after.is_empty()
            || after.starts_with('(')
            || after.starts_with("always")
            || after.starts_with("never")
            || after.starts_with("eventually")
        {
            (PropertyDirective::Cover, after)
        } else {
            (PropertyDirective::Assert, stripped)
        }
    } else if let Some(after) = stripped.strip_prefix("assume") {
        let after = after.trim();
        if after.is_empty()
            || after.starts_with('(')
            || after.starts_with("always")
            || after.starts_with("never")
            || after.starts_with("eventually")
        {
            (PropertyDirective::Assume, after)
        } else {
            (PropertyDirective::Assert, stripped)
        }
    } else {
        (PropertyDirective::Assert, stripped)
    };

    let formula = parse_formula_body(rest, name)?;
    Ok((directive, formula))
}

fn parse_formula_body(stripped: &str, name: &str) -> Result<PropertyFormula, MirrError> {
    // Direct parenthesized form: cover (P) / assume (P) / (shorthand for always)
    if stripped.starts_with('(') && stripped.ends_with(')') {
        let inner = &stripped[1..stripped.len() - 1];
        return parse_always_or_implies_inner(inner, name);
    }

    if let Some(body) = stripped.strip_prefix("always") {
        return parse_always_body(body.trim(), name);
    }

    if let Some(body) = stripped.strip_prefix("never") {
        return parse_never_body(body.trim(), name);
    }

    if let Some(body) = stripped.strip_prefix("eventually") {
        return parse_eventually_body(body.trim(), name);
    }

    Err(MirrError::new(format!(
        "Property '{name}' formula must start with 'always', 'never', or 'eventually'."
    )))
}

fn parse_always_body(body: &str, name: &str) -> Result<PropertyFormula, MirrError> {
    let inner = unwrap_parens(body, name, "always")?;

    // Check for followed_by pattern: always (P followed_by N Q)
    if let Some(formula) = try_parse_followed_by(inner, name)? {
        return Ok(formula);
    }

    parse_always_or_implies_inner(inner, name)
}

/// Parse the inner content which may be a simple expression or an implication.
fn parse_always_or_implies_inner(inner: &str, name: &str) -> Result<PropertyFormula, MirrError> {
    if let Some((lhs, rhs)) = inner.split_once(" -> ") {
        let antecedent = parse_expression(lhs.trim())
            .map_err(|e| MirrError::new(format!("Property '{name}' antecedent error: {e}")))?;
        let consequent = parse_expression(rhs.trim())
            .map_err(|e| MirrError::new(format!("Property '{name}' consequent error: {e}")))?;
        return Ok(PropertyFormula::AlwaysImplies { antecedent, consequent });
    }

    let expr = parse_expression(inner)
        .map_err(|e| MirrError::new(format!("Property '{name}' formula error: {e}")))?;
    Ok(PropertyFormula::Always(expr))
}

fn parse_never_body(body: &str, name: &str) -> Result<PropertyFormula, MirrError> {
    let inner = unwrap_parens(body, name, "never")?;

    // Check for never (P -> Q) — NeverImplies
    if let Some((lhs, rhs)) = inner.split_once(" -> ") {
        let antecedent = parse_expression(lhs.trim())
            .map_err(|e| MirrError::new(format!("Property '{name}' antecedent error: {e}")))?;
        let consequent = parse_expression(rhs.trim())
            .map_err(|e| MirrError::new(format!("Property '{name}' consequent error: {e}")))?;
        return Ok(PropertyFormula::NeverImplies { antecedent, consequent });
    }

    let expr = parse_expression(inner)
        .map_err(|e| MirrError::new(format!("Property '{name}' formula error: {e}")))?;
    Ok(PropertyFormula::Never(expr))
}

fn parse_eventually_body(body: &str, name: &str) -> Result<PropertyFormula, MirrError> {
    // Expected: "within N (P)"
    let rest = body
        .strip_prefix("within")
        .ok_or_else(|| {
            MirrError::new(format!("Property '{name}': expected 'eventually within N (expr)'."))
        })?
        .trim();

    // Split off the cycle count before the '('
    let paren_pos = rest.find('(').ok_or_else(|| {
        MirrError::new(format!(
            "Property '{name}': eventually within requires parenthesized expression."
        ))
    })?;

    let cycles_str = rest[..paren_pos].trim();
    let cycles: u32 = cycles_str.parse().map_err(|_| {
        MirrError::new(format!(
            "Property '{name}': invalid cycle count '{cycles_str}' in eventually within."
        ))
    })?;

    if cycles < 1 {
        return Err(MirrError::new(format!(
            "Property '{name}': eventually within requires cycles >= 1."
        )));
    }

    let expr_part = &rest[paren_pos..];
    let inner = unwrap_parens(expr_part, name, "eventually within")?;
    let expr = parse_expression(inner)
        .map_err(|e| MirrError::new(format!("Property '{name}' formula error: {e}")))?;

    Ok(PropertyFormula::EventuallyWithin { expr, cycles })
}

/// Try to parse "P followed_by N Q" inside an `always (...)` body.
/// Returns `None` if the pattern is not found.
fn try_parse_followed_by(inner: &str, name: &str) -> Result<Option<PropertyFormula>, MirrError> {
    let Some(fb_pos) = inner.find(" followed_by ") else {
        return Ok(None);
    };

    let trigger_str = &inner[..fb_pos].trim();
    let after_fb = inner[fb_pos + " followed_by ".len()..].trim();

    // Split delay from response: "N Q" where Q may contain spaces
    let space_pos = after_fb.find(' ').ok_or_else(|| {
        MirrError::new(format!(
            "Property '{name}': expected 'P followed_by N Q' with delay and response expression."
        ))
    })?;

    let delay_str = &after_fb[..space_pos];
    let response_str = after_fb[space_pos + 1..].trim();

    let delay_cycles: u32 = delay_str.parse().map_err(|_| {
        MirrError::new(format!("Property '{name}': invalid delay '{delay_str}' in followed_by."))
    })?;

    if delay_cycles < 1 {
        return Err(MirrError::new(format!("Property '{name}': followed_by requires delay >= 1.")));
    }

    let trigger = parse_expression(trigger_str)
        .map_err(|e| MirrError::new(format!("Property '{name}' trigger error: {e}")))?;
    let response = parse_expression(response_str)
        .map_err(|e| MirrError::new(format!("Property '{name}' response error: {e}")))?;

    Ok(Some(PropertyFormula::AlwaysFollowedBy { trigger, response, delay_cycles }))
}

fn unwrap_parens<'a>(body: &'a str, name: &str, keyword: &str) -> Result<&'a str, MirrError> {
    let trimmed = body.trim();
    if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
        return Err(MirrError::new(format!(
            "Property '{name}': {keyword} formula must be wrapped in parentheses."
        )));
    }
    Ok(&trimmed[1..trimmed.len() - 1])
}
