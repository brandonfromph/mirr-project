use super::*;

// Section 19: Synchronizer Chain Emission
// ===========================================================================

#[test]
fn synchronizer_chains_header_comment() {
    let module = Module {
        name: "sync_test".to_string(),
        signals: vec![
            signal_decl("clk", SignalKind::Input, SignalType::Bool),
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("data_in", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("data_out", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    let mappings = verilog::emit_synchronizer_chains(&module, 2, &mut out);

    assert!(
        out.contains("// ── Input Synchronizer Chains ──"),
        "must have synchronizer section header"
    );
    assert!(!mappings.is_empty(), "must produce mappings for input signals");
}

#[test]
fn synchronizer_chains_skip_clk_rst() {
    let module = Module {
        name: "sync_skip".to_string(),
        signals: vec![
            signal_decl("clk", SignalKind::Input, SignalType::Bool),
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("data", SignalKind::Input, SignalType::Unsigned(8)),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    let mappings = verilog::emit_synchronizer_chains(&module, 2, &mut out);

    // Only data should be synchronized, not clk or rst_n
    let mut found_clk_sync = false;
    let mut found_data_sync = false;
    for i in 0..MAX_PORTS_CHECK {
        if i >= mappings.len() {
            break;
        }
        let (orig, _synced) = &mappings[i];
        if orig == "clk" || orig == "rst_n" {
            found_clk_sync = true;
        }
        if orig == "data" {
            found_data_sync = true;
        }
    }

    assert!(!found_clk_sync, "clk and rst_n must NOT be synchronized");
    assert!(found_data_sync, "data signal must be synchronized");
}

#[test]
fn synchronizer_chains_produces_sync_register() {
    let module = Module {
        name: "sync_reg".to_string(),
        signals: vec![
            signal_decl("clk", SignalKind::Input, SignalType::Bool),
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("sig_in", SignalKind::Input, SignalType::Unsigned(4)),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    verilog::emit_synchronizer_chains(&module, 2, &mut out);

    assert!(out.contains("sig_in_sync"), "must declare synchronizer register");
    assert!(out.contains("sig_in_s"), "must declare synchronized output signal");
    assert!(out.contains("always_ff"), "synchronizer must use always_ff");
}

#[test]
fn synchronizer_chains_zero_stages_returns_empty() {
    let module = Module {
        name: "sync_zero".to_string(),
        signals: vec![signal_decl("data", SignalKind::Input, SignalType::Unsigned(8))],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    let mappings = verilog::emit_synchronizer_chains(&module, 0, &mut out);

    assert!(mappings.is_empty(), "zero sync stages must return empty mappings");
    assert!(out.is_empty(), "zero sync stages must produce no output");
}

#[test]
fn synchronizer_chains_skip_output_signals() {
    let module = Module {
        name: "sync_out".to_string(),
        signals: vec![
            signal_decl("clk", SignalKind::Input, SignalType::Bool),
            signal_decl("rst_n", SignalKind::Input, SignalType::Bool),
            signal_decl("data_in", SignalKind::Input, SignalType::Unsigned(8)),
            signal_decl("data_out", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };

    let mut out = String::new();
    let mappings = verilog::emit_synchronizer_chains(&module, 2, &mut out);

    let mut found_output_sync = false;
    for i in 0..MAX_PORTS_CHECK {
        if i >= mappings.len() {
            break;
        }
        if mappings[i].0 == "data_out" {
            found_output_sync = true;
        }
    }

    assert!(!found_output_sync, "output signals must NOT be synchronized (only inputs)");
}

// ===========================================================================
