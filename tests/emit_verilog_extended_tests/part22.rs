use super::*;

// Section 22: Temporal Guard Section Header
// ===========================================================================

#[test]
fn temporal_section_header_present() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// ── Temporal Guards ──"), "must have temporal guards section header");
}

#[test]
fn no_temporal_section_without_netlist() {
    let mut cfg = default_config();
    cfg.temporal = false;
    let result = run_pipeline(NO_GUARD_MODULE, &cfg).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// ── Temporal Guards ──"),
        "must NOT have temporal section when temporal stage is disabled"
    );
}

// ===========================================================================