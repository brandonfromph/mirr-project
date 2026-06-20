//! Logic optimization using Yosys and ABC.
//!
//! Provides a post-synthesis boolean minimization pass that
//! reads the emitted SystemVerilog, maps it through ABC to
//! minimize the AIG (And-Inverter Graph), and writes out
//! an optimized netlist.

#![forbid(unsafe_code)]

use std::path::Path;

use crate::toolchain::{invoke_tool, normalize_path_for_mingw, Tool, ToolRegistry, ToolchainError};

/// Result of an optimization run.
#[derive(Debug, Clone)]
pub struct OptimizeResult {
    /// Whether the optimization completed successfully.
    pub success: bool,
    /// Path to the optimized Verilog file.
    pub optimized_path: String,
    /// Raw stdout from Yosys.
    pub stdout: String,
    /// Raw stderr from Yosys.
    pub stderr: String,
}

/// Run post-synthesis logic optimization using Yosys and ABC.
///
/// # Arguments
///
/// * `registry` - Tool registry to locate Yosys.
/// * `sv_path` - Path to the unoptimized SystemVerilog file.
/// * `module_name` - Top-level module name to synthesize.
/// * `working_dir` - Directory to execute the tool in.
///
/// # Errors
///
/// Returns `ToolchainError` if Yosys is not available or fails.
pub fn run_logic_optimization(
    registry: &ToolRegistry,
    sv_path: &Path,
    module_name: &str,
    working_dir: &Path,
    link: &[String],
) -> Result<OptimizeResult, ToolchainError> {
    if !registry.is_available(Tool::Yosys) {
        return Err(ToolchainError::ToolNotFound { tool: "yosys".to_string() });
    }

    let sv_path_str = normalize_path_for_mingw(sv_path);

    // Determine output and script paths based on input path
    let base_name = sv_path.file_stem().unwrap_or_default().to_string_lossy();
    let parent = sv_path.parent().unwrap_or_else(|| Path::new("."));

    // Strip "_synth" if present to avoid "_synth_opt_synth"
    let clean_base =
        if let Some(stripped) = base_name.strip_suffix("_synth") { stripped } else { &base_name };

    let opt_sv_name = format!("{}_opt_synth.sv", clean_base);
    let opt_sv_path = parent.join(&opt_sv_name);
    let opt_sv_path_str = normalize_path_for_mingw(&opt_sv_path);

    let script_name = format!("{}_opt.ys", clean_base);
    let script_path = parent.join(&script_name);

    // Generate Yosys script
    let mut script = String::with_capacity(512);
    for l in link {
        script.push_str(&format!("read_verilog -sv {}\n", l));
    }
    script.push_str(&format!("read_verilog -sv {}\n", sv_path_str));
    script.push_str(&format!("hierarchy -top {}\n", module_name));
    script.push_str("proc; opt; memory; opt; fsm; opt;\n");
    // Standard ABC mapping for boolean minimization into basic logic gates
    script.push_str("abc -g AND,NAND,OR,NOR,XOR,XNOR,MUX\n");
    script.push_str("opt_clean\n");
    script.push_str(&format!("write_verilog -noattr {}\n", opt_sv_path_str));

    std::fs::write(&script_path, script).map_err(|e| ToolchainError::Invocation {
        tool: "yosys".to_string(),
        message: format!("failed to write opt script {}: {}", script_path.display(), e),
    })?;

    let script_path_str = normalize_path_for_mingw(&script_path);

    // Run Yosys
    let output = invoke_tool(registry, Tool::Yosys, &["-q", "-s", &script_path_str], working_dir)?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    Ok(OptimizeResult { success, optimized_path: opt_sv_path_str, stdout, stderr })
}
