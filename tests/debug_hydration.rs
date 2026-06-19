
use mirrc::pipeline::{run_pipeline_with_file, PipelineConfig};

fn run_test(source: &str) -> Result<(), String> {
    let config = PipelineConfig { bootstrap_mode: true, ..Default::default() };
    match run_pipeline_with_file(source, "test.mirr", &config) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[test]
fn test_hydration_idempotency() {
    let source = "
    module test {
        signal s1: bool;
        signal s2: bool;
        reflex r {
            on always {
                s1 = s2;
            }
        }
    }
    ";
    let config = PipelineConfig::default();
    let result = run_pipeline_with_file(source, "test.mirr", &config);
    assert!(result.is_ok());
}

#[test]
fn test_hydration_bug() {
    let source = "
    module test {
        signal input_pulse: in bool;
        signal s1: bool;
        reflex process_pulse {
            on input_pulse {
                s1 = true;
            }
        }
    }
    ";

    // Test that the hydration bug is fixed
    let result = run_test(source);
    assert!(result.is_ok(), "Hydration bug failed: {:?}", result);
}

fn main() {}
