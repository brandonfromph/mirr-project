use super::*;

// Section 7: 1-Cycle Combinational Guard
// ===========================================================================

#[test]
fn one_cycle_guard_is_combinational() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // A 1-cycle guard is purely combinational (assign g_out = g_cond)
    assert!(sv.contains("assign g_out = g_cond"), "1-cycle guard must use combinational assign");
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
