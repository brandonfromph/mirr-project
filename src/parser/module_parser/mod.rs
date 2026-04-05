//! Module-level parser for MIRR source files.
//!
//! Parses the top-level `module` block and all nested declarations: signals,
//! guards, reflexes, properties, and pattern calls. Also dispatches `def` blocks
//! to the pattern parser.

#![forbid(unsafe_code)]

mod formula_parser;
mod guard_reflex;

use std::collections::HashMap;

// Re-export parser utilities for submodule access.
pub(crate) use super::expr_parser::parse_expression;
pub(crate) use super::parse_signal_type_str;
pub(crate) use super::skip_empty_and_comments;
pub(crate) use super::tokenize_signal_decl;

use super::pattern_parser::{is_pattern_call_line, parse_pattern_call, parse_pattern_def};
use crate::ast::pattern::PatternDef;
use crate::ast::program::{ImportDecl, MirrProgram, Module, SignalDecl};
use crate::ast::types::ExtendedType;
use crate::ast::types::{SignalType, MAX_STRUCT_FIELDS};
use crate::error::MirrError;
use crate::span::Span;

/// Maximum number of top-level `def` blocks allowed.
const MAX_PATTERN_DEFS: usize = 64;

/// Maximum number of import declarations allowed.
const MAX_IMPORTS: usize = 16;

/// Maximum number of top-level struct declarations retained during parse.
const MAX_STRUCT_DEFS: usize = 64;

/// Parse a MIRR source file into an in-memory representation.
///
/// Handles zero or more top-level `import` and `def` blocks before the `module` declaration.
pub fn parse_mirr(source: &str) -> Result<MirrProgram, MirrError> {
    let lines: Vec<&str> = source.lines().collect();
    let mut index = 0usize;

    // Parse top-level `import` declarations (bounded by MAX_IMPORTS).
    let mut imports: Vec<ImportDecl> = Vec::new();
    let mut import_count = 0usize;

    loop {
        skip_empty_and_comments(&lines, &mut index);
        if index >= lines.len() {
            break;
        }
        let line = lines[index].trim();
        if line.starts_with("import ") {
            if import_count >= MAX_IMPORTS {
                return Err(MirrError::parse_error(format!(
                    "[E802] Too many import declarations (max {MAX_IMPORTS})."
                )));
            }
            let import = parse_import(line, index)?;
            imports.push(import);
            import_count += 1;
            index += 1;
        } else {
            break;
        }
    }

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

    let mut struct_defs: HashMap<String, Vec<(String, SignalType)>> = HashMap::new();
    while index < lines.len() {
        let line = lines[index].trim();

        if line.starts_with("struct ") {
            if struct_defs.len() >= MAX_STRUCT_DEFS {
                return Err(MirrError::parse_error(format!(
                    "[E804] Too many top-level struct declarations (max {MAX_STRUCT_DEFS})."
                )));
            }
            let (name, fields) = parse_top_level_struct(&lines, &mut index)?;
            struct_defs.insert(name, fields);
            skip_empty_and_comments(&lines, &mut index);
            continue;
        }

        if line.starts_with("interface ") {
            skip_top_level_block(&lines, &mut index)?;
            skip_empty_and_comments(&lines, &mut index);
            continue;
        }
        break;
    }

    if index >= lines.len() {
        return Err(MirrError::parse_error("[E101] MIRR source is empty."));
    }

    let mut module = parse_module(&lines, &mut index)?;
    hydrate_struct_signal_fields(&mut module, &struct_defs);

    Ok(MirrProgram { patterns, imports, module })
}

fn parse_top_level_struct(
    lines: &[&str],
    index: &mut usize,
) -> Result<(String, Vec<(String, SignalType)>), MirrError> {
    if *index >= lines.len() {
        return Err(MirrError::parse_error("[E805] Expected struct declaration header."));
    }

    let header = lines[*index].trim();
    let after_struct = header
        .strip_prefix("struct ")
        .ok_or_else(|| MirrError::parse_error("[E805] Malformed struct declaration."))?;

    let (name_raw, has_open_brace) = if let Some((name_part, _)) = after_struct.split_once('{') {
        (name_part.trim(), true)
    } else {
        (after_struct.trim(), false)
    };

    if name_raw.is_empty() {
        return Err(MirrError::parse_error("[E806] Struct name cannot be empty."));
    }
    *index += 1;
    if !has_open_brace {
        while *index < lines.len() {
            let line = lines[*index].trim();
            if line.is_empty() || line.starts_with("//") {
                *index += 1;
                continue;
            }
            if line == "{" {
                *index += 1;
                break;
            }
            return Err(MirrError::parse_error(format!(
                "[E807] Struct '{}' declaration must include '{{' before field declarations.",
                name_raw
            )));
        }
    }

    let mut fields: Vec<(String, SignalType)> = Vec::new();

    while *index < lines.len() {
        let line = lines[*index].trim();
        if line.is_empty() || line.starts_with("//") {
            *index += 1;
            continue;
        }

        if line == "}" {
            *index += 1;
            return Ok((name_raw.to_string(), fields));
        }

        if fields.len() >= MAX_STRUCT_FIELDS {
            return Err(MirrError::parse_error(format!(
                "[E808] Struct '{}' exceeds maximum field count ({}).",
                name_raw, MAX_STRUCT_FIELDS
            )));
        }

        let without_semicolon = line.strip_suffix(';').ok_or_else(|| {
            MirrError::parse_error(format!(
                "[E809] Struct '{}' field declaration must end with ';'.",
                name_raw
            ))
        })?;

        let (field_name_raw, field_ty_raw) =
            without_semicolon.split_once(':').ok_or_else(|| {
                MirrError::parse_error(format!(
                    "[E810] Struct '{}' field declaration must contain ':'.",
                    name_raw
                ))
            })?;

        let field_name = field_name_raw.trim();
        let field_ty_text = field_ty_raw.trim();

        if field_name.is_empty() {
            return Err(MirrError::parse_error(format!(
                "[E811] Struct '{}' field name cannot be empty.",
                name_raw
            )));
        }

        let field_ty = parse_signal_type_str(field_ty_text).ok_or_else(|| {
            MirrError::parse_error(format!(
                "[E812] Unknown struct field type '{}' in struct '{}'.",
                field_ty_text, name_raw
            ))
        })?;

        fields.push((field_name.to_string(), field_ty));
        *index += 1;
    }

    Err(MirrError::parse_error(format!("[E813] Struct '{}' was not closed with '}}'.", name_raw)))
}

fn hydrate_struct_signal_fields(
    module: &mut Module,
    struct_defs: &HashMap<String, Vec<(String, SignalType)>>,
) {
    for sig in &mut module.signals {
        if let SignalType::Struct { name, fields } = &mut sig.ty.core {
            if fields.is_empty() {
                if let Some(def_fields) = struct_defs.get(name) {
                    *fields = def_fields.clone();
                }
            }
        }
    }
}

/// Parse an import declaration line.
///
/// Grammar: `import "path" as alias;`
fn parse_import(line: &str, line_index: usize) -> Result<ImportDecl, MirrError> {
    let span = Some(Span::full_line(line_index as u32));
    let trimmed = line.trim();

    // Strip trailing semicolon.
    let without_semicolon = trimmed.strip_suffix(';').ok_or_else(|| {
        MirrError::parse_error("[E801] Import declaration must end with ';'.").with_span(span)
    })?;

    // Parse: import "path" as alias
    let after_import = without_semicolon.strip_prefix("import ").ok_or_else(|| {
        MirrError::parse_error("[E801] Malformed import declaration.").with_span(span)
    })?;

    let trimmed_after = after_import.trim();

    // Find the quoted path.
    let (path_part, rest) = if let Some(start) = trimmed_after.find('"') {
        let after_quote = &trimmed_after[start + 1..];
        if let Some(end) = after_quote.find('"') {
            let path = &after_quote[..end];
            let rest = after_quote[end + 1..].trim();
            (path.to_string(), rest)
        } else {
            return Err(MirrError::parse_error("[E801] Unterminated string in import path.")
                .with_span(span));
        }
    } else {
        return Err(
            MirrError::parse_error("[E801] Import path must be a quoted string.").with_span(span)
        );
    };

    // Parse: as alias
    let alias = if rest.starts_with("as ") {
        let alias_part = rest.strip_prefix("as ").unwrap().trim();
        if alias_part.is_empty() {
            return Err(
                MirrError::parse_error("[E801] Import alias cannot be empty.").with_span(span)
            );
        }
        alias_part.to_string()
    } else {
        return Err(MirrError::parse_error("[E801] Import must specify an alias with 'as'.")
            .with_span(span));
    };

    if path_part.is_empty() {
        return Err(MirrError::parse_error("[E803] Import path cannot be empty.").with_span(span));
    }

    Ok(ImportDecl { path: path_part, alias, span })
}

fn split_top_level_statements(body: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut brace_depth: i32 = 0;
    let mut current = String::new();
    let mut chars = body.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                brace_depth += 1;
                current.push(c);
            }
            '}' => {
                brace_depth = brace_depth.saturating_sub(1);
                current.push(c);

                if brace_depth == 0 {
                    // Flush completed block when top-level block closes.
                    let stmt = current.trim();
                    if !stmt.is_empty() {
                        result.push(stmt.to_string());
                    }
                    current.clear();

                    // Skip whitespace before next statement start.
                    while let Some(next_c) = chars.peek() {
                        if next_c.is_whitespace() {
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    continue;
                }
            }
            ';' if brace_depth == 0 => {
                let stmt = current.trim();
                if !stmt.is_empty() {
                    result.push(stmt.to_string());
                }
                current.clear();
                continue;
            }
            _ => {
                current.push(c);
            }
        }
    }

    let stmt = current.trim();
    if !stmt.is_empty() {
        result.push(stmt.to_string());
    }

    result
}

fn skip_top_level_block(lines: &[&str], index: &mut usize) -> Result<(), MirrError> {
    let mut depth = 0i32;
    while *index < lines.len() {
        let line = lines[*index];
        for c in line.chars() {
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
            }
        }
        *index += 1;
        if depth <= 0 {
            return Ok(());
        }
    }
    Err(MirrError::parse_error("[E106] Unclosed block declaration."))
}

fn parse_inline_guard(stmt: &str) -> Result<crate::ast::program::Guard, MirrError> {
    let trimmed = stmt.trim();
    let after_guard = trimmed
        .strip_prefix("guard ")
        .ok_or_else(|| MirrError::parse_error("[E120] Malformed inline guard declaration."))?;

    let open = after_guard.find('{').ok_or_else(|| {
        MirrError::parse_error("[E120] Malformed inline guard declaration: missing '{'.")
    })?;
    let close = after_guard.rfind('}').ok_or_else(|| {
        MirrError::parse_error("[E132] Malformed inline guard declaration: missing '}'.")
    })?;

    let name = after_guard[..open].trim();
    if name.is_empty() {
        return Err(MirrError::parse_error("[E121] Guard name cannot be empty."));
    }

    let body = after_guard[open + 1..close].trim();
    let body = body.strip_suffix(';').unwrap_or(body).trim();

    let when_prefix = "when ";
    let for_keyword = " for ";
    let cycles_suffix = " cycles";

    if !body.starts_with(when_prefix) || !body.contains(for_keyword) {
        return Err(MirrError::parse_error("[E123] Invalid inline guard body."));
    }

    let after_when = &body[when_prefix.len()..];
    let for_pos = after_when.find(for_keyword).ok_or_else(|| {
        MirrError::parse_error("[E123] Invalid inline guard body: missing 'for'.")
    })?;

    let condition = after_when[..for_pos].trim();
    let after_for = after_when[for_pos + for_keyword.len()..].trim();
    let cycles_text = after_for.strip_suffix(cycles_suffix).unwrap_or(after_for).trim();
    let cycles: u64 = cycles_text.parse().map_err(|_| {
        MirrError::parse_error(format!(
            "[E130] Invalid cycle count in guard '{}': {}",
            name, cycles_text
        ))
    })?;

    let lines = [
        format!("guard {name} {{"),
        format!("when {condition}"),
        format!("for {cycles} cycles"),
        "}".to_string(),
    ];
    let line_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    let mut idx = 0;
    guard_reflex::parse_guard(&line_refs, &mut idx)
}

fn parse_inline_reflex(stmt: &str) -> Result<crate::ast::program::Reflex, MirrError> {
    let trimmed = stmt.trim();
    let after_reflex = trimmed
        .strip_prefix("reflex ")
        .ok_or_else(|| MirrError::parse_error("[E138] Malformed inline reflex declaration."))?;

    let open = after_reflex.find('{').ok_or_else(|| {
        MirrError::parse_error("[E138] Malformed inline reflex declaration: missing '{'.")
    })?;
    let close = after_reflex.rfind('}').ok_or_else(|| {
        MirrError::parse_error("[E145] Malformed inline reflex declaration: missing '}'.")
    })?;

    let header = after_reflex[..open].trim();
    let body = after_reflex[open + 1..close].trim();

    let mut lines = vec![format!("reflex {header} {{")];

    for top_stmt in split_top_level_statements(body) {
        let top_stmt = top_stmt.trim();
        if top_stmt.is_empty() {
            continue;
        }

        if top_stmt.starts_with("on ") {
            let on_open = top_stmt.find('{').ok_or_else(|| {
                MirrError::parse_error("[E140] Malformed on clause in inline reflex.")
            })?;
            let on_close = top_stmt.rfind('}').ok_or_else(|| {
                MirrError::parse_error("[E140] Malformed on clause in inline reflex: missing '}'.")
            })?;

            let on_header = top_stmt[..on_open].trim();
            let on_body = top_stmt[on_open + 1..on_close].trim();

            lines.push(format!("{on_header} {{"));
            for assign in split_top_level_statements(on_body) {
                let assign = assign.trim();
                if assign.is_empty() {
                    continue;
                }
                lines.push(format!("{assign};"));
            }
            lines.push("}".to_string());
        } else {
            return Err(MirrError::parse_error(
                "[E140] Inline reflex must contain an 'on' clause.",
            ));
        }
    }

    lines.push("}".to_string());

    let line_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    let mut idx = 0;
    guard_reflex::parse_reflex(&line_refs, &mut idx)
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

    let (name_part, inline_body) = match after_keyword.split_once('{') {
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

    // Handle inline body (single-line module declaration).
    let inline_body = inline_body.trim();
    if !inline_body.is_empty() {
        // Remove trailing '}' if present.
        let body_content = inline_body.strip_suffix('}').unwrap_or(inline_body).trim();
        if !body_content.is_empty() {
            // Split top-level statements while keeping block bodies intact.
            for stmt in split_top_level_statements(body_content) {
                let stmt_trimmed = stmt.trim();
                if stmt_trimmed.is_empty() {
                    continue;
                }
                if stmt_trimmed.starts_with("signal ") {
                    let full_stmt = format!("{stmt_trimmed};");
                    let signal = parse_signal(&full_stmt, module_start)?;
                    module.signals.push(signal);
                } else if stmt_trimmed.starts_with("guard ") {
                    let guard = parse_inline_guard(stmt_trimmed)?;
                    module.guards.push(guard);
                } else if stmt_trimmed.starts_with("reflex ") {
                    let reflex = parse_inline_reflex(stmt_trimmed)?;
                    module.reflexes.push(reflex);
                } else if is_pattern_call_line(stmt_trimmed) {
                    let mut call = parse_pattern_call(stmt_trimmed)?;
                    call.span = Some(Span::full_line(module_start as u32));
                    module.pattern_calls.push(call);
                } else {
                    return Err(MirrError::parse_error(format!(
                        "[E107] Unexpected statement inside module '{}': {stmt_trimmed}",
                        module.name
                    ))
                    .with_span(Some(Span::full_line(module_start as u32))));
                }
            }
        }
        // If the inline body ends with '}', the module is complete.
        if inline_body.ends_with('}') {
            module.span = Some(Span::multi_line(module_start as u32, *index as u32));
            return Ok(module);
        }
    }

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
