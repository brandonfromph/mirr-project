//! Property formula parsing: always, never, eventually, implication, followed-by.

#![forbid(unsafe_code)]

use super::parse_expression;
use super::skip_empty_and_comments;
use crate::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use crate::diagnostic_builder::emit_at;
use crate::error::MirrError;
use crate::error_codes::ErrorCode;
use crate::span::Span;

pub(super) fn parse_property(lines: &[&str], index: &mut usize) -> Result<PropertyDecl, MirrError> {
    if *index >= lines.len() {
        return Err(emit_at(
            ErrorCode::PropertyUnexpectedEof,
            "Unexpected end of file in property declaration.",
            Span::full_line((*index).saturating_sub(1) as u32),
        ));
    }

    let start_line = *index;
    let header = lines[*index].trim();
    let after_keyword = if let Some(stripped) = header.strip_prefix("property ") {
        stripped
    } else {
        header.strip_prefix("assert ").ok_or_else(|| {
            emit_at(
                ErrorCode::PropertyMalformed,
                "Malformed property declaration.",
                Span::full_line(*index as u32),
            )
        })?
    };

    let (name_part, has_block) = if let Some(pos) = find_structural_brace(after_keyword) {
        (&after_keyword[..pos], true)
    } else {
        (after_keyword, false)
    };

    let (name, inline_formula) = match name_part.split_once(':') {
        Some((n, f)) => (n.trim(), f.trim()),
        _ => (name_part.trim(), ""),
    };

    if name.is_empty() {
        return Err(emit_at(
            ErrorCode::PropertyNameEmpty,
            "Property name cannot be empty.",
            Span::full_line(*index as u32),
        ));
    }

    let (directive, formula) = if !inline_formula.is_empty() {
        parse_property_formula(inline_formula, name)?
    } else if has_block {
        *index += 1;
        skip_empty_and_comments(lines, index);

        if *index >= lines.len() {
            return Err(emit_at(
                ErrorCode::PropertyMissingFormula,
                format!("Property '{name}' missing formula (always/never)."),
                Span::full_line((*index).saturating_sub(1) as u32),
            ));
        }

        let formula_line = lines[*index].trim();
        parse_property_formula(formula_line, name)?
    } else {
        return Err(emit_at(
            ErrorCode::PropertyMalformed,
            format!("Property '{name}' expected formula after ':' or a '{{' block."),
            Span::full_line(*index as u32),
        ));
    };

    if has_block {
        *index += 1;
        skip_empty_and_comments(lines, index);

        if *index >= lines.len() {
            return Err(emit_at(
                ErrorCode::PropertyNotClosed,
                format!("Property '{name}' not closed with '}}'."),
                Span::full_line((*index).saturating_sub(1) as u32),
            ));
        }

        let closing = lines[*index].trim();
        if closing != "}" {
            return Err(emit_at(
                ErrorCode::PropertyExpectedClose,
                format!("Property '{name}' expected closing '}}', found: {closing}"),
                Span::full_line(*index as u32),
            ));
        }
        *index += 1;
    } else {
        // Compact form: assert name: formula;
        // The formula parser might have consumed a semicolon.
        // We just need to move to the next line.
        *index += 1;
    }

    Ok(PropertyDecl {
        name: name.to_string(),
        directive,
        formula,
        origin: None,
        span: Some(Span::multi_line(start_line as u32, (*index - 1) as u32)),
    })
}

fn find_structural_brace(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() {
        if bytes[pos] == b'$' && pos + 1 < bytes.len() && bytes[pos + 1] == b'{' {
            pos += 2;
            while pos < bytes.len() && bytes[pos] != b'}' {
                pos += 1;
            }
            pos += 1;
            continue;
        }
        if bytes[pos] == b'{' {
            return Some(pos);
        }
        pos += 1;
    }
    None
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

    Err(emit_at(
        ErrorCode::PropertyBadKeyword,
        format!("Property '{name}' formula must start with 'always', 'never', or 'eventually'."),
        Span::full_line(0),
    ))
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
    if let Some((lhs, rhs)) = inner.split_once(" -> ").or_else(|| inner.split_once(" implies ")) {
        let antecedent = parse_expression(lhs.trim()).map_err(|e| {
            emit_at(
                ErrorCode::PropertyAntecedentError,
                format!("Property '{name}' antecedent error: {e}"),
                Span::full_line(0),
            )
        })?;
        let consequent = parse_expression(rhs.trim()).map_err(|e| {
            emit_at(
                ErrorCode::PropertyConsequentError,
                format!("Property '{name}' consequent error: {e}"),
                Span::full_line(0),
            )
        })?;
        return Ok(PropertyFormula::AlwaysImplies { antecedent, consequent });
    }

    let expr = parse_expression(inner).map_err(|e| {
        emit_at(
            ErrorCode::PropertyConsequentBad,
            format!("Property '{name}' formula error: {e}"),
            Span::full_line(0),
        )
    })?;
    Ok(PropertyFormula::Always(expr))
}

fn parse_never_body(body: &str, name: &str) -> Result<PropertyFormula, MirrError> {
    let inner = unwrap_parens(body, name, "never")?;

    // Check for never (P -> Q) — NeverImplies
    if let Some((lhs, rhs)) = inner.split_once(" -> ").or_else(|| inner.split_once(" implies ")) {
        let antecedent = parse_expression(lhs.trim()).map_err(|e| {
            emit_at(
                ErrorCode::PropertyAntecedentError,
                format!("Property '{name}' antecedent error: {e}"),
                Span::full_line(0),
            )
        })?;
        let consequent = parse_expression(rhs.trim()).map_err(|e| {
            emit_at(
                ErrorCode::PropertyConsequentError,
                format!("Property '{name}' consequent error: {e}"),
                Span::full_line(0),
            )
        })?;
        return Ok(PropertyFormula::NeverImplies { antecedent, consequent });
    }

    let expr = parse_expression(inner).map_err(|e| {
        emit_at(
            ErrorCode::PropertyConsequentBad,
            format!("Property '{name}' formula error: {e}"),
            Span::full_line(0),
        )
    })?;
    Ok(PropertyFormula::Never(expr))
}

fn parse_eventually_body(body: &str, name: &str) -> Result<PropertyFormula, MirrError> {
    // Expected: "within N (P)"
    let rest = body
        .strip_prefix("within")
        .ok_or_else(|| {
            emit_at(
                ErrorCode::PropertyAntecedentError,
                format!("Property '{name}': expected 'eventually within N (expr)'."),
                Span::full_line(0),
            )
        })?
        .trim();

    // Split off the cycle count before the '('
    let paren_pos = rest.find('(').ok_or_else(|| {
        emit_at(
            ErrorCode::PropertyNeedsParens,
            format!("Property '{name}': eventually within requires parenthesized expression."),
            Span::full_line(0),
        )
    })?;

    let cycles_str = rest[..paren_pos].trim();
    let cycles: u32 = cycles_str.parse().map_err(|_| {
        emit_at(
            ErrorCode::GuardInvalidCycleCount,
            format!("Property '{name}': invalid cycle count '{cycles_str}' in eventually within."),
            Span::full_line(0),
        )
    })?;

    if cycles < 1 {
        return Err(emit_at(
            ErrorCode::ForLoopStepZero,
            format!("Property '{name}': eventually within requires cycles >= 1."),
            Span::full_line(0),
        ));
    }

    let expr_part = &rest[paren_pos..];
    let inner = unwrap_parens(expr_part, name, "eventually within")?;
    let expr = parse_expression(inner).map_err(|e| {
        emit_at(
            ErrorCode::PropertyConsequentBad,
            format!("Property '{name}' formula error: {e}"),
            Span::full_line(0),
        )
    })?;

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
        emit_at(ErrorCode::PropertyConsequentBad, format!("Property '{name}': expected 'P followed_by N Q' with delay and response expression."), Span::full_line(0))
    })?;

    let delay_str = &after_fb[..space_pos];
    let response_str = after_fb[space_pos + 1..].trim();

    let delay_cycles: u32 = delay_str.parse().map_err(|_| {
        emit_at(
            ErrorCode::GuardInvalidCycleCount,
            format!("Property '{name}': invalid delay '{delay_str}' in followed_by."),
            Span::full_line(0),
        )
    })?;

    if delay_cycles < 1 {
        return Err(emit_at(
            ErrorCode::ForLoopStepZero,
            format!("Property '{name}': followed_by requires delay >= 1."),
            Span::full_line(0),
        ));
    }

    let trigger = parse_expression(trigger_str).map_err(|e| {
        emit_at(
            ErrorCode::PropertyAntecedentError,
            format!("Property '{name}' trigger error: {e}"),
            Span::full_line(0),
        )
    })?;
    let response = parse_expression(response_str).map_err(|e| {
        emit_at(
            ErrorCode::PropertyConsequentError,
            format!("Property '{name}' response error: {e}"),
            Span::full_line(0),
        )
    })?;

    Ok(Some(PropertyFormula::AlwaysFollowedBy { trigger, response, delay_cycles }))
}

fn unwrap_parens<'a>(body: &'a str, _name: &str, _keyword: &str) -> Result<&'a str, MirrError> {
    let trimmed = body.trim();
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        return Ok(&trimmed[1..trimmed.len() - 1]);
    }
    // MEGA-10: Relaxed - parentheses are now optional for top-level formulas.
    Ok(trimmed)
}
