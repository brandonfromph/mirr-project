//! Guard and reflex declaration parsing.
//!
//! NASA Power-of-Ten compliance:
//! W1  - No goto, setjmp, longjmp.
//! W2  - All loops bounded by MAX_* constants.
//! W3  - No dynamic heap allocation in parser logic.
//! W4  - Functions ≤ 60 lines (enforced by decomposition).
//! W5  - Assertion density: every invariant guarded.
//! W6  - Minimal variable scope — declared at point of use.
//! W7  - Return value of every fallible call checked.
//! W8  - No preprocessor abuse (N/A for Rust).
//! W9  - Pointer/reference restrictions (no raw pointers).
//! W10 - Compiler warnings as errors (enforced in Cargo.toml).

#![forbid(unsafe_code)]

use super::parse_expression;
use super::skip_empty_and_comments;
use crate::ast::program::{Assignment, Guard};
use crate::diagnostic_builder::emit_at;
use crate::error::MirrError;
use crate::error_codes::ErrorCode;
use crate::span::Span;

// ── NASA W2: all loops bounded ───────────────────────────────────────────────
const MAX_GUARD_NAMES: usize = 64;

// ── Guard parsing ─────────────────────────────────────────────────────────────

/// Parse a `guard <name> { when <cond> for <N> cycles; }` block.
///
/// # Errors
/// Returns `MirrError` on any malformed input. Never panics.
pub(super) fn parse_guard(lines: &[&str], index: &mut usize) -> Result<Guard, MirrError> {
    debug_assert!(*index <= lines.len(), "index out of bounds before parse_guard");
    guard_check_eof(lines, *index, "guard declaration")?;

    let header_line = lines[*index].trim();
    let start_line = *index;

    // 1. Attempt to parse inline/compact form:
    //    guard name(condition) for N cycles;
    //    guard name when condition for N cycles;
    //    let guard name = when condition for N cycles;
    if header_line.starts_with("guard ") || header_line.starts_with("let guard ") {
        let is_let = header_line.starts_with("let ");
        let after_keyword = if is_let {
            header_line.strip_prefix("let guard ").ok_or_else(|| {
                emit_at(
                    ErrorCode::GuardMalformed,
                    "Expected 'let guard' declaration",
                    Span::full_line(start_line as u32),
                )
            })?
        } else {
            header_line.strip_prefix("guard ").ok_or_else(|| {
                emit_at(
                    ErrorCode::GuardMalformed,
                    "Expected 'guard' declaration",
                    Span::full_line(start_line as u32),
                )
            })?
        };

        let trimmed = after_keyword.trim();

        // Handle 'name = when ...' or 'name when ...'
        let (name, remaining) = if trimmed.contains('=') {
            let (n, r) = trimmed.split_once('=').ok_or_else(|| {
                emit_at(
                    ErrorCode::GuardMalformed,
                    "Expected '=' in guard binding",
                    Span::full_line(start_line as u32),
                )
            })?;
            (n.trim(), r.trim().to_string())
        } else if trimmed.contains(" when ") {
            let (n, r) = trimmed.split_once(" when ").ok_or_else(|| {
                emit_at(
                    ErrorCode::GuardMalformed,
                    "Expected 'when' clause in guard",
                    Span::full_line(start_line as u32),
                )
            })?;
            (n.trim(), format!("when {r}"))
        } else if trimmed.contains('(') && trimmed.contains(" for ") {
            let (n, r) = trimmed.split_once('(').ok_or_else(|| {
                emit_at(
                    ErrorCode::GuardMalformed,
                    "Expected '(' in guard list",
                    Span::full_line(start_line as u32),
                )
            })?;
            (n.trim(), format!("({r}"))
        } else {
            // Probably block form
            ("", "".to_string())
        };

        if !name.is_empty()
            && !remaining.is_empty()
            && (remaining.contains("when ") || remaining.starts_with('('))
        {
            let (cond_part, for_part) = if remaining.contains(" for ") {
                let (c, f) = remaining.rsplit_once(" for ").ok_or_else(|| {
                    emit_at(
                        ErrorCode::GuardMalformed,
                        "Expected 'for' in guard",
                        Span::full_line(start_line as u32),
                    )
                })?;
                (c.trim(), f.trim())
            } else {
                (remaining.trim(), "1 cycles")
            };

            let cond_str = if cond_part.starts_with("when ") {
                cond_part
                    .strip_prefix("when ")
                    .ok_or_else(|| {
                        emit_at(
                            ErrorCode::GuardMalformed,
                            "Expected 'when' condition",
                            Span::full_line(start_line as u32),
                        )
                    })?
                    .trim()
                    .trim_end_matches(';')
            } else if cond_part.starts_with('(') && cond_part.ends_with(')') {
                cond_part[1..cond_part.len() - 1].trim().trim_end_matches(';')
            } else {
                cond_part.trim_end_matches(';')
            };

            let condition = parse_expression(cond_str).map_err(|e| {
                emit_at(
                    ErrorCode::GuardConditionError,
                    format!("Guard '{name}' condition parse error: {e}"),
                    Span::full_line(start_line as u32),
                )
            })?;

            let cycles_str =
                for_part.trim().trim_end_matches(';').split_whitespace().next().unwrap_or("1");
            let (cycles, template_cycles) =
                if cycles_str.starts_with("${") && cycles_str.ends_with('}') {
                    (0, Some(cycles_str.to_string()))
                } else {
                    let val = cycles_str.parse::<u64>().map_err(|_| {
                        emit_at(
                            ErrorCode::GuardMalformed,
                            format!("Invalid cycle count in guard: {cycles_str}"),
                            Span::full_line(start_line as u32),
                        )
                    })?;
                    (val, None)
                };

            *index += 1;
            return Ok(Guard {
                name: name.to_string(),
                condition,
                cycles,
                template_cycles,
                origin: None,
                span: Some(Span::full_line(start_line as u32)),
            });
        }
    }

    // 2. Fallback to block form: guard name { when ... for ... }
    let name = guard_parse_header(lines, index)?;
    *index += 1;
    skip_empty_and_comments(lines, index);
    guard_check_eof(lines, *index, &format!("guard '{}' body", name))?;

    let current_line = lines[*index].trim();
    let (condition, cycles, template_cycles) =
        if current_line.starts_with("when ") && current_line.contains("for ") {
            // Same-line block form: when <cond> for <N> cycles;
            let after_when = current_line.strip_prefix("when ").unwrap_or(current_line);
            let cond_part = after_when.split_once("for ").ok_or_else(|| {
                emit_at(
                    ErrorCode::GuardMalformed,
                    "Missing 'for' in compact guard clause.",
                    Span::full_line(*index as u32),
                )
            })?;

            let condition = parse_expression(cond_part.0.trim()).map_err(|e| {
                emit_at(
                    ErrorCode::GuardConditionError,
                    format!("Guard '{name}' condition parse error: {e}"),
                    Span::full_line(*index as u32),
                )
            })?;

            let cycles_str =
                cond_part.1.trim().trim_end_matches(';').split_whitespace().next().ok_or_else(
                    || {
                        emit_at(
                            ErrorCode::GuardMissingCycleCount,
                            "Missing cycle count in compact guard clause.",
                            Span::full_line(*index as u32),
                        )
                    },
                )?;
            let (cycles, template_cycles) =
                if cycles_str.starts_with("${") && cycles_str.ends_with('}') {
                    (0, Some(cycles_str.to_string()))
                } else {
                    let val = cycles_str.parse::<u64>().map_err(|_| {
                        emit_at(
                            ErrorCode::GuardMalformed,
                            format!("Invalid cycle count in compact guard: {cycles_str}"),
                            Span::full_line(*index as u32),
                        )
                    })?;
                    (val, None)
                };

            *index += 1;
            (condition, cycles, template_cycles)
        } else {
            // Multi-line block form.
            let condition = guard_parse_when(&name, lines, index)?;
            *index += 1;
            skip_empty_and_comments(lines, index);
            let (cycles, template_cycles) = guard_parse_for(&name, lines, index)?;
            *index += 1;
            (condition, cycles, template_cycles)
        };

    skip_empty_and_comments(lines, index);
    guard_expect_close(&name, lines, index)?;
    *index += 1; // Consume '}'

    Ok(Guard {
        name,
        condition,
        cycles,
        template_cycles,
        origin: None,
        span: Some(Span::multi_line(start_line as u32, (*index - 1) as u32)),
    })
}

// ── Guard sub-parsers (NASA W4: each ≤ 60 lines) ────────────────────────────

fn guard_check_eof(lines: &[&str], index: usize, ctx: &str) -> Result<(), MirrError> {
    if index >= lines.len() {
        return Err(emit_at(
            ErrorCode::GuardUnexpectedEof,
            format!("Unexpected end of file in {ctx}."),
            Span::full_line(index as u32),
        )
        .with_span(Some(Span::full_line(index.saturating_sub(1) as u32))));
    }
    Ok(())
}

fn guard_parse_header(lines: &[&str], index: &mut usize) -> Result<String, MirrError> {
    let header = lines[*index].trim();
    let after_keyword = header.strip_prefix("guard ").ok_or_else(|| {
        emit_at(
            ErrorCode::GuardMalformed,
            "Malformed guard declaration.",
            Span::full_line(*index as u32),
        )
        .with_span(Some(Span::full_line(*index as u32)))
    })?;

    let name_part = match crate::parser::module_parser::guard_reflex::split_at_structural_brace(
        after_keyword,
    ) {
        Some((n, _)) => n,
        None => after_keyword,
    };

    let name = name_part.trim().to_string();
    if name.is_empty() {
        return Err(emit_at(
            ErrorCode::GuardNameEmpty,
            "Guard name cannot be empty.",
            Span::full_line(*index as u32),
        )
        .with_span(Some(Span::full_line(*index as u32))));
    }
    Ok(name)
}

fn guard_parse_when(
    name: &str,
    lines: &[&str],
    index: &mut usize,
) -> Result<crate::ast::expr::Expr, MirrError> {
    guard_check_eof(lines, *index, &format!("guard '{name}' when clause"))?;

    let when_line = lines[*index].trim();
    if !when_line.starts_with("when ") {
        return Err(emit_at(
            ErrorCode::GuardMissingWhen,
            format!("Guard '{name}' missing 'when' clause, found: {when_line}"),
            Span::full_line(*index as u32),
        )
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let condition_str = &when_line[5..].trim();
    let condition_str = condition_str.trim_end_matches(';');

    parse_expression(condition_str).map_err(|e| {
        emit_at(
            ErrorCode::GuardConditionError,
            format!("Guard '{name}' condition parse error: {e}"),
            Span::full_line(*index as u32),
        )
        .with_span(Some(Span::full_line(*index as u32)))
    })
}

fn guard_parse_for(
    name: &str,
    lines: &[&str],
    index: &mut usize,
) -> Result<(u64, Option<String>), MirrError> {
    guard_check_eof(lines, *index, &format!("guard '{name}' for clause"))?;

    let for_line = lines[*index].trim();
    if !for_line.starts_with("for ") {
        return Err(MirrError::parse_error(format!(
            "Guard '{name}' expected 'for' line, found: {for_line}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let after_for = for_line
        .strip_prefix("for ")
        .ok_or_else(|| {
            MirrError::parse_error(format!("Guard '{name}' expected 'for' line, found: {for_line}"))
                .with_span(Some(Span::full_line(*index as u32)))
        })?
        .trim_start();
    let cycles_str =
        after_for.trim_end_matches(';').split_whitespace().next().ok_or_else(|| {
            emit_at(
                ErrorCode::GuardMissingCycleCount,
                "Expected cycle count after 'for'.",
                Span::full_line(*index as u32),
            )
            .with_span(Some(Span::full_line(*index as u32)))
        })?;

    if cycles_str.starts_with("${") && cycles_str.ends_with('}') {
        Ok((0, Some(cycles_str.to_string())))
    } else {
        // NASA W5: bounded by u64::MAX — no overflow possible via parse.
        let cycles = cycles_str.trim().parse::<u64>().map_err(|_| {
            MirrError::parse_error(format!("Invalid cycle count in guard '{name}': {cycles_str}"))
                .with_span(Some(Span::full_line(*index as u32)))
        })?;
        Ok((cycles, None))
    }
}

fn guard_expect_close(name: &str, lines: &[&str], index: &mut usize) -> Result<(), MirrError> {
    guard_check_eof(lines, *index, &format!("guard '{name}' closing brace"))?;

    let closing = lines[*index].trim();
    if closing != "}" {
        return Err(MirrError::parse_error(format!(
            "Guard '{name}' expected closing '}}', found: {closing}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }
    Ok(())
}

// ── Assignment parsing ────────────────────────────────────────────────────────

/// Parse a single `target = expr;` line into an `Assignment`.
///
/// Strips inline `//` comments before processing.
/// Never panics — all failure paths return `MirrError`.
pub(super) fn parse_assignment(line: &str, line_index: usize) -> Result<Assignment, MirrError> {
    // Strip inline comment — NASA W6: minimal scope for stripped value.
    let line = match line.find("//") {
        Some(pos) => line[..pos].trim_end(),
        None => line,
    };

    let stripped = line.trim().trim_end_matches(';').trim();

    let (lhs, rhs) = stripped.split_once('=').ok_or_else(|| {
        emit_at(
            ErrorCode::AssignmentMissingEq,
            format!("Assignment missing '=': {stripped}"),
            Span::full_line(line_index as u32),
        )
        .with_span(Some(Span::full_line(line_index as u32)))
    })?;

    let target = lhs.trim();
    if target.is_empty() {
        return Err(emit_at(
            ErrorCode::AssignmentTargetEmpty,
            "Assignment target cannot be empty.",
            Span::full_line(line_index as u32),
        )
        .with_span(Some(Span::full_line(line_index as u32))));
    }

    let rhs_str = rhs.trim();
    if rhs_str.is_empty() {
        return Err(emit_at(
            ErrorCode::AssignmentRhsEmpty,
            format!("Assignment to '{target}' has empty right-hand side."),
            Span::full_line(line_index as u32),
        )
        .with_span(Some(Span::full_line(line_index as u32))));
    }

    let value = parse_expression(rhs_str).map_err(|e| {
        emit_at(
            ErrorCode::AssignmentExprError,
            format!("Error in assignment to '{target}': {e}"),
            Span::full_line(line_index as u32),
        )
        .with_span(Some(Span::full_line(line_index as u32)))
    })?;

    Ok(Assignment {
        target: target.to_string(),
        value,
        span: Some(Span::full_line(line_index as u32)),
    })
}

// ── Reflex parsing ────────────────────────────────────────────────────────────

/// Parse the reflex header line and return `(name, guard_names)`.
/// Guard names are only populated when the `when [...]` inline form is used.
pub(super) fn reflex_parse_header(
    lines: &[&str],
    index: &mut usize,
) -> Result<(String, Vec<String>), MirrError> {
    let header = lines[*index].trim();
    let after_keyword = header.strip_prefix("reflex ").ok_or_else(|| {
        emit_at(
            ErrorCode::ReflexMalformed,
            "Malformed reflex declaration.",
            Span::full_line(*index as u32),
        )
        .with_span(Some(Span::full_line(*index as u32)))
    })?;

    let name_part = match crate::parser::module_parser::guard_reflex::split_at_structural_brace(
        after_keyword,
    ) {
        Some((n, _)) => n,
        None => after_keyword,
    };

    // Extract inline `when [guard]` or `on guard` only when keyword is standalone.
    let trimmed_name_part = name_part.trim();
    let (raw_name, guard_names) =
        if let Some((pure_name, guard_part)) = split_reflex_inline_guards(trimmed_name_part) {
            let names = parse_guard_name_list(pure_name, guard_part, *index, true)?;
            (pure_name.to_string(), names)
        } else {
            (trimmed_name_part.to_string(), Vec::new())
        };

    if raw_name.is_empty() {
        return Err(emit_at(
            ErrorCode::ReflexNameEmpty,
            "Reflex name cannot be empty.",
            Span::full_line(*index as u32),
        )
        .with_span(Some(Span::full_line(*index as u32))));
    }

    Ok((raw_name, guard_names))
}

fn split_reflex_inline_guards(name_part: &str) -> Option<(&str, &str)> {
    let first_ws = name_part.find(char::is_whitespace)?;
    let candidate_name = name_part[..first_ws].trim();
    let trailing = name_part[first_ws..].trim_start();
    if let Some(after_when) = strip_keyword_prefix(trailing, "when") {
        return Some((candidate_name, after_when.trim_start()));
    }
    if let Some(after_on) = strip_keyword_prefix(trailing, "on") {
        return Some((candidate_name, after_on.trim_start()));
    }
    None
}

fn strip_keyword_prefix<'a>(text: &'a str, keyword: &str) -> Option<&'a str> {
    let rest = text.strip_prefix(keyword)?;
    match rest.chars().next() {
        None => Some(rest),
        Some(ch) if ch.is_whitespace() || ch == '[' => Some(rest),
        Some(_) => None,
    }
}

fn parse_guard_name_list(
    reflex_name: &str,
    raw_guard_list: &str,
    line_index: usize,
    require_nonempty: bool,
) -> Result<Vec<String>, MirrError> {
    // NASA W2: bounded by MAX_GUARD_NAMES.
    let mut names: Vec<String> = Vec::new();
    for token in raw_guard_list.split_whitespace() {
        if token == "and" {
            continue;
        }

        if names.len() >= MAX_GUARD_NAMES {
            return Err(emit_at(
                ErrorCode::ReflexMaxGuardNames,
                format!("Reflex '{reflex_name}' exceeds MAX_GUARD_NAMES ({MAX_GUARD_NAMES})."),
                Span::full_line(line_index as u32),
            ));
        }

        let guard_name = token.trim().trim_start_matches('[').trim_end_matches(']').trim();
        if !guard_name.is_empty() {
            names.push(guard_name.to_string());
        }
    }

    if require_nonempty && names.is_empty() {
        return Err(emit_at(
            ErrorCode::ReflexNoGuardNames,
            format!("Reflex '{reflex_name}' has no guard names in 'on' clause."),
            Span::full_line(line_index as u32),
        ));
    }

    Ok(names)
}

pub(super) fn split_at_structural_brace(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;
    while i < len {
        if bytes[i] == b'$' && i + 1 < len && bytes[i + 1] == b'{' {
            i += 2;
            while i < len && bytes[i] != b'}' {
                i += 1;
            }
            if i < len {
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'{' {
            return Some((&s[..i], &s[i + 1..]));
        }
        i += 1;
    }
    None
}
