//! SymbiYosys (sby) formal verification integration.
//!
//! Generates `.sby` configuration files and runs formal verification
//! using the existing SVA bind file infrastructure.
//!
//! Supports BMC (bounded model checking) and k-induction prove modes
//! with configurable engine selection (Z3, Yices, Bitwuzla, Boolector).

#![forbid(unsafe_code)]

use crate::toolchain::{invoke_tool, normalize_path_for_mingw, Tool, ToolRegistry, ToolchainError};
use std::path::Path;

/// Maximum BMC depth to prevent runaway verification.
pub const MAX_BMC_DEPTH: u32 = 200;

/// Default BMC depth when not specified.
pub const DEFAULT_BMC_DEPTH: u32 = 20;

/// Maximum number of solver engines in one sby run.
pub const MAX_ENGINES: usize = 4;

/// Supported SMT solver engines for sby.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbyEngine {
    /// Z3 SMT solver.
    Z3,
    /// Yices 2 SMT solver.
    Yices,
    /// Bitwuzla SMT solver.
    Bitwuzla,
    /// Boolector SMT solver (btor backend).
    Boolector,
}

impl SbyEngine {
    /// Engine name as expected by sby config.
    pub fn engine_name(&self) -> &'static str {
        match self {
            Self::Z3 => "smtbmc z3",
            Self::Yices => "smtbmc yices",
            Self::Bitwuzla => "smtbmc bitwuzla",
            Self::Boolector => "btor btormc",
        }
    }

    /// Parse engine name from CLI string.
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s {
            "z3" => Some(Self::Z3),
            "yices" => Some(Self::Yices),
            "bitwuzla" => Some(Self::Bitwuzla),
            "btor" | "boolector" => Some(Self::Boolector),
            _ => None,
        }
    }
}

/// Configuration for an sby formal verification run.
#[derive(Debug, Clone)]
pub struct SbyConfig {
    /// BMC depth (bounded model checking).
    pub bmc_depth: u32,
    /// Whether to also run k-induction prove.
    pub prove: bool,
    /// Solver engine to use.
    pub engine: SbyEngine,
}

impl Default for SbyConfig {
    fn default() -> Self {
        Self { bmc_depth: DEFAULT_BMC_DEPTH, prove: false, engine: SbyEngine::Z3 }
    }
}

/// Result of an sby formal verification run.
#[derive(Debug, Clone)]
pub struct SbyResult {
    /// Whether all properties passed.
    pub passed: bool,
    /// Raw stdout from sby.
    pub stdout: String,
    /// Raw stderr from sby.
    pub stderr: String,
    /// Exit code.
    pub exit_code: Option<i32>,
}

/// Generate an sby configuration file for formal verification.
///
/// # Arguments
///
/// * `module_name` — Top-level module name
/// * `sv_path` — Path to the synthesis-clean SystemVerilog file
/// * `bind_path` — Path to the SVA bind file (optional)
/// * `config` — Formal verification configuration
pub fn generate_sby_config(
    module_name: &str,
    sv_path: &Path,
    bind_path: Option<&Path>,
    config: &SbyConfig,
) -> String {
    let depth = config.bmc_depth.min(MAX_BMC_DEPTH);
    let mut out = String::with_capacity(512);

    // Tasks section
    out.push_str("[tasks]\n");
    out.push_str("bmc\n");
    if config.prove {
        out.push_str("prove\n");
    }
    out.push('\n');

    // Options section
    out.push_str("[options]\n");
    out.push_str("bmc: mode bmc\n");
    out.push_str(&format!("bmc: depth {depth}\n"));
    if config.prove {
        out.push_str("prove: mode prove\n");
        out.push_str(&format!("prove: depth {depth}\n"));
    }
    out.push('\n');

    // Engines section
    out.push_str("[engines]\n");
    out.push_str(config.engine.engine_name());
    out.push('\n');
    out.push('\n');

    // Script section
    out.push_str("[script]\n");
    let sv_normalized = normalize_path_for_mingw(sv_path);
    out.push_str(&format!("read_verilog -sv -formal {sv_normalized}\n"));
    if let Some(bind) = bind_path {
        let bind_normalized = normalize_path_for_mingw(bind);
        out.push_str(&format!("read_verilog -sv -formal {bind_normalized}\n"));
    }
    out.push_str(&format!("prep -top {module_name}\n"));
    out.push('\n');

    // Files section
    out.push_str("[files]\n");
    out.push_str(&format!("{sv_normalized}\n"));
    if let Some(bind) = bind_path {
        let bind_normalized = normalize_path_for_mingw(bind);
        out.push_str(&format!("{bind_normalized}\n"));
    }

    out
}

/// Run sby formal verification.
///
/// # Errors
///
/// Returns `ToolchainError` if sby is not available or fails to execute.
pub fn run_formal(
    sby_config_path: &Path,
    working_dir: &Path,
    registry: &ToolRegistry,
) -> Result<SbyResult, ToolchainError> {
    let config_str = normalize_path_for_mingw(sby_config_path);

    let output = invoke_tool(registry, Tool::Sby, &["-f", &config_str], working_dir)?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code();

    let passed = output.status.success();

    Ok(SbyResult { passed, stdout, stderr, exit_code })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sby_config_generation_bmc_only() {
        let config = SbyConfig::default();
        let result = generate_sby_config("test_module", Path::new("test.sv"), None, &config);
        assert!(result.contains("[tasks]"));
        assert!(result.contains("bmc"));
        assert!(result.contains("mode bmc"));
        assert!(result.contains("depth 20"));
        assert!(result.contains("smtbmc z3"));
        assert!(result.contains("test.sv"));
        assert!(result.contains("-top test_module"));
        assert!(!result.contains("prove"));
    }

    #[test]
    fn test_sby_config_generation_with_prove() {
        let config = SbyConfig { prove: true, ..Default::default() };
        let result = generate_sby_config(
            "my_module",
            Path::new("my_module.sv"),
            Some(Path::new("my_module_sva.sv")),
            &config,
        );
        assert!(result.contains("prove"));
        assert!(result.contains("mode prove"));
        assert!(result.contains("my_module_sva.sv"));
    }

    #[test]
    fn test_sby_config_depth_clamped() {
        let config = SbyConfig { bmc_depth: 999, ..Default::default() };
        let result = generate_sby_config("test", Path::new("test.sv"), None, &config);
        assert!(result.contains(&format!("depth {}", MAX_BMC_DEPTH)));
    }

    #[test]
    fn test_engine_names() {
        assert_eq!(SbyEngine::Z3.engine_name(), "smtbmc z3");
        assert_eq!(SbyEngine::Yices.engine_name(), "smtbmc yices");
        assert_eq!(SbyEngine::Bitwuzla.engine_name(), "smtbmc bitwuzla");
        assert_eq!(SbyEngine::Boolector.engine_name(), "btor btormc");
    }

    #[test]
    fn test_engine_from_str() {
        assert_eq!(SbyEngine::from_str_name("z3"), Some(SbyEngine::Z3));
        assert_eq!(SbyEngine::from_str_name("yices"), Some(SbyEngine::Yices));
        assert_eq!(SbyEngine::from_str_name("btor"), Some(SbyEngine::Boolector));
        assert_eq!(SbyEngine::from_str_name("unknown"), None);
    }

    #[test]
    fn test_sby_config_mingw_paths() {
        let config = SbyConfig::default();
        let result =
            generate_sby_config("test", Path::new("C:\\Users\\test\\output.sv"), None, &config);
        // Backslashes should be normalized
        assert!(!result.contains('\\'));
    }
}
