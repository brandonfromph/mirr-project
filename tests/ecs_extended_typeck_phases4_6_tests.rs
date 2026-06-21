/// ECS-Native Regression Suite: Phases 4, 5, and 6
///
/// Verifies that the P10-compliant linear-scan checkers for:
/// - Phase 4: Effect Qualifiers (Pure/Stateful)
/// - Phase 5: Clock Domains (CDC)
/// - Phase 6: Phantom Tags (Safety Tags)
///
/// correctly identify and report architectural violations.
#[cfg(test)]
mod phase_4_6_ecs_tests {
    use mirrc::ast::types::{
        EffectQualifier, ExtendedType, SignalKind, SignalType, TypeAnnotations,
    };
    use mirrc::ecs::components::{
        AssignmentComponent, EntityId, EntityKind, KindComponent, ModuleComponent, NameComponent,
        ReflexComponent, SignalRefComponent, TypeComponent,
    };
    use mirrc::ecs::Registry;
    use mirrc::error::PipelineErrors;
    use mirrc::typeck::extended::domain_checks::{
        check_clock_domains_ecs, check_effect_qualifiers_ecs, check_phantom_tags_ecs,
    };

    fn make_registry() -> (Registry, EntityId) {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);
        (reg, mod_id)
    }

    fn add_signal(
        reg: &mut Registry,
        mod_id: EntityId,
        name: &str,
        annotations: TypeAnnotations,
    ) -> EntityId {
        let sig_id = reg.next_id();
        let sig_idx = sig_id.0 as usize;
        let ext_ty = ExtendedType::new(SignalType::Bool, annotations);
        reg.names[sig_idx] = Some(NameComponent(reg.interner.intern(name)));
        reg.kinds[sig_idx] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Output)));
        reg.types[sig_idx] = Some(TypeComponent(ext_ty));
        reg.modules[sig_idx] = Some(ModuleComponent(mod_id));
        sig_id
    }

    fn add_reflex(
        reg: &mut Registry,
        mod_id: EntityId,
        name: &str,
        target: EntityId,
        value: EntityId,
    ) -> EntityId {
        let reflex_id = reg.next_id();
        let reflex_idx = reflex_id.0 as usize;

        let assign_id = reg.next_id();
        reg.assignment_comps[assign_id.0 as usize] =
            Some(AssignmentComponent { target, value, target_index: None });

        reg.names[reflex_idx] = Some(NameComponent(reg.interner.intern(name)));
        reg.modules[reflex_idx] = Some(ModuleComponent(mod_id));
        reg.reflex_comps[reflex_idx] =
            Some(ReflexComponent { guards: vec![], assignments: vec![assign_id], origin: None });
        reflex_id
    }

    // -----------------------------------------------------------------------
    // Phase 4: Effect Checking (E617)
    // -----------------------------------------------------------------------

    #[test]
    fn test_pure_signal_depending_on_stateful_emits_e617() {
        let (mut reg, mod_id) = make_registry();

        let pure_sig = add_signal(
            &mut reg,
            mod_id,
            "p",
            TypeAnnotations { effect: EffectQualifier::Pure, ..Default::default() },
        );

        let stateful_sig = add_signal(
            &mut reg,
            mod_id,
            "s",
            TypeAnnotations { effect: EffectQualifier::Stateful, ..Default::default() },
        );

        let s_ref = reg.next_id();
        reg.signal_refs[s_ref.0 as usize] = Some(SignalRefComponent(stateful_sig));

        add_reflex(&mut reg, mod_id, "r", pure_sig, s_ref);

        let mut errors = PipelineErrors::new();
        check_effect_qualifiers_ecs(&reg, mod_id, &mut errors);

        assert!(!errors.is_empty(), "Expected E617 error");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E617"), "Error should mention E617; got: {}", msg);
    }

    // -----------------------------------------------------------------------
    // Phase 5: Clock Domain Checking (E618)
    // -----------------------------------------------------------------------

    #[test]
    fn test_clock_domain_crossing_emits_e618() {
        let (mut reg, mod_id) = make_registry();

        let fast_sig = add_signal(
            &mut reg,
            mod_id,
            "f",
            TypeAnnotations { clock_domain: Some("fast".to_string()), ..Default::default() },
        );

        let slow_sig = add_signal(
            &mut reg,
            mod_id,
            "s",
            TypeAnnotations { clock_domain: Some("slow".to_string()), ..Default::default() },
        );

        let f_ref = reg.next_id();
        reg.signal_refs[f_ref.0 as usize] = Some(SignalRefComponent(fast_sig));

        add_reflex(&mut reg, mod_id, "r", slow_sig, f_ref);

        let mut errors = PipelineErrors::new();
        check_clock_domains_ecs(&reg, mod_id, &mut errors);

        assert!(!errors.is_empty(), "Expected E618 error");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E618"), "Error should mention E618; got: {}", msg);
    }

    // -----------------------------------------------------------------------
    // Phase 6: Phantom Tag Checking (E620)
    // -----------------------------------------------------------------------

    #[test]
    fn test_phantom_tag_mismatch_emits_e620() {
        let (mut reg, mod_id) = make_registry();

        let verified_sig = add_signal(
            &mut reg,
            mod_id,
            "v",
            TypeAnnotations { phantom_tag: Some("Verified".to_string()), ..Default::default() },
        );

        let unverified_sig = add_signal(
            &mut reg,
            mod_id,
            "u",
            TypeAnnotations { phantom_tag: Some("Unverified".to_string()), ..Default::default() },
        );

        let u_ref = reg.next_id();
        reg.signal_refs[u_ref.0 as usize] = Some(SignalRefComponent(unverified_sig));

        add_reflex(&mut reg, mod_id, "r", verified_sig, u_ref);

        let mut errors = PipelineErrors::new();
        check_phantom_tags_ecs(&reg, mod_id, &mut errors);

        assert!(!errors.is_empty(), "Expected E620 error");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E620"), "Error should mention E620; got: {}", msg);
    }
}
