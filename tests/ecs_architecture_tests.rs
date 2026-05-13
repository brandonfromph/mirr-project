#![forbid(unsafe_code)]

use nasa_rust_project::ecs::*;
use nasa_rust_project::ast::types::{SignalKind, ExtendedType, SignalType};

#[test]
fn test_ecs_registry_signal_creation() {
    let mut registry = Registry::new();
    
    // TDD: Define what we want
    let sig_name = "sys_clk".to_string();
    let kind = KindComponent(SignalKind::Input);
    let ty = TypeComponent(ExtendedType::from_core(SignalType::Bool));
    
    // Act
    let entity = registry.create_signal(sig_name.clone(), kind, ty);
    
    // Assert: Everything is in the right table (SoA)
    assert_eq!(registry.names.get(&entity).unwrap().0, "sys_clk");
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
            KindComponent(SignalKind::Internal), 
            TypeComponent(ExtendedType::from_core(SignalType::Unsigned(8)))
        );
    }
    
    // Assert SoA layout: names and types are in separate contiguous tables
    assert_eq!(registry.names.len(), 1000);
    assert_eq!(registry.types.len(), 1000);
}

#[test]
fn test_ecs_full_module_ingestion() {
    use nasa_rust_project::parser::parse_mirr;
    use std::fs;
    
    // Load a real module: majority_gate
    let src = fs::read_to_string("stdlib/safety/majority.mirr").expect("failed to read majority.mirr");
    let prog = parse_mirr(&src).expect("failed to parse majority.mirr");
    
    let mut registry = Registry::new();
    let mod_id = registry.ingest_module(&prog.module);
    
    // Verify Module entity
    assert_eq!(registry.names.get(&mod_id).unwrap().0, "majority_gate");
    
    // Verify Signal entities (majority_gate has 3 in, 1 out)
    assert!(registry.get_entity_by_name("input_a").is_some());
    assert!(registry.get_entity_by_name("majority_out").is_some());
    
    // Verify Guard entities
    let a_and_b = registry.get_entity_by_name("a_and_b").expect("Guard a_and_b missing");
    let cond_ref = registry.conditions.get(&a_and_b).expect("Guard condition ref missing");
    
    // Verify flattened expression: (input_a && input_b)
    // The top node should be a BinaryComponent
    let binary = registry.binary_ops.get(&cond_ref.0).expect("Top node of a_and_b must be binary");
    assert_eq!(binary.op, nasa_rust_project::ast::types::BinaryOp::And);
    
    println!("Successfully ingested majority_gate into ECS. Module Entity: {}", mod_id.0);
    println!("Guard 'a_and_b' Entity: {}", a_and_b.0);
}
