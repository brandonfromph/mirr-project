//! Integration tests for src/toolchain/verilator.rs

#![forbid(unsafe_code)]

use mirrc::toolchain::{verilator::*, Tool, ToolInfo, ToolRegistry};
use std::path::Path;

#[test]
fn test_lint_tool_not_found() {
    let reg = ToolRegistry::new(); // Empty registry
    let result = run_lint(Path::new("dummy.sv"), Path::new("."), &reg);
    assert!(result.is_err());
}

#[test]
fn test_simulation_tool_not_found() {
    let reg = ToolRegistry::new();
    let result = run_simulation(Path::new("dummy.sv"), "test", Path::new("."), &reg);
    assert!(result.is_err());
}

#[test]
fn test_verilator_mocked_execution() {
    let tmp_dir = std::env::temp_dir().join("mirrc_verilator_test");
    std::fs::create_dir_all(&tmp_dir).unwrap();

    // Create a tiny rust program that acts as both our mock Verilator and our mock simulation binary.
    // It prints "%Warning" and "%Error" for the lint test, and "Simulated 42 cycles" for the sim test.
    let mock_src = tmp_dir.join("mock.rs");
    std::fs::write(
        &mock_src,
        r#"
        fn main() {
            eprintln!("%Warning: something is iffy");
            eprintln!("%Warning: another thing");
            eprintln!("%Error: boom");
            println!("Simulated 42 cycles");
        }
    "#,
    )
    .unwrap();

    let mock_bin =
        tmp_dir.join(if cfg!(windows) { "mock_verilator.exe" } else { "mock_verilator" });

    // Compile the mock binary using rustc
    let status = std::process::Command::new("rustc")
        .arg(&mock_src)
        .arg("-o")
        .arg(&mock_bin)
        .status()
        .expect("Failed to run rustc");
    assert!(status.success(), "Failed to compile mock binary");

    let mut reg = ToolRegistry::new();
    reg.tools.insert(
        Tool::Verilator,
        ToolInfo {
            path: mock_bin.to_string_lossy().to_string(),
            version: "mock".to_string(),
            available: true,
        },
    );

    let sv_path = tmp_dir.join("dummy.sv");
    std::fs::write(&sv_path, "module dummy; endmodule").unwrap();

    // 1. Test run_lint
    let lint_result =
        run_lint(&sv_path, &tmp_dir, &reg).expect("Lint should succeed invoking mock");
    assert_eq!(lint_result.warning_count, 2);
    assert_eq!(lint_result.error_count, 1);
    assert!(lint_result.passed); // passed relies on exit code 0, which our mock provides

    // 2. Test run_simulation
    // Step 1 of run_simulation will invoke the mock_bin, which exits 0.
    // Step 2 will look for `obj_dir/Vdummy_mod` and run it.
    // So we must copy our mock_bin to `obj_dir/Vdummy_mod`.
    let obj_dir = tmp_dir.join("obj_dir");
    std::fs::create_dir_all(&obj_dir).unwrap();

    let model_bin_name = if cfg!(windows) { "Vdummy_mod.exe" } else { "Vdummy_mod" };
    let model_bin = obj_dir.join(model_bin_name);
    std::fs::copy(&mock_bin, &model_bin).unwrap();

    let sim_result = run_simulation(&sv_path, "dummy_mod", &tmp_dir, &reg)
        .expect("Simulation should succeed invoking mock");

    assert!(sim_result.passed);
    assert_eq!(sim_result.cycles, Some(42));

    // Cleanup
    let _ = std::fs::remove_dir_all(tmp_dir);
}

#[test]
fn test_simulation_compilation_failure() {
    let mut reg = ToolRegistry::new();
    // Use cargo which will fail with the args we pass to Verilator
    reg.tools.insert(
        Tool::Verilator,
        ToolInfo { path: "cargo".to_string(), version: "mock".to_string(), available: true },
    );

    let result = run_simulation(Path::new("dummy.sv"), "test", Path::new("."), &reg);
    assert!(result.is_err());

    match result.unwrap_err() {
        mirrc::toolchain::ToolchainError::ToolFailed { tool, .. } => {
            assert_eq!(tool, "verilator");
        }
        _ => panic!("Expected ToolFailed error"),
    }
}
