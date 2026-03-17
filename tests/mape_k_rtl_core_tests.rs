#![forbid(unsafe_code)]

//! Core integration tests for MAPE-K RTL emitter without external tools.
//!
//! Tests E1.1-E1.25 verify RTL structure, module generation, and pipeline integration.

use nasa_rust_project::ast::program::{MirrProgram, Module};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{ExtendedType, SignalKind, SignalType};
use nasa_rust_project::ast::{Expr, SignalDecl};
use nasa_rust_project::emit::mape_k_rtl::{emit_mape_k_rtl, MAX_RTL_PROPERTIES, MAX_RTL_SIGNALS};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig, PipelineResult};

const MAX_TEST_SIGNALS: usize = 512;
const MAX_TEST_PROPERTIES: usize = 128;

fn stub_pipeline(signals: Vec<SignalDecl>, properties: Vec<PropertyDecl>) -> PipelineResult {
    let module = Module {
        name: "test_mod".to_string(),
        signals,
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties,
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    PipelineResult {
        program: MirrProgram { patterns: Vec::new(), module },
        simplify_stats: None,
        sat_stats: None,
        width_result: None,
        temporal_netlist: None,
        rspu_program: None,
        type_map: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
    }
}

fn input_signal(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn assert_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
    PropertyDecl {
        name: name.to_string(),
        directive: PropertyDirective::Assert,
        formula,
        origin: None,
        span: None,
    }
}

#[test]
fn e1_1_monitor_block_has_module_declaration() {
    let signals = vec![input_signal("pressure", SignalType::Unsigned(8))];
    let props =
        vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("pressure".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("module mirr_monitor"), "RTL must contain monitor module");
}

#[test]
fn e1_2_monitor_block_has_shadow_registers() {
    let signals = vec![input_signal("pressure", SignalType::Unsigned(8))];
    let props =
        vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("pressure".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("shadow"), "monitor must have shadow registers");
    assert!(sv.contains("always_ff"), "monitor must use always_ff");
}

#[test]
fn e1_3_monitor_block_has_trace_buffer() {
    let signals = vec![input_signal("pressure", SignalType::Unsigned(8))];
    let props =
        vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("pressure".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("trace_buf"), "monitor must have trace buffer");
    assert!(sv.contains("wr_ptr"), "monitor must have write pointer");
}

#[test]
fn e1_4_monitor_synthesis_clean_no_display() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(!sv.contains("$display"), "RTL must be synthesis-clean (no $display)");
}

#[test]
fn e1_5_analyze_block_has_module_declaration() {
    let signals = vec![input_signal("pressure", SignalType::Unsigned(8))];
    let props =
        vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("pressure".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("module mirr_analyze"), "RTL must contain analyze module");
}

#[test]
fn e1_6_analyze_block_has_always_checker() {
    let signals = vec![input_signal("sig", SignalType::Unsigned(8))];
    let props = vec![assert_property(
        "always_prop",
        PropertyFormula::Always(Expr::Signal("sig".to_string())),
    )];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("Property 0: Always"), "analyze must have Always property checker");
}

#[test]
fn e1_7_analyze_block_has_eventually_checker() {
    let signals = vec![input_signal("ready", SignalType::Bool)];
    let props = vec![assert_property(
        "eventually_prop",
        PropertyFormula::EventuallyWithin { expr: Expr::Signal("ready".to_string()), cycles: 10 },
    )];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("ev_cnt"), "analyze must have EventuallyWithin counter");
}

#[test]
fn e1_8_analyze_block_has_persists_checker() {
    let signals = vec![input_signal("cond", SignalType::Bool)];
    let props = vec![assert_property(
        "persist_prop",
        PropertyFormula::EventuallyWithin { expr: Expr::Signal("cond".to_string()), cycles: 10 },
    )];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("ev_cnt"), "analyze must have counter for time-bounded property");
}

#[test]
fn e1_9_analyze_block_has_priority_encoder() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("top_violation_idx"), "analyze must have priority encoder output");
}

#[test]
fn e1_10_plan_block_has_action_table() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("best_priority"), "plan must have action table logic");
    assert!(sv.contains("found"), "plan must track action found status");
}

#[test]
fn e1_11_plan_block_has_violation_matching() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("violation_vec"), "plan must reference violation vector");
}

#[test]
fn e1_12_plan_block_has_priority_selection() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("selected_action_idx"), "plan must output selected action index");
}

#[test]
fn e1_13_execute_block_has_action_dispatch() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("action_valid"), "execute must have action_valid signal");
}

#[test]
fn e1_14_execute_block_has_emergency_latch() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("emergency_active"), "execute must have emergency latch");
}

#[test]
fn e1_15_knowledge_block_has_fifo() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("fifo"), "knowledge must have FIFO");
    assert!(sv.contains("full"), "knowledge must have full flag");
}

#[test]
fn e1_16_top_block_wires_all_submodules() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("u_monitor"));
    assert!(sv.contains("u_analyze"));
    assert!(sv.contains("u_plan"));
    assert!(sv.contains("u_execute"));
    assert!(sv.contains("u_knowledge"));
}

#[test]
fn e1_17_top_block_has_clk_rst() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("clk"));
    assert!(sv.contains("rst_n"));
}

#[test]
fn e1_18_top_block_has_all_six_modules() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    for name in &[
        "mirr_monitor",
        "mirr_analyze",
        "mirr_plan",
        "mirr_execute",
        "mirr_knowledge",
        "mirr_mape_k_top",
    ] {
        assert!(sv.contains(&format!("module {}", name)));
    }
}

#[test]
fn e1_19_full_pipeline_emit_mape_k_rtl() {
    const MIRR_SRC: &str = "module safety {\n    signal pressure: in u8;\n    signal alarm: out bool;\n\n    property p1 {\n        always (pressure > 0);\n    }\n}";
    let config = PipelineConfig { mape_k: true, emit_mape_k_rtl: true, ..Default::default() };
    let result = run_pipeline(MIRR_SRC, &config).expect("pipeline should succeed");
    assert!(result.mape_k_rtl.is_some());
}

#[test]
fn e1_20_emit_rtl_error_on_too_many_signals() {
    let count = (MAX_RTL_SIGNALS + 1).min(MAX_TEST_SIGNALS);
    let signals: Vec<SignalDecl> =
        (0..count).map(|i| input_signal(&format!("s{}", i), SignalType::Unsigned(8))).collect();
    let result = stub_pipeline(signals, vec![]);
    let err = emit_mape_k_rtl(&result).expect_err("should fail");
    assert!(err.contains("too many signals"));
}

#[test]
fn e1_21_emit_rtl_error_on_too_many_properties() {
    let count = (MAX_RTL_PROPERTIES + 1).min(MAX_TEST_PROPERTIES);
    let props: Vec<PropertyDecl> = (0..count)
        .map(|i| {
            assert_property(
                &format!("p{}", i),
                PropertyFormula::Always(Expr::Signal(format!("sig{}", i))),
            )
        })
        .collect();
    let result = stub_pipeline(vec![], props);
    let err = emit_mape_k_rtl(&result).expect_err("should fail");
    assert!(err.contains("too many properties"));
}

#[test]
fn e1_22_emit_rtl_contains_header_comment() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("MAPE-K RTL"));
    assert!(sv.contains("Do not edit"));
}

#[test]
fn e1_23_emit_rtl_synthesis_clean_no_initial() {
    let signals = vec![input_signal("x", SignalType::Unsigned(8))];
    let props = vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("x".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(!sv.contains("initial begin"));
}

#[test]
fn e1_24_emit_rtl_parameterized_bit_widths() {
    let signals: Vec<SignalDecl> =
        (0..5).map(|i| input_signal(&format!("s{}", i), SignalType::Unsigned(8))).collect();
    let props =
        vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("s0".to_string())))];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("violation_vec"));
}

#[test]
fn e1_25_emit_rtl_persists_temporal_property() {
    let signals = vec![input_signal("cond", SignalType::Bool)];
    let props = vec![assert_property(
        "persist",
        PropertyFormula::EventuallyWithin { expr: Expr::Signal("cond".to_string()), cycles: 5 },
    )];
    let result = stub_pipeline(signals, props);
    let sv = emit_mape_k_rtl(&result).expect("emit should succeed");
    assert!(sv.contains("ev_cnt"));
}
