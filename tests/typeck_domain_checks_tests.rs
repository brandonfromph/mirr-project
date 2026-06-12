#![forbid(unsafe_code)]

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Assignment, Module, Reflex};
use mirrc::ast::types::{SignalKind, SignalType};
use mirrc::ast::SignalDecl;
use mirrc::typeck::extended::error_codes;
use mirrc::typeck::extended::{
    typecheck_extended, ClockDomain, ExtendedSignalDecl, ExtendedType, PhantomTag, SessionProtocol,
    SessionRole, SessionTransition, SessionTypeRef, TypeQualifier,
};

fn ext_base(name: &str, kind: SignalKind, ty: SignalType) -> ExtendedSignalDecl {
    ExtendedSignalDecl {
        name: name.to_string(),
        kind,
        ty: ty.clone(),
        extended_ty: ExtendedType {
            base: ty,
            refinements: Vec::new(),
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

fn ext_pure(name: &str) -> ExtendedSignalDecl {
    let mut e = ext_base(name, SignalKind::Output, SignalType::Bool);
    e.extended_ty.qualifiers.push(TypeQualifier::Pure);
    e
}

fn ext_stateful(name: &str) -> ExtendedSignalDecl {
    let mut e = ext_base(name, SignalKind::Output, SignalType::Bool);
    e.extended_ty.qualifiers.push(TypeQualifier::Stateful);
    e
}

fn ext_clk(name: &str, domain: &str) -> ExtendedSignalDecl {
    let mut e = ext_base(name, SignalKind::Output, SignalType::Bool);
    e.extended_ty.clock_domain = Some(ClockDomain { name: domain.to_string(), frequency_hz: None });
    e
}

fn ext_phantom(name: &str, tag: &str) -> ExtendedSignalDecl {
    let mut e = ext_base(name, SignalKind::Output, SignalType::Bool);
    e.extended_ty.phantom = Some(PhantomTag { tag: tag.to_string() });
    e
}

fn ext_session(name: &str, proto: &str, state: &str) -> ExtendedSignalDecl {
    let mut e = ext_base(name, SignalKind::Output, SignalType::Bool);
    e.extended_ty.session = Some(SessionTypeRef {
        protocol: proto.to_string(),
        state: state.to_string(),
        role: SessionRole::Sender,
    });
    e
}

fn assign(target: &str, value: Expr) -> Assignment {
    Assignment { target: target.to_string(), value, span: None }
}

fn reflex(name: &str, assignments: Vec<Assignment>) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec!["always".to_string()],
        assignments,
        span: None,
        origin: None,
    }
}

fn module_from_exts(exts: &[ExtendedSignalDecl], reflexes: Vec<Reflex>) -> Module {
    Module {
        name: "test".to_string(),
        signals: exts
            .iter()
            .map(|e| SignalDecl {
                name: e.name.clone(),
                kind: e.kind,
                ty: mirrc::ast::types::ExtendedType::from_core(e.ty.clone()),
                origin: None,
                span: None,
            })
            .collect(),
        guards: Vec::new(),
        reflexes,
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

#[test]
fn e616_pure_signal_cannot_use_prev() {
    let sigs = vec![ext_pure("p")];
    let rx = reflex("r1", vec![assign("p", Expr::Prev { signal: "p".to_string(), delay: 1 })]);
    let module = module_from_exts(&sigs, vec![rx]);
    let res = typecheck_extended(&module, &sigs, &[], &[], &[]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E616_EFF_PURE));
}

#[test]
fn e617_pure_signal_cannot_reference_stateful() {
    let sigs = vec![ext_pure("p"), ext_stateful("s")];
    let rx = reflex("r1", vec![assign("p", Expr::Signal("s".to_string()))]);
    let module = module_from_exts(&sigs, vec![rx]);
    let res = typecheck_extended(&module, &sigs, &[], &[], &[]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E617_EFF_MIX));
}

#[test]
fn e619_clock_domain_undeclared() {
    let sigs = vec![ext_clk("x", "sys_clk")];
    let module = module_from_exts(&sigs, vec![]);
    // passing empty declared domains array
    let res = typecheck_extended(&module, &sigs, &[], &[], &[]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E619_CLK_UNDEF));
}

#[test]
fn e618_clock_domain_crossing_without_sync() {
    let sigs = vec![ext_clk("fast_sig", "fast"), ext_clk("slow_sig", "slow")];
    let declared_domains = vec![
        ClockDomain { name: "fast".to_string(), frequency_hz: None },
        ClockDomain { name: "slow".to_string(), frequency_hz: None },
    ];
    let rx = reflex("r1", vec![assign("slow_sig", Expr::Signal("fast_sig".to_string()))]);
    let module = module_from_exts(&sigs, vec![rx]);
    let res = typecheck_extended(&module, &sigs, &declared_domains, &[], &[]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E618_CLK_CROSS));
}

#[test]
fn e621_phantom_tag_undeclared() {
    let sigs = vec![ext_phantom("x", "Verified")];
    let module = module_from_exts(&sigs, vec![]);
    let res = typecheck_extended(&module, &sigs, &[], &[], &[]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E621_PHT_UNDEF));
}

#[test]
fn e620_phantom_tag_mismatch() {
    let sigs = vec![ext_phantom("target", "Safe"), ext_phantom("source", "Unsafe")];
    let declared_tags =
        vec![PhantomTag { tag: "Safe".to_string() }, PhantomTag { tag: "Unsafe".to_string() }];
    let rx = reflex("r1", vec![assign("target", Expr::Signal("source".to_string()))]);
    let module = module_from_exts(&sigs, vec![rx]);
    let res = typecheck_extended(&module, &sigs, &[], &declared_tags, &[]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E620_PHT_MISMATCH));
}

#[test]
fn e620_untagged_to_tagged() {
    let sigs = vec![
        ext_phantom("target", "Safe"),
        ext_base("source", SignalKind::Input, SignalType::Bool),
    ];
    let declared_tags = vec![PhantomTag { tag: "Safe".to_string() }];
    let rx = reflex("r1", vec![assign("target", Expr::Signal("source".to_string()))]);
    let module = module_from_exts(&sigs, vec![rx]);
    let res = typecheck_extended(&module, &sigs, &[], &declared_tags, &[]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E620_PHT_MISMATCH));
}

#[test]
fn e625_session_protocol_undeclared() {
    let sigs = vec![ext_session("req", "Handshake", "Idle")];
    let module = module_from_exts(&sigs, vec![]);

    // Provide a dummy protocol so protocol_map is not empty, bypassing the early return
    let dummy_proto = SessionProtocol {
        name: "Dummy".to_string(),
        transitions: vec![SessionTransition {
            from: "A".to_string(),
            to: "B".to_string(),
            guard: None,
        }],
        span: None,
    };

    let res = typecheck_extended(&module, &sigs, &[], &[], &[dummy_proto]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E625_SES_PROTOCOL));
}

#[test]
fn e625_session_state_not_in_protocol() {
    let sigs = vec![ext_session("req", "Handshake", "InvalidState")];
    let proto = SessionProtocol {
        name: "Handshake".to_string(),
        transitions: vec![SessionTransition {
            from: "Idle".to_string(),
            to: "Busy".to_string(),
            guard: None,
        }],
        span: None,
    };
    let module = module_from_exts(&sigs, vec![]);
    let res = typecheck_extended(&module, &sigs, &[], &[], &[proto]);
    assert_eq!(res.errors.len(), 1);
    assert!(res.errors.errors[0].message().contains(error_codes::E625_SES_PROTOCOL));
}
