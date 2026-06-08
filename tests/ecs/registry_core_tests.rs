#![forbid(unsafe_code)]
#[allow(unused_imports)]
use mirrc::ast::program::Module;
#[allow(unused_imports)]
use mirrc::ast::types::*;
#[allow(unused_imports)]
use mirrc::ast::BinaryOp;
#[allow(unused_imports)]
use mirrc::ast::Expr;
#[allow(unused_imports)]
use mirrc::ast::UnaryOp;
#[allow(unused_imports)]
use mirrc::ecs::components::*;
#[allow(unused_imports)]
use mirrc::ecs::registry::Registry;
#[allow(unused_imports)]
use mirrc::ecs::systems::*;
#[allow(unused_imports)]
use mirrc::span::Span;

#[test]
fn test_registry_core_1() {
    let reg = Registry::new();
    assert_eq!(reg.names.len(), 100_000);
}
#[test]
fn test_registry_core_2() {
    let mut reg = Registry::new();
    let id1 = reg.next_id();
    let id2 = reg.next_id();
    assert_eq!(id1.0 + 1, id2.0);
}
#[test]
fn test_registry_core_3() {
    let mut reg = Registry::new();
    let id = reg.create_entity("test_entity", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "test_entity");
}
#[test]
fn test_registry_core_4() {
    let mut reg = Registry::new();
    let id = reg.create_kb_chunk(
        "kb_1".to_string(),
        "text".to_string(),
        "src.rs".to_string(),
        (1, 10),
        None,
    );
    assert_eq!(reg.chunk_texts[id.0 as usize].as_ref().unwrap().0, "text");
}
#[test]
fn test_registry_core_5() {
    let mut reg = Registry::new();
    let id = reg.create_kb_chunk(
        "kb_v".to_string(),
        "t".to_string(),
        "s".to_string(),
        (1, 2),
        Some(vec![1.0, 2.0]),
    );
    assert_eq!(reg.vectors[id.0 as usize].as_ref().unwrap().0, vec![1.0, 2.0]);
}
#[test]
fn test_registry_core_6() {
    let mut reg = Registry::new();
    let id = reg.create_entity("test_type", KindComponent::SIGNAL);
    reg.set_type(id, TypeComponent(ExtendedType::new(SignalType::Bool, Default::default())));
    assert!(reg.types[id.0 as usize].is_some());
}
#[test]
fn test_registry_core_7() {
    let mut reg = Registry::new();
    let id1 = reg.create_entity("parent", KindComponent::MODULE);
    let id2 = reg.create_entity("child", KindComponent::SIGNAL);
    reg.set_parent(id2, id1);
    assert_eq!(reg.modules[id2.0 as usize].unwrap().0, id1);
}
#[test]
fn test_registry_core_8() {
    let mut reg = Registry::new();
    let id = reg.create_entity("sig_int", KindComponent(EntityKind::SIGNAL(SignalKind::Internal)));
    assert_eq!(reg.kinds[id.0 as usize].unwrap().0, EntityKind::SIGNAL(SignalKind::Internal));
}
#[test]
fn test_registry_core_9() {
    let mut reg = Registry::new();
    let id1 = reg.create_entity("t1", KindComponent::SIGNAL);
    let id2 = reg.get_entity_by_name("t1").unwrap();
    assert_eq!(id1, id2);
}
#[test]
fn test_registry_core_10() {
    let mut reg = Registry::new();
    let id = reg.create_signal(
        "sig1".to_string(),
        KindComponent(EntityKind::SIGNAL(SignalKind::Internal)),
        TypeComponent(ExtendedType::new(SignalType::Bool, Default::default())),
    );
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "sig1");
}
#[test]
fn test_registry_core_11() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_10", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_10");
}
#[test]
fn test_registry_core_12() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_11", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_11");
}
#[test]
fn test_registry_core_13() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_12", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_12");
}
#[test]
fn test_registry_core_14() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_13", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_13");
}
#[test]
fn test_registry_core_15() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_14", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_14");
}
#[test]
fn test_registry_core_16() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_15", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_15");
}
#[test]
fn test_registry_core_17() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_16", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_16");
}
#[test]
fn test_registry_core_18() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_17", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_17");
}
#[test]
fn test_registry_core_19() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_18", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_18");
}
#[test]
fn test_registry_core_20() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_19", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_19");
}
#[test]
fn test_registry_core_21() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_20", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_20");
}
#[test]
fn test_registry_core_22() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_21", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_21");
}
#[test]
fn test_registry_core_23() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_22", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_22");
}
#[test]
fn test_registry_core_24() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_23", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_23");
}
#[test]
fn test_registry_core_25() {
    let mut reg = Registry::new();
    let id = reg.create_entity("dummy_24", KindComponent::SIGNAL);
    assert_eq!(reg.names[id.0 as usize].as_ref().unwrap().0, "dummy_24");
}
