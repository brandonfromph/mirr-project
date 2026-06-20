use crate::toolchain::ToolchainError;
use crate::toolchain::{invoke_tool, Tool, ToolRegistry};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct OpenLaneConfig {
    pub design_name: String,
    pub verilog_files: Vec<String>,
    pub clock_port: String,
    pub clock_period: f32,
    pub sdc_file: String,
    pub pdk: String,
    pub std_cell_library: String,
    pub pl_target_density: f32,
    pub run_synth: bool,
}

pub struct OpenLaneResult {
    pub success: bool,
    pub gds_path: Option<String>,
    pub routing_violations: u32,
    pub setup_slack_ns: f32,
    pub hold_slack_ns: f32,
    pub stderr: String,
}

/// Runs the OpenLANE physical design flow.
pub fn run_openlane_flow(
    registry: &ToolRegistry,
    working_dir: &Path,
    config: &OpenLaneConfig,
) -> Result<OpenLaneResult, ToolchainError> {
    // 1. Serialize config to config.json
    let config_json =
        serde_json::to_string_pretty(config).map_err(|e| ToolchainError::Invocation {
            tool: "openlane".to_string(),
            message: format!("Failed to serialize OpenLaneConfig: {}", e),
        })?;

    let config_path = working_dir.join("config.json");
    fs::write(&config_path, config_json).map_err(|e| ToolchainError::Invocation {
        tool: "openlane".to_string(),
        message: format!("Failed to write config.json: {}", e),
    })?;

    // 2. Invoke openlane --flow synth_to_gds
    let output = invoke_tool(
        registry,
        Tool::Openlane,
        &["--flow", "synth_to_gds", config_path.to_str().unwrap()],
        working_dir,
    )?;

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    let success = output.status.success();

    // 3. Parse metrics.csv (simplified for MVP, real parsing would read the CSV)
    let mut routing_violations = 0;
    let setup_slack_ns = 0.0;
    let hold_slack_ns = 0.0;

    // Check routing congestion manually from stdout
    if stdout.contains("Routing congestion too high") {
        routing_violations = 1;
    }

    let gds_path = if success {
        Some(
            working_dir
                .join(format!("runs/default/results/final/gds/{}.gds", config.design_name))
                .to_string_lossy()
                .to_string(),
        )
    } else {
        None
    };

    Ok(OpenLaneResult {
        success,
        gds_path,
        routing_violations,
        setup_slack_ns,
        hold_slack_ns,
        stderr,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_config() {
        let config = OpenLaneConfig {
            design_name: "reflex_soc".to_string(),
            verilog_files: vec!["reflex_soc_opt_synth.sv".to_string()],
            clock_port: "clk".to_string(),
            clock_period: 10.0,
            sdc_file: "constraints.sdc".to_string(),
            pdk: "sky130A".to_string(),
            std_cell_library: "sky130_fd_sc_hd".to_string(),
            pl_target_density: 0.65,
            run_synth: false,
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"DESIGN_NAME\":\"reflex_soc\""));
        assert!(json.contains("\"PDK\":\"sky130A\""));
        assert!(json.contains("\"RUN_SYNTH\":false"));
    }
}
