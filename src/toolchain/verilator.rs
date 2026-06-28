//! Verilator integration for RTL linting and compiled simulation.
//!
//! - `run_lint()` -- invoke `verilator --lint-only -Wall --sv` for static RTL checks
//! - `run_simulation()` -- compile and run a cycle simulation via `verilator --sv --cc --exe --build`

#![forbid(unsafe_code)]

use crate::toolchain::{invoke_tool, normalize_path_for_mingw, Tool, ToolRegistry, ToolchainError};
use std::path::Path;

/// Result of a Verilator lint or compilation run.
#[derive(Debug, Clone)]
pub struct VerilatorResult {
    /// Whether the run passed (zero errors, exit code 0).
    pub passed: bool,
    /// Number of warnings detected in stderr.
    pub warning_count: usize,
    /// Number of errors detected in stderr.
    pub error_count: usize,
    /// Raw stdout from Verilator.
    pub stdout: String,
    /// Raw stderr from Verilator (where diagnostics appear).
    pub stderr: String,
    /// Process exit code, if available.
    pub exit_code: Option<i32>,
}

/// Result of a Verilator compiled simulation run.
#[derive(Debug, Clone)]
pub struct SimulationResult {
    /// Whether the simulation completed without assertion failures.
    pub passed: bool,
    /// Simulated cycle count, if parseable from output.
    pub cycles: Option<u64>,
    /// Raw stdout from the compiled simulation binary.
    pub stdout: String,
    /// Raw stderr from the compiled simulation binary.
    pub stderr: String,
}

/// Run Verilator lint-only on a SystemVerilog file.
///
/// Invokes `verilator --lint-only -Wall --sv <path>` and counts
/// warnings and errors from stderr.
///
/// # Errors
///
/// Returns `ToolchainError` if Verilator is not available or fails to spawn.
pub fn run_lint(
    sv_path: &Path,
    working_dir: &Path,
    registry: &ToolRegistry,
    link: &[String],
) -> Result<VerilatorResult, ToolchainError> {
    let sv_normalized = normalize_path_for_mingw(sv_path);

    let mut args: Vec<&str> = vec!["--lint-only", "-Wall", "--sv", "-Wno-MULTITOP", "-Wno-UNSIGNED", &sv_normalized];
    for l in link {
        args.push(l);
    }

    let output = invoke_tool(registry, Tool::Verilator, &args, working_dir)?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let warning_count = stderr.matches("%Warning").count();
    let error_count = stderr.matches("%Error").count();

    Ok(VerilatorResult {
        passed: output.status.success(),
        warning_count,
        error_count,
        stdout,
        stderr,
        exit_code: output.status.code(),
    })
}

/// Run a Verilator compiled simulation.
///
/// This is a two-step process:
/// 1. Compile the SystemVerilog to a C++ model with
///    `verilator --sv --cc --exe --build <path>`
/// 2. Run the resulting binary from `obj_dir/V<module_name>`
///
/// # Errors
///
/// Returns `ToolchainError` if Verilator is not available, compilation fails,
/// or the simulation binary cannot be executed.
pub fn run_simulation(
    sv_path: &Path,
    module_name: &str,
    working_dir: &Path,
    registry: &ToolRegistry,
    link: &[String],
) -> Result<SimulationResult, ToolchainError> {
    let sv_normalized = normalize_path_for_mingw(sv_path);

    let has_cpp = link.iter().any(|s| s.ends_with(".cpp"));

    // Step 1: Compile to C++ and build
    let mut args: Vec<&str> = vec![
        "--sv", "--cc", "--exe", "--build",
        "--top-module", module_name,
        "-Wno-UNSIGNED",
        &sv_normalized
    ];
    if !has_cpp {
        args.push("--main");
    }
    for l in link {
        args.push(l);
    }

    let output = invoke_tool(registry, Tool::Verilator, &args, working_dir)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(ToolchainError::ToolFailed {
            tool: "verilator".to_string(),
            exit_code: output.status.code(),
            stderr,
        });
    }

    // Step 2: Run the compiled simulation binary
    let model_path = format!("obj_dir/V{module_name}");
    let run_output =
        std::process::Command::new(&model_path).current_dir(working_dir).output().map_err(|e| {
            ToolchainError::Invocation { tool: format!("V{module_name}"), message: e.to_string() }
        })?;

    let stdout = String::from_utf8_lossy(&run_output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&run_output.stderr).to_string();

    // Try to parse cycle count from "Simulated N cycles" or similar patterns
    let cycles = stdout.lines().find_map(|line| {
        if line.contains("cycles") {
            line.split_whitespace().find_map(|word| word.parse::<u64>().ok())
        } else {
            None
        }
    });

    Ok(SimulationResult { passed: run_output.status.success(), cycles, stdout, stderr })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verilator_result_default() {
        let result = VerilatorResult {
            passed: true,
            warning_count: 0,
            error_count: 0,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: Some(0),
        };
        assert!(result.passed);
        assert_eq!(result.warning_count, 0);
        assert_eq!(result.error_count, 0);
        assert_eq!(result.exit_code, Some(0));
        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn test_lint_path_normalization() {
        // Verify that normalize_path_for_mingw converts backslashes
        // to forward slashes, which is required for MinGW-based Verilator.
        let path = Path::new("C:\\Users\\test\\design.sv");
        let normalized = normalize_path_for_mingw(path);
        assert_eq!(normalized, "C:/Users/test/design.sv");
        assert!(!normalized.contains('\\'));

        // Already-normalized paths should pass through unchanged.
        let unix_path = Path::new("/tmp/design.sv");
        let normalized_unix = normalize_path_for_mingw(unix_path);
        assert_eq!(normalized_unix, "/tmp/design.sv");
    }
}
