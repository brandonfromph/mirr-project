//! Integration tests for src/toolchain/optimize.rs

#![forbid(unsafe_code)]

use mirrc::toolchain::{optimize::*, Tool, ToolInfo, ToolRegistry};
use std::path::Path;

#[test]
fn test_optimize_tool_not_found() {
    let reg = ToolRegistry::new(); // Empty registry
    let result = run_logic_optimization(&reg, Path::new("dummy.sv"), "top", Path::new("."));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("yosys"));
}

#[test]
fn test_optimize_tool_invocation_success_or_fail() {
    let mut reg = ToolRegistry::new();

    // We mock Yosys by pointing it to `cargo` which is guaranteed to be in PATH.
    // It will be invoked as `cargo -q -s <script.ys>`, which will fail with an error,
    // but the `run_logic_optimization` function itself will successfully return an
    // `OptimizeResult` with `success: false`.
    reg.tools.insert(
        Tool::Yosys,
        ToolInfo { path: "cargo".to_string(), version: "mock".to_string(), available: true },
    );

    let tmp_dir = std::env::temp_dir();
    let sv_path = tmp_dir.join("test_module_synth.sv");

    // Write dummy input to avoid missing file errors if anything reads it
    // (though optimize.rs only creates the script and invokes the tool).
    std::fs::write(&sv_path, "module test_module; endmodule").unwrap();

    let result = run_logic_optimization(&reg, &sv_path, "test_module", &tmp_dir);

    assert!(result.is_ok(), "Expected Ok since the tool was invoked");
    let opt_result = result.unwrap();

    // Cargo will fail with invalid arguments, so success should be false.
    assert!(!opt_result.success);

    // The script should have been written to the parent directory of sv_path.
    let script_path = tmp_dir.join("test_module_opt.ys");
    assert!(script_path.exists(), "Yosys script should have been generated");

    let script_content = std::fs::read_to_string(&script_path).unwrap();
    assert!(script_content.contains("read_verilog"));
    assert!(script_content.contains("test_module"));
    assert!(script_content.contains("abc -g"));

    // The output path string inside the result should reflect the correct pattern.
    // Notice how "_synth" was stripped from "test_module_synth.sv"
    // to form "test_module_opt_synth.sv".
    assert!(opt_result.optimized_path.contains("test_module_opt_synth.sv"));

    // Cleanup
    let _ = std::fs::remove_file(sv_path);
    let _ = std::fs::remove_file(script_path);
}
