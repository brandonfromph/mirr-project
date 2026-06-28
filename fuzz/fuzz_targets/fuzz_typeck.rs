#![no_main]
use libfuzzer_sys::fuzz_target;
use mirrc::{run_pipeline, PipelineConfig};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut config = PipelineConfig::default();
        config.temporal = false;
        config.width = false;
        config.simulate = false;
        config.mape_k = false;
        let _ = run_pipeline(s, &config);
    }
});
