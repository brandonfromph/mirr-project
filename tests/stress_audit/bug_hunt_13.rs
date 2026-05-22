use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
const FIXTURE_COUNTER: &str = r#"
module test_counter {
    signals {
        enable: in bool
        done: out bool
    }

    guard long_delay {
        when enable == true
        for 32 cycles;
    }

    reflex mark_done {
        on long_delay {
            done = true;
        }
    }
}
"#;
#[test]
fn debug_pipeline_error() {
    let config = PipelineConfig { rspu: true, ..Default::default() };
    let res = run_pipeline(FIXTURE_COUNTER, &config);
    assert!(res.is_ok(), "Expected compilation to succeed, got: {:?}", res.err());
    let compile_res = res.unwrap();
    assert!(compile_res.rspu_program.is_some(), "RSPU program was not generated");
    let program = compile_res.rspu_program.unwrap();
    assert!(!program.instructions.is_empty(), "Expected generated instructions to be non-empty");
}
