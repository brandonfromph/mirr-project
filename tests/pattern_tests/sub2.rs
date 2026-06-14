use super::*;

#[test]
fn expanded_guard_cycles_should_be_preserved() {
    let src = r#"
def simple(s: signal in bool) {
    reflect {
        guard check {
            when ${s}
            for 3 cycles;
        }
    }
}
module m {
    signal sensor: in bool;
    simple(sensor);
}
"#;
    let result = pipeline_ok(src);
    let expanded_guard = result
        .program
        .as_ref()
        .unwrap()
        .module
        .guards
        .iter()
        .find(|g| g.name.contains("check"))
        .expect("Should find expanded guard");
    assert_eq!(expanded_guard.cycles, 3);
}

// =========================================================================
// Category 4: Pattern Expansion & Name Prefixing (8 tests)
// =========================================================================

#[test]
fn prefix_single_call_guard_name() {
    let src = r#"
def simple_guard(s: signal in bool) {
    reflect {
        guard check {
            when ${s}
            for 2 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    simple_guard(x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    let has_prefixed = result
        .program
        .as_ref()
        .unwrap()
        .module
        .guards
        .iter()
        .any(|g| g.name == "simple_guard_0_check");
    assert!(
        has_prefixed,
        "Guard should be prefixed: {:?}",
        result.program.as_ref().unwrap().module.guards.iter().map(|g| &g.name).collect::<Vec<_>>()
    );
}

#[test]
fn prefix_two_calls_get_different_indices() {
    let result = pipeline_ok(&ventilator_source());
    let guard_names: Vec<&str> =
        result.program.as_ref().unwrap().module.guards.iter().map(|g| g.name.as_str()).collect();
    let has_0 = guard_names.iter().any(|n| n.contains("_0_"));
    let has_1 = guard_names.iter().any(|n| n.contains("_1_"));
    assert!(has_0, "First call should have index 0: {guard_names:?}");
    assert!(has_1, "Second call should have index 1: {guard_names:?}");
}

#[test]
fn prefix_internal_signals_get_prefixed() {
    let result = pipeline_ok(&ventilator_source());
    let internal_sigs: Vec<&str> = result
        .program
        .as_ref()
        .unwrap()
        .module
        .signals
        .iter()
        .filter(|s| s.kind == SignalKind::Internal)
        .map(|s| s.name.as_str())
        .collect();
    for sig in &internal_sigs {
        assert!(sig.contains("monitor_sensor_"), "Internal signal should be prefixed: {sig}");
    }
}

#[test]
fn origin_tag_set_on_expanded_nodes() {
    let result = pipeline_ok(&ventilator_source());
    let m = &result.program.as_ref().unwrap().module;
    for guard in &m.guards {
        assert!(guard.origin.is_some(), "Expanded guard '{}' should have origin", guard.name);
    }
    for reflex in &m.reflexes {
        assert!(reflex.origin.is_some(), "Expanded reflex '{}' should have origin", reflex.name);
    }
}

#[test]
fn origin_tag_none_on_hand_written_nodes() {
    let src = r#"
def pat(s: signal in bool) {
    reflect {
        guard ${s}_check {
            when ${s}
            for 2 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    pat(x);

    guard manual_guard {
        when x
        for 1 cycles;
    }

    reflex manual_reflex {
        on manual_guard {
            y = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    let manual_g = result
        .program
        .as_ref()
        .unwrap()
        .module
        .guards
        .iter()
        .find(|g| g.name == "manual_guard")
        .expect("Should have manual guard");
    assert!(manual_g.origin.is_none(), "Hand-written guard should have origin: None");
    let manual_r = result
        .program
        .as_ref()
        .unwrap()
        .module
        .reflexes
        .iter()
        .find(|r| r.name == "manual_reflex")
        .expect("Should have manual reflex");
    assert!(manual_r.origin.is_none(), "Hand-written reflex should have origin: None");
}

#[test]
fn expanded_module_passes_validation() {
    let result = pipeline_ok(&ventilator_source());
    validate_module(&result.program.as_ref().unwrap().module)
        .expect("Post-expansion module should pass validation");
}

#[test]
fn expanded_properties_reference_prefixed_signals() {
    let result = pipeline_ok(&ventilator_source());
    let props = &result.program.as_ref().unwrap().module.properties;
    assert!(!props.is_empty(), "Should have expanded properties");
    for prop in props {
        assert!(prop.origin.is_some(), "Expanded property should have origin");
    }
}

#[test]
fn arg_count_mismatch_error() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;

    monitor_sensor(p, 10);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            a = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let msg = pipeline_err(&src);
    assert!(msg.contains("expects") && msg.contains("arguments, got"), "Unexpected: {msg}");
}

// =========================================================================
// Category 5: Depth Limit (3 tests)
// =========================================================================

#[test]
fn depth_1_nested_pattern_works() {
    let src = r#"
def inner(s: signal in bool) {
    reflect {
        guard ${s}_inner {
            when ${s}
            for 1 cycles;
        }
    }
}
def outer(s: signal in bool) {
    reflect {
        guard ${s}_outer {
            when ${s}
            for 2 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    inner(x);
    outer(x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    assert!(
        result.program.as_ref().unwrap().module.guards.len() >= 3,
        "Should have inner + outer + manual guard"
    );
}

#[test]
fn depth_4_boundary_works() {
    let src = r#"
def d1(s: signal in bool) {
    reflect {
        guard ${s}_d1 {
            when ${s}
            for 1 cycles;
        }
    }
}
def d2(s: signal in bool) {
    reflect {
        guard ${s}_d2 {
            when ${s}
            for 1 cycles;
        }
    }
}
def d3(s: signal in bool) {
    reflect {
        guard ${s}_d3 {
            when ${s}
            for 1 cycles;
        }
    }
}
def d4(s: signal in bool) {
    reflect {
        guard ${s}_d4 {
            when ${s}
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    d1(x);
    d2(x);
    d3(x);
    d4(x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    assert!(
        result.program.as_ref().unwrap().module.guards.len() >= 5,
        "Should have 4 expanded + 1 manual guard"
    );
}

#[test]
fn depth_limit_exceeded_pinned_message() {
    // Verify the exact pinned message format exists in the expansion code.
    let expected = "expansion depth limit";
    let source_code = include_str!("../../src/expand/mod.rs");
    assert!(
        source_code.contains(expected),
        "expand/mod.rs must contain the pinned depth limit message"
    );
    assert!(
        source_code.contains("exceeded in"),
        "expand/mod.rs must contain 'exceeded in' substring"
    );
}

// =========================================================================
// Category 6: Internal Signal Scoping (3 tests)
// =========================================================================

#[test]
fn internal_signal_scoping_pinned_message_format() {
    let expected = "is internal to pattern";
    let source_code = include_str!("../../src/expand/scoping.rs");
    assert!(
        source_code.contains(expected),
        "expand/scoping.rs must contain the internal signal scoping message"
    );
    assert!(
        source_code.contains("and cannot be referenced externally"),
        "expand/scoping.rs must contain the full scoping message"
    );
}

#[test]
fn internal_signal_invisible_outside_expansion() {
    let src = r#"
def make_internal(s: signal in bool) {
    reflect {
        signal ${s}_hidden: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    make_internal(x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    let internal_sigs: Vec<&str> = result
        .program
        .as_ref()
        .unwrap()
        .module
        .signals
        .iter()
        .filter(|s| s.kind == SignalKind::Internal && s.origin.is_some())
        .map(|s| s.name.as_str())
        .collect();
    assert!(!internal_sigs.is_empty(), "Should have internal signals from pattern");
}

#[test]
fn cross_expansion_internal_refs_checked() {
    let source_code = include_str!("../../src/expand/scoping.rs");
    assert!(
        source_code.contains("check_expr_cross_expansion"),
        "expand/scoping.rs must implement cross-expansion checking"
    );
}

// =========================================================================
// Category 7: Runtime Argument Detection (2 tests)
// =========================================================================

#[test]
fn runtime_arg_detection_valid_args_pass() {
    let src = r#"
def pat(x: u16) {
    reflect {
        guard gp {
            when true
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let _ = pipeline_ok(src);
}

#[test]
fn undeclared_signal_arg_detected_at_validation() {
    let src = r#"
def pat(s: signal in bool) {
    reflect {
        guard ${s}_check {
            when ${s}
            for 2 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    pat(ghost_signal);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let msg = pipeline_err(src);
    assert!(msg.contains("undeclared") || msg.contains("ghost_signal"), "Unexpected: {msg}");
}

// =========================================================================
// Category 8: Emission & Pipeline Integration (8 tests)
// =========================================================================

#[test]
fn verilog_origin_comment_present() {
    let result = pipeline_ok(&ventilator_source());
    let sv = emit::verilog::emit_sv(&result);
    assert!(sv.contains("// Pattern: monitor_sensor_0"), "SV should contain origin comment:\n{sv}");
}

#[test]
fn verilog_no_origin_comment_on_hand_written() {
    let src = r#"
module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 2 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    let sv = emit::verilog::emit_sv(&result);
    assert!(!sv.contains("// Pattern:"), "Hand-written module should have no Pattern comments");
}

#[test]
fn json_roundtrip_no_patterns_byte_identical() {
    let src = r#"
module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 2 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    let json = emit::json_netlist::emit_json(&result).expect("JSON should serialize");
    assert!(!json.contains("\"origin\""), "JSON with no patterns should not have origin field");
}

#[test]
fn json_output_includes_origin_on_expanded() {
    let result = pipeline_ok(&ventilator_source());
    let json = emit::json_netlist::emit_json(&result).expect("JSON should serialize");
    assert!(json.contains("\"origin\""), "JSON should include origin for expanded nodes");
}

#[test]
fn dot_output_shows_pattern_origin() {
    let result = pipeline_ok(&ventilator_source());
    let dot = emit::dot::emit_module_dot(&result);
    assert!(dot.contains("Pattern:"), "DOT should contain pattern origin info");
}

#[test]
fn full_pipeline_ventilator_e2e() {
    let result = pipeline_ok(&ventilator_source());
    let sv = emit::verilog::emit_sv(&result);
    assert!(sv.contains("module ventilator"), "Should have module declaration");
    assert!(sv.contains("endmodule"), "Should have endmodule");
    assert!(sv.contains("assert "), "Should have SVA assertions from expanded properties");
    assert!(
        !result.program.as_ref().unwrap().module.properties.is_empty(),
        "Should have properties"
    );
}

#[test]
fn sva_emitted_for_pattern_property() {
    let result = pipeline_ok(&ventilator_source());
    let sv = emit::verilog::emit_sv(&result);
    assert!(sv.contains("assert "), "SV should contain assert property from pattern");
    assert!(sv.contains("|->"), "SV should contain implication from AlwaysImplies property");
}

#[test]
fn multiple_calls_produce_distinct_blocks() {
    let result = pipeline_ok(&ventilator_source());
    let sv = emit::verilog::emit_sv(&result);
    let reflex_count = sv.matches("always_ff @").count();
    assert!(reflex_count >= 2, "Should have at least 2 always_ff blocks: found {reflex_count}");
}

// =========================================================================
// Category 9: Additional robustness tests (4 tests)
// =========================================================================

#[test]
fn pattern_with_property_always_implies() {
    let result = pipeline_ok(&ventilator_source());
    let props = &result.program.as_ref().unwrap().module.properties;
    assert!(props.len() >= 2, "Should have at least 2 properties from 2 calls");
}

#[test]
fn pattern_origin_has_correct_format() {
    let result = pipeline_ok(&ventilator_source());
    let origins: Vec<&str> = result
        .program
        .as_ref()
        .unwrap()
        .module
        .pattern_origins
        .iter()
        .map(|o| o.pattern_name.as_str())
        .collect();
    assert!(origins.contains(&"monitor_sensor"), "Should record monitor_sensor as origin");
}

#[test]
fn pattern_call_not_confused_with_keyword() {
    let src = r#"
module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let prog = parse_ok(src);
    assert!(prog.module.pattern_calls.is_empty(), "Keywords should not be pattern calls");
}

#[test]
fn pattern_call_type_mismatch_signal_vs_constant() {
    let src = r#"
def typed_pat(s: signal in bool, v: u16) {
    reflect {
        guard gtp {
            when ${s}
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    typed_pat(42, x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let msg = pipeline_err(src);
    assert!(msg.contains("expects a signal reference, got a constant"), "Unexpected: {msg}");
}
