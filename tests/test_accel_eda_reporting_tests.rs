#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

fn eda_script() -> String {
    let path = Path::new("tests/eda/run_eda_tests.sh");
    fs::read_to_string(path).expect("EDA script must be readable")
}

#[test]
fn eda_script_has_shell_entrypoint() {
    let script = eda_script();
    assert!(script.starts_with("#!/usr/bin/env bash"));
}

#[test]
fn eda_script_uses_bounded_iteration_guard() {
    let script = eda_script();
    assert!(script.contains("MAX_TESTS=200"));
    assert!(script.contains("if [ \"$TOTAL\" -gt \"$MAX_TESTS\" ]"));
}

#[test]
fn eda_script_reports_pass_fail_skip_counters() {
    let script = eda_script();
    assert!(script.contains("PASS=0; FAIL=0; SKIP=0; TOTAL=0"));
    assert!(script.contains("PASS:"));
    assert!(script.contains("FAIL:"));
    assert!(script.contains("SKIP:"));
}

#[test]
fn eda_script_performs_tool_availability_checks() {
    let script = eda_script();
    assert!(script.contains("for tool in yosys iverilog vvp sby"));
    assert!(script.contains("command -v \"$tool\""));
}
