use super::*;

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
        PatternParamKind::Signal { kind: SignalKind::Input, ty: SignalType::Unsigned(16), .. }
    ));
    assert!(matches!(
        params[1].kind,
        PatternParamKind::Signal { kind: SignalKind::Output, ty: SignalType::Bool, .. }
    ));
    assert!(matches!(
        params[2].kind,
        PatternParamKind::Constant { ty: SignalType::Unsigned(16), .. }
    ));
    assert!(matches!(
        params[3].kind,
        PatternParamKind::Constant { ty: SignalType::Unsigned(32), .. }
    ));
    assert!(matches!(params[4].kind, PatternParamKind::Constant { ty: SignalType::Bool, .. }));
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
    let body_guards = &prog.patterns[0].body.guards;
    assert!(!body_guards.is_empty(), "reflect body should have guards");
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
    assert!(
        expanded_guard.name.contains("sensor_active"),
        "Expanded guard name should be sensor_active"
    );
}
