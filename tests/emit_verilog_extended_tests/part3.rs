use super::*;

// Section 3: Signal Type Rendering
// ===========================================================================

#[test]
fn sv_type_bool_renders_as_logic() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Bool type should render as "logic" with trailing spaces for alignment
    assert!(sv.contains("logic       "), "bool type must render as 'logic' with padding");
}

#[test]
fn sv_type_u8_renders_with_width() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic [ 7:0]"), "u8 must render as 'logic [ 7:0]'");
}

#[test]
fn sv_type_u16_renders_with_width() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic [15:0]"), "u16 must render as 'logic [15:0]'");
}

#[test]
fn sv_type_signed_renders_correctly() {
    let result = run_pipeline(SIGNED_TYPES_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("logic signed"), "signed type must contain 'logic signed'");
}

#[test]
fn sv_type_u1_renders_as_logic_no_range() {
    let source = r#"
module u1_mod {
    signal f: in u1;
    signal o: out bool;

    guard g {
        when f > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            o = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module u1_mod").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    assert!(!decl.contains("[0:0]"), "u1 must render as 'logic', not 'logic [0:0]'");
}

// ===========================================================================