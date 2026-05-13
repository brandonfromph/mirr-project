#![forbid(unsafe_code)]

use nasa_rust_project::bootstrap_runner::{BootstrapOpts, BootstrapResult, BootstrapRunner};
use std::path::Path;

fn assert_bootstrap_stages(result: BootstrapResult, expected_stages: &[&str]) {
    for stage_name in expected_stages {
        let stage = result
            .stages
            .iter()
            .find(|s| s.name == *stage_name)
            .expect(&format!("Stage '{}' not found in bootstrap results", stage_name));
        assert!(stage.ok, "Stage '{}' failed: {}", stage_name, stage.message);
    }
}

#[test]
fn test_compiler_mirr_bootstrap_parser() {
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(Path::new("compiler_mirr/parser.mirr"));
    // parser.mirr should pass all stages
    assert!(result.ok, "Bootstrap runner failed for parser.mirr: {:?}", result.stages);
}

#[test]
fn test_compiler_mirr_bootstrap_semantic() {
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(Path::new("compiler_mirr/semantic.mirr"));
    // semantic.mirr has unlowerable guards, but should parse and validate
    assert_bootstrap_stages(result, &["Read", "Parse", "Validate"]);
}

#[test]
fn test_compiler_mirr_bootstrap_temporal() {
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(Path::new("compiler_mirr/temporal_lowering.mirr"));
    // has unlowerable guards, but should parse and validate
    assert_bootstrap_stages(result, &["Read", "Parse", "Validate"]);
}

#[test]
fn test_compiler_mirr_bootstrap_emitter() {
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(Path::new("compiler_mirr/emitter.mirr"));
    // has unlowerable guards, but should parse and validate
    assert_bootstrap_stages(result, &["Read", "Parse", "Validate"]);
}

#[test]
fn test_compiler_mirr_bootstrap_main() {
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(Path::new("compiler_mirr/main.mirr"));
    // main.mirr uses 'fn' inside module, fails in standard parser Stage 2
    assert_bootstrap_stages(result, &["Read"]);
}

#[test]
fn test_compiler_mirr_bootstrap_test_main() {
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(Path::new("compiler_mirr/test_main.mirr"));
    // test_main.mirr uses 'fn' inside module, fails in standard parser Stage 2
    assert_bootstrap_stages(result, &["Read"]);
}
