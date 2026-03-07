// ---------------------------------------------------------------------------
// Line-based module parser
// ---------------------------------------------------------------------------
// Single responsibility: parse MIRR source text into the program AST.
// Handles module, signal, guard, and reflex declarations.
// ---------------------------------------------------------------------------

use super::expr_parser::parse_expression;
use crate::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use crate::ast::types::{SignalKind, SignalType};
use crate::error::MirrError;

/// Parse a MIRR source file into an in-memory representation.
pub fn parse_mirr(source: &str) -> Result<MirrProgram, MirrError> {
    let lines: Vec<&str> = source.lines().collect();
    let mut index = 0usize;

    skip_empty_and_comments(&lines, &mut index);

    if index >= lines.len() {
        return Err(MirrError::new("MIRR source is empty."));
    }

    let module = parse_module(&lines, &mut index)?;

    Ok(MirrProgram { module })
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
    } else {
        return Err(MirrError::new(format!(
            "Unknown signal type: {ty_str}. Expected 'bool' or 'uN'."
        )));
    };

    Ok(SignalDecl { name: name.to_string(), kind, ty })
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

    Ok(Guard { name: name.to_string(), condition, cycles })
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

    Ok(Reflex { name: name.to_string(), guard_names, assignments })
}
