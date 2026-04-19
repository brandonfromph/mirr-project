//! # 🏛️ MRT COMPLIANCE MANDATE (Proposal 090)
//!
//! This file is part of the MIRR Runtime Tooling (MRT / Presidential Arsenal).
//! All modifications MUST adhere to the following standards:
//! 1. CDD LIFECYCLE: Audit → Propose → Sign → Execute → CI.
//! 2. ZERO-DEBT INVARIANT: No wrappers (D1), no dead code (D3), no stale comments (D7).
//! 3. KB STANDARD: All operational telemetry MUST be stashed in `mirr-brain`.
//! 4. NO VIBE-CODING: Surgical edits via `mirr-wave` only.

//!    mirr-wave: The "Executive" of the Presidential Arsenal.
//!    Atomically applies "Wave" edits from signed MIRR proposals.

use clap::Parser;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "mirr-wave", version, about = "MIRR Wave Executor")]
struct Args {
    /// ID of the signed proposal (e.g., "075")
    #[arg(short = 'i', long)]
    proposal_id: String,

    /// Path to the proposal markdown file
    #[arg(short = 'f', long)]
    proposal_file: PathBuf,

    /// Dry run (do not apply changes)
    #[arg(long)]
    dry_run: bool,

    /// Maximum lines allowed per edit (default 50)
    #[arg(long, default_value_t = 50)]
    max_lines: usize,

    /// Stash execution log in the Brain
    #[arg(long)]
    stash: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SignatureEntry {
    signature: String,
    timestamp: String,
    #[serde(default)]
    hash: String, // SHA-256 hash of the target file at proposal time
}

#[derive(Serialize, Debug)]
struct WaveLog {
    proposal_id: String,
    timestamp: String,
    status: String,
    files_applied: Vec<String>,
    errors: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let root = std::env::current_dir()?;
    let mut log = WaveLog {
        proposal_id: args.proposal_id.clone(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        status: "STARTED".to_string(),
        files_applied: Vec::new(),
        errors: Vec::new(),
    };

    // 1. Verify signature in journal
    let journal_path = root.join(".presidential_journal.json");
    if !journal_path.exists() {
        let err = "Execution denied: No presidential journal found at .presidential_journal.json"
            .to_string();
        log.status = "FAILED".to_string();
        log.errors.push(err.clone());
        stash_log(&log, args.stash)?;
        anyhow::bail!(err);
    }
    let journal_content = std::fs::read_to_string(&journal_path)?;
    let journal: HashMap<String, SignatureEntry> = serde_json::from_str(&journal_content)?;

    let entry = journal.get(&args.proposal_id).ok_or_else(|| {
        let err = format!(
            "Execution denied: Proposal {} is not signed in the journal.",
            args.proposal_id
        );
        log.status = "FAILED".to_string();
        log.errors.push(err.clone());
        let _ = stash_log(&log, args.stash);
        anyhow::anyhow!(err)
    })?;

    // 2. Parse Proposal for edits
    let proposal_content = std::fs::read_to_string(&args.proposal_file)?;
    let mut lines = proposal_content.lines();
    let mut in_proposal_section = false;
    let mut edits = Vec::new();

    while let Some(line) = lines.next() {
        if line.starts_with("## Proposal") {
            in_proposal_section = true;
            lines.next(); // Skip table header
            lines.next(); // Skip separator
            continue;
        }
        if in_proposal_section && line.starts_with("## ") {
            in_proposal_section = false;
            continue;
        }

        if in_proposal_section && line.starts_with("|") {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 5 {
                let file_info = parts[2].trim();
                let current = parts[3].trim();
                let proposed = parts[4].trim();

                if !file_info.is_empty() && !current.is_empty() {
                    let path_part = file_info.split(':').next().unwrap();
                    edits.push((path_part.to_string(), current.to_string(), proposed.to_string()));
                }
            }
        }
    }

    if edits.is_empty() {
        let err = "No edits found in proposal table.".to_string();
        log.status = "FAILED".to_string();
        log.errors.push(err.clone());
        stash_log(&log, args.stash)?;
        anyhow::bail!(err);
    }

    // 3. Apply Edits (Transactional + Security Check)
    let mut backups = HashMap::new();
    let mut success = true;

    for (rel_path, old_text, new_text) in &edits {
        let abs_path = root.join(rel_path);
        if !abs_path.exists() {
            let err = format!("Error: File not found: {}", rel_path);
            log.errors.push(err);
            success = false;
            break;
        }

        let content = match std::fs::read_to_string(&abs_path) {
            Ok(c) => c,
            Err(e) => {
                let err = format!("Error reading {}: {}", rel_path, e);
                log.errors.push(err);
                success = false;
                break;
            }
        };

        // Snapshot Integrity (Hash Verification)
        if !entry.hash.is_empty() {
            let actual_hash = sha256_hash(content.as_bytes());
            if actual_hash != entry.hash {
                let err = format!(
                    "Snapshot Integrity FAILED for {}: File has changed since proposal.",
                    rel_path
                );
                log.errors.push(err);
                success = false;
                break;
            }
        }

        let old_text_clean = old_text.replace("`", "").replace("\\|", "|");
        let new_text_clean = proposed_to_literal(new_text);

        // Atomic Chunking (50-line rule)
        let added_lines = new_text_clean.lines().count();
        if added_lines > args.max_lines {
            let err = format!(
                "Atomic Chunking FAILED for {}: Proposed edit ({} lines) exceeds limit ({}).",
                rel_path, added_lines, args.max_lines
            );
            log.errors.push(err);
            success = false;
            break;
        }

        if !content.contains(&old_text_clean) {
            let err =
                format!("Atomic check FAILED for {}: 'Old' text not found exactly.", rel_path);
            log.errors.push(err);
            success = false;
            break;
        }

        if content.matches(&old_text_clean).count() != 1 {
            let err = format!(
                "Atomic check FAILED for {}: 'Old' text found multiple times (ambiguous).",
                rel_path
            );
            log.errors.push(err);
            success = false;
            break;
        }

        // Store backup for rollback
        backups.insert(abs_path.clone(), content.clone());

        if !args.dry_run {
            let updated = content.replacen(&old_text_clean, &new_text_clean, 1);
            if let Err(e) = std::fs::write(&abs_path, updated) {
                let err = format!("Error writing {}: {}", rel_path, e);
                log.errors.push(err);
                success = false;
                break;
            }
            log.files_applied.push(rel_path.clone());
        } else {
            log.files_applied.push(format!("[DRY-RUN] {}", rel_path));
        }
    }

    if !success {
        log.status = "FAILED".to_string();
        if !args.dry_run && !backups.is_empty() {
            for (path, original_content) in backups {
                let _ = std::fs::write(path, original_content);
            }
            log.status = "ROLLED_BACK".to_string();
        }
        stash_log(&log, args.stash)?;
        std::process::exit(1);
    }

    log.status = "SUCCESS".to_string();

    // Phase 1: Automatic Build Certification
    if let Err(e) = auto_certify_wave(&log) {
        eprintln!("[CERT] WARNING: Automated receipt generation failed: {}", e);
    }

    stash_log(&log, args.stash)?;
    Ok(())
}

fn auto_certify_wave(log: &WaveLog) -> anyhow::Result<()> {
    let receipt = format!("CERTIFIED_WAVE_{}_{}", log.proposal_id, log.timestamp);
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "mirr-brain",
            "--",
            "store",
            "--key",
            &format!("receipt_{}", log.proposal_id),
            "--value",
            &receipt,
        ])
        .status()?;
    Ok(())
}

fn proposed_to_literal(proposed: &str) -> String {
    proposed.replace("`", "").replace("\\|", "|").trim().to_string()
}

fn stash_log(log: &WaveLog, stash: bool) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(log)?;
    if stash {
        let key = format!("wave_log_{}", log.proposal_id);
        Command::new("cargo")
            .args(["run", "--bin", "mirr-brain", "--", "store", "--key", &key, "--value", &json])
            .status()?;
    }
    println!("{}", json);
    Ok(())
}

fn sha256_hash(data: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut digest_hex = String::with_capacity(result.len() * 2);
    for byte in result {
        digest_hex.push(HEX[(byte >> 4) as usize] as char);
        digest_hex.push(HEX[(byte & 0x0f) as usize] as char);
    }
    format!("sha256:{digest_hex}")
}
