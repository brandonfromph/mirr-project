use super::*;

// Section 16: Bind File Generation
// ===========================================================================

#[test]
fn bind_file_empty_without_properties() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.is_empty(), "bind file must be empty when module has no properties");
}

#[test]
fn bind_file_contains_sva_module_name() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.contains("module prop_mod_sva"), "bind file must define _sva wrapper module");
}

#[test]
fn bind_file_contains_bind_statement() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(
        bind.contains("bind prop_mod prop_mod_sva u_sva (.*)"),
        "bind file must contain bind statement targeting original module"
    );
}

#[test]
fn bind_file_has_auto_generated_comment() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(
        bind.contains("Auto-generated SVA bind file"),
        "bind file must have auto-generated header"
    );
}

#[test]
fn bind_file_has_endmodule() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.contains("endmodule"), "bind file must contain endmodule");
}

#[test]
fn bind_file_ports_are_all_inputs() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    // In the bind SVA module, all ports should be inputs (observing the DUT)
    let mod_start = bind.find("module prop_mod_sva").expect("must find sva module");
    let mod_end = bind[mod_start..].find(");").expect("must find );") + mod_start;
    let mod_decl = &bind[mod_start..mod_end];

    assert!(
        !mod_decl.contains("output"),
        "bind file ports must all be inputs (observing DUT), found 'output'"
    );
}

#[test]
fn bind_file_contains_assert_property() {
    let result = run_pipeline(PROPERTY_ALL_VARIANTS, &default_config()).unwrap();
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.contains("assert "), "bind file must contain SVA assertions");
}

// ===========================================================================
