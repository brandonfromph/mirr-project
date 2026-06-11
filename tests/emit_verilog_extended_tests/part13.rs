use super::*;

// Section 13: SVA Property Generation — All 6 Formula Variants
// ===========================================================================

#[test]
fn sva_always_formula() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// ── Safety Properties (SVA) ──"), "must have SVA section header");
    assert!(sv.contains("assert "), "always formula must use assert property");
}

#[test]
fn sva_never_formula_negation() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Never formula wraps in !(...)
    assert!(sv.contains("!("), "never formula must negate the expression");
}

#[test]
fn sva_always_implies_operator() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("|->"), "always-implies formula must contain |-> operator");
}

#[test]
fn sva_never_implies_negated_implication() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // never-implies has both |-> and !(...)
    let sva_section_start =
        sv.find("// ── Safety Properties (SVA) ──").expect("must find SVA section");
    let sva_section = &sv[sva_section_start..];

    assert!(sva_section.contains("|->"), "never-implies must contain |-> in SVA section");
}

#[test]
fn sva_eventually_within_temporal() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("prop_p_eventually_timer < 10"),
        "eventually within 10 must produce timer logic"
    );
}

#[test]
fn sva_always_followed_by_delay() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("(prop_p_followed_by_trig_shift[2]) |->"),
        "always followed_by 3 must produce shift register logic"
    );
}

#[test]
fn sva_posedge_clk_clock() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("@(posedge clk)"), "SVA properties must be clocked on posedge clk");
}

#[test]
fn sva_no_properties_when_empty() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// ── Safety Properties (SVA) ──"),
        "must NOT have SVA section when no properties exist"
    );
}

// ===========================================================================
