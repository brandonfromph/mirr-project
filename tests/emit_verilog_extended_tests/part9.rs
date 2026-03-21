use super::*;

// Section 9: Reflex Logic
// ===========================================================================

#[test]
fn reflex_section_header() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("// ── Reflex Assignments ──"),
        "must have reflex assignments section header"
    );
}

#[test]
fn reflex_always_comb_block() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("always_comb begin"), "reflex must use always_comb block");
}

#[test]
fn reflex_default_assignment_prevents_latch() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("= '0;"),
        "always_comb must have default assignments to prevent latch inference"
    );
}

#[test]
fn reflex_guard_out_wire_declared() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic g_out;"), "guard _out wire must be declared for reflex use");
}

#[test]
fn reflex_single_guard_condition() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("if (g_out)"), "single-guard reflex must use guard_out as condition");
}

#[test]
fn reflex_multi_guard_and_join() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("g1_out && g2_out"), "multi-guard reflex must AND-join guard outputs");
}

#[test]
fn reflex_name_in_comment() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// Reflex: r"), "reflex name must appear in comment");
}

#[test]
fn no_reflex_section_when_empty() {
    let result = run_pipeline(NO_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// ── Reflex Assignments ──"),
        "must NOT have reflex section when module has no reflexes"
    );
}

// ===========================================================================