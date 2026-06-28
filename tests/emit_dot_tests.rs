#![allow(clippy::field_reassign_with_default)]
#![forbid(unsafe_code)]
//! DOT emitter edge-case tests.
//!
//! Covers Prev back-edges (dashed red), complex guard cluster nodes,
//! empty module through emit_expr_dot, multi-assignment reflex subgraphs,
//! and internal signal shape.

use mirrc::emit;
use mirrc::pipeline::{run_pipeline, PipelineConfig, PipelineResult};

// ---------------------------------------------------------------------------
// MIRR fixtures (no Prev — those use programmatic AST)
// ---------------------------------------------------------------------------

const MULTI_ASSIGN_MIRR: &str = r#"
module multi_assign {
    signal a: in bool;
    signal b: out bool;
    signal c: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    reflex r {
        on g {
            b = a;
            c = true;
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

const EMPTY_MODULE_MIRR: &str = r#"
module empty {
    signal x: in bool;
    signal y: out bool;
}
"#;

const PREV_MIRR_SRC: &str = r#"
module prev_guard {
    signal x: in u8;
    signal y: out bool;
    guard g { when x > prev(x, 3) for 5 cycles; }
    reflex r { on g { y = true; } }
}
"#;

fn prev_guard_result() -> PipelineResult {
    let mut reg = mirrc::ecs::Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut reg, PREV_MIRR_SRC, None).unwrap();

    PipelineResult {
        hls_result: None,
        program: None,
        simplify_stats: None,
        width_stats: None,
        width_diagnostics: Vec::new(),
        temporal_netlist: None,
        rspu_program: None,
        extended_type_map: None,
        sim_result: None,
        mape_k_result: None,
        sat_stats: None,
        retiming_stats: None,
        totality_result: None,
        symbolic_result: None,
        mape_k_rtl: None,
        ecs_registry: Some(reg),
        file_table: mirrc::span::FileTable::new(),
    }
}

// ---------------------------------------------------------------------------
// Prev back-edges
// ---------------------------------------------------------------------------

#[test]
fn dot_prev_in_guard_produces_dashed_red_edge() {
    let result = prev_guard_result();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(
        dot.contains("style=dashed color=red label=\"prev\""),
        "Prev back-edges should be dashed red. DOT output:\n{dot}"
    );
}

#[test]
fn dot_prev_in_guard_links_signal_to_guard() {
    let result = prev_guard_result();
    let dot = emit::dot::emit_module_dot(&result);

    // The prev references signal 'x', guard is 'g'
    assert!(dot.contains("x -> guard_g"), "Prev edge should link x to guard_g");
}

// ---------------------------------------------------------------------------
// expr-level DOT on Prev guard
// ---------------------------------------------------------------------------

#[test]
fn dot_expr_prev_in_guard_shows_prev_node() {
    let result = prev_guard_result();
    let dot = emit::dot::emit_expr_dot(&result);

    assert!(dot.contains("prev(x,3)"), "Expr-level DOT should show prev(x,3). Got:\n{dot}");
    assert!(dot.contains("style=dashed color=red"), "Prev nodes should be dashed red");
}

// ---------------------------------------------------------------------------
// Empty module through emit_expr_dot
// ---------------------------------------------------------------------------

#[test]
fn dot_expr_empty_module_valid_digraph() {
    let config = PipelineConfig::default();
    let result = run_pipeline(EMPTY_MODULE_MIRR, &config).unwrap();
    let dot = emit::dot::emit_expr_dot(&result);

    assert!(dot.starts_with("digraph empty_expr {"));
    assert!(dot.ends_with("}\n"));
    // No guard or reflex subgraphs
    assert!(!dot.contains("cluster_guard_"));
    assert!(!dot.contains("cluster_r_"));
}

// ---------------------------------------------------------------------------
// Multi-assignment reflex subgraphs
// ---------------------------------------------------------------------------

#[test]
fn dot_expr_multi_assignment_produces_multiple_subgraphs() {
    let config = PipelineConfig::default();
    let result = run_pipeline(MULTI_ASSIGN_MIRR, &config).unwrap();
    let dot = emit::dot::emit_expr_dot(&result);

    assert!(dot.contains("cluster_r_b"), "should have subgraph for assignment to b");
    assert!(dot.contains("cluster_r_c"), "should have subgraph for assignment to c");
}

// ---------------------------------------------------------------------------
// Internal signal shape
// ---------------------------------------------------------------------------

#[test]
fn dot_internal_signal_uses_ellipse_shape() {
    let config = PipelineConfig::default();
    let result = run_pipeline(INTERNAL_SIGNAL_MIRR, &config).unwrap();
    let dot = emit::dot::emit_module_dot(&result);

    // 'buf' is internal -> shape=ellipse
    assert!(
        dot.contains("buf [label=\"buf: u8\" shape=ellipse]"),
        "internal signal should use ellipse shape. Got:\n{dot}"
    );
}

// ---------------------------------------------------------------------------
// Signal node labels
// ---------------------------------------------------------------------------

#[test]
fn dot_signal_node_contains_type_label() {
    let result = prev_guard_result();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.contains("x: u8"), "signal x should be labeled with u8");
    assert!(dot.contains("y: bool"), "signal y should be labeled with bool");
}
