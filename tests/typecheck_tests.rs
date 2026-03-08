//! TYPE-001: Type checker tests.
//!
//! Tests all 16 type rules (T1–T16) and all 7 error codes (E601–E607).

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, SignalKind, SignalType, UnaryOp};
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::typeck::typecheck_module;
use nasa_rust_project::validate_module;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a minimal module with a single guard condition.
fn module_with_guard_condition(condition: Expr) -> Module {
    Module {
        name: "typeck_test".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Bool,
                origin: None,
            },
            SignalDecl {
                name: "y".to_string(),
                kind: SignalKind::Output,
                ty: SignalType::Bool,
                origin: None,
            },
            SignalDecl {
                name: "n".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Unsigned(16),
                origin: None,
            },
            SignalDecl {
                name: "m".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Unsigned(8),
                origin: None,
            },
            SignalDecl {
                name: "out_u16".to_string(),
                kind: SignalKind::Output,
                ty: SignalType::Unsigned(16),
                origin: None,
            },
        ],
        guards: vec![Guard { name: "g".to_string(), condition, cycles: 2, origin: None }],
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

/// Build a module with a specific assignment for type compatibility testing.
fn module_with_assignment(target: &str, target_ty: SignalType, value: Expr) -> Module {
    Module {
        name: "assign_test".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Bool,
                origin: None,
            },
            SignalDecl {
                name: target.to_string(),
                kind: SignalKind::Output,
                ty: target_ty,
                origin: None,
            },
            SignalDecl {
                name: "n".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Unsigned(16),
                origin: None,
            },
            SignalDecl {
                name: "m".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Unsigned(8),
                origin: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Signal("x".to_string()),
            cycles: 1,
            origin: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment { target: target.to_string(), value }],
            origin: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
    }
}

fn typecheck_err(module: &Module) -> String {
    validate_module(module).expect("should pass semantic validation");
    let err = typecheck_module(module).expect_err("should fail type check");
    err.to_string()
}

// ---------------------------------------------------------------------------
// T14: Guard conditions must be Bool
// ---------------------------------------------------------------------------

#[test]
fn guard_condition_bool_signal_passes() {
    let m = module_with_guard_condition(Expr::Signal("x".to_string()));
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("bool guard condition should pass");
}

#[test]
fn guard_condition_comparison_passes() {
    // n > 5 → bool
    let cond = Expr::Binary {
        op: BinaryOp::Gt,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
    };
    let m = module_with_guard_condition(cond);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("comparison guard condition should pass");
}

#[test]
fn guard_condition_unsigned_rejected_e601() {
    // when n (u16 — not bool)
    let m = module_with_guard_condition(Expr::Signal("n".to_string()));
    let msg = typecheck_err(&m);
    assert!(msg.contains("[E601]"), "Expected E601, got: {}", msg);
    assert!(msg.contains("bool"), "Should mention bool: {}", msg);
    assert!(msg.contains("u16"), "Should mention u16: {}", msg);
}

// ---------------------------------------------------------------------------
// T1: Assignment type compatibility
// ---------------------------------------------------------------------------

#[test]
fn assignment_same_type_passes() {
    // out_u16 = n (u16 = u16)
    let m =
        module_with_assignment("out_u16", SignalType::Unsigned(16), Expr::Signal("n".to_string()));
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("same-type assignment should pass");
}

#[test]
fn assignment_bool_to_bool_passes() {
    // y = true (bool = bool)
    let m = module_with_assignment("y", SignalType::Bool, Expr::Literal(LiteralValue::Bool(true)));
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("bool-to-bool assignment should pass");
}

#[test]
fn assignment_type_mismatch_rejected_e602() {
    // y (bool) = n (u16) — mismatch
    let m = module_with_assignment("y", SignalType::Bool, Expr::Signal("n".to_string()));
    let msg = typecheck_err(&m);
    assert!(msg.contains("[E602]"), "Expected E602, got: {}", msg);
}

#[test]
fn assignment_unsigned_to_bool_rejected_e602() {
    // y (bool) = 42 (u6) — mismatch
    let m = module_with_assignment("y", SignalType::Bool, Expr::Literal(LiteralValue::Integer(42)));
    let msg = typecheck_err(&m);
    assert!(msg.contains("[E602]"), "Expected E602, got: {}", msg);
}

// ---------------------------------------------------------------------------
// T2: Arithmetic on unsigned
// ---------------------------------------------------------------------------

#[test]
fn arithmetic_unsigned_passes() {
    // n + m → unsigned(max(16,8)) = u16
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("unsigned arithmetic should pass");
}

// ---------------------------------------------------------------------------
// T3: Arithmetic on Bool → E603
// ---------------------------------------------------------------------------

#[test]
fn arithmetic_on_bool_rejected_e603() {
    // x + x where x is bool
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Signal("x".to_string())),
    };
    let m = module_with_guard_condition(Expr::Signal("x".to_string()));
    // Override the reflex to use the bad expression
    let mut m2 = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    m2.guards = m.guards;
    let msg = typecheck_err(&m2);
    assert!(msg.contains("[E603]"), "Expected E603, got: {}", msg);
}

// ---------------------------------------------------------------------------
// T4: Shift operators
// ---------------------------------------------------------------------------

#[test]
fn shift_unsigned_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Shl,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(2))),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("shift on unsigned should pass");
}

// ---------------------------------------------------------------------------
// T5: Comparison operators → Bool
// ---------------------------------------------------------------------------

#[test]
fn comparison_produces_bool() {
    // n < m → bool, assign to bool target
    let expr = Expr::Binary {
        op: BinaryOp::Lt,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("comparison should produce bool");
}

// ---------------------------------------------------------------------------
// T6: Equality operators
// ---------------------------------------------------------------------------

#[test]
fn equality_same_type_passes() {
    // n == m → bool
    let expr = Expr::Binary {
        op: BinaryOp::Eq,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("equality same category should pass");
}

#[test]
fn equality_bool_bool_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Eq,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Bool(false))),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("bool equality should pass");
}

#[test]
fn equality_cross_category_rejected_e606() {
    // x (bool) == n (u16) → error
    let expr = Expr::Binary {
        op: BinaryOp::Eq,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Signal("n".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_err(&m);
    assert!(msg.contains("[E606]"), "Expected E606, got: {}", msg);
}

// ---------------------------------------------------------------------------
// T7: Ordering on Bool → E605
// ---------------------------------------------------------------------------

#[test]
fn ordering_on_bool_rejected_e605() {
    // x < x where x is bool
    let expr = Expr::Binary {
        op: BinaryOp::Lt,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Signal("x".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_err(&m);
    assert!(msg.contains("[E605]"), "Expected E605, got: {}", msg);
}

// ---------------------------------------------------------------------------
// T8: Logical on Bool
// ---------------------------------------------------------------------------

#[test]
fn logical_and_bool_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Bool(true))),
    };
    let m = module_with_guard_condition(expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("logical and on bool should pass");
}

// ---------------------------------------------------------------------------
// T9: Logical on Unsigned → E604
// ---------------------------------------------------------------------------

#[test]
fn logical_on_unsigned_rejected_e604() {
    // n && m where both are unsigned
    let expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_err(&m);
    assert!(msg.contains("[E604]"), "Expected E604, got: {}", msg);
}

// ---------------------------------------------------------------------------
// T10: XOR matching types
// ---------------------------------------------------------------------------

#[test]
fn xor_same_type_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Xor,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Bool(false))),
    };
    let m = module_with_guard_condition(expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("xor on matching bool types should pass");
}

#[test]
fn xor_mismatched_rejected_e607() {
    // x (bool) ^ n (u16)
    let expr = Expr::Binary {
        op: BinaryOp::Xor,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Signal("n".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_err(&m);
    assert!(msg.contains("[E607]"), "Expected E607, got: {}", msg);
}

// ---------------------------------------------------------------------------
// T11/T12: Unary Not
// ---------------------------------------------------------------------------

#[test]
fn not_bool_passes() {
    let expr = Expr::Unary { op: UnaryOp::Not, operand: Box::new(Expr::Signal("x".to_string())) };
    let m = module_with_guard_condition(expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("not on bool should pass");
}

#[test]
fn not_unsigned_passes() {
    let expr = Expr::Unary { op: UnaryOp::Not, operand: Box::new(Expr::Signal("n".to_string())) };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("not on unsigned should pass (bitwise negation)");
}

// ---------------------------------------------------------------------------
// T13: Prev preserves type
// ---------------------------------------------------------------------------

#[test]
fn prev_preserves_bool_type() {
    let expr = Expr::Prev { signal: "x".to_string(), delay: 1 };
    let m = module_with_guard_condition(expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("prev on bool signal should produce bool");
}

#[test]
fn prev_preserves_unsigned_type() {
    let expr = Expr::Prev { signal: "n".to_string(), delay: 1 };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("prev on u16 signal should produce u16");
}

// ---------------------------------------------------------------------------
// T15/T16: Literal types
// ---------------------------------------------------------------------------

#[test]
fn literal_bool_is_bool() {
    let m = module_with_assignment("y", SignalType::Bool, Expr::Literal(LiteralValue::Bool(true)));
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("bool literal assignment to bool should pass");
}

#[test]
fn literal_integer_is_unsigned() {
    let m = module_with_assignment(
        "out_u16",
        SignalType::Unsigned(16),
        Expr::Literal(LiteralValue::Integer(100)),
    );
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("integer literal assignment to u16 should pass");
}

// ---------------------------------------------------------------------------
// Pipeline integration
// ---------------------------------------------------------------------------

#[test]
fn pipeline_typecheck_enabled_works() {
    let src = r#"
module typeck_pipeline {
    signal pressure: in u16;
    signal alarm: out bool;

    guard high_pressure {
        when pressure > 100
        for 2 cycles;
    }

    reflex activate_alarm {
        on high_pressure {
            alarm = true;
        }
    }
}
"#;
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
    };
    run_pipeline(src, &config).expect("well-typed program should pass full pipeline");
}

#[test]
fn pipeline_typecheck_disabled_skips() {
    let src = r#"
module skip_typeck {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 1 cycles;
    }

    reflex r {
        on g {
            b = true;
        }
    }
}
"#;
    let config = PipelineConfig {
        typecheck: false,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
    };
    run_pipeline(src, &config).expect("should pass with typecheck disabled");
}

// ---------------------------------------------------------------------------
// Parse-then-typecheck integration via text source
// ---------------------------------------------------------------------------

#[test]
fn parse_and_typecheck_well_typed_source() {
    let src = r#"
module well_typed {
    signal sensor: in u16;
    signal flag: out bool;

    guard threshold {
        when sensor > 50
        for 3 cycles;
    }

    reflex set_flag {
        on threshold {
            flag = true;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("should parse");
    validate_module(&program.module).expect("should validate");
    typecheck_module(&program.module).expect("should typecheck");
}

// ---------------------------------------------------------------------------
// Nested expressions
// ---------------------------------------------------------------------------

#[test]
fn nested_comparison_and_logical() {
    // (n > 5) && (m < 10) → bool && bool → bool → valid guard condition
    let expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Binary {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Signal("n".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
        }),
        right: Box::new(Expr::Binary {
            op: BinaryOp::Lt,
            left: Box::new(Expr::Signal("m".to_string())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(10))),
        }),
    };
    let m = module_with_guard_condition(expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("nested comparison + logical should pass");
}

#[test]
fn nested_arithmetic_in_comparison() {
    // (n + m) > 100 → u16 > u7 → bool → valid guard
    let expr = Expr::Binary {
        op: BinaryOp::Gt,
        left: Box::new(Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Signal("n".to_string())),
            right: Box::new(Expr::Signal("m".to_string())),
        }),
        right: Box::new(Expr::Literal(LiteralValue::Integer(100))),
    };
    let m = module_with_guard_condition(expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("arithmetic in comparison should pass");
}

// ---------------------------------------------------------------------------
// Bool ↔ Unsigned(1) promotion
// ---------------------------------------------------------------------------

#[test]
fn bool_to_u1_promotion_passes() {
    // Assign bool signal to u1 target — module_with_assignment already declares out_u1.
    let m =
        module_with_assignment("out_u1", SignalType::Unsigned(1), Expr::Signal("x".to_string()));
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("bool to u1 promotion should pass");
}

#[test]
fn unsigned_widening_u6_to_u8_passes() {
    // Assigning a narrower unsigned (u6 literal 42) to a wider target (u8) is safe.
    let m = module_with_assignment(
        "out8",
        SignalType::Unsigned(8),
        Expr::Literal(LiteralValue::Integer(42)), // min_bits = 6
    );
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("u6 literal assigned to u8 should pass");
}

#[test]
fn unsigned_narrowing_u16_to_u8_rejected() {
    // Assigning a wider unsigned (u16 signal) to a narrower target (u8) is rejected.
    let m = module_with_assignment(
        "out8",
        SignalType::Unsigned(8),
        Expr::Signal("n".to_string()), // n is u16
    );
    validate_module(&m).unwrap();
    let err = typecheck_module(&m).unwrap_err();
    assert!(err.to_string().contains("[E602]"), "Expected E602, got: {}", err);
}
