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
    line_offset: usize,
) -> Result<Vec<ModuleMacroStmt>, MirrError> {
    let mut stmts = Vec::new();
    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }

        let abs_index = *index + line_offset;
        let line = lines[*index].trim();
        if line == "}" || line == "endmodule" || line == "end" {
            break;
        }

        if line.starts_with("for ") {
            let stmt = parse_for_loop(lines, index, line_offset)?;
            stmts.push(stmt);
            continue;
        }

        if line.starts_with("let ") && line.contains('=') && !line.starts_with("let guard ") {
            let stmt = parse_let_binding(line, abs_index)?;
            stmts.push(stmt);
            *index += 1;
            continue;
        }

        if line.starts_with("signal ")
            || line.starts_with("in ")
            || line.starts_with("out ")
            || line.starts_with("internal ")
        {
            let sig = parse_signal(line, abs_index)?;
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
                    let abs_index_inner = *index + line_offset;
                    let line = lines[*index].trim();
                    if line.starts_with("for ") {
                        let stmt = parse_for_loop(lines, index, line_offset)?;
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
                    let sig = parse_signal(line_to_parse, abs_index_inner)?;
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
                let abs_index_inner = *index + line_offset;
                let raw = lines[*index].trim();
                if !raw.is_empty() {
                    let call_line =
                        if raw.ends_with(';') { raw.to_string() } else { format!("{};", raw) };
                    let mut call = parse_pattern_call_single(&call_line)?;
                    call.span = Some(Span::full_line(abs_index_inner as u32));
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
            let reflex = parse_unexpanded_reflex(lines, index, line_offset)?;
            stmts.push(ModuleMacroStmt::Reflex(reflex));
            continue;
        }

        if line.starts_with("property ") || line.starts_with("assert ") {
            let prop = super::formula_parser::parse_property(lines, index)?;
            stmts.push(ModuleMacroStmt::Property(prop));
            continue;
        }

        if is_pattern_call_start(line) {
            let abs_start_line = *index + line_offset;
            let mut call = parse_pattern_call(lines, index)?;
            call.span = Some(Span::full_line(abs_start_line as u32));
            stmts.push(ModuleMacroStmt::PatternCall(call));
            continue;
        }

        if line.starts_with("domain ") {
            let abs_start_line = *index + line_offset;
            let stripped = line.strip_prefix("domain ").unwrap().trim();
            let name = stripped.trim_end_matches(';').trim().to_string();
            stmts.push(ModuleMacroStmt::ClockDomain(crate::ast::program::ClockDomainDecl {
                name,
                span: Some(Span::full_line(abs_start_line as u32)),
            }));
            *index += 1;
            continue;
        }

        if line.contains(':') && !line.ends_with('{') {
            let sig = parse_signal(line, abs_index)?;
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
        let (n, t) = lhs.split_once(':').ok_or_else(|| {
            emit_at(
                ErrorCode::SExprParseError,
                "Malformed let binding type annotation",
                Span::full_line(line_index as u32),
            )
        })?;
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

fn parse_for_loop(
    lines: &[&str],
    index: &mut usize,
    line_offset: usize,
) -> Result<ModuleMacroStmt, MirrError> {
    let abs_header_index = *index + line_offset;
    let header = lines[*index].trim();
    let after_for = header.strip_prefix("for ").ok_or_else(|| {
        emit_at(
            ErrorCode::ForLoopRangeMalformed,
            "Expected 'for' loop header",
            Span::full_line(abs_header_index as u32),
        )
    })?;
    let (var, rest) = after_for.split_once(" in ").ok_or_else(|| {
        emit_at(
            ErrorCode::ForLoopRangeMalformed,
            "Expected 'in'",
            Span::full_line(abs_header_index as u32),
        )
    })?;
    let range_part = rest.trim_end_matches('{').trim();
    let (start_str, end_str) = range_part.split_once("..").ok_or_else(|| {
        emit_at(
            ErrorCode::ForLoopRangeMalformed,
            "Expected '..'",
            Span::full_line(abs_header_index as u32),
        )
    })?;
    let start = start_str.trim().parse::<i32>().unwrap_or(0);
    let end = end_str.trim().parse::<i32>().unwrap_or(0);

    let body_offset = abs_header_index + 1;
    *index += 1; // Consume header
    let block_lines_owned = crate::parser::collect_block_lines(lines, index)?;
    let block_lines: Vec<&str> = block_lines_owned.iter().map(|s| s.as_str()).collect();
    let mut block_index = 0;

    let body = parse_module_macro_stmts(&block_lines, &mut block_index, body_offset)?;

    Ok(ModuleMacroStmt::ForLoop { var: var.trim().to_string(), start, end, body })
}

pub fn parse_unexpanded_reflex(
    lines: &[&str],
    index: &mut usize,
    line_offset: usize,
) -> Result<UnexpandedReflex, MirrError> {
    let abs_start_line = *index + line_offset;
    let (name, inline_guards) = reflex_parse_header(lines, index)?;
    let body_offset = abs_start_line + 1;
    *index += 1; // Consume header

    let block_lines_owned = crate::parser::collect_block_lines(lines, index)?;
    let block_lines: Vec<&str> = block_lines_owned.iter().map(|s| s.as_str()).collect();
    let mut block_index = 0;

    let statements = parse_reflex_macro_stmts(&block_lines, &mut block_index, body_offset)?;

    let abs_end_line = (*index + line_offset - 1) as u32;

    Ok(UnexpandedReflex {
        name,
        guard_names: inline_guards,
        statements,
        span: Some(Span::multi_line(abs_start_line as u32, abs_end_line)),
    })
}

pub fn parse_reflex_macro_stmts(
    lines: &[&str],
    index: &mut usize,
    line_offset: usize,
) -> Result<Vec<ReflexMacroStmt>, MirrError> {
    let mut stmts = Vec::new();
    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }

        let abs_index = *index + line_offset;
        let line = lines[*index].trim();
        if line == "}" {
            break;
        }

        if line.starts_with("on ") || line.starts_with("always {") {
            let (guards, body) = if line.starts_with("always {") {
                // Support shorthand 'always {' -> 'on always {'
                let guards = vec!["always".to_string()];
                *index += 1;
                let body = parse_reflex_macro_stmts(lines, index, line_offset)?;
                if *index < lines.len() && lines[*index].trim().starts_with('}') {
                    *index += 1;
                }
                (guards, body)
            } else {
                parse_on_block(lines, index, line_offset)?
            };
            stmts.push(ReflexMacroStmt::OnBlock { guard_names: guards, body });
            continue;
        }

        if line.starts_with("for ") {
            let stmt = parse_reflex_for_loop(lines, index, line_offset)?;
            stmts.push(stmt);
            continue;
        }

        if line.starts_with("if ") {
            let stmt = parse_if_else(lines, index, line_offset)?;
            stmts.push(stmt);
            continue;
        }

        if line.starts_with("match ") {
            let stmt = parse_match(lines, index, line_offset)?;
            stmts.push(stmt);
            continue;
        }

        if line.starts_with("let ") {
            let stmt = parse_reflex_let_binding(line, abs_index)?;
            stmts.push(stmt);
            *index += 1;
            continue;
        }

        if line.contains('=') {
            // ALLOW top-level assignments in reflexes to support the "Rust-like" sugar contract.
            // This is LOWERED into an "on always" block by the expander.
            let assign = parse_assignment(line, abs_index)?;
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
    line_offset: usize,
) -> Result<(Vec<String>, Vec<ReflexMacroStmt>), MirrError> {
    let abs_header_index = *index + line_offset;
    let header = lines[*index].trim();
    let after_on = header.strip_prefix("on ").ok_or_else(|| {
        emit_at(
            ErrorCode::ReflexMissingOn,
            "Expected 'on' block header",
            Span::full_line(abs_header_index as u32),
        )
    })?;
    let guards_part = after_on.trim_end_matches('{').trim();
    let mut guards_str = guards_part;
    if guards_str.starts_with('[') && guards_str.ends_with(']') {
        guards_str = &guards_str[1..guards_str.len() - 1];
    }

    // Simple space-separated guards for now
    let guards: Vec<String> =
        guards_str.split_whitespace().filter(|s| *s != "and").map(|s| s.to_string()).collect();

    if guards.is_empty() {
        return Err(crate::error_codes::mirrcode(
            crate::error_codes::ErrorCode::ReflexMissingOn,
            "on block must specify at least one guard name",
        )
        .with_span(Some(Span::full_line(abs_header_index as u32))));
    }

    let body_offset = abs_header_index + 1;
    *index += 1;
    let block_lines_owned = crate::parser::collect_block_lines(lines, index)?;
    let block_lines: Vec<&str> = block_lines_owned.iter().map(|s| s.as_str()).collect();
    let mut block_index = 0;

    let body = parse_reflex_macro_stmts(&block_lines, &mut block_index, body_offset)?;
    Ok((guards, body))
}

fn parse_reflex_for_loop(
    lines: &[&str],
    index: &mut usize,
    line_offset: usize,
) -> Result<ReflexMacroStmt, MirrError> {
    let header_abs_index = *index + line_offset;
    let header = lines[*index].trim();
    let after_for = header.strip_prefix("for ").ok_or_else(|| {
        emit_at(
            ErrorCode::ForLoopRangeMalformed,
            "Expected 'for' loop header",
            Span::full_line(header_abs_index as u32),
        )
    })?;
    let (var, rest) = after_for.split_once(" in ").ok_or_else(|| {
        emit_at(
            ErrorCode::ForLoopRangeMalformed,
            "Expected 'in' keyword in loop header",
            Span::full_line(header_abs_index as u32),
        )
    })?;
    let range_part = rest.trim_end_matches('{').trim();
    let (start_str, end_str) = range_part.split_once("..").ok_or_else(|| {
        emit_at(
            ErrorCode::ForLoopRangeMalformed,
            "Expected '..' range operator in loop header",
            Span::full_line(header_abs_index as u32),
        )
    })?;
    let start = start_str.trim().parse::<i32>().unwrap_or(0);
    let end = end_str.trim().parse::<i32>().unwrap_or(0);

    let body_offset = header_abs_index + 1;
    *index += 1;
    let block_lines_owned = crate::parser::collect_block_lines(lines, index)?;
    let block_lines: Vec<&str> = block_lines_owned.iter().map(|s| s.as_str()).collect();
    let mut block_index = 0;

    let body = parse_reflex_macro_stmts(&block_lines, &mut block_index, body_offset)?;

    Ok(ReflexMacroStmt::ForLoop { var: var.trim().to_string(), start, end, body })
}

fn parse_if_else(
    lines: &[&str],
    index: &mut usize,
    line_offset: usize,
) -> Result<ReflexMacroStmt, MirrError> {
    let header_abs_index = *index + line_offset;
    let header = lines[*index].trim();
    let cond_part = header
        .strip_prefix("if ")
        .ok_or_else(|| {
            emit_at(
                ErrorCode::SExprParseError,
                "Expected 'if' block header",
                Span::full_line(header_abs_index as u32),
            )
        })?
        .trim_end_matches('{')
        .trim();
    let condition = parse_expression(cond_part).map_err(|e| {
        emit_at(
            ErrorCode::SExprParseError,
            format!("If error: {e}"),
            Span::full_line(header_abs_index as u32),
        )
    })?;

    let body_offset = header_abs_index + 1;
    *index += 1;
    let block_lines_owned = crate::parser::collect_block_lines(lines, index)?;
    let block_lines: Vec<&str> = block_lines_owned.iter().map(|s| s.as_str()).collect();
    let mut block_index = 0;
    let true_branch = parse_reflex_macro_stmts(&block_lines, &mut block_index, body_offset)?;

    let mut false_branch = Vec::new();

    if *index < lines.len() {
        let abs_index = *index + line_offset;
        let line = lines[*index].trim();
        if line == "else {" || line == "} else {" {
            let else_body_offset = abs_index + 1;
            *index += 1;
            let else_lines_owned = crate::parser::collect_block_lines(lines, index)?;
            let else_lines: Vec<&str> = else_lines_owned.iter().map(|s| s.as_str()).collect();
            let mut else_idx = 0;
            false_branch = parse_reflex_macro_stmts(&else_lines, &mut else_idx, else_body_offset)?;
        } else if line.starts_with("} else if") || line.starts_with("else if") {
            let cond_after_else = if let Some(stripped) = line.strip_prefix("} else if ") {
                stripped
            } else if let Some(stripped) = line.strip_prefix("else if ") {
                stripped
            } else {
                line
            }
            .trim();
            // Recurse into parse_if_else but without the '}' in the header
            let dummy = format!("if {cond_after_else}");
            let mut virtual_lines = lines.to_vec();
            virtual_lines[*index] = &dummy;
            let nested_if = parse_if_else(&virtual_lines, index, line_offset)?;
            false_branch = vec![nested_if];
        }
    }

    Ok(ReflexMacroStmt::IfElse { condition, true_branch, false_branch })
}

fn parse_match(
    lines: &[&str],
    index: &mut usize,
    line_offset: usize,
) -> Result<ReflexMacroStmt, MirrError> {
    let header_abs_index = *index + line_offset;
    let header = lines[*index].trim();
    let expr_part = header
        .strip_prefix("match ")
        .ok_or_else(|| {
            emit_at(
                ErrorCode::SExprParseError,
                "Expected 'match' block header",
                Span::full_line(header_abs_index as u32),
            )
        })?
        .trim_end_matches('{')
        .trim();
    let expr = parse_expression(expr_part).map_err(|e| {
        emit_at(
            ErrorCode::SExprParseError,
            format!("Match error: {e}"),
            Span::full_line(header_abs_index as u32),
        )
    })?;

    *index += 1;
    let mut arms = Vec::new();

    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }

        let abs_index = *index + line_offset;
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
                let block_lines_owned = crate::parser::collect_block_lines(lines, index)?;
                let block_lines: Vec<&str> = block_lines_owned.iter().map(|s| s.as_str()).collect();
                let mut block_idx = 0;
                let body = parse_reflex_macro_stmts(&block_lines, &mut block_idx, abs_index + 1)?;
                arms.push(MatchArm { pattern, body });
            } else {
                // Single line arm
                if !rest.is_empty() {
                    let assign = parse_assignment(rest, abs_index)?;
                    arms.push(MatchArm {
                        pattern,
                        body: vec![ReflexMacroStmt::Assignment(assign)],
                    });
                }
                *index += 1;
            }
        } else {
            return Err(emit_at(
                ErrorCode::SExprParseError,
                "Expected match arm",
                Span::full_line(abs_index as u32),
            ));
        }
    }

    Ok(ReflexMacroStmt::Match { expr, arms })
}
