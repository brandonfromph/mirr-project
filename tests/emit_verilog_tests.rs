//! SystemVerilog emitter tests.
//!
//! Covers internal signals, multi-guard reflex OR join, Prev as _d delay,
//! complex guard assign block, shift-register logic format, counter logic,
//! condition types (SimpleSignal, Negated, Comparison), and temporal/emit.rs
//! emit_verilog (first coverage ever).

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::emit;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig, PipelineResult};

// Also test temporal::emit::emit_verilog directly.
use nasa_rust_project::temporal::emit as temporal_emit;
use nasa_rust_project::temporal::low_level_ir::{
    CompiledGuard, ComplexGuard, ConditionKind, CounterGuard, GeneratedSignal, ShiftRegisterGuard,
    TemporalNetlist,
};

// ---------------------------------------------------------------------------
// MIRR fixtures
// ---------------------------------------------------------------------------

const INTERNAL_SIGNAL_MIRR: &str = r#"
module with_internal {
    signal a: in u8;
    signal b: out u8;
    signal buf: internal u8;

    guard g {
        when a > 10
        for 5 cycles;
    }

    reflex r {
        on g {
            buf = a;
            b = buf;
        }
    }
}
"#;

const MULTI_GUARD_MIRR: &str = r#"
module multi_guard {
    signal x: in bool;
    signal y: in bool;
    signal out: out bool;

    guard g1 {
        when x
        for 2 cycles;
    }

    guard g2 {
        when y
        for 3 cycles;
    }

    reflex both {
        on g1 and g2 {
            out = true;
        }
    }
}
"#;

// ---------------------------------------------------------------------------
// Internal signals
// ---------------------------------------------------------------------------

#[test]
fn sv_internal_signal_declared() {
    let config = PipelineConfig::default();
    let result = run_pipeline(INTERNAL_SIGNAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("// Internal signals"), "should have internal signals section");
    assert!(sv.contains("buf"), "should declare buf signal");
}

#[test]
fn sv_internal_signal_not_in_port_list() {
    let config = PipelineConfig::default();
    let result = run_pipeline(INTERNAL_SIGNAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    // Find the module declaration (between "module" and ");")
    let decl_start = sv.find("module with_internal (").unwrap();
    let decl_end = sv[decl_start..].find(");").unwrap() + decl_start;
    let decl = &sv[decl_start..decl_end];
    assert!(!decl.contains("buf"), "internal signal should not appear in port list");
}

// ---------------------------------------------------------------------------
// Multi-guard reflex OR join
// ---------------------------------------------------------------------------

#[test]
fn sv_multi_guard_reflex_uses_or() {
    let config = PipelineConfig::default();
    let result = run_pipeline(MULTI_GUARD_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("g1_out || g2_out"), "should OR-join guard outputs");
}

// ---------------------------------------------------------------------------
// Prev in reflex RHS maps to _d{delay} (programmatic AST — parser has no prev())
// ---------------------------------------------------------------------------

/// Build a PipelineResult with Prev in reflex assignment.
fn prev_in_reflex_result() -> PipelineResult {
    let module = Module {
        name: "with_prev".to_string(),
        signals: vec![
            SignalDecl {
                name: "sensor".to_string(),
                kind: SignalKind::Input,
                ty: SignalType::Unsigned(16),
            },
            SignalDecl {
                name: "delta".to_string(),
                kind: SignalKind::Output,
                ty: SignalType::Unsigned(16),
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("sensor".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
        }],
        reflexes: vec![Reflex {
            name: "compute".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "delta".to_string(),
                value: Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::Signal("sensor".to_string())),
                    right: Box::new(Expr::Prev { signal: "sensor".to_string(), delay: 1 }),
                },
            }],
        }],
    };

    PipelineResult {
        program: MirrProgram { module },
        simplify_stats: None,
        width_result: None,
        temporal_netlist: None,
    }
}

#[test]
fn sv_prev_renders_as_delayed_signal() {
    let result = prev_in_reflex_result();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("sensor_d1"), "prev(sensor, 1) should render as sensor_d1");
}

// ---------------------------------------------------------------------------
// Condition expression rendering
// ---------------------------------------------------------------------------

#[test]
fn sv_negated_signal_condition_renders_correctly() {
    // Build a netlist with a negated signal guard.
    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::NegatedSignal("reset".to_string());
    let sr = ShiftRegisterGuard::new("neg_guard".to_string(), "reset".to_string(), 4, ck);
    netlist.add_guard(CompiledGuard::ShiftRegister(sr));

    // Use temporal emit to check the verilog output.
    let v = temporal_emit::emit_verilog(&netlist).unwrap();
    assert!(v.contains("ShiftRegister guard 'neg_guard'"), "guard comment should exist");
}

#[test]
fn sv_comparison_condition_in_temporal_verilog() {
    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::Comparison {
        signal: "pressure".to_string(),
        op: nasa_rust_project::ast::types::BinaryOp::Lt,
        value: nasa_rust_project::ast::types::LiteralValue::Integer(50),
    };
    let cg = CounterGuard::new("pressure_drop".to_string(), "pressure".to_string(), 1000, ck);
    netlist.add_signal(GeneratedSignal::counter(cg.counter_signal.clone(), cg.counter_width()));
    netlist.add_signal(GeneratedSignal::comparator(cg.comparator_signal.clone()));
    netlist.add_guard(CompiledGuard::Counter(cg));

    let v = temporal_emit::emit_verilog(&netlist).unwrap();
    assert!(v.contains("Counter guard 'pressure_drop' count=1000"));
}

// ---------------------------------------------------------------------------
// temporal::emit::emit_verilog — first coverage ever
// ---------------------------------------------------------------------------

#[test]
fn temporal_emit_verilog_produces_module() {
    let netlist = TemporalNetlist::new();
    let v = temporal_emit::emit_verilog(&netlist).unwrap();

    assert!(v.contains("module mirr_temporal_netlist();"));
    assert!(v.contains("endmodule"));
}

#[test]
fn temporal_emit_verilog_shift_register_guard() {
    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::SimpleSignal("clk_en".to_string());
    let sr = ShiftRegisterGuard::new("sr_g".to_string(), "clk_en".to_string(), 4, ck);
    for (i, stage_name) in sr.stages.iter().enumerate() {
        netlist.add_signal(GeneratedSignal::shift_register_stage(stage_name.clone(), i as u32));
    }
    netlist.add_guard(CompiledGuard::ShiftRegister(sr));

    let v = temporal_emit::emit_verilog(&netlist).unwrap();
    assert!(v.contains("wire sr_g_sr_0;// Bool"));
    assert!(v.contains("ShiftRegister guard 'sr_g' delay=4"));
}

#[test]
fn temporal_emit_verilog_counter_guard() {
    let mut netlist = TemporalNetlist::new();
    let ck = ConditionKind::SimpleSignal("enable".to_string());
    let cg = CounterGuard::new("ctr_g".to_string(), "enable".to_string(), 100, ck);
    netlist.add_signal(GeneratedSignal::counter(cg.counter_signal.clone(), cg.counter_width()));
    netlist.add_signal(GeneratedSignal::comparator(cg.comparator_signal.clone()));
    netlist.add_guard(CompiledGuard::Counter(cg));

    let v = temporal_emit::emit_verilog(&netlist).unwrap();
    assert!(v.contains("Counter guard 'ctr_g' count=100"));
    // Counter width is 8 for count=100, so wire [7:0]
    assert!(v.contains("[7:0]"));
}

#[test]
fn temporal_emit_verilog_complex_guard() {
    let mut netlist = TemporalNetlist::new();
    let complex = ComplexGuard::new(
        "combo".to_string(),
        vec![],
        nasa_rust_project::ast::Expr::Signal("dummy".to_string()),
    );
    netlist.add_guard(CompiledGuard::Complex(complex));

    let v = temporal_emit::emit_verilog(&netlist).unwrap();
    assert!(v.contains("Complex guard 'combo'"));
}

#[test]
fn temporal_emit_verilog_unsigned_zero_width() {
    let mut netlist = TemporalNetlist::new();
    netlist.add_signal(GeneratedSignal {
        name: "zero_w".to_string(),
        ty: nasa_rust_project::ast::types::SignalType::Unsigned(0),
        kind: nasa_rust_project::temporal::low_level_ir::GeneratedSignalKind::Intermediate,
        source: None,
    });

    let v = temporal_emit::emit_verilog(&netlist).unwrap();
    assert!(v.contains("wire zero_w;// Unsigned(0)"));
}

// ---------------------------------------------------------------------------
// sv_type width cases
// ---------------------------------------------------------------------------

#[test]
fn sv_u1_signal_renders_as_logic() {
    // u1 should render as "logic" same as bool, not "logic [0:0]"
    let source = r#"
module u1_test {
    signal flag: in u1;
    signal out: out bool;

    guard g {
        when flag > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            out = true;
        }
    }
}
"#;
    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    // Should not contain [0:0] for u1
    let decl_start = sv.find("module u1_test").unwrap();
    let decl_end = sv[decl_start..].find(");").unwrap() + decl_start;
    let decl = &sv[decl_start..decl_end];
    assert!(!decl.contains("[0:0]"), "u1 should render as logic, not logic [0:0]");
}
