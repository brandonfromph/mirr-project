use super::*;

// Section 17: Synthesis-Clean Mode
// ===========================================================================

#[test]
fn synthesis_mode_strips_sva() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(!sv.contains("assert property"), "synthesis mode must strip 'assert property'");
    assert!(!sv.contains("assume property"), "synthesis mode must strip 'assume property'");
    assert!(!sv.contains("cover property"), "synthesis mode must strip 'cover property'");
}

#[test]
fn synthesis_mode_preserves_module() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(sv.contains("module prop_mod"), "synthesis mode must preserve module declaration");
    assert!(sv.contains("endmodule"), "synthesis mode must preserve endmodule");
}

#[test]
fn synthesis_mode_preserves_rtl() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(sv.contains("always_ff"), "synthesis mode must preserve always_ff blocks");
    assert!(sv.contains("always_comb"), "synthesis mode must preserve always_comb blocks");
}

#[test]
fn synthesis_mode_no_sva_section_header() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(
        !sv.contains("Safety Properties (SVA)"),
        "synthesis mode must strip the SVA section header"
    );
}

// ===========================================================================