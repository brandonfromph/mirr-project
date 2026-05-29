#![forbid(unsafe_code)]
//! Pipeline parse/expand/temporal integration tests.

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

const SIMPLE_TEMPORAL: &str = r#"
module parse_expand_temporal {
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

#[test]
fn parse_expand_temporal_default_pipeline_succeeds() {
    let result = run_pipeline(SIMPLE_TEMPORAL, &PipelineConfig::default())
        .expect("default pipeline should succeed");
    assert!(result.temporal_netlist.is_some(), "temporal netlist should be produced");
}

#[test]
fn parse_expand_without_temporal_stage_keeps_parse_and_validate() {
    let config = PipelineConfig { temporal: false, ..PipelineConfig::default() };
    let result = run_pipeline(SIMPLE_TEMPORAL, &config).expect("pipeline should succeed");
    assert!(result.temporal_netlist.is_none(), "temporal stage should be skipped");
}
