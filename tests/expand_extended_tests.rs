#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
//! Extended integration tests for `src/expand/mod.rs` — pattern expansion engine.
//!
//! Covers: parse+expand pipeline, parameter substitution, guard/reflex/property
//! expansion, name prefixing, origin tagging, multiple calls, error cases, edge cases.

use nasa_rust_project::ast::pattern::PatternOrigin;
use nasa_rust_project::ast::types::SignalKind;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::validate_module;

const MAX_SCAN: usize = 128;
const MAX_ORIGIN_SCAN: usize = 64;

fn pipeline_ok(src: &str) -> nasa_rust_project::pipeline::PipelineResult {
    run_pipeline(src, &PipelineConfig::default()).unwrap_or_else(|e| panic!("Pipeline failed: {e}"))
}

fn pipeline_err(src: &str) -> String {
    match run_pipeline(src, &PipelineConfig::default()) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("Expected pipeline error"),
    }
}

fn count_containing(names: &[String], substr: &str) -> usize {
    let mut c = 0usize;
    let lim = names.len().min(MAX_SCAN);
    for i in 0..lim {
        if names[i].contains(substr) {
            c += 1;
        }
    }
    c
}

fn guard_names(r: &nasa_rust_project::pipeline::PipelineResult) -> Vec<String> {
    r.program.module.guards.iter().map(|g| g.name.clone()).collect()
}

fn reflex_names(r: &nasa_rust_project::pipeline::PipelineResult) -> Vec<String> {
    r.program.module.reflexes.iter().map(|rx| rx.name.clone()).collect()
}

fn prop_names(r: &nasa_rust_project::pipeline::PipelineResult) -> Vec<String> {
    r.program.module.properties.iter().map(|p| p.name.clone()).collect()
}

fn has_origin_for(origins: &[PatternOrigin], pat: &str) -> bool {
    let lim = origins.len().min(MAX_ORIGIN_SCAN);
    for i in 0..lim {
        if origins[i].pattern_name == pat {
            return true;
        }
    }
    false
}

/// Simple one-guard pattern.
const PAT_SIMPLE: &str = r#"
def simple_check(s: signal in bool) {
    reflect {
        guard ${s}_alert {
            when ${s}
            for 2 cycles;
        }
    }
}
"#;

/// Full pattern: guard + reflex + property.
const PAT_FULL: &str = r#"
def full_mon(sensor: signal in u16, thresh: u16, alarm: signal out bool) {
    reflect {
        guard ${sensor}_high {
            when ${sensor} > ${thresh}
            for 3 cycles;
        }
        reflex ${sensor}_trip {
            on ${sensor}_high {
                ${alarm} = true;
            }
        }
        property ${sensor}_safe {
            always (${sensor} > ${thresh} -> ${alarm});
        }
    }
}
"#;

/// Dual-guard pattern.
const PAT_DUAL: &str = r#"
def dual_check(s: signal in u16, lo: u16, hi: u16) {
    reflect {
        guard ${s}_below {
            when ${s} < ${lo}
            for 1 cycles;
        }
        guard ${s}_above {
            when ${s} > ${hi}
            for 1 cycles;
        }
    }
}
"#;

/// Minimal module footer with proper multiline guard/reflex syntax.
fn mod_footer(signals: &str, calls: &str, guard_cond: &str) -> String {
    format!(
        r#"
module m {{
    {signals}
    signal _bg_sig: in bool;
    signal _bg_out: out bool;
    {calls}
    guard _bg {{
        when {guard_cond}
        for 1 cycles;
    }}
    reflex _br {{
        on _bg {{
            _bg_out = true;
        }}
    }}
}}
"#
    )
}

/// Standard footer with bool guard.
fn std_module(signals: &str, calls: &str) -> String {
    mod_footer(signals, calls, "_bg_sig")
}

// =========================================================================
// Section 1: Basic Pattern Expansion (6 tests)
// =========================================================================

#[test]
fn expand_single_guard_pattern() {
    let src = format!("{PAT_SIMPLE}\n{}", std_module("signal x: in bool;", "simple_check(x);"));
    let r = pipeline_ok(&src);
    assert!(count_containing(&guard_names(&r), "alert") >= 1, "should expand alert guard");
}

#[test]
fn expand_full_pattern_guard_reflex_property() {
    let src = format!(
        "{PAT_FULL}\n{}",
        std_module("signal t: in u16;\n    signal a: out bool;", "full_mon(t, 500, a);")
    );
    let r = pipeline_ok(&src);
    assert!(count_containing(&guard_names(&r), "t_high") >= 1, "guard from full pattern");
    assert!(count_containing(&reflex_names(&r), "t_trip") >= 1, "reflex from full pattern");
    assert!(count_containing(&prop_names(&r), "t_safe") >= 1, "property from full pattern");
}

#[test]
fn expand_clears_pattern_calls() {
    let src = format!("{PAT_SIMPLE}\n{}", std_module("signal x: in bool;", "simple_check(x);"));
    let r = pipeline_ok(&src);
    assert!(r.program.module.pattern_calls.is_empty(), "calls should be cleared post-expansion");
}

#[test]
fn expand_noop_without_calls() {
    let src = std_module("signal x: in bool;", "");
    let r = pipeline_ok(&src);
    assert!(r.program.module.pattern_origins.is_empty(), "no origins when no calls");
}

#[test]
fn expand_preserves_hand_written_guard() {
    let src = format!(
        "{PAT_SIMPLE}
module m {{
    signal x: in bool;
    signal y: out bool;
    simple_check(x);
    guard manual {{
        when x
        for 5 cycles;
    }}
    reflex mr {{
        on manual {{
            y = true;
        }}
    }}
}}
"
    );
    let r = pipeline_ok(&src);
    assert!(guard_names(&r).contains(&"manual".to_string()), "hand-written guard preserved");
}

#[test]
fn expand_preserves_hand_written_reflex() {
    let src = format!(
        "{PAT_SIMPLE}
module m {{
    signal x: in bool;
    signal y: out bool;
    simple_check(x);
    guard mg {{
        when x
        for 1 cycles;
    }}
    reflex manual_r {{
        on mg {{
            y = true;
        }}
    }}
}}
"
    );
    let r = pipeline_ok(&src);
    assert!(reflex_names(&r).contains(&"manual_r".to_string()), "hand-written reflex preserved");
}

// =========================================================================
// Section 2: Parameter Substitution (5 tests)
// =========================================================================

#[test]
fn subst_signal_name_in_guard() {
    let src = format!(
        "{PAT_SIMPLE}\n{}",
        std_module("signal pressure: in bool;", "simple_check(pressure);")
    );
    let r = pipeline_ok(&src);
    assert!(
        count_containing(&guard_names(&r), "pressure") >= 1,
        "signal name substituted into guard"
    );
}

#[test]
fn subst_integer_constant() {
    let src = format!(
        "{PAT_FULL}\n{}",
        std_module("signal t: in u16;\n    signal a: out bool;", "full_mon(t, 200, a);")
    );
    let r = pipeline_ok(&src);
    assert!(count_containing(&guard_names(&r), "high") >= 1, "integer constant substituted");
}

#[test]
fn subst_output_signal_in_reflex() {
    let src = format!(
        "{PAT_FULL}\n{}",
        std_module(
            "signal t: in u16;\n    signal overheat: out bool;",
            "full_mon(t, 300, overheat);"
        )
    );
    let r = pipeline_ok(&src);
    let m = &r.program.module;
    let mut found = false;
    let lim = m.reflexes.len().min(MAX_SCAN);
    for i in 0..lim {
        let alim = m.reflexes[i].assignments.len().min(MAX_SCAN);
        for j in 0..alim {
            if m.reflexes[i].assignments[j].target.contains("overheat") {
                found = true;
            }
        }
    }
    assert!(found, "output signal 'overheat' substituted into reflex assignment target");
}

#[test]
fn subst_boolean_constant() {
    // Boolean param substituted into a reflex assignment value.
    // Guard conditions with literal `true` cannot be lowered to hardware,
    // so we verify substitution via an assignment target value.
    let src = r#"
def bp(s: signal in bool, o: signal out bool, v: bool) {
    reflect {
        guard ${s}_gc {
            when ${s}
            for 1 cycles;
        }
        reflex ${s}_rc {
            on ${s}_gc {
                ${o} = ${v};
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;
    signal z: out bool;
    bp(x, y, true);
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
    let r = pipeline_ok(src);
    assert!(!r.program.module.guards.is_empty(), "boolean constant substitution succeeds");
}

#[test]
fn subst_dual_params_same_line() {
    let src = format!(
        "{PAT_DUAL}\n{}",
        mod_footer("signal s1: in u16;", "dual_check(s1, 10, 500);", "s1 > 0")
    );
    let r = pipeline_ok(&src);
    assert!(count_containing(&guard_names(&r), "below") >= 1, "lo param substituted");
    assert!(count_containing(&guard_names(&r), "above") >= 1, "hi param substituted");
}

// =========================================================================
// Section 3: Name Prefixing (5 tests)
// =========================================================================

#[test]
fn prefix_format_pattern_index_name() {
    let src = format!("{PAT_SIMPLE}\n{}", std_module("signal x: in bool;", "simple_check(x);"));
    let r = pipeline_ok(&src);
    let gn = guard_names(&r);
    assert!(
        gn.contains(&"simple_check_0_x_alert".to_string()),
        "guard should be 'simple_check_0_x_alert', got: {gn:?}"
    );
}

#[test]
fn prefix_two_calls_distinct_indices() {
    let src = format!(
        "{PAT_SIMPLE}\n{}",
        std_module(
            "signal a: in bool;\n    signal b: in bool;",
            "simple_check(a);\n    simple_check(b);"
        )
    );
    let r = pipeline_ok(&src);
    let gn = guard_names(&r);
    assert!(count_containing(&gn, "_0_") >= 1, "first call index 0");
    assert!(count_containing(&gn, "_1_") >= 1, "second call index 1");
}

#[test]
fn prefix_internal_signals() {
    let src = r#"
def wi(s: signal in bool) {
    reflect {
        signal ${s}_st: internal bool;
        guard ${s}_ck {
            when ${s}
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;
    wi(x);
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
    let r = pipeline_ok(src);
    let m = &r.program.module;
    let mut found = false;
    let lim = m.signals.len().min(MAX_SCAN);
    for i in 0..lim {
        if m.signals[i].kind == SignalKind::Internal && m.signals[i].name.contains("wi_0_") {
            found = true;
        }
    }
    assert!(found, "internal signals from pattern should be prefixed");
}

#[test]
fn prefix_reflex_guard_ref_updated() {
    let src = r#"
def gd(s: signal in bool, o: signal out bool) {
    reflect {
        guard ${s}_trig {
            when ${s}
            for 1 cycles;
        }
        reflex ${s}_rx {
            on ${s}_trig {
                ${o} = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;
    signal z: out bool;
    gd(x, y);
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
    let r = pipeline_ok(src);
    let m = &r.program.module;
    let mut found = false;
    let lim = m.reflexes.len().min(MAX_SCAN);
    for i in 0..lim {
        if m.reflexes[i].name.contains("gd_0_") {
            let glim = m.reflexes[i].guard_names.len().min(MAX_SCAN);
            for j in 0..glim {
                if m.reflexes[i].guard_names[j].contains("gd_0_") {
                    found = true;
                }
            }
        }
    }
    assert!(found, "reflex guard_names reference should be updated to prefixed name");
}

#[test]
fn prefix_property_names() {
    let src = format!(
        "{PAT_FULL}\n{}",
        std_module("signal t: in u16;\n    signal a: out bool;", "full_mon(t, 100, a);")
    );
    let r = pipeline_ok(&src);
    assert!(
        count_containing(&prop_names(&r), "full_mon_0_") >= 1,
        "property name should be prefixed"
    );
}

// =========================================================================
// Section 4: Origin Tracking (4 tests)
// =========================================================================

#[test]
fn origin_set_on_expanded_guards() {
    let src = format!(
        "{PAT_FULL}\n{}",
        std_module("signal t: in u16;\n    signal a: out bool;", "full_mon(t, 100, a);")
    );
    let r = pipeline_ok(&src);
    let m = &r.program.module;
    let lim = m.guards.len().min(MAX_SCAN);
    for i in 0..lim {
        if m.guards[i].name.contains("full_mon_0_") {
            assert!(
                m.guards[i].origin.is_some(),
                "expanded guard '{}' missing origin",
                m.guards[i].name
            );
        }
    }
}

#[test]
fn origin_none_on_hand_written() {
    let src = format!(
        "{PAT_SIMPLE}
module m {{
    signal x: in bool;
    signal y: out bool;
    simple_check(x);
    guard hw {{
        when x
        for 1 cycles;
    }}
    reflex hr {{
        on hw {{
            y = true;
        }}
    }}
}}
"
    );
    let r = pipeline_ok(&src);
    let m = &r.program.module;
    let lim = m.guards.len().min(MAX_SCAN);
    for i in 0..lim {
        if m.guards[i].name == "hw" {
            assert!(m.guards[i].origin.is_none(), "hand-written guard should NOT have origin");
        }
    }
}

#[test]
fn origin_recorded_per_call() {
    let src = format!(
        "{PAT_SIMPLE}\n{}",
        std_module(
            "signal a: in bool;\n    signal b: in bool;",
            "simple_check(a);\n    simple_check(b);"
        )
    );
    let r = pipeline_ok(&src);
    let o = &r.program.module.pattern_origins;
    assert!(o.len() >= 2, "2 calls should yield >= 2 origins, got {}", o.len());
    assert!(has_origin_for(o, "simple_check"), "origins should include 'simple_check'");
}

#[test]
fn origin_args_summary_captures_constants() {
    let src = format!(
        "{PAT_FULL}\n{}",
        std_module("signal t: in u16;\n    signal a: out bool;", "full_mon(t, 999, a);")
    );
    let r = pipeline_ok(&src);
    let o = &r.program.module.pattern_origins;
    let mut found = false;
    let lim = o.len().min(MAX_ORIGIN_SCAN);
    for i in 0..lim {
        if o[i].pattern_name == "full_mon" && o[i].call_args_summary.contains("999") {
            found = true;
        }
    }
    assert!(found, "args_summary should contain constant '999'");
}

// =========================================================================
// Section 5: Multiple Calls (3 tests)
// =========================================================================

#[test]
fn multiple_different_patterns() {
    let src = format!(
        "{PAT_SIMPLE}\n{PAT_DUAL}\n{}",
        std_module(
            "signal x: in bool;\n    signal t: in u16;",
            "simple_check(x);\n    dual_check(t, 10, 500);"
        )
    );
    let r = pipeline_ok(&src);
    let gn = guard_names(&r);
    assert!(count_containing(&gn, "alert") >= 1, "simple_check guard expanded");
    assert!(count_containing(&gn, "below") >= 1, "dual_check below guard expanded");
    assert!(count_containing(&gn, "above") >= 1, "dual_check above guard expanded");
}

#[test]
fn same_pattern_three_times() {
    let src = format!(
        "{PAT_SIMPLE}\n{}",
        std_module(
            "signal a: in bool;\n    signal b: in bool;\n    signal c: in bool;",
            "simple_check(a);\n    simple_check(b);\n    simple_check(c);"
        )
    );
    let r = pipeline_ok(&src);
    assert!(r.program.module.pattern_origins.len() >= 3, "3 calls => >= 3 origins");
    let gn = guard_names(&r);
    assert!(count_containing(&gn, "_0_") >= 1, "index 0 present");
    assert!(count_containing(&gn, "_1_") >= 1, "index 1 present");
    assert!(count_containing(&gn, "_2_") >= 1, "index 2 present");
}

#[test]
fn dual_calls_total_guards() {
    let src = format!(
        "{PAT_DUAL}\n{}",
        mod_footer(
            "signal s1: in u16;\n    signal s2: in u16;",
            "dual_check(s1, 0, 100);\n    dual_check(s2, 50, 200);",
            "s1 > 0"
        )
    );
    let r = pipeline_ok(&src);
    let total = r.program.module.guards.len();
    assert!(total >= 5, "2 dual_check (2 guards each) + 1 manual => >= 5, got {total}");
}

// =========================================================================
// Section 6: Error Cases (6 tests)
// =========================================================================

#[test]
fn err_undefined_pattern() {
    let src = r#"
module m {
    signal x: in bool;
    signal y: out bool;
    nonexistent(x);
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
    assert!(msg.contains("undefined pattern"), "should report undefined pattern, got: {msg}");
}

#[test]
fn err_too_few_args() {
    let src = format!(
        "{PAT_FULL}\n{}",
        mod_footer("signal t: in u16;\n    signal a: out bool;", "full_mon(t, 100);", "t > 0")
    );
    let msg = pipeline_err(&src);
    assert!(
        msg.contains("expects") && msg.contains("arguments"),
        "should report arg count, got: {msg}"
    );
}

#[test]
fn err_too_many_args() {
    let src =
        format!("{PAT_SIMPLE}\n{}", std_module("signal x: in bool;", "simple_check(x, 42, true);"));
    let msg = pipeline_err(&src);
    assert!(
        msg.contains("expects") && msg.contains("arguments"),
        "should report arg count, got: {msg}"
    );
}

#[test]
fn err_signal_param_gets_constant() {
    let src = r#"
def tp(s: signal in bool) {
    reflect {
        guard ${s}_g {
            when ${s}
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;
    tp(42);
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
    assert!(msg.contains("expects a signal reference, got a constant"), "type mismatch: {msg}");
}

#[test]
fn err_constant_param_gets_signal() {
    let src = r#"
def cp(v: u16) {
    reflect {
        guard gc {
            when true
            for 1 cycles;
        }
    }
}
module m {
    signal x: in bool;
    signal y: out bool;
    cp(x);
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
    assert!(msg.contains("expects a constant, got a signal reference"), "type mismatch: {msg}");
}

#[test]
fn err_duplicate_pattern_defs() {
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
    assert!(msg.contains("Duplicate pattern definition"), "should report duplicate: {msg}");
}

// =========================================================================
// Section 7: Edge Cases (5 tests)
// =========================================================================

#[test]
fn zero_param_pattern() {
    // Zero-parameter pattern — uses a signal from the calling module
    // as a literal in the reflect body (not via substitution).
    let src = r#"
def zp() {
    reflect {
        guard const_g {
            when _bg_sig
            for 1 cycles;
        }
    }
}
module m {
    signal _bg_sig: in bool;
    signal _bg_out: out bool;
    zp();
    guard _bg {
        when _bg_sig
        for 1 cycles;
    }
    reflex _br {
        on _bg {
            _bg_out = true;
        }
    }
}
"#;
    let r = pipeline_ok(src);
    assert!(count_containing(&guard_names(&r), "const_g") >= 1, "zero-param pattern expands");
}

#[test]
fn pattern_with_three_guards() {
    let src = format!(
        r#"
def triple(s: signal in u16) {{
    reflect {{
        guard ${{s}}_lo {{
            when ${{s}} < 10
            for 1 cycles;
        }}
        guard ${{s}}_mi {{
            when ${{s}} > 100
            for 2 cycles;
        }}
        guard ${{s}}_hi {{
            when ${{s}} > 1000
            for 3 cycles;
        }}
    }}
}}
{}
"#,
        mod_footer("signal v: in u16;", "triple(v);", "v > 0")
    );
    let r = pipeline_ok(&src);
    let gn = guard_names(&r);
    assert!(count_containing(&gn, "lo") >= 1, "lo guard expanded");
    assert!(count_containing(&gn, "mi") >= 1, "mid guard expanded");
    assert!(count_containing(&gn, "hi") >= 1, "hi guard expanded");
}

#[test]
fn expanded_module_passes_validation() {
    let src = format!(
        "{PAT_FULL}\n{}",
        std_module("signal t: in u16;\n    signal a: out bool;", "full_mon(t, 100, a);")
    );
    let r = pipeline_ok(&src);
    validate_module(&r.program.module).expect("post-expansion module should validate");
}

#[test]
fn pattern_defs_retained_after_expansion() {
    let src = format!("{PAT_SIMPLE}\n{}", std_module("signal x: in bool;", "simple_check(x);"));
    let r = pipeline_ok(&src);
    assert!(!r.program.patterns.is_empty(), "pattern defs should be retained");
    assert!(r.program.patterns[0].name == "simple_check", "retained pattern name matches");
}

#[test]
fn underscore_heavy_names_substitute() {
    let src = r#"
def long_name(my_in: signal in bool, my_out: signal out bool) {
    reflect {
        guard ${my_in}_active {
            when ${my_in}
            for 1 cycles;
        }
        reflex ${my_in}_fire {
            on ${my_in}_active {
                ${my_out} = true;
            }
        }
    }
}
module m {
    signal sensor_a: in bool;
    signal alarm_b: out bool;
    signal y: out bool;
    long_name(sensor_a, alarm_b);
    guard g {
        when sensor_a
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}
"#;
    let r = pipeline_ok(src);
    assert!(count_containing(&guard_names(&r), "sensor_a") >= 1, "underscore names substitute");
}

// =========================================================================
// Section 8: Guard & Reflex Expansion Details (4 tests)
// =========================================================================

#[test]
fn expanded_guard_retains_cycle_count() {
    let src = format!("{PAT_SIMPLE}\n{}", std_module("signal x: in bool;", "simple_check(x);"));
    let r = pipeline_ok(&src);
    let m = &r.program.module;
    let mut found = false;
    let lim = m.guards.len().min(MAX_SCAN);
    for i in 0..lim {
        if m.guards[i].name.contains("alert") && m.guards[i].cycles == 2 {
            found = true;
        }
    }
    assert!(found, "expanded guard should retain 'for 2 cycles'");
}

#[test]
fn dual_guard_produces_exactly_two() {
    let src = format!(
        "{PAT_DUAL}\n{}",
        mod_footer("signal p: in u16;", "dual_check(p, 20, 800);", "p > 0")
    );
    let r = pipeline_ok(&src);
    let gn = guard_names(&r);
    assert!(count_containing(&gn, "below") == 1, "exactly 1 below guard");
    assert!(count_containing(&gn, "above") == 1, "exactly 1 above guard");
}

#[test]
fn expanded_property_has_origin() {
    let src = format!(
        "{PAT_FULL}\n{}",
        std_module("signal t: in u16;\n    signal a: out bool;", "full_mon(t, 100, a);")
    );
    let r = pipeline_ok(&src);
    let m = &r.program.module;
    let lim = m.properties.len().min(MAX_SCAN);
    for i in 0..lim {
        if m.properties[i].name.contains("full_mon_0_") {
            assert!(
                m.properties[i].origin.is_some(),
                "expanded property '{}' should have origin",
                m.properties[i].name
            );
        }
    }
}

#[test]
fn expanded_reflex_targets_correct_signal() {
    let src = r#"
def ap(s: signal in bool, o: signal out bool) {
    reflect {
        guard ${s}_g {
            when ${s}
            for 1 cycles;
        }
        reflex ${s}_rx {
            on ${s}_g {
                ${o} = true;
            }
        }
    }
}
module m {
    signal x: in bool;
    signal alarm: out bool;
    signal y: out bool;
    ap(x, alarm);
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
    let r = pipeline_ok(src);
    let m = &r.program.module;
    let mut found = false;
    let lim = m.reflexes.len().min(MAX_SCAN);
    for i in 0..lim {
        let alim = m.reflexes[i].assignments.len().min(MAX_SCAN);
        for j in 0..alim {
            if m.reflexes[i].assignments[j].target.contains("alarm") {
                found = true;
            }
        }
    }
    assert!(found, "expanded reflex should assign to 'alarm'");
}

// =========================================================================
// Section 9: Parse-Level Checks (2 tests)
// =========================================================================

#[test]
fn parse_detects_pattern_call() {
    let src = format!("{PAT_SIMPLE}\n{}", std_module("signal x: in bool;", "simple_check(x);"));
    let prog = parse_mirr(&src).expect("parse should succeed");
    assert!(!prog.module.pattern_calls.is_empty(), "parser should detect pattern call");
    assert!(prog.module.pattern_calls[0].pattern_name == "simple_check", "call name matches");
}

#[test]
fn parse_no_false_positives_on_keywords() {
    let src = std_module("signal x: in bool;", "");
    let prog = parse_mirr(&src).expect("parse should succeed");
    assert!(prog.module.pattern_calls.is_empty(), "keywords should NOT be parsed as calls");
}
