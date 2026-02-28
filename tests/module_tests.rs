// ---------------------------------------------------------------------------
// Module parser tests
// ---------------------------------------------------------------------------

use nasa_rust_project::ast::{BinaryOp, Expr, LiteralValue, SignalKind, SignalType, UnaryOp};
use nasa_rust_project::parse_mirr;

// -- Helpers --

fn assert_parse_ok(source: &str) -> nasa_rust_project::MirrProgram {
    parse_mirr(source).expect("expected parse to succeed")
}

fn assert_parse_err(source: &str, msg_contains: &str) {
    let err = parse_mirr(source).expect_err("expected parse to fail");
    assert!(
        err.to_string().contains(msg_contains),
        "error '{}' should contain '{}'",
        err,
        msg_contains
    );
}

fn bin(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    Expr::Binary {
        op,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn sig(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn int(n: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(n))
}

fn bool_lit(v: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(v))
}

fn not(e: Expr) -> Expr {
    Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(e),
    }
}

// -- Tests --

#[test]
fn minimal_empty_module() {
    let p = assert_parse_ok("module empty {\n}");
    assert_eq!(p.module.name, "empty");
    assert!(p.module.signals.is_empty());
    assert!(p.module.guards.is_empty());
    assert!(p.module.reflexes.is_empty());
}

#[test]
fn neonatal_respirator() {
    let source = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.name, "neonatal_respirator");
    assert_eq!(p.module.signals.len(), 3);
    assert_eq!(p.module.signals[0].name, "respirator_enable");
    assert_eq!(p.module.signals[0].kind, SignalKind::Input);
    assert_eq!(p.module.signals[0].ty, SignalType::Bool);
    assert_eq!(p.module.signals[1].ty, SignalType::Unsigned(16));
    assert_eq!(p.module.guards.len(), 1);
    assert_eq!(p.module.guards[0].name, "sustained_pressure_drop");
    assert_eq!(
        p.module.guards[0].condition,
        bin(BinaryOp::Lt, sig("airway_pressure"), int(50))
    );
    assert_eq!(p.module.guards[0].cycles, 1000);
    assert_eq!(p.module.reflexes.len(), 1);
    assert_eq!(
        p.module.reflexes[0].guard_names,
        ["sustained_pressure_drop"]
    );
    assert_eq!(p.module.reflexes[0].assignments.len(), 1);
    assert_eq!(p.module.reflexes[0].assignments[0].target, "clamp_valve");
    assert_eq!(
        p.module.reflexes[0].assignments[0].value,
        bool_lit(true)
    );
}

#[test]
fn all_signal_kinds_and_types() {
    let source = r#"
module kinds {
    signal a: in bool;
    signal b: out u8;
    signal c: internal u32;
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.signals[0].kind, SignalKind::Input);
    assert_eq!(p.module.signals[1].kind, SignalKind::Output);
    assert_eq!(p.module.signals[2].kind, SignalKind::Internal);
    assert_eq!(p.module.signals[1].ty, SignalType::Unsigned(8));
    assert_eq!(p.module.signals[2].ty, SignalType::Unsigned(32));
}

#[test]
fn reflex_multiple_guards_and_assignments() {
    let source = r#"
module multi {
    signal x: out bool;
    signal y: out bool;
    guard g1 {
        when x
        for 1 cycles;
    }
    guard g2 {
        when y
        for 2 cycles;
    }
    reflex r {
        on g1 and g2 {
            x = true;
            y = false;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.reflexes[0].guard_names, ["g1", "g2"]);
    assert_eq!(p.module.reflexes[0].assignments.len(), 2);
    assert_eq!(p.module.reflexes[0].assignments[0].target, "x");
    assert_eq!(
        p.module.reflexes[0].assignments[0].value,
        bool_lit(true)
    );
    assert_eq!(p.module.reflexes[0].assignments[1].target, "y");
    assert_eq!(
        p.module.reflexes[0].assignments[1].value,
        bool_lit(false)
    );
}

#[test]
fn comments_and_whitespace() {
    let source = r#"
// leading comment
module commented {
    // signal comment
    signal s: in bool;
    guard g {
        when s
        for 10 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.name, "commented");
    assert_eq!(p.module.signals.len(), 1);
    assert_eq!(p.module.guards[0].cycles, 10);
}

#[test]
fn module_with_brace_on_same_line() {
    let p = assert_parse_ok("module foo {\n}");
    assert_eq!(p.module.name, "foo");
}

#[test]
fn guard_for_single_space() {
    let source = r#"
module t {
    signal s: in bool;
    guard g {
        when s
        for 42 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.guards[0].cycles, 42);
}

#[test]
fn guard_for_no_trailing_semicolon() {
    let source = r#"
module t {
    signal s: in bool;
    guard g {
        when s
        for 7 cycles
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.guards[0].cycles, 7);
}

#[test]
fn guard_complex_condition() {
    let source = r#"
module t {
    signal eeg_spike: in bool;
    signal artifact_noise: in bool;
    guard seizure_pattern {
        when eeg_spike && !artifact_noise
        for 32 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(
        p.module.guards[0].condition,
        bin(BinaryOp::And, sig("eeg_spike"), not(sig("artifact_noise")))
    );
}

#[test]
fn reflex_arithmetic_assignment() {
    let source = r#"
module t {
    signal a: in u16;
    signal b: in u16;
    signal result: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            result = a + b * 2;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.reflexes[0].assignments[0].target, "result");
    assert_eq!(
        p.module.reflexes[0].assignments[0].value,
        bin(
            BinaryOp::Add,
            sig("a"),
            bin(BinaryOp::Mul, sig("b"), int(2))
        )
    );
}

// --- Error cases ---

#[test]
fn err_empty_file() {
    assert_parse_err("", "empty");
}

#[test]
fn err_empty_after_comments() {
    assert_parse_err("// only comment\n\n  \n", "empty");
}

#[test]
fn err_no_module() {
    assert_parse_err("signal x: in bool;", "module");
}

#[test]
fn err_module_not_closed() {
    assert_parse_err("module x { signal s: in bool;", "not closed");
}

#[test]
fn err_signal_no_semicolon() {
    assert_parse_err("module m {\n    signal x: in bool\n}", "end with");
}

#[test]
fn err_signal_no_colon() {
    assert_parse_err("module m {\n    signal x in bool;\n}", "contain");
}

#[test]
fn err_signal_bad_kind() {
    assert_parse_err(
        "module m {\n    signal x: foo bool;\n}",
        "Unknown signal kind",
    );
}

#[test]
fn err_signal_bad_type() {
    assert_parse_err(
        "module m {\n    signal x: in x32;\n}",
        "Unknown signal type",
    );
}

#[test]
fn err_signal_empty_name() {
    assert_parse_err("module m {\n    signal : in bool;\n}", "empty");
}

#[test]
fn err_guard_missing_when() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        for 1 cycles;
    }
}
"#;
    assert_parse_err(source, "when");
}

#[test]
fn err_guard_invalid_cycles() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
        for abc cycles;
    }
}
"#;
    assert_parse_err(source, "Invalid cycle count");
}

#[test]
fn err_reflex_empty_on() {
    let source = r#"
module m {
    signal s: out bool;
    guard g {
        when s
        for 1 cycles;
    }
    reflex r {
        on {
        }
    }
}
"#;
    assert_parse_err(source, "no guard names");
}

#[test]
fn err_unexpected_line_in_module() {
    assert_parse_err("module m {\n    garbage line\n}", "Unexpected");
}