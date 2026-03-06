use std::process::Command;

#[test]
fn generator_runs_and_outputs_text_python() {
    let output = Command::new("python")
        .arg("scripts/generate_mirr_stress.py")
        .arg("--type")
        .arg("mux_forest")
        .arg("--size")
        .arg("10")
        .output()
        .expect("failed to execute script");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("module mux_forest"));
}

#[test]
fn generator_runs_and_outputs_text_rust() {
    // use the Rust binary we just added
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "generate_mirr_stress", "--", "--type", "mux_forest", "--size", "10"])
        .output()
        .expect("failed to execute rust generator");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // generator now emits real MIRR; check for the module header
    assert!(stdout.contains("module mux_forest"));
}

// helper: compile generated MIRR code to ensure syntax is valid
fn compile_mirr(code: &str) {
    use std::io::Write;
    let mut file = tempfile::NamedTempFile::new().expect("tempfile");
    write!(file, "{}", code).expect("write");
    let path = file.path().to_str().unwrap();
    let status = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "nasa-rust-project", "--", "--compile", path])
        .status()
        .expect("failed to invoke compiler");
    assert!(status.success(), "compilation failed for generated MIRR");
}

#[test]
fn generated_templates_compile() {
    // small sizes just to validate syntax; full stress tests use larger sizes in research scripts
    let types = ["mux_forest", "temporal_chain", "width_chain"];
    for typ in types {
        let output = Command::new("cargo")
            .args(["run", "--quiet", "--bin", "generate_mirr_stress", "--", "--type", typ, "--size", "5"])
            .output()
            .expect("failed to execute rust generator");
        assert!(output.status.success());
        let code = String::from_utf8_lossy(&output.stdout);
        compile_mirr(&code);
    }
}
