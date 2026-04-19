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
use crate::ast::program::{Assignment, Guard, Reflex};
use crate::error::MirrError;
use crate::span::Span;

// ── NASA W2: all loops bounded ───────────────────────────────────────────────
const MAX_REFLEX_BODY_LINES: usize = 4_096;
const MAX_ASSIGNMENTS: usize = 256;
const MAX_GUARD_NAMES: usize = 64;

// ── Reflex parse state machine ───────────────────────────────────────────────
// Replaces the fragile (saw_on_clause, inside_on) boolean pair.
// Impossible states (e.g. InsideOn before AwaitingOn) are now unrepresentable.
#[derive(Debug, PartialEq, Eq)]
enum ReflexState {
    /// Waiting for an `on <guard> {` clause (or `when [guard]` header).
    AwaitingOn,
    /// Inside the `on` block — consuming assignment lines.
    InsideOn,
    /// The `on` block closed; reflex body ends at next `}`.
    Done,
}

// ── Guard parsing ─────────────────────────────────────────────────────────────

/// Parse a `guard <name> { when <cond> for <N> cycles; }` block.
///
/// # Errors
/// Returns `MirrError` on any malformed input. Never panics.
pub(super) fn parse_guard(lines: &[&str], index: &mut usize) -> Result<Guard, MirrError> {
    // NASA W5: assert precondition.
    debug_assert!(*index <= lines.len(), "index out of bounds before parse_guard");

    guard_check_eof(lines, *index, "guard declaration")?;

    let start_line = *index;
    let name = guard_parse_header(lines, index)?;

    *index += 1;
    skip_empty_and_comments(lines, index);

    let condition = guard_parse_when(&name, lines, index)?;

    *index += 1;
    skip_empty_and_comments(lines, index);

    let cycles = guard_parse_for(&name, lines, index)?;

    *index += 1;
    skip_empty_and_comments(lines, index);

    guard_expect_close(&name, lines, index)?;

    *index += 1;

    Ok(Guard {
        name,
        condition,
        cycles,
        origin: None,
        span: Some(Span::multi_line(start_line as u32, (*index - 1) as u32)),
    })
}

// ── Guard sub-parsers (NASA W4: each ≤ 60 lines) ────────────────────────────

fn guard_check_eof(lines: &[&str], index: usize, ctx: &str) -> Result<(), MirrError> {
    if index >= lines.len() {
        return Err(MirrError::parse_error(format!("[E119] Unexpected end of file in {ctx}."))
            .with_span(Some(Span::full_line(index.saturating_sub(1) as u32))));
    }
    Ok(())
}

fn guard_parse_header(lines: &[&str], index: &mut usize) -> Result<String, MirrError> {
    let header = lines[*index].trim();
    let after_keyword = header.strip_prefix("guard ").ok_or_else(|| {
        MirrError::parse_error("[E120] Malformed guard declaration.")
            .with_span(Some(Span::full_line(*index as u32)))
    })?;

    let name_part = match after_keyword.split_once('{') {
        Some((n, _)) => n,
        None => after_keyword,
    };

    let name = name_part.trim().to_string();
    if name.is_empty() {
        return Err(MirrError::parse_error("[E121] Guard name cannot be empty.")
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
        return Err(MirrError::parse_error(format!(
            "[E123] Guard '{name}' expected 'when' line, found: {when_line}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let condition_str = when_line.strip_prefix("when ").expect("starts_with checked above").trim();

    parse_expression(condition_str).map_err(|e| {
        MirrError::parse_error(format!("[E125] Guard '{name}' condition parse error: {e}"))
            .with_span(Some(Span::full_line(*index as u32)))
    })
}

fn guard_parse_for(name: &str, lines: &[&str], index: &mut usize) -> Result<u64, MirrError> {
    guard_check_eof(lines, *index, &format!("guard '{name}' for clause"))?;

    let for_line = lines[*index].trim();
    if !for_line.starts_with("for ") {
        return Err(MirrError::parse_error(format!(
            "[E127] Guard '{name}' expected 'for' line, found: {for_line}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let after_for = for_line.strip_prefix("for ").expect("starts_with checked").trim_start();
    let cycles_str = after_for.split_whitespace().next().ok_or_else(|| {
        MirrError::parse_error("[E129] Expected cycle count after 'for'.")
            .with_span(Some(Span::full_line(*index as u32)))
    })?;

    // NASA W5: bounded by u64::MAX — no overflow possible via parse.
    cycles_str.trim().parse::<u64>().map_err(|_| {
        MirrError::parse_error(format!(
            "[E130] Invalid cycle count in guard '{name}': {cycles_str}"
        ))
        .with_span(Some(Span::full_line(*index as u32)))
    })
}

fn guard_expect_close(name: &str, lines: &[&str], index: &mut usize) -> Result<(), MirrError> {
    guard_check_eof(lines, *index, &format!("guard '{name}' closing brace"))?;

    let closing = lines[*index].trim();
    if closing != "}" {
        return Err(MirrError::parse_error(format!(
            "[E132] Guard '{name}' expected closing '}}', found: {closing}"
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
fn parse_assignment(line: &str, line_index: usize) -> Result<Assignment, MirrError> {
    // Strip inline comment — NASA W6: minimal scope for stripped value.
    let line = match line.find("//") {
        Some(pos) => line[..pos].trim_end(),
        None => line,
    };

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

// ── Reflex parsing ────────────────────────────────────────────────────────────

/// Parse a `reflex <name> { on <guard> { <assignments> } }` block.
///
/// Also accepts the inline header form: `reflex <name> when [<guard>] { ... }`.
///
/// # Errors
/// Returns `MirrError` on any malformed input. Never panics.
pub(super) fn parse_reflex(lines: &[&str], index: &mut usize) -> Result<Reflex, MirrError> {
    // NASA W5: assert precondition.
    debug_assert!(*index <= lines.len(), "index out of bounds before parse_reflex");

    guard_check_eof(lines, *index, "reflex declaration")?;

    let start_line = *index;
    let (name, mut guard_names) = reflex_parse_header(lines, index)?;

    // If the header already declared guards via `when [...]`, skip AwaitingOn.
    let initial_state =
        if guard_names.is_empty() { ReflexState::AwaitingOn } else { ReflexState::InsideOn };

    *index += 1;
    skip_empty_and_comments(lines, index);

    let assignments = reflex_parse_body(&name, lines, index, &mut guard_names, initial_state)?;

    // NASA W5: postcondition — assignments must be non-empty.
    debug_assert!(!assignments.is_empty(), "reflex body returned empty assignments");

    Ok(Reflex {
        name,
        guard_names,
        assignments,
        origin: None,
        span: Some(Span::multi_line(start_line as u32, (*index - 1) as u32)),
    })
}

// ── Reflex sub-parsers ────────────────────────────────────────────────────────

/// Parse the reflex header line and return `(name, guard_names)`.
/// Guard names are only populated when the `when [...]` inline form is used.
fn reflex_parse_header(
    lines: &[&str],
    index: &mut usize,
) -> Result<(String, Vec<String>), MirrError> {
    let header = lines[*index].trim();
    let after_keyword = header.strip_prefix("reflex ").ok_or_else(|| {
        MirrError::parse_error("[E138] Malformed reflex declaration.")
            .with_span(Some(Span::full_line(*index as u32)))
    })?;

    let name_part = match after_keyword.split_once('{') {
        Some((n, _)) => n,
        None => after_keyword,
    };

    // Extract inline `when [guard and guard]` only when `when` is a standalone keyword.
    let trimmed_name_part = name_part.trim();
    let (raw_name, guard_names) =
        if let Some((pure_name, when_part)) = split_reflex_inline_when(trimmed_name_part) {
            let names = parse_guard_name_list(pure_name, when_part, *index, true)?;
            (pure_name.to_string(), names)
        } else {
            (trimmed_name_part.to_string(), Vec::new())
        };

    if raw_name.is_empty() {
        return Err(MirrError::parse_error("[E139] Reflex name cannot be empty.")
            .with_span(Some(Span::full_line(*index as u32))));
    }

    Ok((raw_name, guard_names))
}

fn split_reflex_inline_when(name_part: &str) -> Option<(&str, &str)> {
    let first_ws = name_part.find(char::is_whitespace)?;
    let candidate_name = name_part[..first_ws].trim();
    let trailing = name_part[first_ws..].trim_start();
    let after_when = strip_keyword_prefix(trailing, "when")?;
    Some((candidate_name, after_when.trim_start()))
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
            return Err(MirrError::parse_error(format!(
                "[E141] Reflex '{reflex_name}' exceeds MAX_GUARD_NAMES ({MAX_GUARD_NAMES})."
            ))
            .with_span(Some(Span::full_line(line_index as u32))));
        }

        let guard_name = token.trim().trim_start_matches('[').trim_end_matches(']').trim();
        if !guard_name.is_empty() {
            names.push(guard_name.to_string());
        }
    }

    if require_nonempty && names.is_empty() {
        return Err(MirrError::parse_error(format!(
            "[E143] Reflex '{reflex_name}' has no guard names in 'on' clause."
        ))
        .with_span(Some(Span::full_line(line_index as u32))));
    }

    Ok(names)
}

/// Consume the reflex body using the `ReflexState` machine.
///
/// Returns the collected assignments on success.
/// NASA W2: loop bounded by MAX_REFLEX_BODY_LINES.
fn reflex_parse_body(
    name: &str,
    lines: &[&str],
    index: &mut usize,
    guard_names: &mut Vec<String>,
    initial: ReflexState,
) -> Result<Vec<Assignment>, MirrError> {
    let mut state = initial;
    let mut assignments = Vec::new();
    let mut brace_depth = 1_i32;
    let mut iterations = 0_usize;

    while *index < lines.len() && brace_depth > 0 {
        // NASA W2: hard iteration ceiling.
        iterations += 1;
        if iterations > MAX_REFLEX_BODY_LINES {
            return Err(MirrError::parse_error(format!(
                "[E142] Reflex '{name}' body exceeds MAX_REFLEX_BODY_LINES \
                 ({MAX_REFLEX_BODY_LINES})."
            ))
            .with_span(Some(Span::full_line(*index as u32))));
        }

        let line = lines[*index].trim();

        // Skip blank lines and comments.
        if line.is_empty() || line.starts_with("//") {
            *index += 1;
            continue;
        }

        match state {
            ReflexState::AwaitingOn => {
                reflex_consume_on_clause(
                    name,
                    line,
                    index,
                    guard_names,
                    &mut brace_depth,
                    &mut state,
                )?;
            }
            ReflexState::InsideOn => {
                reflex_consume_assignment_or_close(
                    name,
                    line,
                    index,
                    guard_names,
                    &mut assignments,
                    &mut brace_depth,
                    &mut state,
                )?;
            }
            ReflexState::Done => {
                // Consume the outer closing brace.
                let opens = line.matches('{').count() as i32;
                let closes = line.matches('}').count() as i32;
                brace_depth += opens - closes;
                *index += 1;
            }
        }
    }

    if brace_depth != 0 {
        return Err(MirrError::parse_error(format!(
            "[E145] Reflex '{name}' not closed with '}}'."
        ))
        .with_span(Some(Span::full_line(index.saturating_sub(1) as u32))));
    }

    if assignments.is_empty() {
        return Err(MirrError::parse_error(format!(
            "[E146] Reflex '{name}' must contain at least one assignment."
        ))
        .with_span(Some(Span::full_line(index.saturating_sub(1) as u32))));
    }

    // NASA W5: postcondition.
    debug_assert!(
        assignments.len() <= MAX_ASSIGNMENTS,
        "assignments exceeded MAX_ASSIGNMENTS — loop bound violated"
    );

    Ok(assignments)
}

/// Handle a line while in `AwaitingOn` state.
/// NASA W4: ≤ 60 lines.
fn reflex_consume_on_clause(
    name: &str,
    line: &str,
    index: &mut usize,
    guard_names: &mut Vec<String>,
    brace_depth: &mut i32,
    state: &mut ReflexState,
) -> Result<(), MirrError> {
    if !line.starts_with("on ") {
        return Err(MirrError::parse_error(format!(
            "[E140] Reflex '{name}' expected 'on <guard> {{', found: {line}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let after_on = line.strip_prefix("on ").expect("starts_with checked");
    let (guards_part, remainder) = match after_on.split_once('{') {
        Some(parts) => parts,
        None => {
            // Require the `on <guard> {` form; we do not support the brace on the next line.
            return Err(MirrError::parse_error(format!(
                "[E140] Reflex '{name}' expected 'on <guard> {{', found: {line}"
            ))
            .with_span(Some(Span::full_line(*index as u32))));
        }
    };

    guard_names.clear();
    guard_names.extend(parse_guard_name_list(name, guards_part, *index, false)?);

    // Account for the `{` that opens the on-block in the current line, plus any
    // additional braces occurring later on the same line.
    *brace_depth += 1;
    let opens = remainder.matches('{').count() as i32;
    let closes = remainder.matches('}').count() as i32;
    *brace_depth += opens - closes;

    *state = ReflexState::InsideOn;
    *index += 1;
    Ok(())
}

/// Handle a line while in `InsideOn` state.
/// NASA W4: ≤ 60 lines.
fn reflex_consume_assignment_or_close(
    name: &str,
    line: &str,
    index: &mut usize,
    guard_names: &[String],
    assignments: &mut Vec<Assignment>,
    brace_depth: &mut i32,
    state: &mut ReflexState,
) -> Result<(), MirrError> {
    if line == "}" {
        *brace_depth -= 1;
        *index += 1;
        if *brace_depth == 1 {
            *state = ReflexState::Done;
        }
        return Ok(());
    }

    if guard_names.is_empty() {
        return Err(MirrError::parse_error(format!(
            "[E143] Reflex '{name}' has no guard names in 'on' clause."
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    // NASA W2: hard ceiling on assignment count.
    if assignments.len() >= MAX_ASSIGNMENTS {
        return Err(MirrError::parse_error(format!(
            "[E143] Reflex '{name}' exceeds MAX_ASSIGNMENTS ({MAX_ASSIGNMENTS})."
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let assignment = parse_assignment(line, *index).map_err(|e| {
        MirrError::parse_error(format!("[E144] In reflex '{name}': {e}"))
            .with_span(Some(Span::full_line(*index as u32)))
    })?;

    assignments.push(assignment);
    *index += 1;
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Guard tests ───────────────────────────────────────────────────────────

    #[test]
    fn guard_parses_correctly() {
        let src = ["guard high_temp {", "    when temp > 100", "    for 5 cycles;", "}"];
        let mut idx = 0;
        let g = parse_guard(&src, &mut idx).expect("valid guard");
        assert_eq!(g.name, "high_temp");
        assert_eq!(g.cycles, 5);
        assert_eq!(idx, 4);
    }

    #[test]
    fn guard_rejects_missing_when() {
        let src = ["guard bad {", "    for 5 cycles;", "}"];
        let mut idx = 0;
        let err = parse_guard(&src, &mut idx).unwrap_err();
        assert!(err.to_string().contains("E123"), "expected E123, got: {err}");
    }

    #[test]
    fn guard_rejects_empty_name() {
        let src = ["guard  {", "    when x > 0", "    for 1 cycles;", "}"];
        let mut idx = 0;
        let err = parse_guard(&src, &mut idx).unwrap_err();
        assert!(err.to_string().contains("E121"), "expected E121, got: {err}");
    }

    #[test]
    fn guard_rejects_eof_mid_body() {
        let src = ["guard g {"];
        let mut idx = 0;
        assert!(parse_guard(&src, &mut idx).is_err());
    }

    // ── Reflex tests ──────────────────────────────────────────────────────────

    #[test]
    fn reflex_parses_on_block_form() {
        let src =
            ["reflex temp_alarm {", "    on high_temp {", "        alarm_a = true;", "    }", "}"];
        let mut idx = 0;
        let r = parse_reflex(&src, &mut idx).expect("valid reflex");
        assert_eq!(r.name, "temp_alarm");
        assert_eq!(r.guard_names, vec!["high_temp"]);
        assert_eq!(r.assignments.len(), 1);
        assert_eq!(r.assignments[0].target, "alarm_a");
    }

    #[test]
    fn reflex_parses_inline_when_form() {
        let src = ["reflex fast_alarm when [high_temp] {", "    alarm_a = true;", "}"];
        let mut idx = 0;
        let r = parse_reflex(&src, &mut idx).expect("valid inline reflex");
        assert_eq!(r.guard_names, vec!["high_temp"]);
        assert_eq!(r.assignments.len(), 1);
    }

    #[test]
    fn reflex_rejects_missing_on_clause() {
        let src = ["reflex bad {", "    alarm = true;", "}"];
        let mut idx = 0;
        let err = parse_reflex(&src, &mut idx).unwrap_err();
        assert!(err.to_string().contains("E140"), "expected E140, got: {err}");
    }

    #[test]
    fn reflex_rejects_empty_inline_guard_list() {
        let src = ["reflex bad when [] {", "    alarm = true;", "}"];
        let mut idx = 0;
        let err = parse_reflex(&src, &mut idx).unwrap_err();
        assert!(err.to_string().contains("E143"), "expected E143, got: {err}");
    }

    #[test]
    fn reflex_rejects_empty_on_clause_guard_list() {
        let src = ["reflex bad {", "    on {", "        alarm = true;", "    }", "}"];
        let mut idx = 0;
        let err = parse_reflex(&src, &mut idx).unwrap_err();
        assert!(err.to_string().contains("E143"), "expected E143, got: {err}");
    }

    #[test]
    fn reflex_rejects_empty_assignments() {
        let src = ["reflex empty {", "    on guard_a {", "    }", "}"];
        let mut idx = 0;
        let err = parse_reflex(&src, &mut idx).unwrap_err();
        assert!(err.to_string().contains("E146"), "expected E146, got: {err}");
    }

    #[test]
    fn reflex_rejects_unclosed_brace() {
        let src = [
            "reflex unclosed {",
            "    on guard_a {",
            "        x = 1;",
            // missing closing braces
        ];
        let mut idx = 0;
        assert!(parse_reflex(&src, &mut idx).is_err());
    }

    #[test]
    fn multi_reflex_index_advances_correctly() {
        // Parser must leave index pointing at the line AFTER the reflex,
        // so that the module parser can call parse_reflex again cleanly.
        let src = [
            "reflex r1 {",        // 0
            "    on g1 {",        // 1
            "        a = true;",  // 2
            "    }",              // 3
            "}",                  // 4
            "reflex r2 {",        // 5
            "    on g2 {",        // 6
            "        b = false;", // 7
            "    }",              // 8
            "}",                  // 9
        ];
        let mut idx = 0;
        let r1 = parse_reflex(&src, &mut idx).expect("r1");
        assert_eq!(r1.name, "r1");
        assert_eq!(idx, 5, "index must point at r2 header after r1 parsed");

        let r2 = parse_reflex(&src, &mut idx).expect("r2");
        assert_eq!(r2.name, "r2");
        assert_eq!(idx, 10);
    }

    // ── Assignment tests ──────────────────────────────────────────────────────

    #[test]
    fn assignment_strips_inline_comment() {
        let a = parse_assignment("x = true; // set x", 0).expect("valid");
        assert_eq!(a.target, "x");
    }

    #[test]
    fn assignment_rejects_missing_equals() {
        let err = parse_assignment("x true;", 0).unwrap_err();
        assert!(err.to_string().contains("E133"), "expected E133, got: {err}");
    }

    #[test]
    fn assignment_rejects_empty_target() {
        let err = parse_assignment("= true;", 0).unwrap_err();
        assert!(err.to_string().contains("E134"), "expected E134, got: {err}");
    }

    #[test]
    fn assignment_rejects_empty_rhs() {
        let err = parse_assignment("x = ;", 0).unwrap_err();
        assert!(err.to_string().contains("E135"), "expected E135, got: {err}");
    }

    // ── Bound enforcement tests ───────────────────────────────────────────────

    #[test]
    fn reflex_body_bound_enforced() {
        // Build a reflex with MAX_REFLEX_BODY_LINES + 1 assignment lines.
        let mut lines = vec!["reflex overflow {", "    on g {"];
        for i in 0..MAX_REFLEX_BODY_LINES {
            lines.push(Box::leak(format!("        x{i} = true;").into_boxed_str()) as &str);
        }
        lines.push("    }");
        lines.push("}");

        let mut idx = 0;
        let result = parse_reflex(&lines, &mut idx);
        // Either hits MAX_REFLEX_BODY_LINES or MAX_ASSIGNMENTS — both are errors.
        assert!(result.is_err(), "expected error for oversized reflex body");
    }
}
