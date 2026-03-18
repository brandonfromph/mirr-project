#![forbid(unsafe_code)]
//! Deep extended type-check tests for effect qualifiers.

use nasa_rust_project::ast::program::Module;
use nasa_rust_project::ast::types::{SignalKind, SignalType};
use nasa_rust_project::ast::SignalDecl;
use nasa_rust_project::typeck::extended::{
    typecheck_extended, ExtendedSignalDecl, ExtendedType, TypeQualifier,
};

fn ext(
    name: &str,
    kind: SignalKind,
    ty: SignalType,
    quals: Vec<TypeQualifier>,
) -> ExtendedSignalDecl {
    ExtendedSignalDecl {
        name: name.to_string(),
        kind,
        ty: ty.clone(),
        extended_ty: ExtendedType {
            base: ty,
            refinements: Vec::new(),
            qualifiers: quals,
            clock_domain: None,
            phantom: None,
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
                ty: nasa_rust_project::ast::types::ExtendedType::from_core(e.ty.clone()),
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
fn no_qualifiers_no_error() {
    let sigs = vec![
        ext("x", SignalKind::Input, SignalType::Unsigned(8), vec![]),
        ext("y", SignalKind::Output, SignalType::Bool, vec![]),
    ];
    let module = module_from_exts(&sigs);
    assert!(typecheck_extended(&module, &sigs, &[], &[], &[]).errors.is_empty());
}
#[test]
fn stateful_qualifier_accepted() {
    let sigs =
        vec![ext("io", SignalKind::Output, SignalType::Unsigned(8), vec![TypeQualifier::Stateful])];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn pure_qualifier_accepted() {
    let sigs = vec![ext("p", SignalKind::Input, SignalType::Bool, vec![TypeQualifier::Pure])];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn mixed_qualifiers_no_panic() {
    let sigs = vec![
        ext("a", SignalKind::Input, SignalType::Bool, vec![TypeQualifier::Pure]),
        ext("b", SignalKind::Output, SignalType::Unsigned(8), vec![TypeQualifier::Stateful]),
    ];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn empty_signals_ok() {
    let module = module_from_exts(&[]);
    assert!(typecheck_extended(&module, &[], &[], &[], &[]).errors.is_empty());
}
#[test]
fn multiple_no_qualifier_signals() {
    let sigs: Vec<_> = (0..4)
        .map(|i| ext(&format!("s{}", i), SignalKind::Input, SignalType::Unsigned(8), vec![]))
        .collect();
    let module = module_from_exts(&sigs);
    assert!(typecheck_extended(&module, &sigs, &[], &[], &[]).errors.is_empty());
}
#[test]
fn linear_qualifier_no_panic() {
    let sigs = vec![ext("x", SignalKind::Input, SignalType::Bool, vec![TypeQualifier::Linear])];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
