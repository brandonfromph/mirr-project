use super::*;

// Section 6: Temporal Guard Logic (Counter)
// ===========================================================================

#[test]
fn counter_guard_comment() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("counter"), "counter guard must have counter-related annotation");
}

#[test]
fn counter_guard_always_ff() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("always_ff @(posedge clk or negedge rst_n)"),
        "counter guard must use always_ff block"
    );
}

#[test]
fn counter_guard_reset_and_count_logic() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Counter resets on !rst_n or when condition is false
    assert!(sv.contains("if (!rst_n)"), "counter must have reset condition");
    assert!(sv.contains("<= '0;"), "counter must reset to zero");
    assert!(sv.contains("+ 1"), "counter must increment by 1");
}

#[test]
fn counter_guard_output_comparison() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains(">= 100"), "counter output must compare against target count 100");
}

// ===========================================================================