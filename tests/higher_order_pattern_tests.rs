//! TYPE-005: Higher-Order Patterns tests.
//!
//! Verifies:
//! 1. Parsing `pattern` parameter kind
//! 2. Pattern-as-argument composition
//! 3. Recursive sub-expansion
//! 4. Cycle detection (E402)
//! 5. Kind mismatch (E401)
//! 6. Undefined pattern ref (E403)
//! 7. Depth limit enforcement
//! 8. Full pipeline with composed patterns

#![forbid(unsafe_code)]
#![deny(warnings)]

extern crate nasa_rust_project;

use nasa_rust_project::ast::pattern::PatternParamKind;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

// ───────────────── helpers ─────────────────

fn parse_ok(source: &str) -> nasa_rust_project::MirrProgram {
    parse_mirr(source).unwrap_or_else(|e| panic!("Parse failed: {e}"))
}

fn pipeline_ok(source: &str) -> nasa_rust_project::pipeline::PipelineResult {
    run_pipeline(source, &PipelineConfig::default())
        .unwrap_or_else(|e| panic!("Pipeline failed: {e}"))
}

fn pipeline_err(source: &str) -> String {
    match run_pipeline(source, &PipelineConfig::default()) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("Expected pipeline error"),
    }
}

// ───────────────── Section 1: Parsing ─────────────────

#[test]
fn parse_pattern_param_kind() {
    let src = r#"
def compose(inner: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        guard g {
            when ${s}
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
    let prog = parse_ok(src);
    assert_eq!(prog.patterns.len(), 1);
    assert_eq!(prog.patterns[0].params.len(), 3);
    assert!(matches!(prog.patterns[0].params[0].kind, PatternParamKind::Pattern));
    assert_eq!(prog.patterns[0].params[0].name, "inner");
}

#[test]
fn parse_pattern_param_alongside_other_kinds() {
    let src = r#"
def multi(a: signal in u8, p: pattern, c: u16) {
    reflect {
        guard g {
            when ${a} > 0
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
    let prog = parse_ok(src);
    assert!(matches!(prog.patterns[0].params[0].kind, PatternParamKind::Signal { .. }));
    assert!(matches!(prog.patterns[0].params[1].kind, PatternParamKind::Pattern));
    assert!(matches!(prog.patterns[0].params[2].kind, PatternParamKind::Constant { .. }));
}

// ───────── Section 2: Composition (two-level expansion) ─────────

#[test]
fn compose_two_patterns() {
    let src = r#"
def inner_pattern(s: signal in bool, out: signal out bool) {
    reflect {
        guard ${s}_check {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_respond {
            on ${s}_check {
                ${out} = true;
            }
        }
    }
}

def outer_pattern(p: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        ${p}(${s}, ${out});
    }
}

module m {
    signal sensor: in bool;
    signal alarm: out bool;

    outer_pattern(inner_pattern, sensor, alarm);
}
"#;
    let result = pipeline_ok(src);
    let module = &result.program.module;

    // The inner pattern should have expanded through the outer.
    assert!(
        module.guards.iter().any(|g| g.name.contains("check")),
        "Should find inner pattern's guard after two-level expansion. Guards: {:?}",
        module.guards.iter().map(|g| &g.name).collect::<Vec<_>>()
    );
    assert!(
        module.reflexes.iter().any(|r| r.name.contains("respond")),
        "Should find inner pattern's reflex after two-level expansion. Reflexes: {:?}",
        module.reflexes.iter().map(|r| &r.name).collect::<Vec<_>>()
    );
}

#[test]
fn compose_preserves_origin_tags() {
    let src = r#"
def leaf(s: signal in bool, out: signal out bool) {
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

def wrapper(p: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        ${p}(${s}, ${out});
    }
}

module m {
    signal x: in bool;
    signal y: out bool;

    wrapper(leaf, x, y);
}
"#;
    let result = pipeline_ok(src);
    let module = &result.program.module;

    // At least one origin should exist from the expansion chain.
    assert!(
        !module.pattern_origins.is_empty(),
        "Should have pattern origins from composed expansion"
    );
}

// ───────── Section 3: Cycle detection (E402) ─────────

#[test]
fn self_referencing_pattern_e402() {
    let src = r#"
def self_ref(s: signal in bool) {
    reflect {
        self_ref(${s});
    }
}

module m {
    signal x: in bool;
    signal y: out bool;

    self_ref(x);

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
    assert!(msg.contains("[E402]") || msg.contains("Circular"), "expected E402, got: {}", msg);
    assert!(msg.contains("self_ref"), "should name the circular pattern, got: {}", msg);
}

#[test]
fn mutual_recursion_e402() {
    let src = r#"
def alpha(s: signal in bool) {
    reflect {
        beta(${s});
    }
}

def beta(s: signal in bool) {
    reflect {
        alpha(${s});
    }
}

module m {
    signal x: in bool;
    signal y: out bool;

    alpha(x);

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
    assert!(msg.contains("[E402]") || msg.contains("Circular"), "expected E402, got: {}", msg);
}

// ───────── Section 4: E426 — kind mismatch ─────────

#[test]
fn pattern_param_receives_constant_e426() {
    let src = r#"
def needs_pattern(p: pattern, s: signal in bool) {
    reflect {
        guard g {
            when ${s}
            for 1 cycles;
        }
    }
}

module m {
    signal x: in bool;
    signal y: out bool;

    needs_pattern(42, x);

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
    assert!(msg.contains("[E426]"), "expected E426 for kind mismatch, got: {}", msg);
}

// ───────── Section 5: Undefined pattern ref ─────────

#[test]
fn undefined_pattern_ref_errors() {
    let src = r#"
def wrapper(p: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        ${p}(${s}, ${out});
    }
}

module m {
    signal x: in bool;
    signal y: out bool;

    wrapper(nonexistent, x, y);
}
"#;
    let msg = pipeline_err(src);
    assert!(
        msg.contains("undefined") || msg.contains("Undefined") || msg.contains("nonexistent"),
        "expected undefined pattern error, got: {}",
        msg
    );
}

// ───────── Section 6: Depth limit ─────────

#[test]
fn depth_limit_exceeded_errors() {
    // Chain of 5 patterns: a -> b -> c -> d -> e (depth=5 > MAX_EXPANSION_DEPTH=4)
    let src = r#"
def e_pat(s: signal in bool, out: signal out bool) {
    reflect {
        guard ${s}_eg {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_er {
            on ${s}_eg {
                ${out} = true;
            }
        }
    }
}

def d_pat(inner: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        ${inner}(${s}, ${out});
    }
}

def c_pat(inner: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        d_pat(${inner}, ${s}, ${out});
    }
}

def b_pat(inner: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        c_pat(${inner}, ${s}, ${out});
    }
}

def a_pat(inner: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        b_pat(${inner}, ${s}, ${out});
    }
}

module m {
    signal x: in bool;
    signal y: out bool;

    a_pat(e_pat, x, y);
}
"#;
    let msg = pipeline_err(src);
    assert!(
        msg.contains("depth") || msg.contains("Depth") || msg.contains("exceeded"),
        "expected depth limit error, got: {}",
        msg
    );
}

// ───────── Section 7: Full pipeline E2E ─────────

#[test]
fn composed_pattern_full_pipeline() {
    let src = r#"
def monitor(s: signal in u16, threshold: u16, alarm: signal out bool) {
    reflect {
        guard ${s}_high {
            when ${s} > ${threshold}
            for 1 cycles;
        }

        reflex ${s}_alert {
            on ${s}_high {
                ${alarm} = true;
            }
        }
    }
}

def apply_monitor(m: pattern, s: signal in u16, t: u16, out: signal out bool) {
    reflect {
        ${m}(${s}, ${t}, ${out});
    }
}

module system {
    signal temp: in u16;
    signal temp_alarm: out bool;

    apply_monitor(monitor, temp, 100, temp_alarm);
}
"#;
    let result = pipeline_ok(src);
    let module = &result.program.module;

    // After composition: should have the expanded guard + reflex from inner pattern.
    assert!(module.guards.iter().any(|g| g.name.contains("high")), "should find monitor's guard");
    assert!(
        module.reflexes.iter().any(|r| r.name.contains("alert")),
        "should find monitor's reflex"
    );
}

#[test]
fn composed_pattern_with_temporal() {
    let src = r#"
def sensor_check(s: signal in u16, limit: u16, out: signal out bool) {
    reflect {
        guard ${s}_over {
            when ${s} > ${limit}
            for 3 cycles;
        }

        reflex ${s}_react {
            on ${s}_over {
                ${out} = true;
            }
        }
    }
}

def apply_check(check: pattern, s: signal in u16, lim: u16, out: signal out bool) {
    reflect {
        ${check}(${s}, ${lim}, ${out});
    }
}

module m {
    signal pressure: in u16;
    signal alarm: out bool;

    apply_check(sensor_check, pressure, 200, alarm);
}
"#;
    let result = pipeline_ok(src);
    // Temporal compilation should succeed.
    assert!(result.temporal_netlist.is_some(), "temporal netlist should be generated");
}

#[test]
fn non_pattern_call_in_pattern_passes() {
    // A pattern that doesn't use higher-order features still works.
    let src = r#"
def simple(s: signal in bool, out: signal out bool) {
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
    signal x: in bool;
    signal y: out bool;

    simple(x, y);
}
"#;
    let result = pipeline_ok(src);
    assert!(!result.program.module.guards.is_empty(), "should have expanded guard");
}

#[test]
fn no_cycle_when_patterns_share_callee() {
    // Diamond: A calls C, B calls C — not circular.
    let src = r#"
def leaf(s: signal in bool, out: signal out bool) {
    reflect {
        guard ${s}_lg {
            when ${s}
            for 1 cycles;
        }

        reflex ${s}_lr {
            on ${s}_lg {
                ${out} = true;
            }
        }
    }
}

def wrapper_a(p: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        ${p}(${s}, ${out});
    }
}

def wrapper_b(p: pattern, s: signal in bool, out: signal out bool) {
    reflect {
        ${p}(${s}, ${out});
    }
}

module m {
    signal x: in bool;
    signal a: out bool;
    signal b: out bool;

    wrapper_a(leaf, x, a);
    wrapper_b(leaf, x, b);
}
"#;
    let result = pipeline_ok(src);
    assert!(result.program.module.guards.len() >= 2, "should have guards from both wrappers");
}

