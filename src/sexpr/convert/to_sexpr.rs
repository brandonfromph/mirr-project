//! ARCHITECTURAL SUB-ENGINE: S-EXPRESSION TRANSPILER
//!
//! Converts the MIRR ECS Registry into a homoiconic S-expression representation.
//! This engine serves as the primary bridge for formal verification.

#![forbid(unsafe_code)]

use crate::ast::pattern::{PatternArg, PatternDef, PatternParam, PatternParamKind};
use crate::ast::property::{PropertyDirective, PropertyFormula};
use crate::ast::types::{
    BinaryOp, EffectQualifier, Linearity, LiteralValue, Refinement, SignalKind, SignalType,
    TypeAnnotations, UnaryOp,
};
use crate::ecs::components::EntityKind;
use crate::ecs::{EntityId, Registry};
use crate::sexpr::types::SExpr;

pub fn registry_to_sexpr(registry: &Registry, module_entity: EntityId) -> SExpr {
    SExpr::list(vec![
        SExpr::sym("program"),
        convert_patterns(registry, module_entity),
        convert_module(registry, module_entity),
    ])
}

fn convert_patterns(registry: &Registry, _module_entity: EntityId) -> SExpr {
    let mut items = vec![SExpr::sym("patterns")];
    for (idx, kind_opt) in registry.kinds.iter().enumerate() {
        if let Some(kind) = kind_opt {
            if std::mem::discriminant(&kind.0) == std::mem::discriminant(&EntityKind::PATTERN) {
                if let Some(def_comp) = registry.pattern_defs[idx].as_ref() {
                    items.push(convert_pattern_def(&def_comp.0));
                }
            }
        }
    }
    SExpr::list(items)
}

fn convert_pattern_def(p: &PatternDef) -> SExpr {
    let mut items = vec![SExpr::sym("pattern-def"), SExpr::str_val(&p.name)];
    let mut param_items = vec![SExpr::sym("params")];
    for param in &p.params {
        param_items.push(convert_pattern_param(param));
    }
    items.push(SExpr::list(param_items));

    items.push(SExpr::list(vec![
        SExpr::sym("reflect"),
        SExpr::list(vec![SExpr::sym("signals")]),
        SExpr::list(vec![SExpr::sym("guards")]),
        SExpr::list(vec![SExpr::sym("reflexes")]),
        SExpr::list(vec![SExpr::sym("properties")]),
        SExpr::list(vec![SExpr::sym("pattern-calls")]),
    ]));
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

fn convert_module(registry: &Registry, module_entity: EntityId) -> SExpr {
    let name = registry.get_entity_name(module_entity).to_string();

    SExpr::list(vec![
        SExpr::sym("module"),
        SExpr::str_val(&name),
        convert_signals(registry, module_entity),
        convert_guards(registry, module_entity),
        convert_reflexes(registry, module_entity),
        convert_properties(registry, module_entity),
        convert_pattern_calls(registry, module_entity),
        convert_pattern_origins(registry, module_entity),
    ])
}

fn get_children_discriminant(
    registry: &Registry,
    parent: EntityId,
    kind_disc: std::mem::Discriminant<EntityKind>,
) -> Vec<EntityId> {
    let mut children = Vec::new();
    for i in 0..registry.active_entities() {
        if let (Some(m), Some(k)) = (registry.modules[i].as_ref(), registry.kinds[i].as_ref()) {
            if m.0 == parent && std::mem::discriminant(&k.0) == kind_disc {
                children.push(EntityId(i as u32));
            }
        }
    }
    children
}

fn convert_signals(registry: &Registry, module_entity: EntityId) -> SExpr {
    let mut items = vec![SExpr::sym("signals")];
    for s in get_children_discriminant(
        registry,
        module_entity,
        std::mem::discriminant(&EntityKind::SIGNAL(SignalKind::Input)),
    ) {
        let name = registry.get_entity_name(s).to_string();
        if let (Some(ty_comp), Some(kind_comp)) =
            (registry.types[s.0 as usize].as_ref(), registry.kinds[s.0 as usize].as_ref())
        {
            if let EntityKind::SIGNAL(sig_kind) = &kind_comp.0 {
                let kind_sexpr = match sig_kind {
                    SignalKind::Input => SExpr::sym("input"),
                    SignalKind::Output => SExpr::sym("output"),
                    SignalKind::Internal => SExpr::sym("internal"),
                };
                let mut sig = vec![
                    SExpr::sym("signal"),
                    SExpr::str_val(&name),
                    kind_sexpr,
                    convert_signal_type(ty_comp.0.core.clone()),
                ];
                if !ty_comp.0.annotations.is_default() {
                    sig.push(convert_annotations(&ty_comp.0.annotations));
                }
                items.push(SExpr::list(sig));
            }
        }
    }
    SExpr::list(items)
}

fn convert_guards(registry: &Registry, module_entity: EntityId) -> SExpr {
    let mut items = vec![SExpr::sym("guards")];
    for g in get_children_discriminant(
        registry,
        module_entity,
        std::mem::discriminant(&EntityKind::GUARD),
    ) {
        let name = registry.get_entity_name(g).to_string();
        if let Some(cond) = registry.conditions[g.0 as usize].as_ref() {
            let mut g_list = vec![
                SExpr::sym("guard"),
                SExpr::str_val(&name),
                convert_expr_ecs(registry, cond.0),
            ];
            let cycles = registry.cycles[g.0 as usize].as_ref().map(|c| c.0).unwrap_or(1);
            g_list.push(SExpr::int(cycles));
            items.push(SExpr::list(g_list));
        }
    }
    SExpr::list(items)
}

fn convert_reflexes(registry: &Registry, module_entity: EntityId) -> SExpr {
    let mut items = vec![SExpr::sym("reflexes")];
    for r in get_children_discriminant(
        registry,
        module_entity,
        std::mem::discriminant(&EntityKind::REFLEX),
    ) {
        let name = registry.get_entity_name(r).to_string();
        if let Some(reflex) = registry.reflex_comps[r.0 as usize].as_ref() {
            let mut reflex_items = vec![SExpr::sym("reflex"), SExpr::str_val(&name)];
            let mut on_items = vec![SExpr::sym("on")];
            for g in &reflex.guards {
                let g_name = registry.get_entity_name(*g).to_string();
                on_items.push(SExpr::str_val(&g_name));
            }
            reflex_items.push(SExpr::list(on_items));

            for a in &reflex.assignments {
                if let Some(assign) = registry.assignment_comps[a.0 as usize].as_ref() {
                    let target_name = registry.get_entity_name(assign.target).to_string();
                    let target_sexpr = if let Some(idx) = assign.target_index {
                        SExpr::list(vec![
                            SExpr::sym("index"),
                            SExpr::str_val(&target_name),
                            SExpr::int(idx as u64),
                        ])
                    } else {
                        SExpr::str_val(&target_name)
                    };
                    reflex_items.push(SExpr::list(vec![
                        SExpr::sym("assign"),
                        target_sexpr,
                        convert_expr_ecs(registry, assign.value),
                    ]));
                }
            }
            items.push(SExpr::list(reflex_items));
        }
    }
    SExpr::list(items)
}

fn convert_properties(registry: &Registry, module_entity: EntityId) -> SExpr {
    let mut items = vec![SExpr::sym("properties")];
    for p in get_children_discriminant(
        registry,
        module_entity,
        std::mem::discriminant(&EntityKind::PROPERTY),
    ) {
        let name = registry.get_entity_name(p).to_string();
        if let Some(prop) = registry.property_comps[p.0 as usize].as_ref() {
            items.push(SExpr::list(vec![
                SExpr::sym("property"),
                SExpr::str_val(&name),
                convert_directive(prop.directive),
                convert_formula_ecs(registry, prop),
            ]));
        }
    }
    SExpr::list(items)
}

fn convert_pattern_calls(registry: &Registry, module_entity: EntityId) -> SExpr {
    let mut items = vec![SExpr::sym("pattern-calls")];
    for c in get_children_discriminant(
        registry,
        module_entity,
        std::mem::discriminant(&EntityKind::PATTERN_CALL),
    ) {
        if let Some(call) = registry.pattern_calls[c.0 as usize].as_ref() {
            let mut call_items =
                vec![SExpr::sym("pattern-call"), SExpr::str_val(&call.0.pattern_name)];
            for arg in &call.0.arguments {
                call_items.push(convert_pattern_arg(arg));
            }
            items.push(SExpr::list(call_items));
        }
    }
    SExpr::list(items)
}

fn convert_pattern_origins(_registry: &Registry, _module_entity: EntityId) -> SExpr {
    SExpr::list(vec![SExpr::sym("pattern-origins")])
}

fn convert_directive(d: PropertyDirective) -> SExpr {
    match d {
        PropertyDirective::Assert => SExpr::sym("assert"),
        PropertyDirective::Cover => SExpr::sym("cover"),
        PropertyDirective::Assume => SExpr::sym("assume"),
    }
}

fn convert_formula_ecs(
    registry: &Registry,
    prop: &crate::ecs::components::PropertyComponent,
) -> SExpr {
    match &prop.formula {
        PropertyFormula::Always(_) => SExpr::list(vec![
            SExpr::sym("always"),
            convert_expr_ecs(registry, prop.formula_exprs[0]),
        ]),
        PropertyFormula::Never(_) => SExpr::list(vec![
            SExpr::sym("never"),
            convert_expr_ecs(registry, prop.formula_exprs[0]),
        ]),
        PropertyFormula::AlwaysImplies { .. } => SExpr::list(vec![
            SExpr::sym("always-implies"),
            convert_expr_ecs(registry, prop.formula_exprs[0]),
            convert_expr_ecs(registry, prop.formula_exprs[1]),
        ]),
        PropertyFormula::NeverImplies { .. } => SExpr::list(vec![
            SExpr::sym("never-implies"),
            convert_expr_ecs(registry, prop.formula_exprs[0]),
            convert_expr_ecs(registry, prop.formula_exprs[1]),
        ]),
        PropertyFormula::EventuallyWithin { cycles, .. } => SExpr::list(vec![
            SExpr::sym("eventually-within"),
            convert_expr_ecs(registry, prop.formula_exprs[0]),
            SExpr::int(*cycles as u64),
        ]),
        PropertyFormula::AlwaysFollowedBy { delay_cycles, .. } => SExpr::list(vec![
            SExpr::sym("always-followed-by"),
            convert_expr_ecs(registry, prop.formula_exprs[0]),
            convert_expr_ecs(registry, prop.formula_exprs[1]),
            SExpr::int(*delay_cycles as u64),
        ]),
    }
}

fn convert_expr_ecs(registry: &Registry, expr_entity: EntityId) -> SExpr {
    if let Some(lit) = registry.literals.get(expr_entity.0 as usize).and_then(|c| c.as_ref()) {
        match lit.0 {
            LiteralValue::Bool(b) => SExpr::Bool(b),
            LiteralValue::Integer(n) => SExpr::Integer(n),
        }
    } else if let Some(sig_ref) =
        registry.signal_refs.get(expr_entity.0 as usize).and_then(|c| c.as_ref())
    {
        let name = registry.get_entity_name(sig_ref.0).to_string();
        SExpr::list(vec![SExpr::sym("signal"), SExpr::str_val(&name)])
    } else if let Some(bin) =
        registry.binary_ops.get(expr_entity.0 as usize).and_then(|c| c.as_ref())
    {
        let op_sym = binop_to_symbol(bin.op);
        SExpr::list(vec![
            SExpr::sym(op_sym),
            convert_expr_ecs(registry, bin.left),
            convert_expr_ecs(registry, bin.right),
        ])
    } else if let Some(un) = registry.unary_ops.get(expr_entity.0 as usize).and_then(|c| c.as_ref())
    {
        let op_sym = match un.op {
            UnaryOp::Not => "not",
            UnaryOp::Negate => "negate",
            UnaryOp::ReductionOr => "reduce_or",
        };
        SExpr::list(vec![SExpr::sym(op_sym), convert_expr_ecs(registry, un.operand)])
    } else if let Some(prev) =
        registry.prev_ops.get(expr_entity.0 as usize).and_then(|c| c.as_ref())
    {
        let name = registry.get_entity_name(prev.signal).to_string();
        SExpr::list(vec![SExpr::sym("prev"), SExpr::str_val(&name), SExpr::int(prev.delay)])
    } else {
        // Assume direct signal reference
        let name = registry.get_entity_name(expr_entity).to_string();
        if name != "<unnamed>" && !name.is_empty() {
            SExpr::list(vec![SExpr::sym("signal"), SExpr::str_val(&name)])
        } else {
            SExpr::Bool(false)
        }
    }
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
            for (fname, ftype) in fields {
                items.push(SExpr::list(vec![SExpr::str_val(&fname), convert_signal_type(ftype)]));
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
