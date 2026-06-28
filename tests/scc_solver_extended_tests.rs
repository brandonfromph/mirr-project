#![forbid(unsafe_code)]
#![allow(clippy::clone_on_copy)]

use mirrc::ast::types::{BinaryOp, LiteralValue, SignalType};
use mirrc::ecs::components::{
    AssignmentComponent, BinaryComponent, CyclesComponent, EntityKind, KindComponent,
    LiteralComponent, PrevComponent, ReflexComponent, SignalRefComponent, TypeComponent,
};
use mirrc::ecs::intern::InternId;
use mirrc::ecs::NameComponent;
use mirrc::ecs::Registry;
use mirrc::width::scc_solver::{solve_expansive, solve_nonexpansive};
use mirrc::width::types::{SccInfo, SccKind};

#[test]
fn solve_nonexpansive_updates_widths() {
    let mut reg = Registry::new();
    let kind_sig = KindComponent(EntityKind::SIGNAL(mirrc::ast::types::SignalKind::Internal));
    let e1 = reg.create_entity("e1", kind_sig.clone());
    let e2 = reg.create_entity("e2", kind_sig);

    reg.types[e1.0 as usize] =
        Some(TypeComponent(mirrc::ast::types::ExtendedType::from_core(SignalType::Unsigned(8))));
    reg.types[e2.0 as usize] =
        Some(TypeComponent(mirrc::ast::types::ExtendedType::from_core(SignalType::Unsigned(16))));

    let scc_non = SccInfo { kind: SccKind::Nonexpansive, signals: vec![e1, e2] };

    let res = solve_nonexpansive(&scc_non, &reg);
    assert_eq!(res.widths, vec![16, 16]);
}

#[test]
fn solve_nonexpansive_zero_width_diagnostic() {
    let mut reg = Registry::new();
    let kind_sig = KindComponent(EntityKind::SIGNAL(mirrc::ast::types::SignalKind::Internal));
    let e3 = reg.create_entity("e3", kind_sig);
    let scc_non_zero = SccInfo { kind: SccKind::Nonexpansive, signals: vec![e3] };
    let res = solve_nonexpansive(&scc_non_zero, &reg);
    assert_eq!(res.widths, vec![0]);
    assert!(res.diagnostics[0].message.contains("e3"));
}

#[test]
fn solve_expansive_uses_declared_width() {
    let mut reg = Registry::new();
    let kind_sig = KindComponent(EntityKind::SIGNAL(mirrc::ast::types::SignalKind::Internal));
    let e2 = reg.create_entity("e2", kind_sig);
    reg.types[e2.0 as usize] =
        Some(TypeComponent(mirrc::ast::types::ExtendedType::from_core(SignalType::Unsigned(16))));

    let scc_exp = SccInfo { kind: SccKind::Expansive, signals: vec![e2] };
    let res = solve_expansive(&scc_exp, &reg);
    assert_eq!(res.widths, vec![16]);
}

#[test]
fn solve_expansive_infers_bound_from_guards() {
    let mut reg = Registry::new();
    let kind_sig = KindComponent(EntityKind::SIGNAL(mirrc::ast::types::SignalKind::Internal));
    let kind_misc = KindComponent(EntityKind::MODULE);

    let sig_id = reg.create_entity("sig_id", kind_sig);
    reg.names[sig_id.0 as usize] = Some(NameComponent(InternId(999)));

    let a_id = reg.create_entity("a_id", kind_misc.clone());
    let reflex_id = reg.create_entity("reflex_id", kind_misc.clone());
    let prev_id = reg.create_entity("prev_id", kind_misc.clone());
    let lit_id = reg.create_entity("lit_id", kind_misc.clone());
    let bin_id = reg.create_entity("bin_id", kind_misc.clone());
    let guard_id = reg.create_entity("guard_id", kind_misc);

    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Integer(2)));
    reg.prev_ops[prev_id.0 as usize] = Some(PrevComponent { signal: sig_id, delay: 1 });
    reg.binary_ops[bin_id.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::Add, left: prev_id, right: lit_id });
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(10));

    reg.assignment_comps[a_id.0 as usize] =
        Some(AssignmentComponent { target: sig_id, target_index: None, value: bin_id });

    reg.reflex_comps[reflex_id.0 as usize] =
        Some(ReflexComponent { assignments: vec![a_id], guards: vec![guard_id], origin: None });

    let scc_exp = SccInfo { kind: SccKind::Expansive, signals: vec![sig_id] };
    let res = solve_expansive(&scc_exp, &reg);
    assert_eq!(res.widths, vec![5]);
}

#[test]
fn solve_expansive_infers_bound_from_guards_reversed_ops() {
    let mut reg = Registry::new();
    let kind_sig = KindComponent(EntityKind::SIGNAL(mirrc::ast::types::SignalKind::Internal));
    let kind_misc = KindComponent(EntityKind::MODULE);

    let sig_id = reg.create_entity("sig_id", kind_sig);
    reg.names[sig_id.0 as usize] = Some(NameComponent(InternId(999)));

    let a_id = reg.create_entity("a_id", kind_misc.clone());
    let reflex_id = reg.create_entity("reflex_id", kind_misc.clone());
    let prev_id = reg.create_entity("prev_id", kind_misc.clone());
    let lit_id = reg.create_entity("lit_id", kind_misc.clone());
    let bin_id = reg.create_entity("bin_id", kind_misc.clone());
    let guard_id = reg.create_entity("guard_id", kind_misc);

    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Integer(2)));
    reg.prev_ops[prev_id.0 as usize] = Some(PrevComponent { signal: sig_id, delay: 1 });
    reg.binary_ops[bin_id.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::Add, left: lit_id, right: prev_id });
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(10));

    reg.assignment_comps[a_id.0 as usize] =
        Some(AssignmentComponent { target: sig_id, target_index: None, value: bin_id });

    reg.reflex_comps[reflex_id.0 as usize] =
        Some(ReflexComponent { assignments: vec![a_id], guards: vec![guard_id], origin: None });

    let scc_exp = SccInfo { kind: SccKind::Expansive, signals: vec![sig_id] };
    let res = solve_expansive(&scc_exp, &reg);
    assert_eq!(res.widths, vec![5]);
}

#[test]
fn solve_expansive_handles_signal_refs() {
    let mut reg = Registry::new();
    let kind_sig = KindComponent(EntityKind::SIGNAL(mirrc::ast::types::SignalKind::Internal));
    let kind_misc = KindComponent(EntityKind::MODULE);

    let sig_id = reg.create_entity("sig_id", kind_sig);
    reg.names[sig_id.0 as usize] = Some(NameComponent(InternId(999)));

    let a_id = reg.create_entity("a_id", kind_misc.clone());
    let reflex_id = reg.create_entity("reflex_id", kind_misc.clone());
    let prev_id = reg.create_entity("prev_id", kind_misc.clone());
    let lit_id = reg.create_entity("lit_id", kind_misc.clone());
    let bin_id = reg.create_entity("bin_id", kind_misc.clone());
    let guard_id = reg.create_entity("guard_id", kind_misc.clone());
    let sig_ref = reg.create_entity("sig_ref", kind_misc);

    reg.signal_refs[sig_ref.0 as usize] = Some(SignalRefComponent(sig_id));
    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Integer(2)));
    reg.prev_ops[prev_id.0 as usize] = Some(PrevComponent { signal: sig_ref, delay: 1 });
    reg.binary_ops[bin_id.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::Add, left: prev_id, right: lit_id });
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(10));

    reg.assignment_comps[a_id.0 as usize] =
        Some(AssignmentComponent { target: sig_id, target_index: None, value: bin_id });

    reg.reflex_comps[reflex_id.0 as usize] =
        Some(ReflexComponent { assignments: vec![a_id], guards: vec![guard_id], origin: None });

    let scc_exp = SccInfo { kind: SccKind::Expansive, signals: vec![sig_id] };
    let res = solve_expansive(&scc_exp, &reg);
    assert_eq!(res.widths, vec![5]);
}

#[test]
fn solve_expansive_assignment_mismatch_fails() {
    let mut reg = Registry::new();
    let kind_sig = KindComponent(EntityKind::SIGNAL(mirrc::ast::types::SignalKind::Internal));
    let kind_misc = KindComponent(EntityKind::MODULE);

    let sig_id = reg.create_entity("sig_id", kind_sig.clone());
    let e1 = reg.create_entity("e1", kind_sig);
    reg.names[sig_id.0 as usize] = Some(NameComponent(InternId(999)));

    let a_id = reg.create_entity("a_id", kind_misc.clone());
    let reflex_id = reg.create_entity("reflex_id", kind_misc.clone());
    let prev_id = reg.create_entity("prev_id", kind_misc.clone());
    let lit_id = reg.create_entity("lit_id", kind_misc.clone());
    let bin_id = reg.create_entity("bin_id", kind_misc.clone());
    let guard_id = reg.create_entity("guard_id", kind_misc);

    reg.literals[lit_id.0 as usize] = Some(LiteralComponent(LiteralValue::Integer(2)));
    reg.prev_ops[prev_id.0 as usize] = Some(PrevComponent { signal: sig_id, delay: 1 });
    reg.binary_ops[bin_id.0 as usize] =
        Some(BinaryComponent { op: BinaryOp::Add, left: prev_id, right: lit_id });
    reg.cycles[guard_id.0 as usize] = Some(CyclesComponent(10));

    reg.assignment_comps[a_id.0 as usize] =
        Some(AssignmentComponent { target: e1, target_index: None, value: bin_id });

    reg.reflex_comps[reflex_id.0 as usize] =
        Some(ReflexComponent { assignments: vec![a_id], guards: vec![guard_id], origin: None });

    let scc_exp = SccInfo { kind: SccKind::Expansive, signals: vec![sig_id] };
    let res = solve_expansive(&scc_exp, &reg);
    assert_eq!(res.widths, vec![0]);
}
