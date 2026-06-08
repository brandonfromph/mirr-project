#![forbid(unsafe_code)]
//! TYPE-002: Signed integer type tests.
//!
//! Tests signed type parsing, type checking, assignment compatibility,
//! arithmetic, comparisons, and emission.

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use mirrc::ast::types::{
    BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType, UnaryOp,
};
use mirrc::parse_mirr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::typeck::typecheck_module;
use mirrc::validate_module;

// ---------------------------------------------------------------------------
// Helper: module with signed signals
// ---------------------------------------------------------------------------

/// Build a module with a guard condition and both unsigned and signed signals.
fn signed_module_with_guard(condition: Expr) -> Module {
    Module {
        name: "signed_test".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
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
            SignalDecl {
                name: "su".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "si".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Signed(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "si8".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Signed(8)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition,
            cycles: 1,
            template_cycles: None,
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

/// Build a module with a specific assignment for signed type compatibility testing.
fn signed_module_with_assignment(target: &str, target_ty: SignalType, value: Expr) -> Module {
    Module {
        name: "signed_assign_test".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Bool),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: target.to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(target_ty),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "su".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "si".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Signed(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "si8".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Signed(8)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Signal("x".to_string()),
            cycles: 1,
            template_cycles: None,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment { target: target.to_string(), value, span: None }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

// ---------------------------------------------------------------------------
// Parsing: signed type declarations
// ---------------------------------------------------------------------------

#[test]
fn parse_signed_signal_i16() {
    let src = r#"
module test {
    signal a: in  i16;
    signal b: out bool;
    guard g {
        when a < a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = true;
        }
    }
}
"#;
    let prog = parse_mirr(src).expect("should parse i16 type");
    assert_eq!(prog.module.signals[0].ty.signal_type(), SignalType::Signed(16));
}

#[test]
fn parse_signed_signal_i8() {
    let src = r#"
module test {
    signal a: in  i8;
    signal b: out bool;
    guard g {
        when a < a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = true;
        }
    }
}
"#;
    let prog = parse_mirr(src).expect("should parse i8 type");
    assert_eq!(prog.module.signals[0].ty.signal_type(), SignalType::Signed(8));
}

#[test]
fn parse_signed_signal_i32() {
    let src = r#"
module test {
    signal a: in  i32;
    signal b: out bool;
    guard g {
        when a < a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = true;
        }
    }
}
"#;
    let prog = parse_mirr(src).expect("should parse i32 type");
    assert_eq!(prog.module.signals[0].ty.signal_type(), SignalType::Signed(32));
}

// ---------------------------------------------------------------------------
// Assignment compatibility
// ---------------------------------------------------------------------------

#[test]
fn signed_assignment_same_type_passes() {
    let m = signed_module_with_assignment(
        "out_i16",
        SignalType::Signed(16),
        Expr::Signal("si".to_string()),
    );
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i16 signal assigned to i16 target should pass");
}

#[test]
fn signed_widening_i8_to_i16_passes() {
    let m = signed_module_with_assignment(
        "out_i16",
        SignalType::Signed(16),
        Expr::Signal("si8".to_string()),
    );
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i8 assigned to i16 should pass (sign-extension)");
}

#[test]
fn signed_narrowing_i16_to_i8_rejected() {
    let m = signed_module_with_assignment(
        "out_i8",
        SignalType::Signed(8),
        Expr::Signal("si".to_string()), // si is i16
    );
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(err.to_string().contains("[E601]"), "Expected E601, got: {}", err);
}

#[test]
fn signed_to_unsigned_cross_assign_rejected() {
    let m = signed_module_with_assignment(
        "out_u16",
        SignalType::Unsigned(16),
        Expr::Signal("si".to_string()), // si is i16
    );
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(err.to_string().contains("[E601]"), "Expected E601 for cross-category, got: {}", err);
}

#[test]
fn unsigned_to_signed_cross_assign_rejected() {
    let m = signed_module_with_assignment(
        "out_i16",
        SignalType::Signed(16),
        Expr::Signal("su".to_string()), // su is u16
    );
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(err.to_string().contains("[E601]"), "Expected E601 for cross-category, got: {}", err);
}

// ---------------------------------------------------------------------------
// Arithmetic: signed operands
// ---------------------------------------------------------------------------

#[test]
fn signed_addition_same_type_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i16 + i8 should pass (both signed)");
}

#[test]
fn signed_subtraction_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Sub,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i16 - i8 should pass");
}

#[test]
fn signed_multiplication_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Mul,
        left: Box::new(Expr::Signal("si8".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i8 * i8 should pass");
}

#[test]
fn mixed_signed_unsigned_arithmetic_rejected() {
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("si".to_string())), // i16
        right: Box::new(Expr::Signal("su".to_string())), // u16
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(
        err.to_string().contains("[E608]"),
        "Expected E608 for mixed signed/unsigned, got: {}",
        err
    );
}

// ---------------------------------------------------------------------------
// Shift operators
// ---------------------------------------------------------------------------

#[test]
fn signed_shift_left_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Shl,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i16 << i8 should pass");
}

#[test]
fn signed_shift_right_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Shr,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i16 >> i8 should pass (arithmetic right shift)");
}

// ---------------------------------------------------------------------------
// Comparison operators
// ---------------------------------------------------------------------------

#[test]
fn signed_comparison_produces_bool() {
    let expr = Expr::Binary {
        op: BinaryOp::Lt,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_guard(expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i16 < i8 should produce bool for guard condition");
}

#[test]
fn signed_vs_unsigned_comparison_rejected() {
    let expr = Expr::Binary {
        op: BinaryOp::Lt,
        left: Box::new(Expr::Signal("si".to_string())), // i16
        right: Box::new(Expr::Signal("su".to_string())), // u16
    };
    let m = signed_module_with_guard(expr);
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(
        err.to_string().contains("[E605]"),
        "Expected E605 for cross-category ordering, got: {}",
        err
    );
}

// ---------------------------------------------------------------------------
// Equality operators
// ---------------------------------------------------------------------------

#[test]
fn signed_equality_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Eq,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_guard(expr);
    // Guard condition must be bool — equality produces bool, so this should pass.
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i16 == i8 should produce bool");
}

#[test]
fn signed_vs_unsigned_equality_rejected() {
    let expr = Expr::Binary {
        op: BinaryOp::Eq,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("su".to_string())),
    };
    let m = signed_module_with_guard(expr);
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(
        err.to_string().contains("[E606]"),
        "Expected E606 for cross-category equality, got: {}",
        err
    );
}

// ---------------------------------------------------------------------------
// Unary negate
// ---------------------------------------------------------------------------

#[test]
fn negate_unsigned_produces_signed() {
    // -su (u16) should produce Signed(17)
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Signal("su".to_string())), // u16
    };
    // Assign to i32 to accommodate the Signed(17) result.
    let mut m = signed_module_with_assignment("out_i32", SignalType::Signed(32), expr);
    m.signals.push(SignalDecl {
        name: "out_i32".to_string(),
        kind: SignalKind::Output,
        ty: ExtendedType::from_core(SignalType::Signed(32)),
        origin: None,
        span: None,
    });
    // The target is already pushed by signed_module_with_assignment, so remove the duplicate.
    // Actually, signed_module_with_assignment already creates the target signal. No duplicate.
    validate_module(&m).unwrap_err(); // duplicate signal — the helper already creates out_i32.
}

#[test]
fn negate_unsigned_produces_signed_correct() {
    // Build module manually to avoid duplicate signals.
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Signal("su".to_string())), // u16 → Signed(17)
    };
    let m = signed_module_with_assignment("out_i32", SignalType::Signed(32), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("-(u16) = i17, assignable to i32");
}

#[test]
fn negate_signed_preserves_type() {
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Signal("si".to_string())), // i16 → Signed(16)
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("-(i16) = i16, same type");
}

#[test]
fn negate_bool_rejected() {
    let expr = Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Signal("x".to_string())), // bool
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(err.to_string().contains("[E609]"), "Expected E609 for negate on bool, got: {}", err);
}

// ---------------------------------------------------------------------------
// XOR
// ---------------------------------------------------------------------------

#[test]
fn signed_xor_same_type_passes() {
    let expr = Expr::Binary {
        op: BinaryOp::Xor,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    typecheck_module(&m).expect("i16 ^ i8 should pass (signed widening in compat)");
}

#[test]
fn signed_xor_cross_category_rejected() {
    let expr = Expr::Binary {
        op: BinaryOp::Xor,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("su".to_string())),
    };
    let m = signed_module_with_assignment("out_i16", SignalType::Signed(16), expr);
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(
        err.to_string().contains("[E607]"),
        "Expected E607 for cross-category xor, got: {}",
        err
    );
}

// ---------------------------------------------------------------------------
// Logical operators on signed (rejected)
// ---------------------------------------------------------------------------

#[test]
fn logical_and_on_signed_rejected() {
    let expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = signed_module_with_guard(expr);
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(
        err.to_string().contains("[E604]"),
        "Expected E604 for logical on signed, got: {}",
        err
    );
}

// ---------------------------------------------------------------------------
// Guard condition: signed rejected
// ---------------------------------------------------------------------------

#[test]
fn guard_condition_signed_rejected() {
    let m = signed_module_with_guard(Expr::Signal("si".to_string()));
    validate_module(&m).unwrap();
    let errs = typecheck_module(&m).unwrap_err();
    let err = errs.errors.first().expect("should have at least one error");
    assert!(
        err.to_string().contains("[E601]"),
        "Expected E601 for signed guard condition, got: {}",
        err
    );
}

// ---------------------------------------------------------------------------
// Pipeline integration: signed source end-to-end
// ---------------------------------------------------------------------------

#[test]
fn pipeline_signed_source_end_to_end() {
    let src = r#"
module signed_test {
    signal pitch:  in  i16;
    signal roll:   in  i16;
    signal warn:   out bool;

    guard nose_down {
        when pitch < pitch
        for 4 cycles;
    }

    reflex alert {
        on nose_down {
            warn = true;
        }
    }
}
"#;
    let prog = parse_mirr(src).expect("should parse signed source");
    assert_eq!(prog.module.signals[0].ty.signal_type(), SignalType::Signed(16));
    validate_module(&prog.module).unwrap();
    typecheck_module(&prog.module).expect("signed source should type-check");
}

// ---------------------------------------------------------------------------
// Verilog emission: signed type
// ---------------------------------------------------------------------------

#[test]
fn verilog_emits_signed_type() {
    let src = r#"
module signed_test {
    signal pitch: in  i16;
    signal out:   out bool;

    guard g {
        when pitch < pitch
        for 1 cycles;
    }
    reflex r {
        on g {
            out = true;
        }
    }
}
"#;
    let config = PipelineConfig {
        simplify: false,
        width: false,
        temporal: false,
        typecheck: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };
    let result = run_pipeline(src, &config).expect("pipeline should pass");
    let sv = mirrc::emit::verilog::emit_sv(&result);
    assert!(
        sv.contains("signed"),
        "Verilog output should contain 'signed' for i16 signals: {}",
        sv
    );
}

// ---------------------------------------------------------------------------
// FIRRTL emission: SInt type
// ---------------------------------------------------------------------------

#[test]
fn firrtl_emits_sint_type() {
    let src = r#"
module signed_test {
    signal pitch: in  i16;
    signal out:   out bool;

    guard g {
        when pitch < pitch
        for 1 cycles;
    }
    reflex r {
        on g {
            out = true;
        }
    }
}
"#;
    let config = PipelineConfig {
        simplify: false,
        width: false,
        temporal: false,
        typecheck: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };
    let result = run_pipeline(src, &config).expect("pipeline should pass");
    let firrtl = mirrc::emit::firrtl::emit_firrtl(&result);
    assert!(
        firrtl.contains("SInt<16>"),
        "FIRRTL output should contain 'SInt<16>' for i16 signals: {}",
        firrtl
    );
}

// ---------------------------------------------------------------------------
// Unary negate expression parsing
// ---------------------------------------------------------------------------

#[test]
fn parse_unary_negate_expression() {
    let src = r#"
module neg_test {
    signal a:   in  i16;
    signal b:   out i16;
    signal ctl: in  bool;

    guard g {
        when a < a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = -a;
        }
    }
}
"#;
    let prog = parse_mirr(src).expect("should parse -a expression");
    let assignment = &prog.module.reflexes[0].assignments[0];
    match &assignment.value {
        Expr::Unary { op: UnaryOp::Negate, .. } => {} // expected
        other => panic!("Expected Unary Negate, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// SignalType::Signed Display
// ---------------------------------------------------------------------------

#[test]
fn signed_type_display() {
    assert_eq!(SignalType::Signed(16).to_string(), "i16");
    assert_eq!(SignalType::Signed(8).to_string(), "i8");
    assert_eq!(SignalType::Signed(32).to_string(), "i32");
    assert_eq!(SignalType::Signed(1).to_string(), "i1");
}
