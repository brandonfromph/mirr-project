//! Helper functions and preprocessors for the MIRR macro processor.
//! Keeps the main macro_proc.rs file within the strict 600-line limit.

#![forbid(unsafe_code)]

use super::inline_helpers::replace_whole_word;
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use crate::ast::Expr;
use crate::parser::parse_expression;
use crate::simplify::simplify_expr;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct MatchArm {
    pub(crate) pattern: String,
    pub(crate) body: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) enum MatchParserState {
    Normal,
    CollectingMatch {
        expr: String,
        arms: Vec<MatchArm>,
        current_pattern: Option<String>,
        current_body: Vec<String>,
        brace_depth: i32,
    },
}

pub(crate) fn count_braces_in_line(line: &str) -> i32 {
    let mut count = 0;
    for c in line.chars() {
        if c == '{' {
            count += 1;
        } else if c == '}' {
            count -= 1;
        }
    }
    count
}

pub(crate) fn generate_if_else(expr: &str, arms: &[MatchArm]) -> String {
    let mut out = String::new();
    for (i, arm) in arms.iter().enumerate() {
        let is_default = arm.pattern == "_" || arm.pattern == "default";
        if i == 0 {
            if is_default {
                out.push_str("{\n");
            } else {
                out.push_str(&format!("if {} == {} {{\n", expr, arm.pattern));
            }
        } else if is_default {
            out.push_str("} else {\n");
        } else {
            out.push_str(&format!("}} else if {} == {} {{\n", expr, arm.pattern));
        }
        for line in &arm.body {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
        }
    }
    out.push_str("}\n");
    out
}

pub(crate) fn preprocess_match_blocks(source: &str) -> String {
    let mut result = String::new();
    let mut state = MatchParserState::Normal;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        match state {
            MatchParserState::Normal => {
                if trimmed.starts_with("match ") && trimmed.contains('{') {
                    let after_match = trimmed.strip_prefix("match ").unwrap_or(trimmed);
                    let expr = after_match
                        .split_once('{')
                        .unwrap_or((after_match, ""))
                        .0
                        .trim()
                        .to_string();
                    state = MatchParserState::CollectingMatch {
                        expr,
                        arms: Vec::new(),
                        current_pattern: None,
                        current_body: Vec::new(),
                        brace_depth: 0,
                    };
                } else {
                    result.push_str(line);
                    result.push('\n');
                }
            }
            MatchParserState::CollectingMatch {
                ref expr,
                ref mut arms,
                ref mut current_pattern,
                ref mut current_body,
                ref mut brace_depth,
            } => {
                if current_pattern.is_none() {
                    if trimmed == "}" {
                        // End of match block
                        let expanded = generate_if_else(expr, arms);
                        result.push_str(&expanded);
                        state = MatchParserState::Normal;
                    } else if trimmed.contains("=>") {
                        let (pat, rest) = trimmed.split_once("=>").unwrap_or((trimmed, ""));
                        let pat = pat.trim().to_string();
                        let rest = rest.trim();
                        *current_pattern = Some(pat);
                        current_body.clear();
                        *brace_depth = 0;
                        if rest.contains('{') {
                            let delta = count_braces_in_line(rest);
                            *brace_depth += delta;
                            let mut body_part = rest.trim_start_matches('{').trim().to_string();
                            if body_part.ends_with('}') {
                                body_part.pop();
                            }
                            let body_part = body_part.trim();
                            if !body_part.is_empty() {
                                current_body.push(body_part.to_string());
                            }
                            if *brace_depth <= 0 {
                                arms.push(MatchArm {
                                    pattern: current_pattern.take().unwrap_or_default(),
                                    body: current_body.clone(),
                                });
                            }
                        } else {
                            current_body.push(rest.to_string());
                            // Single line case without braces
                            arms.push(MatchArm {
                                pattern: current_pattern.take().unwrap_or_default(),
                                body: current_body.clone(),
                            });
                        }
                    }
                } else {
                    let delta = count_braces_in_line(trimmed);
                    *brace_depth += delta;
                    if *brace_depth <= 0 {
                        // Arm closed
                        let mut body_line = trimmed.to_string();
                        if body_line.ends_with('}') {
                            body_line.pop();
                        }
                        let body_line = body_line.trim();
                        if !body_line.is_empty() {
                            current_body.push(body_line.to_string());
                        }
                        arms.push(MatchArm {
                            pattern: current_pattern.take().unwrap_or_default(),
                            body: current_body.clone(),
                        });
                    } else {
                        current_body.push(trimmed.to_string());
                    }
                }
            }
        }
    }
    result
}

pub(crate) fn parse_let_binding(line: &str) -> Option<(String, String, String)> {
    let trimmed = line.trim().trim_end_matches(';');
    if !trimmed.starts_with("let ") {
        return None;
    }
    let content = trimmed.strip_prefix("let ")?.trim();
    // Split by '=' first to get LHS and RHS
    let (lhs, rhs) = content.split_once('=')?;
    let lhs = lhs.trim();
    let rhs = rhs.trim();

    // LHS should contain ':' for type annotation
    let (name, ty) = lhs.split_once(':')?;
    let name = name.trim();
    let ty = ty.trim();

    // Validate name is a valid identifier
    if !name.is_empty()
        && name.chars().next()?.is_alphabetic()
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
    {
        Some((name.to_string(), ty.to_string(), rhs.to_string()))
    } else {
        None
    }
}

pub(crate) fn preprocess_let_bindings(source: &str) -> (String, Vec<String>) {
    let mut result = String::new();
    let mut decls = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        if let Some((name, ty, expr)) = parse_let_binding(trimmed) {
            let decl = format!("signal {}: internal {};", name, ty);
            if !decls.contains(&decl) {
                decls.push(decl);
            }
            // Retain the indentation of the original line
            let indent = line.len() - line.trim_start().len();
            let indent_str = " ".repeat(indent);
            result.push_str(&format!("{}{} = {};\n", indent_str, name, expr));
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    (result, decls)
}

pub(crate) fn inline_let_bindings(source: &str) -> String {
    let mut result = String::new();
    let mut brace_depth = 0;
    let mut inline_stack: Vec<(String, String, i32)> = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();

        let delta = count_braces_in_line(line);
        let old_depth = brace_depth;
        brace_depth += delta;

        // Pop variables that are out of scope
        inline_stack.retain(|(_, _, depth)| brace_depth >= *depth);

        if trimmed.is_empty() || trimmed.starts_with("//") {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        if let Some((name, _ty, expr)) = parse_let_binding(trimmed) {
            if name == "is_reflexive" {
                let mut inlined_expr = expr.clone();
                for (var_name, var_expr, _) in &inline_stack {
                    inlined_expr = replace_whole_word(&inlined_expr, var_name, var_expr);
                }
                inline_stack.push((name, format!("({})", inlined_expr), old_depth));
                continue;
            }
        }

        let mut processed_line = line.to_string();
        for (var_name, var_expr, _) in &inline_stack {
            processed_line = replace_whole_word(&processed_line, var_name, var_expr);
        }
        result.push_str(&processed_line);
        result.push('\n');
    }

    result
}

pub(crate) fn inject_declarations(source: &str, decls: &[String]) -> String {
    if decls.is_empty() {
        return source.to_string();
    }
    let mut out = String::new();
    let mut injected = false;
    for line in source.lines() {
        out.push_str(line);
        out.push('\n');
        if !injected
            && ((line.trim().starts_with("module ") && line.contains('{'))
                || line.trim().starts_with("reflect {"))
        {
            for decl in decls {
                out.push_str("    ");
                out.push_str(decl);
                out.push('\n');
            }
            injected = true;
        }
    }
    out
}

fn format_expr_to_string(expr: &Expr) -> String {
    format_expr_to_string_bounded(expr, 0)
}

fn format_expr_to_string_bounded(expr: &Expr, depth: usize) -> String {
    if depth > 16 {
        return "<complex_expr>".to_string();
    }
    match expr {
        Expr::Literal(val) => match val {
            LiteralValue::Bool(b) => b.to_string(),
            LiteralValue::Integer(i) => i.to_string(),
        },
        Expr::Signal(s) => s.clone(),
        Expr::Unary { op, operand } => {
            let op_str = match op {
                UnaryOp::Not => "!",
                UnaryOp::Negate => "-",
            };
            format!("{}{}", op_str, format_expr_to_string_bounded(operand, depth + 1))
        }
        Expr::Binary { op, left, right } => {
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Shl => "<<",
                BinaryOp::Shr => ">>",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
                BinaryOp::BitwiseOr => "|",
                BinaryOp::BitwiseAnd => "&",
                BinaryOp::Xor => "^",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                BinaryOp::Eq => "==",
                BinaryOp::Ne => "!=",
            };
            format!(
                "({} {} {})",
                format_expr_to_string_bounded(left, depth + 1),
                op_str,
                format_expr_to_string_bounded(right, depth + 1)
            )
        }
        Expr::Prev { signal, delay } => format!("prev({}, {})", signal, delay),
        Expr::ArrayIndex { array, index } => format!(
            "{}[{}]",
            format_expr_to_string_bounded(array, depth + 1),
            format_expr_to_string_bounded(index, depth + 1)
        ),
        Expr::FieldAccess { object, field } => {
            format!("{}.{}", format_expr_to_string_bounded(object, depth + 1), field)
        }
        Expr::ArrayLiteral(elems) => {
            let parts: Vec<String> =
                elems.iter().map(|e| format_expr_to_string_bounded(e, depth + 1)).collect();
            format!("[{}]", parts.join(", "))
        }
        Expr::StructLiteral { name, fields } => {
            let parts: Vec<String> = fields
                .iter()
                .map(|(f, e)| format!("{}: {}", f, format_expr_to_string_bounded(e, depth + 1)))
                .collect();
            format!("{} {{ {} }}", name, parts.join(", "))
        }
        Expr::UnfoldIndex(s) => s.clone(),
    }
}

fn apply_constraints(cond: &str, stack: &[Option<(String, String)>]) -> String {
    let mut current = cond.to_string();
    for opt in stack {
        if let Some((s, val)) = opt {
            current = replace_whole_word(&current, s, val);
        }
    }
    current
}

fn parse_equality_constraint(cond: &str) -> Option<(String, String)> {
    if let Ok(Expr::Binary { op: BinaryOp::Eq, left, right }) = parse_expression(cond) {
        match (*left, *right) {
            (Expr::Signal(s), Expr::Literal(LiteralValue::Integer(i))) => Some((s, i.to_string())),
            (Expr::Literal(LiteralValue::Integer(i)), Expr::Signal(s)) => Some((s, i.to_string())),
            (Expr::Signal(s), Expr::Literal(LiteralValue::Bool(b))) => Some((s, b.to_string())),
            (Expr::Literal(LiteralValue::Bool(b)), Expr::Signal(s)) => Some((s, b.to_string())),
            _ => None,
        }
    } else {
        None
    }
}

fn simplify_conditional_string(cond: &str) -> String {
    if let Ok(expr) = parse_expression(cond) {
        let simplified = simplify_expr(expr);
        format_expr_to_string(&simplified)
    } else {
        cond.to_string()
    }
}

fn has_block_opening_brace(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    let mut prev_char = ' ';
    while let Some(c) = chars.next() {
        if c == '{' && prev_char != '$' {
            return true;
        }
        prev_char = c;
    }
    false
}

pub(crate) fn preprocess_if_else_reflexes(source: &str) -> (String, Vec<String>) {
    let mut result = String::new();
    let mut decls = Vec::new();
    let mut counter = 0usize;
    let mut cond_to_guard = std::collections::HashMap::new();
    let mut constraint_stack: Vec<Option<(String, String)>> = Vec::new();

    // First collect all declared guards to avoid duplicate auto-guards
    let mut declared_guards = std::collections::HashSet::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("guard ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() > 1 {
                let name = parts[1].trim_end_matches('{').trim_end_matches('=').trim();
                declared_guards.insert(name.to_string());
            }
        } else if trimmed.starts_with("let guard ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() > 2 {
                let name = parts[2].trim_end_matches('{').trim_end_matches('=').trim();
                declared_guards.insert(name.to_string());
            }
        }
    }

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        // Inside reflex, look for 'if ' or 'else if '
        let mut check_line = trimmed;
        if check_line.starts_with('}') {
            constraint_stack.pop();
            check_line = check_line.strip_prefix('}').unwrap_or(check_line).trim();
        }

        if check_line.starts_with("if ") && has_block_opening_brace(check_line) {
            let cond_raw =
                check_line.strip_prefix("if ").unwrap_or(check_line).trim_end_matches('{').trim();
            let cond_substituted = apply_constraints(cond_raw, &constraint_stack);
            let cond_simplified = simplify_conditional_string(&cond_substituted);
            let cond = if cond_simplified == "true" || cond_simplified == "(true)" {
                "always"
            } else {
                &cond_simplified
            };
            let prefix = if trimmed.starts_with('}') { "} " } else { "" };

            if cond != "always" {
                let is_simple_guard = cond.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && declared_guards.contains(cond);
                if is_simple_guard {
                    let indent = line.len() - line.trim_start().len();
                    let indent_str = " ".repeat(indent);
                    result.push_str(&format!("{}{}if {} {{\n", indent_str, prefix, cond));
                } else if let Some(existing_guard) = cond_to_guard.get(cond) {
                    let indent = line.len() - line.trim_start().len();
                    let indent_str = " ".repeat(indent);
                    result.push_str(&format!("{}{}if {} {{\n", indent_str, prefix, existing_guard));
                } else {
                    let mut guard_name = format!("auto_g_{}", counter);
                    while declared_guards.contains(&guard_name) {
                        counter += 1;
                        guard_name = format!("auto_g_{}", counter);
                    }
                    counter += 1;

                    let guard_decl = format!(
                        "guard {} {{\n    when {}\n    for 1 cycles;\n}}",
                        guard_name, cond
                    );
                    decls.push(guard_decl);
                    cond_to_guard.insert(cond.to_string(), guard_name.clone());

                    // Retain indentation
                    let indent = line.len() - line.trim_start().len();
                    let indent_str = " ".repeat(indent);
                    result.push_str(&format!("{}{}if {} {{\n", indent_str, prefix, guard_name));
                }
            } else {
                let indent = line.len() - line.trim_start().len();
                let indent_str = " ".repeat(indent);
                result.push_str(&format!("{}{}if always {{\n", indent_str, prefix));
            }
            // Push active equality constraint
            constraint_stack.push(parse_equality_constraint(cond_raw));
        } else if check_line.starts_with("else if ") && has_block_opening_brace(check_line) {
            let cond_raw = check_line
                .strip_prefix("else if ")
                .unwrap_or(check_line)
                .trim_end_matches('{')
                .trim();
            let cond_substituted = apply_constraints(cond_raw, &constraint_stack);
            let cond_simplified = simplify_conditional_string(&cond_substituted);
            let cond = if cond_simplified == "true" || cond_simplified == "(true)" {
                "always"
            } else {
                &cond_simplified
            };
            let prefix = if trimmed.starts_with('}') { "} " } else { "" };

            if cond != "always" {
                let is_simple_guard = cond.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && declared_guards.contains(cond);
                if is_simple_guard {
                    let indent = line.len() - line.trim_start().len();
                    let indent_str = " ".repeat(indent);
                    result.push_str(&format!("{}{}else if {} {{\n", indent_str, prefix, cond));
                } else if let Some(existing_guard) = cond_to_guard.get(cond) {
                    let indent = line.len() - line.trim_start().len();
                    let indent_str = " ".repeat(indent);
                    result.push_str(&format!(
                        "{}{}else if {} {{\n",
                        indent_str, prefix, existing_guard
                    ));
                } else {
                    let mut guard_name = format!("auto_g_{}", counter);
                    while declared_guards.contains(&guard_name) {
                        counter += 1;
                        guard_name = format!("auto_g_{}", counter);
                    }
                    counter += 1;

                    let guard_decl = format!(
                        "guard {} {{\n    when {}\n    for 1 cycles;\n}}",
                        guard_name, cond
                    );
                    decls.push(guard_decl);
                    cond_to_guard.insert(cond.to_string(), guard_name.clone());

                    let indent = line.len() - line.trim_start().len();
                    let indent_str = " ".repeat(indent);
                    result
                        .push_str(&format!("{}{}else if {} {{\n", indent_str, prefix, guard_name));
                }
            } else {
                let indent = line.len() - line.trim_start().len();
                let indent_str = " ".repeat(indent);
                result.push_str(&format!("{}{}else if always {{\n", indent_str, prefix));
            }
            // Push active equality constraint
            constraint_stack.push(parse_equality_constraint(cond_raw));
        } else {
            if has_block_opening_brace(trimmed) && !trimmed.starts_with("//") {
                constraint_stack.push(None);
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    (result, decls)
}
