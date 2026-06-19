#![forbid(unsafe_code)]
//! Tests for AST-to-ECS hydration covering 50 unique scenarios.

use mirrc::ast::types::{
    BinaryOp, EffectQualifier, ExtendedType, Linearity, Refinement, SignalKind, SignalType, UnaryOp,
};
use mirrc::ast::Expr;
use mirrc::ecs::adapter::ingest_program;
use mirrc::ecs::components::*;
use mirrc::ecs::registry::Registry;
use mirrc::parser::module_parser::parse_mirr;

fn hydrate(src: &str) -> Registry {
    let program = parse_mirr(src).expect("Source parse failed");
    let mut registry = Registry::new();
    ingest_program(&mut registry, program, None).expect("ECS hydration failed");
    registry
}

macro_rules! test_hyd {
    ($($name:ident, $src:expr, $assert_fn:expr);* $(;)?) => {
        $(
            #[test]
            fn $name() {
                let reg = hydrate($src);
                let check = $assert_fn;
                check(&reg);
            }
        )*
    };
}

test_hyd! {
    // --- 10 Signal Hydration Tests ---
    hyd_sig_in_bool,
    "module M { signal a: in bool; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::a").expect("Missing M::a");
        assert!(matches!(r.kinds[ent.0 as usize].as_ref().unwrap().0, EntityKind::SIGNAL(SignalKind::Input)));
        assert!(matches!(r.types[ent.0 as usize].as_ref().unwrap().0, ExtendedType { core: SignalType::Bool, .. }));
    };

    hyd_sig_out_u16,
    "module M { signal b: out u16; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::b").expect("Missing M::b");
        assert!(matches!(r.kinds[ent.0 as usize].as_ref().unwrap().0, EntityKind::SIGNAL(SignalKind::Output)));
        assert!(matches!(r.types[ent.0 as usize].as_ref().unwrap().0, ExtendedType { core: SignalType::Unsigned(16), .. }));
    };

    hyd_sig_internal_u32,
    "module M { signal c: internal u32; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::c").expect("Missing M::c");
        assert!(matches!(r.kinds[ent.0 as usize].as_ref().unwrap().0, EntityKind::SIGNAL(SignalKind::Internal)));
        assert!(matches!(r.types[ent.0 as usize].as_ref().unwrap().0, ExtendedType { core: SignalType::Unsigned(32), .. }));
    };

    hyd_sig_refinement_less,
    "module M { signal a: in u8 where x < 10; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::a").expect("Missing M::a");
        let ty = &r.types[ent.0 as usize].as_ref().unwrap().0;
        assert!(ty.annotations.refinement.is_some());
    };

    hyd_sig_refinement_greater,
    "module M { signal a: in u8 where x > 5; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::a").expect("Missing M::a");
        let ty = &r.types[ent.0 as usize].as_ref().unwrap().0;
        assert!(ty.annotations.refinement.is_some());
    };

    hyd_sig_refinement_range,
    "module M { signal a: in u8 where 0..100; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::a").expect("Missing M::a");
        let ty = &r.types[ent.0 as usize].as_ref().unwrap().0;
        assert!(matches!(ty.annotations.refinement.as_ref().unwrap(), Refinement::Range { lo: 0, hi: 100 }));
    };

    hyd_sig_clock_domain,
    "module M { signal a: in bool @clk; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::a").expect("Missing M::a");
        let ty = &r.types[ent.0 as usize].as_ref().unwrap().0;
        assert_eq!(ty.annotations.clock_domain.as_deref(), Some("clk"));
    };

    hyd_sig_phantom,
    "module M { signal a: in bool #Tag; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::a").expect("Missing M::a");
        let ty = &r.types[ent.0 as usize].as_ref().unwrap().0;
        assert_eq!(ty.annotations.phantom_tag.as_deref(), Some("Tag"));
    };

    hyd_sig_linear,
    "module M { signal a: in linear bool; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::a").expect("Missing M::a");
        let ty = &r.types[ent.0 as usize].as_ref().unwrap().0;
        assert_eq!(ty.annotations.linearity, Linearity::Linear);
    };

    hyd_sig_pure,
    "module M { signal a: in pure bool; }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::a").expect("Missing M::a");
        let ty = &r.types[ent.0 as usize].as_ref().unwrap().0;
        assert_eq!(ty.annotations.effect, EffectQualifier::Pure);
    };

    // --- 10 Guard Hydration Tests ---
    hyd_guard_basic,
    "module M { signal a: in bool; guard g { when a for 1 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        assert!(matches!(r.kinds[ent.0 as usize].as_ref().unwrap().0, EntityKind::GUARD));
        assert_eq!(r.cycles[ent.0 as usize].unwrap().0, 1);
        assert!(r.conditions[ent.0 as usize].is_some());
    };

    hyd_guard_cycles,
    "module M { signal a: in bool; guard g { when a for 42 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        assert_eq!(r.cycles[ent.0 as usize].unwrap().0, 42);
    };

    hyd_guard_binary_cond,
    "module M { signal a: in bool; signal b: in bool; guard g { when a && b for 1 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        let cond = r.conditions[ent.0 as usize].unwrap().0;
        assert!(r.binary_ops[cond.0 as usize].is_some());
    };

    hyd_guard_unary_cond,
    "module M { signal a: in bool; guard g { when !a for 1 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        let cond = r.conditions[ent.0 as usize].unwrap().0;
        assert!(r.unary_ops[cond.0 as usize].is_some());
    };

    hyd_guard_prev_cond,
    "module M { signal a: in bool; guard g { when prev(a, 3) for 1 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        let cond = r.conditions[ent.0 as usize].unwrap().0;
        assert!(r.prev_ops[cond.0 as usize].is_some());
    };

    hyd_guard_literal_cond,
    "module M { guard g { when true for 5 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        let cond = r.conditions[ent.0 as usize].unwrap().0;
        assert!(r.literals[cond.0 as usize].is_some());
    };

    hyd_guard_nested_module,
    "module A::B { signal x: in bool; guard g { when x for 1 cycles; } }",
    |r: &Registry| {
        assert!(r.get_entity_by_name("A::B::g").is_some());
    };

    hyd_guard_hex_comp,
    "module M { signal x: in u8; guard g { when x == 0x0F for 1 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        let cond = r.conditions[ent.0 as usize].unwrap().0;
        assert!(r.binary_ops[cond.0 as usize].is_some());
    };

    hyd_guard_dec_comp,
    "module M { signal x: in u8; guard g { when x == 10 for 1 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        let cond = r.conditions[ent.0 as usize].unwrap().0;
        assert!(r.binary_ops[cond.0 as usize].is_some());
    };

    hyd_guard_complex_expr,
    "module M { signal a: in bool; signal b: in bool; guard g { when (a && !b) || prev(a, 1) for 1 cycles; } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::g").expect("Missing M::g");
        let cond = r.conditions[ent.0 as usize].unwrap().0;
        assert!(r.binary_ops[cond.0 as usize].is_some());
    };

    // --- 10 Reflex Hydration Tests ---
    hyd_reflex_basic,
    "module M { signal a: in bool; signal b: out bool; guard g { when a for 1 cycles; } reflex r { on g { b = a; } } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::r").expect("Missing M::r");
        assert!(matches!(r.kinds[ent.0 as usize].as_ref().unwrap().0, EntityKind::REFLEX));
        let rc = r.reflex_comps[ent.0 as usize].as_ref().unwrap();
        assert_eq!(rc.guards.len(), 1);
        assert_eq!(rc.assignments.len(), 1);
    };

    hyd_reflex_always,
    "module M { signal b: out bool; reflex r { on always { b = true; } } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::r").expect("Missing M::r");
        let rc = r.reflex_comps[ent.0 as usize].as_ref().unwrap();
        assert_eq!(rc.guards.len(), 1);
        assert_eq!(r.resolve_name(r.names[rc.guards[0].0 as usize].as_ref().unwrap().0), "always");
    };

    hyd_reflex_multi_guards,
    "module M { signal a: in bool; signal b: in bool; signal c: out bool; guard g1 { when a for 1 cycles; } guard g2 { when b for 1 cycles; } reflex r { on g1 and g2 { c = true; } } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::r").expect("Missing M::r");
        let rc = r.reflex_comps[ent.0 as usize].as_ref().unwrap();
        assert_eq!(rc.guards.len(), 2);
    };

    hyd_reflex_multi_assigns,
    "module M { signal a: in bool; signal b: out bool; signal c: out bool; guard g { when a for 1 cycles; } reflex r { on g { b = a; c = false; } } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::r").expect("Missing M::r");
        let rc = r.reflex_comps[ent.0 as usize].as_ref().unwrap();
        assert_eq!(rc.assignments.len(), 2);
    };

    hyd_reflex_assignment_details,
    "module M { signal a: in bool; signal b: out bool; guard g { when a for 1 cycles; } reflex r { on g { b = a; } } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::r").expect("Missing M::r");
        let rc = r.reflex_comps[ent.0 as usize].as_ref().unwrap();
        let assign_ent = rc.assignments[0];
        assert!(matches!(r.kinds[assign_ent.0 as usize].as_ref().unwrap().0, EntityKind::ASSIGNMENT));
        let ac = r.assignment_comps[assign_ent.0 as usize].as_ref().unwrap();
        let b_sig = r.get_entity_by_name("M::b").unwrap();
        assert_eq!(ac.target, b_sig);
    };

    hyd_reflex_complex_assign,
    "module M { signal a: in u8; signal b: out u8; guard g { when a > 0 for 1 cycles; } reflex r { on g { b = a + 1; } } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::r").expect("Missing M::r");
        let rc = r.reflex_comps[ent.0 as usize].as_ref().unwrap();
        let ac = r.assignment_comps[rc.assignments[0].0 as usize].as_ref().unwrap();
        assert!(r.binary_ops[ac.value.0 as usize].is_some());
    };

    hyd_reflex_always_assign_literal,
    "module M { signal b: out u8; reflex r { on always { b = 42; } } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::r").expect("Missing M::r");
        let rc = r.reflex_comps[ent.0 as usize].as_ref().unwrap();
        let ac = r.assignment_comps[rc.assignments[0].0 as usize].as_ref().unwrap();
        assert!(r.literals[ac.value.0 as usize].is_some());
    };

    hyd_reflex_assignment_to_internal,
    "module M { signal a: in bool; signal b: internal bool; guard g { when a for 1 cycles; } reflex r { on g { b = a; } } }",
    |r: &Registry| {
        let b_sig = r.get_entity_by_name("M::b").expect("Missing M::b");
        assert!(matches!(r.kinds[b_sig.0 as usize].as_ref().unwrap().0, EntityKind::SIGNAL(SignalKind::Internal)));
    };

    hyd_reflex_qualified_guard,
    "module M { signal a: in bool; guard g { when a for 1 cycles; } reflex r { on M::g { a = false; } } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::r").expect("Missing M::r");
        let rc = r.reflex_comps[ent.0 as usize].as_ref().unwrap();
        assert_eq!(rc.guards.len(), 1);
    };

    hyd_reflex_nested_scope,
    "module A::B { signal a: in bool; signal b: out bool; guard g { when a for 1 cycles; } reflex r { on g { b = a; } } }",
    |r: &Registry| {
        assert!(r.get_entity_by_name("A::B::r").is_some());
    };

    // --- 10 Property Hydration Tests ---
    hyd_prop_always,
    "module M { signal a: in bool; property p { always (a); } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::p").expect("Missing M::p");
        assert!(matches!(r.kinds[ent.0 as usize].as_ref().unwrap().0, EntityKind::PROPERTY));
        assert!(r.property_comps[ent.0 as usize].is_some());
    };

    hyd_prop_never,
    "module M { signal a: in bool; property p { never (a); } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::p").expect("Missing M::p");
        assert!(r.property_comps[ent.0 as usize].is_some());
    };

    hyd_prop_sometimes,
    "module M { signal a: in bool; property p { eventually within 10 (a); } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::p").expect("Missing M::p");
        assert!(r.property_comps[ent.0 as usize].is_some());
    };

    hyd_prop_implication,
    "module M { signal a: in bool; signal b: in bool; property p { always (a -> b); } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::p").expect("Missing M::p");
        let pc = r.property_comps[ent.0 as usize].as_ref().unwrap();
        assert_eq!(pc.formula_exprs.len(), 2);
    };

    hyd_prop_complex_formula,
    "module M { signal a: in bool; signal b: in bool; property p { always (a && !b -> prev(a, 1)); } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::p").expect("Missing M::p");
        let pc = r.property_comps[ent.0 as usize].as_ref().unwrap();
        assert_eq!(pc.formula_exprs.len(), 2);
    };

    hyd_prop_nested,
    "module A::B { signal a: in bool; property p { always (a); } }",
    |r: &Registry| {
        assert!(r.get_entity_by_name("A::B::p").is_some());
    };

    hyd_prop_multiple,
    "module M { signal a: in bool; property p1 { always (a); } property p2 { never (!a); } }",
    |r: &Registry| {
        assert!(r.get_entity_by_name("M::p1").is_some());
        assert!(r.get_entity_by_name("M::p2").is_some());
    };

    hyd_prop_with_prev,
    "module M { signal a: in bool; property p { always (prev(a, 5)); } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::p").expect("Missing M::p");
        let pc = r.property_comps[ent.0 as usize].as_ref().unwrap();
        assert!(r.prev_ops[pc.formula_exprs[0].0 as usize].is_some());
    };

    hyd_prop_with_comp,
    "module M { signal a: in u8; property p { always (a == 0); } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::p").expect("Missing M::p");
        let pc = r.property_comps[ent.0 as usize].as_ref().unwrap();
        assert!(r.binary_ops[pc.formula_exprs[0].0 as usize].is_some());
    };

    hyd_prop_always_true,
    "module M { property p { always (true); } }",
    |r: &Registry| {
        let ent = r.get_entity_by_name("M::p").expect("Missing M::p");
        let pc = r.property_comps[ent.0 as usize].as_ref().unwrap();
        assert!(r.literals[pc.formula_exprs[0].0 as usize].is_some());
    };

    // --- 10 Expression Flattening & Reification Tests ---
    hyd_expr_reify_literal,
    "module M { guard g { when true for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Literal(_)));
    };

    hyd_expr_reify_signal_ref,
    "module M { signal a: in bool; guard g { when a for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Signal(_)));
    };

    hyd_expr_reify_binary_op,
    "module M { signal a: in bool; signal b: in bool; guard g { when a && b for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Binary { op: BinaryOp::And, .. }));
    };

    hyd_expr_reify_unary_op,
    "module M { signal a: in bool; guard g { when !a for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Unary { op: UnaryOp::Not, .. }));
    };

    hyd_expr_reify_prev_op,
    "module M { signal a: in bool; guard g { when prev(a, 2) for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Prev { delay: 2, .. }));
    };

    hyd_expr_reify_complex_binary,
    "module M { signal a: in bool; signal b: in bool; guard g { when (a && b) || !a for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Binary { op: BinaryOp::Or, .. }));
    };

    hyd_expr_reify_nested_prev,
    "module M { signal a: in bool; guard g { when prev(a, 3) && !a for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Binary { .. }));
    };

    hyd_expr_reify_multiple_signals,
    "module M { signal a: in bool; signal b: in bool; signal c: in bool; guard g { when a && b && c for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Binary { .. }));
    };

    hyd_expr_reify_arithmetic_comp,
    "module M { signal x: in u8; guard g { when x == 42 for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Binary { op: BinaryOp::Eq, .. }));
    };

    hyd_expr_reify_arithmetic_ineq,
    "module M { signal x: in u8; guard g { when x < 10 for 1 cycles; } }",
    |r: &Registry| {
        let g = r.get_entity_by_name("M::g").unwrap();
        let cond = r.conditions[g.0 as usize].unwrap().0;
        let expr = r.reify_expr(cond).unwrap();
        assert!(matches!(expr, Expr::Binary { op: BinaryOp::Lt, .. }));
    };
}
