//! EQY equivalence checking integration.
//!
//! Generates `.eqy` configuration files and runs equivalence checks
//! between two netlists (e.g., pre-optimization vs post-optimization,
//! or RTL vs gate-level).

#![forbid(unsafe_code)]

use crate::toolchain::{invoke_tool, normalize_path_for_mingw, Tool, ToolRegistry, ToolchainError};
use std::path::Path;

/// Result of an EQY equivalence check.
#[derive(Debug, Clone)]
pub struct EqyResult {
    /// Whether the two designs are equivalent.
    pub equivalent: bool,
    /// If not equivalent, the first divergent signal (if parseable).
    pub divergent_signal: Option<String>,
    /// Raw stdout.
    pub stdout: String,
    /// Raw stderr.
    pub stderr: String,
    /// Exit code.
    pub exit_code: Option<i32>,
}

/// Generate an EQY configuration file for equivalence checking.
///
/// # Arguments
///
/// * `module_name` — Top-level module name (must match in both designs)
/// * `gold_sv` — Path to the golden (reference) SystemVerilog file
/// * `gate_sv` — Path to the gate-level (implementation) SystemVerilog file
pub fn generate_eqy_config(module_name: &str, gold_sv: &Path, gate_sv: &Path) -> String {
    let gold_normalized = normalize_path_for_mingw(gold_sv);
    let gate_normalized = normalize_path_for_mingw(gate_sv);

    let mut out = String::with_capacity(256);

    out.push_str("[gold]\n");
    out.push_str(&format!("read_verilog -sv -formal {gold_normalized}\n"));
    out.push_str(&format!("prep -top {module_name}\n\n"));

    out.push_str("[gate]\n");
    out.push_str(&format!("read_verilog -sv -formal {gate_normalized}\n"));
    out.push_str(&format!("prep -top {module_name}\n\n"));

    out.push_str("[strategy simple]\n");
    out.push_str("use sat\n");
    out.push_str("depth 10\n");

    out
}

/// Run EQY equivalence checking.
///
/// # Errors
///
/// Returns `ToolchainError` if EQY is not available or fails.
pub fn run_equivalence(
    eqy_config_path: &Path,
    working_dir: &Path,
    registry: &ToolRegistry,
) -> Result<EqyResult, ToolchainError> {
    let config_str = normalize_path_for_mingw(eqy_config_path);

    let output = invoke_tool(registry, Tool::Eqy, &[&config_str], working_dir)?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code();

    let equivalent = output.status.success();

    // Try to find divergent signal name from output
    let divergent_signal = if !equivalent {
        stdout
            .lines()
            .chain(stderr.lines())
            .find(|line| line.contains("divergent") || line.contains("FAIL"))
            .map(|line| line.to_string())
    } else {
        None
    };

    Ok(EqyResult { equivalent, divergent_signal, stdout, stderr, exit_code })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eqy_config_generation() {
        let config = generate_eqy_config("test_module", Path::new("gold.sv"), Path::new("gate.sv"));
        assert!(config.contains("[gold]"));
        assert!(config.contains("[gate]"));
        assert!(config.contains("gold.sv"));
        assert!(config.contains("gate.sv"));
        assert!(config.contains("-top test_module"));
        assert!(config.contains("[strategy simple]"));
        assert!(config.contains("use sat"));
    }

    #[test]
    fn test_eqy_config_mingw_paths() {
        let config = generate_eqy_config(
            "test",
            Path::new("C:\\Users\\test\\gold.sv"),
            Path::new("C:\\Users\\test\\gate.sv"),
        );
        assert!(!config.contains('\\'));
        assert!(config.contains("C:/Users/test/gold.sv"));
    }
}
