use super::*;

// Section 4: Internal Signal Declarations
// ===========================================================================

#[test]
fn internal_signals_section_header() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("// Internal signals"), "must have internal signals section comment");
}

#[test]
fn internal_signal_declared_inside_module() {
    let result = run_pipeline(INTERNAL_SIGNALS_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    // Internal signal should appear as a declaration after the port list
    let endport = sv.find(");").expect("must find );");
    let after_ports = &sv[endport..];

    assert!(
        after_ports.contains("accumulator"),
        "internal signal 'accumulator' must be declared after port list"
    );
}

#[test]
fn no_internal_section_when_none_exist() {
    let result = run_pipeline(MINIMAL_MODULE, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        !sv.contains("// Internal signals"),
        "must NOT have internal signals section when there are no internals"
    );
}

// ===========================================================================