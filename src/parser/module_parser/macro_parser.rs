use super::guard_reflex::{parse_assignment, parse_guard, reflex_parse_header};
use super::parse_signal;
use super::skip_empty_and_comments;
use crate::ast::expr::Expr;
use crate::ast::macro_nodes::{MatchArm, ModuleMacroStmt, ReflexMacroStmt, UnexpandedReflex};
use crate::diagnostic_builder::emit_at;
use crate::error::MirrError;
use crate::error_codes::ErrorCode;
use crate::parser::expr_parser::parse_expression;
use crate::parser::pattern_parser::{
    is_pattern_call_start, parse_pattern_call, parse_pattern_call_single,
};
use crate::span::Span;

thread_local! {
    pub(crate) static IN_PATTERN_REFLECT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn parse_module_macro_stmts(
    lines: &[&str],
    index: &mut usize,
) -> Result<Vec<ModuleMacroStmt>, MirrError> {
    let mut stmts = Vec::new();
    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }

        let line = lines[*index].trim();
        if line == "}" || line == "endmodule" || line == "end" {
            break;
        }

        if line.starts_with("for ") {
            let stmt = parse_for_loop(lines, index)?;
            stmts.push(stmt);
            continue;
        }

        if line.starts_with("let ") && line.contains('=') && !line.starts_with("let guard ") {
            let stmt = parse_let_binding(line, *index)?;
            stmts.push(stmt);
            *index += 1;
            continue;
        }

        if line.starts_with("signal ")
            || line.starts_with("in ")
            || line.starts_with("out ")
            || line.starts_with("internal ")
        {
            let sig = parse_signal(line, *index)?;
            stmts.push(ModuleMacroStmt::Signal(sig));
            *index += 1;
            continue;
        }

        if line.starts_with("signals") {
            let next_is_brace = line.ends_with('{')
                || (*index + 1 < lines.len() && lines[*index + 1].trim() == "{");

            if next_is_brace {
                if line.ends_with('{') {
                    *index += 1;
                } else {
                    *index += 2;
                }

                while *index < lines.len() {
                    skip_empty_and_comments(lines, index);
                    if *index < lines.len() && lines[*index].trim() == "}" {
                        *index += 1;
                        break;
                    }
                    let line = lines[*index].trim();
                    if line.starts_with("for ") {
                        let stmt = parse_for_loop(lines, index)?;
                        stmts.push(stmt);
                        continue;
                    }

                    let raw = lines[*index];
                    let normalised: String;
                    let line_to_parse = if raw.trim().ends_with(';') {
                        raw
                    } else {
                        normalised = format!("{};", raw.trim_end());
                        &normalised
                    };
                    let sig = parse_signal(line_to_parse, *index)?;
                    stmts.push(ModuleMacroStmt::Signal(sig));
                    *index += 1;
                }
                continue;
            }
        }

        if line == "calls {" {
            *index += 1;
            while *index < lines.len() {
                skip_empty_and_comments(lines, index);
                if *index < lines.len() && lines[*index].trim() == "}" {
                    *index += 1;
                    break;
                }
                let raw = lines[*index].trim();
                if !raw.is_empty() {
                    let call_line =
                        if raw.ends_with(';') { raw.to_string() } else { format!("{};", raw) };
                    let mut call = parse_pattern_call_single(&call_line)?;
                    call.span = Some(Span::full_line(*index as u32));
                    stmts.push(ModuleMacroStmt::PatternCall(call));
                }
                *index += 1;
            }
            continue;
        }

        if line.starts_with("guard ") || line.starts_with("let guard ") {
            let guard = parse_guard(lines, index)?;
            stmts.push(ModuleMacroStmt::Guard(guard));
            continue;
        }

        if line.starts_with("reflex ") {
            let reflex = parse_unexpanded_reflex(lines, index)?;
            stmts.push(ModuleMacroStmt::Reflex(reflex));
            continue;
        }

        if line.starts_with("property ") || line.starts_with("assert ") {
            let prop = super::formula_parser::parse_property(lines, index)?;
            stmts.push(ModuleMacroStmt::Property(prop));
            continue;
        }

        if is_pattern_call_start(line) {
            let start_line = *index as u32;
            let mut call = parse_pattern_call(lines, index)?;
            call.span = Some(Span::full_line(start_line));
            stmts.push(ModuleMacroStmt::PatternCall(call));
            continue;
        }

        if line.contains(':') && !line.ends_with('{') {
            let sig = parse_signal(line, *index)?;
            stmts.push(ModuleMacroStmt::Signal(sig));
            *index += 1;
            continue;
        }

        // Just break out, letting caller handle final closing
        break;
    }
    Ok(stmts)
}

fn parse_let_binding_raw(
    line: &str,
    line_index: usize,
) -> Result<(String, String, Expr), MirrError> {
    let trimmed = line.trim().trim_end_matches(';');
    let rest = trimmed.strip_prefix("let ").ok_or_else(|| {
        emit_at(
            ErrorCode::SExprParseError,
            "Expected 'let' binding",
            Span::full_line(line_index as u32),
        )
    })?;
    let (lhs, rhs) = rest.split_once('=').ok_or_else(|| {
        emit_at(
            ErrorCode::SExprParseError,
            "Malformed let binding",
            Span::full_line(line_index as u32),
        )
    })?;
    let (name, ty) = if lhs.contains(':') {
        let (n, t) = lhs.split_once(':').unwrap();
        (n.trim().to_string(), t.trim().to_string())
    } else {
        (lhs.trim().to_string(), "auto".to_string())
    };
    let value = parse_expression(rhs.trim()).map_err(|e| {
        emit_at(
            ErrorCode::SExprParseError,
            format!("Let expr error: {e}"),
            Span::full_line(line_index as u32),
        )
    })?;
    Ok((name, ty, value))
}

fn parse_let_binding(line: &str, line_index: usize) -> Result<ModuleMacroStmt, MirrError> {
    let (name, ty, value) = parse_let_binding_raw(line, line_index)?;
    Ok(ModuleMacroStmt::LetBinding { name, ty, value })
}

fn parse_reflex_let_binding(line: &str, line_index: usize) -> Result<ReflexMacroStmt, MirrError> {
    let (name, ty, value) = parse_let_binding_raw(line, line_index)?;
    Ok(ReflexMacroStmt::LetBinding {
        name,
        ty,
        value,
        span: Some(Span::full_line(line_index as u32)),
    })
}

fn parse_for_loop(lines: &[&str], index: &mut usize) -> Result<ModuleMacroStmt, MirrError> {
    let header = lines[*index].trim();
    let after_for = header.strip_prefix("for ").unwrap();
    let (var, rest) = after_for.split_once(" in ").ok_or_else(|| {
        emit_at(ErrorCode::ForLoopRangeMalformed, "Expected 'in'", Span::full_line(*index as u32))
    })?;
    let range_part = rest.trim_end_matches('{').trim();
    let (start_str, end_str) = range_part.split_once("..").ok_or_else(|| {
        emit_at(ErrorCode::ForLoopRangeMalformed, "Expected '..'", Span::full_line(*index as u32))
    })?;
    let start = start_str.trim().parse::<i32>().unwrap_or(0);
    let end = end_str.trim().parse::<i32>().unwrap_or(0);

    *index += 1; // Consume header
    let body = parse_module_macro_stmts(lines, index)?;

    // Expect closing brace
    if *index < lines.len() && lines[*index].trim().starts_with('}') {
        *index += 1;
    }

    Ok(ModuleMacroStmt::ForLoop { var: var.trim().to_string(), start, end, body })
}

pub fn parse_unexpanded_reflex(
    lines: &[&str],
    index: &mut usize,
) -> Result<UnexpandedReflex, MirrError> {
    let start_line = *index as u32;
    let (name, inline_guards) = reflex_parse_header(lines, index)?;
    *index += 1; // Consume header

    let statements = parse_reflex_macro_stmts(lines, index)?;

    if *index < lines.len() && lines[*index].trim().starts_with('}') {
        *index += 1;
    }
    let end_line = (*index - 1) as u32;

    Ok(UnexpandedReflex {
        name,
        guard_names: inline_guards,
        statements,
        span: Some(Span::multi_line(start_line, end_line)),
    })
}

pub fn parse_reflex_macro_stmts(
    lines: &[&str],
    index: &mut usize,
) -> Result<Vec<ReflexMacroStmt>, MirrError> {
    let mut stmts = Vec::new();
    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }

        let line = lines[*index].trim();
        if line == "}" {
            break;
        }

        if line.starts_with("on ") || line.starts_with("always {") {
            let (guards, body) = if line.starts_with("always {") {
                // Support shorthand 'always {' -> 'on always {'
                let guards = vec!["always".to_string()];
                *index += 1;
                let body = parse_reflex_macro_stmts(lines, index)?;
                if *index < lines.len() && lines[*index].trim().starts_with('}') {
                    *index += 1;
                }
                (guards, body)
            } else {
                parse_on_block(lines, index)?
            };
            stmts.push(ReflexMacroStmt::OnBlock { guard_names: guards, body });
            continue;
        }

        if line.starts_with("for ") {
            let stmt = parse_reflex_for_loop(lines, index)?;
            stmts.push(stmt);
            continue;
        }

        if line.starts_with("if ") {
            let stmt = parse_if_else(lines, index)?;
            stmts.push(stmt);
            continue;
        }

        if line.starts_with("match ") {
            let stmt = parse_match(lines, index)?;
            stmts.push(stmt);
            continue;
        }

        if line.starts_with("let ") {
            let stmt = parse_reflex_let_binding(line, *index)?;
            stmts.push(stmt);
            *index += 1;
            continue;
        }

        if line.contains('=') {
            // ALLOW top-level assignments in reflexes to support the "Rust-like" sugar contract.
            // This is LOWERED into an "on always" block by the expander.
            let assign = parse_assignment(line, *index)?;
            stmts.push(ReflexMacroStmt::Assignment(assign));
            *index += 1;
            continue;
        }

        // Unrecognized, skip one line to avoid infinite loop
        *index += 1;
    }
    Ok(stmts)
}

fn parse_on_block(
    lines: &[&str],
    index: &mut usize,
) -> Result<(Vec<String>, Vec<ReflexMacroStmt>), MirrError> {
    let header = lines[*index].trim();
    let after_on = header.strip_prefix("on ").unwrap();
    let guards_part = after_on.trim_end_matches('{').trim();

    // Simple space-separated guards for now
    let guards: Vec<String> = guards_part
        .split_whitespace()
        .filter(|s| *s != "and")
        .map(|s| s.trim_start_matches('[').trim_end_matches(']').to_string())
        .collect();

    if guards.is_empty() {
        return Err(crate::error_codes::mirrcode(
            crate::error_codes::ErrorCode::ReflexMissingOn,
            "on block must specify at least one guard name",
        )
        .with_span(Some(Span::full_line(*index as u32))));
    }

    *index += 1;
    let body = parse_reflex_macro_stmts(lines, index)?;
    if *index < lines.len() && lines[*index].trim().starts_with('}') {
        *index += 1;
    }
    Ok((guards, body))
}

fn parse_reflex_for_loop(lines: &[&str], index: &mut usize) -> Result<ReflexMacroStmt, MirrError> {
    let header = lines[*index].trim();
    let after_for = header.strip_prefix("for ").unwrap();
    let (var, rest) = after_for.split_once(" in ").unwrap();
    let range_part = rest.trim_end_matches('{').trim();
    let (start_str, end_str) = range_part.split_once("..").unwrap();
    let start = start_str.trim().parse::<i32>().unwrap_or(0);
    let end = end_str.trim().parse::<i32>().unwrap_or(0);

    *index += 1;
    let body = parse_reflex_macro_stmts(lines, index)?;
    if *index < lines.len() && lines[*index].trim().starts_with('}') {
        *index += 1;
    }

    Ok(ReflexMacroStmt::ForLoop { var: var.trim().to_string(), start, end, body })
}

fn parse_if_else(lines: &[&str], index: &mut usize) -> Result<ReflexMacroStmt, MirrError> {
    let header = lines[*index].trim();
    let cond_part = header.strip_prefix("if ").unwrap().trim_end_matches('{').trim();
    let condition = parse_expression(cond_part).map_err(|e| {
        emit_at(
            ErrorCode::SExprParseError,
            format!("If error: {e}"),
            Span::full_line(*index as u32),
        )
    })?;

    *index += 1;
    let true_branch = parse_reflex_macro_stmts(lines, index)?;
    let mut false_branch = Vec::new();

    if *index < lines.len() {
        let line = lines[*index].trim();
        if line == "} else {" {
            *index += 1;
            false_branch = parse_reflex_macro_stmts(lines, index)?;
            if *index < lines.len() && lines[*index].trim().starts_with('}') {
                *index += 1;
            }
        } else if line.starts_with("} else if") {
            // Support 'else if' by recursing with a virtual 'if' line
            let cond_after_else = line.strip_prefix("} else if ").unwrap().trim();
            // We need a dummy line for parse_if_else to consume
            let dummy = format!("if {cond_after_else}");
            let mut virtual_lines = lines.to_vec();
            virtual_lines[*index] = &dummy;
            let nested_if = parse_if_else(&virtual_lines, index)?;
            false_branch = vec![nested_if];
        } else if line.starts_with('}') {
            *index += 1;
        }
    }

    Ok(ReflexMacroStmt::IfElse { condition, true_branch, false_branch })
}

fn parse_match(lines: &[&str], index: &mut usize) -> Result<ReflexMacroStmt, MirrError> {
    let header = lines[*index].trim();
    let expr_part = header.strip_prefix("match ").unwrap().trim_end_matches('{').trim();
    let expr = parse_expression(expr_part).map_err(|e| {
        emit_at(
            ErrorCode::SExprParseError,
            format!("Match error: {e}"),
            Span::full_line(*index as u32),
        )
    })?;

    *index += 1;
    let mut arms = Vec::new();

    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }

        let line = lines[*index].trim();
        if line == "}" {
            *index += 1;
            break;
        }

        if let Some((pat, rest)) = line.split_once("=>") {
            let pattern = pat.trim().to_string();
            let rest = rest.trim();
            if rest.contains('{') {
                *index += 1;
                let body = parse_reflex_macro_stmts(lines, index)?;
                if *index < lines.len() && lines[*index].trim().starts_with('}') {
                    *index += 1;
                }
                arms.push(MatchArm { pattern, body });
            } else {
                // Single line arm
                if !rest.is_empty() {
                    let assign = parse_assignment(rest, *index)?;
                    arms.push(MatchArm {
                        pattern,
                        body: vec![ReflexMacroStmt::Assignment(assign)],
                    });
                } else {
                    arms.push(MatchArm { pattern, body: vec![] });
                }
                *index += 1;
            }
        } else {
            *index += 1;
        }
    }

    Ok(ReflexMacroStmt::Match { expr, arms })
}
