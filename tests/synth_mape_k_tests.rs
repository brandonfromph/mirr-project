//! Integration tests for MAPE-K RTL synthesis.
//!
//! Verifies that the generated MAPE-K SystemVerilog modules
//! are synthesis-clean using Yosys.

#![forbid(unsafe_code)]

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

/// Helper: compile a .mirr source string through the full pipeline with MAPE-K RTL enabled.
fn compile_mape_k(source: &str) -> String {
    let config = PipelineConfig { mape_k: true, emit_mape_k_rtl: true, ..Default::default() };

    let result = run_pipeline(source, &config).expect("pipeline should succeed");
    result.mape_k_rtl.expect("MAPE-K RTL should be emitted")
}

/// Helper: check if Yosys is available and functional on this system.
fn yosys_available() -> bool {
    std::process::Command::new("yosys")
        .arg("-V")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[test]
fn synth_mape_k_neonatal_respirator() {
    if !yosys_available() {
        eprintln!("Yosys not found in PATH, skipping MAPE-K synthesis test");
        return;
    }

    let source = include_str!("../examples/neonatal_respirator.mirr");
    let sv = compile_mape_k(source);

    // Write to a temp file.
    let tmp_dir = std::env::temp_dir();
    let sv_path = tmp_dir.join("mirr_mape_k_synth_test.sv");
    std::fs::write(&sv_path, &sv).expect("failed to write temp SV file");

    // Yosys (MinGW) requires forward slashes on Windows.
    let sv_path_str = sv_path.display().to_string().replace('\\', "/");
    let yosys_cmd = format!("read_verilog -sv {sv_path_str}; synth -top mirr_mape_k_top; stat");

    let output = std::process::Command::new("yosys")
        .args(["-p", &yosys_cmd])
        .output()
        .expect("failed to run yosys");

    assert!(
        output.status.success(),
        "Yosys synthesis failed for MAPE-K RTL (neonatal_respirator):\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Clean up.
    let _ = std::fs::remove_file(&sv_path);
}

#[test]
fn synth_mape_k_icu_monitor() {
    if !yosys_available() {
        return;
    }

    let source = include_str!("../examples/icu_monitor.mirr");
    let sv = compile_mape_k(source);

    let tmp_dir = std::env::temp_dir();
    let sv_path = tmp_dir.join("mirr_mape_k_icu_test.sv");
    std::fs::write(&sv_path, &sv).expect("failed to write temp SV file");

    let sv_path_str = sv_path.display().to_string().replace('\\', "/");
    let yosys_cmd = format!("read_verilog -sv {sv_path_str}; synth -top mirr_mape_k_top; stat");

    let output = std::process::Command::new("yosys")
        .args(["-p", &yosys_cmd])
        .output()
        .expect("failed to run yosys");

    assert!(
        output.status.success(),
        "Yosys synthesis failed for MAPE-K RTL (icu_monitor):\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_file(&sv_path);
}
