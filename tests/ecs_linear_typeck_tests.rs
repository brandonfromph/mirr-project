/// TDD: ECS Linear Type Checking — `check_linear_signals_ecs`
///
/// Covers:
///   E613 — LIN-UNUSED: linear signal declared but never consumed.
///   E614 — LIN-DOUBLE: linear signal consumed more than once in a reflex.
///
/// Tests written BEFORE implementation per project TDD mandate.
/// All registry manipulation uses the public Registry API + direct SoA writes
/// that mirror the patterns in `tests/ecs_extended_typeck_phase7_tests.rs`.
#[cfg(test)]
mod ecs_linear_typeck_tests {
    use mirrc::ast::types::{ExtendedType, Linearity, SignalKind, SignalType, TypeAnnotations};
    use mirrc::ecs::components::{
        AssignmentComponent, EntityId, EntityKind, KindComponent, ModuleComponent, NameComponent,
        ReflexComponent, SignalRefComponent, TypeComponent,
    };
    use mirrc::ecs::Registry;
    use mirrc::error::PipelineErrors;
    use mirrc::typeck::extended::check_linear_signals_ecs;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn linear_annotations() -> TypeAnnotations {
        TypeAnnotations { linearity: Linearity::Linear, ..Default::default() }
    }

    fn nonlinear_annotations() -> TypeAnnotations {
        TypeAnnotations { linearity: Linearity::Unrestricted, ..Default::default() }
    }

    /// Add a signal entity to the registry and return its EntityId.
    fn add_signal(
        reg: &mut Registry,
        mod_id: EntityId,
        name: &str,
        annotations: TypeAnnotations,
    ) -> EntityId {
        let id = reg.next_id();
        let idx = id.0 as usize;
        reg.names[idx] = Some(NameComponent(reg.interner.intern(name)));
        reg.kinds[idx] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Internal)));
        reg.types[idx] = Some(TypeComponent(ExtendedType::new(SignalType::Bool, annotations)));
        reg.modules[idx] = Some(ModuleComponent(mod_id));
        id
    }

    /// Add a SignalRef expression entity pointing at `target_signal`.
    /// Returns the expression EntityId.
    fn add_signal_ref_expr(reg: &mut Registry, target_signal: EntityId) -> EntityId {
        let id = reg.next_id();
        let idx = id.0 as usize;
        reg.signal_refs[idx] = Some(SignalRefComponent(target_signal));
        id
    }

    /// Add an assignment entity: `lhs_signal = rhs_expr`.
    fn add_assignment(reg: &mut Registry, lhs_signal: EntityId, rhs_expr: EntityId) -> EntityId {
        let id = reg.next_id();
        let idx = id.0 as usize;
        reg.assignment_comps[idx] =
            Some(AssignmentComponent { target: lhs_signal, value: rhs_expr });
        id
    }

    /// Add a reflex entity with a given list of assignment entity IDs.
    fn add_reflex(
        reg: &mut Registry,
        mod_id: EntityId,
        name: &str,
        assignments: Vec<EntityId>,
    ) -> EntityId {
        let id = reg.next_id();
        let idx = id.0 as usize;
        reg.names[idx] = Some(NameComponent(reg.interner.intern(name)));
        reg.kinds[idx] = Some(KindComponent(EntityKind::REFLEX));
        reg.modules[idx] = Some(ModuleComponent(mod_id));
        reg.reflex_comps[idx] = Some(ReflexComponent { guards: vec![], assignments, origin: None });
        id
    }

    // -----------------------------------------------------------------------
    // T1: No linear signals → no errors (fast-path).
    // -----------------------------------------------------------------------
    #[test]
    fn test_linear_no_linear_signals_is_noop() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        add_signal(&mut reg, mod_id, "clk", nonlinear_annotations());
        add_signal(&mut reg, mod_id, "rst", nonlinear_annotations());

        let mut errors = PipelineErrors::new();
        check_linear_signals_ecs(&reg, mod_id, &mut errors);

        assert!(errors.is_empty(), "No linear signals → no errors");
    }

    // -----------------------------------------------------------------------
    // T2: Linear signal consumed exactly once → no errors.
    // -----------------------------------------------------------------------
    #[test]
    fn test_linear_single_consumption_is_valid() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        let linear_sig = add_signal(&mut reg, mod_id, "token", linear_annotations());
        let out_sig = add_signal(&mut reg, mod_id, "out", nonlinear_annotations());

        // out = token  (reads `token` once)
        let expr = add_signal_ref_expr(&mut reg, linear_sig);
        let assign = add_assignment(&mut reg, out_sig, expr);
        add_reflex(&mut reg, mod_id, "r1", vec![assign]);

        let mut errors = PipelineErrors::new();
        check_linear_signals_ecs(&reg, mod_id, &mut errors);

        assert!(errors.is_empty(), "Exactly-once consumption must produce no errors");
    }

    // -----------------------------------------------------------------------
    // T3: Linear signal consumed twice in same reflex → E614.
    // -----------------------------------------------------------------------
    #[test]
    fn test_linear_double_consumption_emits_e614() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        let linear_sig = add_signal(&mut reg, mod_id, "token", linear_annotations());
        let out1 = add_signal(&mut reg, mod_id, "out1", nonlinear_annotations());
        let out2 = add_signal(&mut reg, mod_id, "out2", nonlinear_annotations());

        // Two assignments, both read `token`
        let e1 = add_signal_ref_expr(&mut reg, linear_sig);
        let e2 = add_signal_ref_expr(&mut reg, linear_sig);
        let a1 = add_assignment(&mut reg, out1, e1);
        let a2 = add_assignment(&mut reg, out2, e2);
        add_reflex(&mut reg, mod_id, "r_double", vec![a1, a2]);

        let mut errors = PipelineErrors::new();
        check_linear_signals_ecs(&reg, mod_id, &mut errors);

        assert!(!errors.is_empty(), "Double consumption must emit E614");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E614"), "Error must mention E614; got: {}", msg);
        assert!(msg.contains("token"), "Error must name the signal; got: {}", msg);
    }

    // -----------------------------------------------------------------------
    // T4: Linear signal never consumed → E613.
    // -----------------------------------------------------------------------
    #[test]
    fn test_linear_never_consumed_emits_e613() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        add_signal(&mut reg, mod_id, "ghost", linear_annotations());
        let out = add_signal(&mut reg, mod_id, "out", nonlinear_annotations());

        // A reflex that reads `out` itself (not `ghost`)
        let expr = add_signal_ref_expr(&mut reg, out);
        let assign = add_assignment(&mut reg, out, expr);
        add_reflex(&mut reg, mod_id, "r_empty", vec![assign]);

        let mut errors = PipelineErrors::new();
        check_linear_signals_ecs(&reg, mod_id, &mut errors);

        assert!(!errors.is_empty(), "Unconsumed linear signal must emit E613");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E613"), "Error must mention E613; got: {}", msg);
        assert!(msg.contains("ghost"), "Error must name the signal; got: {}", msg);
    }

    // -----------------------------------------------------------------------
    // T5: Double in one reflex, single in another → E614 (from bad reflex).
    // -----------------------------------------------------------------------
    #[test]
    fn test_linear_double_in_one_reflex_valid_in_other() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        let linear_sig = add_signal(&mut reg, mod_id, "tok", linear_annotations());
        let out1 = add_signal(&mut reg, mod_id, "o1", nonlinear_annotations());
        let out2 = add_signal(&mut reg, mod_id, "o2", nonlinear_annotations());
        let out3 = add_signal(&mut reg, mod_id, "o3", nonlinear_annotations());

        // Reflex 1: reads tok twice → E614
        let e1 = add_signal_ref_expr(&mut reg, linear_sig);
        let e2 = add_signal_ref_expr(&mut reg, linear_sig);
        let a1 = add_assignment(&mut reg, out1, e1);
        let a2 = add_assignment(&mut reg, out2, e2);
        add_reflex(&mut reg, mod_id, "r_bad", vec![a1, a2]);

        // Reflex 2: reads tok once → valid
        let e3 = add_signal_ref_expr(&mut reg, linear_sig);
        let a3 = add_assignment(&mut reg, out3, e3);
        add_reflex(&mut reg, mod_id, "r_good", vec![a3]);

        let mut errors = PipelineErrors::new();
        check_linear_signals_ecs(&reg, mod_id, &mut errors);

        let msg = format!("{:?}", errors);
        assert!(msg.contains("E614"), "Expected E614 from r_bad; got: {}", msg);
    }

    // -----------------------------------------------------------------------
    // T6: Signal in a different module is not checked.
    // -----------------------------------------------------------------------
    #[test]
    fn test_linear_only_checks_target_module() {
        let mut reg = Registry::new();
        let mod_a = reg.create_entity("modA", KindComponent::MODULE);
        let mod_b = reg.create_entity("modB", KindComponent::MODULE);

        // Linear signal in modB — must not be flagged when checking modA
        add_signal(&mut reg, mod_b, "foreign_token", linear_annotations());

        let mut errors = PipelineErrors::new();
        check_linear_signals_ecs(&reg, mod_a, &mut errors);

        assert!(errors.is_empty(), "Linear signals in other modules must not trigger errors");
    }

    // -----------------------------------------------------------------------
    // T7: Two linear signals — only the double-consumed one errors.
    // -----------------------------------------------------------------------
    #[test]
    fn test_linear_multiple_signals_isolated_errors() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        let tok1 = add_signal(&mut reg, mod_id, "tok1", linear_annotations());
        let tok2 = add_signal(&mut reg, mod_id, "tok2", linear_annotations());
        let out1 = add_signal(&mut reg, mod_id, "o1", nonlinear_annotations());
        let out2 = add_signal(&mut reg, mod_id, "o2", nonlinear_annotations());
        let out3 = add_signal(&mut reg, mod_id, "o3", nonlinear_annotations());

        // tok1: consumed once (valid)
        let e1 = add_signal_ref_expr(&mut reg, tok1);
        let a1 = add_assignment(&mut reg, out1, e1);

        // tok2: consumed twice (E614)
        let e2 = add_signal_ref_expr(&mut reg, tok2);
        let e3 = add_signal_ref_expr(&mut reg, tok2);
        let a2 = add_assignment(&mut reg, out2, e2);
        let a3 = add_assignment(&mut reg, out3, e3);

        add_reflex(&mut reg, mod_id, "r1", vec![a1, a2, a3]);

        let mut errors = PipelineErrors::new();
        check_linear_signals_ecs(&reg, mod_id, &mut errors);

        assert_eq!(errors.len(), 1, "Expected exactly 1 error; got: {:?}", errors);
        let msg = format!("{:?}", errors);
        assert!(msg.contains("tok2"), "Error must name tok2; got: {}", msg);
        assert!(!msg.contains("tok1"), "tok1 must not appear in errors");
    }

    // -----------------------------------------------------------------------
    // T8: No reflexes → linear signal is never consumed → E613.
    // -----------------------------------------------------------------------
    #[test]
    fn test_linear_no_reflexes_emits_e613() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        add_signal(&mut reg, mod_id, "phantom_tok", linear_annotations());
        // No reflexes added at all

        let mut errors = PipelineErrors::new();
        check_linear_signals_ecs(&reg, mod_id, &mut errors);

        assert!(!errors.is_empty(), "Linear signal with no reflexes must emit E613");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E613"), "Expected E613; got: {}", msg);
    }
}
