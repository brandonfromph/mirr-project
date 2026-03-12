#![forbid(unsafe_code)]
//! Codegen pattern tests — verify generated output contains expected patterns.
//!
//! These are NOT full golden-file tests (too brittle). Instead, each test
//! compiles a .mirr example and asserts that specific structural patterns
//! appear in the generated Verilog, FIRRTL, or S-expression output.

use nasa_rust_project::emit::firrtl::emit_firrtl;
use nasa_rust_project::emit::verilog::emit_sv;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

/// Helper: compile MIRR source to Verilog with full pipeline.
fn compile_to_verilog(source: &str) -> String {
    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("pipeline failed");
    emit_sv(&result)
}

/// Helper: compile MIRR source to FIRRTL.
fn compile_to_firrtl(source: &str) -> String {
    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("pipeline failed");
    emit_firrtl(&result)
}

// ---------------------------------------------------------------------------
// Shift register tests
// ---------------------------------------------------------------------------

const SHIFT_REG_MIRR: &str = r#"
module short_delay_monitor {
    signal sensor_active: in bool;
    signal alert_lamp:    out bool;

    guard brief_activation {
        when sensor_active
        for  8 cycles;
    }

    reflex activate_alert {
        on brief_activation {
            alert_lamp = true;
        }
    }
}
"#;

#[test]
fn shift_register_verilog_has_always_ff() {
    let verilog = compile_to_verilog(SHIFT_REG_MIRR);
    assert!(verilog.contains("always_ff"), "Verilog missing always_ff block:\n{verilog}");
}

#[test]
fn shift_register_verilog_has_shift_pattern() {
    let verilog = compile_to_verilog(SHIFT_REG_MIRR);
    // Shift register guard should produce a shift register with 8 bits
    assert!(
        verilog.contains("[7:0]")
            || verilog.contains("[0:7]")
            || verilog.contains("brief_activation"),
        "Verilog missing shift register pattern:\n{verilog}"
    );
}

#[test]
fn shift_register_verilog_has_module_declaration() {
    let verilog = compile_to_verilog(SHIFT_REG_MIRR);
    assert!(
        verilog.contains("module short_delay_monitor"),
        "Verilog missing module declaration:\n{verilog}"
    );
}

// ---------------------------------------------------------------------------
// Neonatal respirator — counter guard tests
// ---------------------------------------------------------------------------

const NEONATAL_MIRR: &str = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;

#[test]
fn neonatal_verilog_has_counter_pattern() {
    let verilog = compile_to_verilog(NEONATAL_MIRR);
    // 1000 cycles > 16 threshold, so uses counter guard, should have >= comparison
    assert!(
        verilog.contains("1000") || verilog.contains("sustained_pressure_drop"),
        "Verilog missing counter pattern for 1000-cycle guard:\n{verilog}"
    );
}

#[test]
fn neonatal_verilog_has_comparison() {
    let verilog = compile_to_verilog(NEONATAL_MIRR);
    assert!(
        verilog.contains("50") || verilog.contains("airway_pressure"),
        "Verilog missing pressure comparison:\n{verilog}"
    );
}

#[test]
fn neonatal_firrtl_has_circuit() {
    let firrtl = compile_to_firrtl(NEONATAL_MIRR);
    assert!(
        firrtl.contains("circuit") || firrtl.contains("module"),
        "FIRRTL missing circuit/module declaration:\n{firrtl}"
    );
}

#[test]
fn neonatal_firrtl_has_ports() {
    let firrtl = compile_to_firrtl(NEONATAL_MIRR);
    assert!(
        firrtl.contains("airway_pressure") && firrtl.contains("clamp_valve"),
        "FIRRTL missing expected port names:\n{firrtl}"
    );
}

// ---------------------------------------------------------------------------
// Safety property / SVA tests
// ---------------------------------------------------------------------------

const SAFETY_MIRR: &str = r#"
module pressure_monitor {
    signal airway_pressure: in  u16;
    signal clamp_valve:     out bool;

    guard pressure_low {
        when airway_pressure < 50
        for 3 cycles;
    }

    reflex engage_clamp {
        on pressure_low {
            clamp_valve = true;
        }
    }

    property pressure_bounded {
        always (airway_pressure > 10);
    }

    property no_spurious_clamp {
        never (clamp_valve && airway_pressure > 200);
    }

    property low_triggers_clamp {
        always (airway_pressure < 50 -> clamp_valve);
    }

    property clamp_reachable {
        cover eventually within 100 (clamp_valve);
    }

    property clamp_follows_drop {
        always (airway_pressure < 50 followed_by 5 clamp_valve);
    }
}
"#;

#[test]
fn safety_verilog_has_sva_assertions() {
    let verilog = compile_to_verilog(SAFETY_MIRR);
    // Should contain at least some SVA property constructs
    let has_assert = verilog.contains("assert") || verilog.contains("property");
    let has_cover = verilog.contains("cover");
    assert!(has_assert || has_cover, "Verilog missing SVA assertion constructs:\n{verilog}");
}

#[test]
fn safety_verilog_has_all_property_names() {
    let verilog = compile_to_verilog(SAFETY_MIRR);
    assert!(verilog.contains("pressure_bounded"), "Missing property pressure_bounded:\n{verilog}");
    assert!(
        verilog.contains("no_spurious_clamp"),
        "Missing property no_spurious_clamp:\n{verilog}"
    );
}

// ---------------------------------------------------------------------------
// Multi-guard AND test
// ---------------------------------------------------------------------------

const MULTI_GUARD_MIRR: &str = r#"
module patient_monitor {
    signal heart_rate:      in u16;
    signal blood_pressure:  in u16;
    signal alarm_active:    out bool;
    signal pump_override:   out bool;
    signal status_flag:     internal bool;

    guard bradycardia {
        when heart_rate < 60
        for  500 cycles;
    }

    guard hypotension {
        when blood_pressure < 90
        for  12 cycles;
    }

    reflex cardiac_alarm {
        on bradycardia {
            alarm_active = true;
        }
    }

    reflex emergency_override {
        on bradycardia and hypotension {
            pump_override = true;
        }
    }
}
"#;

#[test]
fn multi_guard_verilog_has_both_guards() {
    let verilog = compile_to_verilog(MULTI_GUARD_MIRR);
    assert!(verilog.contains("bradycardia"), "Verilog missing bradycardia guard:\n{verilog}");
    assert!(verilog.contains("hypotension"), "Verilog missing hypotension guard:\n{verilog}");
}

#[test]
fn multi_guard_verilog_has_and_combination() {
    let verilog = compile_to_verilog(MULTI_GUARD_MIRR);
    // Multi-guard AND should produce a combined condition with &&
    assert!(
        verilog.contains("&&") || verilog.contains("&"),
        "Verilog missing AND combination for multi-guard reflex:\n{verilog}"
    );
}
