//! Guard and reflex declaration parsing.

#![forbid(unsafe_code)]

use super::parse_expression;
use super::skip_empty_and_comments;
use crate::ast::program::{Assignment, Guard, Reflex};
use crate::error::MirrError;
use crate::span::Span;

pub(super) fn parse_guard(lines: &[&str], index: &mut usize) -> Result<Guard, MirrError> {
    if *index >= lines.len() {
        return Err(MirrError::parse_error("[E119] Unexpected end of file in guard declaration.")
            .with_span(Some(Span::full_line((*index).saturating_sub(1) as u32))));
    }

    let start_line = *index;
    let header = lines[*index].trim();
    let after_keyword = header.strip_prefix("guard ").ok_or_else(|| {
        MirrError::parse_error("[E120] Malformed guard declaration.")
            .with_span(Some(Span::full_line(*index as u32)))
    })?;

    let (name_part, _) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::parse_error("[E121] Guard name cannot be empty.")
            .with_span(Some(Span::full_line(*index as u32))));
    }

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::parse_error(format!(
            "[E122] Guard '{name}' missing 'when' clause."
        ))
        .with_span(Some(Span::full_line((*index).saturating_sub(1) as u32))));
    }

    let when_line = lines[*index].trim();
    if !when_line.starts_with("when ") {
        return Err(MirrError::parse_error(format!(
            "[E123] Guard '{name}' expected 'when' line, found: {when_line}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let condition_str = when_line
        .strip_prefix("when ")
        .ok_or_else(|| {
            MirrError::parse_error("[E124] Malformed 'when' line.")
                .with_span(Some(Span::full_line(*index as u32)))
        })?
        .trim();

    let condition = parse_expression(condition_str).map_err(|e| {
        MirrError::parse_error(format!("[E125] Guard '{name}' condition parse error: {e}"))
            .with_span(Some(Span::full_line(*index as u32)))
    })?;

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::parse_error(format!("[E126] Guard '{name}' missing 'for' clause."))
            .with_span(Some(Span::full_line((*index).saturating_sub(1) as u32))));
    }

    let for_line = lines[*index].trim();
    if !for_line.starts_with("for ") {
        return Err(MirrError::parse_error(format!(
            "[E127] Guard '{name}' expected 'for' line, found: {for_line}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let after_for = for_line
        .strip_prefix("for ")
        .ok_or_else(|| {
            MirrError::parse_error("[E128] Malformed 'for' line.")
                .with_span(Some(Span::full_line(*index as u32)))
        })?
        .trim_start();

    let mut for_parts = after_for.split_whitespace();
    let cycles_str = for_parts.next().ok_or_else(|| {
        MirrError::parse_error("[E129] Expected cycle count after 'for'.")
            .with_span(Some(Span::full_line(*index as u32)))
    })?;

    let cycles: u64 = cycles_str.trim().parse().map_err(|_| {
        MirrError::parse_error(format!(
            "[E130] Invalid cycle count in guard '{name}': {cycles_str}"
        ))
        .with_span(Some(Span::full_line(*index as u32)))
    })?;

    *index += 1;
    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::parse_error(format!("[E131] Guard '{name}' not closed with '}}'."))
            .with_span(Some(Span::full_line((*index).saturating_sub(1) as u32))));
    }

    let closing = lines[*index].trim();
    if closing != "}" {
        return Err(MirrError::parse_error(format!(
            "[E132] Guard '{name}' expected closing '}}', found: {closing}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    *index += 1;

    Ok(Guard {
        name: name.to_string(),
        condition,
        cycles,
        origin: None,
        span: Some(Span::multi_line(start_line as u32, (*index - 1) as u32)),
    })
}

/// Parse a single assignment line like `clamp_valve = true;` into an
/// Assignment struct with a parsed expression on the RHS.
fn parse_assignment(line: &str, line_index: usize) -> Result<Assignment, MirrError> {
    // Strip inline comments before processing.
    let line = if let Some(pos) = line.find("//") { line[..pos].trim_end() } else { line };
    let stripped = line.strip_suffix(';').unwrap_or(line).trim();

    let (lhs, rhs) = stripped.split_once('=').ok_or_else(|| {
        MirrError::parse_error(format!("[E133] Assignment missing '=': {stripped}"))
            .with_span(Some(Span::full_line(line_index as u32)))
    })?;

    let target = lhs.trim();
    if target.is_empty() {
        return Err(MirrError::parse_error("[E134] Assignment target cannot be empty.")
            .with_span(Some(Span::full_line(line_index as u32))));
    }

    let rhs_str = rhs.trim();
    if rhs_str.is_empty() {
        return Err(MirrError::parse_error(format!(
            "[E135] Assignment to '{target}' has empty right-hand side."
        ))
        .with_span(Some(Span::full_line(line_index as u32))));
    }

    let value = parse_expression(rhs_str).map_err(|e| {
        MirrError::parse_error(format!("[E136] Error in assignment to '{target}': {e}"))
            .with_span(Some(Span::full_line(line_index as u32)))
    })?;

    Ok(Assignment {
        target: target.to_string(),
        value,
        span: Some(Span::full_line(line_index as u32)),
    })
}

pub(super) fn parse_reflex(lines: &[&str], index: &mut usize) -> Result<Reflex, MirrError> {
    if *index >= lines.len() {
        return Err(MirrError::parse_error("[E137] Unexpected end of file in reflex declaration.")
            .with_span(Some(Span::full_line((*index).saturating_sub(1) as u32))));
    }

    let start_line = *index;
    let header = lines[*index].trim();
    let after_keyword = header.strip_prefix("reflex ").ok_or_else(|| {
        MirrError::parse_error("[E138] Malformed reflex declaration.")
            .with_span(Some(Span::full_line(*index as u32)))
    })?;

    let (name_part, _) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::parse_error("[E139] Reflex name cannot be empty.")
            .with_span(Some(Span::full_line(*index as u32))));
    }

    // Parse guard names from the header's 'when' clause (e.g. "reflex r when [g] { ... }").
    let mut guard_names: Vec<String> = Vec::new();
    if let Some(pos) = name_part.find("when") {
        let when_part = name_part[pos + "when".len()..].trim();
        // Allow optional surrounding brackets and split on 'and'.
        let when_body = when_part
            .trim_start_matches('[')
            .trim_end_matches(']')
            .trim();
        for part in when_body.split("and") {
            let g = part.trim().trim_start_matches('[').trim_end_matches(']').trim();
            if !g.is_empty() {
                guard_names.push(g.to_string());
            }
        }
    }

    *index += 1;
    skip_empty_and_comments(lines, index);

    // Optional 'on' clause may override the guard list parsed from the 'when' clause.
    if *index < lines.len() {
        let on_line = lines[*index].trim();
        if on_line.starts_with("on ") {
            let after_on = on_line.strip_prefix("on ").ok_or_else(|| {
                MirrError::parse_error("[E142] Malformed 'on' line.")
                    .with_span(Some(Span::full_line(*index as u32)))
            })?;

            let (guards_part, _) = match after_on.split_once('{') {
                Some(parts) => parts,
                None => (after_on, ""),
            };

            guard_names.clear();
            for part in guards_part.split("and") {
                let g = part.trim();
                if !g.is_empty() {
                    guard_names.push(g.to_string());
                }
            }

            *index += 1;
            skip_empty_and_comments(lines, index);
        }
    }

    if guard_names.is_empty() {
        return Err(MirrError::parse_error(format!(
            "[E140] Reflex '{name}' missing 'on' clause."
        ))
        .with_span(Some(Span::full_line((*index).saturating_sub(1) as u32))));
    }

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

        let assignment = parse_assignment(line, *index).map_err(|e| {
            MirrError::parse_error(format!("[E144] In reflex '{name}': {e}"))
                .with_span(Some(Span::full_line(*index as u32)))
        })?;
        assignments.push(assignment);

        *index += 1;
    }

    skip_empty_and_comments(lines, index);

    if *index >= lines.len() {
        return Err(MirrError::parse_error(format!(
            "[E145] Reflex '{name}' not closed with '}}'."
        ))
        .with_span(Some(Span::full_line((*index).saturating_sub(1) as u32))));
    }

    let closing = lines[*index].trim();
    if closing != "}" {
        return Err(MirrError::parse_error(format!(
            "[E146] Reflex '{name}' expected closing '}}', found: {closing}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    *index += 1;

    Ok(Reflex {
        name: name.to_string(),
        guard_names,
        assignments,
        origin: None,
        span: Some(Span::multi_line(start_line as u32, (*index - 1) as u32)),
    })
}
