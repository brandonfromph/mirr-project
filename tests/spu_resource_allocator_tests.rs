//! Contract: SPU Resource Allocator Validation
//! This test suite enforces that the R-SPU Resource Allocator correctly maps
//! logical ECS-native signals to physical SPU registers within R-SPU architectural bounds.

use mirrc::ast::program::SignalDecl;
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::ecs::{components::*, Registry};
use mirrc::temporal::allocator::RspuAllocator;

#[test]
fn test_allocator_maps_signals_to_unique_registers() {
    let mut registry = Registry::new();
    let mod_entity = registry.create_entity("test_module", KindComponent::MODULE);

    // Register two signals

    let sig1 = SignalDecl {
        name: "s1".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Unsigned(8)),
        origin: None,
        span: None,
    };
    let sig2 = SignalDecl {
        name: "s2".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Unsigned(8)),
        origin: None,
        span: None,
    };

    let ent1 = registry.create_signal(sig1.name.clone(), KindComponent(EntityKind::SIGNAL(sig1.kind)), TypeComponent(sig1.ty.clone()));
    let ent2 = registry.create_signal(sig2.name.clone(), KindComponent(EntityKind::SIGNAL(sig2.kind)), TypeComponent(sig2.ty.clone()));

    // Perform Allocation
    let mut allocator = RspuAllocator::new();
    let reg1 = allocator.allocate(&registry, ent1).expect("Allocation s1 failed");
    let reg2 = allocator.allocate(&registry, ent2).expect("Allocation s2 failed");

    assert_ne!(reg1, reg2, "Physical registers must be unique");
    assert!(reg1 < 256, "Registers must be within RS-16 bounds (0-255)");
}

#[test]
fn test_allocator_enforces_register_limit() {
    let mut registry = Registry::new();
    let mod_entity = registry.create_entity("test_module", KindComponent::MODULE);

    let mut allocator = RspuAllocator::new();
    // Fill 256 registers
    for i in 0..256 {
        let sig = SignalDecl {
            name: format!("s{i}"),
            kind: SignalKind::Internal,
            ty: ExtendedType::from_core(SignalType::Unsigned(8)),
            origin: None,
            span: None,
        };
        let ent = registry.create_signal(sig.name.clone(), KindComponent(EntityKind::SIGNAL(sig.kind)), TypeComponent(sig.ty.clone()));
        allocator.allocate(&registry, ent).expect("Allocation failed");
    }

    // 257th signal should fail (E706 constraint)
    let sig = SignalDecl {
        name: "extra".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Unsigned(8)),
        origin: None,
        span: None,
    };
    let ent = registry.create_signal(sig.name.clone(), KindComponent(EntityKind::SIGNAL(sig.kind)), TypeComponent(sig.ty.clone()));

    assert!(
        allocator.allocate(&registry, ent).is_err(),
        "Allocation should fail due to register limit"
    );
}
