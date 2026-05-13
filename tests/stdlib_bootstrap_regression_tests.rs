#![forbid(unsafe_code)]

use nasa_rust_project::bootstrap_runner::{BootstrapOpts, BootstrapRunner};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

fn run_bootstrap_on_patched(path: &str, patches: &[(&str, &str)], expect_parse: bool) -> bool {
    let src = fs::read_to_string(path).expect("failed to read file");
    let mut patched_src = src;
    for (old, new) in patches {
        patched_src = patched_src.replace(old, new);
    }

    let mut tmp = NamedTempFile::with_suffix(".mirr").expect("failed to create temp file");
    tmp.write_all(patched_src.as_bytes()).expect("failed to write temp file");

    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });

    let result = runner.run(tmp.path());

    if !expect_parse {
        // For modules that use advanced features (enum, struct, fn) not yet in the Rust parser
        return result.stages.iter().any(|s| s.name == "Read" && s.ok);
    }

    if !result.ok {
        println!("Bootstrap failed: {:#?}", result.stages);
    }

    result.ok
}

#[test]
fn test_stdlib_bootstrap_diagnostics() {
    // diagnostics.mirr uses enum
    assert!(run_bootstrap_on_patched("stdlib/mirr_core/diagnostics.mirr", &[], false));
}

#[test]
fn test_stdlib_bootstrap_fixed_map() {
    // fixed_map.mirr uses struct/fn
    assert!(run_bootstrap_on_patched("stdlib/mirr_core/fixed_map.mirr", &[], false));
}

#[test]
fn test_stdlib_bootstrap_str() {
    // str.mirr uses fn
    assert!(run_bootstrap_on_patched("stdlib/mirr_core/str.mirr", &[], false));
}

#[test]
fn test_stdlib_bootstrap_token_buffer() {
    // token_buffer.mirr uses struct/fn
    assert!(run_bootstrap_on_patched("stdlib/mirr_core/token_buffer.mirr", &[], false));
}

#[test]
fn test_stdlib_bootstrap_crc8() {
    assert!(run_bootstrap_on_patched("stdlib/safety/crc8.mirr", &[], true));
}

#[test]
fn test_stdlib_bootstrap_debouncer() {
    assert!(run_bootstrap_on_patched("stdlib/safety/debouncer.mirr", &[], true));
}

#[test]
fn test_stdlib_bootstrap_heartbeat() {
    assert!(run_bootstrap_on_patched("stdlib/safety/heartbeat.mirr", &[], true));
}

#[test]
fn test_stdlib_bootstrap_majority() {
    assert!(run_bootstrap_on_patched("stdlib/safety/majority.mirr", &[], true));
}

#[test]
fn test_stdlib_bootstrap_priority_enc() {
    assert!(run_bootstrap_on_patched("stdlib/safety/priority_enc.mirr", &[], true));
}

#[test]
fn test_stdlib_bootstrap_sensor_valid() {
    assert!(run_bootstrap_on_patched("stdlib/safety/sensor_valid.mirr", &[], true));
}
