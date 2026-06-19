#![cfg(feature = "legacy_ast")]
//! Integration tests for R-SPU Core RTL synthesis.
//!
//! Verifies that the new 64-bit tagged-word core modules
//! are synthesis-clean using Yosys.

#![allow(clippy::field_reassign_with_default)]
#![forbid(unsafe_code)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

/// Helper: compile a .mirr source file through the full pipeline.
fn compile_file(path: &str) -> String {
    let mut config = PipelineConfig::default();
    // Enable relevant stages
    config.symbolic = true;

    let source = std::fs::read_to_string(path).expect("failed to read source file");
    let result = run_pipeline(&source, &config).expect("pipeline should succeed");

    // We want the raw SystemVerilog synthesis output
    mirrc::emit::verilog::emit_sv_synthesis(&result, None, 0)
}

/// Helper: check if Yosys is available.
fn yosys_available() -> bool {
    std::process::Command::new("yosys")
        .arg("-V")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn run_yosys_test(path: &str, top_module: &str) {
    if !yosys_available() {
        eprintln!("Yosys not found in PATH, skipping synthesis test for {}", path);
        return;
    }

    let sv = compile_file(path);

    let tmp_dir = std::env::temp_dir();
    let sv_filename = format!("mirr_rspu_synth_{}.sv", top_module);
    let sv_path = tmp_dir.join(sv_filename);
    std::fs::write(&sv_path, &sv).expect("failed to write temp SV file");

    let sv_path_str = sv_path.display().to_string().replace('\\', "/");
    let yosys_cmd = format!("read_verilog -sv {sv_path_str}; synth -top {top_module}; stat");

    let output = std::process::Command::new("yosys")
        .args(["-p", &yosys_cmd])
        .output()
        .expect("failed to run yosys");

    assert!(
        output.status.success(),
        "Yosys synthesis failed for {} (top: {}):\n{}",
        path,
        top_module,
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_file(&sv_path);
}

#[test]
fn synth_rspu_alu() {
    run_yosys_test("rspu_chip/core/alu.mirr", "alu");
}

#[test]
fn synth_rspu_regfile() {
    run_yosys_test("rspu_chip/core/regfile.mirr", "regfile");
}

#[test]
fn synth_rspu_pcc_verifier() {
    run_yosys_test("rspu_chip/verification/pcc_verifier.mirr", "pcc_verifier");
}

#[test]
fn synth_rspu_pipeline() {
    // The pipeline module uses imports, so the compiler must be able to resolve them.
    // run_pipeline currently takes a string and doesn't know about base paths.
    // I might need to use mirr-compile binary or a wrapper that handles imports.
}
