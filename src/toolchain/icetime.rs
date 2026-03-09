//! icetime static timing analysis integration (iCE40 only).
//!
//! Parses icetime output to extract Fmax and critical path delay
//! for post-PnR timing closure verification.
//!
//! - `run_timing()` -- invoke `icetime -d <device> <asc_path>`
//! - `parse_frequency()` -- extract MHz from icetime output

#![forbid(unsafe_code)]

use crate::toolchain::{invoke_tool, normalize_path_for_mingw, Tool, ToolRegistry, ToolchainError};
use std::path::Path;

/// Result of a static timing analysis run.
#[derive(Debug, Clone)]
pub struct TimingResult {
    /// Maximum frequency in MHz, if parseable from output.
    pub max_frequency_mhz: Option<f64>,
    /// Critical path delay in nanoseconds, if parseable from output.
    pub critical_path_ns: Option<f64>,
    /// Whether the timing analysis completed successfully.
    pub passed: bool,
    /// Raw stdout from icetime.
    pub stdout: String,
    /// Raw stderr from icetime.
    pub stderr: String,
}

/// Run icetime static timing analysis on an ASC bitstream file.
///
/// Invokes `icetime -d <device> <asc_path>` and parses the output
/// for frequency and critical path information.
///
/// # Arguments
///
/// * `asc_path` -- Path to the iCE40 ASCII bitstream file (.asc)
/// * `device` -- Device string (e.g., "hx8k", "lp1k")
/// * `working_dir` -- Working directory for the command
/// * `registry` -- Tool registry for locating icetime
///
/// # Errors
///
/// Returns `ToolchainError` if icetime is not available or fails to execute.
pub fn run_timing(
    asc_path: &Path,
    device: &str,
    working_dir: &Path,
    registry: &ToolRegistry,
) -> Result<TimingResult, ToolchainError> {
    let asc_normalized = normalize_path_for_mingw(asc_path);

    let output =
        invoke_tool(registry, Tool::Icetime, &["-d", device, &asc_normalized], working_dir)?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let passed = output.status.success();

    let max_frequency_mhz = parse_frequency(&stdout);

    // Try to extract critical path delay from "Total path delay: X.XX ns"
    let critical_path_ns = stdout.lines().find_map(|line| {
        if line.contains("Total path delay:") {
            line.split(':').nth(1).and_then(|s| {
                let cleaned = s.trim().replace(" ns", "");
                cleaned.parse::<f64>().ok()
            })
        } else {
            None
        }
    });

    Ok(TimingResult { max_frequency_mhz, critical_path_ns, passed, stdout, stderr })
}

/// Parse a frequency in MHz from icetime output.
///
/// Scans each line for the substring "MHz" and attempts to extract
/// a floating-point number from the same line. Returns `None` if no
/// line containing "MHz" yields a parseable number.
pub fn parse_frequency(output: &str) -> Option<f64> {
    for line in output.lines() {
        if line.contains("MHz") {
            // Try each whitespace-delimited token for a valid f64
            for token in line.split_whitespace() {
                // Strip common trailing punctuation that might interfere
                let cleaned = token.trim_end_matches(|c: char| !c.is_ascii_digit() && c != '.');
                if let Ok(freq) = cleaned.parse::<f64>() {
                    if freq > 0.0 {
                        return Some(freq);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frequency() {
        let output = "\
// icetime topological timing analysis report
Total path delay: 12.50 ns (before clock uncertainty)
Maximum frequency: 80.00 MHz
";
        let freq = parse_frequency(output);
        assert!(freq.is_some());
        let mhz = freq.unwrap();
        assert!((mhz - 80.0).abs() < 0.01, "expected ~80.0 MHz, got {mhz}");
    }

    #[test]
    fn test_parse_frequency_no_match() {
        let output = "no timing information here\njust some random text\n";
        let freq = parse_frequency(output);
        assert!(freq.is_none());
    }

    #[test]
    fn test_timing_result_default() {
        let result = TimingResult {
            max_frequency_mhz: None,
            critical_path_ns: None,
            passed: false,
            stdout: String::new(),
            stderr: String::new(),
        };
        assert!(!result.passed);
        assert!(result.max_frequency_mhz.is_none());
        assert!(result.critical_path_ns.is_none());
        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());
    }
}
