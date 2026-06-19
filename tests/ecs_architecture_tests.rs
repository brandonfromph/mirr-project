#![forbid(unsafe_code)]

use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::ecs::*;

#[test]
fn test_ecs_registry_signal_creation() {
    let mut registry = Registry::new();

    // TDD: Define what we want
    let sig_name = "sys_clk".to_string();
    let kind = KindComponent(EntityKind::SIGNAL(SignalKind::Input));
    let ty = TypeComponent(ExtendedType::from_core(SignalType::Bool));

    // Act
    let entity = registry.create_signal(sig_name.clone(), kind, ty);

    // Assert: Everything is in the right table (SoA)
    assert_eq!(
        registry.resolve_name(registry.names[entity.0 as usize].as_ref().unwrap().0),
        "sys_clk"
    );
    assert_eq!(registry.get_entity_by_name("sys_clk"), Some(entity));

    println!("ECS Entity {} created for signal {}", entity.0, sig_name);
}

#[test]
fn test_ecs_soa_performance_layout() {
    let mut registry = Registry::new();

    // Create 1000 signals
    for i in 0..1000 {
        let name = format!("sig_{}", i);
        registry.create_signal(
            name,
            KindComponent(EntityKind::SIGNAL(SignalKind::Internal)),
            TypeComponent(ExtendedType::from_core(SignalType::Unsigned(8))),
        );
    }

    // Assert SoA layout: names and types are in separate contiguous tables
    assert_eq!(registry.names.iter().filter(|n| n.is_some()).count(), 1000);
}

#[test]
fn test_ecs_full_module_ingestion() {
    use mirrc::parser::parse_mirr;
    use std::fs;

    // Load a real module: majority_gate
    let src =
        fs::read_to_string("stdlib/safety/majority.mirr").expect("failed to read majority.mirr");
    let prog = parse_mirr(&src).expect("failed to parse majority.mirr");

    let mut registry = Registry::new();
    let mod_id = registry.ingest_module(&prog.module).expect("failed to ingest");

    // Verify Module entity
    assert_eq!(
        registry.resolve_name(registry.names[mod_id.0 as usize].as_ref().unwrap().0),
        "majority_gate"
    );

    // Verify Signal entities (majority_gate has 3 in, 1 out)
    assert!(registry.get_entity_by_name("input_a").is_some());
    assert!(registry.get_entity_by_name("majority_out").is_some());

    // Verify Guard entities
    let a_and_b = registry.get_entity_by_name("a_and_b").expect("Guard a_and_b missing");
    let cond_ref =
        registry.conditions[a_and_b.0 as usize].as_ref().expect("Guard condition ref missing");

    // Verify flattened expression: (input_a && input_b)
    // The top node should be a BinaryComponent
    let binary = registry.binary_ops[cond_ref.0 .0 as usize]
        .as_ref()
        .expect("Top node of a_and_b must be binary");
    assert_eq!(binary.op, mirrc::ast::types::BinaryOp::And);

    println!("Successfully ingested majority_gate into ECS. Module Entity: {}", mod_id.0);
    println!("Guard 'a_and_b' Entity: {}", a_and_b.0);
}

#[test]
fn test_ecs_constant_folding_system() {
    use mirrc::ast::types::LiteralValue;
    use mirrc::ecs::systems::parallel_constant_folding_system;
    use mirrc::parser::parse_mirr;

    let src = r#"
    module test_simplify {
        guard always_true {
            when true && true
            for 1 cycles;
        }
    }
    "#;
    let prog = parse_mirr(src).expect("failed to parse");

    let mut registry = Registry::new();
    registry.ingest_module(&prog.module).expect("failed to ingest");

    // Find the guard entity
    let guard_ent = registry.get_entity_by_name("always_true").unwrap();
    let cond_ent = registry.conditions[guard_ent.0 as usize].as_ref().unwrap().0;

    // Before system: it's a binary op
    assert!(registry.binary_ops[cond_ent.0 as usize].is_some());

    // Run the parallel system!
    parallel_constant_folding_system(&mut registry);

    // After system: it's a literal true
    assert!(registry.binary_ops[cond_ent.0 as usize].is_none());
    let lit = registry.literals[cond_ent.0 as usize].as_ref().expect("Should be a literal now");
    assert_eq!(lit.0, LiteralValue::Bool(true));

    println!("Parallel ECS Constant Folding System verified for entity {}", cond_ent.0);
}

#[test]
fn test_ecs_parallel_scaling() {
    use mirrc::ast::types::{BinaryOp, LiteralValue};
    use mirrc::ecs::systems::parallel_constant_folding_system;

    let mut registry = Registry::new();

    // Stress test: Create 10,000 binary operations to fold
    // This demonstrates how ECS scales across cores
    for _ in 0..10000 {
        let l = registry.create_signal(
            "".to_string(),
            KindComponent(EntityKind::SIGNAL(SignalKind::Internal)),
            TypeComponent(ExtendedType::from_core(SignalType::Bool)),
        );
        let r = registry.create_signal(
            "".to_string(),
            KindComponent(EntityKind::SIGNAL(SignalKind::Internal)),
            TypeComponent(ExtendedType::from_core(SignalType::Bool)),
        );
        registry.literals[l.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
        registry.literals[r.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));

        let op_id = registry.create_signal(
            "".to_string(),
            KindComponent(EntityKind::SIGNAL(SignalKind::Internal)),
            TypeComponent(ExtendedType::from_core(SignalType::Bool)),
        );
        registry.binary_ops[op_id.0 as usize] =
            Some(BinaryComponent { op: BinaryOp::And, left: l, right: r });
    }

    let start = std::time::Instant::now();
    parallel_constant_folding_system(&mut registry);
    let duration = start.elapsed();

    println!("Parallel system processed 10,000 entities in {:?}", duration);
}

#[test]
fn test_ecs_parallel_vector_search() {
    use mirrc::ecs::systems::parallel_vector_search_system;

    let mut registry = Registry::new();

    // Create 3 chunks with different vectors
    registry.create_kb_chunk(
        "chunk_1".to_string(),
        "text_1".to_string(),
        "source".to_string(),
        (1, 1),
        Some(vec![1.0, 0.0, 0.0]),
    );
    registry.create_kb_chunk(
        "chunk_2".to_string(),
        "text_2".to_string(),
        "source".to_string(),
        (1, 1),
        Some(vec![0.0, 1.0, 0.0]),
    );
    registry.create_kb_chunk(
        "chunk_3".to_string(),
        "text_3".to_string(),
        "source".to_string(),
        (1, 1),
        Some(vec![0.0, 0.0, 1.0]),
    );

    // Query for something close to chunk_2
    let query = vec![0.1, 0.9, 0.1];
    let hits = parallel_vector_search_system(&registry, &query, 1);

    assert_eq!(hits.len(), 1);
    let top_entity = hits[0].0;
    assert_eq!(
        registry.resolve_name(registry.names[top_entity.0 as usize].as_ref().unwrap().0),
        "chunk_2"
    );

    println!(
        "Parallel Vector Search found top match: {}",
        registry.resolve_name(registry.names[top_entity.0 as usize].as_ref().unwrap().0)
    );
}

#[test]
fn test_ecs_parallel_width_inference() {
    use mirrc::ecs::systems::parallel_width_inference_system;

    let mut registry = Registry::new();

    // Ingest some signals
    registry.create_signal(
        "sig_a".to_string(),
        KindComponent(EntityKind::SIGNAL(SignalKind::Internal)),
        TypeComponent(ExtendedType::from_core(SignalType::Bool)),
    );
    registry.create_signal(
        "sig_b".to_string(),
        KindComponent(EntityKind::SIGNAL(SignalKind::Internal)),
        TypeComponent(ExtendedType::from_core(SignalType::Unsigned(8))),
    );

    // Run the width inference system
    let (_, _, _, stats) = parallel_width_inference_system(&mut registry);

    assert_eq!(stats.nodes_analyzed, 2);
    assert_eq!(stats.scc_count, 2);

    println!("Parallel ECS Width Inference System verified with {} signals.", stats.nodes_analyzed);
}
