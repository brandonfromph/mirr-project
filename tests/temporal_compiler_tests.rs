#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

//! Integration tests for the temporal guard compiler.

use nasa_rust_project::ast::program::{Guard, Module};
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use nasa_rust_project::ast::Expr;
use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::temporal::compiler::{ImplementationStrategy, ResourceEstimator};
use nasa_rust_project::temporal::low_level_ir::{
    CompiledGuard, ConditionKind, CounterGuard, GeneratedSignalKind, ShiftRegisterGuard,
    TemporalNetlist,
};
use nasa_rust_project::temporal::TemporalGuardCompiler;

const MAX_TEST_GUARDS: usize = 64;
const MAX_TEST_SIGNALS: usize = 1024;
const MAX_TEST_STAGES: usize = 256;

// --- helpers (no recursion) ---

fn compile_src(source: &str) -> TemporalNetlist {
    let program = parse_mirr(source).expect("MIRR parse should succeed");
    let mut compiler = TemporalGuardCompiler::new();
    compiler.compile_temporal_guards(&program.module).expect("compilation should succeed")
}

fn compile_guards(guards: Vec<Guard>) -> TemporalNetlist {
    let module = Module {
        name: "t".into(),
        signals: Vec::new(),
        guards,
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    let mut compiler = TemporalGuardCompiler::new();
    compiler.compile_temporal_guards(&module).expect("compilation should succeed")
}

fn sig_guard(name: &str, signal: &str, cycles: u64) -> Guard {
    Guard {
        name: name.into(),
        condition: Expr::Signal(signal.into()),
        cycles,
        origin: None,
        span: None,
    }
}

fn guard_src(name: &str, cond: &str, cycles: u64) -> String {
    format!("module t {{\n    signal {cond}: in bool;\n    guard {name} {{\n        when {cond}\n        for {cycles} cycles;\n    }}\n}}\n")
}

fn expect_sr(netlist: &TemporalNetlist) -> ShiftRegisterGuard {
    match &netlist.guards[0] {
        CompiledGuard::ShiftRegister(sr) => sr.clone(),
        other => panic!("expected ShiftRegister, got {:?}", other),
    }
}

fn expect_counter(netlist: &TemporalNetlist) -> CounterGuard {
    match &netlist.guards[0] {
        CompiledGuard::Counter(c) => c.clone(),
        other => panic!("expected Counter, got {:?}", other),
    }
}

// --- shift-register compilation (N <= 16) ---

#[test]
fn test_simple_signal_shift_register() {
    let nl = compile_src(&guard_src("g", "a", 4));
    assert_eq!(nl.guards.len(), 1, "should have exactly one guard");
    let sr = expect_sr(&nl);
    assert_eq!(sr.name, "g", "guard name should be 'g'");
    assert_eq!(sr.delay_cycles, 4, "delay should be 4 cycles");
    assert_eq!(sr.stages.len(), 4, "should have 4 stages");
    assert_eq!(sr.input_signal, "a", "input signal should be 'a'");
}

#[test]
fn test_one_cycle_shift_register() {
    let sr = expect_sr(&compile_src(&guard_src("g", "clk_en", 1)));
    assert_eq!(sr.delay_cycles, 1, "delay should be 1 cycle");
    assert_eq!(sr.stages.len(), 1, "should have exactly 1 stage");
}

#[test]
fn test_threshold_boundary_16_uses_shift_register() {
    let sr = expect_sr(&compile_src(&guard_src("g", "x", 16)));
    assert_eq!(sr.delay_cycles, 16, "16 cycles should use shift register");
    assert_eq!(sr.stages.len(), 16, "should have 16 stages");
}

// --- counter compilation (N > 16) ---

#[test]
fn test_simple_signal_counter() {
    let c = expect_counter(&compile_src(&guard_src("g", "a", 100)));
    assert_eq!(c.name, "g", "guard name should be 'g'");
    assert_eq!(c.target_count, 100, "target count should be 100");
    assert_eq!(c.input_signal, "a", "input signal should be 'a'");
}

#[test]
fn test_threshold_boundary_17_uses_counter() {
    let c = expect_counter(&compile_src(&guard_src("g", "x", 17)));
    assert_eq!(c.target_count, 17, "17 cycles should use counter strategy");
}

#[test]
fn test_counter_1000_cycles_and_width() {
    let c = expect_counter(&compile_src(&guard_src("g", "p", 1000)));
    assert_eq!(c.target_count, 1000, "target should be 1000");
    assert_eq!(c.counter_width(), 11, "ceil(log2(1000))+1 = 11 bits");
}

// --- condition lowering: all six comparison operators ---

#[test]
fn test_comparison_operators_all_six() {
    let cases: [(BinaryOp, &str, u64); 6] = [
        (BinaryOp::Lt, "sensor < 50", 50),
        (BinaryOp::Gt, "temp > 200", 200),
        (BinaryOp::Eq, "status == 42", 42),
        (BinaryOp::Ne, "flags != 0", 0),
        (BinaryOp::Le, "level <= 100", 100),
        (BinaryOp::Ge, "count >= 5", 5),
    ];
    for i in 0..cases.len().min(MAX_TEST_GUARDS) {
        let (expected_op, cond, expected_val) = &cases[i];
        let sr = expect_sr(&compile_src(&guard_src("g", cond, 5)));
        if let ConditionKind::Comparison { op, value, .. } = &sr.condition_kind {
            assert_eq!(op, expected_op, "op mismatch for '{}'", cond);
            assert_eq!(value, &LiteralValue::Integer(*expected_val), "val mismatch for '{}'", cond);
        } else {
            panic!("expected Comparison for '{}', got {:?}", cond, sr.condition_kind);
        }
    }
}

// --- condition lowering: negated and simple signals ---

#[test]
fn test_negated_signal_guard() {
    let sr = expect_sr(&compile_src(&guard_src("g", "!reset", 4)));
    assert_eq!(
        sr.condition_kind,
        ConditionKind::NegatedSignal("reset".into()),
        "should lower to NegatedSignal"
    );
    assert_eq!(sr.input_signal, "reset", "input signal should be 'reset'");
}

#[test]
fn test_negated_signal_direct_construction() {
    let guard = Guard {
        name: "ng".into(),
        condition: Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Signal("rst_n".into())),
        },
        cycles: 8,
        origin: None,
        span: None,
    };
    let sr = expect_sr(&compile_guards(vec![guard]));
    assert_eq!(
        sr.condition_kind,
        ConditionKind::NegatedSignal("rst_n".into()),
        "negated signal should produce NegatedSignal"
    );
}

// --- multiple guards and empty modules ---

#[test]
fn test_multiple_guards_in_module() {
    let src = "module t {\n    guard sg {\n        when a\n        for 4 cycles;\n    }\n    guard lg {\n        when b\n        for 100 cycles;\n    }\n}\n";
    let nl = compile_src(src);
    assert_eq!(nl.guards.len(), 2, "should have two guards");
    assert!(nl.guards.len() <= MAX_TEST_GUARDS, "guard count within bound");
    assert!(
        matches!(&nl.guards[0], CompiledGuard::ShiftRegister(sr) if sr.name == "sg"),
        "first guard should be SR"
    );
    assert!(
        matches!(&nl.guards[1], CompiledGuard::Counter(c) if c.name == "lg"),
        "second guard should be counter"
    );
}

#[test]
fn test_empty_module_no_guards() {
    let nl = compile_src("module empty {\n}\n");
    assert_eq!(nl.guards.len(), 0, "empty module should have no guards");
    assert_eq!(nl.signals.len(), 0, "empty module should have no signals");
}

// --- AND combination (complex guard) ---

#[test]
fn test_and_combination_produces_complex_guard() {
    let nl = compile_src(&guard_src("g", "a && b", 4));
    assert_eq!(nl.guards.len(), 1, "should have one guard");
    match &nl.guards[0] {
        CompiledGuard::Complex(cx) => {
            assert_eq!(cx.name, "g", "complex guard name should be 'g'");
            assert_eq!(cx.sub_guards.len(), 2, "AND should produce two sub-guards");
            assert!(cx.output_signal.ends_with("_out"), "output should end with '_out'");
        }
        other => panic!("expected Complex guard for AND, got {:?}", other),
    }
}

// --- guard naming conventions ---

#[test]
fn test_shift_register_stage_naming() {
    let sr = expect_sr(&compile_guards(vec![sig_guard("d4", "input", 4)]));
    let bound = sr.stages.len().min(MAX_TEST_STAGES);
    for i in 0..bound {
        assert_eq!(sr.stages[i], format!("d4_sr_{}", i), "stage {} naming", i);
    }
    assert_eq!(sr.output_signal, "d4_out", "output signal should be <name>_out");
}

#[test]
fn test_counter_signal_naming() {
    let c = expect_counter(&compile_guards(vec![sig_guard("ld", "input", 100)]));
    assert_eq!(c.output_signal, "ld_out", "counter output naming");
    assert_eq!(c.counter_signal, "ld_counter", "counter register naming");
    assert_eq!(c.comparator_signal, "ld_cmp", "comparator naming");
}

// --- condition kind stored in compiled IR ---

#[test]
fn test_condition_kind_stored_in_sr_ir() {
    let sr = expect_sr(&compile_guards(vec![sig_guard("g", "clk_en", 4)]));
    assert_eq!(
        sr.condition_kind,
        ConditionKind::SimpleSignal("clk_en".into()),
        "SR IR should carry SimpleSignal condition kind"
    );
}

#[test]
fn test_condition_kind_stored_in_counter_ir() {
    let guard = Guard {
        name: "g".into(),
        condition: Expr::Binary {
            op: BinaryOp::Lt,
            left: Box::new(Expr::Signal("pressure".into())),
            right: Box::new(Expr::Literal(LiteralValue::Integer(50))),
        },
        cycles: 100,
        origin: None,
        span: None,
    };
    let c = expect_counter(&compile_guards(vec![guard]));
    assert_eq!(
        c.condition_kind,
        ConditionKind::Comparison {
            signal: "pressure".into(),
            op: BinaryOp::Lt,
            value: LiteralValue::Integer(50),
        },
        "counter IR should carry Comparison condition kind"
    );
}

// --- counter width calculations ---

#[test]
fn test_counter_width_calculations() {
    let c17 = expect_counter(&compile_guards(vec![sig_guard("g", "x", 17)]));
    assert_eq!(c17.counter_width(), 6, "width for 17: ceil(log2(17))+1 = 6");
    let c100 = expect_counter(&compile_guards(vec![sig_guard("g", "x", 100)]));
    assert_eq!(c100.counter_width(), 8, "width for 100: ceil(log2(100))+1 = 8");
}

// --- edge cases ---

#[test]
fn test_zero_cycle_guard() {
    let sr = expect_sr(&compile_guards(vec![sig_guard("g", "x", 0)]));
    assert_eq!(sr.delay_cycles, 0, "delay should be 0");
    assert_eq!(sr.stages.len(), 0, "should have 0 stages for 0 cycles");
}

// --- error paths ---

#[test]
fn test_unsupported_condition_literal() {
    let program = parse_mirr(&guard_src("g", "true", 4)).expect("parse should succeed");
    let mut compiler = TemporalGuardCompiler::new();
    assert!(
        compiler.compile_temporal_guards(&program.module).is_err(),
        "literal condition should be rejected"
    );
}

#[test]
fn test_unsupported_condition_arithmetic() {
    let program = parse_mirr(&guard_src("g", "a + b", 4)).expect("parse should succeed");
    let mut compiler = TemporalGuardCompiler::new();
    assert!(
        compiler.compile_temporal_guards(&program.module).is_err(),
        "arithmetic condition (Add) should be rejected"
    );
}

#[test]
fn test_unsupported_condition_prev() {
    let guard = Guard {
        name: "g".into(),
        condition: Expr::Prev { signal: "s".into(), delay: 1 },
        cycles: 4,
        origin: None,
        span: None,
    };
    let module = Module {
        name: "t".into(),
        signals: Vec::new(),
        guards: vec![guard],
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    let mut compiler = TemporalGuardCompiler::new();
    assert!(
        compiler.compile_temporal_guards(&module).is_err(),
        "Prev condition should be rejected"
    );
}

#[test]
fn test_unsupported_negation_of_non_signal() {
    let guard = Guard {
        name: "g".into(),
        condition: Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Literal(LiteralValue::Bool(true))),
        },
        cycles: 4,
        origin: None,
        span: None,
    };
    let module = Module {
        name: "t".into(),
        signals: Vec::new(),
        guards: vec![guard],
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    let mut compiler = TemporalGuardCompiler::new();
    assert!(
        compiler.compile_temporal_guards(&module).is_err(),
        "negation of non-signal should be rejected"
    );
}

// --- netlist statistics ---

#[test]
fn test_netlist_statistics_shift_register() {
    let nl = compile_guards(vec![sig_guard("g", "a", 4)]);
    assert_eq!(nl.statistics.shift_registers_used, 4, "4 SR stages");
    assert_eq!(nl.statistics.counters_used, 0, "no counters");
    assert!(nl.statistics.logic_gates_used >= 1, "at least 1 logic gate");
    assert_eq!(nl.statistics.max_delay_cycles, 4, "max delay 4");
    assert!(nl.signals.len() <= MAX_TEST_SIGNALS, "signals within bound");
    assert_eq!(
        nl.statistics.total_signals,
        nl.signals.len() as u32,
        "total_signals should match actual count"
    );
}

#[test]
fn test_netlist_statistics_counter() {
    let nl = compile_guards(vec![sig_guard("g", "a", 100)]);
    assert_eq!(nl.statistics.shift_registers_used, 0, "no SR stages");
    assert_eq!(nl.statistics.counters_used, 1, "1 counter");
    assert_eq!(nl.statistics.max_delay_cycles, 100, "max delay 100");
}

#[test]
fn test_netlist_statistics_mixed_guards() {
    let nl = compile_guards(vec![sig_guard("short", "a", 4), sig_guard("long", "b", 100)]);
    assert_eq!(nl.guards.len(), 2, "two compiled guards");
    assert_eq!(nl.statistics.shift_registers_used, 4, "4 SR stages from short");
    assert_eq!(nl.statistics.counters_used, 1, "1 counter from long");
    assert_eq!(nl.statistics.max_delay_cycles, 100, "max delay 100");
}

// --- netlist summary ---

#[test]
fn test_netlist_summary_format() {
    let nl = compile_guards(vec![sig_guard("g", "a", 4)]);
    let summary = nl.summary();
    assert!(summary.contains("Guards: 1"), "summary should show guard count");
    assert!(summary.contains("Max Delay: 4"), "summary should show max delay");
}

// --- generated signal kinds ---

#[test]
fn test_generated_signal_kinds_shift_register() {
    let nl = compile_guards(vec![sig_guard("g", "a", 4)]);
    let mut sr_count = 0u32;
    let mut gate_count = 0u32;
    for i in 0..nl.signals.len().min(MAX_TEST_SIGNALS) {
        match nl.signals[i].kind {
            GeneratedSignalKind::ShiftRegisterStage => sr_count += 1,
            GeneratedSignalKind::LogicGate => gate_count += 1,
            _ => {}
        }
    }
    assert_eq!(sr_count, 4, "4 SR stage signals");
    assert!(gate_count >= 1, "at least 1 logic gate signal");
}

#[test]
fn test_generated_signal_kinds_counter() {
    let nl = compile_guards(vec![sig_guard("g", "a", 100)]);
    let (mut cnt, mut cmp) = (0u32, 0u32);
    for i in 0..nl.signals.len().min(MAX_TEST_SIGNALS) {
        match nl.signals[i].kind {
            GeneratedSignalKind::Counter => cnt += 1,
            GeneratedSignalKind::Comparator => cmp += 1,
            _ => {}
        }
    }
    assert_eq!(cnt, 1, "1 counter signal");
    assert_eq!(cmp, 1, "1 comparator signal");
}

// --- resource estimator and strategy selection ---

#[test]
fn test_resource_estimator() {
    let sr = ResourceEstimator::estimate_shift_register_resources(4);
    assert_eq!(sr.shift_registers, 4, "4 SRs for 4 cycles");
    assert_eq!(sr.counters, 0, "no counters");
    assert_eq!(sr.logic_gates, 1, "1 logic gate");
    assert_eq!(sr.total_signals, 5, "4 stages + 1 output");
    let ctr = ResourceEstimator::estimate_counter_resources(100);
    assert_eq!(ctr.shift_registers, 0, "no SRs");
    assert_eq!(ctr.counters, 1, "1 counter");
    assert_eq!(ctr.logic_gates, 2, "2 logic gates");
}

#[test]
fn test_strategy_selection_at_boundaries() {
    match ResourceEstimator::choose_optimal_strategy(16) {
        ImplementationStrategy::ShiftRegister(_) => {}
        other => panic!("expected ShiftRegister for N=16, got {:?}", other),
    }
    match ResourceEstimator::choose_optimal_strategy(17) {
        ImplementationStrategy::Counter(_) => {}
        other => panic!("expected Counter for N=17, got {:?}", other),
    }
}

// --- ConditionKind helpers ---

#[test]
fn test_condition_kind_describe_and_primary_signal() {
    let s = ConditionKind::SimpleSignal("clk".into());
    assert_eq!(s.describe(), "when clk (high)", "SimpleSignal describe");
    assert_eq!(s.primary_signal(), "clk", "SimpleSignal primary");
    let n = ConditionKind::NegatedSignal("reset".into());
    assert_eq!(n.describe(), "when !reset (low)", "NegatedSignal describe");
    assert_eq!(n.primary_signal(), "reset", "NegatedSignal primary");
    let c = ConditionKind::Comparison {
        signal: "p".into(),
        op: BinaryOp::Lt,
        value: LiteralValue::Integer(50),
    };
    assert_eq!(c.describe(), "when p < 50", "Comparison describe");
    assert_eq!(c.primary_signal(), "p", "Comparison primary");
}

#[test]
fn test_condition_kind_try_from_expr() {
    let sig = ConditionKind::try_from_expr(&Expr::Signal("s".into()));
    assert!(sig.is_ok(), "Signal should be accepted");
    assert_eq!(sig.unwrap(), ConditionKind::SimpleSignal("s".into()), "should be SimpleSignal");

    let neg = ConditionKind::try_from_expr(&Expr::Unary {
        op: UnaryOp::Not,
        operand: Box::new(Expr::Signal("en".into())),
    });
    assert!(neg.is_ok(), "Negated signal should be accepted");
    assert_eq!(neg.unwrap(), ConditionKind::NegatedSignal("en".into()), "should be NegatedSignal");

    let and_expr = Expr::Binary {
        op: BinaryOp::And,
        left: Box::new(Expr::Signal("a".into())),
        right: Box::new(Expr::Signal("b".into())),
    };
    assert!(ConditionKind::try_from_expr(&and_expr).is_err(), "AND should be rejected");
}
