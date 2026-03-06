// Ensure the Python stress-test generator produces valid MIRR that the
// Rust parser can consume.  This tests the integration of the helper script
// produced earlier and guards against accidental syntax regressions.

use std::process::Command;
use nasa_rust_project::parser::parse_mirr;

#[test]
fn python_stress_generator_outputs_valid_mirr() {
    // Invoke the Rust binary shipping with the workspace.  `cargo run` is
    // convenient because it builds the binary if needed and prints to stdout.
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "generate_mirr_stress", "--", "--type", "mux_forest", "--size", "10"])
        .output()
        .expect("failed to run rust generator");
    assert!(output.status.success(), "generator binary failed to run");

    let code = String::from_utf8(output.stdout).expect("non-UTF8 output");
    // Parsing should succeed and produce a module with at least one signal.
    let program = parse_mirr(&code).expect("parser rejected generator output");
    assert!(!program.module.signals.is_empty(), "generated module had no signals");
}
