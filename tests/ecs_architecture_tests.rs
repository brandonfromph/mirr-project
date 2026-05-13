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
