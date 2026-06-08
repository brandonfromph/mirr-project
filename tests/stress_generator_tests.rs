#![forbid(unsafe_code)]
use std::process::Command;

#[test]
fn generator_runs_and_outputs_text_rust() {
    let generator_bin = env!("CARGO_BIN_EXE_generate_mirr_stress");
    let output = Command::new(generator_bin)
        .args(["--type", "mux_forest", "--size", "10"])
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
    let compiler_bin = env!("CARGO_BIN_EXE_mirrc");
    let status = Command::new(compiler_bin)
        .args(["--compile", path])
        .status()
        .expect("failed to invoke compiler");
    assert!(status.success(), "compilation failed for generated MIRR");
}

#[test]
fn generated_templates_compile() {
    // small sizes just to validate syntax
    let types = ["mux_forest", "temporal_chain", "width_chain"];
    for typ in types {
        let generator_bin = env!("CARGO_BIN_EXE_generate_mirr_stress");
        let output = Command::new(generator_bin)
            .args(["--type", typ, "--size", "5"])
            .output()
            .expect("failed to execute rust generator");
        assert!(output.status.success());
        let code = String::from_utf8_lossy(&output.stdout);
        compile_mirr(&code);
    }
}
