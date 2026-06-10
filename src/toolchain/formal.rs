#![forbid(unsafe_code)]

//! Formal verification orchestration.
//!
//! Extracts the SymbiYosys-based formal verification flow from
//! mirr-compile into a reusable module.

use std::path::Path;

use crate::toolchain::sby::{SbyConfig, SbyEngine};
use crate::toolchain::{Tool, ToolRegistry, ToolchainError};

/// Maximum BMC depth for formal verification.
pub const MAX_FORMAL_DEPTH: u32 = 200;

/// Maximum number of properties to verify in one run.
pub const MAX_FORMAL_PROPERTIES: usize = 256;

/// Maximum number of stdout lines to scan for verdicts.
const MAX_VERDICT_LINES: usize = 4096;

/// Configuration for a formal verification run.
#[derive(Debug, Clone)]
pub struct FormalConfig {
    /// BMC depth (default: 20, max: MAX_FORMAL_DEPTH).
    pub bmc_depth: u32,
    /// Whether to also run k-induction prove.
    pub prove: bool,
    /// Solver engine.
    pub engine: SbyEngine,
    /// Path to the SystemVerilog source file.
    pub sv_path: String,
    /// Path to the SVA bind file (if any).
    pub bind_path: Option<String>,
    /// Extra Verilog files to link.
    pub extra_files: Vec<String>,
}

impl Default for FormalConfig {
    fn default() -> Self {
        Self {
            bmc_depth: 20,
            prove: false,
            engine: SbyEngine::Z3,
            sv_path: String::new(),
            bind_path: None,
            extra_files: Vec::new(),
        }
    }
}

/// Result of a formal verification run.
#[derive(Debug, Clone)]
pub struct FormalResult {
    /// sby exit code.
    pub exit_code: Option<i32>,
    /// Whether all checks passed.
    pub passed: bool,
    /// Per-property verdicts parsed from sby output.
    pub verdicts: Vec<PropertyVerdict>,
    /// Raw sby stdout.
    pub stdout: String,
    /// Raw sby stderr.
    pub stderr: String,
}

/// Per-property verdict from sby output.
#[derive(Debug, Clone)]
pub struct PropertyVerdict {
    /// Property or assertion name.
    pub name: String,
    /// Task that produced this verdict (e.g. "bmc", "prove").
    pub task: String,
    /// Pass / Fail / Unknown.
    pub status: FormalStatus,
}

/// Formal verification status for a single property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormalStatus {
    Pass,
    Fail,
    Unknown,
}

/// Run the formal verification pipeline.
///
/// 1. Check sby is available in registry.
/// 2. Generate sby config file.
/// 3. Run sby.
/// 4. Parse per-property verdicts from stdout.
pub fn run_formal_pipeline(
    registry: &ToolRegistry,
    config: &FormalConfig,
    module_name: &str,
    working_dir: &Path,
) -> Result<FormalResult, ToolchainError> {
    // 1. Check sby availability.
    if !registry.is_available(Tool::Sby) {
        return Err(ToolchainError::ToolNotFound { tool: "sby".to_string() });
    }

    // 2. Clamp depth and build sby-level config.
    let depth = config.bmc_depth.min(MAX_FORMAL_DEPTH);
    let sby_cfg = SbyConfig {
        bmc_depth: depth,
        prove: config.prove,
        cover: false,
        engine: config.engine,
        extra_files: config.extra_files.clone(),
    };

    let sv_path = Path::new(&config.sv_path);
    let bind_path_buf; // keep the Path alive if needed
    let bind_ref = match config.bind_path {
        Some(ref p) => {
            bind_path_buf = p.clone();
            Some(Path::new(bind_path_buf.as_str()))
        }
        None => None,
    };

    let sby_content =
        crate::toolchain::sby::generate_sby_config(module_name, sv_path, bind_ref, &sby_cfg);

    // Write sby config next to the SV file.
    let sby_path = derive_sby_path(&config.sv_path);
    std::fs::write(&sby_path, &sby_content).map_err(|e| ToolchainError::Invocation {
        tool: "sby".to_string(),
        message: format!("failed to write config {sby_path}: {e}"),
    })?;

    // 3. Run sby.
    let sby_result =
        crate::toolchain::sby::run_formal(Path::new(&sby_path), working_dir, registry)?;

    // 4. Parse verdicts from stdout.
    let verdicts = parse_sby_verdicts(&sby_result.stdout);

    Ok(FormalResult {
        exit_code: sby_result.exit_code,
        passed: sby_result.passed,
        verdicts,
        stdout: sby_result.stdout,
        stderr: sby_result.stderr,
    })
}

/// Derive the `.sby` config path from the SV source path.
fn derive_sby_path(sv_path: &str) -> String {
    if let Some(dot) = sv_path.rfind('.') {
        format!("{}.sby", &sv_path[..dot])
    } else {
        format!("{sv_path}.sby")
    }
}

/// Parse per-property verdicts from sby stdout.
///
/// sby emits lines like:
///   `SBY  0:00:01 [bmc] engine_0: PASS`
///   `SBY  0:00:02 [prove] engine_0: FAIL`
///   `SBY  0:00:03 [bmc] engine_0.basecase: PASS`
///
/// Iteration is bounded by `MAX_VERDICT_LINES`.
fn parse_sby_verdicts(stdout: &str) -> Vec<PropertyVerdict> {
    let mut verdicts = Vec::new();

    for (lines_scanned, line) in stdout.lines().enumerate() {
        if lines_scanned >= MAX_VERDICT_LINES {
            break;
        }

        // Match lines containing task verdict pattern: "[task] engine_N: STATUS"
        let trimmed = line.trim();
        if !trimmed.starts_with("SBY") {
            continue;
        }

        // Look for "[task]" bracket section.
        let bracket_start = match trimmed.find('[') {
            Some(i) => i,
            None => continue,
        };
        let bracket_end = match trimmed[bracket_start..].find(']') {
            Some(i) => bracket_start + i,
            None => continue,
        };
        let task = &trimmed[bracket_start + 1..bracket_end];

        // Look for "PASS" or "FAIL" after the bracket.
        let after_bracket = &trimmed[bracket_end + 1..];
        let status = if after_bracket.contains("PASS") {
            FormalStatus::Pass
        } else if after_bracket.contains("FAIL") {
            FormalStatus::Fail
        } else {
            continue; // Not a verdict line.
        };

        // Extract the engine/property name (text between "] " and ": PASS/FAIL").
        let name = after_bracket.trim().split(": ").next().unwrap_or("unknown").to_string();

        if verdicts.len() >= MAX_FORMAL_PROPERTIES {
            break;
        }
        verdicts.push(PropertyVerdict { name, task: task.to_string(), status });
    }

    verdicts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = FormalConfig::default();
        assert_eq!(cfg.bmc_depth, 20);
        assert!(!cfg.prove);
        assert_eq!(cfg.engine, SbyEngine::Z3);
        assert!(cfg.sv_path.is_empty());
        assert!(cfg.bind_path.is_none());
    }

    #[test]
    fn test_derive_sby_path() {
        assert_eq!(derive_sby_path("foo_synth.sv"), "foo_synth.sby");
        assert_eq!(derive_sby_path("no_ext"), "no_ext.sby");
    }

    #[test]
    fn test_parse_verdicts_pass() {
        let stdout = "SBY  0:00:01 [bmc] engine_0: PASS\n";
        let verdicts = parse_sby_verdicts(stdout);
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].task, "bmc");
        assert_eq!(verdicts[0].status, FormalStatus::Pass);
        assert_eq!(verdicts[0].name, "engine_0");
    }

    #[test]
    fn test_parse_verdicts_fail() {
        let stdout = "SBY  0:00:02 [prove] engine_0: FAIL\n";
        let verdicts = parse_sby_verdicts(stdout);
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].task, "prove");
        assert_eq!(verdicts[0].status, FormalStatus::Fail);
    }

    #[test]
    fn test_parse_verdicts_mixed() {
        let stdout = "\
SBY  0:00:01 [bmc] engine_0: PASS
SBY  0:00:02 [prove] engine_0: FAIL
SBY  0:00:03 [bmc] Summary: some info
SBY  0:00:04 [bmc] engine_0.basecase: PASS
";
        let verdicts = parse_sby_verdicts(stdout);
        assert_eq!(verdicts.len(), 3);
        assert_eq!(verdicts[0].status, FormalStatus::Pass);
        assert_eq!(verdicts[1].status, FormalStatus::Fail);
        assert_eq!(verdicts[2].status, FormalStatus::Pass);
    }

    #[test]
    fn test_parse_verdicts_ignores_non_sby_lines() {
        let stdout = "INFO: reading file\nDone.\n";
        let verdicts = parse_sby_verdicts(stdout);
        assert!(verdicts.is_empty());
    }

    #[test]
    fn test_depth_clamp_in_config() {
        // Verify MAX_FORMAL_DEPTH matches sby's MAX_BMC_DEPTH.
        assert_eq!(MAX_FORMAL_DEPTH, crate::toolchain::sby::MAX_BMC_DEPTH);
    }
}
