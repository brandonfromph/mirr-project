//! Phase 7b coverage gap tests — 60 tests for untested error paths,
//! boundary conditions, and emission code paths.
//!
//! This file covers:
//!   A. Parser error paths (16 tests)
//!   B. Semantic validation error paths (4 tests)
//!   C. Expander error paths (12 tests)
//!   D. Expression renaming edge cases (6 tests)
//!   E. Emission coverage (8 tests)
//!   F. Pipeline integration edge cases (6 tests)
//!   G. is_pattern_call_line edge cases (4 tests)
//!   H. Serde backward compatibility (4 tests)
//!
//! Zero modifications to any `src/` file.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::pattern::{PatternDef, PatternParam, PatternParamKind, ReflectBlock};
use nasa_rust_project::ast::program::Guard;
use nasa_rust_project::ast::types::{LiteralValue, SignalType};
use nasa_rust_project::emit;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::validation::validate_pattern_defs;

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

/// Minimal source with monitor_sensor pattern.
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

/// Minimal module footer.
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

// =========================================================================
// Section A: Parser error paths (16 tests)
// =========================================================================

#[test]
fn parse_param_missing_colon() {
    let src = format!(
        r#"
def bad(sensor signal in u16) {{
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
    let msg = parse_err(&src);
    assert!(msg.contains("missing ':'"), "Expected 'missing :' error, got: {msg}");
}

#[test]
fn parse_param_empty_name() {
    let src = format!(
        r#"
def bad( : u16) {{
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
    let msg = parse_err(&src);
    assert!(msg.contains("empty name"), "Expected 'empty name' error, got: {msg}");
}

#[test]
fn parse_signal_param_missing_direction() {
    let src = format!(
        r#"
def bad(s: signal) {{
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
    let msg = parse_err(&src);
    // MEGA-1 tokenizer wraps the error through E413; underlying error is E112.
    assert!(
        msg.contains("Signal kind") || msg.contains("missing"),
        "Expected 'missing direction' error, got: {msg}"
    );
}

#[test]
fn parse_signal_param_unknown_kind() {
    let src = format!(
        r#"
def bad(s: signal inout u16) {{
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
    let msg = parse_err(&src);
    // MEGA-1 tokenizer wraps the error through E413; underlying error is E115.
    assert!(
        msg.contains("Unknown signal kind") || msg.contains("inout"),
        "Expected 'unknown signal kind' error, got: {msg}"
    );
}

#[test]
fn parse_signal_param_missing_type() {
    let src = format!(
        r#"
def bad(s: signal in) {{
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
    let msg = parse_err(&src);
    // MEGA-1 tokenizer wraps the error through E413; underlying error is E173.
    assert!(
        msg.contains("Missing base type") || msg.contains("missing"),
        "Expected 'missing type' error, got: {msg}"
    );
}

#[test]
fn parse_invalid_unsigned_width() {
    let src = format!(
        r#"
def bad(x: uABC) {{
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
    let msg = parse_err(&src);
    // MEGA-1 tokenizer wraps the error through E417; underlying error is E118.
    assert!(
        msg.contains("Unknown signal type") || msg.contains("uABC"),
        "Expected 'unknown type' error, got: {msg}"
    );
}

#[test]
fn parse_too_many_params_exceeds_32() {
    // Build a param list with 33 parameters.
    let params: Vec<String> = (0..33).map(|i| format!("p{i}: u16")).collect();
    let param_str = params.join(", ");

    let src = format!(
        r#"
def toomany({param_str}) {{
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
    let msg = parse_err(&src);
    assert!(
        msg.contains("too many parameters"),
        "Expected 'too many parameters' error, got: {msg}"
    );
}

#[test]
fn parse_call_too_many_args_exceeds_32() {
    let src = format!(
        r#"
def small(x: u16) {{
    reflect {{
        guard g {{
            when true
            for 1 cycles;
        }}
    }}
}}
module m {{
    signal x: in bool;
    signal y: out bool;

    small({args});

    guard g {{
        when x
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            y = true;
        }}
    }}
}}
"#,
        args = (0..33).map(|i| i.to_string()).collect::<Vec<_>>().join(", ")
    );
    let msg = parse_err(&src);
    assert!(msg.contains("too many arguments"), "Expected 'too many arguments' error, got: {msg}");
}

#[test]
fn parse_call_empty_arg_after_comma() {
    let src = r#"
def p(x: u16, y: u16) {
    reflect {
        guard g {
            when true
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    p(42, );

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
    let msg = parse_err(&src);
    assert!(msg.contains("empty argument"), "Expected 'empty argument' error, got: {msg}");
}

#[test]
fn parse_reflect_brace_depth_exceeds_16() {
    // Build nested braces: reflect { { { ... (17 levels) ... } } }
    let open_braces = "{".repeat(17);
    let close_braces = "}".repeat(17);
    let src = format!(
        r#"
def deep() {{
    reflect {{
        {open_braces}
        {close_braces}
    }}
}}
{MOD_FOOTER}
"#
    );
    let msg = parse_err(&src);
    assert!(
        msg.contains("exceeds maximum brace depth"),
        "Expected 'exceeds maximum brace depth' error, got: {msg}"
    );
}

#[test]
fn parse_reflect_unclosed_body() {
    // Reflect body that never closes.
    let src = format!(
        r#"
def unclosed() {{
    reflect {{
        guard g {{
            when true
            for 1 cycles;
{MOD_FOOTER}
"#
    );
    let msg = parse_err(&src);
    assert!(msg.contains("not closed with '}'"), "Expected 'not closed' error, got: {msg}");
}

#[test]
fn parse_def_header_not_closed() {
    // Header that never closes with `) {`.
    let src = format!(
        r#"
def broken(x: u16
{MOD_FOOTER}
"#
    );
    let msg = parse_err(&src);
    assert!(
        msg.contains("not closed with ') {'") || msg.contains("missing closing ')'"),
        "Expected header closure error, got: {msg}"
    );
}

#[test]
fn parse_def_missing_open_paren() {
    let src = format!(
        r#"
def noparen {{
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
    let msg = parse_err(&src);
    // `collect_def_header` sees `noparen {{` which contains `{` but no `)`,
    // so it either reports "missing '('" or "header not closed" depending on path.
    assert!(
        msg.contains("missing '('") || msg.contains("not closed with ') {'"),
        "Expected paren or header error, got: {msg}"
    );
}

#[test]
fn parse_def_missing_close_paren() {
    let src = format!(
        r#"
def noclose(x: u16 {{
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
    let msg = parse_err(&src);
    // `collect_def_header` joins `noclose(x: u16 {{` and sees `{` but no `) {` pattern,
    // so it may report "not closed" or "missing closing ')'" depending on path.
    assert!(
        msg.contains("missing closing ')'") || msg.contains("not closed with ') {'"),
        "Expected close paren or header error, got: {msg}"
    );
}

#[test]
fn parse_multiline_def_header() {
    // Header spanning multiple lines.
    let src = format!(
        r#"
def multiline(
    a: u16,
    b: u16
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
    assert_eq!(prog.patterns.len(), 1);
    assert_eq!(prog.patterns[0].name, "multiline");
    assert_eq!(prog.patterns[0].params.len(), 2);
}

#[test]
fn parse_reflect_brace_on_next_line() {
    // `reflect` keyword then `{` on the next line.
    let src = format!(
        r#"
def next_brace() {{
    reflect
    {{
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
    assert_eq!(prog.patterns[0].name, "next_brace");
    assert!(!prog.patterns[0].body.raw_lines.is_empty());
}

// =========================================================================
// Section B: Semantic validation error paths (4 tests)
// =========================================================================

#[test]
fn validate_duplicate_param_name() {
    let defs = vec![PatternDef {
        name: "dup_params".to_string(),
        params: vec![
            PatternParam {
                name: "x".to_string(),
                kind: PatternParamKind::Constant {
                    ty: SignalType::Unsigned(16),
                    annotations: Default::default(),
                },
            },
            PatternParam {
                name: "x".to_string(),
                kind: PatternParamKind::Constant {
                    ty: SignalType::Unsigned(16),
                    annotations: Default::default(),
                },
            },
        ],
        body: ReflectBlock { raw_lines: vec!["guard g { when true for 1 cycles; }".to_string()] },
        span: None,
    }];
    let err = validate_pattern_defs(&defs).unwrap_err().errors[0].to_string();
    assert!(
        err.contains("duplicate parameter name"),
        "Expected 'duplicate parameter name', got: {err}"
    );
}

#[test]
fn validate_too_many_params_semantic() {
    let params: Vec<PatternParam> = (0..33)
        .map(|i| PatternParam {
            name: format!("p{i}"),
            kind: PatternParamKind::Constant {
                ty: SignalType::Unsigned(16),
                annotations: Default::default(),
            },
        })
        .collect();

    let defs = vec![PatternDef {
        name: "big".to_string(),
        params,
        body: ReflectBlock { raw_lines: vec!["guard g { when true for 1 cycles; }".to_string()] },
        span: None,
    }];
    let err = validate_pattern_defs(&defs).unwrap_err().errors[0].to_string();
    assert!(err.contains("33 parameters (max 32)"), "Expected params count error, got: {err}");
}

#[test]
fn validate_body_too_many_lines() {
    let raw_lines: Vec<String> = (0..513).map(|i| format!("// line {i}")).collect();

    let defs = vec![PatternDef {
        name: "big_body".to_string(),
        params: vec![],
        body: ReflectBlock { raw_lines },
        span: None,
    }];
    let err = validate_pattern_defs(&defs).unwrap_err().errors[0].to_string();
    assert!(err.contains("513 lines (max 512)"), "Expected body line count error, got: {err}");
}

#[test]
fn validate_pattern_defs_called_from_pipeline() {
    // A program with a duplicate-named pattern. The pipeline should catch it
    // via validate_pattern_defs BEFORE expansion.
    let src = r#"
def dup(x: u16) {
    reflect {
        guard g1 {
            when true
            for 1 cycles;
        }
    }
}

def dup(y: u16) {
    reflect {
        guard g2 {
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
    let msg = pipeline_err(src);
    assert!(
        msg.contains("Duplicate pattern definition"),
        "Expected pipeline to catch duplicate via validate_pattern_defs, got: {msg}"
    );
}

// =========================================================================
// Section C: Expander error paths (12 tests)
// =========================================================================

#[test]
fn expand_max_items_exceeded() {
    // Create a pattern that expands into many items, call it enough times to exceed 256.
    // Each monitor_sensor call expands into 1 signal + 2 guards + 2 reflexes + 1 property = 6 items.
    // 43 calls * 6 = 258 > 256.
    let mut calls = String::new();
    let mut signals = String::new();
    for i in 0..43 {
        signals.push_str(&format!("    signal s{i}: in u16;\n"));
        signals.push_str(&format!("    signal a{i}: out bool;\n"));
        calls.push_str(&format!("    monitor_sensor(s{i}, 10, 200, 500, a{i});\n"));
    }
    let src = format!(
        r#"
{monitor}
module m {{
{signals}
    signal dummy: in bool;
    signal dummy_out: out bool;

{calls}
    guard g {{
        when dummy
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            dummy_out = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let msg = pipeline_err(&src);
    assert!(msg.contains("exceeds maximum"), "Expected 'exceeds maximum' error, got: {msg}");
}

#[test]
fn expand_input_signal_in_reflect_rejected() {
    let src = r#"
def bad_input() {
    reflect {
        signal illegal: in bool;

        guard g {
            when illegal
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    bad_input();

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
    assert!(msg.contains("input signal"), "Expected 'input signal' rejection, got: {msg}");
}

#[test]
fn expand_output_signal_in_reflect_rejected() {
    let src = r#"
def bad_output() {
    reflect {
        signal illegal: out bool;

        guard g {
            when true
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    bad_output();

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
    assert!(msg.contains("output signal"), "Expected 'output signal' rejection, got: {msg}");
}

#[test]
fn expand_constant_param_receives_signal_ref_error() {
    let src = r#"
def needs_const(v: u16) {
    reflect {
        guard g {
            when true
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    needs_const(x);

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
    assert!(
        msg.contains("expects a constant, got a signal reference"),
        "Expected const/signal mismatch, got: {msg}"
    );
}

#[test]
fn scope_handwritten_guard_refs_pattern_internal() {
    let src = r#"
def make_int(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_check {
                ${s}_internal = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    make_int(x);

    guard bad {
        when make_int_0_x_internal
        for 1 cycles;
    }

    reflex r {
        on bad {
            y = true;
        }
    }
}
"#;
    let msg = pipeline_err(src);
    assert!(
        msg.contains("is internal to pattern") && msg.contains("cannot be referenced externally"),
        "Expected scoping error, got: {msg}"
    );
}

#[test]
fn scope_handwritten_reflex_assigns_pattern_internal() {
    let src = r#"
def make_int(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_check {
                ${s}_internal = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    make_int(x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex bad {
        on g {
            make_int_0_x_internal = true;
        }
    }
}
"#;
    let msg = pipeline_err(src);
    assert!(
        msg.contains("is internal to pattern") && msg.contains("cannot be referenced externally"),
        "Expected scoping error, got: {msg}"
    );
}

#[test]
fn scope_handwritten_reflex_rhs_refs_pattern_internal() {
    let src = r#"
def make_int(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_check {
                ${s}_internal = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    make_int(x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex bad {
        on g {
            y = make_int_0_x_internal;
        }
    }
}
"#;
    let msg = pipeline_err(src);
    assert!(
        msg.contains("is internal to pattern") && msg.contains("cannot be referenced externally"),
        "Expected scoping error, got: {msg}"
    );
}

#[test]
fn scope_handwritten_property_refs_pattern_internal() {
    let src = r#"
def make_int(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_check {
                ${s}_internal = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    make_int(x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property bad_prop {
        always (make_int_0_x_internal);
    }
}
"#;
    let msg = pipeline_err(src);
    assert!(
        msg.contains("is internal to pattern") && msg.contains("cannot be referenced externally"),
        "Expected scoping error, got: {msg}"
    );
}

#[test]
fn scope_cross_expansion_guard_refs_other_internal() {
    // Two expansions of the same pattern: expansion A's guard should NOT
    // be able to reference expansion B's internal signal.
    // Since both expansions come from the same pattern, we need two different
    // patterns to create a cross-expansion reference scenario.
    // We'll construct the scenario by having the referenced name match
    // another expansion's prefixed internal signal.
    let src = r#"
def alpha(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_check {
                ${s}_internal = true;
            }
        }
    }
}
def beta(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_cross_check {
            when alpha_0_x_internal
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_cross_check {
                ${s}_internal = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    alpha(x);
    beta(x);

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
    assert!(
        msg.contains("is internal to pattern") || msg.contains("undeclared"),
        "Expected scoping or undeclared error, got: {msg}"
    );
}

#[test]
fn scope_cross_expansion_reflex_target_other_internal() {
    let src = r#"
def alpha(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_check {
                ${s}_internal = true;
            }
        }
    }
}
def beta(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check2 {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_cross_set {
            on ${s}_check2 {
                alpha_0_x_internal = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    alpha(x);
    beta(x);

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
    assert!(
        msg.contains("is internal to pattern") || msg.contains("undeclared"),
        "Expected scoping or undeclared error, got: {msg}"
    );
}

#[test]
fn scope_cross_expansion_reflex_rhs_other_internal() {
    let src = r#"
def alpha(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_check {
                ${s}_internal = true;
            }
        }
    }
}
def beta(s: signal in bool, out: signal out bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check2 {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_cross_rhs {
            on ${s}_check2 {
                ${out} = alpha_0_x_internal;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    alpha(x);
    beta(x, y);

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
    assert!(
        msg.contains("is internal to pattern") || msg.contains("undeclared"),
        "Expected scoping or undeclared error, got: {msg}"
    );
}

#[test]
fn scope_property_never_refs_pattern_internal() {
    let src = r#"
def make_int(s: signal in bool) {
    reflect {
        signal ${s}_internal: internal bool;

        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_set {
            on ${s}_check {
                ${s}_internal = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    make_int(x);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property bad_never {
        never (make_int_0_x_internal);
    }
}
"#;
    let msg = pipeline_err(src);
    assert!(
        msg.contains("is internal to pattern") && msg.contains("cannot be referenced externally"),
        "Expected scoping error for never property, got: {msg}"
    );
}

// =========================================================================
// Section D: Expression renaming edge cases (6 tests)
// =========================================================================

#[test]
fn rename_prev_expr_in_reflect() {
    // `prev()` is not parseable MIRR syntax, so we test renaming via
    // the rename_expr_signals function using a programmatic Expr::Prev node.
    // Instead, test that a pattern with a signal references in a Unary expression
    // properly gets renamed.
    let src = r#"
def with_ref(s: signal in bool) {
    reflect {
        signal ${s}_state: internal bool;

        guard ${s}_check {
            when !${s}
            for 1 cycles;
        }

        reflex ${s}_act {
            on ${s}_check {
                ${s}_state = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    with_ref(x);

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
    // The guard name should be prefixed.
    let guard = result
        .program
        .module
        .guards
        .iter()
        .find(|g| g.name.contains("check"))
        .expect("Should find check guard");
    assert!(guard.name.starts_with("with_ref_0_"), "Guard should be prefixed: {}", guard.name);
    // The internal signal should be prefixed.
    let int_sig = result
        .program
        .module
        .signals
        .iter()
        .find(|s| s.name.contains("state"))
        .expect("Should find state signal");
    assert!(
        int_sig.name.starts_with("with_ref_0_"),
        "Internal signal should be prefixed: {}",
        int_sig.name
    );
}

#[test]
fn rename_unary_not_expr_in_reflect() {
    let src = r#"
def with_not(s: signal in bool) {
    reflect {
        guard ${s}_neg {
            when !${s}
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    with_not(x);

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
    let negated_guard = result.program.module.guards.iter().find(|g| g.name.contains("neg"));
    assert!(negated_guard.is_some(), "Should have negated guard from pattern");
}

#[test]
fn rename_property_always_formula() {
    let src = r#"
def with_always(s: signal in bool, out: signal out bool) {
    reflect {
        guard ${s}_g {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_r {
            on ${s}_g {
                ${out} = true;
            }
        }

        property ${s}_prop {
            always (${s});
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;
    signal z: out bool;

    with_always(x, y);

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            z = true;
        }
    }
}
"#;
    let result = pipeline_ok(src);
    let prop = result.program.module.properties.iter().find(|p| p.name.contains("prop"));
    assert!(prop.is_some(), "Should have always property from pattern");
    assert!(prop.unwrap().origin.is_some(), "Property should have origin");
}

#[test]
fn rename_property_never_formula() {
    let src = r#"
def with_never(s: signal in bool) {
    reflect {
        guard ${s}_g {
            when ${s}
            for 1 cycles;
        }

        property ${s}_never_prop {
            never (${s});
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    with_never(x);

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
    let prop = result.program.module.properties.iter().find(|p| p.name.contains("never_prop"));
    assert!(prop.is_some(), "Should have never property from pattern");
}

#[test]
fn rename_deeply_nested_binary_expr() {
    let src = r#"
def deep_expr(a: signal in u16, b: signal in u16) {
    reflect {
        guard deep_cond {
            when (${a} > 10) && (${b} < 100)
            for 1 cycles;
        }
    }
}
module m {
    signal x: in u16;
    signal y: in u16;
    signal z: out bool;

    deep_expr(x, y);

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            z = true;
        }
    }
}
"#;
    // Skip temporal since complex guard conditions may not lower to hardware.
    let config = PipelineConfig { temporal: false, ..PipelineConfig::default() };
    let result = run_pipeline(src, &config).unwrap_or_else(|e| panic!("Pipeline failed: {e}"));
    let deep_guard = result.program.module.guards.iter().find(|g| g.name.contains("deep_cond"));
    assert!(deep_guard.is_some(), "Should have deeply nested guard");
}

#[test]
fn rename_does_not_touch_external_signal_refs() {
    // External signals passed as parameters should NOT be renamed with prefix.
    let src = r#"
def ext_ref(s: signal in bool, out: signal out bool) {
    reflect {
        guard ${s}_g {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_r {
            on ${s}_g {
                ${out} = true;
            }
        }
    }
}
module m {
    signal sensor: in bool;
    signal alarm: out bool;
    signal led: out bool;

    ext_ref(sensor, alarm);

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
    let sv = emit::verilog::emit_sv(&result);
    // The external signal 'sensor' should appear unrenamed in SV output.
    assert!(sv.contains("sensor"), "External signal 'sensor' should be in SV output");
    assert!(sv.contains("alarm"), "External signal 'alarm' should be in SV output");
}

// =========================================================================
// Section E: Emission coverage (8 tests)
// =========================================================================

#[test]
fn verilog_origin_on_internal_signal() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    let sv = emit::verilog::emit_sv(&result);
    // The internal signal should have a Pattern origin comment.
    assert!(
        sv.contains("// Pattern: monitor_sensor_0"),
        "SV should have origin comment on internal signal: {sv}"
    );
    // Verify internal signal declaration follows the comment.
    assert!(sv.contains("Internal signals"), "SV should have internal signals section");
}

#[test]
fn verilog_origin_on_property_assertion() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    let sv = emit::verilog::emit_sv(&result);
    // Find the property assertion section — it should have a Pattern origin comment.
    let after_safety = sv.split("Safety Properties").last().unwrap_or("");
    assert!(
        after_safety.contains("// Pattern: monitor_sensor_0"),
        "SVA section should have origin comment: {sv}"
    );
}

#[test]
fn verilog_origin_on_reflex_block() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    let sv = emit::verilog::emit_sv(&result);
    // Reflex section should have pattern origin comment.
    let after_reflex = sv.split("Reflex Assignments").last().unwrap_or("");
    assert!(
        after_reflex.contains("// Pattern: monitor_sensor_0"),
        "Reflex section should have origin comment: {sv}"
    );
}

#[test]
fn verilog_no_origin_on_handwritten_reflex() {
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
    let result = pipeline_ok(src);
    let sv = emit::verilog::emit_sv(&result);
    let reflex_section = sv.split("Reflex Assignments").last().unwrap_or("");
    assert!(
        !reflex_section.contains("// Pattern:"),
        "Hand-written reflex should NOT have pattern origin comment"
    );
}

#[test]
fn dot_guard_tooltip_has_pattern_origin() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    let dot = emit::dot::emit_module_dot(&result);
    assert!(
        dot.contains("tooltip=\"Pattern:"),
        "DOT guard should have tooltip with Pattern origin: {dot}"
    );
}

#[test]
fn dot_signal_tooltip_has_pattern_origin() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    let dot = emit::dot::emit_module_dot(&result);
    // The internal signal should have a tooltip.
    let signal_section = dot.split("// Signals").last().unwrap_or("");
    assert!(
        signal_section.contains("tooltip=\"Pattern:"),
        "DOT signal should have tooltip with Pattern origin: {dot}"
    );
}

#[test]
fn json_module_guard_has_origin_field() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    let json = emit::json_netlist::emit_json(&result).expect("JSON should serialize");
    // The expanded guards should have "origin" in the JSON.
    assert!(
        json.contains("\"origin\""),
        "JSON should include origin field for expanded guards: {json}"
    );
}

#[test]
fn json_module_guard_no_origin_when_none() {
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
    let result = pipeline_ok(src);
    let json = emit::json_netlist::emit_json(&result).expect("JSON should serialize");
    // With skip_serializing_if, guards with origin:None should NOT have "origin" key.
    assert!(
        !json.contains("\"origin\""),
        "JSON should NOT have origin field when no patterns used: {json}"
    );
}

// =========================================================================
// Section F: Pipeline integration edge cases (6 tests)
// =========================================================================

#[test]
fn pattern_expansion_interacts_with_simplifier() {
    // Expanded guard with `true && x` should simplify to just `x`.
    let src = r#"
def simplifiable(s: signal in bool) {
    reflect {
        guard ${s}_simplified {
            when true && ${s}
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;

    simplifiable(x);

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
    // Simplifier should have applied at least 1 rule.
    assert!(result.simplify_stats.is_some(), "Simplify should have run");
    let stats = result.simplify_stats.as_ref().unwrap();
    assert!(stats.rules_applied > 0, "Simplifier should have applied rules to `true && x`");
}

#[test]
fn pattern_expansion_interacts_with_width_inference() {
    // Expanded guard with u16 comparison should pass width inference.
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source(),
    );
    let result = pipeline_ok(&src);
    assert!(result.width_result.is_some(), "Width inference should have run");
    assert!(!result.has_width_errors(), "Width inference should have no errors");
}

#[test]
fn pattern_expansion_interacts_with_temporal_compiler() {
    // Expanded guards should compile to temporal netlist.
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    assert!(result.temporal_netlist.is_some(), "Temporal netlist should have been generated");
    let netlist = result.temporal_netlist.as_ref().unwrap();
    // Should have guards from both hand-written and expanded.
    assert!(
        netlist.guards.len() >= 3,
        "Should have at least 3 temporal guards, got {}",
        netlist.guards.len()
    );
}

#[test]
fn pattern_with_comparison_in_reflect_body() {
    // Test that comparison expressions work correctly inside reflect bodies.
    let src = r#"
def with_cmp(s: signal in u16, limit: u16) {
    reflect {
        guard ${s}_over {
            when ${s} > ${limit}
            for 2 cycles;
        }
    }
}
module m {
    signal x: in u16;
    signal y: out bool;

    with_cmp(x, 100);

    guard g {
        when x > 0
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
    let cmp_guard = result.program.module.guards.iter().find(|g| g.name.contains("over"));
    assert!(cmp_guard.is_some(), "Should have comparison-based guard from pattern");
}

#[test]
fn multiple_patterns_different_param_types() {
    let src = r#"
def bool_guard(s: signal in bool) {
    reflect {
        guard ${s}_active {
            when ${s}
            for 1 cycles;
        }
    }
}
def threshold(sensor: signal in u16, limit: u16) {
    reflect {
        guard ${sensor}_over {
            when ${sensor} > ${limit}
            for 2 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal p: in u16;
    signal y: out bool;

    bool_guard(x);
    threshold(p, 50);

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
        result.program.module.guards.len() >= 3,
        "Should have guards from both patterns plus hand-written"
    );
}

#[test]
fn pattern_origin_survives_full_pipeline() {
    let src = format!(
        r#"
{monitor}
module m {{
    signal p: in u16;
    signal a: out bool;
    signal b: out bool;

    monitor_sensor(p, 50, 200, 1000, a);

    guard g {{
        when p > 0
        for 1 cycles;
    }}

    reflex r {{
        on g {{
            b = true;
        }}
    }}
}}
"#,
        monitor = monitor_sensor_source()
    );
    let result = pipeline_ok(&src);
    // After simplify + width + temporal, origin tags should still be present.
    for guard in &result.program.module.guards {
        if guard.name.contains("monitor_sensor") {
            assert!(
                guard.origin.is_some(),
                "Origin should survive full pipeline for guard '{}'",
                guard.name
            );
        }
    }
    for reflex in &result.program.module.reflexes {
        if reflex.name.contains("monitor_sensor") {
            assert!(
                reflex.origin.is_some(),
                "Origin should survive full pipeline for reflex '{}'",
                reflex.name
            );
        }
    }
}

// =========================================================================
// Section G: is_pattern_call_line edge cases (4 tests)
// =========================================================================

#[test]
fn call_line_with_non_alnum_chars_rejected() {
    // Use the parser's behavior: an identifier with `-` or `.` should not
    // match as a pattern call.
    use nasa_rust_project::parser::pattern_parser::is_pattern_call_line;

    assert!(!is_pattern_call_line("my-func(x);"), "Hyphenated identifier should not be call");
    assert!(!is_pattern_call_line("my.func(x);"), "Dotted identifier should not be call");
}

#[test]
fn call_line_keyword_guard_not_treated_as_call() {
    use nasa_rust_project::parser::pattern_parser::is_pattern_call_line;

    assert!(!is_pattern_call_line("guard(x);"), "'guard' is a keyword, not a call");
}

#[test]
fn call_line_keyword_signal_not_treated_as_call() {
    use nasa_rust_project::parser::pattern_parser::is_pattern_call_line;

    assert!(!is_pattern_call_line("signal(x);"), "'signal' is a keyword, not a call");
}

#[test]
fn call_line_empty_identifier_rejected() {
    use nasa_rust_project::parser::pattern_parser::is_pattern_call_line;

    assert!(!is_pattern_call_line("(x);"), "Empty identifier should not be call");
}

// =========================================================================
// Section H: Serde backward compatibility (4 tests)
// =========================================================================

#[test]
fn guard_origin_none_serializes_without_field() {
    let guard = Guard {
        name: "g".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles: 1,
        origin: None,
        span: None,
    };
    let json = serde_json::to_string(&guard).unwrap();
    assert!(
        !json.contains("\"origin\""),
        "Guard with origin:None should not serialize origin field: {json}"
    );
}

#[test]
fn guard_origin_some_serializes_with_field() {
    let guard = Guard {
        name: "g".to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles: 1,
        origin: Some("test_origin_0".to_string()),
        span: None,
    };
    let json = serde_json::to_string(&guard).unwrap();
    assert!(
        json.contains("\"origin\""),
        "Guard with origin:Some should serialize origin field: {json}"
    );
    assert!(json.contains("test_origin_0"), "Origin value should be present: {json}");
}

#[test]
fn deserialize_guard_without_origin_defaults_none() {
    let json = r#"{"name":"g","condition":{"Literal":{"Bool":true}},"cycles":1}"#;
    let guard: Guard = serde_json::from_str(json).unwrap();
    assert!(guard.origin.is_none(), "Deserializing without origin should default to None");
}

#[test]
fn deserialize_program_without_patterns_field() {
    // A minimal program JSON without the "patterns" field should deserialize
    // with patterns defaulting to an empty vec.
    let json = r#"{
        "module": {
            "name": "m",
            "signals": [],
            "guards": [],
            "reflexes": [],
            "properties": [],
            "pattern_calls": [],
            "pattern_origins": []
        }
    }"#;
    let program: nasa_rust_project::MirrProgram = serde_json::from_str(json).unwrap();
    assert!(program.patterns.is_empty(), "Missing 'patterns' field should default to empty vec");
}
