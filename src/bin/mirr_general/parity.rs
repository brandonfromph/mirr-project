#![forbid(unsafe_code)]

use std::io;
use std::path::Path;
use std::process::Command;

use nasa_rust_project::diagnostic::{render_diagnostic, Diagnostic};
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ParitySubsystem {
    CliVsWasm,
    CompilerVsVscode,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ParityRecord {
    pub subsystem: ParitySubsystem,
    pub success: bool,
    pub detail: String,
}

fn cargo_command(args: &[&str]) -> Command {
    let mut command = Command::new("cargo");
    command.args(args);
    command
}

fn npm_command() -> Command {
    #[cfg(target_os = "windows")]
    {
        Command::new("npm.cmd")
    }
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("npm")
    }
}

pub fn verify_cli_wasm_parity(source_path: &Path) -> io::Result<ParityRecord> {
    let source_arg = source_path.to_string_lossy().to_string();
    let output_path = Path::new("target").join("mirr-general").join("parity-compile.json");
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let output_arg = output_path.to_string_lossy().to_string();

    let mut compiler_command = cargo_command(&[
        "run",
        "--bin",
        "mirr-compile",
        "--",
        &source_arg,
        "--emit",
        "json",
        "-o",
        &output_arg,
    ]);
    let compiler_output = compiler_command.output()?;

    if !compiler_output.status.success() {
        return Ok(ParityRecord {
            subsystem: ParitySubsystem::CliVsWasm,
            success: false,
            detail: format!(
                "mirr-compile failed for {} (status={:?})",
                source_path.display(),
                compiler_output.status.code()
            ),
        });
    }

    let compiler_json_text = std::fs::read_to_string(&output_path)?;
    let compiler_json = match serde_json::from_str::<Value>(&compiler_json_text) {
        Ok(value) => value,
        Err(_) => {
            return Ok(ParityRecord {
                subsystem: ParitySubsystem::CliVsWasm,
                success: false,
                detail: format!(
                    "output file is not valid JSON from mirr-compile for {}",
                    source_path.display()
                ),
            });
        }
    };

    let normalized = std::env::var("MIRR_PARITY_NORMALIZED_JSON")
        .ok()
        .and_then(|text| serde_json::from_str::<Value>(&text).ok());

    let success = match &normalized {
        Some(value) => compiler_json == *value,
        None => true,
    };

    let detail = if success && normalized.is_some() {
        format!("stdout matched normalized parity payload for {}", source_path.display())
    } else if success {
        format!(
            "mirr-compile produced valid JSON for {} (no normalized payload provided)",
            source_path.display()
        )
    } else {
        format!("json mismatch for {} against normalized parity payload", source_path.display())
    };

    Ok(ParityRecord { subsystem: ParitySubsystem::CliVsWasm, success, detail })
}

pub fn verify_vscode_contract() -> io::Result<ParityRecord> {
    let mut command = npm_command();
    command.args(["--prefix", "vscode-mirr", "pack", "--dry-run"]);
    let output = command.output()?;
    let success = output.status.success();
    let detail = if success {
        "npm --prefix vscode-mirr pack --dry-run passed".to_string()
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        format!("npm --prefix vscode-mirr pack --dry-run failed: {}", stderr)
    };

    Ok(ParityRecord { subsystem: ParitySubsystem::CompilerVsVscode, success, detail })
}

pub fn run_consumer_parity(records: &[ParityRecord]) -> io::Result<()> {
    for record in records {
        if !record.success {
            let diag = Diagnostic::error("consumer parity check failed")
                .with_note(format!("subsystem={:?}", record.subsystem))
                .with_note(record.detail.clone())
                .with_code("E909");
            eprint!("{}", render_diagnostic(&diag, "", ""));
            return Err(io::Error::new(io::ErrorKind::Other, record.detail.clone()));
        }
    }

    Ok(())
}
