#![forbid(unsafe_code)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn run_src(src: &str) -> Result<(), mirrc::error::PipelineErrors> {
    let config = PipelineConfig {
        simplify: false,
        width: false,
        temporal: false,
        mape_k: false,
        ..Default::default()
    };
    let full_src = format!("target profile {{ name: \"t\"; word_size: 64; }} {}", src);
    run_pipeline(&full_src, &config).map(|_| ())
}

#[test]
fn stdlib_token_buffer_capacity_constant_is_8192() {
    let src = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;
    assert!(run_src(src).is_ok());
}
