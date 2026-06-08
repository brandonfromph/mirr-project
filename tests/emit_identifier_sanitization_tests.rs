#![forbid(unsafe_code)]
//! DOT identifier sanitization tests.

use mirrc::ast::program::{MirrProgram, Module, SignalDecl};
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::emit;
use mirrc::pipeline::PipelineResult;

fn result_with_module_name(module_name: &str) -> PipelineResult {
    let module = Module {
        name: module_name.to_string(),
        signals: vec![SignalDecl {
            name: "sig-name.with spaces".to_string(),
            kind: SignalKind::Internal,
            ty: ExtendedType::from_core(SignalType::Unsigned(8)),
            origin: None,
            span: None,
        }],
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };

    PipelineResult {
        hls_result: None,
        program: MirrProgram { patterns: Vec::new(), imports: Vec::new(), module },
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

#[test]
fn dot_output_sanitizes_non_identifier_characters() {
    let result = result_with_module_name("mod name/with punctuation");
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.contains("digraph mod_name_with_punctuation"), "module id should be sanitized");
    assert!(dot.contains("sig_name_with_spaces"), "signal id should be sanitized");
}

#[test]
fn dot_output_disambiguates_colliding_sanitized_names() {
    let dot_a = emit::dot::emit_module_dot(&result_with_module_name("a-b"));
    let dot_b = emit::dot::emit_module_dot(&result_with_module_name("a/b"));

    let header_a = dot_a.lines().next().expect("dot output should have a header");
    let header_b = dot_b.lines().next().expect("dot output should have a header");

    assert!(
        header_a.starts_with("digraph a_b_"),
        "first header should be disambiguated: {header_a}"
    );
    assert!(
        header_b.starts_with("digraph a_b_"),
        "second header should be disambiguated: {header_b}"
    );
    assert_ne!(header_a, header_b, "colliding sanitized names must not share the same DOT id");
}
