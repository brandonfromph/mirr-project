//! S-expression to AST conversion.

#![forbid(unsafe_code)]

use crate::ast::macro_nodes::{ModuleMacroStmt, ReflexMacroStmt, UnexpandedReflex};
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
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected program list", crate::error_codes::ec(805)))
    })?;
    if items.is_empty() {
        return Err(sexpr_err(format!("{} Empty program list", crate::error_codes::ec(806))));
    }
    expect_head(items, "program")?;
    if items.len() < 3 {
        return Err(sexpr_err(format!(
            "{} Program requires (patterns ...) and (module ...)",
            crate::error_codes::ec(806)
        )));
    }
    let patterns = parse_patterns_section(&items[1])?;
    let module = parse_module_section(&items[2])?;
    Ok(MirrProgram { patterns, imports: vec![], module })
}

pub(super) fn expect_head(items: &[SExpr], expected: &str) -> Result<(), MirrError> {
    match items[0].as_symbol() {
        Some(s) if s == expected => Ok(()),
        Some(s) => Err(sexpr_err(format!(
            "{} Expected '{expected}', found '{s}'",
            crate::error_codes::ec(805)
        ))),
        None => Err(sexpr_err(format!(
            "{} Expected symbol '{expected}' as list head",
            crate::error_codes::ec(805)
        ))),
    }
}

fn parse_patterns_section(sexpr: &SExpr) -> Result<Vec<PatternDef>, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected patterns list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "patterns")?;
    let mut patterns = Vec::new();
    for item in &items[1..] {
        patterns.push(parse_pattern_def(item)?);
    }
    Ok(patterns)
}

fn parse_pattern_def(sexpr: &SExpr) -> Result<PatternDef, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected pattern-def list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "pattern-def")?;
    if items.len() < 4 {
        return Err(sexpr_err(format!(
            "{} pattern-def requires name, params, reflect",
            crate::error_codes::ec(806)
        )));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!("{} pattern-def name must be a string", crate::error_codes::ec(806)))
        })?
        .to_string();
    let params = parse_params_section(&items[2])?;
    let body = parse_reflect_section(&items[3])?;
    Ok(PatternDef { name, params, body, span: None })
}

fn parse_params_section(sexpr: &SExpr) -> Result<Vec<PatternParam>, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected params list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "params")?;
    let mut params = Vec::new();
    for item in &items[1..] {
        params.push(parse_pattern_param(item)?);
    }
    Ok(params)
}

fn parse_pattern_param(sexpr: &SExpr) -> Result<PatternParam, MirrError> {
    let items = sexpr
        .as_list()
        .ok_or_else(|| sexpr_err(format!("{} Expected param list", crate::error_codes::ec(805))))?;
    expect_head(items, "param")?;
    if items.len() < 3 {
        return Err(sexpr_err(format!(
            "{} param requires name and kind",
            crate::error_codes::ec(806)
        )));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!("{} param name must be a string", crate::error_codes::ec(806)))
        })?
        .to_string();
    let kind_sym = items[2].as_symbol().ok_or_else(|| {
        sexpr_err(format!("{} param kind must be a symbol", crate::error_codes::ec(806)))
    })?;
    let kind = match kind_sym {
        "signal" => {
            if items.len() < 5 {
                return Err(sexpr_err(format!(
                    "{} signal param requires kind and type",
                    crate::error_codes::ec(806)
                )));
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
                return Err(sexpr_err(format!(
                    "{} constant param requires type",
                    crate::error_codes::ec(806)
                )));
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
        other => {
            return Err(sexpr_err(format!(
                "{} Unknown param kind: {other}",
                crate::error_codes::ec(806)
            )))
        }
    };
    Ok(PatternParam { name, kind })
}

fn parse_reflect_section(sexpr: &SExpr) -> Result<ReflectBlock, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected reflect list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "reflect")?;
    let mut statements = Vec::new();

    for item in &items[1..] {
        let inner = item.as_list().ok_or_else(|| {
            sexpr_err(format!("{} Expected reflect section list", crate::error_codes::ec(805)))
        })?;
        if inner.is_empty() {
            continue;
        }
        match inner[0].as_symbol() {
            Some("signals") => {
                for s in &inner[1..] {
                    statements.push(ModuleMacroStmt::Signal(parse_signal_decl(s)?));
                }
            }
            Some("guards") => {
                for g in &inner[1..] {
                    statements.push(ModuleMacroStmt::Guard(parse_guard(g)?));
                }
            }
            Some("reflexes") => {
                for r in &inner[1..] {
                    let flat = parse_reflex(r)?;
                    let unexp = UnexpandedReflex {
                        name: flat.name,
                        guard_names: flat.guard_names,
                        statements: flat
                            .assignments
                            .into_iter()
                            .map(ReflexMacroStmt::Assignment)
                            .collect(),
                        span: flat.span,
                    };
                    statements.push(ModuleMacroStmt::Reflex(unexp));
                }
            }
            Some("properties") => {
                for p in &inner[1..] {
                    statements.push(ModuleMacroStmt::Property(parse_property(p)?));
                }
            }
            Some("pattern-calls") => {
                for c in &inner[1..] {
                    statements.push(ModuleMacroStmt::PatternCall(parse_pattern_call(c)?));
                }
            }
            _ => {}
        }
    }
    Ok(ReflectBlock { statements })
}

fn parse_module_section(sexpr: &SExpr) -> Result<Module, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected module list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "module")?;
    if items.len() < 3 {
        return Err(sexpr_err(format!(
            "{} Module requires at least name and signals",
            crate::error_codes::ec(806)
        )));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!("{} Module name must be a string", crate::error_codes::ec(806)))
        })?
        .to_string();

    let mut signals = Vec::new();
    let mut guards = Vec::new();
    let mut reflexes = Vec::new();
    let mut properties = Vec::new();
    let mut pattern_calls = Vec::new();
    let mut pattern_origins = Vec::new();

    for item in &items[2..] {
        let inner = item.as_list().ok_or_else(|| {
            sexpr_err(format!("{} Expected module section list", crate::error_codes::ec(805)))
        })?;
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
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected signal list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "signal")?;
    if items.len() < 4 {
        return Err(sexpr_err(format!(
            "{} signal requires name, kind, type",
            crate::error_codes::ec(806)
        )));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!("{} Signal name must be a string", crate::error_codes::ec(806)))
        })?
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
        Some(other) => {
            Err(sexpr_err(format!("{} Unknown signal kind: {other}", crate::error_codes::ec(807))))
        }
        None => {
            Err(sexpr_err(format!("{} Signal kind must be a symbol", crate::error_codes::ec(807))))
        }
    }
}

fn parse_signal_type(sexpr: &SExpr) -> Result<SignalType, MirrError> {
    match sexpr {
        SExpr::Symbol(s) if s == "bool" => Ok(SignalType::Bool),
        SExpr::List(items) if !items.is_empty() => {
            let head = items[0].as_symbol().ok_or_else(|| {
                sexpr_err(format!("{} Type head must be a symbol", crate::error_codes::ec(807)))
            })?;
            match head {
                "unsigned" => {
                    if items.len() != 2 {
                        return Err(sexpr_err(format!(
                            "{} unsigned requires exactly one width",
                            crate::error_codes::ec(807)
                        )));
                    }
                    let width = items[1].as_integer().ok_or_else(|| {
                        sexpr_err(format!(
                            "{} Type width must be an integer",
                            crate::error_codes::ec(807)
                        ))
                    })?;
                    Ok(SignalType::Unsigned(width as u32))
                }
                "signed" => {
                    if items.len() != 2 {
                        return Err(sexpr_err(format!(
                            "{} signed requires exactly one width",
                            crate::error_codes::ec(807)
                        )));
                    }
                    let width = items[1].as_integer().ok_or_else(|| {
                        sexpr_err(format!(
                            "{} Type width must be an integer",
                            crate::error_codes::ec(807)
                        ))
                    })?;
                    Ok(SignalType::Signed(width as u32))
                }
                "array" => {
                    if items.len() != 3 {
                        return Err(sexpr_err(format!(
                            "{} array requires element type and length",
                            crate::error_codes::ec(807)
                        )));
                    }
                    let element = Box::new(parse_signal_type(&items[1])?);
                    let length = items[2].as_integer().ok_or_else(|| {
                        sexpr_err(format!(
                            "{} array length must be an integer",
                            crate::error_codes::ec(807)
                        ))
                    })?;
                    Ok(SignalType::Array { element, length })
                }
                "struct" => {
                    if items.len() < 2 {
                        return Err(sexpr_err(format!(
                            "{} struct requires a name",
                            crate::error_codes::ec(807)
                        )));
                    }
                    let name = items[1]
                        .as_str_val()
                        .ok_or_else(|| {
                            sexpr_err(format!(
                                "{} struct name must be a string",
                                crate::error_codes::ec(807)
                            ))
                        })?
                        .to_string();
                    let mut fields = Vec::new();
                    for item in items[2..].iter().take(32) {
                        let field_list = item.as_list().ok_or_else(|| {
                            sexpr_err(format!(
                                "{} struct field must be a list",
                                crate::error_codes::ec(807)
                            ))
                        })?;
                        if field_list.len() != 2 {
                            return Err(sexpr_err(format!(
                                "{} struct field requires name and type",
                                crate::error_codes::ec(807)
                            )));
                        }
                        let field_name = field_list[0]
                            .as_str_val()
                            .ok_or_else(|| {
                                sexpr_err(format!(
                                    "{} field name must be a string",
                                    crate::error_codes::ec(807)
                                ))
                            })?
                            .to_string();
                        let field_type = parse_signal_type(&field_list[1])?;
                        fields.push((field_name, field_type));
                    }
                    Ok(SignalType::Struct { name, fields })
                }
                "fixed" => {
                    if items.len() != 3 {
                        return Err(sexpr_err(format!(
                            "{} fixed requires total_bits and frac_bits",
                            crate::error_codes::ec(807)
                        )));
                    }
                    let total_bits = items[1].as_integer().ok_or_else(|| {
                        sexpr_err(format!(
                            "{} total_bits must be an integer",
                            crate::error_codes::ec(807)
                        ))
                    })? as u32;
                    let frac_bits = items[2].as_integer().ok_or_else(|| {
                        sexpr_err(format!(
                            "{} frac_bits must be an integer",
                            crate::error_codes::ec(807)
                        ))
                    })? as u32;
                    Ok(SignalType::FixedPoint { total_bits, frac_bits })
                }
                "interface" => {
                    if items.len() != 2 {
                        return Err(sexpr_err(format!(
                            "{} interface requires exactly one name",
                            crate::error_codes::ec(807)
                        )));
                    }
                    let name = items[1]
                        .as_str_val()
                        .ok_or_else(|| {
                            sexpr_err(format!(
                                "{} interface name must be a string",
                                crate::error_codes::ec(807)
                            ))
                        })?
                        .to_string();
                    Ok(SignalType::Bundle(name))
                }
                "fifo" => {
                    if items.len() != 3 {
                        return Err(sexpr_err(format!(
                            "{} fifo requires element type and depth",
                            crate::error_codes::ec(807)
                        )));
                    }
                    let fifo_fields = match (items[1].as_list(), items[2].as_list()) {
                        (Some(element_items), Some(depth_items))
                            if element_items.len() == 2
                                && element_items[0].as_symbol() == Some("element")
                                && depth_items.len() == 2
                                && depth_items[0].as_symbol() == Some("depth") =>
                        {
                            let element = Box::new(parse_signal_type(&element_items[1])?);
                            let depth = depth_items[1].as_integer().ok_or_else(|| {
                                sexpr_err(format!(
                                    "{} fifo depth must be an integer",
                                    crate::error_codes::ec(807)
                                ))
                            })?;
                            Some((element, depth))
                        }
                        _ => None,
                    };
                    let (element, depth) = match fifo_fields {
                        Some(fields) => fields,
                        None => {
                            let element = Box::new(parse_signal_type(&items[1])?);
                            let depth = items[2].as_integer().ok_or_else(|| {
                                sexpr_err(format!(
                                    "{} fifo depth must be an integer",
                                    crate::error_codes::ec(807)
                                ))
                            })?;
                            (element, depth)
                        }
                    };
                    Ok(SignalType::Fifo { element, depth })
                }
                other => {
                    Err(sexpr_err(format!("{} Unknown type: {other}", crate::error_codes::ec(807))))
                }
            }
        }
        _ => Err(sexpr_err(format!("{} Invalid type S-expression", crate::error_codes::ec(807)))),
    }
}

fn parse_annotations(sexpr: &SExpr) -> Result<TypeAnnotations, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected annotations list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "annotations")?;
    let mut ann = TypeAnnotations::default();
    for item in &items[1..] {
        let inner = item.as_list().ok_or_else(|| {
            sexpr_err(format!("{} Annotation must be a list", crate::error_codes::ec(806)))
        })?;
        if inner.is_empty() {
            continue;
        }
        match inner[0].as_symbol() {
            Some("linearity") if inner.len() > 1 && inner[1].as_symbol() == Some("linear") => {
                ann.linearity = Linearity::Linear;
            }
            Some("effect") if inner.len() > 1 => match inner[1].as_symbol() {
                Some("stateful") => ann.effect = EffectQualifier::Stateful,
                Some("pure") => ann.effect = EffectQualifier::Pure,
                _ => {}
            },
            Some("refinement") if inner.len() > 1 => {
                let ref_list = inner[1].as_list().ok_or_else(|| {
                    sexpr_err(format!(
                        "{} Refinement value must be a list",
                        crate::error_codes::ec(806)
                    ))
                })?;
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
            Some("clock-domain") if inner.len() > 1 => {
                ann.clock_domain = inner[1].as_str_val().map(|s| s.to_string());
            }
            Some("phantom-tag") if inner.len() > 1 => {
                ann.phantom_tag = inner[1].as_str_val().map(|s| s.to_string());
            }
            _ => {}
        }
    }
    Ok(ann)
}

fn parse_guard(sexpr: &SExpr) -> Result<Guard, MirrError> {
    let items = sexpr
        .as_list()
        .ok_or_else(|| sexpr_err(format!("{} Expected guard list", crate::error_codes::ec(805))))?;
    expect_head(items, "guard")?;
    if items.len() < 4 {
        return Err(sexpr_err(format!(
            "{} guard requires name, condition, cycles",
            crate::error_codes::ec(806)
        )));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!("{} Guard name must be a string", crate::error_codes::ec(806)))
        })?
        .to_string();
    let condition = parse_expr(&items[2])?;
    let cycles_item = &items[3];
    let (cycles, template_cycles) = if let Some(s) = cycles_item.as_str_val() {
        (0, Some(s.to_string()))
    } else if let Some(i) = cycles_item.as_integer() {
        (i, None)
    } else {
        return Err(sexpr_err(format!(
            "{} Guard cycles must be an integer or string",
            crate::error_codes::ec(806)
        )));
    };
    Ok(Guard { name, condition, cycles, template_cycles, origin: None, span: None })
}

fn parse_reflex(sexpr: &SExpr) -> Result<Reflex, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected reflex list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "reflex")?;
    if items.len() < 4 {
        return Err(sexpr_err(format!(
            "{} reflex requires name, on-clause, assignments",
            crate::error_codes::ec(806)
        )));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!("{} Reflex name must be a string", crate::error_codes::ec(806)))
        })?
        .to_string();

    // Parse (on "guard1" "guard2" ...)
    let on_list = items[2]
        .as_list()
        .ok_or_else(|| sexpr_err(format!("{} Expected on-list", crate::error_codes::ec(805))))?;
    expect_head(on_list, "on")?;
    let guard_names: Vec<String> =
        on_list[1..].iter().filter_map(|s| s.as_str_val().map(|v| v.to_string())).collect();

    // Parse assignments
    let mut assignments = Vec::new();
    for item in &items[3..] {
        let assign_list = item.as_list().ok_or_else(|| {
            sexpr_err(format!("{} Expected assign list", crate::error_codes::ec(805)))
        })?;
        expect_head(assign_list, "assign")?;
        if assign_list.len() < 3 {
            return Err(sexpr_err(format!(
                "{} assign requires target and value",
                crate::error_codes::ec(806)
            )));
        }
        let target = assign_list[1]
            .as_str_val()
            .ok_or_else(|| {
                sexpr_err(format!("{} assign target must be a string", crate::error_codes::ec(806)))
            })?
            .to_string();
        let value = parse_expr(&assign_list[2])?;
        assignments.push(Assignment { target, value, span: None });
    }

    Ok(Reflex { name, guard_names, assignments, origin: None, span: None })
}

fn parse_property(sexpr: &SExpr) -> Result<PropertyDecl, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected property list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "property")?;
    if items.len() < 4 {
        return Err(sexpr_err(format!(
            "{} property requires name, directive, formula",
            crate::error_codes::ec(806)
        )));
    }
    let name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!("{} Property name must be a string", crate::error_codes::ec(806)))
        })?
        .to_string();
    let directive = match items[2].as_symbol() {
        Some("assert") => PropertyDirective::Assert,
        Some("cover") => PropertyDirective::Cover,
        Some("assume") => PropertyDirective::Assume,
        _ => {
            return Err(sexpr_err(format!(
                "{} Unknown property directive",
                crate::error_codes::ec(806)
            )))
        }
    };
    let formula = parse_formula(&items[3])?;
    Ok(PropertyDecl { name, directive, formula, origin: None, span: None })
}

fn parse_formula(sexpr: &SExpr) -> Result<PropertyFormula, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected formula list", crate::error_codes::ec(805)))
    })?;
    if items.is_empty() {
        return Err(sexpr_err(format!("{} Empty formula list", crate::error_codes::ec(806))));
    }
    match items[0].as_symbol() {
        Some("always") => {
            if items.len() < 2 {
                return Err(sexpr_err(format!(
                    "{} always requires expression",
                    crate::error_codes::ec(806)
                )));
            }
            Ok(PropertyFormula::Always(parse_expr(&items[1])?))
        }
        Some("never") => {
            if items.len() < 2 {
                return Err(sexpr_err(format!(
                    "{} never requires expression",
                    crate::error_codes::ec(806)
                )));
            }
            Ok(PropertyFormula::Never(parse_expr(&items[1])?))
        }
        Some("always-implies") => {
            if items.len() < 3 {
                return Err(sexpr_err(format!(
                    "{} always-implies requires antecedent and consequent",
                    crate::error_codes::ec(806)
                )));
            }
            Ok(PropertyFormula::AlwaysImplies {
                antecedent: parse_expr(&items[1])?,
                consequent: parse_expr(&items[2])?,
            })
        }
        Some("never-implies") => {
            if items.len() < 3 {
                return Err(sexpr_err(format!(
                    "{} never-implies requires antecedent and consequent",
                    crate::error_codes::ec(806)
                )));
            }
            Ok(PropertyFormula::NeverImplies {
                antecedent: parse_expr(&items[1])?,
                consequent: parse_expr(&items[2])?,
            })
        }
        Some("eventually-within") => {
            if items.len() < 3 {
                return Err(sexpr_err(format!(
                    "{} eventually-within requires expr and cycles",
                    crate::error_codes::ec(806)
                )));
            }
            let cycles = items[2].as_integer().ok_or_else(|| {
                sexpr_err(format!("{} cycles must be integer", crate::error_codes::ec(806)))
            })? as u32;
            Ok(PropertyFormula::EventuallyWithin { expr: parse_expr(&items[1])?, cycles })
        }
        Some("always-followed-by") => {
            if items.len() < 4 {
                return Err(sexpr_err(format!(
                    "{} always-followed-by requires trigger, response, delay",
                    crate::error_codes::ec(806)
                )));
            }
            let delay_cycles = items[3].as_integer().ok_or_else(|| {
                sexpr_err(format!("{} delay must be integer", crate::error_codes::ec(806)))
            })? as u32;
            Ok(PropertyFormula::AlwaysFollowedBy {
                trigger: parse_expr(&items[1])?,
                response: parse_expr(&items[2])?,
                delay_cycles,
            })
        }
        Some(other) => {
            Err(sexpr_err(format!("{} Unknown formula form: {other}", crate::error_codes::ec(805))))
        }
        None => {
            Err(sexpr_err(format!("{} Formula head must be a symbol", crate::error_codes::ec(805))))
        }
    }
}
