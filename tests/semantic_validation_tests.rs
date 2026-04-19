#![forbid(unsafe_code)]
//! Semantic validation edge-case tests.
//!
//! Covers duplicate name detection, Prev delay=0 errors, and
//! undeclared signal references inside Prev expressions.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::parse_mirr;
use nasa_rust_project::validate_module;

/// Helper: parse then validate, return the error message string.
fn validate_err(source: &str) -> String {
    let program = parse_mirr(source).expect("should parse");
    let errs = validate_module(&program.module).expect_err("should fail validation");
    errs.errors[0].to_string()
}

fn parse_only_err(source: &str) -> String {
    let err = parse_mirr(source).expect_err("should fail parse");
    err.to_string()
}

/// Helper: validate a hand-built module, return the error message.
fn validate_module_err(module: &Module) -> String {
    let errs = validate_module(module).expect_err("should fail validation");
    errs.errors[0].to_string()
}

// ---------------------------------------------------------------------------
// Duplicate name detection
// ---------------------------------------------------------------------------

#[test]
fn duplicate_signal_name_pinned_message() {
    let source = r#"
module dup_sig {
    signal x: in bool;
    signal x: out bool;

    guard g {
        when x
        for 2 cycles;
    }

    reflex r {
        on g {
            x = true;
        }
    }
}
"#;
    let msg = validate_err(source);
    assert!(
        msg.contains("[E201]") && msg.contains("Duplicate signal name: 'x'."),
        "expected E201 duplicate signal error, got: {msg}"
    );
}

#[test]
fn duplicate_guard_name_pinned_message() {
    let source = r#"
module dup_guard {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    guard g {
        when a
        for 5 cycles;
    }

    reflex r {
        on g {
            b = true;
        }
    }
}
"#;
    let msg = validate_err(source);
    assert!(
        msg.contains("[E202]") && msg.contains("Duplicate guard name: 'g'."),
        "expected E202 duplicate guard error, got: {msg}"
    );
}

#[test]
fn duplicate_reflex_name_pinned_message() {
    let source = r#"
module dup_reflex {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    reflex r {
        on g {
            b = true;
        }
    }

    reflex r {
        on g {
            b = false;
        }
    }
}
"#;
    let msg = validate_err(source);
    assert!(
        msg.contains("[E203]") && msg.contains("Duplicate reflex name: 'r'."),
        "expected E203 duplicate reflex error, got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// Prev delay=0 validation
// ---------------------------------------------------------------------------

/// Build a minimal module with a guard whose condition contains a Prev node.
fn module_with_prev_in_guard(delay: u64) -> Module {
    Module {
        name: "prev_test".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "y".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Prev { signal: "x".to_string(), delay }),
                right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
            },
            cycles: 3,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "y".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

/// Build a minimal module with a reflex RHS containing a Prev node.
fn module_with_prev_in_reflex(delay: u64) -> Module {
    Module {
        name: "prev_reflex_test".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "y".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("x".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
            },
            cycles: 3,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "y".to_string(),
                value: Expr::Prev { signal: "x".to_string(), delay },
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

#[test]
fn prev_delay_zero_in_guard_condition_pinned_message() {
    let module = module_with_prev_in_guard(0);
    let msg = validate_module_err(&module);
    assert!(
        msg.contains("[E209]")
            && msg.contains("'g' contains prev('x') with delay 0; delay must be >= 1."),
        "expected E209 prev delay error, got: {msg}"
    );
}

#[test]
fn prev_delay_zero_in_reflex_rhs_pinned_message() {
    let module = module_with_prev_in_reflex(0);
    let msg = validate_module_err(&module);
    assert!(
        msg.contains("[E209]")
            && msg.contains("'r' contains prev('x') with delay 0; delay must be >= 1."),
        "expected E209 prev delay error, got: {msg}"
    );
}

#[test]
fn prev_delay_zero_in_guard_condition_from_source_pinned_message() {
    let source = r#"
module prev_guard_zero {
    signal x: in u8;
    signal y: out bool;

    guard g {
        when prev(x, 0) > 5
        for 3 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let msg = validate_err(source);
    assert!(
        msg.contains("[E209]")
            && msg.contains("'g' contains prev('x') with delay 0; delay must be >= 1."),
        "expected E209 prev delay error from parsed source, got: {msg}"
    );
}

#[test]
fn prev_delay_zero_in_reflex_rhs_from_source_pinned_message() {
    let source = r#"
module prev_reflex_zero {
    signal x: in u8;
    signal y: out u8;

    guard g {
        when x > 5
        for 3 cycles;
    }

    reflex r {
        on g {
            y = prev(x, 0);
        }
    }
}
"#;
    let msg = validate_err(source);
    assert!(
        msg.contains("[E209]")
            && msg.contains("'r' contains prev('x') with delay 0; delay must be >= 1."),
        "expected E209 prev delay error from parsed source, got: {msg}"
    );
}

#[test]
fn malformed_prev_in_guard_reports_parse_error() {
    let source = r#"
module bad_prev_guard {
    signal x: in u8;
    signal y: out bool;

    guard g {
        when prev(x)
        for 2 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let msg = parse_only_err(source);
    assert!(
        msg.contains("prev() expects exactly 2 arguments"),
        "expected strict prev arity parse error, got: {msg}"
    );
}

#[test]
fn malformed_prev_in_reflex_reports_parse_error() {
    let source = r#"
module bad_prev_reflex {
    signal x: in u8;
    signal y: out u8;

    guard g {
        when x > 1
        for 2 cycles;
    }

    reflex r {
        on g {
            y = prev(x, y);
        }
    }
}
"#;
    let msg = parse_only_err(source);
    assert!(
        msg.contains("delay must be an integer literal"),
        "expected prev delay literal parse error, got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// Undeclared signal inside Prev
// ---------------------------------------------------------------------------

#[test]
fn undeclared_signal_inside_prev_in_guard() {
    // Prev references 'ghost' which is not in signal list.
    let module = Module {
        name: "prev_undecl_guard".to_string(),
        signals: vec![SignalDecl {
            name: "y".to_string(),
            kind: SignalKind::Output,
            ty: ExtendedType::from_core(SignalType::Bool),
            origin: None,
            span: None,
        }],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Prev { signal: "ghost".to_string(), delay: 1 },
            cycles: 2,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "y".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    let msg = validate_module_err(&module);
    assert!(
        msg.contains("[E204]") && msg.contains("Guard 'g' references undeclared signal 'ghost'."),
        "expected E204 undeclared signal error, got: {msg}"
    );
}

#[test]
fn undeclared_signal_inside_prev_in_reflex_rhs() {
    let module = Module {
        name: "prev_undecl_reflex".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Signal("a".to_string()),
            cycles: 2,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Prev { signal: "phantom".to_string(), delay: 1 },
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    let msg = validate_module_err(&module);
    assert!(
        msg.contains("[E208]")
            && msg.contains("Reflex 'r' assignment references undeclared signal 'phantom'."),
        "expected E208 undeclared signal error, got: {msg}"
    );
}

// ---------------------------------------------------------------------------
// Valid Prev delay (positive case — should not fail)
// ---------------------------------------------------------------------------

#[test]
fn prev_delay_one_is_valid() {
    let module = module_with_prev_in_guard(1);
    validate_module(&module).expect("should pass validation");
}

#[test]
fn prev_delay_one_from_source_is_valid() {
    let source = r#"
module prev_ok {
    signal x: in u8;
    signal y: out bool;

    guard g {
        when prev(x, 1) > 5
        for 2 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let program = parse_mirr(source).expect("should parse");
    validate_module(&program.module).expect("prev delay 1 should pass validation");
}

// ---------------------------------------------------------------------------
// Composite validation traversal budget handling
// ---------------------------------------------------------------------------

fn deep_binary_expr(depth: usize) -> Expr {
    let mut expr = Expr::Literal(LiteralValue::Integer(0));
    for _ in 0..depth {
        expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(expr),
            right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
        };
    }
    expr
}

fn module_with_composite_budget_exhaustion() -> Module {
    let invalid_array_index = Expr::ArrayIndex {
        array: Box::new(Expr::Signal("flag".to_string())),
        index: Box::new(Expr::Literal(LiteralValue::Integer(0))),
    };
    let huge_rhs = deep_binary_expr(700);

    Module {
        name: "composite_budget".to_string(),
        signals: vec![
            SignalDecl {
                name: "flag".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "out".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(invalid_array_index),
                right: Box::new(huge_rhs),
            },
            cycles: 2,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "out".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn composite_budget_error_message(module: &Module) -> String {
    let errs = validate_module(module).expect_err("expected traversal-budget semantic error");
    errs.errors
        .iter()
        .map(ToString::to_string)
        .find(|m| m.contains("[E231]"))
        .expect("expected [E231] traversal-budget diagnostic")
}

#[test]
fn composite_validation_budget_exhaustion_reports_explicit_error() {
    let module = module_with_composite_budget_exhaustion();
    let msg = composite_budget_error_message(&module);
    assert!(
        msg.contains("[E231]")
            && msg.contains("Composite semantic validation traversal budget exhausted"),
        "expected deterministic traversal-budget diagnostic, got: {msg}"
    );
}

#[test]
fn composite_validation_budget_exhaustion_error_is_deterministic() {
    let module = module_with_composite_budget_exhaustion();
    let first = composite_budget_error_message(&module);
    let second = composite_budget_error_message(&module);
    assert_eq!(first, second, "traversal-budget diagnostic must be deterministic");
}
