#![forbid(unsafe_code)]
//! Pipeline temporal-to-emitter projection tests.

use nasa_rust_project::emit;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

const TEMPORAL_SRC: &str = r#"
module temporal_projection {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 3 cycles;
    }

    reflex r {
        on g {
            b = true;
        }
    }
}
"#;

#[test]
fn temporal_netlist_projects_into_verilog() {
    let result =
        run_pipeline(TEMPORAL_SRC, &PipelineConfig::default()).expect("pipeline should succeed");
    let sv = emit::verilog::emit_sv(&result);
    assert!(sv.contains("Temporal Guards"), "expected temporal section in Verilog output");
}

#[test]
fn temporal_netlist_projects_into_dot_subgraph() {
    let result =
        run_pipeline(TEMPORAL_SRC, &PipelineConfig::default()).expect("pipeline should succeed");
    let dot = emit::dot::emit_module_dot(&result);
    assert!(dot.contains("cluster_temporal"), "expected temporal cluster in DOT output");
}

#[test]
fn temporal_disabled_omits_temporal_dot_cluster() {
    let config = PipelineConfig { temporal: false, ..PipelineConfig::default() };
    let result = run_pipeline(TEMPORAL_SRC, &config).expect("pipeline should succeed");
    let dot = emit::dot::emit_module_dot(&result);
    assert!(
        !dot.contains("cluster_temporal"),
        "temporal cluster must be absent when temporal is disabled"
    );
}
