#![forbid(unsafe_code)]
//! MEGA-5 symbolic engine tests — G11 through G17.
//! Pipeline integration, stress tests, widening stability.

use nasa_rust_project::ast::program::{Module, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, SignalKind, SignalType};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::symbolic::{analyze_module, sym_eval_binary, sym_widen, SymValue};

const MAX_STRESS: usize = 16;

fn sig(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}
fn run_sym(
    src: &str,
) -> Result<nasa_rust_project::pipeline::PipelineResult, nasa_rust_project::error::PipelineErrors> {
    let cfg = PipelineConfig { symbolic: true, ..PipelineConfig::default() };
    run_pipeline(src, &cfg)
}

// G11: Pipeline integration
#[test]
fn g11_symbolic_valid_module() {
    let r = run_sym("module sm { signal x: in u8; signal y: out bool; }");
    assert!(r.is_ok(), "symbolic pipeline: {:?}", r.err());
}
#[test]
fn g11_symbolic_with_guard() {
    let r = run_sym(
        r#"module sg {
    signal p: in u16;
    signal a: out bool;
    guard gh { when (p > 2000) for 1 cycles; }
    reflex rh when [gh] { a = true; }
}"#,
    );
    assert!(r.is_ok(), "symbolic with guard: {:?}", r.err());
}
#[test]
fn g11_symbolic_off_works() {
    let cfg = PipelineConfig { symbolic: false, ..PipelineConfig::default() };
    let r = run_pipeline("module ns { signal x: in bool; signal y: out bool; }", &cfg);
    assert!(r.is_ok(), "symbolic=false must work");
}
#[test]
fn g11_result_field_exists() {
    if let Ok(pr) = run_sym("module sr { signal x: in u8; signal y: out bool; }") {
        let _ = pr.symbolic_result;
    }
}

// G12: Bounds
#[test]
fn g12_analyze_grows_with_signals() {
    let mut m = Module {
        name: "big".to_string(),
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    let mut i = 0usize;
    while i < MAX_STRESS.min(8) {
        m.signals.push(sig(&format!("s{}", i), SignalKind::Input, SignalType::Unsigned(8)));
        i += 1;
    }
    assert!(analyze_module(&m).is_ok(), "analysis of 8-signal module must succeed");
}

// G13: Arithmetic stress
#[test]
fn g13_add_chain() {
    let mut acc = SymValue::Concrete(0);
    let mut i = 0u64;
    while i < MAX_STRESS as u64 {
        acc = sym_eval_binary(BinaryOp::Add, acc, SymValue::Concrete(1));
        i += 1;
    }
    assert_eq!(acc, SymValue::Concrete(MAX_STRESS as u64));
}
#[test]
fn g13_mul_by_zero() {
    let r = sym_eval_binary(BinaryOp::Mul, SymValue::Concrete(42), SymValue::Concrete(0));
    assert_eq!(r, SymValue::Concrete(0));
}

// G14: Two-guard pipeline
#[test]
fn g14_two_guard_compiles() {
    let r = run_pipeline(
        r#"module tg {
    signal a: in u8; signal b: in u16;
    signal o1: out bool; signal o2: out bool;
    guard g1 { when (a > 100) for 1 cycles; }
    guard g2 { when (b > 1000) for 2 cycles; }
    reflex r1 when [g1] { o1 = true; }
    reflex r2 when [g2] { o2 = true; }
}"#,
        &PipelineConfig::default(),
    );
    assert!(r.is_ok(), "two-guard: {:?}", r.err());
}

// G15: Property with symbolic
#[test]
fn g15_property_symbolic() {
    let r = run_sym(r#"module pm { signal x: in bool; property p { always (x); } }"#);
    assert!(r.is_ok(), "property + symbolic: {:?}", r.err());
}

// G16: Interval widening
#[test]
fn g16_widen_interval_to_interval() {
    let a = SymValue::Interval { lo: 0, hi: 50 };
    let b = SymValue::Interval { lo: 25, hi: 100 };
    let r = sym_widen(a, b);
    assert!(matches!(r, SymValue::Interval { .. } | SymValue::Top));
}
#[test]
fn g16_widen_top_stays_top() {
    let r = sym_widen(SymValue::Top, SymValue::Top);
    assert_eq!(r, SymValue::Top);
}

// G17: Combined analysis
#[test]
fn g17_concrete_add_sub_round_trip() {
    let start = SymValue::Concrete(100);
    let after_add = sym_eval_binary(BinaryOp::Add, start, SymValue::Concrete(50));
    let back = sym_eval_binary(BinaryOp::Sub, after_add, SymValue::Concrete(50));
    assert_eq!(back, SymValue::Concrete(100));
}
#[test]
fn g17_eq_same_value() {
    let r = sym_eval_binary(BinaryOp::Eq, SymValue::Concrete(7), SymValue::Concrete(7));
    assert_eq!(r, SymValue::Concrete(1));
}
#[test]
fn g17_ne_different_values() {
    let r = sym_eval_binary(BinaryOp::Ne, SymValue::Concrete(7), SymValue::Concrete(8));
    assert_eq!(r, SymValue::Concrete(1));
}
