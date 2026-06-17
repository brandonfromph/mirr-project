/// TDD: Phase 7 — Session Type Checking (ECS-Native)
///
/// Tests for `check_session_types_ecs()` in `typeck::extended::domain_checks`.
/// Written BEFORE implementation per project TDD mandate.
///
/// Error codes under test:
///   E625 — SES-PROTOCOL: signal references undeclared protocol or invalid state.
#[cfg(test)]
mod phase7_session_ecs_tests {
    use mirrc::ast::types::{ExtendedType, SignalKind, SignalType, TypeAnnotations};
    use mirrc::ecs::components::{
        EntityId, EntityKind, KindComponent, ModuleComponent, NameComponent, TypeComponent,
    };
    use mirrc::ecs::Registry;
    use mirrc::error::PipelineErrors;
    use mirrc::typeck::extended::domain_checks::check_session_types_ecs;
    use mirrc::typeck::extended::{
        typecheck_extended_ecs_with_protocols, SessionProtocol, SessionTransition,
    };

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_session_annotations(protocol: &str, state: &str) -> TypeAnnotations {
        TypeAnnotations {
            session: Some(mirrc::ast::types::SessionTypeRef {
                protocol: protocol.to_string(),
                state: state.to_string(),
            }),
            ..Default::default()
        }
    }

    fn simple_protocol(name: &str, states: &[(&str, &str)]) -> SessionProtocol {
        SessionProtocol {
            name: name.to_string(),
            transitions: states
                .iter()
                .map(|(from, to)| SessionTransition {
                    from: from.to_string(),
                    to: to.to_string(),
                    guard: None,
                })
                .collect(),
            span: None,
        }
    }

    /// Build a minimal Registry with one module and one signal annotated with a session type.
    fn registry_with_session_signal(protocol: &str, state: &str) -> (Registry, EntityId) {
        let mut reg = Registry::new();

        // Module entity
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        // Signal entity
        let sig_id = reg.next_id();
        let sig_idx = sig_id.0 as usize;

        let annotations = make_session_annotations(protocol, state);
        let ext_ty = ExtendedType::new(SignalType::Bool, annotations);

        reg.names[sig_idx] = Some(NameComponent("req".to_string()));
        reg.kinds[sig_idx] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Output)));
        reg.types[sig_idx] = Some(TypeComponent(ext_ty));
        reg.modules[sig_idx] = Some(ModuleComponent(mod_id));

        (reg, mod_id)
    }

    // -----------------------------------------------------------------------
    // T1: Signal with valid protocol + valid state → no errors.
    // -----------------------------------------------------------------------
    #[test]
    fn test_session_valid_protocol_and_state() {
        let (reg, mod_id) = registry_with_session_signal("Handshake", "Idle");
        let protocols = vec![simple_protocol(
            "Handshake",
            &[("Idle", "Ready"), ("Ready", "Ack"), ("Ack", "Idle")],
        )];

        let mut errors = PipelineErrors::new();
        check_session_types_ecs(&reg, mod_id, &protocols, &mut errors);

        assert!(
            errors.is_empty(),
            "Expected no errors for valid protocol/state; got: {:?}",
            errors
        );
    }

    // -----------------------------------------------------------------------
    // T2: Signal references an undeclared protocol → E625.
    // -----------------------------------------------------------------------
    #[test]
    fn test_session_undeclared_protocol_emits_e625() {
        let (reg, mod_id) = registry_with_session_signal("MissingProtocol", "Idle");
        let protocols: Vec<SessionProtocol> = vec![]; // none declared

        let mut errors = PipelineErrors::new();
        check_session_types_ecs(&reg, mod_id, &protocols, &mut errors);

        assert!(!errors.is_empty(), "Expected E625 for undeclared protocol");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E625"), "Error should mention E625; got: {}", msg);
        assert!(
            msg.contains("MissingProtocol"),
            "Error should mention protocol name; got: {}",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // T3: Signal references a declared protocol but an invalid state → E625.
    // -----------------------------------------------------------------------
    #[test]
    fn test_session_invalid_state_in_declared_protocol_emits_e625() {
        let (reg, mod_id) = registry_with_session_signal("Handshake", "BogusState");
        let protocols = vec![simple_protocol("Handshake", &[("Idle", "Ready"), ("Ready", "Ack")])];

        let mut errors = PipelineErrors::new();
        check_session_types_ecs(&reg, mod_id, &protocols, &mut errors);

        assert!(!errors.is_empty(), "Expected E625 for invalid state");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E625"), "Error should mention E625; got: {}", msg);
        assert!(msg.contains("BogusState"), "Error should mention state name; got: {}", msg);
    }

    // -----------------------------------------------------------------------
    // T4: No session-annotated signals → no errors (fast-path).
    // -----------------------------------------------------------------------
    #[test]
    fn test_session_no_annotated_signals_is_noop() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        // Signal with NO session annotation
        let sig_id = reg.next_id();
        let sig_idx = sig_id.0 as usize;
        reg.names[sig_idx] = Some(NameComponent("clk".to_string()));
        reg.kinds[sig_idx] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
        reg.types[sig_idx] =
            Some(TypeComponent(ExtendedType::new(SignalType::Bool, Default::default())));
        reg.modules[sig_idx] = Some(ModuleComponent(mod_id));

        let protocols = vec![simple_protocol("SomeProtocol", &[("A", "B")])];
        let mut errors = PipelineErrors::new();
        check_session_types_ecs(&reg, mod_id, &protocols, &mut errors);

        assert!(errors.is_empty(), "Expected no errors when no signals have session types");
    }

    // -----------------------------------------------------------------------
    // T5: Multiple signals — valid + invalid mix → exactly one error.
    // -----------------------------------------------------------------------
    #[test]
    fn test_session_mixed_valid_invalid_signals() {
        let mut reg = Registry::new();
        let mod_id = reg.create_entity("top", KindComponent::MODULE);

        let protocols = vec![simple_protocol("Handshake", &[("Idle", "Ready"), ("Ready", "Ack")])];

        // Valid signal
        let s1 = reg.next_id();
        let idx1 = s1.0 as usize;
        reg.names[idx1] = Some(NameComponent("req".to_string()));
        reg.kinds[idx1] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Output)));
        reg.types[idx1] = Some(TypeComponent(ExtendedType::new(
            SignalType::Bool,
            make_session_annotations("Handshake", "Idle"),
        )));
        reg.modules[idx1] = Some(ModuleComponent(mod_id));

        // Invalid signal (bad state)
        let s2 = reg.next_id();
        let idx2 = s2.0 as usize;
        reg.names[idx2] = Some(NameComponent("ack".to_string()));
        reg.kinds[idx2] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
        reg.types[idx2] = Some(TypeComponent(ExtendedType::new(
            SignalType::Bool,
            make_session_annotations("Handshake", "INVALID_STATE"),
        )));
        reg.modules[idx2] = Some(ModuleComponent(mod_id));

        let mut errors = PipelineErrors::new();
        check_session_types_ecs(&reg, mod_id, &protocols, &mut errors);

        assert_eq!(errors.len(), 1, "Expected exactly one error for the invalid signal");
        let msg = format!("{:?}", errors);
        assert!(msg.contains("E625"));
        assert!(msg.contains("INVALID_STATE"));
    }

    // -----------------------------------------------------------------------
    // T6: Signal in a different module is NOT checked when targeting mod_a.
    // -----------------------------------------------------------------------
    #[test]
    fn test_session_only_checks_signals_in_target_module() {
        let mut reg = Registry::new();

        // Module A (target)
        let mod_a = reg.create_entity("modA", KindComponent::MODULE);
        // Module B (different)
        let mod_b = reg.create_entity("modB", KindComponent::MODULE);

        let protocols: Vec<SessionProtocol> = vec![]; // no protocols declared at all

        // Signal in modB with an invalid protocol — should NOT be flagged when checking mod_a
        let s = reg.next_id();
        let idx = s.0 as usize;
        reg.names[idx] = Some(NameComponent("out".to_string()));
        reg.kinds[idx] = Some(KindComponent(EntityKind::SIGNAL(SignalKind::Output)));
        reg.types[idx] = Some(TypeComponent(ExtendedType::new(
            SignalType::Bool,
            make_session_annotations("MissingProtocol", "Idle"),
        )));
        reg.modules[idx] = Some(ModuleComponent(mod_b)); // belongs to modB, NOT modA

        let mut errors = PipelineErrors::new();
        check_session_types_ecs(&reg, mod_a, &protocols, &mut errors);

        assert!(
            errors.is_empty(),
            "Signals in other modules must not generate errors when checking modA"
        );
    }

    // -----------------------------------------------------------------------
    // T7: State appearing only as a `to` target is still valid.
    // -----------------------------------------------------------------------
    #[test]
    fn test_session_state_in_to_field_is_valid() {
        // "Ack" only appears as a `to` target in transitions, not as a `from`.
        // The check must accept it as a reachable state.
        let (reg, mod_id) = registry_with_session_signal("Handshake", "Ack");
        let protocols = vec![simple_protocol("Handshake", &[("Idle", "Ready"), ("Ready", "Ack")])];

        let mut errors = PipelineErrors::new();
        check_session_types_ecs(&reg, mod_id, &protocols, &mut errors);

        assert!(errors.is_empty(), "State appearing only as 'to' is still a valid state");
    }

    // -----------------------------------------------------------------------
    // T8: typecheck_extended_ecs_with_protocols end-to-end propagates E625.
    // -----------------------------------------------------------------------
    #[test]
    fn test_typecheck_extended_ecs_with_protocols_propagates_session_error() {
        let (reg, mod_id) = registry_with_session_signal("MyProtocol", "BadState");
        let protocols = vec![simple_protocol("MyProtocol", &[("GoodState", "Done")])];

        let result = typecheck_extended_ecs_with_protocols(&reg, mod_id, &protocols);

        assert!(!result.errors.is_empty(), "Expected E625 through the full ECS typecheck path");
        let msg = format!("{:?}", result.errors);
        assert!(msg.contains("E625"), "Expected E625 in errors, got: {}", msg);
    }
}
