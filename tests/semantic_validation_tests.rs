//! Semantic validation edge-case tests.
//!
//! Covers duplicate name detection, Prev delay=0 errors, and
//! undeclared signal references inside Prev expressions.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::parse_mirr;
use nasa_rust_project::validate_module;

/// Helper: parse then validate, return the error message string.
fn validate_err(source: &str) -> String {
    let program = parse_mirr(source).expect("should parse");
    let err = validate_module(&program.module).expect_err("should fail validation");
    err.to_string()
}

/// Helper: validate a hand-built module, return the error message.
fn validate_module_err(module: &Module) -> String {
    let err = validate_module(module).expect_err("should fail validation");
    err.to_string()
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
    assert_eq!(msg, "[E200] Semantic error: Duplicate signal name: 'x'.");
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
    assert_eq!(msg, "[E200] Semantic error: Duplicate guard name: 'g'.");
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
    assert_eq!(msg, "[E200] Semantic error: Duplicate reflex name: 'r'.");
}

// ---------------------------------------------------------------------------
// Prev delay=0 validation (ASTs constructed programmatically
// because the parser does not support prev() syntax)
// ---------------------------------------------------------------------------

/// Build a minimal module with a guard whose condition contains a Prev node.
fn module_with_prev_in_guard(delay: u64) -> Module {
    Module {
        name: "prev_test".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Unsigned(8),
                origin: None,
            },
            SignalDecl {
                name: "y".to_string(),
                kind: SignalKind::Output,
                ty: SignalType::Bool,
                origin: None,
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
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "y".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
            }],
            origin: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
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
                ty: SignalType::Unsigned(8),
                origin: None,
            },
            SignalDecl {
                name: "y".to_string(),
                kind: SignalKind::Output,
                ty: SignalType::Unsigned(8),
                origin: None,
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
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "y".to_string(),
                value: Expr::Prev { signal: "x".to_string(), delay },
            }],
            origin: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
    }
}

#[test]
fn prev_delay_zero_in_guard_condition_pinned_message() {
    let module = module_with_prev_in_guard(0);
    let msg = validate_module_err(&module);
    assert_eq!(
        msg,
        "[E200] Semantic error: 'g' contains prev('x') with delay 0; delay must be >= 1."
    );
}

#[test]
fn prev_delay_zero_in_reflex_rhs_pinned_message() {
    let module = module_with_prev_in_reflex(0);
    let msg = validate_module_err(&module);
    assert_eq!(
        msg,
        "[E200] Semantic error: 'r' contains prev('x') with delay 0; delay must be >= 1."
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
            ty: SignalType::Bool,
            origin: None,
        }],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Prev { signal: "ghost".to_string(), delay: 1 },
            cycles: 2,
            origin: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "y".to_string(),
                value: Expr::Literal(LiteralValue::Bool(true)),
            }],
            origin: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
    };
    let msg = validate_module_err(&module);
    assert_eq!(msg, "[E200] Semantic error: Guard 'g' references undeclared signal 'ghost'.");
}

#[test]
fn undeclared_signal_inside_prev_in_reflex_rhs() {
    let module = Module {
        name: "prev_undecl_reflex".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Bool,
                origin: None,
            },
            SignalDecl {
                name: "b".to_string(),
                kind: SignalKind::Output,
                ty: SignalType::Bool,
                origin: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Signal("a".to_string()),
            cycles: 2,
            origin: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Prev { signal: "phantom".to_string(), delay: 1 },
            }],
            origin: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
    };
    let msg = validate_module_err(&module);
    assert_eq!(
        msg,
        "[E200] Semantic error: Reflex 'r' assignment references undeclared signal 'phantom'."
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
