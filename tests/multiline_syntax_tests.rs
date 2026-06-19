#![cfg(any())]
use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_multiline_pattern_call() {
    let source = "
    def my_pattern(s: signal in bool) {
        reflect {
            signal internal_sig: internal bool;
            reflex r { internal_sig = ${s}; }
        }
    }

    module test {
        signal input_sig: in bool;
        my_pattern(
            input_sig
        );
    }
    ";
    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);

    assert!(
        result.is_ok(),
        "Multi-line pattern call should be supported. Error: {:?}",
        result.err()
    );
}
