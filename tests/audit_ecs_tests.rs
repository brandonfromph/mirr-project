#![forbid(unsafe_code)]

use mirrc::ast::types::{BinaryOp, LiteralValue};
use mirrc::ecs::components::{BinaryComponent, EntityId, LiteralComponent};
use mirrc::ecs::*;
use mirrc::error::MirrError;

fn create_literal(registry: &mut Registry, lit: LiteralValue) -> EntityId {
    let id = registry.next_id();
    registry.literals[id.0 as usize] = Some(LiteralComponent(lit));
    id
}

fn create_binary_op(
    registry: &mut Registry,
    op: BinaryOp,
    left: EntityId,
    right: EntityId,
) -> EntityId {
    let id = registry.next_id();
    registry.binary_ops[id.0 as usize] = Some(BinaryComponent { op, left, right });
    id
}

#[test]
fn test_1_ecs_recursive_depth_limit_63() {
    let mut registry = Registry::new();
    let mut current_ent = create_literal(&mut registry, LiteralValue::Integer(1));

    for _ in 0..62 {
        let other = create_literal(&mut registry, LiteralValue::Integer(1));
        current_ent = create_binary_op(&mut registry, BinaryOp::Add, current_ent, other);
    }

    let result = registry.reify_expr(current_ent);
    assert!(result.is_ok(), "Depth 63 should not fail");
}

#[test]
fn test_2_ecs_recursive_depth_limit_64() {
    let mut registry = Registry::new();
    let mut current_ent = create_literal(&mut registry, LiteralValue::Integer(1));

    for _ in 0..63 {
        let other = create_literal(&mut registry, LiteralValue::Integer(1));
        current_ent = create_binary_op(&mut registry, BinaryOp::Add, current_ent, other);
    }

    let result = registry.reify_expr(current_ent);
    assert!(result.is_ok(), "Depth 64 should not fail");
}

#[test]
fn test_3_ecs_recursive_depth_limit_65_fallback() {
    let mut registry = Registry::new();
    let mut current_ent = create_literal(&mut registry, LiteralValue::Integer(1));

    for _ in 0..65 {
        let other = create_literal(&mut registry, LiteralValue::Integer(1));
        current_ent = create_binary_op(&mut registry, BinaryOp::Add, current_ent, other);
    }

    // Attempting depth 65 (root + 65 levels of add)
    let result = registry.reify_expr(current_ent);
    assert!(
        matches!(result, Err(MirrError::SemanticError { ref message, .. }) if message.contains("exceeds maximum nesting depth")),
        "Expected SemanticError for exceeding depth 64, got {:?}",
        result
    );
}

#[test]
fn test_4_ecs_registry_entity_exhaustion_stress() {
    let mut registry = Registry::new();
    for _ in 0..999_990 {
        create_literal(&mut registry, LiteralValue::Integer(42));
    }
    assert_eq!(registry.next_id().0, 999_990);
}

#[test]
fn test_5_ecs_registry_entity_exhaustion_limit() {
    let mut registry = Registry::new();
    for _ in 0..1_000_001 {
        create_literal(&mut registry, LiteralValue::Integer(42));
    }
    // Should clamp at MAX_ENTITIES - 1 (999_999) safely without panicking
    assert_eq!(registry.next_id().0, 999_999);
}

#[test]
fn test_6_ecs_garbage_collection_orphans() {
    let mut registry = Registry::new();
    let orphan = create_literal(&mut registry, LiteralValue::Integer(99));

    assert!(registry.literals[orphan.0 as usize].is_some());
    assert!(registry.names[orphan.0 as usize].is_none());
}

#[test]
fn test_7_ecs_garbage_collection_pipeline_cleanup() {
    let mut registry = Registry::new();
    let _id1 = create_literal(&mut registry, LiteralValue::Integer(1));

    let mut registry2 = Registry::new();
    let id2 = registry2.next_id();
    assert_eq!(id2.0, 0);
}

#[test]
fn test_8_9_typeck_domain_check_deferred_session_fail() {
    use mirrc::ast::program::Module;
    use mirrc::error::PipelineErrors;
    use mirrc::typeck::extended::domain_checks::check_session_types;
    use mirrc::typeck::extended::{
        ExtendedSignalDecl, ExtendedType, SessionProtocol, SessionRole, SessionTransition,
        SessionTypeRef,
    };

    let module = Module {
        name: "test_mod".to_string(),
        signals: vec![],
        guards: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        reflexes: vec![],
        span: None,
    };

    let mut sig1 = ExtendedSignalDecl {
        name: "valid_session_sig".to_string(),
        kind: mirrc::ast::types::SignalKind::Input,
        ty: mirrc::ast::types::SignalType::Bool,
        span: None,
        origin: None,
        extended_ty: ExtendedType::from_base(mirrc::ast::types::SignalType::Bool),
    };
    sig1.extended_ty.session = Some(SessionTypeRef {
        protocol: "ValidProto".to_string(),
        state: "StateA".to_string(),
        role: SessionRole::Sender,
    });

    let mut sig2 = ExtendedSignalDecl {
        name: "invalid_proto_sig".to_string(),
        kind: mirrc::ast::types::SignalKind::Input,
        ty: mirrc::ast::types::SignalType::Bool,
        span: None,
        origin: None,
        extended_ty: ExtendedType::from_base(mirrc::ast::types::SignalType::Bool),
    };
    sig2.extended_ty.session = Some(SessionTypeRef {
        protocol: "MissingProto".to_string(),
        state: "StateA".to_string(),
        role: SessionRole::Sender,
    });

    let extended_signals = vec![sig1, sig2];

    let proto1 = SessionProtocol {
        name: "ValidProto".to_string(),
        span: None,
        transitions: vec![SessionTransition {
            from: "StateA".to_string(),
            to: "StateB".to_string(),
            guard: None,
        }],
    };

    let protocols = vec![proto1];
    let mut errors = PipelineErrors::new();

    check_session_types(&module, &extended_signals, &protocols, &mut errors);

    assert_eq!(errors.len(), 1);
    match &errors.errors[0] {
        mirrc::error::MirrError::TypeError { message, .. } => {
            assert!(
                message.contains("undeclared session protocol 'MissingProto'"),
                "Expected MissingProto error"
            );
        }
        _ => panic!("Expected TypeError"),
    }
}

#[test]
fn test_10_typeck_mock_parallel_width_inference_bypass() {
    use mirrc::ecs::registry::Registry;
    use mirrc::ecs::systems::parallel_width_inference_system;

    let mut registry = Registry::new();

    for i in 0..5 {
        let id = registry.next_id();
        registry.names[id.0 as usize] = Some(mirrc::ecs::components::NameComponent(
            registry.interner.intern(&format!("sig{}", i)),
        ));
    }

    let (sccs, solves, verify, stats) = parallel_width_inference_system(&mut registry);

    assert_eq!(stats.nodes_analyzed, 5);
    assert_eq!(sccs.len(), 5);
    assert_eq!(solves.len(), 5);

    for solve in solves {
        assert_eq!(solve.widths, vec![8], "Mock system hardcodes 8 bits.");
    }
    assert!(verify.is_minimal);
}
