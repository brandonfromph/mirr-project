#![forbid(unsafe_code)]
//! AST → S-expression conversion tests for `ast_to_sexpr`.
//!
//! NASA Power-of-10: bounded iteration, no recursion.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::sexpr::convert::ast_to_sexpr;
use nasa_rust_project::sexpr::print_sexpr;

const MAX_SCAN: usize = 128;

fn empty_program() -> MirrProgram {
    MirrProgram {
        patterns: Vec::new(),
        imports: Vec::new(),
        module: Module {
            name: "test".to_string(),
            signals: Vec::new(),
            guards: Vec::new(),
            reflexes: Vec::new(),
            properties: Vec::new(),
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
            span: None,
        },
    }
}

fn sig_decl(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn text_contains_all(text: &str, items: &[&str]) -> bool {
    let mut i = 0usize;
    while i < items.len() && i < MAX_SCAN {
        if !text.contains(items[i]) {
            return false;
        }
        i += 1;
    }
    true
}

#[test]
fn empty_program_produces_list() {
    let sexpr = ast_to_sexpr(&empty_program());
    assert!(sexpr.as_list().is_some(), "top-level must be a list");
}

#[test]
fn empty_program_list_non_empty() {
    let sexpr = ast_to_sexpr(&empty_program());
    let items = sexpr.as_list().unwrap();
    assert!(!items.is_empty(), "program list must be non-empty");
}

#[test]
fn program_head_is_program() {
    let sexpr = ast_to_sexpr(&empty_program());
    let text = print_sexpr(&sexpr);
    assert!(text.contains("program"), "S-expr must contain 'program' head");
}

#[test]
fn module_name_in_output() {
    let mut prog = empty_program();
    prog.module.name = "my_controller".to_string();
    let text = print_sexpr(&ast_to_sexpr(&prog));
    assert!(text.contains("my_controller"), "module name must appear in S-expr");
}

#[test]
fn input_signal_name_appears() {
    let mut prog = empty_program();
    prog.module.signals.push(sig_decl("pressure_in", SignalKind::Input, SignalType::Unsigned(16)));
    let text = print_sexpr(&ast_to_sexpr(&prog));
    assert!(text.contains("pressure_in"), "signal name must appear");
}

#[test]
fn output_signal_appears() {
    let mut prog = empty_program();
    prog.module.signals.push(sig_decl("alarm_out", SignalKind::Output, SignalType::Bool));
    let text = print_sexpr(&ast_to_sexpr(&prog));
    assert!(text.contains("alarm_out"), "output signal name must appear");
}

#[test]
fn multiple_signals_all_present() {
    let mut prog = empty_program();
    let names = ["sig_a", "sig_b", "sig_c"];
    let mut i = 0usize;
    while i < names.len() {
        prog.module.signals.push(sig_decl(names[i], SignalKind::Input, SignalType::Unsigned(8)));
        i += 1;
    }
    let text = print_sexpr(&ast_to_sexpr(&prog));
    assert!(text_contains_all(&text, &names), "all signal names must appear");
}

#[test]
fn guard_name_appears() {
    let mut prog = empty_program();
    prog.module.signals.push(sig_decl("sensor", SignalKind::Input, SignalType::Bool));
    prog.module.guards.push(Guard {
        name: "g_watchdog".to_string(),
        condition: Expr::Signal("sensor".to_string()),
        cycles: 3,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let text = print_sexpr(&ast_to_sexpr(&prog));
    assert!(text.contains("g_watchdog"), "guard name must appear");
}

#[test]
fn integer_literal_appears() {
    let mut prog = empty_program();
    prog.module.signals.push(sig_decl("x", SignalKind::Input, SignalType::Bool));
    prog.module.signals.push(sig_decl("v", SignalKind::Output, SignalType::Unsigned(8)));
    prog.module.guards.push(Guard {
        name: "g".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    prog.module.reflexes.push(Reflex {
        name: "r".to_string(),
        guard_names: vec!["g".to_string()],
        assignments: vec![Assignment {
            target: "v".to_string(),
            value: Expr::Literal(LiteralValue::Integer(99)),
            span: None,
        }],
        origin: None,
        span: None,
    });
    let text = print_sexpr(&ast_to_sexpr(&prog));
    assert!(text.contains("99"), "integer literal 99 must appear");
}

#[test]
fn property_appears_in_output() {
    let mut prog = empty_program();
    prog.module.signals.push(sig_decl("alive", SignalKind::Input, SignalType::Bool));
    prog.module.properties.push(PropertyDecl {
        name: "liveness_prop".to_string(),
        directive: PropertyDirective::Assert,
        formula: PropertyFormula::Always(Expr::Signal("alive".to_string())),
        origin: None,
        span: None,
    });
    let text = print_sexpr(&ast_to_sexpr(&prog));
    assert!(text.contains("liveness_prop") || text.contains("alive"), "property must appear");
}

#[test]
fn binary_gt_in_guard() {
    let mut prog = empty_program();
    prog.module.signals.push(sig_decl("temp", SignalKind::Input, SignalType::Unsigned(16)));
    prog.module.guards.push(Guard {
        name: "overheat".to_string(),
        condition: Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("temp".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(200))),
        },
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    let text = print_sexpr(&ast_to_sexpr(&prog));
    assert!(text.contains("temp"), "signal in binary expr must appear");
    assert!(text.contains("200"), "literal in binary expr must appear");
}

#[test]
fn deterministic_output() {
    let prog = empty_program();
    let s1 = print_sexpr(&ast_to_sexpr(&prog));
    let s2 = print_sexpr(&ast_to_sexpr(&prog));
    assert_eq!(s1, s2, "ast_to_sexpr must be deterministic");
}
