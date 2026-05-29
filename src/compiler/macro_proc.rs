//! Ergonomic macro processor for MIRR source.
//! This module handles text-level expansion of ergonomic syntax
//! into the AST parser's strict grammar.

#![forbid(unsafe_code)]

use super::inline_helpers::inline_types_functions;
use super::macro_helpers::{
    inject_declarations, inline_let_bindings, preprocess_if_else_reflexes, preprocess_let_bindings,
    preprocess_match_blocks,
};

#[derive(Debug, PartialEq, Clone)]
enum ParserState {
    TopLevel,
    AwaitingSignalsBrace,
    InSignals,
    InLoop {
        var: String,
        start: i32,
        end: i32,
        body: Vec<String>,
    },
    InReflex {
        injected_on_always: bool,
    },
    InReflexLoop {
        var: String,
        start: i32,
        end: i32,
        body: Vec<String>,
        depth: i32,
    },
    InTopLevelLoop {
        var: String,
        start: i32,
        end: i32,
        body: Vec<String>,
        depth: i32,
    },
    /// Verbatim pass-through for `def` pattern blocks.
    /// The FSM must not apply any transformation to def body lines.
    InPatternDef {
        depth: i32,
    },
}

pub fn expand_macros(source: &str) -> String {
    let inlined = inline_types_functions(source);
    let mut current = inlined;
    let max_iterations = 4;

    for _ in 0..max_iterations {
        let next = expand_macros_pass(&current);
        if next == current {
            if std::env::var("MIRR_DUMP_EXPANDED").is_ok() {
                if let Err(e) = std::fs::write("DEBUG_EXPANDED.mirr", &next) {
                    eprintln!("Warning: failed to write DEBUG_EXPANDED.mirr: {}", e);
                }
            }
            return next;
        }
        current = next;
    }
    if std::env::var("MIRR_DUMP_EXPANDED").is_ok() {
        if let Err(e) = std::fs::write("DEBUG_EXPANDED.mirr", &current) {
            eprintln!("Warning: failed to write DEBUG_EXPANDED.mirr: {}", e);
        }
    }
    current
}

fn expand_macros_pass(source: &str) -> String {
    // Pass 1: Parse and preprocess match blocks into standard if/else if/else structures
    let matched_source = preprocess_match_blocks(source);

    // Inline let-bindings (e.g. is_reflexive) before extracting complex conditions
    let inlined_source = inline_let_bindings(&matched_source);

    // Pass 0: Parse and extract complex conditional expressions inside reflexes to standard guards
    let (if_source, if_decls) = preprocess_if_else_reflexes(&inlined_source);
    let if_injected = inject_declarations(&if_source, &if_decls);

    // Pass 2: Parse and extract local let-bindings into top-level signals
    let (let_source, decls) = preprocess_let_bindings(&if_injected);

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
                // Detect start of a `def` pattern block — pass through verbatim.
                if trimmed.starts_with("def ") && trimmed.contains('(') {
                    // Count the opening brace depth on this line (ignoring ${...}).
                    let open_depth = count_structural_braces(trimmed);
                    result.push_str(line);
                    result.push('\n');
                    if open_depth > 0 {
                        state = ParserState::InPatternDef { depth: open_depth };
                    }
                    continue;
                }

                if trimmed.starts_with("for ") {
                    if let Some(loop_info) = parse_for_loop_header(trimmed) {
                        state = ParserState::InTopLevelLoop {
                            var: loop_info.var,
                            start: loop_info.start,
                            end: loop_info.end,
                            body: Vec::new(),
                            depth: 1,
                        };
                        result.push('\n');
                        continue;
                    }
                }

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
                        let (prefix, rest) = trimmed.split_once('{').unwrap_or((trimmed, ""));
                        result.push_str(prefix);
                        result.push_str(" {\n");
                        let mut inner = rest.trim();
                        if inner.ends_with('}') {
                            inner = inner[..inner.len() - 1].trim();
                        }
                        if !inner.is_empty() {
                            let inner_trimmed = inner.trim();
                            let has_conditional = inner_trimmed.starts_with("on ")
                                || inner_trimmed.starts_with("if ")
                                || inner_trimmed.starts_with("else");
                            if inner.contains('=') && !has_conditional {
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
                        state = ParserState::InReflex { injected_on_always: false };
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
            ParserState::InPatternDef { ref mut depth } => {
                // Emit lines verbatim. Track brace depth to know when the def block closes.
                let delta = count_structural_braces(trimmed);
                *depth += delta;
                result.push_str(line);
                result.push('\n');
                if *depth <= 0 {
                    state = ParserState::TopLevel;
                }
            }
            ParserState::InSignals => {
                if trimmed.starts_with('}') {
                    state = ParserState::TopLevel;
                    result.push('\n');
                } else if trimmed.starts_with("for ") {
                    if let Some(loop_info) = parse_for_loop_header(trimmed) {
                        state = ParserState::InLoop {
                            var: loop_info.var,
                            start: loop_info.start,
                            end: loop_info.end,
                            body: Vec::new(),
                        };
                        result.push('\n');
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
                            let expanded = unroll_line(line, var, i);
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
            ParserState::InReflex { ref mut injected_on_always } => {
                if trimmed.starts_with("for ") {
                    if let Some(loop_info) = parse_for_loop_header(trimmed) {
                        state = ParserState::InReflexLoop {
                            var: loop_info.var,
                            start: loop_info.start,
                            end: loop_info.end,
                            body: Vec::new(),
                            depth: 1,
                        };
                        result.push('\n');
                        continue;
                    }
                }

                let delta = count_braces(trimmed);
                reflex_depth += delta;

                // Flexible check for 'if', 'else if', 'else' with possible leading '}'
                let mut check_line = trimmed;
                if check_line.starts_with('}') {
                    check_line = check_line.strip_prefix('}').unwrap_or(check_line).trim();
                }

                if (check_line.starts_with("on ")
                    || check_line.starts_with("if ")
                    || check_line.starts_with("else"))
                    && check_line.contains('{')
                {
                    if *injected_on_always {
                        result.push_str("}\n");
                        *injected_on_always = false;
                    }
                    in_on_block = true;

                    if check_line.starts_with("if ") {
                        let cond = check_line
                            .strip_prefix("if ")
                            .unwrap_or(check_line)
                            .trim_end_matches('{')
                            .trim();
                        result.push_str("on ");
                        result.push_str(cond);
                        result.push_str(" {\n");
                        continue;
                    } else if check_line.starts_with("else if ") {
                        let cond = check_line
                            .strip_prefix("else if ")
                            .unwrap_or(check_line)
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
                    if *injected_on_always {
                        result.push_str("}\n");
                    }
                    state = ParserState::TopLevel;
                    result.push_str(line);
                    result.push('\n');
                } else if trimmed == "}" && in_on_block {
                    if *injected_on_always {
                        result.push_str("}\n");
                        *injected_on_always = false;
                    }
                    in_on_block = false;
                    result.push_str(line);
                    result.push('\n');
                } else if trimmed.contains('=') {
                    if !in_on_block {
                        result.push_str("on always {\n");
                        result.push_str(line);
                        result.push('\n');
                        *injected_on_always = true;
                        in_on_block = true;
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                } else {
                    result.push_str(line);
                    result.push('\n');
                }
            }
            ParserState::InReflexLoop { ref var, start, end, ref mut body, ref mut depth } => {
                let delta = count_braces(trimmed);
                *depth += delta;
                if *depth <= 0 {
                    // Unroll the loop body
                    for i in start..end {
                        for body_line in body.iter() {
                            let expanded = unroll_line(body_line, var, i);
                            result.push_str(&expanded);
                            result.push('\n');
                        }
                    }
                    state = ParserState::InReflex { injected_on_always: false };
                } else {
                    body.push(line.to_string());
                }
            }
            ParserState::InTopLevelLoop { ref var, start, end, ref mut body, ref mut depth } => {
                let delta = count_braces(trimmed);
                *depth += delta;
                if *depth <= 0 {
                    // Unroll the loop body
                    for i in start..end {
                        for body_line in body.iter() {
                            let expanded = unroll_line(body_line, var, i);
                            result.push_str(&expanded);
                            result.push('\n');
                        }
                    }
                    state = ParserState::TopLevel;
                } else {
                    body.push(line.to_string());
                }
            }
        }
    }

    result
}

struct LoopInfo {
    var: String,
    start: i32,
    end: i32,
}

fn parse_for_loop_header(line: &str) -> Option<LoopInfo> {
    let trimmed = line.trim();
    if !trimmed.starts_with("for ") {
        return None;
    }

    // Expected: for <var> in <start>..<end> {
    let after_for = trimmed.strip_prefix("for ")?.trim();
    let (var, rest) = after_for.split_once(" in ")?;
    let var = var.trim().to_string();

    // The range part might contain the opening brace '{'
    let range_part = rest.split_whitespace().next()?.trim_end_matches('{');
    let (start_str, end_str) = range_part.split_once("..")?;

    let start = start_str.trim().parse().ok()?;
    let end = end_str.trim().parse().ok()?;

    Some(LoopInfo { var, start, end })
}

fn unroll_line(line: &str, var: &str, i: i32) -> String {
    let mut expanded = line.to_string();

    // Support both suffix format s[i] -> s_0
    let placeholder = format!("[{}]", var);
    let replacement = format!("_{}", i);
    expanded = expanded.replace(&placeholder, &replacement);

    // And interpolation format ${i} -> 0
    let placeholder2 = format!("${{{}}}", var);
    let replacement2 = format!("{}", i);
    expanded = expanded.replace(&placeholder2, &replacement2);

    expanded
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

/// Count the net structural brace depth on a single line, skipping `${…}` template
/// substitutions so that `guard g_${n} {` counts as +1, not +2.
fn count_structural_braces(line: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            for inner in chars.by_ref() {
                if inner == '}' {
                    break;
                }
            }
            continue;
        }
        if ch == '{' {
            depth += 1;
        } else if ch == '}' {
            depth -= 1;
        }
    }
    depth
}
