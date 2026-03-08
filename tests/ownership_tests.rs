//! TYPE-004: Linear Signal Ownership tests.
//!
//! Verifies:
//! 1. Two different reflexes writing the same signal → E216
//! 2. Single reflex with multiple assignments to same signal → allowed
//! 3. Pattern-expanded conflicting writers include origin tags in E216
//! 4. Multiple signals each with unique writers → passes
//! 5. Pipeline (parse + validate) rejects multi-writer via source text

#![forbid(unsafe_code)]
#![deny(warnings)]

extern crate nasa_rust_project;

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::validate_module;

// ───────────────────── helpers ─────────────────────

fn sig(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl { name: name.to_string(), kind, ty, origin: None }
}

fn bool_out(name: &str) -> SignalDecl {
    sig(name, SignalKind::Output, SignalType::Bool)
}

fn u8_out(name: &str) -> SignalDecl {
    sig(name, SignalKind::Output, SignalType::Unsigned(8))
}

fn u8_in(name: &str) -> SignalDecl {
    sig(name, SignalKind::Input, SignalType::Unsigned(8))
}

fn u8_internal(name: &str) -> SignalDecl {
    sig(name, SignalKind::Internal, SignalType::Unsigned(8))
}

fn bool_guard(name: &str, signal_name: &str) -> Guard {
    Guard {
        name: name.to_string(),
        condition: Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal(signal_name.to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
        },
        cycles: 1,
        origin: None,
    }
}

fn assign(target: &str, value: u64) -> Assignment {
    Assignment {
        target: target.to_string(),
        value: Expr::Literal(LiteralValue::Integer(value)),
    }
}

fn assign_bool(target: &str, value: bool) -> Assignment {
    Assignment {
        target: target.to_string(),
        value: Expr::Literal(LiteralValue::Bool(value)),
    }
}

fn reflex(name: &str, guard: &str, assignments: Vec<Assignment>) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec![guard.to_string()],
        assignments,
        origin: None,
    }
}

fn reflex_with_origin(
    name: &str,
    guard: &str,
    assignments: Vec<Assignment>,
    origin: &str,
) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec![guard.to_string()],
        assignments,
        origin: Some(origin.to_string()),
    }
}

fn base_module(
    name: &str,
    signals: Vec<SignalDecl>,
    guards: Vec<Guard>,
    reflexes: Vec<Reflex>,
) -> Module {
    Module {
        name: name.to_string(),
        signals,
        guards,
        reflexes,
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
    }
}

fn validate_err(module: &Module) -> String {
    let err = validate_module(module).expect_err("should fail validation");
    err.to_string()
}

// ───────────────── E216: two reflexes writing same output ─────────────────

#[test]
fn two_reflexes_writing_same_output_e216() {
    let module = base_module(
        "multi_writer_out",
        vec![u8_in("x"), u8_out("out")],
        vec![bool_guard("g", "x")],
        vec![
            reflex("r1", "g", vec![assign("out", 1)]),
            reflex("r2", "g", vec![assign("out", 2)]),
        ],
    );
    let msg = validate_err(&module);
    assert!(msg.contains("[E216]"), "expected E216, got: {}", msg);
    assert!(msg.contains("'out'"), "should name the signal, got: {}", msg);
    assert!(msg.contains("'r1'"), "should name first reflex, got: {}", msg);
    assert!(msg.contains("'r2'"), "should name second reflex, got: {}", msg);
}

// ───────────────── E216: two reflexes writing same internal ─────────────────

#[test]
fn two_reflexes_writing_same_internal_e216() {
    let module = base_module(
        "multi_writer_int",
        vec![u8_in("x"), u8_internal("temp")],
        vec![bool_guard("g", "x")],
        vec![
            reflex("r1", "g", vec![assign("temp", 10)]),
            reflex("r2", "g", vec![assign("temp", 20)]),
        ],
    );
    let msg = validate_err(&module);
    assert!(msg.contains("[E216]"), "expected E216, got: {}", msg);
    assert!(msg.contains("'temp'"), "should name the signal, got: {}", msg);
}

// ───────────────── Allowed: single reflex, multiple assignments ─────────────────

#[test]
fn single_reflex_multiple_assignments_allowed() {
    let module = base_module(
        "single_writer",
        vec![u8_in("x"), u8_out("out")],
        vec![bool_guard("g", "x")],
        vec![reflex("r", "g", vec![assign("out", 1), assign("out", 2)])],
    );
    validate_module(&module).expect("single reflex writing same signal multiple times should pass");
}

// ───────────────── Allowed: different reflexes writing different signals ─────────────────

#[test]
fn different_reflexes_different_signals_allowed() {
    let module = base_module(
        "unique_writers",
        vec![u8_in("x"), u8_out("a"), u8_out("b")],
        vec![bool_guard("g", "x")],
        vec![
            reflex("r1", "g", vec![assign("a", 1)]),
            reflex("r2", "g", vec![assign("b", 2)]),
        ],
    );
    validate_module(&module).expect("each signal has a unique writer — should pass");
}

// ───────────────── Allowed: single reflex, single assignment ─────────────────

#[test]
fn single_reflex_single_assignment_passes() {
    let module = base_module(
        "basic",
        vec![u8_in("x"), u8_out("out")],
        vec![bool_guard("g", "x")],
        vec![reflex("r", "g", vec![assign("out", 42)])],
    );
    validate_module(&module).expect("basic single-writer module should pass");
}

// ───────────────── E216 with pattern origins ─────────────────

#[test]
fn pattern_expanded_conflict_shows_origins() {
    let module = base_module(
        "pattern_conflict",
        vec![u8_in("x"), u8_out("out")],
        vec![bool_guard("g", "x")],
        vec![
            reflex_with_origin("r_pat1", "g", vec![assign("out", 1)], "watchdog"),
            reflex_with_origin("r_pat2", "g", vec![assign("out", 2)], "limiter"),
        ],
    );
    let msg = validate_err(&module);
    assert!(msg.contains("[E216]"), "expected E216, got: {}", msg);
    assert!(
        msg.contains("pattern 'watchdog'"),
        "should mention first origin, got: {}",
        msg
    );
    assert!(
        msg.contains("pattern 'limiter'"),
        "should mention second origin, got: {}",
        msg
    );
}

// ───────────────── E216: mixed origin (one pattern, one hand-written) ─────────────────

#[test]
fn mixed_origin_conflict_no_pattern_suffix() {
    let module = base_module(
        "mixed_origin",
        vec![u8_in("x"), u8_out("out")],
        vec![bool_guard("g", "x")],
        vec![
            reflex("r_hand", "g", vec![assign("out", 1)]),
            reflex_with_origin("r_pat", "g", vec![assign("out", 2)], "watchdog"),
        ],
    );
    let msg = validate_err(&module);
    assert!(msg.contains("[E216]"), "expected E216, got: {}", msg);
    // When one side has no origin, should use the simpler message format
    assert!(msg.contains("'r_hand'"), "should name hand-written reflex, got: {}", msg);
    assert!(msg.contains("'r_pat'"), "should name pattern reflex, got: {}", msg);
}

// ───────────────── E216: three reflexes, first two conflict ─────────────────

#[test]
fn three_reflexes_first_two_conflict() {
    let module = base_module(
        "three_reflex",
        vec![u8_in("x"), u8_out("a"), u8_out("b")],
        vec![bool_guard("g", "x")],
        vec![
            reflex("r1", "g", vec![assign("a", 1)]),
            reflex("r2", "g", vec![assign("a", 2)]),
            reflex("r3", "g", vec![assign("b", 3)]),
        ],
    );
    let msg = validate_err(&module);
    assert!(msg.contains("[E216]"), "expected E216, got: {}", msg);
    assert!(msg.contains("'a'"), "conflict is on signal 'a', got: {}", msg);
}

// ───────── E216: bool output with two writers ─────────

#[test]
fn bool_output_two_writers_e216() {
    let module = base_module(
        "bool_conflict",
        vec![
            SignalDecl {
                name: "trigger".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Bool,
                origin: None,
            },
            bool_out("flag"),
        ],
        vec![Guard {
            name: "g".to_string(),
            condition: Expr::Signal("trigger".to_string()),
            cycles: 1,
            origin: None,
        }],
        vec![
            reflex("r1", "g", vec![assign_bool("flag", true)]),
            reflex("r2", "g", vec![assign_bool("flag", false)]),
        ],
    );
    let msg = validate_err(&module);
    assert!(msg.contains("[E216]"), "expected E216, got: {}", msg);
    assert!(msg.contains("'flag'"), "should name signal 'flag', got: {}", msg);
}

// ───────── Allowed: no reflexes at all ─────────

#[test]
fn empty_reflexes_passes() {
    let module = base_module(
        "no_reflexes",
        vec![u8_in("x")],
        vec![bool_guard("g", "x")],
        vec![],
    );
    validate_module(&module).expect("module with no reflexes should pass ownership check");
}

// ───────── Allowed: reflex with no assignments ─────────

#[test]
fn reflex_with_no_assignments_passes() {
    let module = base_module(
        "empty_reflex",
        vec![u8_in("x")],
        vec![bool_guard("g", "x")],
        vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![],
            origin: None,
        }],
    );
    validate_module(&module).expect("reflex with no assignments should pass");
}

// ───────── Pipeline integration: parse + validate multi-writer ─────────

#[test]
fn pipeline_parse_validate_multi_writer_e216() {
    let source = r#"
module mw_test {
    signal x: in u8;
    signal out: out u8;

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r1 {
        on g {
            out = 1;
        }
    }

    reflex r2 {
        on g {
            out = 2;
        }
    }
}
"#;
    let program = nasa_rust_project::parse_mirr(source).expect("should parse");
    let err = validate_module(&program.module).expect_err("should fail with E216");
    let msg = err.to_string();
    assert!(msg.contains("[E216]"), "expected E216, got: {}", msg);
    assert!(msg.contains("'out'"), "should name signal, got: {}", msg);
}

#[test]
fn pipeline_parse_validate_single_writer_passes() {
    let source = r#"
module sw_test {
    signal x: in u8;
    signal out: out u8;

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            out = x;
        }
    }
}
"#;
    let program = nasa_rust_project::parse_mirr(source).expect("should parse");
    validate_module(&program.module).expect("single-writer module should pass validation");
}

// ───────── E216 pinned message format ─────────

#[test]
fn e216_pinned_message_format() {
    let module = base_module(
        "pinned",
        vec![u8_in("x"), u8_out("out")],
        vec![bool_guard("g", "x")],
        vec![
            reflex("alpha", "g", vec![assign("out", 1)]),
            reflex("beta", "g", vec![assign("out", 2)]),
        ],
    );
    let msg = validate_err(&module);
    assert_eq!(
        msg,
        "Semantic error: [E216] Signal 'out' has multiple writers: reflex 'alpha' and reflex 'beta'."
    );
}

#[test]
fn e216_pinned_message_format_with_origins() {
    let module = base_module(
        "pinned_origin",
        vec![u8_in("x"), u8_out("out")],
        vec![bool_guard("g", "x")],
        vec![
            reflex_with_origin("alpha", "g", vec![assign("out", 1)], "pat_a"),
            reflex_with_origin("beta", "g", vec![assign("out", 2)], "pat_b"),
        ],
    );
    let msg = validate_err(&module);
    assert_eq!(
        msg,
        "Semantic error: [E216] Signal 'out' has multiple writers: \
         reflex 'alpha' (from pattern 'pat_a') and reflex 'beta' (from pattern 'pat_b')."
    );
}
