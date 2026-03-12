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

/// Maximum lines to scan in sby output.
pub const MAX_SBY_PARSE_LINES: usize = 10_000;

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
    /// Whether to also run cover mode.
    pub cover: bool,
    /// Solver engine to use.
    pub engine: SbyEngine,
}

impl Default for SbyConfig {
    fn default() -> Self {
        Self { bmc_depth: DEFAULT_BMC_DEPTH, prove: false, cover: false, engine: SbyEngine::Z3 }
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
    if config.cover {
        out.push_str("cover\n");
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
    if config.cover {
        out.push_str("cover: mode cover\n");
        out.push_str(&format!("cover: depth {depth}\n"));
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

/// Status of a single formal property check.
#[derive(Debug, Clone, PartialEq)]
pub enum FormalStatus {
    Pass,
    Fail,
    Unknown,
}

/// Result of a single property verdict from sby.
#[derive(Debug, Clone)]
pub struct PropertyVerdict {
    pub name: String,
    pub task: String,
    pub status: FormalStatus,
    pub cycle: Option<u64>,
}

/// Parse sby stdout into per-property verdicts.
///
/// Scans for lines matching patterns like:
///   `SBY HH:MM:SS [task] engine_N.check: STATUS`
/// Bounded by `MAX_SBY_PARSE_LINES`.
pub fn parse_sby_output(stdout: &str) -> Vec<PropertyVerdict> {
    let mut verdicts = Vec::new();

    for (lines_scanned, line) in stdout.lines().enumerate() {
        if lines_scanned >= MAX_SBY_PARSE_LINES {
            break;
        }

        let trimmed = line.trim();
        if !trimmed.starts_with("SBY") {
            continue;
        }

        // Extract task name from [task]
        let bracket_open = match trimmed.find('[') {
            Some(i) => i,
            None => continue,
        };
        let bracket_close = match trimmed[bracket_open..].find(']') {
            Some(i) => bracket_open + i,
            None => continue,
        };
        let task = &trimmed[bracket_open + 1..bracket_close];

        // After "] " find the property name and status
        let after_bracket = &trimmed[bracket_close + 1..].trim_start();

        // Split on ": " to get "engine_N.check" and "STATUS ..."
        let colon_pos = match after_bracket.find(": ") {
            Some(i) => i,
            None => continue,
        };
        let name = &after_bracket[..colon_pos];
        let status_part = &after_bracket[colon_pos + 2..];

        let status = if status_part.starts_with("PASS") {
            FormalStatus::Pass
        } else if status_part.starts_with("FAIL") {
            FormalStatus::Fail
        } else if status_part.starts_with("UNKNOWN") {
            FormalStatus::Unknown
        } else {
            continue;
        };

        // Extract failing cycle from "(step N)"
        let cycle = if let Some(step_start) = status_part.find("(step ") {
            let after_step = &status_part[step_start + 6..];
            after_step.split(')').next().and_then(|s| s.parse::<u64>().ok())
        } else {
            None
        };

        verdicts.push(PropertyVerdict {
            name: name.to_string(),
            task: task.to_string(),
            status,
            cycle,
        });
    }

    verdicts
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

    #[test]
    fn test_sby_config_generation_with_cover() {
        let config = SbyConfig { cover: true, ..Default::default() };
        let result = generate_sby_config("cov_mod", Path::new("cov.sv"), None, &config);
        assert!(result.contains("cover\n"));
        assert!(result.contains("cover: mode cover\n"));
        assert!(result.contains("cover: depth 20\n"));
        assert!(result.contains("bmc\n"));
        assert!(result.contains("bmc: mode bmc\n"));
    }

    #[test]
    fn test_sby_config_generation_all_tasks() {
        let config = SbyConfig { prove: true, cover: true, ..Default::default() };
        let result = generate_sby_config("all_mod", Path::new("all.sv"), None, &config);
        assert!(result.contains("bmc\n"));
        assert!(result.contains("prove\n"));
        assert!(result.contains("cover\n"));
        assert!(result.contains("bmc: mode bmc\n"));
        assert!(result.contains("prove: mode prove\n"));
        assert!(result.contains("cover: mode cover\n"));
    }

    #[test]
    fn test_parse_sby_output_pass() {
        let stdout = "SBY  0:00:03 [bmc] engine_0.basecase: PASS\n\
                       SBY  0:00:05 [prove] engine_0.induction: PASS\n";
        let verdicts = parse_sby_output(stdout);
        assert_eq!(verdicts.len(), 2);
        assert_eq!(verdicts[0].task, "bmc");
        assert_eq!(verdicts[0].name, "engine_0.basecase");
        assert_eq!(verdicts[0].status, FormalStatus::Pass);
        assert_eq!(verdicts[0].cycle, None);
        assert_eq!(verdicts[1].task, "prove");
        assert_eq!(verdicts[1].status, FormalStatus::Pass);
    }

    #[test]
    fn test_parse_sby_output_fail_with_step() {
        let stdout = "SBY  0:00:04 [bmc] engine_0.basecase: FAIL (step 12)\n";
        let verdicts = parse_sby_output(stdout);
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].status, FormalStatus::Fail);
        assert_eq!(verdicts[0].cycle, Some(12));
    }

    #[test]
    fn test_parse_sby_output_cover() {
        let stdout = "SBY  0:00:02 [cover] engine_0.cover: PASS (1 traces)\n";
        let verdicts = parse_sby_output(stdout);
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].task, "cover");
        assert_eq!(verdicts[0].status, FormalStatus::Pass);
        assert_eq!(verdicts[0].cycle, None);
    }

    #[test]
    fn test_parse_sby_output_unknown() {
        let stdout = "SBY  0:00:10 [bmc] engine_0.basecase: UNKNOWN\n";
        let verdicts = parse_sby_output(stdout);
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].status, FormalStatus::Unknown);
    }

    #[test]
    fn test_parse_sby_output_skips_non_sby_lines() {
        let stdout = "some random log line\n\
                       INFO: starting verification\n\
                       SBY  0:00:03 [bmc] engine_0.basecase: PASS\n\
                       another line\n";
        let verdicts = parse_sby_output(stdout);
        assert_eq!(verdicts.len(), 1);
        assert_eq!(verdicts[0].task, "bmc");
    }

    #[test]
    fn test_parse_sby_output_empty() {
        let verdicts = parse_sby_output("");
        assert!(verdicts.is_empty());
    }
}
