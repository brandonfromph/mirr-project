//! Bidirectional AST ↔ S-expression conversion.
//!
//! For all parseable MIRR programs:
//! `parse_mirr(source) == sexpr_to_ast(ast_to_sexpr(parse_mirr(source)))`

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::pattern::{
    PatternArg, PatternCall, PatternDef, PatternOrigin, PatternParam, PatternParamKind,
    ReflectBlock,
};
use crate::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use crate::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use crate::ast::types::{
    BinaryOp, EffectQualifier, ExtendedType, Linearity, LiteralValue, Refinement, SignalKind,
    SignalType, TypeAnnotations, UnaryOp,
};
use crate::error::MirrError;
use crate::sexpr::parser::sexpr_err;
use crate::sexpr::types::SExpr;

/// Maximum nesting depth for expression conversion/parsing (NASA Power-of-10).
const MAX_CONVERT_DEPTH: usize = 64;

// =========================================================================
// AST -> S-Expression
// =========================================================================

/// Convert a full MIRR program to its S-expression representation.
pub fn ast_to_sexpr(program: &MirrProgram) -> SExpr {
    SExpr::list(vec![
        SExpr::sym("program"),
        convert_patterns(&program.patterns),
        convert_module(&program.module),
    ])
}

fn convert_patterns(patterns: &[PatternDef]) -> SExpr {
    let mut items = vec![SExpr::sym("patterns")];
    for p in patterns {
        items.push(convert_pattern_def(p));
    }
    SExpr::list(items)
}

fn convert_pattern_def(p: &PatternDef) -> SExpr {
    let mut items = vec![SExpr::sym("pattern-def"), SExpr::str_val(&p.name)];
    // Params
    let mut param_items = vec![SExpr::sym("params")];
    for param in &p.params {
        param_items.push(convert_pattern_param(param));
    }
    items.push(SExpr::list(param_items));
    // Reflect body
    let mut reflect_items = vec![SExpr::sym("reflect")];
    for line in &p.body.raw_lines {
        reflect_items.push(SExpr::str_val(line));
    }
    items.push(SExpr::list(reflect_items));
    SExpr::list(items)
}

fn convert_pattern_param(param: &PatternParam) -> SExpr {
    let mut items = vec![SExpr::sym("param"), SExpr::str_val(&param.name)];
    match &param.kind {
        PatternParamKind::Signal { kind, ty, annotations } => {
            items.push(SExpr::sym("signal"));
            items.push(convert_signal_kind(*kind));
            items.push(convert_signal_type(*ty));
            if !annotations.is_default() {
                items.push(convert_annotations(annotations));
            }
        }
        PatternParamKind::Constant { ty, annotations } => {
            items.push(SExpr::sym("constant"));
            items.push(convert_signal_type(*ty));
            if !annotations.is_default() {
                items.push(convert_annotations(annotations));
            }
        }
        PatternParamKind::Pattern => {
            items.push(SExpr::sym("pattern"));
        }
    }
    SExpr::list(items)
}

fn convert_module(module: &Module) -> SExpr {
    SExpr::list(vec![
        SExpr::sym("module"),
        SExpr::str_val(&module.name),
        convert_signals(&module.signals),
        convert_guards(&module.guards),
        convert_reflexes(&module.reflexes),
        convert_properties(&module.properties),
        convert_pattern_calls(&module.pattern_calls),
        convert_pattern_origins(&module.pattern_origins),
    ])
}

fn convert_signals(signals: &[SignalDecl]) -> SExpr {
    let mut items = vec![SExpr::sym("signals")];
    for s in signals {
        let mut sig = vec![
            SExpr::sym("signal"),
            SExpr::str_val(&s.name),
            convert_signal_kind(s.kind),
            convert_signal_type(s.ty.core),
        ];
        if !s.ty.annotations.is_default() {
            sig.push(convert_annotations(&s.ty.annotations));
        }
        items.push(SExpr::list(sig));
    }
    SExpr::list(items)
}

fn convert_signal_kind(kind: SignalKind) -> SExpr {
    match kind {
        SignalKind::Input => SExpr::sym("input"),
        SignalKind::Output => SExpr::sym("output"),
        SignalKind::Internal => SExpr::sym("internal"),
    }
}

fn convert_signal_type(ty: SignalType) -> SExpr {
    match ty {
        SignalType::Bool => SExpr::sym("bool"),
        SignalType::Unsigned(w) => SExpr::list(vec![SExpr::sym("unsigned"), SExpr::int(w as u64)]),
        SignalType::Signed(w) => SExpr::list(vec![SExpr::sym("signed"), SExpr::int(w as u64)]),
    }
}

fn convert_annotations(ann: &TypeAnnotations) -> SExpr {
    let mut items = vec![SExpr::sym("annotations")];
    if ann.linearity == Linearity::Linear {
        items.push(SExpr::list(vec![SExpr::sym("linearity"), SExpr::sym("linear")]));
    }
    match ann.effect {
        EffectQualifier::Stateful => {
            items.push(SExpr::list(vec![SExpr::sym("effect"), SExpr::sym("stateful")]));
        }
        EffectQualifier::Pure => {
            items.push(SExpr::list(vec![SExpr::sym("effect"), SExpr::sym("pure")]));
        }
        EffectQualifier::Unspecified => {}
    }
    if let Some(ref r) = ann.refinement {
        match r {
            Refinement::Range { lo, hi } => {
                items.push(SExpr::list(vec![
                    SExpr::sym("refinement"),
                    SExpr::list(vec![SExpr::sym("range"), SExpr::int(*lo), SExpr::int(*hi)]),
                ]));
            }
            Refinement::Predicate(expr) => {
                items.push(SExpr::list(vec![
                    SExpr::sym("refinement"),
                    SExpr::list(vec![SExpr::sym("predicate"), SExpr::str_val(expr)]),
                ]));
            }
        }
    }
    if let Some(ref cd) = ann.clock_domain {
        items.push(SExpr::list(vec![SExpr::sym("clock-domain"), SExpr::str_val(cd)]));
    }
    if let Some(ref pt) = ann.phantom_tag {
        items.push(SExpr::list(vec![SExpr::sym("phantom-tag"), SExpr::str_val(pt)]));
    }
    SExpr::list(items)
}

fn convert_guards(guards: &[Guard]) -> SExpr {
    let mut items = vec![SExpr::sym("guards")];
    for g in guards {
        items.push(SExpr::list(vec![
            SExpr::sym("guard"),
            SExpr::str_val(&g.name),
            convert_expr(&g.condition),
            SExpr::int(g.cycles),
        ]));
    }
    SExpr::list(items)
}

fn convert_reflexes(reflexes: &[Reflex]) -> SExpr {
    let mut items = vec![SExpr::sym("reflexes")];
    for r in reflexes {
        let mut reflex_items = vec![SExpr::sym("reflex"), SExpr::str_val(&r.name)];
        let mut on_items = vec![SExpr::sym("on")];
        for gn in &r.guard_names {
            on_items.push(SExpr::str_val(gn));
        }
        reflex_items.push(SExpr::list(on_items));
        for a in &r.assignments {
            reflex_items.push(SExpr::list(vec![
                SExpr::sym("assign"),
                SExpr::str_val(&a.target),
                convert_expr(&a.value),
            ]));
        }
        items.push(SExpr::list(reflex_items));
    }
    SExpr::list(items)
}

fn convert_properties(props: &[PropertyDecl]) -> SExpr {
    let mut items = vec![SExpr::sym("properties")];
    for p in props {
        items.push(SExpr::list(vec![
            SExpr::sym("property"),
            SExpr::str_val(&p.name),
            convert_directive(p.directive),
            convert_formula(&p.formula),
        ]));
    }
    SExpr::list(items)
}

fn convert_directive(d: PropertyDirective) -> SExpr {
    match d {
        PropertyDirective::Assert => SExpr::sym("assert"),
        PropertyDirective::Cover => SExpr::sym("cover"),
        PropertyDirective::Assume => SExpr::sym("assume"),
    }
}

fn convert_formula(f: &PropertyFormula) -> SExpr {
    match f {
        PropertyFormula::Always(e) => SExpr::list(vec![SExpr::sym("always"), convert_expr(e)]),
        PropertyFormula::Never(e) => SExpr::list(vec![SExpr::sym("never"), convert_expr(e)]),
        PropertyFormula::AlwaysImplies { antecedent, consequent } => SExpr::list(vec![
            SExpr::sym("always-implies"),
            convert_expr(antecedent),
            convert_expr(consequent),
        ]),
        PropertyFormula::NeverImplies { antecedent, consequent } => SExpr::list(vec![
            SExpr::sym("never-implies"),
            convert_expr(antecedent),
            convert_expr(consequent),
        ]),
        PropertyFormula::EventuallyWithin { expr, cycles } => SExpr::list(vec![
            SExpr::sym("eventually-within"),
            convert_expr(expr),
            SExpr::int(*cycles as u64),
        ]),
        PropertyFormula::AlwaysFollowedBy { trigger, response, delay_cycles } => SExpr::list(vec![
            SExpr::sym("always-followed-by"),
            convert_expr(trigger),
            convert_expr(response),
            SExpr::int(*delay_cycles as u64),
        ]),
    }
}

/// Work item for iterative `convert_expr` (replaces recursion).
enum ConvertWork<'a> {
    /// Process an expression node.
    Process(&'a Expr),
    /// Compose a unary S-expression from one result.
    BuildUnary(&'static str),
    /// Compose a binary S-expression from two results.
    BuildBinary(&'static str),
}

fn convert_expr(expr: &Expr) -> SExpr {
    const MAX_ITER: usize = MAX_CONVERT_DEPTH * 4;
    let mut work_stack: Vec<ConvertWork<'_>> = Vec::with_capacity(MAX_CONVERT_DEPTH);
    let mut result_stack: Vec<SExpr> = Vec::with_capacity(MAX_CONVERT_DEPTH);
    work_stack.push(ConvertWork::Process(expr));

    let mut iterations: usize = 0;
    while let Some(work) = work_stack.pop() {
        iterations += 1;
        if iterations > MAX_ITER {
            break;
        }
        match work {
            ConvertWork::Process(e) => match e {
                Expr::Literal(LiteralValue::Bool(b)) => result_stack.push(SExpr::Bool(*b)),
                Expr::Literal(LiteralValue::Integer(n)) => result_stack.push(SExpr::Integer(*n)),
                Expr::Signal(name) => {
                    result_stack
                        .push(SExpr::list(vec![SExpr::sym("signal"), SExpr::str_val(name)]));
                }
                Expr::Prev { signal, delay } => {
                    result_stack.push(SExpr::list(vec![
                        SExpr::sym("prev"),
                        SExpr::str_val(signal),
                        SExpr::int(*delay),
                    ]));
                }
                Expr::Unary { op, operand } => {
                    let op_sym = match op {
                        UnaryOp::Not => "not",
                        UnaryOp::Negate => "negate",
                    };
                    work_stack.push(ConvertWork::BuildUnary(op_sym));
                    work_stack.push(ConvertWork::Process(operand));
                }
                Expr::Binary { op, left, right } => {
                    let op_sym = binop_to_symbol(*op);
                    work_stack.push(ConvertWork::BuildBinary(op_sym));
                    work_stack.push(ConvertWork::Process(right));
                    work_stack.push(ConvertWork::Process(left));
                }
            },
            ConvertWork::BuildUnary(op_sym) => {
                let operand = result_stack.pop().unwrap_or(SExpr::Bool(false));
                result_stack.push(SExpr::list(vec![SExpr::sym(op_sym), operand]));
            }
            ConvertWork::BuildBinary(op_sym) => {
                let right = result_stack.pop().unwrap_or(SExpr::Bool(false));
                let left = result_stack.pop().unwrap_or(SExpr::Bool(false));
                result_stack.push(SExpr::list(vec![SExpr::sym(op_sym), left, right]));
            }
        }
    }

    result_stack.pop().unwrap_or(SExpr::Bool(false))
}

fn binop_to_symbol(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
        BinaryOp::Xor => "xor",
        BinaryOp::Lt => "<",
        BinaryOp::Le => "<=",
        BinaryOp::Gt => ">",
        BinaryOp::Ge => ">=",
        BinaryOp::Eq => "==",
        BinaryOp::Ne => "!=",
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Shl => "<<",
        BinaryOp::Shr => ">>",
    }
}

fn convert_pattern_calls(calls: &[PatternCall]) -> SExpr {
    let mut items = vec![SExpr::sym("pattern-calls")];
    for c in calls {
        let mut call_items = vec![SExpr::sym("pattern-call"), SExpr::str_val(&c.pattern_name)];
        for arg in &c.arguments {
            call_items.push(convert_pattern_arg(arg));
        }
        items.push(SExpr::list(call_items));
    }
    SExpr::list(items)
}

fn convert_pattern_arg(arg: &PatternArg) -> SExpr {
    match arg {
        PatternArg::SignalRef(name) => {
            SExpr::list(vec![SExpr::sym("signal-ref"), SExpr::str_val(name)])
        }
        PatternArg::ConstInt(n) => SExpr::list(vec![SExpr::sym("const-int"), SExpr::int(*n)]),
        PatternArg::ConstBool(b) => {
            SExpr::list(vec![SExpr::sym("const-bool"), SExpr::bool_val(*b)])
        }
        PatternArg::PatternRef(name) => {
            SExpr::list(vec![SExpr::sym("pattern-ref"), SExpr::str_val(name)])
        }
    }
}

fn convert_pattern_origins(origins: &[PatternOrigin]) -> SExpr {
    let mut items = vec![SExpr::sym("pattern-origins")];
    for o in origins {
        items.push(SExpr::list(vec![
            SExpr::sym("pattern-origin"),
            SExpr::str_val(&o.pattern_name),
            SExpr::str_val(&o.call_args_summary),
        ]));
    }
    SExpr::list(items)
}

// =========================================================================
// S-Expression -> AST
// =========================================================================

/// Convert an S-expression back to a MIRR program AST.
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
    Ok(MirrProgram { patterns, module })
}

fn expect_head(items: &[SExpr], expected: &str) -> Result<(), MirrError> {
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
        SExpr::List(items) if items.len() == 2 => {
            let head = items[0]
                .as_symbol()
                .ok_or_else(|| sexpr_err("[E807] Type head must be a symbol"))?;
            let width = items[1]
                .as_integer()
                .ok_or_else(|| sexpr_err("[E807] Type width must be an integer"))?;
            match head {
                "unsigned" => Ok(SignalType::Unsigned(width as u32)),
                "signed" => Ok(SignalType::Signed(width as u32)),
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

/// Work item for iterative `parse_expr` (replaces recursion).
enum ParseWork<'a> {
    /// Parse an S-expression node into an Expr.
    Process(&'a SExpr),
    /// Compose a unary Expr from one result.
    BuildUnary(UnaryOp),
    /// Compose a binary Expr from two results.
    BuildBinary(BinaryOp),
}

fn parse_expr(sexpr: &SExpr) -> Result<Expr, MirrError> {
    const MAX_ITER: usize = MAX_CONVERT_DEPTH * 4;
    let mut work_stack: Vec<ParseWork<'_>> = Vec::with_capacity(MAX_CONVERT_DEPTH);
    let mut result_stack: Vec<Expr> = Vec::with_capacity(MAX_CONVERT_DEPTH);
    work_stack.push(ParseWork::Process(sexpr));

    let mut iterations: usize = 0;
    while let Some(work) = work_stack.pop() {
        iterations += 1;
        if iterations > MAX_ITER {
            return Err(sexpr_err("[E808] Expression nesting exceeds maximum depth"));
        }
        match work {
            ParseWork::Process(s) => match s {
                SExpr::Bool(b) => result_stack.push(Expr::Literal(LiteralValue::Bool(*b))),
                SExpr::Integer(n) => result_stack.push(Expr::Literal(LiteralValue::Integer(*n))),
                SExpr::List(items) if !items.is_empty() => {
                    let head = items[0]
                        .as_symbol()
                        .ok_or_else(|| sexpr_err("[E805] Expression head must be a symbol"))?;
                    match head {
                        "signal" => {
                            if items.len() < 2 {
                                return Err(sexpr_err("[E806] signal-ref requires name"));
                            }
                            let name = items[1]
                                .as_str_val()
                                .ok_or_else(|| sexpr_err("[E806] signal name must be string"))?
                                .to_string();
                            result_stack.push(Expr::Signal(name));
                        }
                        "prev" => {
                            if items.len() < 3 {
                                return Err(sexpr_err("[E806] prev requires signal and delay"));
                            }
                            let signal = items[1]
                                .as_str_val()
                                .ok_or_else(|| sexpr_err("[E806] prev signal must be string"))?
                                .to_string();
                            let delay = items[2]
                                .as_integer()
                                .ok_or_else(|| sexpr_err("[E806] prev delay must be integer"))?;
                            result_stack.push(Expr::Prev { signal, delay });
                        }
                        "not" => {
                            if items.len() < 2 {
                                return Err(sexpr_err("[E806] not requires operand"));
                            }
                            work_stack.push(ParseWork::BuildUnary(UnaryOp::Not));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        "negate" => {
                            if items.len() < 2 {
                                return Err(sexpr_err("[E806] negate requires operand"));
                            }
                            work_stack.push(ParseWork::BuildUnary(UnaryOp::Negate));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        _ => {
                            // Binary operator
                            if items.len() < 3 {
                                return Err(sexpr_err(format!(
                                    "[E805] Unknown or incomplete expression form: {head}"
                                )));
                            }
                            let op = symbol_to_binop(head)?;
                            work_stack.push(ParseWork::BuildBinary(op));
                            work_stack.push(ParseWork::Process(&items[2]));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                    }
                }
                _ => return Err(sexpr_err("[E805] Invalid expression S-expression")),
            },
            ParseWork::BuildUnary(op) => {
                let operand = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing operand in expression stack"))?;
                result_stack.push(Expr::Unary { op, operand: Box::new(operand) });
            }
            ParseWork::BuildBinary(op) => {
                let right = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing right operand in expression stack"))?;
                let left = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing left operand in expression stack"))?;
                result_stack.push(Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
        }
    }

    result_stack.pop().ok_or_else(|| sexpr_err("[E808] Empty expression result"))
}

fn symbol_to_binop(sym: &str) -> Result<BinaryOp, MirrError> {
    match sym {
        "and" => Ok(BinaryOp::And),
        "or" => Ok(BinaryOp::Or),
        "xor" => Ok(BinaryOp::Xor),
        "<" => Ok(BinaryOp::Lt),
        "<=" => Ok(BinaryOp::Le),
        ">" => Ok(BinaryOp::Gt),
        ">=" => Ok(BinaryOp::Ge),
        "==" => Ok(BinaryOp::Eq),
        "!=" => Ok(BinaryOp::Ne),
        "+" => Ok(BinaryOp::Add),
        "-" => Ok(BinaryOp::Sub),
        "*" => Ok(BinaryOp::Mul),
        "<<" => Ok(BinaryOp::Shl),
        ">>" => Ok(BinaryOp::Shr),
        other => Err(sexpr_err(format!("[E805] Unknown binary operator: {other}"))),
    }
}

fn parse_pattern_call(sexpr: &SExpr) -> Result<PatternCall, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected pattern-call list"))?;
    expect_head(items, "pattern-call")?;
    if items.len() < 2 {
        return Err(sexpr_err("[E806] pattern-call requires name"));
    }
    let pattern_name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] pattern-call name must be a string"))?
        .to_string();
    let mut args = Vec::new();
    for item in &items[2..] {
        args.push(parse_pattern_arg(item)?);
    }
    Ok(PatternCall { pattern_name, arguments: args, span: None })
}

fn parse_pattern_arg(sexpr: &SExpr) -> Result<PatternArg, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected pattern-arg list"))?;
    if items.is_empty() {
        return Err(sexpr_err("[E806] Empty pattern-arg list"));
    }
    match items[0].as_symbol() {
        Some("signal-ref") => {
            let name = items
                .get(1)
                .and_then(|s| s.as_str_val())
                .ok_or_else(|| sexpr_err("[E806] signal-ref requires name"))?
                .to_string();
            Ok(PatternArg::SignalRef(name))
        }
        Some("const-int") => {
            let n = items
                .get(1)
                .and_then(|s| s.as_integer())
                .ok_or_else(|| sexpr_err("[E806] const-int requires value"))?;
            Ok(PatternArg::ConstInt(n))
        }
        Some("const-bool") => {
            let b = items
                .get(1)
                .and_then(|s| s.as_bool())
                .ok_or_else(|| sexpr_err("[E806] const-bool requires value"))?;
            Ok(PatternArg::ConstBool(b))
        }
        Some("pattern-ref") => {
            let name = items
                .get(1)
                .and_then(|s| s.as_str_val())
                .ok_or_else(|| sexpr_err("[E806] pattern-ref requires name"))?
                .to_string();
            Ok(PatternArg::PatternRef(name))
        }
        Some(other) => Err(sexpr_err(format!("[E806] Unknown pattern arg kind: {other}"))),
        None => Err(sexpr_err("[E806] Pattern arg head must be a symbol")),
    }
}

fn parse_pattern_origin(sexpr: &SExpr) -> Result<PatternOrigin, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected pattern-origin list"))?;
    expect_head(items, "pattern-origin")?;
    if items.len() < 3 {
        return Err(sexpr_err("[E806] pattern-origin requires name and summary"));
    }
    let pattern_name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] pattern-origin name must be a string"))?
        .to_string();
    let summary = items[2]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] pattern-origin summary must be a string"))?
        .to_string();
    Ok(PatternOrigin { pattern_name, call_args_summary: summary })
}
