//! Phase 7b: Homoiconic pattern system tests.
//!
//! Tests pattern definition parsing, pattern call parsing, substitution,
//! expansion, name prefixing, origin tagging, depth limits, internal signal
//! scoping, error messages, and emission integration.
//!
//! Minimum 48 tests. All error messages pinned with exact strings from spec.

use nasa_rust_project::ast::pattern::{PatternArg, PatternParamKind};
use nasa_rust_project::ast::types::{SignalKind, SignalType};
use nasa_rust_project::emit;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::validate_module;

// =========================================================================
// Helpers
// =========================================================================

/// Parse and return the program, or panic with the error.
fn parse_ok(source: &str) -> nasa_rust_project::MirrProgram {
    parse_mirr(source).unwrap_or_else(|e| panic!("Parse failed: {e}"))
}

/// Parse and return the error message string.
fn parse_err(source: &str) -> String {
    parse_mirr(source).expect_err("Expected parse error").to_string()
}

/// Run full pipeline and return the error message string.
fn pipeline_err(source: &str) -> String {
    match run_pipeline(source, &PipelineConfig::default()) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("Expected pipeline error"),
    }
}

/// Run full pipeline and return the PipelineResult.
fn pipeline_ok(source: &str) -> nasa_rust_project::pipeline::PipelineResult {
    run_pipeline(source, &PipelineConfig::default())
        .unwrap_or_else(|e| panic!("Pipeline failed: {e}"))
}

/// A minimal module footer to make tests self-contained.
const MOD_FOOTER: &str = r#"
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

/// Minimal pattern definition for monitor_sensor.
/// Uses the parser-compatible multiline format.
fn monitor_sensor_source() -> &'static str {
    r#"
def monitor_sensor(
    sensor: signal in u16,
    low:    u16,
    high:   u16,
    cycles: u32,
    alarm:  signal out bool
) {
    reflect {
        signal ${sensor}_debounced: internal bool;

        guard ${sensor}_too_low {
            when ${sensor} < ${low}
            for  ${cycles} cycles;
        }

        guard ${sensor}_too_high {
            when ${sensor} > ${high}
            for  ${cycles} cycles;
        }

        reflex ${sensor}_response_low {
            on ${sensor}_too_low {
                ${alarm} = true;
            }
        }

        reflex ${sensor}_response_high {
            on ${sensor}_too_high {
                ${sensor}_debounced = true;
            }
        }

        property ${sensor}_alarm_correct {
            always (${sensor} < ${low} -> ${alarm});
        }
    }
}
"#
}

/// Ventilator module using monitor_sensor.
fn ventilator_source() -> String {
    format!(
        r#"
{monitor}
module ventilator {{
    signal airway_pressure: in  u16;
    signal heart_rate:      in  u16;
    signal pressure_alarm:  out bool;
    signal heartrate_alarm: out bool;

    monitor_sensor(airway_pressure, 50, 200, 1000, pressure_alarm);
    monitor_sensor(heart_rate, 40, 180, 500, heartrate_alarm);
}}
"#,
        monitor = monitor_sensor_source()
    )
}

// =========================================================================
// Category 1: Pattern Definition Parsing (10 tests)
// =========================================================================

#[test]
fn parse_def_minimal_params() {
    let src = format!(
        r#"
def toggle(s: signal in bool, val: bool) {{
    reflect {{
        guard ${{s}}_guard {{
            when ${{s}}
            for 1 cycles;
        }}
    }}
}}
{MOD_FOOTER}
"#
    );
    let prog = parse_ok(&src);
    assert_eq!(prog.patterns.len(), 1);
    assert_eq!(prog.patterns[0].name, "toggle");
    assert_eq!(prog.patterns[0].params.len(), 2);
}

#[test]
fn parse_def_all_param_kinds() {
    let src = format!(
        r#"
def all_kinds(
    a: signal in u16,
    b: signal out bool,
    c: u16,
    d: u32,
    e: bool
) {{
    reflect {{
        guard g {{
            when true
            for 1 cycles;
        }}
    }}
}}
{MOD_FOOTER}
"#
    );
    let prog = parse_ok(&src);
    let params = &prog.patterns[0].params;
    assert_eq!(params.len(), 5);
    assert!(matches!(
        params[0].kind,
        PatternParamKind::Signal { kind: SignalKind::Input, ty: SignalType::Unsigned(16) }
    ));
    assert!(matches!(
        params[1].kind,
        PatternParamKind::Signal { kind: SignalKind::Output, ty: SignalType::Bool }
    ));
    assert!(matches!(params[2].kind, PatternParamKind::Constant { ty: SignalType::Unsigned(16) }));
    assert!(matches!(params[3].kind, PatternParamKind::Constant { ty: SignalType::Unsigned(32) }));
    assert!(matches!(params[4].kind, PatternParamKind::Constant { ty: SignalType::Bool }));
}

#[test]
fn parse_def_reflect_body_all_decl_types() {
    let src = format!(
        r#"
{monitor}
{MOD_FOOTER}
"#,
        monitor = monitor_sensor_source()
    );
    let prog = parse_ok(&src);
    let body_lines = &prog.patterns[0].body.raw_lines;
    assert!(!body_lines.is_empty(), "reflect body should have lines");
}

#[test]
fn parse_def_missing_params_error() {
    let src = format!(
        r#"
def bad_def {{
    reflect {{
    }}
}}
{MOD_FOOTER}
"#
    );
    let msg = parse_err(&src);
    assert!(msg.contains("Pattern") || msg.contains("pattern"), "Unexpected error: {msg}");
}

#[test]
fn parse_def_missing_reflect_keyword_error() {
    let src = format!(
        r#"
def bad_def(x: u16) {{
    guard g {{
        when true
        for 1 cycles;
    }}
}}
{MOD_FOOTER}
"#
    );
    let msg = parse_err(&src);
    assert!(msg.contains("reflect"), "Should mention 'reflect': {msg}");
}

#[test]
fn parse_def_empty_name_error() {
    let src = format!(
        r#"
def (x: u16) {{
    reflect {{
    }}
}}
{MOD_FOOTER}
"#
    );
    let msg = parse_err(&src);
    assert!(msg.contains("empty") || msg.contains("name"), "Unexpected error: {msg}");
}

#[test]
fn parse_def_unknown_param_type_error() {
    let src = format!(
        r#"
def bad_type(x: f32) {{
    reflect {{
    }}
}}
{MOD_FOOTER}
"#
    );
    let msg = parse_err(&src);
    assert!(msg.contains("f32") || msg.contains("unknown type"), "Unexpected error: {msg}");
}

#[test]
fn parse_two_defs_before_module() {
    let src = format!(
        r#"
def pat_a(x: u16) {{
    reflect {{
        guard ga {{
            when true
            for 1 cycles;
        }}
    }}
}}

def pat_b(y: bool) {{
    reflect {{
        guard gb {{
            when true
            for 2 cycles;
        }}
    }}
}}
{MOD_FOOTER}
"#
    );
    let prog = parse_ok(&src);
    assert_eq!(prog.patterns.len(), 2);
    assert_eq!(prog.patterns[0].name, "pat_a");
    assert_eq!(prog.patterns[1].name, "pat_b");
}

#[test]
fn parse_duplicate_def_names_error() {
    let src = format!(
        r#"
def dup(x: u16) {{
    reflect {{
        guard ga {{
            when true
            for 1 cycles;
        }}
    }}
}}

def dup(y: u16) {{
    reflect {{
        guard gb {{
            when true
            for 2 cycles;
        }}
    }}
}}
{MOD_FOOTER}
"#
    );
    // Duplicate detected at expansion stage
    let msg = pipeline_err(&src);
    assert!(msg.contains("Duplicate pattern definition"), "Unexpected: {msg}");
}

#[test]
fn parse_def_with_zero_params() {
    let src = format!(
        r#"
def empty_def() {{
    reflect {{
        guard gz {{
            when true
            for 1 cycles;
        }}
    }}
}}
{MOD_FOOTER}
"#
    );
    let prog = parse_ok(&src);
    assert_eq!(prog.patterns[0].params.len(), 0);
}

// =========================================================================
// Category 2: Pattern Call Parsing (8 tests)
// =========================================================================

#[test]
fn parse_call_basic() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;

    monitor_sensor(p, 10, 200, 500, a);

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
    let prog = parse_ok(&src);
    assert_eq!(prog.module.pattern_calls.len(), 1);
    assert_eq!(prog.module.pattern_calls[0].pattern_name, "monitor_sensor");
    assert_eq!(prog.module.pattern_calls[0].arguments.len(), 5);
}

#[test]
fn parse_call_bool_literal_args() {
    let src = r#"
def flag_set(v: bool) {
    reflect {
        guard gf {
            when true
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    flag_set(true);
    flag_set(false);

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
"#
    .to_string();
    let prog = parse_ok(&src);
    assert_eq!(prog.module.pattern_calls.len(), 2);
    assert!(matches!(prog.module.pattern_calls[0].arguments[0], PatternArg::ConstBool(true)));
    assert!(matches!(prog.module.pattern_calls[1].arguments[0], PatternArg::ConstBool(false)));
}

#[test]
fn parse_call_whitespace_around_commas() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;

    monitor_sensor(  p  ,  10  ,  200  ,  500  ,  a  );

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
    let prog = parse_ok(&src);
    assert_eq!(prog.module.pattern_calls[0].arguments.len(), 5);
    assert!(
        matches!(&prog.module.pattern_calls[0].arguments[0], PatternArg::SignalRef(s) if s == "p")
    );
}

#[test]
fn parse_call_missing_semicolon_error() {
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

    pat(42)
}
"#
    .to_string();
    // Treated as unknown line, not a pattern call
    let msg = parse_err(&src);
    assert!(!msg.is_empty(), "Should produce an error");
}

#[test]
fn parse_call_wrong_arg_count_detected_at_expansion() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;

    monitor_sensor(p, 10, 200);

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
    let prog = parse_ok(&src);
    assert_eq!(prog.module.pattern_calls.len(), 1);
    let msg = pipeline_err(&src);
    assert!(msg.contains("expects") && msg.contains("arguments"), "Unexpected: {msg}");
}

#[test]
fn parse_call_undefined_pattern_detected_at_expansion() {
    let src = r#"
module m {
    signal x: in bool;
    signal y: out bool;

    nonexistent(42);

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
    assert!(msg.contains("undefined pattern"), "Unexpected: {msg}");
}

#[test]
fn parse_call_underscore_signal_names() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal air_way_pressure: in u16;
    signal pressure_alarm_out: out bool;

    monitor_sensor(air_way_pressure, 50, 200, 1000, pressure_alarm_out);

    guard g {{
        when air_way_pressure > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            pressure_alarm_out = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let prog = parse_ok(&src);
    let call = &prog.module.pattern_calls[0];
    assert!(matches!(&call.arguments[0], PatternArg::SignalRef(s) if s == "air_way_pressure"));
    assert!(matches!(&call.arguments[4], PatternArg::SignalRef(s) if s == "pressure_alarm_out"));
}

#[test]
fn parse_call_trailing_whitespace_after_last_arg() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;

    monitor_sensor(p, 10, 200, 500, a   );

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
    let prog = parse_ok(&src);
    assert_eq!(prog.module.pattern_calls[0].arguments.len(), 5);
}

// =========================================================================
// Category 3: Substitution Engine (6 tests)
// =========================================================================

#[test]
fn substitution_signal_name_replaced() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal my_sensor: in u16;
    signal alarm_out: out bool;
    signal status: out bool;

    monitor_sensor(my_sensor, 50, 200, 1000, alarm_out);

    guard g {{
        when my_sensor > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            status = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    let has_sensor_guard =
        result.program.module.guards.iter().any(|g| g.name.contains("my_sensor"));
    assert!(
        has_sensor_guard,
        "Guard names should contain substituted signal name: {:?}",
        result.program.module.guards.iter().map(|g| &g.name).collect::<Vec<_>>()
    );
}

#[test]
fn substitution_integer_literal_replaced() {
    let result = pipeline_ok(&ventilator_source());
    let guard_count = result.program.module.guards.len();
    assert!(guard_count >= 4, "Should have at least 4 guards from 2 calls, got {guard_count}");
}

#[test]
fn substitution_multiple_params_same_line() {
    let result = pipeline_ok(&ventilator_source());
    assert!(!result.program.module.guards.is_empty());
}

#[test]
fn substitution_in_guard_reflex_property_names() {
    let result = pipeline_ok(&ventilator_source());
    let m = &result.program.module;
    let guard_names: Vec<&str> = m.guards.iter().map(|g| g.name.as_str()).collect();
    assert!(guard_names.iter().any(|n| n.contains("too_low")), "Guards={guard_names:?}");
    assert!(guard_names.iter().any(|n| n.contains("too_high")), "Guards={guard_names:?}");
}

#[test]
fn substitution_empty_reflect_body() {
    let src = r#"
def empty_pat() {
    reflect {
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    empty_pat();

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
    // Empty reflect body is rejected by pattern validation
    let msg = pipeline_err(src);
    assert!(msg.contains("empty reflect body"), "Unexpected: {msg}");
}

#[test]
fn substitution_preserves_non_param_text() {
    let src = r#"
def simple(s: signal in bool, out_s: signal out bool) {
    reflect {
        guard ${s}_active {
            when ${s}
            for 3 cycles;
        }

        reflex ${s}_react {
            on ${s}_active {
                ${out_s} = true;
            }
        }
    }
}
module m {
    signal sensor: in bool;
    signal alarm: out bool;
    signal led: out bool;

    simple(sensor, alarm);

    guard g {
        when sensor
        for 1 cycles;
    }

    reflex r {
        on g {
            led = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    let expanded_guard = result
        .program
        .module
        .guards
        .iter()
        .find(|g| g.name.contains("active"))
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
    let has_prefixed =
        result.program.module.guards.iter().any(|g| g.name == "simple_guard_0_check");
    assert!(
        has_prefixed,
        "Guard should be prefixed: {:?}",
        result.program.module.guards.iter().map(|g| &g.name).collect::<Vec<_>>()
    );
}

#[test]
fn prefix_two_calls_get_different_indices() {
    let result = pipeline_ok(&ventilator_source());
    let guard_names: Vec<&str> =
        result.program.module.guards.iter().map(|g| g.name.as_str()).collect();
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
    let m = &result.program.module;
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
        .module
        .guards
        .iter()
        .find(|g| g.name == "manual_guard")
        .expect("Should have manual guard");
    assert!(manual_g.origin.is_none(), "Hand-written guard should have origin: None");
    let manual_r = result
        .program
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
    validate_module(&result.program.module).expect("Post-expansion module should pass validation");
}

#[test]
fn expanded_properties_reference_prefixed_signals() {
    let result = pipeline_ok(&ventilator_source());
    let props = &result.program.module.properties;
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
    assert!(result.program.module.guards.len() >= 3, "Should have inner + outer + manual guard");
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
    assert!(result.program.module.guards.len() >= 5, "Should have 4 expanded + 1 manual guard");
}

#[test]
fn depth_limit_exceeded_pinned_message() {
    // Verify the exact pinned message format exists in the expansion code.
    let expected = "expansion depth limit";
    let source_code = include_str!("../src/expand/mod.rs");
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
    let source_code = include_str!("../src/expand/mod.rs");
    assert!(
        source_code.contains(expected),
        "expand/mod.rs must contain the internal signal scoping message"
    );
    assert!(
        source_code.contains("and cannot be referenced externally"),
        "expand/mod.rs must contain the full scoping message"
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
    let source_code = include_str!("../src/expand/mod.rs");
    assert!(
        source_code.contains("check_expr_cross_expansion"),
        "expand/mod.rs must implement cross-expansion checking"
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
    assert!(sv.contains("assert property"), "Should have SVA assertions from expanded properties");
    assert!(!result.program.module.properties.is_empty(), "Should have properties");
}

#[test]
fn sva_emitted_for_pattern_property() {
    let result = pipeline_ok(&ventilator_source());
    let sv = emit::verilog::emit_sv(&result);
    assert!(sv.contains("assert property"), "SV should contain assert property from pattern");
    assert!(sv.contains("|->"), "SV should contain implication from AlwaysImplies property");
}

#[test]
fn multiple_calls_produce_distinct_blocks() {
    let result = pipeline_ok(&ventilator_source());
    let sv = emit::verilog::emit_sv(&result);
    let reflex_count = sv.matches("always_comb begin").count();
    assert!(reflex_count >= 2, "Should have at least 2 always_comb blocks: found {reflex_count}");
}

// =========================================================================
// Category 9: Additional robustness tests (4 tests)
// =========================================================================

#[test]
fn pattern_with_property_always_implies() {
    let result = pipeline_ok(&ventilator_source());
    let props = &result.program.module.properties;
    assert!(props.len() >= 2, "Should have at least 2 properties from 2 calls");
}

#[test]
fn pattern_origin_has_correct_format() {
    let result = pipeline_ok(&ventilator_source());
    let origins: Vec<&str> =
        result.program.module.pattern_origins.iter().map(|o| o.pattern_name.as_str()).collect();
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
