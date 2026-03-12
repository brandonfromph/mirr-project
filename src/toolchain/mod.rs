//! OSS-CAD-Suite Toolchain Integration
//!
//! Centralizes tool discovery, path normalization, and invocation
//! for all oss-cad-suite tools used by the MIRR compiler.
//!
//! # Supported Tools
//!
//! - Yosys — synthesis
//! - SymbiYosys (sby) — formal verification
//! - Verilator — RTL linting and compiled simulation
//! - nextpnr — place and route (ice40, ecp5, nexus)
//! - icetime — static timing analysis (iCE40)
//! - EQY — equivalence checking
//!
//! # Path Normalization
//!
//! Windows MinGW builds of oss-cad-suite require forward slashes in
//! all file paths passed to tools. `normalize_path_for_mingw()` handles
//! this centrally.

#![forbid(unsafe_code)]

pub mod eqy;
pub mod formal;
pub mod icetime;
pub mod sby;
pub mod verilator;

use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Output};

/// Maximum number of tools in the registry.
pub const MAX_TOOLS: usize = 32;

/// Maximum length for a tool version string.
pub const MAX_VERSION_LEN: usize = 128;

/// Maximum time (seconds) to wait for a tool invocation.
pub const MAX_TOOL_TIMEOUT_SECS: u64 = 300;

/// Supported tools in the oss-cad-suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tool {
    Yosys,
    Sby,
    Verilator,
    IcarusVerilog,
    NextpnrIce40,
    NextpnrEcp5,
    NextpnrNexus,
    Icepack,
    Icetime,
    Ecppack,
    Eqy,
}

impl Tool {
    /// The command-line binary name for this tool.
    pub fn binary_name(&self) -> &'static str {
        match self {
            Self::Yosys => "yosys",
            Self::Sby => "sby",
            Self::Verilator => "verilator",
            Self::IcarusVerilog => "iverilog",
            Self::NextpnrIce40 => "nextpnr-ice40",
            Self::NextpnrEcp5 => "nextpnr-ecp5",
            Self::NextpnrNexus => "nextpnr-nexus",
            Self::Icepack => "icepack",
            Self::Icetime => "icetime",
            Self::Ecppack => "ecppack",
            Self::Eqy => "eqy",
        }
    }

    /// The flag to get version information.
    pub fn version_flag(&self) -> &'static str {
        match self {
            Self::Verilator => "--version",
            Self::IcarusVerilog => "--version",
            _ => "--version",
        }
    }
}

/// Information about a discovered tool.
#[derive(Debug, Clone)]
pub struct ToolInfo {
    /// The resolved path to the tool binary.
    pub path: String,
    /// Version string (truncated to MAX_VERSION_LEN).
    pub version: String,
    /// Whether the tool is usable.
    pub available: bool,
}

/// Registry of discovered oss-cad-suite tools.
#[derive(Debug, Clone, Default)]
pub struct ToolRegistry {
    /// Map from tool to its info.
    pub tools: HashMap<Tool, ToolInfo>,
}

impl ToolRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }

    /// Probe a tool to check if it's available and get its version.
    pub fn probe(&mut self, tool: Tool) -> bool {
        let binary = tool.binary_name();
        match Command::new(binary).arg(tool.version_flag()).output() {
            Ok(output) => {
                let version_raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let version = if version_raw.len() > MAX_VERSION_LEN {
                    version_raw[..MAX_VERSION_LEN].to_string()
                } else if version_raw.is_empty() {
                    String::from_utf8_lossy(&output.stderr)
                        .lines()
                        .next()
                        .unwrap_or("unknown")
                        .chars()
                        .take(MAX_VERSION_LEN)
                        .collect()
                } else {
                    version_raw
                };
                self.tools
                    .insert(tool, ToolInfo { path: binary.to_string(), version, available: true });
                true
            }
            Err(_) => {
                self.tools.insert(
                    tool,
                    ToolInfo { path: binary.to_string(), version: String::new(), available: false },
                );
                false
            }
        }
    }

    /// Probe all known tools.
    pub fn probe_all(&mut self) {
        let tools = [
            Tool::Yosys,
            Tool::Sby,
            Tool::Verilator,
            Tool::IcarusVerilog,
            Tool::NextpnrIce40,
            Tool::NextpnrEcp5,
            Tool::NextpnrNexus,
            Tool::Icepack,
            Tool::Icetime,
            Tool::Ecppack,
            Tool::Eqy,
        ];
        for tool in tools {
            self.probe(tool);
        }
    }

    /// Check if a tool is available.
    pub fn is_available(&self, tool: Tool) -> bool {
        self.tools.get(&tool).is_some_and(|info| info.available)
    }

    /// Get the version string for a tool.
    pub fn version(&self, tool: Tool) -> Option<&str> {
        self.tools.get(&tool).filter(|info| info.available).map(|info| info.version.as_str())
    }
}

/// Invoke a tool with the given arguments and working directory.
///
/// # Errors
///
/// Returns `ToolchainError::ToolNotFound` if the tool is not in the registry
/// or not available. Returns `ToolchainError::Invocation` if the tool fails
/// to spawn.
pub fn invoke_tool(
    registry: &ToolRegistry,
    tool: Tool,
    args: &[&str],
    working_dir: &Path,
) -> Result<Output, ToolchainError> {
    let info = registry
        .tools
        .get(&tool)
        .filter(|t| t.available)
        .ok_or_else(|| ToolchainError::ToolNotFound { tool: tool.binary_name().to_string() })?;

    Command::new(&info.path).args(args).current_dir(working_dir).output().map_err(|e| {
        ToolchainError::Invocation { tool: tool.binary_name().to_string(), message: e.to_string() }
    })
}

/// Normalize a path for MinGW-based tools (replace backslashes with forward slashes).
///
/// This is essential on Windows where oss-cad-suite tools are MinGW builds
/// that do not handle Windows-native backslash paths.
pub fn normalize_path_for_mingw(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Errors from toolchain operations.
#[derive(Debug, Clone)]
pub enum ToolchainError {
    /// A required tool was not found in the registry or is not available.
    ToolNotFound { tool: String },
    /// The tool failed to execute.
    Invocation { tool: String, message: String },
    /// The tool returned a non-zero exit code.
    ToolFailed { tool: String, exit_code: Option<i32>, stderr: String },
    /// Failed to parse tool output.
    ParseError { tool: String, message: String },
}

impl std::fmt::Display for ToolchainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ToolNotFound { tool } => {
                write!(f, "tool not found: {tool} (is oss-cad-suite in PATH?)")
            }
            Self::Invocation { tool, message } => {
                write!(f, "failed to invoke {tool}: {message}")
            }
            Self::ToolFailed { tool, exit_code, stderr } => {
                write!(
                    f,
                    "{tool} failed (exit code {:?}): {}",
                    exit_code,
                    stderr.lines().next().unwrap_or("(no output)")
                )
            }
            Self::ParseError { tool, message } => {
                write!(f, "failed to parse {tool} output: {message}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path_for_mingw() {
        let path = Path::new("C:\\Users\\test\\project\\output.sv");
        assert_eq!(normalize_path_for_mingw(path), "C:/Users/test/project/output.sv");
    }

    #[test]
    fn test_normalize_path_already_forward() {
        let path = Path::new("/tmp/project/output.sv");
        assert_eq!(normalize_path_for_mingw(path), "/tmp/project/output.sv");
    }

    #[test]
    fn test_tool_binary_names() {
        assert_eq!(Tool::Yosys.binary_name(), "yosys");
        assert_eq!(Tool::Sby.binary_name(), "sby");
        assert_eq!(Tool::Verilator.binary_name(), "verilator");
        assert_eq!(Tool::NextpnrIce40.binary_name(), "nextpnr-ice40");
        assert_eq!(Tool::NextpnrEcp5.binary_name(), "nextpnr-ecp5");
        assert_eq!(Tool::NextpnrNexus.binary_name(), "nextpnr-nexus");
        assert_eq!(Tool::Icetime.binary_name(), "icetime");
        assert_eq!(Tool::Eqy.binary_name(), "eqy");
    }

    #[test]
    fn test_registry_probe_unavailable() {
        let registry = ToolRegistry::new();
        assert!(!registry.is_available(Tool::Yosys));
        assert!(registry.version(Tool::Yosys).is_none());
    }

    #[test]
    fn test_toolchain_error_display() {
        let err = ToolchainError::ToolNotFound { tool: "yosys".into() };
        assert!(err.to_string().contains("yosys"));
        assert!(err.to_string().contains("not found"));
    }
}
