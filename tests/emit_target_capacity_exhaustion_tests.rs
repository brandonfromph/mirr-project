#![forbid(unsafe_code)]
//! Emitter capacity exhaustion boundary tests.

use mirrc::ast::program::MirrProgram;
use mirrc::ast::types::{LiteralValue, SignalKind};
use mirrc::pipeline::PipelineResult;
use mirrc::temporal::low_level_ir::{
    CompiledGuard, ConditionKind, ShiftRegisterGuard, TemporalNetlist,
};

const GUARD_COUNT: usize = 4200;
const DOT_CAPACITY: usize = 4096;

fn result_with_many_temporal_guards() -> PipelineResult {
    let mut netlist = TemporalNetlist::new();
    let mut i = 0usize;
    while i < GUARD_COUNT {
        let name = format!("g{i}");
        let output_signal = format!("out_sig_{i}");
        let sr = ShiftRegisterGuard::new(
            name.clone(),
            output_signal,
            2,
            ConditionKind::SimpleSignal("in_sig".to_string()),
        );
        netlist.add_guard(CompiledGuard::ShiftRegister(sr));
        i += 1;
    }

    PipelineResult {
        hls_result: None,
        program: MirrProgram {
            target: None,
            patterns: Vec::new(),
            imports: Vec::new(),
            module: mirrc::ast::program::Module {
                name: "emit_capacity".to_string(),
                signals: vec![mirrc::ast::program::SignalDecl {
                    name: "in_sig".to_string(),
                    kind: SignalKind::Input,
                    ty: mirrc::ast::types::ExtendedType::from_core(
                        mirrc::ast::types::SignalType::Bool,
                    ),
                    origin: None,
                    span: None,
                }],
                guards: Vec::new(),
                reflexes: Vec::new(),
                properties: Vec::new(),
                pattern_calls: Vec::new(),
                pattern_origins: Vec::new(),
                span: None,
            },
        },
        simplify_stats: None,
        sat_stats: None,
        width_result: None,
        temporal_netlist: Some(netlist),
        rspu_program: None,
        type_map: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
        file_table: mirrc::span::FileTable::new(),
    }
}

#[test]
fn dot_temporal_subgraph_caps_emitted_capacity() {
    let result = result_with_many_temporal_guards();
    let dot = mirrc::emit::dot::emit_module_dot(&result);
    let sr_nodes = dot.matches("SR:").count();

    assert_eq!(sr_nodes, DOT_CAPACITY, "DOT temporal subgraph should stop at the configured cap");
    assert!(dot.contains("g4095_out"), "last in-capacity temporal node should be emitted");
    assert!(!dot.contains("g4096_out"), "first over-capacity temporal node should be omitted");
}

#[test]
fn temporal_emit_still_produces_valid_output_under_load() {
    let mut netlist = TemporalNetlist::new();
    let sr = ShiftRegisterGuard::new(
        "base".to_string(),
        "in_sig".to_string(),
        4,
        ConditionKind::Comparison {
            signal: "in_sig".to_string(),
            op: mirrc::ast::types::BinaryOp::Eq,
            value: LiteralValue::Bool(true),
        },
    );
    netlist.add_guard(CompiledGuard::ShiftRegister(sr));

    let verilog = mirrc::temporal::emit::emit_verilog(&netlist)
        .expect("temporal verilog emission should succeed");
    assert!(verilog.contains("module mirr_temporal_netlist"));
}
