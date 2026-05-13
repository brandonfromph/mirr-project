use clap::{Parser, Subcommand, ValueEnum};
use glob::glob;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "mirr-audit", version, about = "MIRR Zero-Debt Compliance Engine", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<AuditCommand>,

    /// Output format for the audit results
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    /// Stash findings in the Brain under this key
    #[arg(long, env = "MIRR_AUDIT_STASH_KEY")]
    stash_key: Option<String>,

    /// Export CLI schema as JSON for tool integration
    #[arg(long, hide = true)]
    help_json: bool,
}

#[derive(Subcommand, Debug)]
enum AuditCommand {
    /// Perform a debt audit of the entire workspace
    Workspace {
        /// Glob pattern to scan (e.g., "src/**/*.rs")
        #[arg(short, long, default_value = "src/**/*.rs")]
        glob: String,
    },
    /// Audit a specific proposal file for compliance
    Proposal {
        /// Path to the proposal .md file
        path: PathBuf,
    },
    /// Audit the refinement gap between proposals and implementation
    Refinement {
        /// Glob pattern for source code to match against proposals
        #[arg(short, long, default_value = "src/**/*.rs")]
        glob: String,
    },
}

#[derive(ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum OutputFormat {
    /// Pretty-printed table (Human readable)
    Text,
    /// Structured data (Machine readable)
    Json,
    /// JSON output plus stashing in the Brain
    Stash,
}

#[derive(Serialize, Debug, Clone)]
struct AuditFinding {
    file: String,
    line: usize,
    rule: String,
    message: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.help_json {
        use clap::CommandFactory;
        fn get_cmd_manifest(cmd: &clap::Command) -> serde_json::Value {
            let mut args_list = Vec::new();
            for arg in cmd.get_arguments() {
                args_list.push(serde_json::json!({
                    "id": arg.get_id().as_str(),
                    "long": arg.get_long(),
                    "short": arg.get_short(),
                    "help": arg.get_help().map(|h| h.to_string()),
                    "required": arg.is_required_set(),
                }));
            }

            let mut subs = Vec::new();
            for sub in cmd.get_subcommands() {
                subs.push(get_cmd_manifest(sub));
            }

            serde_json::json!({
                "name": cmd.get_name(),
                "about": cmd.get_about().map(|a| a.to_string()),
                "version": cmd.get_version().map(|v| v.to_string()),
                "args": args_list,
                "subcommands": subs,
            })
        }

        let cmd = Args::command();
        println!("{}", serde_json::to_string_pretty(&get_cmd_manifest(&cmd)).unwrap());
        return Ok(());
    }

    let command = args.command.unwrap_or_else(|| {
        eprintln!("Error: no audit mode specified.\nRun with --help for usage.");
        std::process::exit(1);
    });

    let root = std::env::current_dir()?;

    // D2: No deprecated aliases
    let re_deprecated = Regex::new(r"#\[deprecated\]|#\[allow\(deprecated\)\]")?;
    // D3: No dead code
    let re_dead_code = Regex::new(r"#\[allow\(dead_code\)\]|#\[cfg\(never\)\]")?;
    // D5: No backward-compat shims
    let re_shim = Regex::new(&format!(
        r"\b(_unused|_{}|_compat|_{})\b|//.*(removed|deprecated|TODO: remove)",
        "old", "legacy"
    ))?;
    // D7: No misleading comments (heuristic)
    let re_stale =
        Regex::new(&format!(r"//.*\b(stale|{}|{}|previous version)\b", "legacy", "old"))?;
    // Security: Red Lines
    let re_red_line = Regex::new(r"std::net|std::fs|std::process|Command::new|TcpStream")?;

    let mut findings = Vec::new();

    match command {
        AuditCommand::Refinement { glob: glob_pattern } => {
            let mut implementation_structs = HashMap::new();
            let re_struct = Regex::new(r"(?:pub\s+)?struct\s+(\w+)")?;

            let patterns = if glob_pattern != "src/**/*.rs" {
                vec![glob_pattern.as_str()]
            } else {
                vec!["src/**/*.rs", "crates/**/*.rs", "benches/**/*.rs"]
            };

            for pattern in patterns {
                let full_pattern = root.join(pattern).to_string_lossy().to_string();
                for entry in glob(&full_pattern)? {
                    let path = entry?;
                    if !path.is_file() {
                        continue;
                    }
                    let content = std::fs::read_to_string(&path)?;
                    for cap in re_struct.captures_iter(&content) {
                        implementation_structs.insert(cap[1].to_string(), path.clone());
                    }
                }
            }

            let re_prop_struct = Regex::new(r"struct\s+(\w+)")?;
            let prop_pattern = root.join("proposals/**/*.md").to_string_lossy().to_string();
            for entry in glob(&prop_pattern)? {
                let path = entry?;
                let content = std::fs::read_to_string(&path)?;
                let mut in_code_block = false;
                for (line_num, line) in content.lines().enumerate() {
                    if line.trim().starts_with("```") {
                        in_code_block = !in_code_block;
                        continue;
                    }
                    if in_code_block {
                        if let Some(cap) = re_prop_struct.captures(line) {
                            let struct_name = &cap[1];
                            if !implementation_structs.contains_key(struct_name) {
                                findings.push(AuditFinding {
                                    file: path
                                        .strip_prefix(&root)
                                        .unwrap_or(&path)
                                        .to_string_lossy()
                                        .to_string(),
                                    line: line_num + 1,
                                    rule: "E801".to_string(),
                                    message: format!(
                                        "Refinement Gap: Struct '{}' proposed but not implemented.",
                                        struct_name
                                    ),
                                });
                            }
                        }
                    }
                }
            }
        }
        AuditCommand::Proposal { path } => {
            if !path.exists() {
                anyhow::bail!("Proposal file not found: {}", path.display());
            }
            let content = std::fs::read_to_string(&path)?;
            for (line_num, line) in content.lines().enumerate() {
                if line.starts_with("|") && line.contains("|") {
                    let parts: Vec<&str> = line.split('|').collect();
                    if parts.len() >= 5 {
                        let proposed = parts[4].trim();
                        if re_red_line.is_match(proposed) {
                            findings.push(AuditFinding {
                                file: path
                                    .strip_prefix(&root)
                                    .unwrap_or(&path)
                                    .to_string_lossy()
                                    .to_string(),
                                line: line_num + 1,
                                rule: "SEC-01".to_string(),
                                message:
                                    "Red Line violation: Unauthorized IO/Process usage in proposal."
                                        .to_string(),
                            });
                        }
                        if re_deprecated.is_match(proposed)
                            || re_dead_code.is_match(proposed)
                            || re_shim.is_match(proposed)
                        {
                            findings.push(AuditFinding {
                                file: path
                                    .strip_prefix(&root)
                                    .unwrap_or(&path)
                                    .to_string_lossy()
                                    .to_string(),
                                line: line_num + 1,
                                rule: "D1-D7".to_string(),
                                message: "Zero-Debt violation in proposed code.".to_string(),
                            });
                        }
                    }
                }
            }
        }
        AuditCommand::Workspace { glob: glob_pattern } => {
            let full_pattern = root.join(&glob_pattern).to_string_lossy().to_string();
            for entry in glob(&full_pattern)? {
                let path = entry?;
                if !path.is_file() {
                    continue;
                }
                let content = std::fs::read_to_string(&path)?;
                let rel_path =
                    path.strip_prefix(&root).unwrap_or(&path).to_string_lossy().to_string();

                for (line_num, line) in content.lines().enumerate() {
                    let l_num = line_num + 1;
                    if re_deprecated.is_match(line) {
                        findings.push(AuditFinding {
                            file: rel_path.clone(),
                            line: l_num,
                            rule: "D2".to_string(),
                            message: "Deprecated alias".to_string(),
                        });
                    }
                    if re_dead_code.is_match(line) {
                        findings.push(AuditFinding {
                            file: rel_path.clone(),
                            line: l_num,
                            rule: "D3".to_string(),
                            message: "Allowed dead code".to_string(),
                        });
                    }
                    if re_shim.is_match(line) {
                        findings.push(AuditFinding {
                            file: rel_path.clone(),
                            line: l_num,
                            rule: "D5".to_string(),
                            message: "Compat shim".to_string(),
                        });
                    }
                    if re_stale.is_match(line) {
                        findings.push(AuditFinding {
                            file: rel_path.clone(),
                            line: l_num,
                            rule: "D7".to_string(),
                            message: "Stale comment".to_string(),
                        });
                    }
                }
            }
        }
    }

    let json_output = serde_json::to_string_pretty(&findings)?;

    if let Some(key) = args.stash_key {
        let status = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "mirr-brain",
                "--",
                "store",
                "--key",
                &key,
                "--value",
                &json_output,
            ])
            .status()?;
        if !status.success() {
            anyhow::bail!("Failed to stash audit findings.");
        }
        println!("[AUDIT] Findings stashed in the Brain under key: {}", key);
    }

    match args.format {
        OutputFormat::Json | OutputFormat::Stash => println!("{}", json_output),
        OutputFormat::Text => {
            if findings.is_empty() {
                println!("Zero-Debt Invariant verified: No violations found.");
            } else {
                println!("| File | Line | Rule | Message |");
                println!("|------|------|------|---------|");
                for f in findings {
                    println!("| {} | {} | {} | {} |", f.file, f.line, f.rule, f.message);
                }
            }
        }
    }

    Ok(())
}
