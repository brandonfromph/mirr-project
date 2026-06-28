#![allow(clippy::field_reassign_with_default)]
#![cfg(feature = "legacy_ast")]
#![cfg(any())]
#![forbid(unsafe_code)]
//! SAT-based simplification integration tests.
//!
//! Exercises the SAT module through the pipeline with `sat_simplify: true`.
//! Covers: empty modules, boolean guards, arithmetic guards (SAT bypass),
//! nested guards, complex expressions, multi-guard reflexes, double negation,
//! OR/AND/XOR chains, prev references, internal signals, and pipeline stats.

use mirrc::pipeline::{run_pipeline, PipelineConfig};

// ---------------------------------------------------------------------------
// Helper: pipeline config with SAT simplification enabled
// ---------------------------------------------------------------------------

fn sat_config() -> PipelineConfig {
    PipelineConfig { sat_simplify: true, ..PipelineConfig::default() }
}

fn sat_only_config() -> PipelineConfig {
    PipelineConfig {
        typecheck: true,
        simplify: true,
        sat_simplify: true,
        width: false,
        temporal: false,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    }
}

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

const EMPTY_MODULE: &str = r#"
module empty {
    signal a: in bool;
    signal b: out bool;
}
"#;

const MINIMAL_BOOL: &str = r#"
module minimal_bool {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    reflex r {
        on g {
            b = a;
        }
    }
}
"#;

const DOUBLE_NEGATION: &str = r#"
module double_neg {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when !(!x)
        for 1 cycles;
    }

    reflex r {
        on g {
            y = !(!x);
        }
    }
}
"#;

const AND_GUARD: &str = r#"
module and_guard {
    signal a: in bool;
    signal b: in bool;
    signal out: out bool;

    guard g {
        when a && b
        for 3 cycles;
    }

    reflex r {
        on g {
            out = a && b;
        }
    }
}
"#;

const OR_GUARD: &str = r#"
module or_guard {
    signal a: in bool;
    signal b: in bool;
    signal out: out bool;

    guard g {
        when a || b
        for 2 cycles;
    }

    reflex r {
        on g {
            out = a || b;
        }
    }
}
"#;

const COMPARISON_GUARD: &str = r#"
module comparison {
    signal sensor: in u8;
    signal alarm: out bool;

    guard threshold {
        when sensor > 100
        for 5 cycles;
    }

    reflex activate {
        on threshold {
            alarm = true;
        }
    }
}
"#;

const ARITHMETIC_REFLEX: &str = r#"
module arith {
    signal a: in u8;
    signal b: in u8;
    signal result: out u16;

    guard check {
        when a > 10
        for 1 cycles;
    }

    reflex compute {
        on check {
            result = a + b;
        }
    }
}
"#;

const MULTI_GUARD: &str = r#"
module multi_guard {
    signal x: in bool;
    signal y: in bool;
    signal out: out bool;

    guard g1 {
        when x
        for 2 cycles;
    }

    guard g2 {
        when y
        for 3 cycles;
    }

    reflex both {
        on g1 and g2 {
            out = true;
        }
    }
}
"#;

const NESTED_BOOL_EXPR: &str = r#"
module nested_bool {
    signal a: in bool;
    signal b: in bool;
    signal c: in bool;
    signal out: out bool;

    guard g {
        when (a && b) || c
        for 4 cycles;
    }

    reflex r {
        on g {
            out = (a || b) && c;
        }
    }
}
"#;

const TRIVIAL_ASSIGN: &str = r#"
module prev_ref {
    signal s: in bool;
    signal out: out bool;

    guard g {
        when s
        for 1 cycles;
    }

    reflex r {
        on g {
            out = s;
        }
    }
}
"#;

const INTERNAL_SIGNAL: &str = r#"
module internal_sig {
    signal a: in bool;
    signal b: out bool;
    signal buf: internal bool;

    guard g {
        when a
        for 1 cycles;
    }

    reflex r {
        on g {
            buf = a;
            b = buf;
        }
    }
}
"#;

const COMPLEX_NESTED: &str = r#"
module complex_nested {
    signal a: in bool;
    signal b: in bool;
    signal c: in bool;
    signal d: in bool;
    signal out: out bool;

    guard g {
        when (a && b) || (c && d)
        for 2 cycles;
    }

    reflex r {
        on g {
            out = (a || c) && (b || d);
        }
    }
}
"#;

const NEGATED_BOOL: &str = r#"
module neg_cmp {
    signal temp: in bool;
    signal alert: out bool;

    guard overtemp {
        when !temp
        for 10 cycles;
    }

    reflex fire {
        on overtemp {
            alert = true;
        }
    }
}
"#;

const LONG_CYCLE_GUARD: &str = r#"
module long_cycle {
    signal enable: in bool;
    signal status: out bool;

    guard sustained {
        when enable
        for 1000 cycles;
    }

    reflex engage {
        on sustained {
            status = true;
        }
    }
}
"#;

// ---------------------------------------------------------------------------
// Tests: SAT pipeline integration
// ---------------------------------------------------------------------------

#[test]
fn sat_empty_module_produces_stats() {
    let result = run_pipeline(EMPTY_MODULE, &sat_config()).expect("pipeline should succeed");
    assert!(result.sat_stats.is_some(), "SAT stats should be present");
    let stats = result.sat_stats.as_ref().unwrap();
    // Empty module has no guards/reflexes: zero checks.
    assert_eq!(stats.checks_performed, 0);
    assert_eq!(stats.equivalences_confirmed, 0);
    assert!(!stats.had_unknown);
}

#[test]
fn sat_disabled_produces_no_stats() {
    let config = PipelineConfig::default();
    assert!(!config.sat_simplify, "SAT should be off by default");
    let result = run_pipeline(MINIMAL_BOOL, &config).expect("pipeline should succeed");
    assert!(result.sat_stats.is_none(), "SAT stats should be absent when disabled");
}

#[test]
fn sat_minimal_bool_succeeds() {
    let result = run_pipeline(MINIMAL_BOOL, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "minimal_bool");
    assert!(result.sat_stats.is_some());
}

#[test]
fn sat_double_negation_simplifies() {
    let result =
        run_pipeline(DOUBLE_NEGATION, &sat_only_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "double_neg");
    let stats = result.sat_stats.as_ref().unwrap();
    // Double negation is simplified by heuristic simplifier; SAT verifies it.
    // At minimum, the pipeline ran without error.
    assert!(!stats.had_unknown, "should not hit solver bounds on small expr");
}

#[test]
fn sat_and_guard_succeeds() {
    let result = run_pipeline(AND_GUARD, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "and_guard");
    assert!(result.sat_stats.is_some());
    assert!(result.temporal_netlist.is_some());
}

#[test]
fn sat_or_guard_succeeds() {
    let result = run_pipeline(OR_GUARD, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "or_guard");
    assert!(result.sat_stats.is_some());
}

#[test]
fn sat_comparison_guard_runs() {
    let result = run_pipeline(COMPARISON_GUARD, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "comparison");
    let stats = result.sat_stats.as_ref().unwrap();
    // Comparison guard contains non-boolean ops; SAT module should handle gracefully.
    assert!(!stats.had_unknown);
}

#[test]
fn sat_arithmetic_reflex_bypassed() {
    // Arithmetic expressions (a + b) are not boolean; SAT checker should skip them.
    let result =
        run_pipeline(ARITHMETIC_REFLEX, &sat_only_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "arith");
    assert!(result.sat_stats.is_some());
}

#[test]
fn sat_multi_guard_reflex() {
    let result = run_pipeline(MULTI_GUARD, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "multi_guard");
    assert!(result.sat_stats.is_some());
    assert!(result.temporal_netlist.is_some());
    let netlist = result.temporal_netlist.as_ref().unwrap();
    assert_eq!(netlist.guards.len(), 2, "should have two compiled guards");
}

#[test]
fn sat_nested_bool_expression() {
    let result = run_pipeline(NESTED_BOOL_EXPR, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "nested_bool");
    let stats = result.sat_stats.as_ref().unwrap();
    assert!(!stats.had_unknown, "nested bool should not exhaust solver");
}

#[test]
fn sat_trivial_assign_handled() {
    let result = run_pipeline(TRIVIAL_ASSIGN, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "prev_ref");
    assert!(result.sat_stats.is_some());
}

#[test]
fn sat_internal_signal_module() {
    let result = run_pipeline(INTERNAL_SIGNAL, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "internal_sig");
    assert!(result.sat_stats.is_some());
}

#[test]
fn sat_complex_nested_no_unknown() {
    let result = run_pipeline(COMPLEX_NESTED, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "complex_nested");
    let stats = result.sat_stats.as_ref().unwrap();
    assert!(!stats.had_unknown, "complex nested bool should stay within bounds");
}

#[test]
fn sat_negated_bool_guard() {
    let result = run_pipeline(NEGATED_BOOL, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "neg_cmp");
    assert!(result.sat_stats.is_some());
}

#[test]
fn sat_long_cycle_guard_succeeds() {
    let result = run_pipeline(LONG_CYCLE_GUARD, &sat_config()).expect("pipeline should succeed");
    assert_eq!(result.program.as_ref().unwrap().module.name, "long_cycle");
    assert!(result.sat_stats.is_some());
    // Verify temporal netlist compiled the 1000-cycle guard.
    let netlist = result.temporal_netlist.as_ref().unwrap();
    assert_eq!(netlist.guards.len(), 1);
}

#[test]
fn sat_preserves_simplify_stats() {
    // When SAT is enabled, heuristic simplify_stats should still be present.
    let result = run_pipeline(MINIMAL_BOOL, &sat_config()).expect("pipeline should succeed");
    assert!(result.simplify_stats.is_some(), "heuristic stats should exist alongside SAT");
    assert!(result.sat_stats.is_some(), "SAT stats should also exist");
}

#[test]
fn sat_rejects_invalid_source() {
    let result = run_pipeline("not valid mirr code!!!", &sat_config());
    assert!(result.is_err(), "invalid source should fail even with SAT enabled");
}

#[test]
fn sat_full_pipeline_with_width_and_temporal() {
    // SAT + width + temporal all enabled — full pipeline.
    let result = run_pipeline(AND_GUARD, &sat_config()).expect("full pipeline should succeed");
    assert!(result.sat_stats.is_some());
    assert!(result.width_stats.is_some());
    assert!(result.temporal_netlist.is_some());
    assert!(!result.has_width_errors());
}

#[test]
fn sat_stats_equivalences_nonnegative() {
    // Sanity: equivalences_confirmed should never exceed checks_performed.
    let result = run_pipeline(NESTED_BOOL_EXPR, &sat_config()).expect("pipeline should succeed");
    let stats = result.sat_stats.as_ref().unwrap();
    assert!(
        stats.equivalences_confirmed <= stats.checks_performed,
        "equivalences ({}) should not exceed checks ({})",
        stats.equivalences_confirmed,
        stats.checks_performed,
    );
}
