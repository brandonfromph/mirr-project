#![forbid(unsafe_code)]
//! Pipeline typecheck/temporal gating tests.

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

const ASSIGNMENT_TYPE_MISMATCH: &str = r#"
module typeck_gate {
    signal trig: in bool;
    signal n: in u16;
    signal out: out bool;

    guard g {
        when trig
        for 1 cycles;
    }

    reflex r {
        on g {
            out = n;
        }
    }
}
"#;

#[test]
fn typecheck_enabled_rejects_assignment_mismatch() {
    let err = match run_pipeline(ASSIGNMENT_TYPE_MISMATCH, &PipelineConfig::default()) {
        Ok(_) => panic!("typecheck-enabled pipeline should reject mismatch"),
        Err(e) => e,
    };
    assert!(err.to_string().contains("[E602]"));
}

#[test]
fn typecheck_disabled_allows_mismatch_to_reach_temporal_stage() {
    let config = PipelineConfig {
        typecheck: false,
        simplify: true,
        width: true,
        temporal: true,
        ..PipelineConfig::default()
    };
    let result = run_pipeline(ASSIGNMENT_TYPE_MISMATCH, &config)
        .expect("typecheck-disabled pipeline should continue");
    assert!(result.temporal_netlist.is_some(), "temporal stage should still run");
}
