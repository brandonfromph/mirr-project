//! AST to S-expression conversion.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::pattern::{
    PatternArg, PatternCall, PatternDef, PatternOrigin, PatternParam, PatternParamKind,
};
use crate::ast::program::{Guard, MirrProgram, Module, Reflex, SignalDecl};
use crate::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use crate::ast::types::{
    BinaryOp, EffectQualifier, Linearity, LiteralValue, Refinement, SignalKind, SignalType,
    TypeAnnotations, UnaryOp,
};
use crate::sexpr::types::SExpr;

use super::MAX_CONVERT_DEPTH;

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
            items.push(convert_signal_type(ty.clone()));
            if !annotations.is_default() {
                items.push(convert_annotations(annotations));
            }
        }
        PatternParamKind::Constant { ty, annotations } => {
            items.push(SExpr::sym("constant"));
            items.push(convert_signal_type(ty.clone()));
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
            convert_signal_type(s.ty.core.clone()),
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
        SignalType::Array { element, length } => SExpr::list(vec![
            SExpr::sym("array"),
            convert_signal_type(*element),
            SExpr::int(length),
        ]),
        SignalType::Struct { name, fields } => {
            let mut items = vec![SExpr::sym("struct"), SExpr::str_val(&name)];
            let mut i = 0;
            while i < fields.len() && i < crate::ast::types::MAX_STRUCT_FIELDS {
                let (ref fname, ref ftype) = fields[i];
                items.push(SExpr::list(vec![
                    SExpr::str_val(fname),
                    convert_signal_type(ftype.clone()),
                ]));
                i += 1;
            }
            SExpr::list(items)
        }
        SignalType::FixedPoint { total_bits, frac_bits } => SExpr::list(vec![
            SExpr::sym("fixed"),
            SExpr::int(total_bits as u64),
            SExpr::int(frac_bits as u64),
        ]),
        SignalType::Bundle(name) => {
            SExpr::list(vec![SExpr::sym("interface"), SExpr::str_val(&name)])
        }
        SignalType::Fifo { element, depth } => {
            SExpr::list(vec![SExpr::sym("fifo"), convert_signal_type(*element), SExpr::int(depth)])
        }
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
    /// Compose `(field-access <obj> <field>)` — one result + stored field name.
    BuildFieldAccess(&'a str),
    /// Compose `(array-literal <e0> <e1> ...)` from `count` results.
    BuildArrayLiteral(usize),
    /// Compose `(struct-literal <name> (<f0> <v0>) ...)` from `count` results + stored metadata.
    BuildStructLiteral { name: &'a str, field_names: &'a [(String, Expr)], count: usize },
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
                Expr::ArrayIndex { array, index } => {
                    work_stack.push(ConvertWork::BuildBinary("aref"));
                    work_stack.push(ConvertWork::Process(index));
                    work_stack.push(ConvertWork::Process(array));
                }
                Expr::FieldAccess { object, field } => {
                    work_stack.push(ConvertWork::BuildFieldAccess(field));
                    work_stack.push(ConvertWork::Process(object));
                }
                Expr::ArrayLiteral(elems) => {
                    let count = elems.len().min(MAX_CONVERT_DEPTH);
                    work_stack.push(ConvertWork::BuildArrayLiteral(count));
                    // Push in reverse so they are processed left-to-right.
                    let mut i = count;
                    while i > 0 {
                        i -= 1;
                        work_stack.push(ConvertWork::Process(&elems[i]));
                    }
                }
                Expr::StructLiteral { name, fields } => {
                    let count = fields.len().min(MAX_CONVERT_DEPTH);
                    work_stack.push(ConvertWork::BuildStructLiteral {
                        name,
                        field_names: fields,
                        count,
                    });
                    let mut i = count;
                    while i > 0 {
                        i -= 1;
                        work_stack.push(ConvertWork::Process(&fields[i].1));
                    }
                }
                Expr::UnfoldIndex(name) => {
                    result_stack
                        .push(SExpr::list(vec![SExpr::sym("unfold-index"), SExpr::str_val(name)]));
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
            ConvertWork::BuildFieldAccess(field) => {
                let obj = result_stack.pop().unwrap_or(SExpr::Bool(false));
                result_stack.push(SExpr::list(vec![
                    SExpr::sym("field-access"),
                    obj,
                    SExpr::str_val(field),
                ]));
            }
            ConvertWork::BuildArrayLiteral(count) => {
                let mut items = vec![SExpr::sym("array-literal")];
                let mut i = 0;
                while i < count {
                    items.push(result_stack.pop().unwrap_or(SExpr::Bool(false)));
                    i += 1;
                }
                // Results were popped in reverse order; reverse back.
                items[1..].reverse();
                result_stack.push(SExpr::list(items));
            }
            ConvertWork::BuildStructLiteral { name, field_names, count } => {
                let mut items = vec![SExpr::sym("struct-literal"), SExpr::str_val(name)];
                // Collect results (popped in reverse).
                let mut vals = Vec::with_capacity(count);
                let mut i = 0;
                while i < count {
                    vals.push(result_stack.pop().unwrap_or(SExpr::Bool(false)));
                    i += 1;
                }
                vals.reverse();
                let mut j = 0;
                while j < count && j < field_names.len() {
                    items.push(SExpr::list(vec![
                        SExpr::str_val(&field_names[j].0),
                        vals[j].clone(),
                    ]));
                    j += 1;
                }
                result_stack.push(SExpr::list(items));
            }
        }
    }

    result_stack.pop().unwrap_or(SExpr::Bool(false))
}

fn binop_to_symbol(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
        BinaryOp::BitwiseOr => "bitor",
        BinaryOp::BitwiseAnd => "bitand",
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
