use super::*;

// Section 7: 1-Cycle Combinational Guard
// ===========================================================================

#[test]
fn one_cycle_guard_no_always_ff() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // A 1-cycle guard is purely combinational
    assert!(!sv.contains("always_ff"), "1-cycle guard must NOT use always_ff (combinational only)");
}

#[test]
fn one_cycle_guard_uses_assign() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("assign g_out = g_cond"),
        "1-cycle guard must use combinational assign for output"
    );
}

#[test]
fn one_cycle_guard_condition_wire() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic g_cond"), "1-cycle guard must declare condition wire");
    assert!(sv.contains("assign g_cond ="), "1-cycle guard must assign condition wire");
}

// ===========================================================================