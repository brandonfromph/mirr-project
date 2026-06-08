#![forbid(unsafe_code)]
//! Deep extended type-check tests for type refinements.

use mirrc::ast::program::Module;
use mirrc::ast::types::{ExtendedType as AstExtendedType, SignalKind, SignalType};
use mirrc::ast::SignalDecl;
use mirrc::typeck::extended::ExtendedType as CET;
use mirrc::typeck::extended::{
    typecheck_extended, ExtendedSignalDecl, RefinementBound, RefinementPredicate,
};

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

fn refined(
    name: &str,
    kind: SignalKind,
    ty: SignalType,
    preds: Vec<RefinementPredicate>,
) -> ExtendedSignalDecl {
    ExtendedSignalDecl {
        name: name.to_string(),
        kind,
        ty: ty.clone(),
        extended_ty: CET {
            base: ty,
            refinements: preds,
            qualifiers: Vec::new(),
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

fn plain(name: &str, kind: SignalKind, ty: SignalType) -> ExtendedSignalDecl {
    refined(name, kind, ty, vec![])
}

#[test]
fn no_refinements_no_error() {
    let sigs = vec![plain("x", SignalKind::Input, SignalType::Unsigned(8))];
    let module = module_from_exts(&sigs);
    assert!(typecheck_extended(&module, &sigs, &[], &[], &[]).errors.is_empty());
}
#[test]
fn range_refinement_constructed() {
    let pred =
        RefinementPredicate { bound: RefinementBound::ValueInRange { lo: 0, hi: 255 }, span: None };
    let sigs = vec![refined("x", SignalKind::Input, SignalType::Unsigned(8), vec![pred])];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn max_refinement_constructed() {
    let pred = RefinementPredicate { bound: RefinementBound::ValueLe(100), span: None };
    let sigs = vec![refined("x", SignalKind::Input, SignalType::Unsigned(16), vec![pred])];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn min_refinement_constructed() {
    let pred = RefinementPredicate { bound: RefinementBound::ValueGe(10), span: None };
    let sigs = vec![refined("x", SignalKind::Input, SignalType::Unsigned(16), vec![pred])];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn nonzero_refinement() {
    let pred = RefinementPredicate { bound: RefinementBound::ValueNe(0), span: None };
    let sigs = vec![refined("x", SignalKind::Input, SignalType::Unsigned(8), vec![pred])];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn multiple_refinements_combined() {
    let preds = vec![
        RefinementPredicate { bound: RefinementBound::ValueGe(1), span: None },
        RefinementPredicate { bound: RefinementBound::ValueLe(99), span: None },
    ];
    let sigs = vec![refined("bounded", SignalKind::Input, SignalType::Unsigned(8), preds)];
    let module = module_from_exts(&sigs);
    let _ = typecheck_extended(&module, &sigs, &[], &[], &[]);
}
#[test]
fn refinement_bound_literal_value() {
    match RefinementBound::ValueEq(42) {
        RefinementBound::ValueEq(v) => assert_eq!(v, 42),
        _ => panic!("Expected ValueEq"),
    }
}
