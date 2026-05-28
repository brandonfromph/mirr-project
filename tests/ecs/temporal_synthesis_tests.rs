#![forbid(unsafe_code)]
#![allow(clippy::unnecessary_cast)]
#[allow(unused_imports)]
use nasa_rust_project::ast::program::Module;
#[allow(unused_imports)]
use nasa_rust_project::ast::types::*;
#[allow(unused_imports)]
use nasa_rust_project::ast::BinaryOp;
#[allow(unused_imports)]
use nasa_rust_project::ast::Expr;
#[allow(unused_imports)]
use nasa_rust_project::ast::UnaryOp;
#[allow(unused_imports)]
use nasa_rust_project::ecs::components::*;
#[allow(unused_imports)]
use nasa_rust_project::ecs::registry::Registry;
#[allow(unused_imports)]
use nasa_rust_project::ecs::systems::*;
#[allow(unused_imports)]
use nasa_rust_project::span::Span;

#[test]
fn test_temporal_1() {
    let mut reg = Registry::new();
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_2() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_1", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(4));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_3() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_2", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(20));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id)); // > 16 should be Counter
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_4() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_3", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(3 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_5() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_4", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(4 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_6() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_5", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(5 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_7() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_6", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(6 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_8() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_7", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(7 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_9() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_8", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(8 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_10() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_9", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(9 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_11() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_10", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(10 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_12() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_11", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(11 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_13() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_12", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(12 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_14() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_13", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(13 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_15() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_14", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(14 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_16() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_15", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(15 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_17() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_16", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(16 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_18() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_17", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(17 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_19() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_18", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(18 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_20() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_19", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(19 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_21() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_20", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(20 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_22() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_21", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(21 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_23() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_22", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(22 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_24() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_23", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(23 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
#[test]
fn test_temporal_25() {
    let mut reg = Registry::new();
    let guard_id = reg.create_entity("guard_24", KindComponent::GUARD);
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(24 as u64));
    let lit_id = reg.next_id();
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Bool(true)));
    reg.conditions[guard_id.0 as usize] = Some(ConditionComponent(lit_id));
    let _res = temporal_synthesis_system(&mut reg);
    assert!(_res.is_ok(), "{:?}", _res.err());
}
