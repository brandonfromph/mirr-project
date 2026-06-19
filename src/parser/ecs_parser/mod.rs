//! ECS-native parser for MIRR source files.
//!
//! This module implements the "Direct-to-ECS" parsing phase (Phase B).
//! It parses source text directly into a `Registry`, bypassing the legacy AST.

#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalType, UnaryOp};
use crate::ecs::components::{
    AssignmentComponent, BinaryComponent, ConditionComponent, CyclesComponent, EntityId,
    EntityKind, KindComponent, LiteralComponent, NameComponent, PatternCallComponent,
    PatternDefComponent, PendingSignalRef, PrevComponent, PropertyComponent, ReflexComponent,
    SignalRefComponent, SpanComponent, TypeComponent, UnaryComponent,
};
use crate::ecs::Registry;
use crate::error::MirrError;
use crate::lexer::tokenizer::Token;
use crate::parser::{skip_empty_and_comments, tokenize_signal_decl};
use crate::span::Span;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Parse a MIRR source file directly into an ECS Registry.
pub fn parse_mirr_ecs(registry: &mut Registry, source: &str) -> Result<(), MirrError> {
    parse_mirr_ecs_with_base_dir(registry, source, None)
}

/// Parse a MIRR source file into an ECS Registry, with a base directory for resolving imports.
pub fn parse_mirr_ecs_with_base_dir(
    registry: &mut Registry,
    source: &str,
    base_dir: Option<&Path>,
) -> Result<(), MirrError> {
    let mut loaded_files = HashSet::new();
    parse_mirr_ecs_internal(registry, source, base_dir, &mut loaded_files, None)
}

/// Internal recursive parser with context.
fn parse_mirr_ecs_internal(
    registry: &mut Registry,
    source: &str,
    base_dir: Option<&Path>,
    loaded_files: &mut HashSet<PathBuf>,
    alias_prefix: Option<&str>,
) -> Result<(), MirrError> {
    // Normalization logic (identical to legacy)
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
        if !in_comment && !in_quotes && ch == '$' && chars.peek() == Some(&'{') {
            in_interpolation = true;
        }
        expanded.push(ch);
        if !in_quotes && !in_comment && !in_interpolation && (ch == ';' || ch == '{' || ch == '}') {
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

    let mut struct_defs: HashMap<String, Vec<(String, SignalType)>> = HashMap::new();

    loop {
        skip_empty_and_comments(&lines, &mut index);
        if index >= lines.len() {
            break;
        }
        let line = lines[index].trim();

        if line.starts_with("import ") {
            let (path_str, alias) = parse_import_line(line, index)?;
            if let Some(dir) = base_dir {
                let import_path = dir.join(&path_str);
                if let Ok(canonical) = import_path.canonicalize() {
                    if !loaded_files.contains(&canonical) {
                        loaded_files.insert(canonical.clone());
                        if let Ok(imported_source) = std::fs::read_to_string(&import_path) {
                            parse_mirr_ecs_internal(
                                registry,
                                &imported_source,
                                import_path.parent(),
                                loaded_files,
                                if alias.is_empty() { None } else { Some(&alias) },
                            )?;
                        }
                    }
                }
            }
            index += 1;
            continue;
        }

        if line.starts_with("def ") || line.starts_with("pattern ") {
            let pat = crate::parser::pattern_parser::parse_pattern_def(&lines, &mut index)?;
            let entity = registry.create_entity(&pat.name, KindComponent::PATTERN);
            registry.set_type(entity, TypeComponent::pattern(pat.clone()));
            registry.set_pattern_def(entity, PatternDefComponent(pat.clone()));

            if let Some(prefix) = alias_prefix {
                let qualified_name = format!("{}::{}", prefix, pat.name);
                registry.register_symbol(&qualified_name, entity);
            }
            continue;
        }

        if line.starts_with("struct ") {
            let (name, fields) = parse_top_level_struct_ecs(&lines, &mut index)?;
            struct_defs.insert(name, fields);
            continue;
        }

        if line.starts_with("target ") || line == "target" {
            let config = parse_target_ecs(&lines, &mut index)?;
            registry.target_config = Some(config);
            continue;
        }

        if line.starts_with("module ") {
            parse_module_ecs(registry, &lines, &mut index, &struct_defs)?;
            continue;
        }

        index += 1;
    }

    Ok(())
}

fn parse_import_line(line: &str, line_index: usize) -> Result<(String, String), MirrError> {
    let trimmed = line.trim();
    let without_semicolon = trimmed.strip_suffix(';').ok_or_else(|| {
        MirrError::parse_error(format!("Import at line {} must end with ';'.", line_index + 1))
    })?;

    let after_import = without_semicolon.strip_prefix("import ").unwrap();
    let trimmed_after = after_import.trim();

    if let Some((path_part, alias_part)) = trimmed_after.split_once(" as ") {
        let path = path_part.trim().trim_matches('"').to_string();
        let alias = alias_part.trim().to_string();
        Ok((path, alias))
    } else {
        let path = trimmed_after.trim().trim_matches('"').to_string();
        Ok((path, String::new()))
    }
}

fn parse_target_ecs(
    lines: &[&str],
    index: &mut usize,
) -> Result<crate::ast::program::TargetConfig, MirrError> {
    let start_line = *index as u32;
    let line = lines[*index].trim();

    let mut name = if let Some(stripped) = line.strip_prefix("target ") {
        stripped.trim_end_matches('{').trim().to_string()
    } else {
        "unnamed".to_string()
    };

    if !lines[*index].contains('{') {
        *index += 1;
        skip_empty_and_comments(lines, index);
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

        if let Some((key, val)) = line.split_once(':') {
            let key = key.trim();
            let val = val.trim().trim_end_matches(';');
            match key {
                "name" => name = val.trim_matches('"').to_string(),
                "word_size" => word_size = val.parse().unwrap_or(64),
                "reg_bits" => reg_bits = val.parse().unwrap_or(10),
                "guard_bits" => guard_bits = val.parse().unwrap_or(6),
                _ => {}
            }
        }
        *index += 1;
    }

    Ok(crate::ast::program::TargetConfig {
        name,
        word_size,
        reg_bits,
        guard_bits,
        span: Some(Span::full_line(start_line)),
    })
}

fn parse_top_level_struct_ecs(
    lines: &[&str],
    index: &mut usize,
) -> Result<(String, Vec<(String, SignalType)>), MirrError> {
    let header = lines[*index].trim();
    let after_struct = header.strip_prefix("struct ").unwrap();
    let (name, mut has_brace) = if let Some((n, _)) = after_struct.split_once('{') {
        (n.trim().to_string(), true)
    } else {
        (after_struct.trim().to_string(), false)
    };

    *index += 1;
    if !has_brace {
        skip_empty_and_comments(lines, index);
        if *index < lines.len() && lines[*index].trim().starts_with('{') {
            has_brace = true;
            *index += 1;
        }
    }

    let mut fields = Vec::new();
    if has_brace {
        while *index < lines.len() {
            skip_empty_and_comments(lines, index);
            let line = lines[*index].trim();
            if line == "}" {
                *index += 1;
                break;
            }
            if let Some((f_name, f_ty_str)) = line.split_once(':') {
                let f_name = f_name.trim().to_string();
                let f_ty_str = f_ty_str.trim().trim_end_matches(';');
                if let Some(f_ty) = crate::parser::parse_signal_type_str(f_ty_str) {
                    fields.push((f_name, f_ty));
                }
            }
            *index += 1;
        }
    }

    Ok((name, fields))
}

fn parse_module_ecs(
    registry: &mut Registry,
    lines: &[&str],
    index: &mut usize,
    struct_defs: &HashMap<String, Vec<(String, SignalType)>>,
) -> Result<EntityId, MirrError> {
    let header = lines[*index].trim();
    let after_keyword = header
        .strip_prefix("module ")
        .ok_or_else(|| MirrError::parse_error("Malformed module declaration."))?;

    let (name_part, _inline_body) = match after_keyword.split_once('{') {
        Some(parts) => parts,
        None => (after_keyword, ""),
    };

    let name = name_part.trim();
    if name.is_empty() {
        return Err(MirrError::parse_error("Module name cannot be empty."));
    }

    let module_entity = registry.create_entity(name, KindComponent::MODULE);
    let mod_name = name.to_string();
    *index += 1;

    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        if *index >= lines.len() {
            break;
        }
        let line = lines[*index].trim();
        if line == "}" {
            *index += 1;
            return Ok(module_entity);
        }

        if line.starts_with("signal ")
            || line.starts_with("in ")
            || line.starts_with("out ")
            || line.starts_with("internal ")
        {
            parse_signal_ecs(registry, module_entity, &mod_name, line, *index, struct_defs)?;
            *index += 1;
            continue;
        }

        if line.starts_with("guard ") {
            parse_guard_ecs(registry, module_entity, &mod_name, lines, index)?;
            continue;
        }

        if line.starts_with("reflex ") {
            parse_reflex_ecs(registry, module_entity, &mod_name, lines, index)?;
            continue;
        }

        if line.starts_with("property ") || line.starts_with("assert ") {
            parse_property_ecs(registry, module_entity, &mod_name, lines, index)?;
            continue;
        }

        if crate::parser::pattern_parser::is_pattern_call_start(line) {
            let pat_call = crate::parser::pattern_parser::parse_pattern_call(lines, index)?;
            let entity = registry.create_entity(
                &format!("{}_call", pat_call.pattern_name),
                KindComponent::PATTERN_CALL,
            );
            registry.set_module(entity, crate::ecs::components::ModuleComponent(module_entity));
            registry.set_pattern_call(entity, PatternCallComponent(pat_call));
            continue;
        }

        *index += 1;
    }

    Ok(module_entity)
}

fn parse_signal_ecs(
    registry: &mut Registry,
    module_entity: EntityId,
    mod_name: &str,
    line: &str,
    line_index: usize,
    struct_defs: &HashMap<String, Vec<(String, SignalType)>>,
) -> Result<EntityId, MirrError> {
    let span = Span::full_line(line_index as u32);
    let trimmed = line.trim();

    let after_keyword =
        if let Some(stripped) = trimmed.strip_prefix("signal ") { stripped } else { trimmed };
    let without_semicolon = after_keyword
        .trim()
        .strip_suffix(';')
        .ok_or_else(|| MirrError::parse_error("Signal declaration must end with ';'."))?;

    let (name_part, rest) = without_semicolon
        .split_once(':')
        .ok_or_else(|| MirrError::parse_error("Signal declaration must contain ':'."))?;

    let name_part = name_part.trim();
    let name_tokens: Vec<&str> = name_part.split_whitespace().collect();
    let name = if name_tokens.len() == 1 {
        name_tokens[0]
    } else if name_tokens.len() == 2 {
        name_tokens[1]
    } else {
        return Err(MirrError::parse_error("Malformed signal header."));
    };

    let tokenized = tokenize_signal_decl(rest.trim()).map_err(|e| e.with_span(Some(span)))?;

    let mut ty = tokenized.ty;
    hydrate_signal_type(&mut ty, struct_defs);

    let entity = registry.create_entity(name, KindComponent(EntityKind::SIGNAL(tokenized.kind)));
    registry.register_symbol(&format!("{}::{}", mod_name, name), entity);

    let ext_type = ExtendedType { core: ty, annotations: tokenized.annotations };
    registry.set_type(entity, TypeComponent(ext_type));
    registry.set_span(entity, SpanComponent(span));
    registry.set_parent(entity, module_entity);

    Ok(entity)
}

fn hydrate_signal_type(
    ty: &mut SignalType,
    struct_defs: &HashMap<String, Vec<(String, SignalType)>>,
) {
    match ty {
        SignalType::Array { element, .. } => hydrate_signal_type(element, struct_defs),
        SignalType::Struct { name, fields } => {
            if fields.is_empty() {
                if let Some(def_fields) = struct_defs.get(name) {
                    *fields = def_fields.clone();
                }
            }
            for (_, f_ty) in fields {
                hydrate_signal_type(f_ty, struct_defs);
            }
        }
        _ => {}
    }
}

fn parse_guard_ecs(
    registry: &mut Registry,
    module_entity: EntityId,
    mod_name: &str,
    lines: &[&str],
    index: &mut usize,
) -> Result<EntityId, MirrError> {
    let header = lines[*index].trim();
    let after_guard = header
        .strip_prefix("guard ")
        .ok_or_else(|| MirrError::parse_error("Malformed guard declaration."))?;

    let name = after_guard.trim().trim_end_matches('{').trim();
    if name.is_empty() {
        return Err(MirrError::parse_error("Guard name cannot be empty."));
    }

    *index += 1;

    let mut condition_ent = None;
    let mut cycles = 1u64;

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

        if let Some(rest) = line.strip_prefix("when ") {
            let mut expr_str = rest.trim_end_matches(';').trim();
            if let Some(idx) = expr_str.find(" for ") {
                let cycles_str = &expr_str[idx + 5..].trim();
                let parts: Vec<&str> = cycles_str.split_whitespace().collect();
                if parts.len() >= 2 && parts[1].starts_with("cycles") {
                    cycles = parts[0].parse().map_err(|_| MirrError::parse_error("Invalid cycle count."))?;
                }
                expr_str = expr_str[..idx].trim();
            }
            condition_ent = Some(parse_expression_ecs(registry, expr_str)?);
        } else if let Some(rest) = line.strip_prefix("for ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 && parts[1].starts_with("cycles") {
                cycles =
                    parts[0].parse().map_err(|_| MirrError::parse_error("Invalid cycle count."))?;
            }
        }
        *index += 1;
    }

    let cond_ent =
        condition_ent.ok_or_else(|| MirrError::parse_error("Guard missing 'when' condition."))?;

    let guard_id = registry.create_entity(name, KindComponent(EntityKind::GUARD));
    registry.register_symbol(&format!("{}::{}", mod_name, name), guard_id);
    registry.set_condition(guard_id, ConditionComponent(cond_ent));
    registry.set_cycle(guard_id, CyclesComponent(cycles));
    registry.set_parent(guard_id, module_entity);

    Ok(guard_id)
}

fn parse_reflex_ecs(
    registry: &mut Registry,
    module_entity: EntityId,
    mod_name: &str,
    lines: &[&str],
    index: &mut usize,
) -> Result<EntityId, MirrError> {
    let header = lines[*index].trim();
    let after_reflex = header
        .strip_prefix("reflex ")
        .ok_or_else(|| MirrError::parse_error("Malformed reflex declaration."))?;

    let name = after_reflex.trim().trim_end_matches('{').trim();
    if name.is_empty() {
        return Err(MirrError::parse_error("Reflex name cannot be empty."));
    }

    *index += 1;

    let mut guard_ents = Vec::new();
    let mut assignment_ents = Vec::new();

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

        if let Some(rest) = line.strip_prefix("on ") {
            let guard_list_part = rest.trim_end_matches('{').trim();
            for g_name in guard_list_part.split(',') {
                let g_name = g_name.trim();
                if g_name == "always" {
                    let always_ent = if let Some(ent) = registry.get_entity_by_name("always") {
                        ent
                    } else {
                        let ent =
                            registry.create_entity("always", KindComponent(EntityKind::GUARD));
                        registry.set_cycle(ent, CyclesComponent(0));
                        ent
                    };
                    guard_ents.push(always_ent);
                } else if let Some(g_ent) = registry.get_entity_by_name(g_name) {
                    guard_ents.push(g_ent);
                }
            }

            if !rest.contains('{') {
                *index += 1;
                skip_empty_and_comments(lines, index);
            }
            *index += 1;

            while *index < lines.len() {
                skip_empty_and_comments(lines, index);
                let inner_line = lines[*index].trim();
                if inner_line == "}" {
                    break;
                }
                if let Some((target_name, expr_str)) = inner_line.split_once('=') {
                    let target_name = target_name.trim();
                    let expr_str = expr_str.trim_end_matches(';').trim();
                    let target_ent = registry.get_entity_by_name(target_name).ok_or_else(|| {
                        MirrError::parse_error(format!(
                            "Assignment target signal '{}' not found.",
                            target_name
                        ))
                    })?;
                    let rvalue_ent = parse_expression_ecs(registry, expr_str)?;
                    let assign_id = registry.next_id();
                    let interned_name =
                        registry.interner.intern(&format!("_assign_{}", assign_id.0));
                    registry.set_name(assign_id, NameComponent(interned_name));
                    registry.set_kind(assign_id, KindComponent(EntityKind::ASSIGNMENT));
                    registry.set_assignment(
                        assign_id,
                        AssignmentComponent { target: target_ent, value: rvalue_ent },
                    );
                    assignment_ents.push(assign_id);
                }
                *index += 1;
            }
        }
        *index += 1;
    }

    let reflex_id = registry.create_entity(name, KindComponent(EntityKind::REFLEX));
    registry.register_symbol(&format!("{}::{}", mod_name, name), reflex_id);
    registry.set_reflex(
        reflex_id,
        ReflexComponent { guards: guard_ents, assignments: assignment_ents, origin: None },
    );
    registry.set_parent(reflex_id, module_entity);

    Ok(reflex_id)
}

fn parse_property_ecs(
    registry: &mut Registry,
    module_entity: EntityId,
    mod_name: &str,
    lines: &[&str],
    index: &mut usize,
) -> Result<EntityId, MirrError> {
    let header = lines[*index].trim();
    let is_assert = header.starts_with("assert ");
    let after_keyword = if is_assert {
        header.strip_prefix("assert ").unwrap()
    } else {
        header.strip_prefix("property ").unwrap()
    };

    let name = after_keyword
        .split_once(':')
        .map(|(n, _)| n.trim())
        .unwrap_or(after_keyword.trim_end_matches('{').trim());
    if name.is_empty() {
        return Err(MirrError::parse_error("Property name cannot be empty."));
    }

    let directive = if header.contains("cover") {
        crate::ast::property::PropertyDirective::Cover
    } else if header.contains("assume") {
        crate::ast::property::PropertyDirective::Assume
    } else {
        crate::ast::property::PropertyDirective::Assert
    };

    let true_ent = registry.next_id();
    registry.set_literal(true_ent, crate::ecs::components::LiteralComponent(LiteralValue::Bool(true)));
    let formula_exprs = vec![true_ent];
    let formula = crate::ast::property::PropertyFormula::Always(crate::ast::expr::Expr::Literal(
        LiteralValue::Bool(true),
    ));

    *index += 1;
    while *index < lines.len() {
        skip_empty_and_comments(lines, index);
        let line = lines[*index].trim();
        if line == "}" || line.ends_with(';') {
            if line == "}" {
                *index += 1;
            }
            break;
        }
        *index += 1;
    }

    let prop_id = registry.create_entity(name, KindComponent(EntityKind::PROPERTY));
    registry.register_symbol(&format!("{}::{}", mod_name, name), prop_id);
    registry.set_property(
        prop_id,
        PropertyComponent { directive, formula, formula_exprs, origin: None },
    );
    registry.set_parent(prop_id, module_entity);

    Ok(prop_id)
}

// ── Expression Parsing (ECS-Native) ──────────────────────────────────────────

const MAX_EXPR_DEPTH: usize = 128;

pub fn parse_expression_ecs(
    registry: &mut Registry,
    expr_str: &str,
) -> Result<EntityId, MirrError> {
    let tokens = crate::lexer::tokenize_expr(expr_str)?;
    let mut parser = EcsExprParser::new(registry, tokens);
    parser.parse_full()
}

struct EcsExprParser<'a> {
    registry: &'a mut Registry,
    tokens: Vec<Token>,
    pos: usize,
    depth: usize,
}

impl<'a> EcsExprParser<'a> {
    fn new(registry: &'a mut Registry, tokens: Vec<Token>) -> Self {
        Self { registry, tokens, pos: 0, depth: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }
    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn parse_full(&mut self) -> Result<EntityId, MirrError> {
        if self.tokens.is_empty() {
            return Err(MirrError::parse_error("Empty expression."));
        }
        let expr = self.parse_expr(0)?;
        if !self.at_end() {
            return Err(MirrError::parse_error(format!(
                "Unexpected token in expression: {:?}",
                self.peek()
            )));
        }
        Ok(expr)
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<EntityId, MirrError> {
        self.depth += 1;
        if self.depth > MAX_EXPR_DEPTH {
            return Err(MirrError::parse_error("Expression too deep."));
        }

        let mut lhs = self.parse_primary()?;

        while let Some(tok) = self.peek() {
            let op = match token_to_binop(tok) {
                Some(op) => op,
                None => break,
            };

            let (l_bp, r_bp) = infix_binding_power(&op);
            if l_bp < min_bp {
                break;
            }

            self.advance();
            let rhs = self.parse_expr(r_bp)?;

            let id = self.registry.next_id();
            self.registry.set_binary_op(id, BinaryComponent { op, left: lhs, right: rhs });
            lhs = id;
        }

        self.depth -= 1;
        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<EntityId, MirrError> {
        let tok = self
            .advance()
            .cloned()
            .ok_or_else(|| MirrError::parse_error("Expected primary expression, found EOF."))?;

        match tok {
            Token::Integer(n) => {
                let id = self.registry.next_id();
                self.registry.set_literal(id, LiteralComponent(LiteralValue::Integer(n)));
                Ok(id)
            }
            Token::True => {
                let id = self.registry.next_id();
                self.registry.set_literal(id, LiteralComponent(LiteralValue::Bool(true)));
                Ok(id)
            }
            Token::False => {
                let id = self.registry.next_id();
                self.registry.set_literal(id, LiteralComponent(LiteralValue::Bool(false)));
                Ok(id)
            }
            Token::Ident(name) => {
                if name == "prev" {
                    return self.parse_prev();
                }
                let id = self.registry.next_id();
                if let Some(sig_ent) = self.registry.get_entity_by_name(&name) {
                    self.registry.set_signal_ref(id, SignalRefComponent(sig_ent));
                } else {
                    self.registry.set_pending_signal_ref(id, PendingSignalRef(name));
                }
                Ok(id)
            }
            Token::LParen => {
                let expr = self.parse_expr(0)?;
                match self.advance() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(MirrError::parse_error("Expected ')' after expression.")),
                }
            }
            Token::Bang => {
                let operand = self.parse_expr(22)?;
                let id = self.registry.next_id();
                self.registry.set_unary_op(id, UnaryComponent { op: UnaryOp::Not, operand });
                Ok(id)
            }
            Token::Minus => {
                let operand = self.parse_expr(22)?;
                let id = self.registry.next_id();
                self.registry.set_unary_op(id, UnaryComponent { op: UnaryOp::Negate, operand });
                Ok(id)
            }
            _ => Err(MirrError::parse_error(format!(
                "Unexpected token in primary expression: {:?}",
                tok
            ))),
        }
    }

    fn parse_prev(&mut self) -> Result<EntityId, MirrError> {
        match self.advance() {
            Some(Token::LParen) => {}
            _ => return Err(MirrError::parse_error("Expected '(' after 'prev'.")),
        }

        let signal_name = match self.advance() {
            Some(Token::Ident(name)) => name.clone(),
            _ => {
                return Err(MirrError::parse_error(
                    "Expected signal name as first argument to 'prev'.",
                ))
            }
        };

        match self.advance() {
            Some(Token::Comma) => {}
            _ => return Err(MirrError::parse_error("Expected ',' after signal name in 'prev'.")),
        }

        let delay = match self.advance() {
            Some(Token::Integer(n)) => *n,
            _ => {
                return Err(MirrError::parse_error(
                    "Expected integer literal as second argument to 'prev'.",
                ))
            }
        };

        match self.advance() {
            Some(Token::RParen) => {}
            _ => return Err(MirrError::parse_error("Expected ')' after 'prev' arguments.")),
        }

        let sig_node = self.registry.next_id();
        if let Some(sig_ent) = self.registry.get_entity_by_name(&signal_name) {
            self.registry.set_signal_ref(sig_node, SignalRefComponent(sig_ent));
        } else {
            self.registry.set_pending_signal_ref(sig_node, PendingSignalRef(signal_name));
        }

        let prev_node = self.registry.next_id();
        self.registry.set_prev_op(prev_node, PrevComponent { signal: sig_node, delay });
        Ok(prev_node)
    }
}

fn infix_binding_power(op: &BinaryOp) -> (u8, u8) {
    match op {
        BinaryOp::Or => (2, 3),
        BinaryOp::And => (4, 5),
        BinaryOp::BitwiseOr => (6, 7),
        BinaryOp::BitwiseAnd => (8, 9),
        BinaryOp::Xor => (10, 11),
        BinaryOp::Eq | BinaryOp::Ne => (12, 13),
        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => (14, 15),
        BinaryOp::Shl | BinaryOp::Shr => (16, 17),
        BinaryOp::Add | BinaryOp::Sub => (18, 19),
        BinaryOp::Mul => (20, 21),
    }
}

fn token_to_binop(tok: &Token) -> Option<BinaryOp> {
    match tok {
        Token::AmpAmp => Some(BinaryOp::And),
        Token::Amp => Some(BinaryOp::BitwiseAnd),
        Token::PipePipe => Some(BinaryOp::Or),
        Token::Pipe => Some(BinaryOp::BitwiseOr),
        Token::Caret => Some(BinaryOp::Xor),
        Token::EqEq => Some(BinaryOp::Eq),
        Token::BangEq => Some(BinaryOp::Ne),
        Token::Lt => Some(BinaryOp::Lt),
        Token::Le => Some(BinaryOp::Le),
        Token::Gt => Some(BinaryOp::Gt),
        Token::Ge => Some(BinaryOp::Ge),
        Token::LtLt => Some(BinaryOp::Shl),
        Token::GtGt => Some(BinaryOp::Shr),
        Token::Plus => Some(BinaryOp::Add),
        Token::Minus => Some(BinaryOp::Sub),
        Token::Star => Some(BinaryOp::Mul),
        _ => None,
    }
}
