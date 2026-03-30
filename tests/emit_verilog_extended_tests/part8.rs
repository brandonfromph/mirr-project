use super::*;

// Section 8: Complex Guard (Programmatic)
// ===========================================================================

#[test]
fn complex_guard_assign_block() {
    let module = Module {
        name: "complex_test".to_string(),
        signals: vec![
            signal_decl("a", SignalKind::Input, SignalType::Bool),
            signal_decl("b", SignalKind::Output, SignalType::Bool),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut netlist = TemporalNetlist::new();
    let complex = ComplexGuard::new("combo".to_string(), vec![], Expr::Signal("a".to_string()));
    netlist.add_guard(CompiledGuard::Complex(complex));

    let result = result_with_netlist(module, netlist);
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("Complex guard: combo"), "complex guard must have descriptive comment");
    assert!(
        sv.contains("assign combo_out = a"),
        "complex guard must have assign for combination logic"
    );
}

// ===========================================================================
