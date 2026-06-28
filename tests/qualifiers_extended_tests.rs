#![forbid(unsafe_code)]

use mirrc::ast::types::{EffectQualifier, Linearity, Refinement, SignalType, TypeAnnotations};
use mirrc::ecs::components::{EntityKind, KindComponent};
use mirrc::ecs::NameComponent;
use mirrc::ecs::Registry;
use mirrc::typeck::extended::{
    ClockDomain, ExtendedSignalDecl, SessionRole, SessionTypeRef, TypeNat, TypeQualifier,
};

#[test]
fn type_qualifier_formatting() {
    assert_eq!(TypeQualifier::Pure.to_string(), "pure");
    assert_eq!(TypeQualifier::Stateful.to_string(), "stateful");
}

#[test]
fn clock_domain_formatting() {
    let cd = ClockDomain::new("clk_fast").with_frequency(100_000_000);
    assert_eq!(cd.to_string(), "@clk_fast(100000000Hz)");
}

#[test]
fn type_nat_formatting() {
    let tn = TypeNat::new(4).unwrap();
    assert_eq!(tn.to_string(), "4");
}

#[test]
fn session_type_ref_formatting() {
    let st_ref = SessionTypeRef {
        protocol: "P".to_string(),
        state: "S".to_string(),
        role: SessionRole::Sender,
    };
    assert_eq!(st_ref.to_string(), "session P::S (sender)");
}

#[test]
fn extended_signal_decl_mapping() {
    let ext_ty = mirrc::ast::types::ExtendedType::new(
        SignalType::Bool,
        TypeAnnotations {
            linearity: Linearity::Linear,
            effect: EffectQualifier::Stateful,
            refinement: Some(Refinement::Range { lo: 0, hi: 1 }),
            ..Default::default()
        },
    );
    let sig_decl = mirrc::ast::program::SignalDecl {
        name: "test".to_string(),
        kind: mirrc::ast::types::SignalKind::Internal,
        ty: ext_ty.clone(),
        origin: None,
        span: None,
    };
    let ext_sig = ExtendedSignalDecl::from_ast(&sig_decl);
    assert!(ext_sig.extended_ty.qualifiers.contains(&TypeQualifier::Linear));
    assert!(ext_sig.extended_ty.qualifiers.contains(&TypeQualifier::Stateful));
    assert!(!ext_sig.extended_ty.refinements.is_empty());
}

#[test]
fn extended_signal_decl_mapping_pure() {
    let ext_ty2 = mirrc::ast::types::ExtendedType::new(
        SignalType::Bool,
        TypeAnnotations {
            linearity: Linearity::Unrestricted,
            effect: EffectQualifier::Pure,
            refinement: Some(Refinement::Predicate("x > 0".to_string())),
            ..Default::default()
        },
    );
    let sig_decl2 = mirrc::ast::program::SignalDecl {
        name: "test2".to_string(),
        kind: mirrc::ast::types::SignalKind::Internal,
        ty: ext_ty2,
        origin: None,
        span: None,
    };
    let ext_sig2 = ExtendedSignalDecl::from_ast(&sig_decl2);
    assert!(ext_sig2.extended_ty.qualifiers.contains(&TypeQualifier::Pure));
    assert!(!ext_sig2.extended_ty.refinements.is_empty());
}

#[test]
fn extended_signal_decl_from_ecs_fails_on_module() {
    let mut reg = Registry::new();
    let kind_misc = KindComponent(EntityKind::MODULE);
    let id1 = reg.create_entity("test_module", kind_misc);
    reg.names[id1.0 as usize] = Some(NameComponent(mirrc::ecs::intern::InternId(0)));

    assert!(ExtendedSignalDecl::from_ecs(&reg, id1).is_none());
}

#[test]
fn extended_signal_decl_default_type() {
    let raw = r#"{
        "name": "default",
        "kind": "Internal",
        "ty": "Bool"
    }"#;
    let de_sig: ExtendedSignalDecl = serde_json::from_str(raw).unwrap();
    assert_eq!(de_sig.ty, SignalType::Bool);
}
