//! Module-level parser for MIRR source files.
//!
//! Parses the top-level `module` block and all nested declarations: signals,
//! guards, reflexes, properties, and pattern calls. Also dispatches `def` blocks
//! to the pattern parser.

#![forbid(unsafe_code)]

mod formula_parser;
mod guard_reflex;

// Re-export parser utilities for submodule access.
pub(crate) use super::expr_parser::parse_expression;
pub(crate) use super::skip_empty_and_comments;
pub(crate) use super::tokenize_signal_decl;

use super::pattern_parser::{is_pattern_call_line, parse_pattern_call, parse_pattern_def};
use crate::ast::pattern::PatternDef;
use crate::ast::program::{MirrProgram, Module, SignalDecl};
use crate::ast::types::ExtendedType;
use crate::error::MirrError;
use crate::span::Span;

/// Maximum number of top-level `def` blocks allowed.
const MAX_PATTERN_DEFS: usize = 64;

/// Parse a MIRR source file into an in-memory representation.
///
/// Handles zero or more top-level `def` blocks before the `module` declaration.
pub fn parse_mirr(source: &str) -> Result<MirrProgram, MirrError> {
    let lines: Vec<&str> = source.lines().collect();
    let mut index = 0usize;

    // Parse top-level `def` blocks (bounded by MAX_PATTERN_DEFS).
    let mut patterns: Vec<PatternDef> = Vec::new();
    let mut def_count = 0usize;

    loop {
        skip_empty_and_comments(&lines, &mut index);
        if index >= lines.len() {
            break;
        }
        let line = lines[index].trim();
        if line.starts_with("def ") {
            if def_count >= MAX_PATTERN_DEFS {
                return Err(MirrError::PatternError {
                    message: format!(
                        "[E400] Too many pattern definitions (max {MAX_PATTERN_DEFS})."
                    ),
                    span: None,
                });
            }
            let pat = parse_pattern_def(&lines, &mut index)?;
            patterns.push(pat);
            def_count += 1;
        } else {
            break;
        }
    }

    skip_empty_and_comments(&lines, &mut index);

    if index >= lines.len() {
        return Err(MirrError::parse_error("[E101] MIRR source is empty."));
    }

    let module = parse_module(&lines, &mut index)?;

    Ok(MirrProgram { patterns, module })
}

fn parse_module(lines: &[&str], index: &mut usize) -> Result<Module, MirrError> {
    if *index >= lines.len() {
        return Err(MirrError::parse_error(
            "[E102] Expected 'module' declaration but found end of file.",
        ));
    }

    let module_start = *index;
    let header = lines[*index].trim();

    if !header.starts_with("module ") {
        return Err(MirrError::parse_error(format!(
            "[E103] Expected 'module' declaration, found: {header}"
        ))
        .with_span(Some(Span::full_line(*index as u32))));
    }

    let after_keyword = header
        .strip_prefix("module ")
        .ok_or_else(|| MirrError::parse_error("[E104] Malformed module declaration."))?;

    let (name_part, _) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::parse_error("[E105] Module name cannot be empty.")
            .with_span(Some(Span::full_line(*index as u32))));
    }

    let mut module = Module {
        name: name.to_string(),
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
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
            module.span = Some(Span::multi_line(module_start as u32, *index as u32));
            *index += 1;
            return Ok(module);
        } else if line.starts_with("signal ") {
            let signal = parse_signal(line, *index)?;
            module.signals.push(signal);
            *index += 1;
        } else if line.starts_with("guard ") {
            let guard = guard_reflex::parse_guard(lines, index)?;
            module.guards.push(guard);
        } else if line.starts_with("reflex ") {
            let reflex = guard_reflex::parse_reflex(lines, index)?;
            module.reflexes.push(reflex);
        } else if line.starts_with("property ") {
            let prop = formula_parser::parse_property(lines, index)?;
            module.properties.push(prop);
        } else if is_pattern_call_line(line) {
            let mut call = parse_pattern_call(line)?;
            call.span = Some(Span::full_line(*index as u32));
            module.pattern_calls.push(call);
            *index += 1;
        } else {
            return Err(MirrError::parse_error(format!(
                "[E107] Unexpected line inside module '{}': {}",
                module.name, line
            ))
            .with_span(Some(Span::full_line(*index as u32))));
        }
    }

    Err(MirrError::parse_error(format!(
        "[E106] Module '{}' was not closed with '}}'.",
        module.name
    )))
}

fn parse_signal(line: &str, line_index: usize) -> Result<SignalDecl, MirrError> {
    let span = Some(Span::full_line(line_index as u32));
    let after_keyword = line.strip_prefix("signal ").ok_or_else(|| {
        MirrError::parse_error("[E108] Malformed signal declaration.").with_span(span)
    })?;

    let trimmed = after_keyword.trim();
    let without_semicolon = trimmed
        .strip_suffix(';')
        .ok_or_else(|| MirrError::parse_error("[E109] Signal declaration must end with ';'."))?;

    let (name_part, rest) = without_semicolon
        .split_once(':')
        .ok_or_else(|| MirrError::parse_error("[E110] Signal declaration must contain ':'."))?;

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::parse_error("[E111] Signal name cannot be empty."));
    }

    let rest = rest.trim();

    // Delegate to the shared MEGA-1 tokenizer which handles:
    //   <kind> [linear] [stateful|pure] <base_type> [where <refinement>] [@clock] [#phantom]
    // Backward compatible: plain `<kind> <type>` produces default annotations.
    let parsed = tokenize_signal_decl(rest).map_err(|e| e.with_span(span))?;

    Ok(SignalDecl {
        name: name.to_string(),
        kind: parsed.kind,
        ty: ExtendedType::new(parsed.ty, parsed.annotations),
        origin: None,
        span,
    })
}
