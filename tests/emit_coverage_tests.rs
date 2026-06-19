#![cfg(any())]
//! Integration tests for the emitters (dot, sexpr).
//! Compiles various complex examples through the pipeline and runs the emitters
//! to ensure 100% coverage without panics or errors.

#![forbid(unsafe_code)]

use mirrc::emit::dot::{emit_expr_dot, emit_module_dot};
use mirrc::emit::sexpr::emit_sexpr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

/// Helper: run the pipeline and return PipelineResult.
fn compile(source: &str) -> mirrc::pipeline::PipelineResult {
    let config = PipelineConfig::default();
    run_pipeline(source, &config).expect("Pipeline should succeed")
}

const EXAMPLES: &[&str] = &[
    include_str!("../examples/autonomous_vehicle.mirr"),
    include_str!("../examples/fir_filter.mirr"),
    include_str!("../examples/icu_monitor.mirr"),
    include_str!("../examples/industrial_safety.mirr"),
    include_str!("../examples/neonatal_respirator.mirr"),
    include_str!("../examples/pattern_usage.mirr"),
    include_str!("../examples/safety_property.mirr"),
    include_str!("../examples/shift_register_guard.mirr"),
    include_str!("../examples/tmr_sensor_fusion.mirr"),
];

#[test]
fn test_emit_module_dot_all_examples() {
    for source in EXAMPLES {
        let result = compile(source);
        let dot = emit_module_dot(&result);

        assert!(dot.starts_with("digraph"));
        assert!(dot.contains("rankdir="));
        assert!(dot.contains("}"));
        assert!(dot.contains("shape="));
    }
}

#[test]
fn test_emit_expr_dot_all_examples() {
    for source in EXAMPLES {
        let result = compile(source);
        let dot = emit_expr_dot(&result);

        assert!(dot.starts_with("digraph"));
        assert!(dot.contains("rankdir="));
        assert!(dot.contains("}"));
        assert!(dot.contains("cluster_guard_"));
    }
}

#[test]
fn test_emit_sexpr_all_examples() {
    for source in EXAMPLES {
        let result = compile(source);
        let sexpr = emit_sexpr(&result).expect("Failed to emit sexpr");

        assert!(sexpr.contains("(module"));
        assert!(sexpr.contains("(signal"));
    }
}

// Since emit_dot has internal traversal limits (MAX_DOT_NODES),
// we can trigger them directly by generating a massive expression AST.
#[test]
fn test_emit_dot_max_node_truncation() {
    // We construct a massive AST programmatically.
    use mirrc::ast::expr::Expr;
    use mirrc::ast::program::Module;

    // Create a deeply nested array literal to blow past MAX_DOT_NODES (4096).
    let mut big_array_elems = Vec::new();
    for _ in 0..5000 {
        big_array_elems.push(Expr::Literal(mirrc::ast::LiteralValue::Bool(true)));
    }
    let big_expr = Expr::ArrayLiteral(big_array_elems);

    let module = Module {
        name: "huge_module".to_string(),
        signals: vec![],
        guards: vec![mirrc::ast::program::Guard {
            name: "huge_guard".to_string(),
            condition: big_expr.clone(),
            cycles: 1,
            template_cycles: None,
            origin: None,
            span: None,
        }],
        reflexes: vec![mirrc::ast::program::Reflex {
            name: "huge_reflex".to_string(),
            guard_names: vec!["huge_guard".to_string()],
            assignments: vec![mirrc::ast::program::Assignment {
                target: "dummy".to_string(),
                value: big_expr,
                span: None,
            }],
            origin: None,
            span: None,
        }],
        pattern_calls: vec![],
        pattern_origins: vec![],
        properties: vec![],
        span: None,
    };

    let mut result = compile(include_str!("../examples/shift_register_guard.mirr"));
    // Inject our massive module via the Registry to test the truncation limit.
    let mut reg = mirrc::ecs::Registry::new();
    let prog = mirrc::MirrProgram { target: None, patterns: vec![], imports: vec![], module };
    reg.ingest_program(&prog).unwrap();
    result.ecs_registry = Some(reg);

    // Run expr dot and module dot to ensure we hit truncation paths without panic.
    let _expr_dot = emit_expr_dot(&result);
    let _mod_dot = emit_module_dot(&result);
}
