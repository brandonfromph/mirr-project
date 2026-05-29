use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
const FIXTURE_SIGNAL: &str = r#"
module test_signals {
    signals {
        a: in bool
        b: out u8
    }
}
"#;
#[test]
fn debug_pipeline_signal_error() {
    let config = PipelineConfig { rspu: true, ..Default::default() };
    let res = run_pipeline(FIXTURE_SIGNAL, &config);
    assert!(res.is_ok(), "Expected compilation to succeed, got: {:?}", res.err());
    let compile_res = res.unwrap();
    assert!(compile_res.rspu_program.is_some(), "RSPU program was not generated");
    let program = compile_res.rspu_program.unwrap();
    assert!(!program.instructions.is_empty(), "Expected generated instructions to be non-empty");
}
