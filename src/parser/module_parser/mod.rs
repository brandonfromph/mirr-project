//! Module-level parser for MIRR source files.
//!
//! Parses the top-level `module` block and all nested declarations: signals,
//! guards, reflexes, properties, and pattern calls. Also dispatches `def` blocks
//! to the pattern parser.

#![forbid(unsafe_code)]

mod formula_parser;
mod guard_reflex;
pub mod macro_parser;

use std::collections::HashMap;

// Re-export parser utilities for submodule access.
pub(crate) use super::expr_parser::parse_expression;
pub(crate) use super::parse_signal_type_str;
pub(crate) use super::skip_empty_and_comments;
pub(crate) use super::tokenize_signal_decl;

use super::pattern_parser::{
    is_pattern_call_line, is_pattern_call_start, parse_pattern_call, parse_pattern_call_single,
    parse_pattern_def,
};
use crate::ast::pattern::PatternDef;
use crate::ast::program::{ImportDecl, MirrProgram, Module, SignalDecl, TargetConfig};
use crate::ast::types::ExtendedType;
use crate::ast::types::{SignalType, MAX_STRUCT_FIELDS};
use crate::diagnostic_builder::emit_at;
use crate::error::MirrError;
use crate::error_codes::ErrorCode;
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
    // Normalization: Ensure single-line sources (common in tests) are expanded
    // so the line-based parser can process them. We split by ';' and '{'/'}',
    // taking care NOT to split inside quotes (paths), comments, or
    // pattern interpolations (${param}).
    let mut expanded = String::with_capacity(source.len() * 2);
    let mut in_quotes = false;
    let mut in_comment = false;
    let mut in_interpolation = false;
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        if !in_comment && ch == '"' {
            in_quotes = !in_quotes;
        }
        if !in_quotes && ch == '/' && chars.peek() == Some(&'/') {
            in_comment = true;
        }
        if ch == '\n' {
            in_comment = false;
        }

        // Pattern interpolation: ${name}
        if !in_comment && !in_quotes && ch == '$' && chars.peek() == Some(&'{') {
            in_interpolation = true;
        }

        expanded.push(ch);

        if !in_quotes && !in_comment && !in_interpolation && (ch == ';' || ch == '{' || ch == '}') {
            // Only add newline if NOT at the very end and NOT already followed by newline
            match chars.peek() {
                Some(&'\n') | Some(&'\r') | None => {}
                _ => expanded.push('\n'),
            }
        }

        if in_interpolation && ch == '}' {
            in_interpolation = false;
        }
    }

    let lines: Vec<&str> = expanded.lines().map(|s| s.trim()).collect();
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
                return Err(emit_at(
                    ErrorCode::SExprUnexpectedToken,
                    format!("Too many import declarations (max {MAX_IMPORTS})."),
                    Span::full_line(index as u32),
                ));
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
        if line.starts_with("def ") || line.starts_with("pattern ") {
            if def_count >= MAX_PATTERN_DEFS {
                return Err(MirrError::PatternError {
                    message: format!(
                        "{} Too many pattern definitions (max {MAX_PATTERN_DEFS}).",
                        crate::error_codes::ErrorCode::PatternFallback.bracketed()
                    ),
                    span: Some(Span::full_line(index as u32)),
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

    // Parse optional `target` block.
    let mut target: Option<TargetConfig> = None;
    if index < lines.len() && lines[index].trim().starts_with("target") {
        target = Some(parse_target(&lines, &mut index)?);
        skip_empty_and_comments(&lines, &mut index);
    }

    let mut struct_defs: HashMap<String, Vec<(String, SignalType)>> = HashMap::new();
    while index < lines.len() {
        let line = lines[index].trim();

        if line.starts_with("struct ") {
            if struct_defs.len() >= MAX_STRUCT_DEFS {
                return Err(emit_at(
                    ErrorCode::SExprInvalidAtom,
                    format!("Too many top-level struct declarations (max {MAX_STRUCT_DEFS})."),
                    Span::full_line(index as u32),
                ));
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
        if lines.is_empty() {
            return Err(emit_at(
                ErrorCode::MirrSourceEmpty,
                "MIRR source is empty.",
                Span::full_line(0),
            ));
        } else {
            return Err(emit_at(
                ErrorCode::ExpectedModuleEof,
                "Expected 'module' declaration but found end of file (source is otherwise empty).",
                Span::full_line(index.saturating_sub(1) as u32),
            ));
        }
    }

    let mut module = parse_module(&lines, &mut index)?;
    hydrate_struct_signal_fields(&mut module, &struct_defs);

    Ok(MirrProgram { target, patterns, imports, module })
}

fn parse_top_level_struct(
    lines: &[&str],
    index: &mut usize,
) -> Result<(String, Vec<(String, SignalType)>), MirrError> {
    if *index >= lines.len() {
        return Err(emit_at(
            ErrorCode::StructHeaderExpected,
            "Expected struct declaration header.",
            Span::full_line(*index as u32),
        ));
    }

    let header = lines[*index].trim();
    let after_struct = header.strip_prefix("struct ").ok_or_else(|| {
        emit_at(
            ErrorCode::StructHeaderExpected,
            "Malformed struct declaration.",
            Span::full_line(*index as u32),
        )
    })?;

    let (name_raw, has_open_brace) = if let Some((name_part, _)) = after_struct.split_once('{') {
        (name_part.trim(), true)
    } else {
        (after_struct.trim(), false)
    };

    if name_raw.is_empty() {
        return Err(emit_at(
            ErrorCode::StructNameEmpty,
            "Struct name cannot be empty.",
            Span::full_line(*index as u32),
        ));
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
            return Err(emit_at(
                ErrorCode::StructOpenBrace,
                format!(
                    "Struct '{}' declaration must include '{{' before field declarations.",
                    name_raw
                ),
                Span::full_line(*index as u32),
            ));
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
            return Err(emit_at(
                ErrorCode::StructMaxFields,
                format!(
                    "Struct '{}' exceeds maximum field count ({}).",
                    name_raw, MAX_STRUCT_FIELDS
                ),
                Span::full_line(*index as u32),
            ));
        }

        let without_semicolon = line.strip_suffix(';').ok_or_else(|| {
            emit_at(
                ErrorCode::StructFieldSemicolon,
                format!("Struct '{}' field declaration must end with ';'.", name_raw),
                Span::full_line(*index as u32),
            )
        })?;

        let (field_name_raw, field_ty_raw) =
            without_semicolon.split_once(':').ok_or_else(|| {
                emit_at(
                    ErrorCode::StructFieldColon,
                    format!("Struct '{}' field declaration must contain ':'.", name_raw),
                    Span::full_line(*index as u32),
                )
            })?;

        let field_name = field_name_raw.trim();
        let field_ty_text = field_ty_raw.trim();

        if field_name.is_empty() {
            return Err(emit_at(
                ErrorCode::StructFieldNameEmpty,
                format!("Struct '{}' field name cannot be empty.", name_raw),
                Span::full_line(*index as u32),
            ));
        }

        let field_ty = parse_signal_type_str(field_ty_text).ok_or_else(|| {
            emit_at(
                ErrorCode::StructFieldTypeBad,
                format!("Unknown struct field type '{}' in struct '{}'.", field_ty_text, name_raw),
                Span::full_line(*index as u32),
            )
        })?;

        fields.push((field_name.to_string(), field_ty));
        *index += 1;
    }

    Err(emit_at(
        ErrorCode::SExprTooDeep,
        format!("Struct '{}' was not closed with '}}'.", name_raw),
        Span::full_line(*index as u32),
    ))
}

fn hydrate_type(ty: &mut SignalType, struct_defs: &HashMap<String, Vec<(String, SignalType)>>) {
    match ty {
        SignalType::Struct { name, fields } => {
            if fields.is_empty() {
                if let Some(def_fields) = struct_defs.get(name) {
                    *fields = def_fields.clone();
                }
            }
            for (_, field_ty) in fields.iter_mut() {
                hydrate_type(field_ty, struct_defs);
            }
        }
        SignalType::Array { element, .. } => {
            hydrate_type(element.as_mut(), struct_defs);
        }
        SignalType::Fifo { element, .. } => {
            hydrate_type(element.as_mut(), struct_defs);
        }
        _ => {}
    }
}

fn hydrate_struct_signal_fields(
    module: &mut Module,
    struct_defs: &HashMap<String, Vec<(String, SignalType)>>,
) {
    let mut resolved_defs = struct_defs.clone();
    for _ in 0..8 {
        let current_defs = resolved_defs.clone();
        for fields in resolved_defs.values_mut() {
            for (_, field_ty) in fields.iter_mut() {
                hydrate_type(field_ty, &current_defs);
            }
        }
    }

    for sig in &mut module.signals {
        hydrate_type(&mut sig.ty.core, &resolved_defs);
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
        emit_at(
            ErrorCode::SExprParseError,
            "Import declaration must end with ';'.",
            Span::full_line(line_index as u32),
        )
    })?;

    // Parse: import "path" as alias
    let after_import = without_semicolon.strip_prefix("import ").ok_or_else(|| {
        emit_at(
            ErrorCode::SExprParseError,
            "Malformed import declaration.",
            Span::full_line(line_index as u32),
        )
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
            return Err(emit_at(
                ErrorCode::SExprParseError,
                "Unterminated string in import path.",
                Span::full_line(line_index as u32),
            ));
        }
    } else {
        return Err(emit_at(
            ErrorCode::SExprParseError,
            "Import path must be a quoted string.",
            Span::full_line(line_index as u32),
        ));
    };

    // Parse: as alias
    let alias = if rest.starts_with("as ") {
        let alias_part = rest
            .strip_prefix("as ")
            .ok_or_else(|| {
                emit_at(
                    ErrorCode::SExprParseError,
                    "Import alias must follow 'as'.",
                    Span::full_line(line_index as u32),
                )
            })?
            .trim();
        if alias_part.is_empty() {
            return Err(emit_at(
                ErrorCode::SExprParseError,
                "Import alias cannot be empty.",
                Span::full_line(line_index as u32),
            ));
        }
        alias_part.to_string()
    } else {
        return Err(emit_at(
            ErrorCode::SExprParseError,
            "Import must specify an alias with 'as'.",
            Span::full_line(line_index as u32),
        ));
    };

    if path_part.is_empty() {
        return Err(emit_at(
            ErrorCode::SExprUnclosedParen,
            "Import path cannot be empty.",
            Span::full_line(line_index as u32),
        ));
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
    Err(emit_at(
        ErrorCode::ModuleNotClosed,
        "Unclosed block declaration.",
        Span::full_line(index.saturating_sub(1) as u32),
    ))
}

fn parse_target(lines: &[&str], index: &mut usize) -> Result<TargetConfig, MirrError> {
    let start_line = *index as u32;
    let line = lines[*index].trim();
    if !line.starts_with("target ") && line != "target" {
        return Err(emit_at(
            ErrorCode::SExprUnexpectedToken,
            "Expected 'target' block.",
            Span::full_line(start_line),
        ));
    }

    let mut name = if line.starts_with("target ") {
        line.strip_prefix("target ").unwrap().trim_end_matches('{').trim().to_string()
    } else {
        "unnamed".to_string()
    };

    if !lines[*index].contains('{') {
        *index += 1;
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() || !lines[*index].starts_with('{') {
            return Err(emit_at(
                ErrorCode::TargetOpenBrace,
                "Expected '{' after 'target'.",
                Span::full_line(*index as u32),
            ));
        }
    }
    *index += 1;

    let mut word_size = 64;
    let mut reg_bits = 10;
    let mut guard_bits = 6;

    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        let line = lines[*index].trim();
        if line == "}" {
            *index += 1;
            break;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            return Err(emit_at(
                ErrorCode::TargetPropertyInvalid,
                format!("Invalid target property: {line}"),
                Span::full_line(*index as u32),
            ));
        }

        let key = parts[0].trim();
        let val = parts[1].trim().trim_end_matches(';');

        match key {
            "name" => name = val.trim_matches('"').to_string(),
            "word_size" => {
                word_size = val.parse().map_err(|_| {
                    emit_at(
                        ErrorCode::TargetPropertyInvalid,
                        "Invalid word_size",
                        Span::full_line(*index as u32),
                    )
                })?
            }
            "reg_bits" => {
                reg_bits = val.parse().map_err(|_| {
                    emit_at(
                        ErrorCode::TargetPropertyInvalid,
                        "Invalid reg_bits",
                        Span::full_line(*index as u32),
                    )
                })?
            }
            "guard_bits" => {
                guard_bits = val.parse().map_err(|_| {
                    emit_at(
                        ErrorCode::TargetPropertyInvalid,
                        "Invalid guard_bits",
                        Span::full_line(*index as u32),
                    )
                })?
            }
            _ => {} // Ignore unknown properties
        }
        *index += 1;
    }

    Ok(TargetConfig {
        name,
        word_size,
        reg_bits,
        guard_bits,
        span: Some(Span::full_line(start_line)),
    })
}

fn parse_module(lines: &[&str], index: &mut usize) -> Result<Module, MirrError> {
    if *index >= lines.len() {
        return Err(emit_at(
            ErrorCode::ExpectedModuleEof,
            "Expected 'module' declaration but found end of file.",
            Span::full_line(index.saturating_sub(1) as u32),
        ));
    }

    let module_start = *index;
    let header = lines[*index].trim();

    if !header.starts_with("module ") {
        return Err(emit_at(
            ErrorCode::ExpectedModuleFound,
            format!("Expected 'module' declaration, found: '{header}'"),
            Span::full_line(*index as u32),
        ));
    }

    let after_keyword = header.strip_prefix("module ").ok_or_else(|| {
        emit_at(
            ErrorCode::MalformedModule,
            "Malformed module declaration.",
            Span::full_line(*index as u32),
        )
    })?;

    let (name_part, inline_body) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(emit_at(
            ErrorCode::ModuleNameEmpty,
            "Module name cannot be empty.",
            Span::full_line(*index as u32),
        ));
    }

    let mut unexpanded = crate::ast::macro_nodes::UnexpandedModule {
        name: name.to_string(),
        statements: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
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
                let dummy_lines_vec =
                    if !stmt_trimmed.ends_with(';') && !stmt_trimmed.ends_with('}') {
                        vec![format!("{stmt_trimmed};")]
                    } else {
                        vec![stmt_trimmed.to_string()]
                    };
                let dummy_lines_refs: Vec<&str> =
                    dummy_lines_vec.iter().map(|s| s.as_str()).collect();
                let mut local_index = 0;
                let parsed_stmts =
                    crate::parser::module_parser::macro_parser::parse_module_macro_stmts(
                        &dummy_lines_refs,
                        &mut local_index,
                    )?;
                unexpanded.statements.extend(parsed_stmts);

                if local_index < dummy_lines_refs.len() {
                    let remaining = dummy_lines_refs[local_index].trim();
                    if is_pattern_call_line(remaining) {
                        let mut call = parse_pattern_call_single(remaining)?;
                        call.span = Some(Span::full_line(module_start as u32));
                        unexpanded.pattern_calls.push(call);
                    }
                }
            }
        }
        // If the inline body ends with '}', the module is complete.
        if inline_body.ends_with('}') {
            let mut expanded = crate::expand::ast_expand::expand_module(unexpanded)?;
            expanded.span = Some(Span::multi_line(module_start as u32, *index as u32));
            return Ok(expanded);
        }
    }

    let parsed_stmts =
        crate::parser::module_parser::macro_parser::parse_module_macro_stmts(lines, index)?;
    unexpanded.statements.extend(parsed_stmts);

    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }

        let line = lines[*index];
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            *index += 1;
            continue;
        }

        let first_token = trimmed.split_whitespace().next().unwrap_or("");

        match first_token {
            "}" | "endmodule" | "end" => {
                // End of module.
                let mut expanded = crate::expand::ast_expand::expand_module(unexpanded)?;
                expanded.span = Some(Span::multi_line(module_start as u32, *index as u32));
                *index += 1;
                skip_empty_and_comments(lines, index);
                return Ok(expanded);
            }
            "property" => {
                let prop = formula_parser::parse_property(lines, index)?;
                unexpanded.properties.push(prop);
            }
            _ => {
                // Heuristic: if it looks like the start of a pattern call (ident + '(')
                // but didn't start with a known keyword, try parsing it as a call.
                if is_pattern_call_start(trimmed) {
                    let start_line = *index as u32;
                    let mut call = parse_pattern_call(lines, index)?;
                    call.span = Some(Span::full_line(start_line));
                    unexpanded.pattern_calls.push(call);
                    // *index is already incremented by parse_pattern_call
                } else {
                    return Err(emit_at(
                        ErrorCode::UnexpectedModuleLine,
                        format!("Unexpected line inside module '{}': {}", unexpanded.name, line),
                        Span::full_line(*index as u32),
                    ));
                }
            }
        }
    }

    Err(emit_at(
        ErrorCode::ModuleNotClosed,
        format!("Module '{}' was not closed with '}}'.", unexpanded.name),
        Span::full_line(index.saturating_sub(1) as u32),
    ))
}

pub fn parse_signal(line: &str, line_index: usize) -> Result<SignalDecl, MirrError> {
    let span = Span::full_line(line_index as u32);
    let trimmed_line = line.trim();

    // Support either "signal name: type;" or "name: type;"
    let after_keyword = if let Some(stripped) = trimmed_line.strip_prefix("signal ") {
        stripped
    } else {
        trimmed_line
    };

    let trimmed = after_keyword.trim();
    let without_semicolon = trimmed.strip_suffix(';').ok_or_else(|| {
        emit_at(ErrorCode::SignalMissingSemicolon, "Signal declaration must end with ';'.", span)
    })?;

    let (name_part, rest) = without_semicolon.split_once(':').ok_or_else(|| {
        emit_at(ErrorCode::SignalMissingColon, "Signal declaration must contain ':'.", span)
    })?;

    let name_part = name_part.trim();
    if name_part.is_empty() {
        return Err(emit_at(ErrorCode::SignalNameEmpty, "Signal name cannot be empty.", span));
    }

    // Ensure name_part is either a single identifier or a valid kind+name pair.
    // If it's something like "unknown s1", we should have caught "unknown" as a kind.
    let name_tokens: Vec<&str> = name_part.split_whitespace().collect();
    let name = if name_tokens.len() == 1 {
        name_tokens[0]
    } else if name_tokens.len() == 2 {
        let kind_token = name_tokens[0];
        if !matches!(kind_token, "in" | "out" | "internal" | "signal") {
            return Err(emit_at(
                ErrorCode::SignalUnknownKind,
                format!("Unknown signal kind '{}'", kind_token),
                span,
            ));
        }
        name_tokens[1]
    } else {
        return Err(emit_at(
            ErrorCode::SignalTooManyTokens,
            format!("Malformed signal header: '{}'", name_part),
            span,
        ));
    };

    let rest = rest.trim();

    // Delegate to the shared MEGA-1 tokenizer which handles:
    //   <kind> [linear] [stateful|pure] <base_type> [where <refinement>] @clock [#phantom]
    // Backward compatible: plain `<kind> <type>` produces default annotations.
    let parsed = tokenize_signal_decl(rest).map_err(|e| e.with_span(Some(span)))?;

    Ok(SignalDecl {
        name: name.to_string(),
        kind: parsed.kind,
        ty: ExtendedType::new(parsed.ty, parsed.annotations),
        origin: None,
        span: Some(span),
    })
}
