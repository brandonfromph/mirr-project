#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

fn read_text(path: &str) -> String {
    let full_path = Path::new(path);
    fs::read_to_string(full_path)
        .unwrap_or_else(|_| panic!("{} must be readable", full_path.display()))
}

#[test]
fn gate_contract_includes_core_rust_checks() {
    let src = read_text("src/bin/mirr-general.rs");

    for command in [
        "cargo fmt --all -- --check",
        "cargo check --all-targets",
        "cargo clippy --all-targets -- -D warnings",
        "cargo nextest run --workspace --no-fail-fast",
    ] {
        assert!(src.contains(command), "missing gate command: {command}");
    }
}

#[test]
fn gate_contract_includes_consumer_checks() {
    let src = read_text("src/bin/mirr-general.rs");

    for command in [
        "npm --prefix paper/demos pack --dry-run",
        "npm --prefix vscode-mirr pack --dry-run",
        "bash tests/eda/run_eda_tests.sh",
    ] {
        assert!(src.contains(command), "missing consumer gate command: {command}");
    }
}

#[test]
fn gate_contract_wrappers_delegate_to_mirr_general_ci() {
    let ps1 = read_text("run_wave_gates.ps1");
    let sh = read_text("run_wave_gates.sh");

    let expected = "cargo run --bin mirr-general -- ci --format json";
    assert!(ps1.contains(expected));
    assert!(sh.contains(expected));
}

#[test]
fn gate_contract_uses_proposal097_target_dir() {
    let ps1 = read_text("run_wave_gates.ps1");
    let sh = read_text("run_wave_gates.sh");

    assert!(ps1.contains("target/proposal-097-run"));
    assert!(sh.contains("target/proposal-097-run"));
}
