use super::*;

// Section 5: Temporal Guard Logic (Shift Register)
// ===========================================================================

#[test]
fn shift_register_guard_comment() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// Guard:"), "must contain guard comment annotations");
}

#[test]
fn shift_register_always_ff_block() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("always_ff @(posedge clk or negedge rst_n)"),
        "shift register guard must use always_ff with clk and rst_n"
    );
}

#[test]
fn shift_register_reset_logic() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("if (!rst_n)"), "shift register must have reset condition");
    assert!(sv.contains("<= '0;"), "shift register must reset to zero");
}

#[test]
fn shift_register_sr_declaration() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // g1 has 3 cycles, so shift register width is [2:0]
    assert!(sv.contains("g1_sr"), "must declare g1 shift register");
    // g2 has 5 cycles, so shift register width is [4:0]
    assert!(sv.contains("g2_sr"), "must declare g2 shift register");
}

#[test]
fn shift_register_condition_wire() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("g1_cond"), "must declare condition wire for g1");
    assert!(sv.contains("g2_cond"), "must declare condition wire for g2");
}

#[test]
fn shift_register_output_and_reduction() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Output fires when all stages are 1: assign gN_out = &gN_sr;
    assert!(sv.contains("&g1_sr"), "g1 output must use AND-reduction of shift register");
    assert!(sv.contains("&g2_sr"), "g2 output must use AND-reduction of shift register");
}

// ===========================================================================