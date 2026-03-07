//! Temporal Guard Emitter Tests
//!
//! Requirement coverage: P2-REQ-007, P2-REQ-008
//! Ref: MIRR-PHASE2-001 §6 (traceability table)

use nasa_rust_project::{parse_mirr, TemporalGuardCompiler};

// ---------------------------------------------------------------------------
// P2-REQ-007: JSON emission includes guard names and statistics fields
// ---------------------------------------------------------------------------
#[test]
fn test_json_emission_structure() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard test_guard {
        when input_signal
        for 4 cycles;
    }

    reflex test_reflex {
        on test_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let mut compiler = TemporalGuardCompiler::new();
    let netlist = compiler.compile_temporal_guards(&program.module).expect("compile failed");

    let json = compiler.emit_netlist_json(&netlist).expect("json emit failed");

    // Must contain the guard name
    assert!(json.contains("test_guard"), "JSON must include guard name");
    // Must contain SR variant tag
    assert!(json.contains("ShiftRegister"), "JSON must include ShiftRegister variant");
    // Must contain statistics fields
    assert!(json.contains("shift_registers_used"), "JSON missing shift_registers_used");
    assert!(json.contains("counters_used"), "JSON missing counters_used");
    assert!(json.contains("logic_gates_used"), "JSON missing logic_gates_used");
}

// ---------------------------------------------------------------------------
// P2-REQ-007 (counter path): JSON for counter guard includes correct variant
// ---------------------------------------------------------------------------
#[test]
fn test_json_counter_variant() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard long_guard {
        when input_signal
        for 100 cycles;
    }

    reflex test_reflex {
        on long_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let mut compiler = TemporalGuardCompiler::new();
    let netlist = compiler.compile_temporal_guards(&program.module).expect("compile failed");

    let json = compiler.emit_netlist_json(&netlist).expect("json emit failed");
    assert!(json.contains("Counter"), "JSON must include Counter variant");
    assert!(json.contains("long_guard"), "JSON must include guard name");
}

// ---------------------------------------------------------------------------
// P2-REQ-008: DOT output is syntactically well-formed
// ---------------------------------------------------------------------------
#[test]
fn test_dot_emission_structure() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard test_guard {
        when input_signal
        for 4 cycles;
    }

    reflex test_reflex {
        on test_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let mut compiler = TemporalGuardCompiler::new();
    let netlist = compiler.compile_temporal_guards(&program.module).expect("compile failed");

    let dot = compiler.emit_netlist_dot(&netlist).expect("dot emit failed");

    // DOT must open and close the digraph
    assert!(
        dot.starts_with("digraph TemporalNetlist {"),
        "DOT must start with digraph declaration"
    );
    assert!(dot.ends_with("}\n"), "DOT must end with closing brace");

    // Must contain the guard cluster
    assert!(dot.contains("cluster_test_guard"), "DOT must contain guard cluster");
    // Must reference the input signal
    assert!(dot.contains("input_signal"), "DOT must reference input signal");
}

// ---------------------------------------------------------------------------
// P2-REQ-008 (counter path): DOT for counter guard contains counter cluster
// ---------------------------------------------------------------------------
#[test]
fn test_dot_counter_cluster() {
    let src = r#"
module test_module {
    signal input_signal: in bool;
    signal output_signal: out bool;

    guard sustained_guard {
        when input_signal
        for 1000 cycles;
    }

    reflex test_reflex {
        on sustained_guard {
            output_signal = input_signal;
        }
    }
}
"#;
    let program = parse_mirr(src).expect("parse failed");
    let mut compiler = TemporalGuardCompiler::new();
    let netlist = compiler.compile_temporal_guards(&program.module).expect("compile failed");

    let dot = compiler.emit_netlist_dot(&netlist).expect("dot emit failed");
    assert!(dot.contains("cluster_sustained_guard"), "DOT must contain counter guard cluster");
    assert!(dot.contains("Counter:"), "DOT counter label must say 'Counter:'");
}
