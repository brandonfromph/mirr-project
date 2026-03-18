#![forbid(unsafe_code)]
//! Deep extended type-check tests for phantom type tags.

use nasa_rust_project::ast::program::Module;
use nasa_rust_project::ast::types::{ExtendedType as AstExtendedType, SignalKind, SignalType};
use nasa_rust_project::ast::SignalDecl;
use nasa_rust_project::typeck::extended::{
    typecheck_extended, ExtendedSignalDecl, ExtendedType as TypeckExtendedType, PhantomTag,
};

fn plain(name: &str, kind: SignalKind, ty: SignalType) -> ExtendedSignalDecl {
    ExtendedSignalDecl {
        name: name.to_string(),
        kind,
        ty: ty.clone(),
        extended_ty: TypeckExtendedType::from_base(ty),
        origin: None,
        span: None,
    }
}

fn with_phantom(name: &str, kind: SignalKind, ty: SignalType, tag: &str) -> ExtendedSignalDecl {
    ExtendedSignalDecl {
        name: name.to_string(),
        kind,
        ty: ty.clone(),
        extended_ty: TypeckExtendedType {
            base: ty,
            refinements: Vec::new(),
            qualifiers: Vec::new(),
            clock_domain: None,
            phantom: Some(PhantomTag { tag: tag.to_string() }),
            type_nat: None,
            dependent_params: Vec::new(),
            session: None,
            span: None,
        },
        origin: None,
        span: None,
    }
}

fn module_from_exts(exts: &[ExtendedSignalDecl]) -> Module {
    Module {
        name: "typeck_test".to_string(),
        signals: exts
            .iter()
            .map(|e| SignalDecl {
                name: e.name.clone(),
                kind: e.kind,
                ty: AstExtendedType::from_core(e.ty.clone()),
                origin: None,
                span: None,
            })
            .collect(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

#[test]
fn no_phantom_default() {
    let s = plain("x", SignalKind::Input, SignalType::Bool);
    assert!(s.extended_ty.phantom.is_none());
}
#[test]
fn phantom_tag_name_preserved() {
    let s = with_phantom("x", SignalKind::Input, SignalType::Bool, "SafeValue");
    assert_eq!(s.extended_ty.phantom.unwrap().tag, "SafeValue");
}
#[test]
fn phantom_accepted_in_typecheck() {
    let sigs = vec![with_phantom("p", SignalKind::Input, SignalType::Unsigned(16), "Calibrated")];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn same_phantom_two_signals_no_panic() {
    let sigs = vec![
        with_phantom("a", SignalKind::Input, SignalType::Bool, "Tag"),
        with_phantom("b", SignalKind::Output, SignalType::Bool, "Tag"),
    ];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn different_phantoms_no_panic() {
    let sigs = vec![
        with_phantom("a", SignalKind::Input, SignalType::Unsigned(8), "TypeA"),
        with_phantom("b", SignalKind::Input, SignalType::Unsigned(8), "TypeB"),
    ];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn mixed_phantom_and_plain() {
    let sigs = vec![
        with_phantom("tagged", SignalKind::Input, SignalType::Unsigned(8), "Verified"),
        plain("untagged", SignalKind::Output, SignalType::Bool),
    ];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn empty_phantom_name_accepted() {
    let sigs = vec![with_phantom("x", SignalKind::Input, SignalType::Bool, "")];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
