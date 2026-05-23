//! Ergonomic macro processor for MIRR source.
//! This module handles text-level expansion of ergonomic syntax
//! into the AST parser's strict grammar.

#![forbid(unsafe_code)]

#[derive(Debug, PartialEq, Clone)]
enum ParserState {
    TopLevel,
    AwaitingSignalsBrace,
    InSignals,
    InLoop { var: String, start: i32, end: i32, body: Vec<String> },
    InReflex,
}

#[derive(Debug, Clone)]
struct MatchArm {
    pattern: String,
    body: Vec<String>,
}

#[derive(Debug, Clone)]
enum MatchParserState {
    Normal,
    CollectingMatch {
        expr: String,
        arms: Vec<MatchArm>,
        current_pattern: Option<String>,
        current_body: Vec<String>,
        brace_depth: i32,
    },
}

pub fn expand_macros(source: &str) -> String {
    // Pass 1: Parse and preprocess match blocks into standard if/else if/else structures
    let matched_source = preprocess_match_blocks(source);

    // Pass 2: Parse and extract local let-bindings into top-level signals
    let (let_source, decls) = preprocess_let_bindings(&matched_source);

    // Pass 3: Inject the collected signal declarations at the top of the module
    let preprocessed = inject_declarations(&let_source, &decls);

    let mut result = String::new();
    let mut state = ParserState::TopLevel;
    let mut in_on_block = false;
    let mut reflex_depth = 0;

    for line in preprocessed.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        match state {
            ParserState::TopLevel => {
                if trimmed.starts_with("signals") {
                    if trimmed.contains('{') {
                        if trimmed.contains('}') {
                            result.push('\n');
                        } else {
                            state = ParserState::InSignals;
                            result.push('\n');
                        }
                    } else {
                        state = ParserState::AwaitingSignalsBrace;
                        result.push('\n');
                    }
                } else if trimmed.starts_with("reflex ") && trimmed.contains('{') {
                    // Check if there is a STRUCTURAL closing brace on the same line
                    let mut count = 0;
                    let mut has_close = false;
                    let mut chars = trimmed.chars().peekable();
                    while let Some(c) = chars.next() {
                        if c == '$' {
                            if let Some('{') = chars.peek() {
                                chars.next(); // consume '{'
                                for inner in chars.by_ref() {
                                    if inner == '}' {
                                        break;
                                    }
                                }
                                continue;
                            }
                        }
                        if c == '{' {
                            count += 1;
                        } else if c == '}' {
                            count -= 1;
                            if count == 0 {
                                has_close = true;
                            }
                        }
                    }

                    if has_close {
                        // Single-line reflex
                        let (prefix, rest) = trimmed.split_once('{').unwrap();
                        result.push_str(prefix);
                        result.push_str(" {\n");
                        let inner = rest.trim_end_matches('}').trim();
                        if !inner.is_empty() {
                            if inner.contains('=') {
                                result.push_str("on always {\n");
                                result.push_str(inner.trim_end_matches(';'));
                                result.push_str(";\n}\n");
                            } else {
                                result.push_str(inner);
                                result.push('\n');
                            }
                        }
                        result.push_str("}\n");
                    } else {
                        state = ParserState::InReflex;
                        reflex_depth = 1;
                        in_on_block = trimmed.contains(" when ");
                        result.push_str(line);
                        result.push('\n');
                    }
                } else if (trimmed.starts_with("guard ") || trimmed.starts_with("let guard "))
                    && trimmed.contains('=')
                    && (trimmed.contains("when") || trimmed.contains("guard"))
                {
                    if let Some(expanded) = expand_guard_assignment(trimmed) {
                        result.push_str(&expanded);
                        result.push('\n');
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                } else {
                    result.push_str(line);
                    result.push('\n');
                }
            }
            ParserState::AwaitingSignalsBrace => {
                if trimmed == "{" {
                    state = ParserState::InSignals;
                    result.push('\n');
                } else {
                    state = ParserState::TopLevel;
                    result.push_str(line);
                    result.push('\n');
                }
            }
            ParserState::InSignals => {
                if trimmed.starts_with('}') {
                    state = ParserState::TopLevel;
                    result.push('\n');
                } else if trimmed.starts_with("for ") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let var = parts[1].to_string();
                        let range = parts[3].trim_end_matches('{').trim();
                        let range_parts: Vec<&str> = range.split("..").collect();
                        if range_parts.len() == 2 {
                            let start = range_parts[0].parse().unwrap_or(0);
                            let end = range_parts[1].parse().unwrap_or(0);
                            state = ParserState::InLoop { var, start, end, body: Vec::new() };
                            result.push('\n');
                        } else {
                            result.push_str(line);
                            result.push('\n');
                        }
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                } else {
                    result.push_str("    ");
                    result.push_str(&expand_signal_line(trimmed));
                    result.push('\n');
                }
            }
            ParserState::InLoop { ref var, start, end, ref mut body } => {
                if trimmed == "}" {
                    for i in start..end {
                        for line in body.iter() {
                            let mut expanded = line.clone();
                            let placeholder = format!("[{}]", var);
                            let replacement = format!("_{}", i);
                            expanded = expanded.replace(&placeholder, &replacement);
                            result.push_str("    signal ");
                            result.push_str(&expanded);
                            result.push_str(";\n");
                        }
                    }
                    state = ParserState::InSignals;
                    result.push('\n');
                } else {
                    body.push(trimmed.trim_end_matches(';').to_string());
                    result.push('\n');
                }
            }
            ParserState::InReflex => {
                let delta = count_braces(trimmed);
                reflex_depth += delta;

                // Flexible check for 'if', 'else if', 'else' with possible leading '}'
                let mut check_line = trimmed;
                if check_line.starts_with('}') {
                    check_line = check_line.strip_prefix('}').unwrap().trim();
                }

                if (check_line.starts_with("on ")
                    || check_line.starts_with("if ")
                    || check_line.starts_with("else"))
                    && check_line.contains('{')
                {
                    in_on_block = true;

                    if check_line.starts_with("if ") {
                        let cond =
                            check_line.strip_prefix("if ").unwrap().trim_end_matches('{').trim();
                        result.push_str("on ");
                        result.push_str(cond);
                        result.push_str(" {\n");
                        continue;
                    } else if check_line.starts_with("else if ") {
                        let cond = check_line
                            .strip_prefix("else if ")
                            .unwrap()
                            .trim_end_matches('{')
                            .trim();
                        result.push_str("} on ");
                        result.push_str(cond);
                        result.push_str(" {\n");
                        continue;
                    } else if check_line.starts_with("else") {
                        result.push_str("} on always {\n");
                        continue;
                    }
                }

                if reflex_depth <= 0 {
                    state = ParserState::TopLevel;
                    result.push_str(line);
                    result.push('\n');
                } else if trimmed == "}" && in_on_block {
                    in_on_block = false;
                    result.push_str(line);
                    result.push('\n');
                } else if trimmed.contains('=') {
                    if !in_on_block {
                        result.push_str("on always {\n");
                        result.push_str(trimmed.trim_end_matches(';'));
                        result.push_str(";\n}\n");
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                } else {
                    result.push_str(line);
                    result.push('\n');
                }
            }
        }
    }

    result
}

fn count_braces(line: &str) -> i32 {
    let mut count = 0;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            if let Some('{') = chars.peek() {
                chars.next(); // consume '{'
                for inner in chars.by_ref() {
                    if inner == '}' {
                        break;
                    }
                }
                continue;
            }
        }
        if c == '{' {
            count += 1;
        } else if c == '}' {
            count -= 1;
        }
    }
    count
}

fn expand_signal_line(line: &str) -> String {
    let trimmed = line.trim_end_matches(';');
    if trimmed.starts_with("signal ") || trimmed.is_empty() {
        line.to_string()
    } else {
        format!("signal {};", trimmed)
    }
}

fn expand_guard_assignment(line: &str) -> Option<String> {
    let trimmed = line.trim_end_matches(';');
    let (lhs, rhs_raw) = trimmed.split_once('=')?;
    let rhs = rhs_raw.trim();
    let target = lhs.strip_prefix("let guard ").or_else(|| lhs.strip_prefix("guard "))?.trim();

    let after_when = rhs.strip_prefix("when ")?.trim();

    // Check if it has a 'for' clause
    if let Some((cond, for_part)) = after_when.split_once(" for ") {
        let cycles = for_part.trim_end_matches(" cycles").trim();
        Some(format!("guard {} {{\n  when {}\n  for {} cycles\n}}", target, cond.trim(), cycles))
    } else {
        Some(format!("guard {} {{\n  when {}\n  for 1 cycles\n}}", target, after_when))
    }
}

// --- NEW RUST-LIKE ERGONOMIC SYNTAX SUGAR PREPROCESSORS ---

fn count_braces_in_line(line: &str) -> i32 {
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

fn generate_if_else(expr: &str, arms: &[MatchArm]) -> String {
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

fn preprocess_match_blocks(source: &str) -> String {
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
                    let expr = trimmed
                        .strip_prefix("match ")
                        .unwrap()
                        .split_once('{')
                        .unwrap()
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
                        let (pat, rest) = trimmed.split_once("=>").unwrap();
                        let pat = pat.trim().to_string();
                        let rest = rest.trim();
                        *current_pattern = Some(pat);
                        current_body.clear();
                        *brace_depth = 0;
                        if rest.contains('{') {
                            *brace_depth += count_braces_in_line(rest);
                            let body_part = rest.trim_start_matches('{').trim();
                            if !body_part.is_empty() {
                                current_body.push(body_part.to_string());
                            }
                        } else {
                            current_body.push(rest.to_string());
                            // Single line case without braces
                            arms.push(MatchArm {
                                pattern: current_pattern.take().unwrap(),
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
                            pattern: current_pattern.take().unwrap(),
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

fn parse_let_binding(line: &str) -> Option<(String, String, String)> {
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
    if !name.is_empty() && name.chars().next()?.is_alphabetic() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Some((name.to_string(), ty.to_string(), rhs.to_string()))
    } else {
        None
    }
}

fn preprocess_let_bindings(source: &str) -> (String, Vec<String>) {
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

fn inject_declarations(source: &str, decls: &[String]) -> String {
    if decls.is_empty() {
        return source.to_string();
    }
    let mut out = String::new();
    let mut injected = false;
    for line in source.lines() {
        out.push_str(line);
        out.push('\n');
        if !injected && ((line.trim().starts_with("module ") && line.contains('{')) || line.trim().starts_with("reflect {")) {
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
