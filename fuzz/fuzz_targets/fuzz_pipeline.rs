#![no_main]
use libfuzzer_sys::fuzz_target;
use nasa_rust_project::{run_pipeline, PipelineConfig};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let config = PipelineConfig {
            typecheck: true,
            simplify: true,
            width: true,
            temporal: true,
            rspu: false,
            extended_typecheck: false,
            simulate: false,
            ..PipelineConfig::default()
        };
        let _ = run_pipeline(s, &config);
    }
});
