#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

fn read_text(path: &str) -> String {
    let full_path = Path::new(path);
    fs::read_to_string(full_path)
        .unwrap_or_else(|_| panic!("{} must be readable", full_path.display()))
}

#[test]
fn full_gate_wrappers_keep_canonical_command() {
    let wrappers = ["run_wave_gates.ps1", "run_wave_gates.sh"];
    let expected = "cargo run --bin mirr-general -- ci --format json";

    for path in wrappers {
        let text = read_text(path);
        assert!(text.contains(expected), "wrapper missing canonical gate command: {path}");
    }
}

#[test]
fn full_gate_runs_workspace_nextest_without_fail_fast() {
    let src = read_text("src/bin/mirr-general.rs");
    assert!(src.contains("cargo nextest run --workspace --no-fail-fast"));
}

#[test]
fn full_gate_includes_docs_and_eda_steps() {
    let src = read_text("src/bin/mirr-general.rs");
    assert!(src.contains("RUSTDOCFLAGS=-D warnings cargo doc --no-deps"));
    assert!(src.contains("bash tests/eda/run_eda_tests.sh"));
}

#[test]
fn full_gate_uses_windows_npm_resolution_helper() {
    let src = read_text("src/bin/mirr-general.rs");
    assert!(src.contains("fn npm_command()"));
    assert!(src.contains("npm.cmd"));
}
