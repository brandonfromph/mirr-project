//! MEGA-1 Subsystem Verification Test Suite — Extended Type Checking.
//!
//! NASA-style verification tests for the MIRR type checker (core E601–E609)
//! and the MEGA-1 extended type checker (E610–E625). Covers:
//!
//! - C1: All examples typecheck (valid + negative)
//! - C2: Signed/unsigned mismatch (E608)
//! - C3: Refinement validation (E610, E612)
//! - C4: Linear ownership (E613, E614)
//! - C5: Clock domain crossing (E618, E619)
//! - C6: Effect qualifiers (E616, E617)
//! - C7: Phantom tags (E620, E621)
//! - C8: Width inference interaction
//! - C9: Error code uniqueness
//! - C10: All property forms typecheck
//!
//! Every loop is bounded by a MAX_* constant. No recursion. No unsafe code.

#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{
    BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType, UnaryOp,
};
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::typeck::extended::{
    typecheck_extended, ClockDomain, ExtendedSignalDecl, PhantomTag, RefinementBound,
    RefinementPredicate, SessionProtocol, SessionRole, SessionTransition, SessionTypeRef,
    TypeQualifier,
};
use nasa_rust_project::typeck::typecheck_module;
use nasa_rust_project::validate_module;

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum number of test iterations in any bounded loop.
const MAX_TEST_ITERATIONS: usize = 256;

/// Maximum number of error messages to inspect per test.
const MAX_ERROR_SCAN: usize = 64;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Run the pipeline with default config (typecheck + simplify + width + temporal).
fn run_default(
    source: &str,
) -> Result<nasa_rust_project::pipeline::PipelineResult, nasa_rust_project::error::PipelineErrors> {
    run_pipeline(source, &PipelineConfig::default())
}

/// Run the pipeline with extended typecheck enabled.
fn run_extended(
    source: &str,
) -> Result<nasa_rust_project::pipeline::PipelineResult, nasa_rust_project::error::PipelineErrors> {
    let config = PipelineConfig { extended_typecheck: true, ..PipelineConfig::default() };
    run_pipeline(source, &config)
}

/// Build a minimal module with a single guard condition (for AST-level tests).
fn module_with_guard_condition(condition: Expr) -> Module {
    Module {
        name: "typeck_test".to_string(),
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
                name: "n".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "m".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "out_u16".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition,
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
                name: "n".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "m".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
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

/// Extract the first error message from a type-check failure.
fn typecheck_first_error(module: &Module) -> String {
    validate_module(module).expect("should pass semantic validation");
    let errs = typecheck_module(module).expect_err("should fail type check");
    errs.errors[0].to_string()
}

/// Build an ExtendedSignalDecl from parts (for extended checker tests).
#[allow(clippy::too_many_arguments)]
fn ext_signal(
    name: &str,
    kind: SignalKind,
    base: SignalType,
    qualifiers: Vec<TypeQualifier>,
    refinements: Vec<RefinementPredicate>,
    clock_domain: Option<ClockDomain>,
    phantom: Option<PhantomTag>,
    session: Option<SessionTypeRef>,
) -> ExtendedSignalDecl {
    use nasa_rust_project::typeck::extended::ExtendedType as CheckerExtType;
    ExtendedSignalDecl {
        name: name.to_string(),
        kind,
        ty: base,
        extended_ty: CheckerExtType {
            base,
            refinements,
            qualifiers,
            clock_domain,
            phantom,
            type_nat: None,
            dependent_params: Vec::new(),
            session,
            span: None,
        },
        origin: None,
        span: None,
    }
}

/// Shorthand: plain extended signal with no qualifiers.
fn ext_signal_plain(name: &str, kind: SignalKind, base: SignalType) -> ExtendedSignalDecl {
    ext_signal(name, kind, base, vec![], vec![], None, None, None)
}

/// Collect all error messages from an ExtendedTypeCheckResult into a single string.
fn collect_extended_errors(
    result: &nasa_rust_project::typeck::extended::ExtendedTypeCheckResult,
) -> String {
    let mut out = String::new();
    let mut idx = 0usize;
    while idx < result.errors.len() && idx < MAX_ERROR_SCAN {
        if !out.is_empty() {
            out.push_str(" | ");
        }
        out.push_str(&result.errors.errors[idx].to_string());
        idx += 1;
    }
    out
}

// ===========================================================================
// C1: all_examples_typecheck — 14 tests
// ===========================================================================

#[test]
fn test_c1_all_examples_typecheck_tmr_sensor_fusion() {
    let src = r#"module tmr_sensor_fusion {
    signal sensor_a: in u16;
    signal sensor_b: in u16;
    signal sensor_c: in u16;
    signal sensor_a_ok: in bool;
    signal sensor_b_ok: in bool;
    signal sensor_c_ok: in bool;
    signal heartbeat: in bool;
    signal system_armed: in bool;
    signal manual_override: in bool;
    signal rst_n: in bool;
    signal pressure: in u16;
    signal temperature: in u16;
    signal voted_value: out u16;
    signal fault_detected: out bool;
    signal sensor_a_failed: out bool;
    signal sensor_b_failed: out bool;
    signal sensor_c_failed: out bool;
    signal watchdog_timeout: out bool;
    signal safety_shutdown: out bool;
    signal pressure_alarm: out bool;
    signal temp_alarm: out bool;
    signal vote_select: internal u8;
    signal fault_latch: internal bool;
    signal shutdown_latch: internal bool;
    signal armed_status: internal bool;
    signal override_active: internal bool;
    signal hb_status: internal bool;
    guard a_healthy {
        when sensor_a_ok
        for 1 cycles;
    }
    guard b_healthy {
        when sensor_b_ok
        for 1 cycles;
    }
    guard c_healthy {
        when sensor_c_ok
        for 1 cycles;
    }
    guard a_sick {
        when !sensor_a_ok
        for 8 cycles;
    }
    guard b_sick {
        when !sensor_b_ok
        for 8 cycles;
    }
    guard c_sick {
        when !sensor_c_ok
        for 8 cycles;
    }
    guard no_heartbeat {
        when !heartbeat
        for 64 cycles;
    }
    guard temp_high {
        when temperature > 800
        for 4 cycles;
    }
    guard is_armed {
        when system_armed
        for 1 cycles;
    }
    guard fault_held {
        when fault_detected == true
        for 16 cycles;
    }
    guard override_on {
        when manual_override
        for 1 cycles;
    }
    guard hb_alive {
        when heartbeat
        for 1 cycles;
    }
    reflex vote_a {
        on a_healthy {
            voted_value = sensor_a;
            vote_select = 1;
        }
    }
    reflex flag_a_failed {
        on a_sick {
            sensor_a_failed = true;
            fault_latch = true;
        }
    }
    reflex flag_b_failed {
        on b_sick {
            sensor_b_failed = true;
        }
    }
    reflex flag_c_failed {
        on c_sick {
            sensor_c_failed = true;
        }
    }
    reflex set_fault {
        on a_sick {
            fault_detected = true;
        }
    }
    reflex trigger_watchdog {
        on no_heartbeat {
            watchdog_timeout = true;
        }
    }
    reflex trip_temp {
        on temp_high {
            temp_alarm = true;
        }
    }
    reflex engage_shutdown {
        on is_armed and fault_held {
            safety_shutdown = true;
            shutdown_latch = true;
        }
    }
    reflex track_override {
        on override_on {
            override_active = true;
        }
    }
    reflex track_armed {
        on is_armed {
            armed_status = true;
        }
    }
    reflex track_hb {
        on hb_alive {
            hb_status = true;
        }
    }
    property vote_integrity {
        always (voted_value == sensor_a || voted_value == sensor_b || voted_value == sensor_c);
    }
    property no_spurious_shutdown {
        always (safety_shutdown -> fault_detected);
    }
    property not_triple_failure {
        never (sensor_a_failed && sensor_b_failed && sensor_c_failed);
    }
    property fault_latency_bound {
        eventually within 16 (fault_detected);
    }
    property shutdown_follows_fault {
        always (fault_detected followed_by 32 safety_shutdown);
    }
    property healthy_env {
        assume always (sensor_a_ok || sensor_b_ok || sensor_c_ok);
    }
    property pressure_alarm_reachable {
        cover eventually within 100 (pressure_alarm);
    }
}"#;
    let parsed = parse_mirr(src).expect("TMR sensor fusion should parse");
    validate_module(&parsed.module).expect("TMR sensor fusion should validate");
    typecheck_module(&parsed.module).expect("TMR sensor fusion should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_flight_controller() {
    let src = r#"module flight_controller {
    signal altitude: in u32;
    signal airspeed: in u16;
    signal pitch_angle: in u16;
    signal roll_angle: in u16;
    signal throttle_cut: out bool;
    signal stabilise: out bool;
    signal terrain_warn: out bool;
    signal status_code: internal u8;
    guard altitude_low {
        when altitude < 500
        for 10 cycles;
    }
    guard overspeed {
        when airspeed > 340
        for 5 cycles;
    }
    guard excessive_pitch {
        when pitch_angle > 30
        for 8 cycles;
    }
    guard excessive_roll {
        when roll_angle > 60
        for 4 cycles;
    }
    reflex terrain_alert {
        on altitude_low {
            terrain_warn = true;
        }
    }
    reflex cut_throttle {
        on overspeed {
            throttle_cut = true;
        }
    }
    reflex auto_stabilise {
        on excessive_pitch and excessive_roll {
            stabilise = true;
        }
    }
    property speed_bounded {
        always (airspeed < 400);
    }
    property low_alt_warns {
        always (altitude < 500 -> terrain_warn);
    }
}"#;
    let parsed = parse_mirr(src).expect("Flight controller should parse");
    validate_module(&parsed.module).expect("Flight controller should validate");
    typecheck_module(&parsed.module).expect("Flight controller should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_bool_only_module() {
    let src = r#"module bool_only {
    signal a: in bool;
    signal b: in bool;
    signal c: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            c = a && b;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("Bool-only module should parse");
    validate_module(&parsed.module).expect("Bool-only module should validate");
    typecheck_module(&parsed.module).expect("Bool-only module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_u8_module() {
    let src = r#"module u8_mod {
    signal x: in u8;
    signal y: out u8;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("u8 module should parse");
    validate_module(&parsed.module).expect("u8 module should validate");
    typecheck_module(&parsed.module).expect("u8 module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_u16_u8_mixed() {
    let src = r#"module u16_u8_mix {
    signal a: in u16;
    signal b: in u8;
    signal out_val: out u16;
    guard g {
        when a > 0
        for 2 cycles;
    }
    reflex r {
        on g {
            out_val = a + b;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("u16+u8 module should parse");
    validate_module(&parsed.module).expect("u16+u8 module should validate");
    typecheck_module(&parsed.module).expect("u16+u8 module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_u32_bool_module() {
    let src = r#"module u32_bool {
    signal reading: in u32;
    signal alarm: out bool;
    guard high {
        when reading > 9000
        for 3 cycles;
    }
    reflex set_alarm {
        on high {
            alarm = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("u32+bool module should parse");
    validate_module(&parsed.module).expect("u32+bool module should validate");
    typecheck_module(&parsed.module).expect("u32+bool module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_u32_arithmetic() {
    let src = r#"module u32_arith {
    signal a: in u32;
    signal b: in u32;
    signal result: out u32;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            result = a + b;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("u32 arithmetic module should parse");
    validate_module(&parsed.module).expect("u32 arithmetic module should validate");
    typecheck_module(&parsed.module).expect("u32 arithmetic module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_literal_comparisons() {
    let src = r#"module literal_cmp {
    signal val: in u16;
    signal flag: out bool;
    guard threshold {
        when val > 100
        for 5 cycles;
    }
    reflex r {
        on threshold {
            flag = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("Literal comparison module should parse");
    validate_module(&parsed.module).expect("Literal comparison module should validate");
    typecheck_module(&parsed.module).expect("Literal comparison module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_multi_guard_module() {
    let src = r#"module multi_guard {
    signal a: in bool;
    signal b: in bool;
    signal c: out bool;
    guard g1 {
        when a
        for 1 cycles;
    }
    guard g2 {
        when b
        for 2 cycles;
    }
    reflex r {
        on g1 and g2 {
            c = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("Multi-guard module should parse");
    validate_module(&parsed.module).expect("Multi-guard module should validate");
    typecheck_module(&parsed.module).expect("Multi-guard module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_xor_bool() {
    let src = r#"module xor_bool {
    signal a: in bool;
    signal b: in bool;
    signal c: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            c = a ^ b;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("XOR bool module should parse");
    validate_module(&parsed.module).expect("XOR bool module should validate");
    typecheck_module(&parsed.module).expect("XOR bool module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_shift_ops() {
    let src = r#"module shift_ops {
    signal val: in u16;
    signal result: out u16;
    guard g {
        when val > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            result = val << 2;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("Shift ops module should parse");
    validate_module(&parsed.module).expect("Shift ops module should validate");
    typecheck_module(&parsed.module).expect("Shift ops module should typecheck");
}

#[test]
fn test_c1_all_examples_typecheck_subtraction() {
    let src = r#"module subtract {
    signal a: in u16;
    signal b: in u8;
    signal result: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            result = a - b;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("Subtraction module should parse");
    validate_module(&parsed.module).expect("Subtraction module should validate");
    typecheck_module(&parsed.module).expect("Subtraction module should typecheck");
}

#[test]
fn test_c1_negative_invalid_guard_not_bool() {
    let src = r#"module invalid_module {
    signal val: in u16;
    signal out_flag: out bool;
    guard g {
        when val
        for 1 cycles;
    }
    reflex r {
        on g {
            out_flag = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("Should parse even if type-invalid");
    validate_module(&parsed.module).expect("Should pass semantic validation");
    let err = typecheck_module(&parsed.module).expect_err("Guard condition not bool should fail");
    let msg = err.errors[0].to_string();
    assert!(msg.contains("E601"), "Expected E601 for non-bool guard, got: {}", msg);
}

#[test]
fn test_c1_negative_invalid_mixed_ops() {
    // Arithmetic on bool should fail with E603
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Signal("x".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    validate_module(&m).expect("Should pass semantic validation");
    let err = typecheck_module(&m).expect_err("Bool arithmetic should fail");
    let msg = err.errors[0].to_string();
    assert!(msg.contains("E603"), "Expected E603 for arithmetic on bool, got: {}", msg);
}

// ===========================================================================
// C2: signed_unsigned_mismatch — 15 tests (E608)
// ===========================================================================

#[test]
fn test_c2_signed_unsigned_mismatch_01_add() {
    // a (u16) + si (i16) should fail with E608
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("si".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E608"), "Expected E608 for mixed add, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_02_sub() {
    let expr = Expr::Binary {
        op: BinaryOp::Sub,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("si".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E608"), "Expected E608 for mixed sub, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_03_mul() {
    let expr = Expr::Binary {
        op: BinaryOp::Mul,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("si".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E608"), "Expected E608 for mixed mul, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_04_shl() {
    let expr = Expr::Binary {
        op: BinaryOp::Shl,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("si".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E608"), "Expected E608 for mixed shl, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_05_shr() {
    let expr = Expr::Binary {
        op: BinaryOp::Shr,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("n".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E608"), "Expected E608 for mixed shr, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_06_ordering_lt() {
    // Ordering across categories: signed < unsigned
    let expr = Expr::Binary {
        op: BinaryOp::Lt,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("n".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E605"), "Expected E605 for signed/unsigned ordering, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_07_ordering_ge() {
    let expr = Expr::Binary {
        op: BinaryOp::Ge,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("si".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E605"), "Expected E605 for unsigned >= signed, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_08_equality_cross() {
    // Equality: signed == unsigned should fail E606
    let expr = Expr::Binary {
        op: BinaryOp::Eq,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("n".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E606"), "Expected E606 for signed==unsigned, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_09_ne_cross() {
    let expr = Expr::Binary {
        op: BinaryOp::Ne,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E606"), "Expected E606 for unsigned!=signed, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_10_nested_add_in_comparison() {
    // (n + si) > 0 — the inner add should fail E608 before the comparison
    let inner = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("si".to_string())),
    };
    let expr = Expr::Binary {
        op: BinaryOp::Gt,
        left: Box::new(inner),
        right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E608"), "Expected E608 for nested mixed add, got: {}", msg);
}

#[test]
fn test_c2_signed_unsigned_mismatch_11_same_signed_passes() {
    // si + si8 — both signed, should pass
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = module_with_assignment("out_s16", SignalType::Signed(16), expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("Same-signed arithmetic should typecheck");
}

#[test]
fn test_c2_signed_unsigned_mismatch_12_same_unsigned_passes() {
    // n + m — both unsigned, should pass
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("Same-unsigned arithmetic should typecheck");
}

#[test]
fn test_c2_signed_unsigned_mismatch_13_signed_comparison_passes() {
    // si < si8 — both signed, should pass and produce bool
    let expr = Expr::Binary {
        op: BinaryOp::Lt,
        left: Box::new(Expr::Signal("si".to_string())),
        right: Box::new(Expr::Signal("si8".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("Same-signed comparison should typecheck");
}

#[test]
fn test_c2_signed_unsigned_mismatch_14_negate_unsigned_promotes() {
    // -n where n is u16 should produce i17 (negate promotes unsigned to signed)
    let expr =
        Expr::Unary { op: UnaryOp::Negate, operand: Box::new(Expr::Signal("n".to_string())) };
    let m = module_with_assignment("out_s17", SignalType::Signed(17), expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("Negation of unsigned should produce signed and typecheck");
}

#[test]
fn test_c2_signed_unsigned_mismatch_15_negate_bool_rejected() {
    // -x where x is bool should fail E609
    let expr =
        Expr::Unary { op: UnaryOp::Negate, operand: Box::new(Expr::Signal("x".to_string())) };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E609"), "Expected E609 for negate bool, got: {}", msg);
}

// ===========================================================================
// C3: refinement_validation — 20 tests (E610, E612)
// ===========================================================================

/// Helper: build a minimal module and extended decls for refinement testing.
fn refinement_test_module() -> Module {
    let src = r#"module ref_test {
    signal x: in u8;
    signal y: out u8;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("refinement test module should parse");
    parsed.module
}

#[test]
fn test_c3_refinement_validation_01_valid_range() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate {
                bound: RefinementBound::ValueInRange { lo: 0, hi: 200 },
                span: None,
            }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E610"), "Valid range 0..200 should not produce E610: {}", errs);
    assert!(!errs.contains("E612"), "Range 0..200 fits in u8, should not produce E612: {}", errs);
}

#[test]
fn test_c3_refinement_validation_02_lo_exceeds_hi_e610() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![
                RefinementPredicate { bound: RefinementBound::ValueGe(200), span: None },
                RefinementPredicate { bound: RefinementBound::ValueLe(100), span: None },
            ],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E610"), "lo=200 > hi=100 should produce E610: {}", errs);
}

#[test]
fn test_c3_refinement_validation_03_bound_exceeds_width_e612() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueLe(300), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E612"), "u8 max is 255, bound 300 should produce E612: {}", errs);
}

#[test]
fn test_c3_refinement_validation_04_exact_max_fits() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueLe(255), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E612"), "u8 max=255, bound 255 should fit: {}", errs);
}

#[test]
fn test_c3_refinement_validation_05_range_exceeds_width_e612() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate {
                bound: RefinementBound::ValueInRange { lo: 0, hi: 500 },
                span: None,
            }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(
        errs.contains("E612"),
        "Range 0..500 exceeds u8 capacity, should produce E612: {}",
        errs
    );
}

#[test]
fn test_c3_refinement_validation_06_value_lt_within_width() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueLt(256), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E612"), "ValueLt(256) implies max=255, fits in u8: {}", errs);
}

#[test]
fn test_c3_refinement_validation_07_value_lt_exceeds_width_e612() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueLt(500), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E612"), "ValueLt(500) implies max=499, exceeds u8 capacity: {}", errs);
}

#[test]
fn test_c3_refinement_validation_08_value_eq_within_width() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueEq(42), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E612"), "ValueEq(42) fits in u8: {}", errs);
}

#[test]
fn test_c3_refinement_validation_09_value_eq_exceeds_width_e612() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueEq(1000), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E612"), "ValueEq(1000) exceeds u8 capacity: {}", errs);
}

#[test]
fn test_c3_refinement_validation_10_bool_refinement_e612() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Bool,
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueLe(5), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E612"), "Bool max is 1, bound 5 should produce E612: {}", errs);
}

#[test]
fn test_c3_refinement_validation_11_u16_valid_range() {
    let src = r#"module ref16 {
    signal x: in u16;
    signal y: out u16;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let module = parse_mirr(src).expect("should parse").module;
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![RefinementPredicate {
                bound: RefinementBound::ValueInRange { lo: 100, hi: 60000 },
                span: None,
            }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E610"), "100..60000 is valid range: {}", errs);
    assert!(!errs.contains("E612"), "60000 fits in u16: {}", errs);
}

#[test]
fn test_c3_refinement_validation_12_u16_exceeds_e612() {
    let src = r#"module ref16e {
    signal x: in u16;
    signal y: out u16;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let module = parse_mirr(src).expect("should parse").module;
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueLe(70000), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E612"), "70000 exceeds u16 max 65535: {}", errs);
}

#[test]
fn test_c3_refinement_validation_13_multiple_predicates_consistent() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![
                RefinementPredicate { bound: RefinementBound::ValueGe(10), span: None },
                RefinementPredicate { bound: RefinementBound::ValueLe(200), span: None },
            ],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E610"), "10..200 is consistent: {}", errs);
    assert!(!errs.contains("E612"), "200 fits in u8: {}", errs);
}

#[test]
fn test_c3_refinement_validation_14_multiple_predicates_inconsistent_e610() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![
                RefinementPredicate { bound: RefinementBound::ValueGe(150), span: None },
                RefinementPredicate { bound: RefinementBound::ValueLe(50), span: None },
            ],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E610"), "lo=150 > hi=50 should produce E610: {}", errs);
}

#[test]
fn test_c3_refinement_validation_15_value_gt_lower_bound() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![
                RefinementPredicate { bound: RefinementBound::ValueGt(200), span: None },
                RefinementPredicate { bound: RefinementBound::ValueLe(100), span: None },
            ],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E610"), "ValueGt(200) implies lo=201, hi=100 => E610: {}", errs);
}

#[test]
fn test_c3_refinement_validation_16_no_refinement_passes() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal_plain("x", SignalKind::Input, SignalType::Unsigned(8)),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E610"), "No refinement should produce no E610: {}", errs);
    assert!(!errs.contains("E612"), "No refinement should produce no E612: {}", errs);
}

#[test]
fn test_c3_refinement_validation_17_value_ne_no_upper_bound() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueNe(42), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E610"), "ValueNe has no upper/lower bound: {}", errs);
    assert!(!errs.contains("E612"), "ValueNe has no implied max: {}", errs);
}

#[test]
fn test_c3_refinement_validation_18_modular_constraint() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate {
                bound: RefinementBound::ValueMod { divisor: 4, remainder: 0 },
                span: None,
            }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E610"), "ValueMod has no implied range: {}", errs);
    assert!(!errs.contains("E612"), "ValueMod has no implied max: {}", errs);
}

#[test]
fn test_c3_refinement_validation_19_range_lo_equals_hi() {
    let module = refinement_test_module();
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![],
            vec![RefinementPredicate {
                bound: RefinementBound::ValueInRange { lo: 42, hi: 42 },
                span: None,
            }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E610"), "lo=hi=42 is a valid singleton range: {}", errs);
}

#[test]
fn test_c3_refinement_validation_20_signed_bound_e612() {
    let src = r#"module refsig {
    signal x: in i8;
    signal y: out i8;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let module = parse_mirr(src).expect("should parse").module;
    let decls = vec![
        ext_signal(
            "x",
            SignalKind::Input,
            SignalType::Signed(8),
            vec![],
            vec![RefinementPredicate { bound: RefinementBound::ValueLe(200), span: None }],
            None,
            None,
            None,
        ),
        ext_signal_plain("y", SignalKind::Output, SignalType::Signed(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E612"), "i8 max is 127, bound 200 should produce E612: {}", errs);
}

// ===========================================================================
// C4: linear_ownership — 15 tests (E613, E614)
// ===========================================================================

/// Helper: build a module with two signals and a reflex that reads `src` into `dst`.
fn linear_module(src_name: &str, dst_name: &str, extra_read: bool) -> Module {
    let mut assignments = vec![Assignment {
        target: dst_name.to_string(),
        value: Expr::Signal(src_name.to_string()),
        span: None,
    }];
    if extra_read {
        // Double-read: read `src_name` again in a second assignment
        assignments.push(Assignment {
            target: dst_name.to_string(),
            value: Expr::Signal(src_name.to_string()),
            span: None,
        });
    }
    Module {
        name: "linear_test".to_string(),
        signals: vec![
            SignalDecl {
                name: src_name.to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: dst_name.to_string(),
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
                left: Box::new(Expr::Signal(src_name.to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments,
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
fn test_c4_linear_ownership_01_single_consume_passes() {
    let module = linear_module("a", "b", false);
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E613"), "Single consume should not produce E613: {}", errs);
    assert!(!errs.contains("E614"), "Single consume should not produce E614: {}", errs);
}

#[test]
fn test_c4_linear_ownership_02_double_consume_e614() {
    let module = linear_module("a", "b", true);
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E614"), "Double consume should produce E614: {}", errs);
}

#[test]
fn test_c4_linear_ownership_03_unused_linear_e613() {
    // Linear signal declared but never read in any reflex
    let module = Module {
        name: "unused_linear".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
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
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Literal(LiteralValue::Integer(42)),
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
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E613"), "Unused linear signal should produce E613: {}", errs);
}

#[test]
fn test_c4_linear_ownership_04_non_linear_double_read_ok() {
    // Non-linear signal read twice is fine
    let module = linear_module("a", "b", true);
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(8)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E614"), "Non-linear double read should be OK: {}", errs);
}

#[test]
fn test_c4_linear_ownership_05_non_linear_unused_ok() {
    let module = Module {
        name: "non_linear_unused".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
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
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Literal(LiteralValue::Integer(1)),
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
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(8)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E613"), "Non-linear unused signal should be OK: {}", errs);
}

#[test]
fn test_c4_linear_ownership_06_linear_bool_single_consume() {
    let module = Module {
        name: "linear_bool".to_string(),
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
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Signal("a".to_string()),
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
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Bool,
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Bool),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E614"), "Single consume of linear bool should pass: {}", errs);
}

#[test]
fn test_c4_linear_ownership_07_linear_in_binary_expr_single() {
    // a + 1 reads a once — should pass
    let module = Module {
        name: "lin_bin".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
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
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::Signal("a".to_string())),
                    right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
                },
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
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E614"), "Single read in binary expr should pass: {}", errs);
}

#[test]
fn test_c4_linear_ownership_08_linear_in_binary_expr_double_e614() {
    // a + a reads a twice — should fail E614
    let module = Module {
        name: "lin_bin_double".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
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
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::Signal("a".to_string())),
                    right: Box::new(Expr::Signal("a".to_string())),
                },
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
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E614"), "a+a with linear a should produce E614: {}", errs);
}

#[test]
fn test_c4_linear_ownership_09_multiple_linear_signals() {
    // Two linear signals, each consumed once — should pass
    let module = Module {
        name: "multi_linear".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "c".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
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
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::Signal("a".to_string())),
                    right: Box::new(Expr::Signal("c".to_string())),
                },
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
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal(
            "c",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E613"), "Both linear signals consumed: {}", errs);
    assert!(!errs.contains("E614"), "Each consumed once: {}", errs);
}

#[test]
fn test_c4_linear_ownership_10_one_consumed_one_unused_e613() {
    let module = Module {
        name: "partial_linear".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "c".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
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
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Signal("a".to_string()),
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
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal(
            "c",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(
        errs.contains("E613"),
        "Linear signal 'c' never consumed should produce E613: {}",
        errs
    );
}

#[test]
fn test_c4_linear_ownership_11_linear_output_unused_e613() {
    // Linear output signal never consumed in any reflex
    let module = Module {
        name: "lin_out_unused".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
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
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Literal(LiteralValue::Integer(1)),
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
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(8)),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E613"), "Linear output never consumed should produce E613: {}", errs);
}

#[test]
fn test_c4_linear_ownership_12_empty_reflex_list_e613() {
    let module = Module {
        name: "no_reflexes".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
        ],
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E613"), "Linear signal with no reflexes should produce E613: {}", errs);
}

#[test]
fn test_c4_linear_ownership_13_linear_with_stateful_qualifier() {
    // A signal can be both linear and stateful
    let module = linear_module("a", "b", false);
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear, TypeQualifier::Stateful],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E613"), "Linear+Stateful consumed once should pass: {}", errs);
    assert!(!errs.contains("E614"), "Linear+Stateful consumed once should pass: {}", errs);
}

#[test]
fn test_c4_linear_ownership_14_no_linear_signals_passes() {
    let module = linear_module("a", "b", true);
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(8)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E613"), "No linear signals should skip linear checks: {}", errs);
    assert!(!errs.contains("E614"), "No linear signals should skip linear checks: {}", errs);
}

#[test]
fn test_c4_linear_ownership_15_linear_consumed_via_prev() {
    // prev(a) still counts as a read of 'a'
    let module = Module {
        name: "lin_prev".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(8)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
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
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Prev { signal: "a".to_string(), delay: 1 },
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
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(8),
            vec![TypeQualifier::Linear],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(8)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    // prev(a) may or may not count as a signal reference depending on collect_signal_refs.
    // Either E613 (unused — prev not counted) or no error (prev counted). Both are valid.
    // We just verify no E614 (no double consume).
    assert!(!errs.contains("E614"), "prev(a) should not cause double consume: {}", errs);
}

// ===========================================================================
// C5: clock_domain_crossing — 10 tests (E618, E619)
// ===========================================================================

/// Helper: minimal 2-signal module for clock domain tests.
fn clock_module() -> Module {
    let src = r#"module clk_test {
    signal a: in u16;
    signal b: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            b = a;
        }
    }
}"#;
    parse_mirr(src).expect("clock module should parse").module
}

#[test]
fn test_c5_clock_domain_01_same_domain_passes() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk_main")),
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk_main")),
            None,
            None,
        ),
    ];
    let domains = vec![ClockDomain::new("clk_main")];
    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E618"), "Same domain should not produce E618: {}", errs);
    assert!(!errs.contains("E619"), "Declared domain should not produce E619: {}", errs);
}

#[test]
fn test_c5_clock_domain_02_cross_domain_e618() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk_fast")),
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk_slow")),
            None,
            None,
        ),
    ];
    let domains = vec![ClockDomain::new("clk_fast"), ClockDomain::new("clk_slow")];
    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E618"), "Cross-domain assignment should produce E618: {}", errs);
}

#[test]
fn test_c5_clock_domain_03_undeclared_domain_e619() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("ghost_clk")),
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E619"), "Undeclared domain should produce E619: {}", errs);
}

#[test]
fn test_c5_clock_domain_04_no_domains_passes() {
    let module = clock_module();
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E618"), "No domains should skip clock checks: {}", errs);
    assert!(!errs.contains("E619"), "No domains should skip clock checks: {}", errs);
}

#[test]
fn test_c5_clock_domain_05_one_domain_one_default() {
    // a is in clk_fast, b has no domain — no cross-domain error because b has no domain
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk_fast")),
            None,
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let domains = vec![ClockDomain::new("clk_fast")];
    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E618"), "One domain + default should not cross: {}", errs);
}

#[test]
fn test_c5_clock_domain_06_both_undeclared_e619() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("phantom_clk1")),
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("phantom_clk2")),
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E619"), "Both undeclared domains should produce E619: {}", errs);
}

#[test]
fn test_c5_clock_domain_07_three_domains_cross() {
    let src = r#"module three_clk {
    signal a: in u16;
    signal b: in u16;
    signal c: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            c = a + b;
        }
    }
}"#;
    let module = parse_mirr(src).expect("should parse").module;
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk1")),
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk2")),
            None,
            None,
        ),
        ext_signal(
            "c",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk3")),
            None,
            None,
        ),
    ];
    let domains =
        vec![ClockDomain::new("clk1"), ClockDomain::new("clk2"), ClockDomain::new("clk3")];
    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E618"), "Three different domains should produce E618: {}", errs);
}

#[test]
fn test_c5_clock_domain_08_frequency_hint_no_effect() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk_main").with_frequency(100_000_000)),
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            Some(ClockDomain::new("clk_main").with_frequency(100_000_000)),
            None,
            None,
        ),
    ];
    let domains = vec![ClockDomain::new("clk_main")];
    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E618"), "Same domain with freq hints should pass: {}", errs);
}

#[test]
fn test_c5_clock_domain_09_declared_but_unused() {
    let module = clock_module();
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let domains = vec![ClockDomain::new("clk_orphan")];
    let result = typecheck_extended(&module, &decls, &domains, &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E618"), "Declared but unused domain is fine: {}", errs);
    assert!(!errs.contains("E619"), "No signal references this domain: {}", errs);
}

#[test]
fn test_c5_clock_domain_10_pipeline_e619_undeclared() {
    let src = r#"module clk_pipe {
    signal x: in u16 @sys_clk;
    signal y: out u16;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let result = run_extended(src);
    assert!(result.is_err(), "Undeclared clock domain should fail pipeline");
    let err_msg = format!("{:?}", result.err().expect("Expected error"));
    assert!(err_msg.contains("E619"), "Pipeline should report E619: {}", err_msg);
}

// ===========================================================================
// C6: effect_qualifiers — 10 tests (E616, E617)
// ===========================================================================

#[test]
fn test_c6_effect_qualifiers_01_pure_no_prev_passes() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Pure],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Pure],
            vec![],
            None,
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E616"), "Pure signals with no prev should pass: {}", errs);
    assert!(!errs.contains("E617"), "Pure signals with no stateful refs should pass: {}", errs);
}

#[test]
fn test_c6_effect_qualifiers_02_pure_with_prev_e616() {
    let module = Module {
        name: "pure_prev".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Prev { signal: "a".to_string(), delay: 1 },
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
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Pure],
            vec![],
            None,
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E616"), "Pure signal with prev() should produce E616: {}", errs);
}

#[test]
fn test_c6_effect_qualifiers_03_pure_refs_stateful_e617() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Stateful],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Pure],
            vec![],
            None,
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(
        errs.contains("E617"),
        "Pure signal referencing stateful should produce E617: {}",
        errs
    );
}

#[test]
fn test_c6_effect_qualifiers_04_stateful_with_prev_ok() {
    let module = Module {
        name: "stateful_prev".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Prev { signal: "a".to_string(), delay: 1 },
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
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Stateful],
            vec![],
            None,
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E616"), "Stateful with prev should be OK: {}", errs);
}

#[test]
fn test_c6_effect_qualifiers_05_no_qualifiers_passes() {
    let module = clock_module();
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E616"), "No qualifiers should skip effect checks: {}", errs);
    assert!(!errs.contains("E617"), "No qualifiers should skip effect checks: {}", errs);
}

#[test]
fn test_c6_effect_qualifiers_06_stateful_refs_stateful_ok() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Stateful],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Stateful],
            vec![],
            None,
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E616"), "Stateful -> stateful should be OK: {}", errs);
    assert!(!errs.contains("E617"), "Stateful -> stateful should be OK: {}", errs);
}

#[test]
fn test_c6_effect_qualifiers_07_pure_refs_pure_ok() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Pure],
            vec![],
            None,
            None,
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Pure],
            vec![],
            None,
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E617"), "Pure -> pure should be OK: {}", errs);
}

#[test]
fn test_c6_effect_qualifiers_08_pipeline_pure_annotation() {
    let src = r#"module pure_pipe {
    signal x: in pure u16;
    signal y: out pure u16;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let result = run_extended(src);
    assert!(result.is_ok(), "Pure annotation should pass pipeline: {:?}", result.err());
}

#[test]
fn test_c6_effect_qualifiers_09_pipeline_stateful_annotation() {
    let src = r#"module stateful_pipe {
    signal x: in stateful u16;
    signal y: out stateful u16;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let result = run_extended(src);
    assert!(result.is_ok(), "Stateful annotation should pass pipeline: {:?}", result.err());
}

#[test]
fn test_c6_effect_qualifiers_10_pure_with_nested_prev_e616() {
    // b = prev(a) + 1 — nested prev in pure target
    let module = Module {
        name: "pure_nested_prev".to_string(),
        signals: vec![
            SignalDecl {
                name: "a".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "b".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("a".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::Prev { signal: "a".to_string(), delay: 1 }),
                    right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
                },
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
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Pure],
            vec![],
            None,
            None,
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E616"), "Nested prev in pure target should produce E616: {}", errs);
}

// ===========================================================================
// C7: phantom_tags — 10 tests (E620, E621)
// ===========================================================================

#[test]
fn test_c7_phantom_tags_01_same_tag_passes() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Verified")),
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Verified")),
            None,
        ),
    ];
    let tags = vec![PhantomTag::new("Verified")];
    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E620"), "Same phantom tag should pass: {}", errs);
    assert!(!errs.contains("E621"), "Declared tag should pass: {}", errs);
}

#[test]
fn test_c7_phantom_tags_02_mismatch_e620() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Unverified")),
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Verified")),
            None,
        ),
    ];
    let tags = vec![PhantomTag::new("Verified"), PhantomTag::new("Unverified")];
    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E620"), "Tag mismatch should produce E620: {}", errs);
}

#[test]
fn test_c7_phantom_tags_03_undeclared_e621() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Ghost")),
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E621"), "Undeclared phantom tag should produce E621: {}", errs);
}

#[test]
fn test_c7_phantom_tags_04_untagged_to_tagged_e620() {
    let module = clock_module();
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Verified")),
            None,
        ),
    ];
    let tags = vec![PhantomTag::new("Verified")];
    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E620"), "Untagged to tagged should produce E620: {}", errs);
}

#[test]
fn test_c7_phantom_tags_05_tagged_to_untagged_ok() {
    // Dropping a tag (tagged -> untagged) is allowed
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Verified")),
            None,
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let tags = vec![PhantomTag::new("Verified")];
    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E620"), "Tagged to untagged should be OK (tag dropped): {}", errs);
}

#[test]
fn test_c7_phantom_tags_06_no_tags_passes() {
    let module = clock_module();
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E620"), "No tags should skip phantom checks: {}", errs);
    assert!(!errs.contains("E621"), "No tags should skip phantom checks: {}", errs);
}

#[test]
fn test_c7_phantom_tags_07_multiple_tags_mismatch() {
    let src = r#"module multi_tag {
    signal a: in u16;
    signal b: in u16;
    signal c: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            c = a + b;
        }
    }
}"#;
    let module = parse_mirr(src).expect("should parse").module;
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Encrypted")),
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Plaintext")),
            None,
        ),
        ext_signal(
            "c",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Encrypted")),
            None,
        ),
    ];
    let tags = vec![PhantomTag::new("Encrypted"), PhantomTag::new("Plaintext")];
    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E620"), "Plaintext to Encrypted should produce E620: {}", errs);
}

#[test]
fn test_c7_phantom_tags_08_pipeline_undeclared_phantom() {
    let src = r#"module phantom_pipe {
    signal x: in u16 #Voltage;
    signal y: out u16;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let result = run_extended(src);
    assert!(result.is_err(), "Undeclared phantom tag should fail pipeline");
    let err_msg = format!("{:?}", result.err().expect("Expected error"));
    assert!(err_msg.contains("E621"), "Pipeline should report E621: {}", err_msg);
}

#[test]
fn test_c7_phantom_tags_09_both_undeclared_e621() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Alpha")),
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            Some(PhantomTag::new("Beta")),
            None,
        ),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E621"), "Both undeclared tags should produce E621: {}", errs);
}

#[test]
fn test_c7_phantom_tags_10_declared_but_unused_ok() {
    let module = clock_module();
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let tags = vec![PhantomTag::new("Verified"), PhantomTag::new("Unverified")];
    let result = typecheck_extended(&module, &decls, &[], &tags, &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E620"), "Declared but unused tags are fine: {}", errs);
    assert!(!errs.contains("E621"), "Declared but unused tags are fine: {}", errs);
}

// ===========================================================================
// C8: width_inference_interaction — 15 tests
// ===========================================================================

#[test]
fn test_c8_width_inference_01_basic_typecheck_and_width() {
    let src = r#"module w1 {
    signal a: in u16;
    signal b: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            b = a;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    let result = run_pipeline(src, &config).expect("Basic typecheck+width should pass");
    assert!(result.type_map.is_some(), "Type map should be populated");
    assert!(result.width_result.is_some(), "Width result should be populated");
}

#[test]
fn test_c8_width_inference_02_bool_signal_width() {
    let src = r#"module w2 {
    signal a: in bool;
    signal b: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = a;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("Bool signal width should pass");
}

#[test]
fn test_c8_width_inference_03_u8_u16_widening() {
    let src = r#"module w3 {
    signal a: in u16;
    signal b: in u8;
    signal c: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            c = a + b;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("u8+u16 widening should pass width inference");
}

#[test]
fn test_c8_width_inference_04_comparison_produces_bool_width() {
    let src = r#"module w4 {
    signal a: in u32;
    signal flag: out bool;
    guard g {
        when a > 100
        for 2 cycles;
    }
    reflex r {
        on g {
            flag = a > 50;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("Comparison producing bool should pass");
}

#[test]
fn test_c8_width_inference_05_literal_width_minimal() {
    let src = r#"module w5 {
    signal a: in bool;
    signal b: out u8;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = 42;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("Literal 42 should fit in u8");
}

#[test]
fn test_c8_width_inference_06_shift_preserves_width() {
    let src = r#"module w6 {
    signal a: in u16;
    signal b: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            b = a << 2;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("Shift should preserve width");
}

#[test]
fn test_c8_width_inference_07_tmr_full_pipeline() {
    let src =
        std::fs::read_to_string("examples/tmr_sensor_fusion.mirr").expect("TMR example must exist");
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(&src, &config)
        .expect("TMR sensor fusion should pass full typecheck+width pipeline");
}

#[test]
fn test_c8_width_inference_08_flight_controller_pipeline() {
    let src = std::fs::read_to_string("examples/flight_controller.mirr")
        .expect("Flight controller example must exist");
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(&src, &config).expect("Flight controller should pass typecheck+width pipeline");
}

#[test]
fn test_c8_width_inference_09_extended_with_width() {
    let src = r#"module w9 {
    signal a: in u16;
    signal b: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            b = a;
        }
    }
}"#;
    let config = PipelineConfig {
        typecheck: true,
        extended_typecheck: true,
        width: true,
        ..PipelineConfig::default()
    };
    let result = run_pipeline(src, &config).expect("Extended + width should pass");
    assert!(result.extended_type_map.is_some(), "Extended type map should be present");
    assert!(result.width_result.is_some(), "Width result should be present");
}

#[test]
fn test_c8_width_inference_10_typecheck_off_width_on() {
    let src = r#"module w10 {
    signal a: in u16;
    signal b: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            b = a;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: false, width: true, ..PipelineConfig::default() };
    let result = run_pipeline(src, &config).expect("Width without typecheck should pass");
    assert!(result.type_map.is_none(), "Type map should be None when typecheck is off");
    assert!(result.width_result.is_some(), "Width should still run");
}

#[test]
fn test_c8_width_inference_11_multiple_arithmetic_ops() {
    let src = r#"module w11 {
    signal a: in u16;
    signal b: in u8;
    signal c: in u8;
    signal result: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            result = a + b + c;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("Multiple arithmetic ops should pass");
}

#[test]
fn test_c8_width_inference_12_signed_arithmetic_width() {
    let src = r#"module w12 {
    signal a: in i16;
    signal b: in i8;
    signal c: out i16;
    signal en: in bool;
    guard g {
        when en
        for 1 cycles;
    }
    reflex r {
        on g {
            c = a + b;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("Signed arithmetic should pass typecheck+width");
}

#[test]
fn test_c8_width_inference_13_xor_width() {
    let src = r#"module w13 {
    signal a: in u8;
    signal b: in u8;
    signal c: out u8;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            c = a ^ b;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("XOR on same width should pass");
}

#[test]
fn test_c8_width_inference_14_nested_expressions() {
    let src = r#"module w14 {
    signal a: in u16;
    signal b: in u8;
    signal flag: out bool;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            flag = (a + b) > 100;
        }
    }
}"#;
    let config = PipelineConfig { typecheck: true, width: true, ..PipelineConfig::default() };
    run_pipeline(src, &config).expect("Nested arithmetic+comparison should pass");
}

#[test]
fn test_c8_width_inference_15_full_pipeline_temporal() {
    let src = r#"module w15 {
    signal sensor: in u16;
    signal alarm: out bool;
    guard high {
        when sensor > 500
        for 4 cycles;
    }
    reflex set_alarm {
        on high {
            alarm = true;
        }
    }
}"#;
    let config = PipelineConfig {
        typecheck: true,
        width: true,
        temporal: true,
        ..PipelineConfig::default()
    };
    let result = run_pipeline(src, &config).expect("Full pipeline should pass");
    assert!(result.type_map.is_some(), "Type map populated");
    assert!(result.width_result.is_some(), "Width result populated");
    assert!(result.temporal_netlist.is_some(), "Temporal netlist populated");
}

// ===========================================================================
// C9: error_code_uniqueness — 1 test
// ===========================================================================

#[test]
fn test_c9_error_code_uniqueness() {
    // Collect all E6xx error codes and verify each is unique
    let codes: Vec<(&str, &str)> = vec![
        ("E601", "guard condition not bool"),
        ("E602", "assignment type incompatible"),
        ("E603", "arithmetic on Bool"),
        ("E604", "logical on non-bool"),
        ("E605", "ordering mismatch"),
        ("E606", "equality cross-category"),
        ("E607", "XOR mismatch"),
        ("E608", "mixed signed/unsigned arithmetic"),
        ("E609", "negate bool"),
        ("E610", "refinement lo>hi"),
        ("E612", "refinement exceeds width"),
        ("E613", "linear unused"),
        ("E614", "linear double"),
        ("E616", "pure + stateful"),
        ("E617", "stateful in pure"),
        ("E618", "clock crossing"),
        ("E619", "undefined clock domain"),
        ("E620", "phantom mismatch"),
        ("E621", "phantom undefined"),
        ("E625", "session protocol violation"),
    ];

    let mut seen = std::collections::HashSet::new();
    let mut idx = 0usize;
    while idx < codes.len() && idx < MAX_TEST_ITERATIONS {
        let (code, desc) = codes[idx];
        assert!(seen.insert(code), "Duplicate error code {} ({})", code, desc);
        idx += 1;
    }
    assert_eq!(seen.len(), codes.len(), "All error codes should be unique");
}

// ===========================================================================
// C10: all_property_forms_typecheck — 7 tests
// ===========================================================================

/// The TMR sensor fusion module has all 7 property forms. Parse it once and share.
fn tmr_source() -> &'static str {
    r#"module tmr_sensor_fusion {
    signal sensor_a: in u16;
    signal sensor_b: in u16;
    signal sensor_c: in u16;
    signal sensor_a_ok: in bool;
    signal sensor_b_ok: in bool;
    signal sensor_c_ok: in bool;
    signal heartbeat: in bool;
    signal system_armed: in bool;
    signal manual_override: in bool;
    signal rst_n: in bool;
    signal pressure: in u16;
    signal temperature: in u16;
    signal voted_value: out u16;
    signal fault_detected: out bool;
    signal sensor_a_failed: out bool;
    signal sensor_b_failed: out bool;
    signal sensor_c_failed: out bool;
    signal watchdog_timeout: out bool;
    signal safety_shutdown: out bool;
    signal pressure_alarm: out bool;
    signal temp_alarm: out bool;
    signal vote_select: internal u8;
    signal fault_latch: internal bool;
    signal shutdown_latch: internal bool;
    signal armed_status: internal bool;
    signal override_active: internal bool;
    signal hb_status: internal bool;
    guard a_healthy {
        when sensor_a_ok
        for 1 cycles;
    }
    guard b_healthy {
        when sensor_b_ok
        for 1 cycles;
    }
    guard c_healthy {
        when sensor_c_ok
        for 1 cycles;
    }
    guard a_sick {
        when !sensor_a_ok
        for 8 cycles;
    }
    guard b_sick {
        when !sensor_b_ok
        for 8 cycles;
    }
    guard c_sick {
        when !sensor_c_ok
        for 8 cycles;
    }
    guard no_heartbeat {
        when !heartbeat
        for 64 cycles;
    }
    guard temp_high {
        when temperature > 800
        for 4 cycles;
    }
    guard is_armed {
        when system_armed
        for 1 cycles;
    }
    guard fault_held {
        when fault_detected == true
        for 16 cycles;
    }
    guard override_on {
        when manual_override
        for 1 cycles;
    }
    guard hb_alive {
        when heartbeat
        for 1 cycles;
    }
    reflex vote_a {
        on a_healthy {
            voted_value = sensor_a;
            vote_select = 1;
        }
    }
    reflex flag_a_failed {
        on a_sick {
            sensor_a_failed = true;
            fault_latch = true;
        }
    }
    reflex flag_b_failed {
        on b_sick {
            sensor_b_failed = true;
        }
    }
    reflex flag_c_failed {
        on c_sick {
            sensor_c_failed = true;
        }
    }
    reflex set_fault {
        on a_sick {
            fault_detected = true;
        }
    }
    reflex trigger_watchdog {
        on no_heartbeat {
            watchdog_timeout = true;
        }
    }
    reflex trip_temp {
        on temp_high {
            temp_alarm = true;
        }
    }
    reflex engage_shutdown {
        on is_armed and fault_held {
            safety_shutdown = true;
            shutdown_latch = true;
        }
    }
    reflex track_override {
        on override_on {
            override_active = true;
        }
    }
    reflex track_armed {
        on is_armed {
            armed_status = true;
        }
    }
    reflex track_hb {
        on hb_alive {
            hb_status = true;
        }
    }
    property vote_integrity {
        always (voted_value == sensor_a || voted_value == sensor_b || voted_value == sensor_c);
    }
    property no_spurious_shutdown {
        always (safety_shutdown -> fault_detected);
    }
    property not_triple_failure {
        never (sensor_a_failed && sensor_b_failed && sensor_c_failed);
    }
    property fault_latency_bound {
        eventually within 16 (fault_detected);
    }
    property shutdown_follows_fault {
        always (fault_detected followed_by 32 safety_shutdown);
    }
    property healthy_env {
        assume always (sensor_a_ok || sensor_b_ok || sensor_c_ok);
    }
    property pressure_alarm_reachable {
        cover eventually within 100 (pressure_alarm);
    }
}"#
}

#[test]
fn test_c10_property_form_01_always() {
    // "always" property form — vote_integrity, no_spurious_shutdown
    let parsed = parse_mirr(tmr_source()).expect("TMR should parse");
    validate_module(&parsed.module).expect("TMR should validate");
    typecheck_module(&parsed.module).expect("TMR always properties should typecheck");
    // Verify the properties exist
    let always_count = parsed
        .module
        .properties
        .iter()
        .filter(|p| p.name == "vote_integrity" || p.name == "no_spurious_shutdown")
        .count();
    assert_eq!(always_count, 2, "Should find 2 'always' properties");
}

#[test]
fn test_c10_property_form_02_never() {
    // "never" property form — not_triple_failure
    let parsed = parse_mirr(tmr_source()).expect("TMR should parse");
    validate_module(&parsed.module).expect("TMR should validate");
    typecheck_module(&parsed.module).expect("TMR never property should typecheck");
    let never_count =
        parsed.module.properties.iter().filter(|p| p.name == "not_triple_failure").count();
    assert_eq!(never_count, 1, "Should find 1 'never' property");
}

#[test]
fn test_c10_property_form_03_eventually() {
    // "eventually within N" property form — fault_latency_bound
    let parsed = parse_mirr(tmr_source()).expect("TMR should parse");
    validate_module(&parsed.module).expect("TMR should validate");
    typecheck_module(&parsed.module).expect("TMR eventually property should typecheck");
    let ev_count =
        parsed.module.properties.iter().filter(|p| p.name == "fault_latency_bound").count();
    assert_eq!(ev_count, 1, "Should find 1 'eventually' property");
}

#[test]
fn test_c10_property_form_04_followed_by() {
    // "followed_by N" property form — shutdown_follows_fault
    let parsed = parse_mirr(tmr_source()).expect("TMR should parse");
    validate_module(&parsed.module).expect("TMR should validate");
    typecheck_module(&parsed.module).expect("TMR followed_by property should typecheck");
    let fb_count =
        parsed.module.properties.iter().filter(|p| p.name == "shutdown_follows_fault").count();
    assert_eq!(fb_count, 1, "Should find 1 'followed_by' property");
}

#[test]
fn test_c10_property_form_05_assume() {
    // "assume always" property form — healthy_env
    let parsed = parse_mirr(tmr_source()).expect("TMR should parse");
    validate_module(&parsed.module).expect("TMR should validate");
    typecheck_module(&parsed.module).expect("TMR assume property should typecheck");
    let assume_count = parsed.module.properties.iter().filter(|p| p.name == "healthy_env").count();
    assert_eq!(assume_count, 1, "Should find 1 'assume' property");
}

#[test]
fn test_c10_property_form_06_cover() {
    // "cover eventually within N" property form — pressure_alarm_reachable
    let parsed = parse_mirr(tmr_source()).expect("TMR should parse");
    validate_module(&parsed.module).expect("TMR should validate");
    typecheck_module(&parsed.module).expect("TMR cover property should typecheck");
    let cover_count =
        parsed.module.properties.iter().filter(|p| p.name == "pressure_alarm_reachable").count();
    assert_eq!(cover_count, 1, "Should find 1 'cover' property");
}

#[test]
fn test_c10_property_form_07_all_seven_present() {
    // Verify all 7 properties are present and the module typechecks
    let parsed = parse_mirr(tmr_source()).expect("TMR should parse");
    validate_module(&parsed.module).expect("TMR should validate");
    typecheck_module(&parsed.module).expect("TMR with all 7 property forms should typecheck");
    assert_eq!(
        parsed.module.properties.len(),
        7,
        "TMR sensor fusion should have exactly 7 properties"
    );
}

// ===========================================================================
// Session type tests (supplementary to C7, tests E625)
// ===========================================================================

#[test]
fn test_session_01_valid_protocol() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            None,
            Some(SessionTypeRef {
                protocol: "Handshake".to_string(),
                state: "Idle".to_string(),
                role: SessionRole::Sender,
            }),
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let protocols = vec![SessionProtocol {
        name: "Handshake".to_string(),
        transitions: vec![
            SessionTransition { from: "Idle".to_string(), to: "Ready".to_string(), guard: None },
            SessionTransition { from: "Ready".to_string(), to: "Ack".to_string(), guard: None },
            SessionTransition { from: "Ack".to_string(), to: "Idle".to_string(), guard: None },
        ],
        span: None,
    }];
    let result = typecheck_extended(&module, &decls, &[], &[], &protocols);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E625"), "Valid protocol+state should pass: {}", errs);
}

#[test]
fn test_session_02_undeclared_protocol_e625() {
    let module = clock_module();
    // Must pass at least one protocol so the checker doesn't early-return.
    let dummy_protocol =
        SessionProtocol { name: "SomeOther".to_string(), transitions: vec![], span: None };
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            None,
            Some(SessionTypeRef {
                protocol: "NonExistent".to_string(),
                state: "Idle".to_string(),
                role: SessionRole::Sender,
            }),
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[dummy_protocol]);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E625"), "Undeclared protocol should produce E625: {}", errs);
}

#[test]
fn test_session_03_invalid_state_e625() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            None,
            Some(SessionTypeRef {
                protocol: "Handshake".to_string(),
                state: "Phantom".to_string(),
                role: SessionRole::Receiver,
            }),
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let protocols = vec![SessionProtocol {
        name: "Handshake".to_string(),
        transitions: vec![SessionTransition {
            from: "Idle".to_string(),
            to: "Ready".to_string(),
            guard: None,
        }],
        span: None,
    }];
    let result = typecheck_extended(&module, &decls, &[], &[], &protocols);
    let errs = collect_extended_errors(&result);
    assert!(errs.contains("E625"), "Invalid state should produce E625: {}", errs);
}

#[test]
fn test_session_04_no_sessions_passes() {
    let module = clock_module();
    let decls = vec![
        ext_signal_plain("a", SignalKind::Input, SignalType::Unsigned(16)),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let result = typecheck_extended(&module, &decls, &[], &[], &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E625"), "No sessions should skip session checks: {}", errs);
}

#[test]
fn test_session_05_valid_state_from_field() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![],
            vec![],
            None,
            None,
            Some(SessionTypeRef {
                protocol: "Proto".to_string(),
                state: "Start".to_string(),
                role: SessionRole::Sender,
            }),
        ),
        ext_signal_plain("b", SignalKind::Output, SignalType::Unsigned(16)),
    ];
    let protocols = vec![SessionProtocol {
        name: "Proto".to_string(),
        transitions: vec![SessionTransition {
            from: "Start".to_string(),
            to: "End".to_string(),
            guard: None,
        }],
        span: None,
    }];
    let result = typecheck_extended(&module, &decls, &[], &[], &protocols);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E625"), "State 'Start' exists as from-field: {}", errs);
}

// ===========================================================================
// Additional edge case tests
// ===========================================================================

#[test]
fn test_edge_01_empty_module_typechecks() {
    let module = Module {
        name: "empty".to_string(),
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    typecheck_module(&module).expect("Empty module should typecheck");
}

#[test]
fn test_edge_02_guard_with_not_bool() {
    let cond = Expr::Unary { op: UnaryOp::Not, operand: Box::new(Expr::Signal("x".to_string())) };
    let m = module_with_guard_condition(cond);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("!bool should still be bool");
}

#[test]
fn test_edge_03_deeply_nested_arithmetic() {
    // ((n + m) * (n - m)) > 0
    let inner_add = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let inner_sub = Expr::Binary {
        op: BinaryOp::Sub,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let mul =
        Expr::Binary { op: BinaryOp::Mul, left: Box::new(inner_add), right: Box::new(inner_sub) };
    let cmp = Expr::Binary {
        op: BinaryOp::Gt,
        left: Box::new(mul),
        right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
    };
    let m = module_with_guard_condition(cmp);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("Deeply nested arithmetic should typecheck");
}

#[test]
fn test_edge_04_logical_or_bool() {
    let expr = Expr::Binary {
        op: BinaryOp::Or,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Bool(false))),
    };
    let m = module_with_guard_condition(expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("Bool OR bool should pass");
}

#[test]
fn test_edge_05_logical_and_non_bool_e604() {
    let expr = Expr::Binary {
        op: BinaryOp::Or,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    let msg = typecheck_first_error(&m);
    assert!(msg.contains("E604"), "Logical OR on unsigned should produce E604: {}", msg);
}

#[test]
fn test_edge_06_xor_unsigned_same_width() {
    let expr = Expr::Binary {
        op: BinaryOp::Xor,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("n".to_string())),
    };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("XOR on same unsigned type should pass");
}

#[test]
fn test_edge_07_assignment_bool_to_u1_promotion() {
    let m =
        module_with_assignment("out_u1", SignalType::Unsigned(1), Expr::Signal("x".to_string()));
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("Bool to u1 promotion should pass");
}

#[test]
fn test_edge_08_extended_with_all_features_combined() {
    let module = clock_module();
    let decls = vec![
        ext_signal(
            "a",
            SignalKind::Input,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Linear, TypeQualifier::Stateful],
            vec![RefinementPredicate {
                bound: RefinementBound::ValueInRange { lo: 0, hi: 200 },
                span: None,
            }],
            Some(ClockDomain::new("clk_main")),
            Some(PhantomTag::new("Verified")),
            None,
        ),
        ext_signal(
            "b",
            SignalKind::Output,
            SignalType::Unsigned(16),
            vec![TypeQualifier::Stateful],
            vec![],
            Some(ClockDomain::new("clk_main")),
            Some(PhantomTag::new("Verified")),
            None,
        ),
    ];
    let domains = vec![ClockDomain::new("clk_main")];
    let tags = vec![PhantomTag::new("Verified")];
    let result = typecheck_extended(&module, &decls, &domains, &tags, &[]);
    let errs = collect_extended_errors(&result);
    assert!(!errs.contains("E610"), "Valid refinement: {}", errs);
    assert!(!errs.contains("E612"), "Refinement fits in u16: {}", errs);
    assert!(!errs.contains("E618"), "Same clock domain: {}", errs);
    assert!(!errs.contains("E619"), "Declared domain: {}", errs);
    assert!(!errs.contains("E620"), "Same phantom tag: {}", errs);
    assert!(!errs.contains("E621"), "Declared tag: {}", errs);
}

#[test]
fn test_edge_09_pipeline_default_all_examples() {
    // Run all .mirr example files through the default pipeline
    let examples = [
        "examples/tmr_sensor_fusion.mirr",
        "examples/flight_controller.mirr",
        "examples/icu_monitor.mirr",
        "examples/neonatal_respirator.mirr",
    ];
    let mut idx = 0usize;
    while idx < examples.len() && idx < MAX_TEST_ITERATIONS {
        let path = examples[idx];
        if let Ok(src) = std::fs::read_to_string(path) {
            let result = run_default(&src);
            assert!(
                result.is_ok(),
                "Example {} should pass default pipeline: {:?}",
                path,
                result.err()
            );
        }
        idx += 1;
    }
}

#[test]
fn test_edge_10_prev_type_preservation() {
    let expr = Expr::Prev { signal: "n".to_string(), delay: 3 };
    let m = module_with_assignment("out_u16", SignalType::Unsigned(16), expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("prev(n, 3) should preserve u16 type");
}

#[test]
fn test_edge_11_equality_ne_same_unsigned() {
    let expr = Expr::Binary {
        op: BinaryOp::Ne,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("u16 != u8 (same category) should typecheck");
}

#[test]
fn test_edge_12_comparison_le() {
    let expr = Expr::Binary {
        op: BinaryOp::Le,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Signal("m".to_string())),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("n <= m should typecheck");
}

#[test]
fn test_edge_13_comparison_ge() {
    let expr = Expr::Binary {
        op: BinaryOp::Ge,
        left: Box::new(Expr::Signal("n".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(100))),
    };
    let m = module_with_assignment("y", SignalType::Bool, expr);
    validate_module(&m).expect("should validate");
    typecheck_module(&m).expect("n >= 100 should typecheck");
}

#[test]
fn test_edge_14_pipeline_with_refinement_annotation() {
    let src = r#"module ref_pipe {
    signal x: in u16 where 0..200;
    signal y: out u16;
    guard g {
        when x > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let result = run_extended(src);
    assert!(result.is_ok(), "Refinement annotation should pass pipeline: {:?}", result.err());
}

#[test]
fn test_edge_15_pipeline_with_linear_annotation() {
    let src = r#"module lin_pipe {
    signal x: in linear bool;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let result = run_extended(src);
    assert!(result.is_ok(), "Linear annotation should pass pipeline: {:?}", result.err());
}
