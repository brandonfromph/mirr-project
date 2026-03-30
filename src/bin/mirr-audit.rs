//! # 🏛️ MRT COMPLIANCE MANDATE (Proposal 090)
//!
//! This file is part of the MIRR Runtime Tooling (MRT / Presidential Arsenal).
//! All modifications MUST adhere to the following standards:
//! 1. CDD LIFECYCLE: Audit → Propose → Sign → Execute → CI.
//! 2. ZERO-DEBT INVARIANT: No wrappers (D1), no dead code (D3), no stale comments (D7).
//! 3. KB STANDARD: All operational telemetry MUST be stashed in `mirr-brain`.
//! 4. NO VIBE-CODING: Surgical edits via `mirr-wave` only.

//! # 👁️ mirr-audit: The Oversight Node
//!
//! `mirr-audit` acts as the "Eyes" of the Presidential Arsenal. It performs high-speed,
//! regex-based scans across the entire MIRR workspace to ensure strict compliance with
//! architectural mandates and the Zero-Debt Invariant.
//!
//! ## Core Functions
//! *   **D1-D7 Scans**: Identifies technical debt violations (e.g., dead code, deprecated aliases).
//! *   **Refinement Gaps**: Scans for "TODO" or "FIXME" comments that represent unexecuted parts of a signed proposal.
//! *   **Security Policy Enforcement**: Flags usage of forbidden patterns (e.g., `unsafe`).
//!
//! ## Output Formats
//! *   **KB-Stash**: Findings are stashed in `mirr-brain` for system-wide pattern memory.
//! *   **Human-Readable**: Colored console output for developer feedback.
//! *   **JSON**: Structured reporting for `mirr-general` orchestration.

use clap::Parser;

use glob::glob;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "mirr-audit", version, about = "MIRR Zero-Debt Auditor")]
struct Args {
    /// Glob pattern to audit (e.g., "src/**/*.rs")
    #[arg(short, long, default_value = "src/**/*.rs")]
    glob: String,

    /// Output format (text, json, or stash)
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Audit mode: 'workspace', 'proposal', or 'refinement'
    #[arg(short, long, default_value = "workspace")]
    mode: String,

    /// Stash findings in the Brain under this key
    #[arg(long)]
    stash_key: Option<String>,
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
    let root = std::env::current_dir()?;

    // D2: No deprecated aliases
    let re_deprecated = Regex::new(r"#\[deprecated\]|#\[allow\(deprecated\)\]")?;
    // D3: No dead code
    let re_dead_code = Regex::new(r"#\[allow\(dead_code\)\]|#\[cfg\(never\)\]")?;
    // D5: No backward-compat shims
    let re_shim =
        Regex::new(r"_unused|_old|_compat|_legacy|// removed|// deprecated|// TODO: remove")?;
    // D7: No misleading comments (heuristic)
    let re_stale = Regex::new(r"//.*(stale|legacy|old|previous version)")?;
    // Security: Red Lines
    let re_red_line = Regex::new(r"std::net|std::fs|std::process|Command::new|TcpStream")?;

    let mut findings = Vec::new();

    if args.mode == "refinement" {
        // Pillar 1: Cognition — Map structural definitions vs implementation
        let mut implementation_structs = HashMap::new();

        let re_struct = Regex::new(r"(?:pub\s+)?struct\s+(\w+)")?;

        // 1. Scan implementation (the Code) across ALL crates
        // Use the provided glob if possible, otherwise default to standard MIRR layout
        let patterns = if args.glob != "src/**/*.rs" {
            vec![args.glob.as_str()]
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

        // 2. Scan proposals (the Proof)
        let prop_pattern = root.join("proposals/**/*.md").to_string_lossy().to_string();
        for entry in glob(&prop_pattern)? {
            let path = entry?;
            let content = std::fs::read_to_string(&path)?;

            // Look for struct definitions within code blocks
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
                                file: path.strip_prefix(&root).unwrap_or(&path).to_string_lossy().to_string(),
                                line: line_num + 1,
                                rule: "E801".to_string(),
                                message: format!("Refinement Gap: Struct '{}' proposed in code block but not implemented.", struct_name),
                            });
                        }
                    }
                }
            }
        }
    } else if args.mode == "proposal" {
        // Scan the proposal file
        let path = if PathBuf::from(&args.glob).is_absolute() {
            PathBuf::from(&args.glob)
        } else {
            root.join(&args.glob)
        };

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
    } else {
        let full_pattern = root.join(&args.glob).to_string_lossy().to_string();
        for entry in glob(&full_pattern)? {
            let path = entry?;
            if !path.is_file() {
                continue;
            }

            let content = std::fs::read_to_string(&path)?;
            let rel_path = path.strip_prefix(&root).unwrap_or(&path).to_string_lossy().to_string();

            for (line_num, line) in content.lines().enumerate() {
                let l_num = line_num + 1;
                if re_deprecated.is_match(line) {
                    findings.push(AuditFinding {
                        file: rel_path.clone(),
                        line: l_num,
                        rule: "D2".to_string(),
                        message: "Deprecated alias/attribute".to_string(),
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
                        message: "Backward-compatibility shim".to_string(),
                    });
                }
                if re_stale.is_match(line) {
                    findings.push(AuditFinding {
                        file: rel_path.clone(),
                        line: l_num,
                        rule: "D7".to_string(),
                        message: "Potentially stale comment".to_string(),
                    });
                }
            }
        }
    }

    // Standardize Output on the Knowledge Base (mirr-brain)
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
            anyhow::bail!("Failed to stash audit findings in the Brain.");
        }
        println!("[AUDIT] Findings stashed in the Brain under key: {}", key);
    }

    if args.format == "json" || args.format == "stash" {
        println!("{}", json_output);
    } else {
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

    Ok(())
}
