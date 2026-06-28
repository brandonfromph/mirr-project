#![allow(clippy::field_reassign_with_default)]
#![cfg(any())]
#![forbid(unsafe_code)]
// ---------------------------------------------------------------------------
// FIRRTL Emission Tests
// ---------------------------------------------------------------------------
// Verifies the FIRRTL backend produces correct FIRRTL output for all MIRR
// constructs: module declarations, ports, wires, temporal guards, reflexes,
// and property annotations.
// ---------------------------------------------------------------------------

use mirrc::emit::firrtl::emit_firrtl;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

/// Helper: run pipeline and emit FIRRTL.
fn firrtl_from(src: &str) -> String {
    let config = PipelineConfig::default();
    let result = run_pipeline(src, &config).expect("pipeline should succeed");
    emit_firrtl(&result).expect("Failed to emit FIRRTL")
}

/// Helper: run pipeline with temporal disabled.
fn firrtl_no_temporal(src: &str) -> String {
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };
    let result = run_pipeline(src, &config).expect("pipeline should succeed");
    emit_firrtl(&result).expect("Failed to emit FIRRTL")
}

// ---------------------------------------------------------------------------
// Basic structure
// ---------------------------------------------------------------------------

#[test]
fn firrtl_emits_circuit_and_module() {
    let src = r#"
module test_mod {
    signal s: in bool;
    signal out_s: out bool;

    guard g {
        when s
        for 1 cycles;
    }

    reflex r {
        on g {
            out_s = s;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("FIRRTL version 1.1.0"), "missing FIRRTL version header");
    assert!(output.contains("circuit test_mod :"), "missing circuit declaration");
    assert!(output.contains("module test_mod :"), "missing module declaration");
}

#[test]
fn firrtl_emits_clock_and_reset() {
    let src = r#"
module clk_rst {
    signal x: in u8;
    signal y: out u8;

    guard g {
        when x > 10
        for 2 cycles;
    }

    reflex r {
        on g {
            y = x;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("input clk : Clock"), "missing clock port");
    assert!(output.contains("input rst_n : UInt<1>"), "missing reset port");
}

// ---------------------------------------------------------------------------
// Port declarations
// ---------------------------------------------------------------------------

#[test]
fn firrtl_emits_input_ports() {
    let src = r#"
module ports {
    signal sensor: in u16;
    signal enable: in bool;
    signal alarm: out bool;

    guard g {
        when sensor > 100
        for 1 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("input sensor : UInt<16>"), "missing u16 input port");
    assert!(output.contains("input enable : UInt<1>"), "missing bool input port");
    assert!(output.contains("output alarm : UInt<1>"), "missing bool output port");
}

#[test]
fn firrtl_emits_internal_wires() {
    let src = r#"
module wire_test {
    signal x: in bool;
    signal y: out bool;
    signal tmp: internal bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            tmp = x;
            y = tmp;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("wire tmp : UInt<1>"), "missing internal wire");
}

// ---------------------------------------------------------------------------
// Temporal guard hardware
// ---------------------------------------------------------------------------

#[test]
fn firrtl_emits_shift_register_guard() {
    let src = r#"
module sr_test {
    signal s: in bool;
    signal alarm: out bool;

    guard sustained {
        when s
        for 5 cycles;
    }

    reflex trigger {
        on sustained {
            alarm = true;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("Temporal Guards"), "missing temporal section");
    assert!(output.contains("sustained_sr"), "missing shift register");
    assert!(output.contains("sustained_cond"), "missing condition wire");
    assert!(output.contains("sustained_out"), "missing output wire");
}

#[test]
fn firrtl_emits_counter_guard() {
    let src = r#"
module ctr_test {
    signal pressure: in u16;
    signal valve: out bool;

    guard high_pressure {
        when pressure > 500
        for 100 cycles;
    }

    reflex close_valve {
        on high_pressure {
            valve = true;
        }
    }
}
"#;
    let output = firrtl_from(src);
    // Counter guards use a counter register instead of shift register.
    assert!(output.contains("counter") || output.contains("_sr"), "missing temporal hardware");
    assert!(output.contains("high_pressure_out"), "missing guard output");
}

// ---------------------------------------------------------------------------
// Reflex assignments
// ---------------------------------------------------------------------------

#[test]
fn firrtl_emits_reflex_when_blocks() {
    let src = r#"
module reflex_test {
    signal s: in bool;
    signal out1: out bool;
    signal out2: out u8;

    guard g {
        when s
        for 1 cycles;
    }

    reflex multi_assign {
        on g {
            out1 = true;
            out2 = 42;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("Reflex Assignments"), "missing reflex section");
    assert!(output.contains("when g_out"), "missing when block");
    assert!(output.contains("connect out1"), "missing out1 assignment");
    assert!(output.contains("connect out2"), "missing out2 assignment");
}

// ---------------------------------------------------------------------------
// Property annotations
// ---------------------------------------------------------------------------

#[test]
fn firrtl_emits_property_comments() {
    let src = r#"
module prop_test {
    signal temp: in u16;
    signal alarm: out bool;

    guard hot {
        when temp > 200
        for 10 cycles;
    }

    reflex r {
        on hot {
            alarm = true;
        }
    }

    property bounded {
        always (temp < 500);
    }

    property no_overheat {
        never (temp > 1000);
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("Safety Properties"), "missing properties section");
    assert!(output.contains("property bounded"), "missing always property");
    assert!(output.contains("property no_overheat"), "missing never property");
}

#[test]
fn firrtl_implies_property_comment() {
    let src = r#"
module implies_test {
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

    property implication {
        always (a -> b);
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("property implication"), "missing implies property");
    assert!(output.contains("->"), "missing implication arrow in comment");
}

// ---------------------------------------------------------------------------
// Expression mapping
// ---------------------------------------------------------------------------

#[test]
fn firrtl_maps_binary_ops_to_primops() {
    let src = r#"
module expr_test {
    signal a: in u8;
    signal b: in u8;
    signal out_val: out u8;

    guard g {
        when a > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            out_val = a + b;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("add("), "missing add primop");
}

// ---------------------------------------------------------------------------
// No temporal netlist (temporal stage disabled)
// ---------------------------------------------------------------------------

#[test]
fn firrtl_works_without_temporal() {
    let src = r#"
module no_temporal {
    signal x: in bool;
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
}
"#;
    let output = firrtl_no_temporal(src);
    assert!(output.contains("circuit no_temporal :"), "missing circuit");
    assert!(output.contains("module no_temporal :"), "missing module");
    // No temporal section when stage is disabled.
    assert!(!output.contains("Temporal Guards"), "should not have temporal section");
}

// ---------------------------------------------------------------------------
// Auto-generated header
// ---------------------------------------------------------------------------

#[test]
fn firrtl_has_auto_generated_header() {
    let src = r#"
module hdr {
    signal s: in bool;
    signal o: out bool;

    guard g {
        when s
        for 1 cycles;
    }

    reflex r {
        on g {
            o = s;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("Auto-generated by MIRR compiler"), "missing header comment");
    assert!(output.contains("Do not edit"), "missing do-not-edit comment");
}

// ---------------------------------------------------------------------------
// FPGA-001 Bug Fix Verification Tests
// ---------------------------------------------------------------------------

#[test]
fn firrtl_multi_guard_reflex_uses_and() {
    let src = r#"
module multi_guard_firrtl {
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
    let output = firrtl_from(src);
    assert!(output.contains("and("), "multi-guard should use and() not or()");
    assert!(!output.contains("or(g1_out"), "should not have or() join");
}

#[test]
fn firrtl_reflex_has_default_else() {
    let src = r#"
module else_test {
    signal s: in bool;
    signal out: out bool;

    guard g {
        when s
        for 1 cycles;
    }

    reflex r {
        on g {
            out = true;
        }
    }
}
"#;
    let output = firrtl_from(src);
    assert!(output.contains("else :"), "FIRRTL when blocks should have else clause");
    assert!(output.contains("connect out , UInt(0)"), "else should connect to UInt(0)");
}
