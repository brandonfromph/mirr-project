use super::*;

// Section 2: Module Declaration and Port List
// ===========================================================================

#[test]
fn module_decl_contains_module_name() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("module minimal ("), "must contain module name in declaration");
}

#[test]
fn module_decl_auto_injects_clk_with_guards() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module minimal (").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    assert!(decl.contains("clk"), "clk must be auto-injected when guards exist");
    assert!(decl.contains("rst_n"), "rst_n must be auto-injected when guards exist");
}

#[test]
fn module_decl_no_clk_without_guards() {
    let result = run_pipeline(NO_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module bare (").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    assert!(!decl.contains("clk"), "clk must NOT be injected without guards");
    assert!(!decl.contains("rst_n"), "rst_n must NOT be injected without guards");
}

#[test]
fn module_decl_input_direction() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("input "), "must contain input direction for input signals");
}

#[test]
fn module_decl_output_direction() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("output"), "must contain output direction for output signals");
}

#[test]
fn module_decl_port_commas_correct() {
    let result = run_pipeline(MULTI_GUARD_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module multi_guard_mod (").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    // Count commas vs ports: N ports should have N-1 commas
    let comma_count = decl.chars().filter(|c| *c == ',').count();
    let port_lines: Vec<&str> =
        decl.lines().filter(|l| l.contains("input") || l.contains("output")).collect();

    assert!(port_lines.len() > 1, "must have multiple ports, found {}", port_lines.len());
    assert_eq!(
        comma_count,
        port_lines.len() - 1,
        "port list comma count ({}) must equal port count ({}) minus 1",
        comma_count,
        port_lines.len()
    );
}

#[test]
fn module_decl_internal_not_in_port_list() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    let decl_start = sv.find("module internals (").expect("must find module decl");
    let decl_end = sv[decl_start..].find(");").expect("must find );") + decl_start;
    let decl = &sv[decl_start..decl_end];

    assert!(!decl.contains("accumulator"), "internal signal must NOT appear in port list");
}

// ===========================================================================
