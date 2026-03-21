use super::*;

// Section 18: SVA-Only Mode
// ===========================================================================

#[test]
fn sva_only_no_module_wrapper() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sva = verilog::emit_sva_only(&result);

    assert!(!sva.contains("module prop_mod ("), "sva_only must NOT contain module declaration");
    assert!(!sva.contains("endmodule"), "sva_only must NOT contain endmodule");
}

#[test]
fn sva_only_contains_assertions() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sva = verilog::emit_sva_only(&result);

    assert!(sva.contains("assert property"), "sva_only must contain SVA assertions");
}

#[test]
fn sva_only_has_module_comment() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let sva = verilog::emit_sva_only(&result);

    assert!(sva.contains("Module: prop_mod"), "sva_only must reference the module name");
}

#[test]
fn sva_only_empty_for_no_properties() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sva = verilog::emit_sva_only(&result);

    // The header is still emitted but no assertions follow
    assert!(
        !sva.contains("assert property"),
        "sva_only must have no assertions when module has no properties"
    );
}

// ===========================================================================