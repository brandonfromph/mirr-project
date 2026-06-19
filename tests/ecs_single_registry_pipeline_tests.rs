#![forbid(unsafe_code)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn hls_source() -> &'static str {
    "module hls_top {\n\
     signal a: in bool;\n\
     signal b: in bool;\n\
     signal y: out bool;\n\
     reflex r {\n\
     on always {\n\
     y = a && b;\n\
     }\n\
     }\n\
     }"
}

#[test]
fn hls_components_are_returned_in_the_canonical_registry() {
    let config = PipelineConfig { hls: true, temporal: false, ..PipelineConfig::default() };
    let result = run_pipeline(hls_source(), &config).expect("pipeline should run HLS");
    assert!(result.hls_result.is_some(), "HLS stage should report completion");

    let registry = result.ecs_registry.expect("pipeline should return the ECS registry");
    assert!(
        registry.hls_dataflow.iter().any(Option::is_some),
        "returned registry must include HLS dataflow components"
    );
    assert!(
        registry.hls_schedules.iter().any(Option::is_some),
        "returned registry must include HLS schedule components"
    );
    assert!(
        registry.hls_bindings.iter().any(Option::is_some),
        "returned registry must include HLS binding components"
    );
}
