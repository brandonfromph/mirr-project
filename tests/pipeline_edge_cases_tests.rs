//! Pipeline edge-case tests.
//!
//! Covers semantic error propagation, has_width_errors() == true,
//! temporal compilation error from XOR guard, untested PipelineConfig
//! combinations, simplify_stats numeric accuracy, and internal signals.

use nasa_rust_project::emit;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

// ---------------------------------------------------------------------------
// MIRR fixtures
// ---------------------------------------------------------------------------

const DUPLICATE_SIGNAL_MIRR: &str = r#"
module dup_sig {
    signal x: in bool;
    signal x: out bool;

    guard g {
        when x
        for 2 cycles;
    }

    reflex r {
        on g {
            x = true;
        }
    }
}
"#;

/// XOR guard condition — unsupported for temporal lowering.
const XOR_GUARD_MIRR: &str = r#"
module xor_mod {
    signal a: in bool;
    signal b: in bool;
    signal out: out bool;

    guard g {
        when a ^ b
        for 5 cycles;
    }

    reflex r {
        on g {
            out = true;
        }
    }
}
"#;

const SIMPLIFIABLE_MIRR: &str = r#"
module simp {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a && true
        for 2 cycles;
    }

    reflex r {
        on g {
            b = a || false;
        }
    }
}
"#;

const INTERNAL_SIGNAL_MIRR: &str = r#"
module with_internal {
    signal a: in u8;
    signal b: out u8;
    signal buf: internal u8;

    guard g {
        when a > 10
        for 5 cycles;
    }

    reflex r {
        on g {
            buf = a;
            b = buf;
        }
    }
}
"#;

// ---------------------------------------------------------------------------
// Semantic error propagation through pipeline
// ---------------------------------------------------------------------------

#[test]
fn pipeline_semantic_error_for_duplicate_signal() {
    let config = PipelineConfig::default();
    let result = run_pipeline(DUPLICATE_SIGNAL_MIRR, &config);
    let err = match result {
        Err(e) => e,
        Ok(_) => panic!("expected semantic error for duplicate signal"),
    };
    let msg = err.to_string();
    assert_eq!(msg, "Semantic error: [E201] Duplicate signal name: 'x'.");
}

// ---------------------------------------------------------------------------
// Temporal compilation error propagation
// ---------------------------------------------------------------------------

#[test]
fn pipeline_temporal_error_for_xor_guard() {
    let config = PipelineConfig::default();
    let result = run_pipeline(XOR_GUARD_MIRR, &config);
    let err = match result {
        Err(e) => e,
        Ok(_) => panic!("expected temporal error for XOR guard"),
    };
    let msg = err.to_string();
    assert!(
        msg.contains("guard 'g'") && msg.contains("unsupported"),
        "expected temporal compilation error, got: {msg}"
    );
}

#[test]
fn pipeline_xor_guard_ok_when_temporal_disabled() {
    let config = PipelineConfig { typecheck: true, simplify: true, width: true, temporal: false, rspu: false };
    let result = run_pipeline(XOR_GUARD_MIRR, &config);
    assert!(result.is_ok(), "XOR guard should pass when temporal is disabled");
}

// ---------------------------------------------------------------------------
// Untested PipelineConfig combinations
// ---------------------------------------------------------------------------

#[test]
fn pipeline_no_simplify_yes_width_yes_temporal() {
    let config = PipelineConfig { typecheck: true, simplify: false, width: true, temporal: true, rspu: false };
    let source = r#"
module combo1 {
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
    let result = run_pipeline(source, &config).unwrap();
    assert!(result.simplify_stats.is_none());
    assert!(result.width_result.is_some());
    assert!(result.temporal_netlist.is_some());
}

#[test]
fn pipeline_yes_simplify_no_width_yes_temporal() {
    let config = PipelineConfig { typecheck: true, simplify: true, width: false, temporal: true, rspu: false };
    let source = r#"
module combo2 {
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
    let result = run_pipeline(source, &config).unwrap();
    assert!(result.simplify_stats.is_some());
    assert!(result.width_result.is_none());
    assert!(result.temporal_netlist.is_some());
}

// ---------------------------------------------------------------------------
// simplify_stats numeric accuracy
// ---------------------------------------------------------------------------

#[test]
fn pipeline_simplify_stats_rules_applied_nonzero() {
    let config = PipelineConfig::default();
    let result = run_pipeline(SIMPLIFIABLE_MIRR, &config).unwrap();
    let stats = result.simplify_stats.as_ref().unwrap();

    // `a && true` -> `a` (1 rule), `a || false` -> `a` (1 rule)
    assert!(stats.rules_applied >= 2, "expected >= 2 rules applied, got {}", stats.rules_applied);
    assert!(stats.nodes_before > stats.nodes_after, "simplification should reduce node count");
}

#[test]
fn pipeline_simplify_stats_nodes_before_gt_zero() {
    let config = PipelineConfig::default();
    let result = run_pipeline(SIMPLIFIABLE_MIRR, &config).unwrap();
    let stats = result.simplify_stats.as_ref().unwrap();

    assert!(stats.nodes_before > 0, "nodes_before should be > 0");
    assert!(stats.nodes_after > 0, "nodes_after should be > 0 (at least the signal remains)");
}

// ---------------------------------------------------------------------------
// Internal signals through full pipeline emit
// ---------------------------------------------------------------------------

#[test]
fn pipeline_internal_signal_in_verilog() {
    let config = PipelineConfig::default();
    let result = run_pipeline(INTERNAL_SIGNAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("// Internal signals"), "should see internal signals section");
    assert!(sv.contains("buf"), "should see 'buf' internal signal");
}

#[test]
fn pipeline_internal_signal_in_dot() {
    let config = PipelineConfig::default();
    let result = run_pipeline(INTERNAL_SIGNAL_MIRR, &config).unwrap();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.contains("buf"), "DOT should contain internal signal 'buf'");
    assert!(dot.contains("shape=ellipse"), "internal signal should use ellipse shape");
}

#[test]
fn pipeline_internal_signal_in_json() {
    let config = PipelineConfig::default();
    let result = run_pipeline(INTERNAL_SIGNAL_MIRR, &config).unwrap();
    let json_str = emit::json_netlist::emit_json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Check that the signal list contains buf
    let signals = &parsed["program"]["module"]["signals"];
    let has_buf = signals.as_array().unwrap().iter().any(|s| s["name"] == "buf");
    assert!(has_buf, "JSON output should contain internal signal 'buf'");
}
