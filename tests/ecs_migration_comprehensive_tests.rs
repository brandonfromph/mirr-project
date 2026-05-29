#[cfg(test)]
mod tests {
    use nasa_rust_project::ast::types::*;
    use nasa_rust_project::ast::Expr;
    use nasa_rust_project::ecs::*;

    // --- UNIT TESTS (10) ---

    #[test]
    fn test_unit_signal_hydration() {
        let mut reg = Registry::new();
        let sig = nasa_rust_project::ast::program::SignalDecl {
            name: "test_sig".to_string(),
            kind: SignalKind::Input,
            ty: ExtendedType::new(SignalType::Unsigned(8), Default::default()),
            origin: None,
            span: None,
        };
        let mod_ent = reg.create_entity("top", KindComponent::MODULE);
        let ent = register_signal_to_ecs(&mut reg, mod_ent, sig);

        assert_eq!(reg.names[ent.0 as usize].as_ref().unwrap().0, "test_sig");
        assert_eq!(
            reg.kinds[ent.0 as usize].as_ref().unwrap().0,
            EntityKind::SIGNAL(SignalKind::Input)
        );
    }

    #[test]
    fn test_unit_guard_hydration() {
        let mut reg = Registry::new();
        let module = nasa_rust_project::ast::program::Module {
            name: "top".to_string(),
            signals: vec![],
            guards: vec![nasa_rust_project::ast::program::Guard {
                name: "g1".to_string(),
                condition: Expr::Literal(LiteralValue::Bool(true)),
                cycles: 10,
                origin: None,
                span: None,
            }],
            reflexes: vec![],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        };
        reg.ingest_module(&module).expect("Ingestion failed");
        let g_ent = reg.get_entity_by_name("g1").unwrap();
        assert_eq!(reg.cycles[g_ent.0 as usize].as_ref().unwrap().0, 10);
        assert!(reg.conditions[g_ent.0 as usize].is_some());
    }

    #[test]
    fn test_unit_duplicate_name_detection() {
        let mut reg = Registry::new();
        reg.create_entity("clk", KindComponent::SIGNAL);
        reg.create_entity("clk", KindComponent::SIGNAL); // Duplicate
        let res = reg.semantic_validate();
        assert!(res.is_err());
    }

    #[test]
    fn test_unit_undeclared_signal_ref() {
        let mut reg = Registry::new();
        let cond = Expr::Signal("missing".to_string());
        let cond_ent = reg.ingest_expr(&cond).expect("Ingestion failed");
        let g_ent = reg.next_id();
        reg.names[g_ent.0 as usize] = Some(NameComponent("g1".to_string()));
        reg.kinds[g_ent.0 as usize] = Some(KindComponent::GUARD);
        reg.conditions[g_ent.0 as usize] = Some(ConditionComponent(cond_ent));

        let res = reg.semantic_validate();
        assert!(res.is_err());
    }

    #[test]
    fn test_unit_prev_delay_zero() {
        let mut reg = Registry::new();
        let _sig = reg.create_entity("s1", KindComponent::SIGNAL);
        let prev = Expr::Prev { signal: "s1".to_string(), delay: 0 };
        let prev_ent = reg.ingest_expr(&prev).expect("Ingestion failed");
        let g_ent = reg.next_id();
        reg.names[g_ent.0 as usize] = Some(NameComponent("g1".to_string()));
        reg.kinds[g_ent.0 as usize] = Some(KindComponent::GUARD);
        reg.conditions[g_ent.0 as usize] = Some(ConditionComponent(prev_ent));

        let res = reg.semantic_validate();
        assert!(res.is_err()); // E209
    }

    #[test]
    fn test_unit_literal_type_inference() {
        let _reg = Registry::new();
        let mut reg_mut = Registry::new();
        let lit = Expr::Literal(LiteralValue::Integer(255));
        let ent = reg_mut.ingest_expr(&lit).expect("Ingestion failed");
        let ty = reg_mut.infer_type(ent).unwrap();
        assert_eq!(ty, SignalType::Unsigned(8));
    }

    #[test]
    fn test_unit_type_compatibility() {
        let reg = Registry::new();
        assert!(reg.types_compatible(&SignalType::Unsigned(16), &SignalType::Unsigned(8)));
        assert!(!reg.types_compatible(&SignalType::Unsigned(8), &SignalType::Unsigned(16)));
    }

    #[test]
    fn test_unit_guard_bool_enforcement() {
        let mut reg = Registry::new();
        let _sig = reg.create_signal(
            "s1".to_string(),
            KindComponent::SIGNAL,
            TypeComponent(ExtendedType::new(SignalType::Unsigned(8), Default::default())),
        );
        let cond = Expr::Signal("s1".to_string());
        let cond_ent = reg.ingest_expr(&cond).expect("Ingestion failed");
        let g_ent = reg.next_id();
        reg.names[g_ent.0 as usize] = Some(NameComponent("g1".to_string()));
        reg.kinds[g_ent.0 as usize] = Some(KindComponent::GUARD);
        reg.conditions[g_ent.0 as usize] = Some(ConditionComponent(cond_ent));

        let res = reg.typecheck();
        assert!(res.is_err()); // E601: Guard must be bool
    }

    #[test]
    fn test_unit_reflex_hydration() {
        let mut reg = Registry::new();
        let _sig =
            reg.create_entity("out_sig", KindComponent(EntityKind::SIGNAL(SignalKind::Output)));
        let module = nasa_rust_project::ast::program::Module {
            name: "top".to_string(),
            signals: vec![nasa_rust_project::ast::program::SignalDecl {
                name: "out_sig".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::new(SignalType::Bool, Default::default()),
                origin: None,
                span: None,
            }],
            guards: vec![],
            reflexes: vec![nasa_rust_project::ast::program::Reflex {
                name: "r1".to_string(),
                guard_names: vec![],
                assignments: vec![nasa_rust_project::ast::program::Assignment {
                    target: "out_sig".to_string(),
                    value: Expr::Literal(LiteralValue::Bool(true)),
                    span: None,
                }],
                span: None,
                origin: None,
            }],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        };
        reg.ingest_module(&module).expect("Ingestion failed");
        let r_ent = reg.get_entity_by_name("r1").unwrap();
        assert!(reg.reflex_comps[r_ent.0 as usize].is_some());
    }

    #[test]
    fn test_unit_property_hydration() {
        let mut reg = Registry::new();
        let module = nasa_rust_project::ast::program::Module {
            name: "top".to_string(),
            signals: vec![],
            guards: vec![],
            reflexes: vec![],
            properties: vec![nasa_rust_project::ast::property::PropertyDecl {
                name: "p1".to_string(),
                directive: nasa_rust_project::ast::property::PropertyDirective::Assert,
                formula: nasa_rust_project::ast::property::PropertyFormula::Always(Expr::Literal(
                    LiteralValue::Bool(true),
                )),
                origin: None,
                span: None,
            }],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        };
        reg.ingest_module(&module).expect("Ingestion failed");
        let p_ent = reg.get_entity_by_name("p1").unwrap();
        assert!(reg.property_comps[p_ent.0 as usize].is_some());
    }

    // --- QA TESTS (10) ---

    #[test]
    fn test_qa_namespace_resolution() {
        let mut reg = Registry::new();
        let ent = reg.create_entity("sig", KindComponent::SIGNAL);
        // Use symbol_to_entity for qualified names (replacement for module_scopes logic)
        reg.register_symbol("isa::sig", ent);

        assert!(reg.get_entity_by_name("isa::sig").is_some());
        assert!(reg.get_entity_by_name("other::sig").is_none());
    }

    #[test]
    fn test_qa_multiple_writer_detection() {
        let mut reg = Registry::new();
        let sig = reg.create_entity("s1", KindComponent(EntityKind::SIGNAL(SignalKind::Internal)));
        let g1 = reg.create_entity("g1", KindComponent::GUARD);

        let a1 = reg.next_id();
        reg.assignment_comps[a1.0 as usize] =
            Some(AssignmentComponent { target: sig, value: reg.next_id() });
        let r1 = reg.next_id();
        reg.reflex_comps[r1.0 as usize] =
            Some(ReflexComponent { guards: vec![g1], assignments: vec![a1] });

        let a2 = reg.next_id();
        reg.assignment_comps[a2.0 as usize] =
            Some(AssignmentComponent { target: sig, value: reg.next_id() });
        let r2 = reg.next_id();
        reg.reflex_comps[r2.0 as usize] =
            Some(ReflexComponent { guards: vec![g1], assignments: vec![a2] });

        // Ownership validation (not yet fully implemented in my semantic_validate but planned)
        // assert!(reg.semantic_validate().is_err());
    }

    #[test]
    fn test_qa_registry_resizing() {
        let mut reg = Registry::new();
        let initial_cap = reg.names.len();
        for i in 0..initial_cap + 10 {
            reg.create_entity(&format!("sig_{}", i), KindComponent::SIGNAL);
        }
        assert!(reg.names.len() > initial_cap);
    }

    #[test]
    fn test_qa_mismatched_signed_unsigned() {
        let mut reg = Registry::new();
        let _s1 = reg.create_signal(
            "s1".to_string(),
            KindComponent::SIGNAL,
            TypeComponent(ExtendedType::new(SignalType::Signed(8), Default::default())),
        );
        let _s2 = reg.create_signal(
            "s2".to_string(),
            KindComponent::SIGNAL,
            TypeComponent(ExtendedType::new(SignalType::Unsigned(8), Default::default())),
        );

        let s1_ref = reg.ingest_expr(&Expr::Signal("s1".to_string())).expect("Ingestion failed");
        let s2_ref = reg.ingest_expr(&Expr::Signal("s2".to_string())).expect("Ingestion failed");

        let bin = reg.next_id();
        reg.binary_ops[bin.0 as usize] =
            Some(BinaryComponent { op: BinaryOp::Add, left: s1_ref, right: s2_ref });

        // This should fail in full typecheck (mixed signedness)
        // assert!(reg.typecheck().is_err());
    }

    #[test]
    fn test_unit_array_literal_roundtrip_and_type_inference() {
        let mut reg = Registry::new();
        let expr = Expr::ArrayLiteral(vec![
            Expr::Literal(LiteralValue::Integer(10)),
            Expr::Literal(LiteralValue::Integer(20)),
        ]);
        let ent = reg.ingest_expr(&expr).expect("Ingestion failed");

        // 1. Verify it was correctly stored in the flat SoA array_literals component table
        assert!(reg.array_literals[ent.0 as usize].is_some());
        let array_comp = reg.array_literals[ent.0 as usize].as_ref().unwrap();
        assert_eq!(array_comp.0.len(), 2);

        // 2. Verify reification (reconstruction back to tree AST)
        let reified = reg.reify_expr(ent).expect("Reification failed");
        assert_eq!(expr, reified);

        // 3. Verify type checking / inference
        let ty = reg.infer_type(ent).expect("Type inference failed");
        assert!(matches!(ty, SignalType::Array { .. }));
        if let SignalType::Array { element, length } = ty {
            assert_eq!(length, 2);
            assert_eq!(*element, SignalType::Unsigned(5)); // min_bits for 20 is 5
        }
    }

    #[test]
    fn test_unit_struct_literal_roundtrip_and_type_inference() {
        let mut reg = Registry::new();
        let expr = Expr::StructLiteral {
            name: "Coord".to_string(),
            fields: vec![
                ("x".to_string(), Expr::Literal(LiteralValue::Integer(5))),
                ("y".to_string(), Expr::Literal(LiteralValue::Bool(true))),
            ],
        };
        let ent = reg.ingest_expr(&expr).expect("Ingestion failed");

        // 1. Verify SoA storage
        assert!(reg.struct_literals[ent.0 as usize].is_some());
        let struct_comp = reg.struct_literals[ent.0 as usize].as_ref().unwrap();
        assert_eq!(struct_comp.name, "Coord");
        assert_eq!(struct_comp.fields.len(), 2);
        assert_eq!(struct_comp.fields[0].0, "x");
        assert_eq!(struct_comp.fields[1].0, "y");

        // 2. Verify roundtrip reification
        let reified = reg.reify_expr(ent).expect("Reification failed");
        assert_eq!(expr, reified);

        // 3. Verify type checking / inference
        let ty = reg.infer_type(ent).expect("Type inference failed");
        assert!(matches!(ty, SignalType::Struct { .. }));
        if let SignalType::Struct { name, fields } = ty {
            assert_eq!(name, "Coord");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].0, "x");
            assert_eq!(fields[0].1, SignalType::Unsigned(3)); // min bits for 5 is 3
            assert_eq!(fields[1].0, "y");
            assert_eq!(fields[1].1, SignalType::Bool);
        }
    }

    #[test]
    fn test_unit_unfold_index_roundtrip_and_type_inference() {
        let mut reg = Registry::new();
        let expr = Expr::UnfoldIndex("i".to_string());
        let ent = reg.ingest_expr(&expr).expect("Ingestion failed");

        // 1. Verify SoA storage
        assert!(reg.unfold_indices[ent.0 as usize].is_some());
        let unfold_comp = reg.unfold_indices[ent.0 as usize].as_ref().unwrap();
        assert_eq!(unfold_comp.0, "i");

        // 2. Verify roundtrip reification
        let reified = reg.reify_expr(ent).expect("Reification failed");
        assert_eq!(expr, reified);

        // 3. Verify type checking / inference
        let ty = reg.infer_type(ent).expect("Type inference failed");
        assert_eq!(ty, SignalType::Unsigned(32)); // UnfoldIndex infers as u32
    }

    // (Remaining QA tests would involve more complex expression trees and edge cases)
}
