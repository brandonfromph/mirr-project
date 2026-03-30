use super::*;

// Section 25: Full Pipeline Integration (MIRR source -> SV)
// ===========================================================================

#[test]
fn full_pipeline_round_trip_minimal() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Verify all major sections are present
    assert!(sv.contains("Auto-generated"), "missing header");
    assert!(sv.contains("module minimal"), "missing module declaration");
    assert!(sv.contains("always_comb"), "missing always_comb");
    assert!(sv.contains("endmodule"), "missing endmodule");
}

#[test]
fn full_pipeline_counter_guard_module() {
    let result = run_pipeline(COUNTER_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("module counter_mod"), "missing module name");
    assert!(sv.contains("always_ff"), "counter guard must have always_ff");
    assert!(sv.contains("always_comb"), "reflex must have always_comb");
    assert!(sv.contains(">= 100"), "counter must compare against 100");
}

#[test]
fn full_pipeline_multi_guard_module() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("module multi_guard_mod"), "missing module name");
    assert!(sv.contains("g1_sr"), "missing g1 shift register");
    assert!(sv.contains("g2_sr"), "missing g2 shift register");
    assert!(sv.contains("g1_out && g2_out"), "missing AND-joined guard condition");
}

#[test]
fn full_pipeline_all_properties_module() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Verify all 6 property names appear as comments
    let property_names =
        ["p_always", "p_never", "p_implies", "p_never_implies", "p_eventually", "p_followed_by"];
    for i in 0..MAX_PROPERTY_VARIANTS {
        if i >= property_names.len() {
            break;
        }
        assert!(
            sv.contains(&format!("// property: {}", property_names[i])),
            "missing property comment for {}",
            property_names[i]
        );
    }
}

// ===========================================================================
