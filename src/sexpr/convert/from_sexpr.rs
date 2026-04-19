//! S-expression to AST conversion.

#![forbid(unsafe_code)]

use crate::ast::pattern::{PatternDef, PatternParam, PatternParamKind, ReflectBlock};
use crate::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use crate::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use crate::ast::types::{
    EffectQualifier, ExtendedType, Linearity, Refinement, SignalKind, SignalType, TypeAnnotations,
};
use crate::error::MirrError;
use crate::sexpr::parser::sexpr_err;
use crate::sexpr::types::SExpr;

use super::parse_expr::{parse_expr, parse_pattern_call, parse_pattern_origin};
pub fn sexpr_to_ast(sexpr: &SExpr) -> Result<MirrProgram, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected program list"))?;
    if items.is_empty() {
        return Err(sexpr_err("[E806] Empty program list"));
    }
    expect_head(items, "program")?;
    if items.len() < 3 {
        return Err(sexpr_err("[E806] Program requires (patterns ...) and (module ...)"));
    }
    let patterns = parse_patterns_section(&items[1])?;
    let module = parse_module_section(&items[2])?;
    Ok(MirrProgram { patterns, imports: vec![], module })
}

pub(super) fn expect_head(items: &[SExpr], expected: &str) -> Result<(), MirrError> {
    match items[0].as_symbol() {
        Some(s) if s == expected => Ok(()),
        Some(s) => Err(sexpr_err(format!("[E805] Expected '{expected}', found '{s}'"))),
        None => Err(sexpr_err(format!("[E805] Expected symbol '{expected}' as list head"))),
    }
}

fn parse_patterns_section(sexpr: &SExpr) -> Result<Vec<PatternDef>, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected patterns list"))?;
    expect_head(items, "patterns")?;
    let mut patterns = Vec::new();
    for item in &items[1..] {
        patterns.push(parse_pattern_def(item)?);
    }
    Ok(patterns)
}

fn parse_pattern_def(sexpr: &SExpr) -> Result<PatternDef, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected pattern-def list"))?;
    expect_head(items, "pattern-def")?;
    if items.len() < 4 {
        return Err(sexpr_err("[E806] pattern-def requires name, params, reflect"));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] pattern-def name must be a string"))?
        .to_string();
    let params = parse_params_section(&items[2])?;
    let body = parse_reflect_section(&items[3])?;
    Ok(PatternDef { name, params, body, span: None })
}

fn parse_params_section(sexpr: &SExpr) -> Result<Vec<PatternParam>, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected params list"))?;
    expect_head(items, "params")?;
    let mut params = Vec::new();
    for item in &items[1..] {
        params.push(parse_pattern_param(item)?);
    }
    Ok(params)
}

fn parse_pattern_param(sexpr: &SExpr) -> Result<PatternParam, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected param list"))?;
    expect_head(items, "param")?;
    if items.len() < 3 {
        return Err(sexpr_err("[E806] param requires name and kind"));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] param name must be a string"))?
        .to_string();
    let kind_sym =
        items[2].as_symbol().ok_or_else(|| sexpr_err("[E806] param kind must be a symbol"))?;
    let kind = match kind_sym {
        "signal" => {
            if items.len() < 5 {
                return Err(sexpr_err("[E806] signal param requires kind and type"));
            }
            let sk = parse_signal_kind(&items[3])?;
            let st = parse_signal_type(&items[4])?;
            let annotations = if items.len() > 5 {
                parse_annotations(&items[5])?
            } else {
                TypeAnnotations::default()
            };
            PatternParamKind::Signal { kind: sk, ty: st, annotations }
        }
        "constant" => {
            if items.len() < 4 {
                return Err(sexpr_err("[E806] constant param requires type"));
            }
            let st = parse_signal_type(&items[3])?;
            let annotations = if items.len() > 4 {
                parse_annotations(&items[4])?
            } else {
                TypeAnnotations::default()
            };
            PatternParamKind::Constant { ty: st, annotations }
        }
        "pattern" => PatternParamKind::Pattern,
        other => return Err(sexpr_err(format!("[E806] Unknown param kind: {other}"))),
    };
    Ok(PatternParam { name, kind })
}

fn parse_reflect_section(sexpr: &SExpr) -> Result<ReflectBlock, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected reflect list"))?;
    expect_head(items, "reflect")?;
    let mut raw_lines = Vec::new();
    for item in &items[1..] {
        let line = item
            .as_str_val()
            .ok_or_else(|| sexpr_err("[E806] reflect line must be a string"))?
            .to_string();
        raw_lines.push(line);
    }
    Ok(ReflectBlock { raw_lines })
}

fn parse_module_section(sexpr: &SExpr) -> Result<Module, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected module list"))?;
    expect_head(items, "module")?;
    if items.len() < 3 {
        return Err(sexpr_err("[E806] Module requires at least name and signals"));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] Module name must be a string"))?
        .to_string();

    let mut signals = Vec::new();
    let mut guards = Vec::new();
    let mut reflexes = Vec::new();
    let mut properties = Vec::new();
    let mut pattern_calls = Vec::new();
    let mut pattern_origins = Vec::new();

    for item in &items[2..] {
        let inner =
            item.as_list().ok_or_else(|| sexpr_err("[E805] Expected module section list"))?;
        if inner.is_empty() {
            continue;
        }
        match inner[0].as_symbol() {
            Some("signals") => {
                for s in &inner[1..] {
                    signals.push(parse_signal_decl(s)?);
                }
            }
            Some("guards") => {
                for g in &inner[1..] {
                    guards.push(parse_guard(g)?);
                }
            }
            Some("reflexes") => {
                for r in &inner[1..] {
                    reflexes.push(parse_reflex(r)?);
                }
            }
            Some("properties") => {
                for p in &inner[1..] {
                    properties.push(parse_property(p)?);
                }
            }
            Some("pattern-calls") => {
                for c in &inner[1..] {
                    pattern_calls.push(parse_pattern_call(c)?);
                }
            }
            Some("pattern-origins") => {
                for o in &inner[1..] {
                    pattern_origins.push(parse_pattern_origin(o)?);
                }
            }
            _ => {}
        }
    }

    Ok(Module {
        name,
        signals,
        guards,
        reflexes,
        properties,
        pattern_calls,
        pattern_origins,
        span: None,
    })
}

fn parse_signal_decl(sexpr: &SExpr) -> Result<SignalDecl, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected signal list"))?;
    expect_head(items, "signal")?;
    if items.len() < 4 {
        return Err(sexpr_err("[E806] signal requires name, kind, type"));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] Signal name must be a string"))?
        .to_string();
    let kind = parse_signal_kind(&items[2])?;
    let core = parse_signal_type(&items[3])?;
    let annotations =
        if items.len() > 4 { parse_annotations(&items[4])? } else { TypeAnnotations::default() };
    Ok(SignalDecl {
        name,
        kind,
        ty: ExtendedType::new(core, annotations),
        origin: None,
        span: None,
    })
}

fn parse_signal_kind(sexpr: &SExpr) -> Result<SignalKind, MirrError> {
    match sexpr.as_symbol() {
        Some("input") => Ok(SignalKind::Input),
        Some("output") => Ok(SignalKind::Output),
        Some("internal") => Ok(SignalKind::Internal),
        Some(other) => Err(sexpr_err(format!("[E807] Unknown signal kind: {other}"))),
        None => Err(sexpr_err("[E807] Signal kind must be a symbol")),
    }
}

fn parse_signal_type(sexpr: &SExpr) -> Result<SignalType, MirrError> {
    match sexpr {
        SExpr::Symbol(s) if s == "bool" => Ok(SignalType::Bool),
        SExpr::List(items) if !items.is_empty() => {
            let head = items[0]
                .as_symbol()
                .ok_or_else(|| sexpr_err("[E807] Type head must be a symbol"))?;
            match head {
                "unsigned" => {
                    if items.len() != 2 {
                        return Err(sexpr_err("[E807] unsigned requires exactly one width"));
                    }
                    let width = items[1]
                        .as_integer()
                        .ok_or_else(|| sexpr_err("[E807] Type width must be an integer"))?;
                    Ok(SignalType::Unsigned(width as u32))
                }
                "signed" => {
                    if items.len() != 2 {
                        return Err(sexpr_err("[E807] signed requires exactly one width"));
                    }
                    let width = items[1]
                        .as_integer()
                        .ok_or_else(|| sexpr_err("[E807] Type width must be an integer"))?;
                    Ok(SignalType::Signed(width as u32))
                }
                "array" => {
                    if items.len() != 3 {
                        return Err(sexpr_err("[E807] array requires element type and length"));
                    }
                    let element = Box::new(parse_signal_type(&items[1])?);
                    let length = items[2]
                        .as_integer()
                        .ok_or_else(|| sexpr_err("[E807] array length must be an integer"))?;
                    Ok(SignalType::Array { element, length })
                }
                "struct" => {
                    if items.len() < 2 {
                        return Err(sexpr_err("[E807] struct requires a name"));
                    }
                    let name = items[1]
                        .as_str_val()
                        .ok_or_else(|| sexpr_err("[E807] struct name must be a string"))?
                        .to_string();
                    let mut fields = Vec::new();
                    for item in items[2..].iter().take(32) {
                        let field_list = item
                            .as_list()
                            .ok_or_else(|| sexpr_err("[E807] struct field must be a list"))?;
                        if field_list.len() != 2 {
                            return Err(sexpr_err("[E807] struct field requires name and type"));
                        }
                        let field_name = field_list[0]
                            .as_str_val()
                            .ok_or_else(|| sexpr_err("[E807] field name must be a string"))?
                            .to_string();
                        let field_type = parse_signal_type(&field_list[1])?;
                        fields.push((field_name, field_type));
                    }
                    Ok(SignalType::Struct { name, fields })
                }
                "fixed" => {
                    if items.len() != 3 {
                        return Err(sexpr_err("[E807] fixed requires total_bits and frac_bits"));
                    }
                    let total_bits = items[1]
                        .as_integer()
                        .ok_or_else(|| sexpr_err("[E807] total_bits must be an integer"))?
                        as u32;
                    let frac_bits = items[2]
                        .as_integer()
                        .ok_or_else(|| sexpr_err("[E807] frac_bits must be an integer"))?
                        as u32;
                    Ok(SignalType::FixedPoint { total_bits, frac_bits })
                }
                "interface" => {
                    if items.len() != 2 {
                        return Err(sexpr_err("[E807] interface requires exactly one name"));
                    }
                    let name = items[1]
                        .as_str_val()
                        .ok_or_else(|| sexpr_err("[E807] interface name must be a string"))?
                        .to_string();
                    Ok(SignalType::Bundle(name))
                }
                "fifo" => {
                    if items.len() != 3 {
                        return Err(sexpr_err("[E807] fifo requires element type and depth"));
                    }
                    let fifo_fields = match (items[1].as_list(), items[2].as_list()) {
                        (Some(element_items), Some(depth_items))
                            if element_items.len() == 2
                                && element_items[0].as_symbol() == Some("element")
                                && depth_items.len() == 2
                                && depth_items[0].as_symbol() == Some("depth") =>
                        {
                            let element = Box::new(parse_signal_type(&element_items[1])?);
                            let depth = depth_items[1]
                                .as_integer()
                                .ok_or_else(|| sexpr_err("[E807] fifo depth must be an integer"))?;
                            Some((element, depth))
                        }
                        _ => None,
                    };
                    let (element, depth) = match fifo_fields {
                        Some(fields) => fields,
                        None => {
                            let element = Box::new(parse_signal_type(&items[1])?);
                            let depth = items[2]
                                .as_integer()
                                .ok_or_else(|| sexpr_err("[E807] fifo depth must be an integer"))?;
                            (element, depth)
                        }
                    };
                    Ok(SignalType::Fifo { element, depth })
                }
                other => Err(sexpr_err(format!("[E807] Unknown type: {other}"))),
            }
        }
        _ => Err(sexpr_err("[E807] Invalid type S-expression")),
    }
}

fn parse_annotations(sexpr: &SExpr) -> Result<TypeAnnotations, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected annotations list"))?;
    expect_head(items, "annotations")?;
    let mut ann = TypeAnnotations::default();
    for item in &items[1..] {
        let inner = item.as_list().ok_or_else(|| sexpr_err("[E806] Annotation must be a list"))?;
        if inner.is_empty() {
            continue;
        }
        match inner[0].as_symbol() {
            Some("linearity") => {
                if inner.len() > 1 && inner[1].as_symbol() == Some("linear") {
                    ann.linearity = Linearity::Linear;
                }
            }
            Some("effect") => {
                if inner.len() > 1 {
                    match inner[1].as_symbol() {
                        Some("stateful") => ann.effect = EffectQualifier::Stateful,
                        Some("pure") => ann.effect = EffectQualifier::Pure,
                        _ => {}
                    }
                }
            }
            Some("refinement") => {
                if inner.len() > 1 {
                    let ref_list = inner[1]
                        .as_list()
                        .ok_or_else(|| sexpr_err("[E806] Refinement value must be a list"))?;
                    if !ref_list.is_empty() {
                        match ref_list[0].as_symbol() {
                            Some("range") if ref_list.len() >= 3 => {
                                let lo = ref_list[1].as_integer().unwrap_or(0);
                                let hi = ref_list[2].as_integer().unwrap_or(0);
                                ann.refinement = Some(Refinement::Range { lo, hi });
                            }
                            Some("predicate") if ref_list.len() >= 2 => {
                                let expr = ref_list[1].as_str_val().unwrap_or("").to_string();
                                ann.refinement = Some(Refinement::Predicate(expr));
                            }
                            _ => {}
                        }
                    }
                }
            }
            Some("clock-domain") => {
                if inner.len() > 1 {
                    ann.clock_domain = inner[1].as_str_val().map(|s| s.to_string());
                }
            }
            Some("phantom-tag") => {
                if inner.len() > 1 {
                    ann.phantom_tag = inner[1].as_str_val().map(|s| s.to_string());
                }
            }
            _ => {}
        }
    }
    Ok(ann)
}

fn parse_guard(sexpr: &SExpr) -> Result<Guard, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected guard list"))?;
    expect_head(items, "guard")?;
    if items.len() < 4 {
        return Err(sexpr_err("[E806] guard requires name, condition, cycles"));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] Guard name must be a string"))?
        .to_string();
    let condition = parse_expr(&items[2])?;
    let cycles =
        items[3].as_integer().ok_or_else(|| sexpr_err("[E806] Guard cycles must be an integer"))?;
    Ok(Guard { name, condition, cycles, origin: None, span: None })
}

fn parse_reflex(sexpr: &SExpr) -> Result<Reflex, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected reflex list"))?;
    expect_head(items, "reflex")?;
    if items.len() < 4 {
        return Err(sexpr_err("[E806] reflex requires name, on-clause, assignments"));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] Reflex name must be a string"))?
        .to_string();

    // Parse (on "guard1" "guard2" ...)
    let on_list = items[2].as_list().ok_or_else(|| sexpr_err("[E805] Expected on-list"))?;
    expect_head(on_list, "on")?;
    let guard_names: Vec<String> =
        on_list[1..].iter().filter_map(|s| s.as_str_val().map(|v| v.to_string())).collect();

    // Parse assignments
    let mut assignments = Vec::new();
    for item in &items[3..] {
        let assign_list = item.as_list().ok_or_else(|| sexpr_err("[E805] Expected assign list"))?;
        expect_head(assign_list, "assign")?;
        if assign_list.len() < 3 {
            return Err(sexpr_err("[E806] assign requires target and value"));
        }
        let target = assign_list[1]
            .as_str_val()
            .ok_or_else(|| sexpr_err("[E806] assign target must be a string"))?
            .to_string();
        let value = parse_expr(&assign_list[2])?;
        assignments.push(Assignment { target, value, span: None });
    }

    Ok(Reflex { name, guard_names, assignments, origin: None, span: None })
}

fn parse_property(sexpr: &SExpr) -> Result<PropertyDecl, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected property list"))?;
    expect_head(items, "property")?;
    if items.len() < 4 {
        return Err(sexpr_err("[E806] property requires name, directive, formula"));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] Property name must be a string"))?
        .to_string();
    let directive = match items[2].as_symbol() {
        Some("assert") => PropertyDirective::Assert,
        Some("cover") => PropertyDirective::Cover,
        Some("assume") => PropertyDirective::Assume,
        _ => return Err(sexpr_err("[E806] Unknown property directive")),
    };
    let formula = parse_formula(&items[3])?;
    Ok(PropertyDecl { name, directive, formula, origin: None, span: None })
}

fn parse_formula(sexpr: &SExpr) -> Result<PropertyFormula, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected formula list"))?;
    if items.is_empty() {
        return Err(sexpr_err("[E806] Empty formula list"));
    }
    match items[0].as_symbol() {
        Some("always") => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] always requires expression"));
            }
            Ok(PropertyFormula::Always(parse_expr(&items[1])?))
        }
        Some("never") => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] never requires expression"));
            }
            Ok(PropertyFormula::Never(parse_expr(&items[1])?))
        }
        Some("always-implies") => {
            if items.len() < 3 {
                return Err(sexpr_err("[E806] always-implies requires antecedent and consequent"));
            }
            Ok(PropertyFormula::AlwaysImplies {
                antecedent: parse_expr(&items[1])?,
                consequent: parse_expr(&items[2])?,
            })
        }
        Some("never-implies") => {
            if items.len() < 3 {
                return Err(sexpr_err("[E806] never-implies requires antecedent and consequent"));
            }
            Ok(PropertyFormula::NeverImplies {
                antecedent: parse_expr(&items[1])?,
                consequent: parse_expr(&items[2])?,
            })
        }
        Some("eventually-within") => {
            if items.len() < 3 {
                return Err(sexpr_err("[E806] eventually-within requires expr and cycles"));
            }
            let cycles =
                items[2].as_integer().ok_or_else(|| sexpr_err("[E806] cycles must be integer"))?
                    as u32;
            Ok(PropertyFormula::EventuallyWithin { expr: parse_expr(&items[1])?, cycles })
        }
        Some("always-followed-by") => {
            if items.len() < 4 {
                return Err(sexpr_err(
                    "[E806] always-followed-by requires trigger, response, delay",
                ));
            }
            let delay_cycles =
                items[3].as_integer().ok_or_else(|| sexpr_err("[E806] delay must be integer"))?
                    as u32;
            Ok(PropertyFormula::AlwaysFollowedBy {
                trigger: parse_expr(&items[1])?,
                response: parse_expr(&items[2])?,
                delay_cycles,
            })
        }
        Some(other) => Err(sexpr_err(format!("[E805] Unknown formula form: {other}"))),
        None => Err(sexpr_err("[E805] Formula head must be a symbol")),
    }
}
