#![forbid(unsafe_code)]

use mirrc::ast::types::{
    BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType, UnaryOp,
};
use mirrc::ast::Expr;
use mirrc::ecs::components::*;
use mirrc::ecs::systems::*;
use mirrc::ecs::Registry;
use mirrc::parse_mirr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_registry_construction_and_basic_entity_ops() {
    let mut reg = Registry::new();
    assert_eq!(reg.next_id().0, 0);
    assert_eq!(reg.next_id().0, 1);
    assert_eq!(reg.next_id().0, 2);

    let ent = reg.create_entity("my_sig", KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
    assert_eq!(reg.names[ent.0 as usize].as_ref().unwrap().0, "my_sig");
    assert!(matches!(reg.kinds[ent.0 as usize].unwrap().0, EntityKind::SIGNAL(_)));
    assert_eq!(reg.get_entity_by_name("my_sig"), Some(ent));

    let ty = ExtendedType::new(SignalType::Bool, Default::default());
    reg.set_type(ent, TypeComponent(ty.clone()));
    assert_eq!(reg.types[ent.0 as usize].as_ref().unwrap().0, ty);

    let mod_ent = reg.create_entity("my_mod", KindComponent(EntityKind::MODULE));
    reg.set_parent(ent, mod_ent);
    assert_eq!(reg.modules[ent.0 as usize].unwrap().0, mod_ent);

    let kb_id =
        reg.create_kb_chunk("kb_1".into(), "test".into(), "src.rs".into(), (1, 2), Some(vec![0.5]));
    assert_eq!(reg.chunk_texts[kb_id.0 as usize].as_ref().unwrap().0, "test");
    assert_eq!(reg.source_paths[kb_id.0 as usize].as_ref().unwrap().0, "src.rs");
    assert_eq!(reg.line_ranges[kb_id.0 as usize].unwrap().0, (1, 2));
    assert_eq!(reg.vectors[kb_id.0 as usize].as_ref().unwrap().0, vec![0.5]);
}

#[test]
fn test_ingest_expr_basic() {
    let mut reg = Registry::new();

    // Literals
    let e_true = reg.ingest_expr(&Expr::Literal(LiteralValue::Bool(true))).expect("ingest");
    assert_eq!(reg.literals[e_true.0 as usize].as_ref().unwrap().0, LiteralValue::Bool(true));

    let e_int = reg.ingest_expr(&Expr::Literal(LiteralValue::Integer(42))).expect("ingest");
    assert_eq!(reg.literals[e_int.0 as usize].as_ref().unwrap().0, LiteralValue::Integer(42));

    // Known Signal
    let s_ent = reg.create_entity("known_sig", KindComponent::SIGNAL);
    let e_sig = reg.ingest_expr(&Expr::Signal("known_sig".into())).expect("ingest");
    assert_eq!(reg.signal_refs[e_sig.0 as usize].unwrap().0, s_ent);

    // Unknown Signal
    let e_unk = reg.ingest_expr(&Expr::Signal("unknown".into())).expect("ingest");
    assert_eq!(reg.pending_signal_refs[e_unk.0 as usize].as_ref().unwrap().0, "unknown");

    // Unary
    let e_not = reg
        .ingest_expr(&Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Signal("known_sig".into())),
        })
        .expect("ingest");
    let un_comp = reg.unary_ops[e_not.0 as usize].unwrap();
    assert_eq!(un_comp.op, UnaryOp::Not);

    // Binary
    let e_bin = reg
        .ingest_expr(&Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Literal(LiteralValue::Integer(1))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(2))),
        })
        .expect("ingest");
    let bin_comp = reg.binary_ops[e_bin.0 as usize].unwrap();
    assert_eq!(bin_comp.op, BinaryOp::Add);

    // Field Access & Array Index
    let e_field = reg
        .ingest_expr(&Expr::FieldAccess {
            object: Box::new(Expr::Signal("obj".into())),
            field: "f".into(),
        })
        .expect("ingest");
    assert_eq!(reg.field_accesses[e_field.0 as usize].as_ref().unwrap().field, "f");

    let e_idx = reg
        .ingest_expr(&Expr::ArrayIndex {
            array: Box::new(Expr::Signal("arr".into())),
            index: Box::new(Expr::Literal(LiteralValue::Integer(0))),
        })
        .expect("ingest");
    assert!(reg.array_indices[e_idx.0 as usize].is_some());

    // Array Literal
    let e_arr = reg
        .ingest_expr(&Expr::ArrayLiteral(vec![Expr::Literal(LiteralValue::Integer(1))]))
        .expect("ingest");
    assert_eq!(reg.array_literals[e_arr.0 as usize].as_ref().unwrap().0.len(), 1);

    // Unfold index
    let e_unf = reg.ingest_expr(&Expr::UnfoldIndex("i".into())).expect("ingest");
    assert_eq!(reg.unfold_indices[e_unf.0 as usize].as_ref().unwrap().0, "i");
}

#[test]
fn test_reify_expr_roundtrips() {
    let mut reg = Registry::new();

    let exprs = vec![
        Expr::Literal(LiteralValue::Bool(true)),
        Expr::Literal(LiteralValue::Integer(99)),
        Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Literal(LiteralValue::Bool(false))),
        },
        Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Literal(LiteralValue::Integer(1))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(2))),
        },
        Expr::Binary {
            op: BinaryOp::Or,
            left: Box::new(Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Signal("a".into())),
                right: Box::new(Expr::Signal("b".into())),
            }),
            right: Box::new(Expr::Signal("c".into())),
        },
        Expr::ArrayLiteral(vec![
            Expr::Literal(LiteralValue::Integer(1)),
            Expr::Literal(LiteralValue::Integer(2)),
            Expr::Literal(LiteralValue::Integer(3)),
        ]),
        Expr::FieldAccess { object: Box::new(Expr::Signal("obj".into())), field: "f".into() },
        Expr::ArrayIndex {
            array: Box::new(Expr::Signal("arr".into())),
            index: Box::new(Expr::Literal(LiteralValue::Integer(0))),
        },
    ];

    for e in exprs {
        let ent = reg.ingest_expr(&e).expect("ingest");
        let reified = reg.reify_expr(ent).expect("reify");
        // For signals without entity declaration, they are stored as pending and reify back perfectly.
        assert_eq!(reified, e);

        // Test memoization
        let reified2 = reg.reify_expr(ent).expect("reify2");
        assert_eq!(reified2, e);
    }
}

#[test]
fn test_reify_depth_limit() {
    let mut reg = Registry::new();
    let mut prev_id = reg.create_entity("lit", KindComponent::SIGNAL);
    reg.literals[prev_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));

    // Build 65-deep binary tree manually to trigger depth limit
    for _ in 0..65 {
        let bin_id = reg.next_id();
        let idx = bin_id.0 as usize;
        reg.binary_ops[idx] =
            Some(BinaryComponent { op: BinaryOp::And, left: prev_id, right: prev_id });
        prev_id = bin_id;
    }

    let res = reg.reify_expr(prev_id);
    assert!(res.is_err(), "Should error on depth > 64");
    assert!(res.unwrap_err().to_string().contains("depth"));
}

#[test]
fn test_ingest_module_integration() {
    let src = r#"
        module m {
            signal x: in bool;
            signal y: out bool;
            guard g { when x for 1 cycles; }
            reflex r { on g { y = true; } }
            property p { always(x -> y); }
        }
    "#;
    let prog = parse_mirr(src).expect("parse");
    let mut reg = Registry::new();
    let mod_id = reg.ingest_module(&prog.module).expect("ingest module");

    assert_eq!(reg.kinds[mod_id.0 as usize].unwrap().0, EntityKind::MODULE);
    assert!(reg.get_entity_by_name("m").is_some());

    let sig_x = reg.get_entity_by_name("x").unwrap();
    assert_eq!(reg.names[sig_x.0 as usize].as_ref().unwrap().0, "x");
    assert!(reg.get_entity_by_name("m::x").is_some());

    let g_id = reg.get_entity_by_name("g").unwrap();
    assert_eq!(reg.cycles[g_id.0 as usize].unwrap().0, 1);
    assert!(reg.conditions[g_id.0 as usize].is_some());

    let r_id = reg.get_entity_by_name("r").unwrap();
    let r_comp = reg.reflex_comps[r_id.0 as usize].as_ref().unwrap();
    assert_eq!(r_comp.guards[0], g_id);

    let p_id = reg.get_entity_by_name("p").unwrap();
    assert!(reg.property_comps[p_id.0 as usize].is_some());
}

#[test]
fn test_ecs_systems_parallel_folding() {
    let mut reg = Registry::new();

    let e_true = reg.ingest_expr(&Expr::Literal(LiteralValue::Bool(true))).unwrap();
    let e_false = reg.ingest_expr(&Expr::Literal(LiteralValue::Bool(false))).unwrap();
    let e_3 = reg.ingest_expr(&Expr::Literal(LiteralValue::Integer(3))).unwrap();
    let e_5 = reg.ingest_expr(&Expr::Literal(LiteralValue::Integer(5))).unwrap();

    let b_and = reg.next_id();
    reg.binary_ops[b_and.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::And, left: e_true, right: e_false });

    let b_or = reg.next_id();
    reg.binary_ops[b_or.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::Or, left: e_false, right: e_true });

    let b_add = reg.next_id();
    reg.binary_ops[b_add.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::Add, left: e_3, right: e_5 });

    let b_eq = reg.next_id();
    reg.binary_ops[b_eq.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::Eq, left: e_3, right: e_3 });

    let b_mixed = reg.next_id();
    reg.binary_ops[b_mixed.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::And, left: e_3, right: e_true });

    parallel_constant_folding_system(&mut reg);

    assert_eq!(reg.literals[b_and.0 as usize].as_ref().unwrap().0, LiteralValue::Bool(false));
    assert_eq!(reg.literals[b_or.0 as usize].as_ref().unwrap().0, LiteralValue::Bool(true));
    assert_eq!(reg.literals[b_add.0 as usize].as_ref().unwrap().0, LiteralValue::Integer(8));
    assert_eq!(reg.literals[b_eq.0 as usize].as_ref().unwrap().0, LiteralValue::Bool(true));
    // Mixed does not fold
    assert!(reg.literals[b_mixed.0 as usize].is_none());
}

#[test]
fn test_ecs_systems_vector_search() {
    let mut reg = Registry::new();

    let id1 =
        reg.create_kb_chunk("kb1".into(), "1".into(), "".into(), (0, 0), Some(vec![1.0, 0.0, 0.0]));
    let _id2 =
        reg.create_kb_chunk("kb2".into(), "2".into(), "".into(), (0, 0), Some(vec![0.0, 1.0, 0.0]));
    let id3 =
        reg.create_kb_chunk("kb3".into(), "3".into(), "".into(), (0, 0), Some(vec![1.0, 0.1, 0.0]));

    let res = parallel_vector_search_system(&reg, &[1.0, 0.0, 0.0], 2);
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].0, id1);
    assert!(res[0].1 > 0.99); // Identical
    assert_eq!(res[1].0, id3);

    let res_empty = parallel_vector_search_system(&reg, &[], 2);
    assert_eq!(res_empty.len(), 2);

    let res_orth = parallel_vector_search_system(&reg, &[0.0, 0.0, 1.0], 2);
    assert_eq!(res_orth.len(), 2);
    assert_eq!(res_orth[0].1, 0.0); // Orthogonal
}

#[test]
fn test_ecs_systems_width_and_pipeline() {
    let mut reg = Registry::new();
    let stats = parallel_width_inference_system(&mut reg);
    assert_eq!(stats.3.nodes_analyzed, 0);

    let src = r#"module m { signal x: in bool; }"#;
    let res = run_pipeline(src, &PipelineConfig::default());
    assert!(res.is_ok());
}

#[test]
fn test_semantic_validate_adversarial_paths() {
    let mut reg = Registry::new();

    let mod_id = reg.next_id();
    reg.names[mod_id.0 as usize] = Some(NameComponent("mymod".into()));
    reg.kinds[mod_id.0 as usize] = Some(KindComponent::MODULE);

    // Duplicate signals
    let s1 = reg.next_id();
    reg.names[s1.0 as usize] = Some(NameComponent("sig".into()));
    reg.kinds[s1.0 as usize] = Some(KindComponent::SIGNAL);
    reg.modules[s1.0 as usize] = Some(ModuleComponent(mod_id));

    let s2 = reg.next_id();
    reg.names[s2.0 as usize] = Some(NameComponent("sig".into()));
    reg.kinds[s2.0 as usize] = Some(KindComponent::SIGNAL);
    reg.modules[s2.0 as usize] = Some(ModuleComponent(mod_id));

    // Duplicate guards
    let g1 = reg.next_id();
    reg.names[g1.0 as usize] = Some(NameComponent("g".into()));
    reg.kinds[g1.0 as usize] = Some(KindComponent::GUARD);
    reg.modules[g1.0 as usize] = Some(ModuleComponent(mod_id));

    let g2 = reg.next_id();
    reg.names[g2.0 as usize] = Some(NameComponent("g".into()));
    reg.kinds[g2.0 as usize] = Some(KindComponent::GUARD);
    reg.modules[g2.0 as usize] = Some(ModuleComponent(mod_id));

    // Duplicate reflexes
    let r1 = reg.next_id();
    reg.names[r1.0 as usize] = Some(NameComponent("r".into()));
    reg.kinds[r1.0 as usize] = Some(KindComponent::REFLEX);
    reg.modules[r1.0 as usize] = Some(ModuleComponent(mod_id));

    let r2 = reg.next_id();
    reg.names[r2.0 as usize] = Some(NameComponent("r".into()));
    reg.kinds[r2.0 as usize] = Some(KindComponent::REFLEX);
    reg.modules[r2.0 as usize] = Some(ModuleComponent(mod_id));

    // Guard missing condition and cycles
    let g_bad = reg.next_id();
    reg.names[g_bad.0 as usize] = Some(NameComponent("g_bad".into()));
    reg.kinds[g_bad.0 as usize] = Some(KindComponent::GUARD);
    reg.modules[g_bad.0 as usize] = Some(ModuleComponent(mod_id));

    // Non-guard has condition
    let s_bad = reg.next_id();
    reg.names[s_bad.0 as usize] = Some(NameComponent("s_bad".into()));
    reg.kinds[s_bad.0 as usize] = Some(KindComponent::SIGNAL);
    reg.modules[s_bad.0 as usize] = Some(ModuleComponent(mod_id));
    reg.conditions[s_bad.0 as usize] = Some(ConditionComponent(EntityId(0)));

    // Reflex invalid guard ref
    let ref_bad = reg.next_id();
    reg.names[ref_bad.0 as usize] = Some(NameComponent("ref_bad".into()));
    reg.kinds[ref_bad.0 as usize] = Some(KindComponent::REFLEX);
    reg.modules[ref_bad.0 as usize] = Some(ModuleComponent(mod_id));
    reg.reflex_comps[ref_bad.0 as usize] =
        Some(ReflexComponent { guards: vec![EntityId(9999)], assignments: vec![] }); // Past max_id

    // Broken leaf expr
    let l_bad = reg.next_id();
    reg.names[l_bad.0 as usize] = Some(NameComponent("expr_bad".into()));
    reg.kinds[l_bad.0 as usize] = Some(KindComponent(EntityKind::ASSIGNMENT));

    let errors = reg.semantic_validate().unwrap_err();
    let err_str = format!("{:?}", errors);

    assert!(err_str.contains("E201"), "Duplicate signal missing E201");
    assert!(err_str.contains("E213"), "Duplicate guard missing E213");
    assert!(err_str.contains("E212"), "Duplicate reflex missing E212");
    assert!(err_str.contains("E306"), "Guard missing condition missing E306");
    assert!(err_str.contains("E200"), "Non-guard with condition missing E200");
    assert!(err_str.contains("E205"), "Invalid guard ref missing E205");
}

#[test]
fn test_temporal_synthesis_system_paths() {
    // Empty registry -> ok
    let mut reg = Registry::new();
    assert!(temporal_synthesis_system(&mut reg).is_ok());

    // Short delay -> shift register
    let src = r#"
        module m {
            signal x: in bool;
            guard g_short { when x for 10 cycles; }
        }
    "#;
    let prog = parse_mirr(src).expect("parse");
    reg.ingest_module(&prog.module).unwrap();
    let netlist = temporal_synthesis_system(&mut reg).unwrap();
    assert!(!netlist.guards.is_empty());

    let g_id = reg.get_entity_by_name("g_short").unwrap();
    let t_node = reg.temporal_nodes[g_id.0 as usize].as_ref().unwrap();
    assert!(matches!(t_node.strategy, TemporalStrategy::ShiftRegister));
    assert_eq!(t_node.delay_cycles, 10);
    assert!(!t_node.generated_signals.is_empty());

    // Long delay -> counter
    let src_long = r#"
        module m2 {
            signal x: in bool;
            guard g_long { when x for 20 cycles; }
        }
    "#;
    let prog2 = parse_mirr(src_long).expect("parse");
    let mut reg2 = Registry::new();
    reg2.ingest_module(&prog2.module).unwrap();
    temporal_synthesis_system(&mut reg2).unwrap();

    let g2_id = reg2.get_entity_by_name("g_long").unwrap();
    let t_node2 = reg2.temporal_nodes[g2_id.0 as usize].as_ref().unwrap();
    assert!(matches!(t_node2.strategy, TemporalStrategy::Counter { .. }));
}

// --- AUTO GENERATED EXPANSION TESTS ---

macro_rules! test_semantic_err {
    ($name:ident, $src:expr, $err:expr) => {
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let prog = parse_mirr($src)?;
            let mut reg = Registry::new();
            reg.ingest_module(&prog.module)?;
            let errs = reg.semantic_validate().unwrap_err();
            assert!(format!("{:?}", errs).contains($err));
            Ok(())
        }
    };
}
test_semantic_err!(
    test_sem_adv_1,
    "module m { signal x_1: in bool; signal x_1: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_2,
    "module m { signal x_2: in bool; signal x_2: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_3,
    "module m { signal x_3: in bool; signal x_3: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_4,
    "module m { signal x_4: in bool; signal x_4: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_5,
    "module m { signal x_5: in bool; signal x_5: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_6,
    "module m { signal x_6: in bool; signal x_6: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_7,
    "module m { signal x_7: in bool; signal x_7: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_8,
    "module m { signal x_8: in bool; signal x_8: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_9,
    "module m { signal x_9: in bool; signal x_9: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_10,
    "module m { signal x_10: in bool; signal x_10: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_11,
    "module m { signal x_11: in bool; signal x_11: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_12,
    "module m { signal x_12: in bool; signal x_12: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_13,
    "module m { signal x_13: in bool; signal x_13: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_14,
    "module m { signal x_14: in bool; signal x_14: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_15,
    "module m { signal x_15: in bool; signal x_15: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_16,
    "module m { signal x_16: in bool; signal x_16: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_17,
    "module m { signal x_17: in bool; signal x_17: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_18,
    "module m { signal x_18: in bool; signal x_18: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_19,
    "module m { signal x_19: in bool; signal x_19: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_20,
    "module m { signal x_20: in bool; signal x_20: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_21,
    "module m { signal x_21: in bool; signal x_21: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_22,
    "module m { signal x_22: in bool; signal x_22: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_23,
    "module m { signal x_23: in bool; signal x_23: in bool; }",
    "E201"
);
test_semantic_err!(
    test_sem_adv_24,
    "module m { signal x_24: in bool; signal x_24: in bool; }",
    "E201"
);

macro_rules! test_expr_roundtrip {
    ($name:ident, $src:expr) => {
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            use mirrc::parser::parse_expression;
            let expr = parse_expression($src)?;
            let mut reg = Registry::new();
            let ent = reg.ingest_expr(&expr)?;
            let reified = reg.reify_expr(ent)?;
            assert_eq!(expr, reified);
            Ok(())
        }
    };
}
test_expr_roundtrip!(test_expr_rt_1, "(a + 1) * 2");
test_expr_roundtrip!(test_expr_rt_2, "(a + 2) * 3");
test_expr_roundtrip!(test_expr_rt_3, "(a + 3) * 4");
test_expr_roundtrip!(test_expr_rt_4, "(a + 4) * 5");
test_expr_roundtrip!(test_expr_rt_5, "(a + 5) * 6");
test_expr_roundtrip!(test_expr_rt_6, "(a + 6) * 7");
test_expr_roundtrip!(test_expr_rt_7, "(a + 7) * 8");
test_expr_roundtrip!(test_expr_rt_8, "(a + 8) * 9");
test_expr_roundtrip!(test_expr_rt_9, "(a + 9) * 10");
test_expr_roundtrip!(test_expr_rt_10, "(a + 10) * 11");
test_expr_roundtrip!(test_expr_rt_11, "(a + 11) * 12");
test_expr_roundtrip!(test_expr_rt_12, "(a + 12) * 13");
test_expr_roundtrip!(test_expr_rt_13, "(a + 13) * 14");
test_expr_roundtrip!(test_expr_rt_14, "(a + 14) * 15");
test_expr_roundtrip!(test_expr_rt_15, "(a + 15) * 16");
test_expr_roundtrip!(test_expr_rt_16, "(a + 16) * 17");
test_expr_roundtrip!(test_expr_rt_17, "(a + 17) * 18");
test_expr_roundtrip!(test_expr_rt_18, "(a + 18) * 19");
test_expr_roundtrip!(test_expr_rt_19, "(a + 19) * 20");
test_expr_roundtrip!(test_expr_rt_20, "(a + 20) * 21");
test_expr_roundtrip!(test_expr_rt_21, "(a + 21) * 22");
test_expr_roundtrip!(test_expr_rt_22, "(a + 22) * 23");
test_expr_roundtrip!(test_expr_rt_23, "(a + 23) * 24");
test_expr_roundtrip!(test_expr_rt_24, "(a + 24) * 25");
test_expr_roundtrip!(test_expr_rt_25, "(a + 25) * 26");
test_expr_roundtrip!(test_expr_rt_26, "(a + 26) * 27");
test_expr_roundtrip!(test_expr_rt_27, "(a + 27) * 28");
test_expr_roundtrip!(test_expr_rt_28, "(a + 28) * 29");
test_expr_roundtrip!(test_expr_rt_29, "(a + 29) * 30");
test_expr_roundtrip!(test_expr_rt_30, "(a + 30) * 31");
test_expr_roundtrip!(test_expr_rt_31, "(a + 31) * 32");
test_expr_roundtrip!(test_expr_rt_32, "(a + 32) * 33");
test_expr_roundtrip!(test_expr_rt_33, "(a + 33) * 34");
test_expr_roundtrip!(test_expr_rt_34, "(a + 34) * 35");
test_expr_roundtrip!(test_expr_rt_35, "(a + 35) * 36");
test_expr_roundtrip!(test_expr_rt_36, "(a + 36) * 37");

macro_rules! test_sys_fold {
    ($name:ident, $val:expr) => {
        #[test]
        fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let mut reg = Registry::new();
            let left = reg.ingest_expr(&Expr::Literal(LiteralValue::Integer($val)))?;
            let right = reg.ingest_expr(&Expr::Literal(LiteralValue::Integer($val)))?;
            let id = reg.next_id();
            reg.binary_ops[id.0 as usize] =
                Some(BinaryComponent { op: BinaryOp::Add, left, right });
            parallel_constant_folding_system(&mut reg);
            if let Some(res) = &reg.literals[id.0 as usize] {
                assert_eq!(res.0, LiteralValue::Integer($val + $val));
            } else {
                return Err("folding failed".into());
            }
            Ok(())
        }
    };
}
test_sys_fold!(test_sys_fold_1, 1);
test_sys_fold!(test_sys_fold_2, 2);
test_sys_fold!(test_sys_fold_3, 3);
test_sys_fold!(test_sys_fold_4, 4);
test_sys_fold!(test_sys_fold_5, 5);
test_sys_fold!(test_sys_fold_6, 6);
test_sys_fold!(test_sys_fold_7, 7);
test_sys_fold!(test_sys_fold_8, 8);
test_sys_fold!(test_sys_fold_9, 9);
test_sys_fold!(test_sys_fold_10, 10);
test_sys_fold!(test_sys_fold_11, 11);
test_sys_fold!(test_sys_fold_12, 12);
test_sys_fold!(test_sys_fold_13, 13);
test_sys_fold!(test_sys_fold_14, 14);
test_sys_fold!(test_sys_fold_15, 15);
test_sys_fold!(test_sys_fold_16, 16);
test_sys_fold!(test_sys_fold_17, 17);
test_sys_fold!(test_sys_fold_18, 18);
test_sys_fold!(test_sys_fold_19, 19);
test_sys_fold!(test_sys_fold_20, 20);
test_sys_fold!(test_sys_fold_21, 21);
test_sys_fold!(test_sys_fold_22, 22);
test_sys_fold!(test_sys_fold_23, 23);
test_sys_fold!(test_sys_fold_24, 24);
test_sys_fold!(test_sys_fold_25, 25);
test_sys_fold!(test_sys_fold_26, 26);
test_sys_fold!(test_sys_fold_27, 27);
test_sys_fold!(test_sys_fold_28, 28);
test_sys_fold!(test_sys_fold_29, 29);
test_sys_fold!(test_sys_fold_30, 30);
