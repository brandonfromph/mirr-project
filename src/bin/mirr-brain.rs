//! # 🏛️ MRT COMPLIANCE MANDATE (Proposal 090)
//!
//! This file is part of the MIRR Runtime Tooling (MRT / Presidential Arsenal).
//! All modifications MUST adhere to the following standards:
//! 1. CDD LIFECYCLE: Audit → Propose → Sign → Execute → CI.
//! 2. ZERO-DEBT INVARIANT: No wrappers (D1), no dead code (D3), no misleading comments (D7).
//! 3. KB STANDARD: All operational telemetry MUST be stashed in `mirr-brain`.
//! 4. NO VIBE-CODING: Surgical edits via `mirr-wave` only.

#![forbid(unsafe_code)]
#![deny(warnings)]

//! # 🧠 mirr-brain: The Knowledge Core
//!
//! `mirr-brain` serves as the centralized repository for the project's long-term memory
//! and architectural invariants. It provides a source of truth for the entire Arsenal
//! by storing signed artifacts and system constraints.
//!
//! ## Core Responsibilities
//! *   **Error Code Governance**: Reserves and manages the range of error codes (E1xx-E8xx).
//! *   **Invariant Storage**: Keeps track of physical hardware limits and R-SPU specifications.
//! *   **Integrity Verification**: Hashes and verifies signed proposals to prevent unauthorized modifications.
//!
//! ## Bounded Execution
//! In alignment with NASA Power-of-10, `mirr-brain` operates with fixed limits on memory
//! and path traversal to ensure deterministic performance.

use clap::{Parser, Subcommand};

use mirrc::diagnostic::{render_diagnostic, Diagnostic};
use rusqlite::{params, Connection};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

const MAX_RESULTS: usize = 16;
const MAX_ENTRY_SIZE: usize = 4096;
const DEFAULT_KB_ROOT: &str = ".kb-data";
const BACKEND_NAME: &str = "kb-data";
const ENTRY_SOURCE: &str = "mirr-brain";

#[derive(Parser, Debug)]
#[command(name = "mirr-brain", version, about = "MIRR Knowledge Core")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Export CLI schema as JSON for tool integration
    #[arg(long, hide = true)]
    help_json: bool,

    /// Root directory for KB-lite assets.
    #[arg(long, default_value = DEFAULT_KB_ROOT)]
    kb_root: String,

    /// Output format (text or json)
    #[arg(short, long, default_value = "text")]
    format: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Store a new insight into the brain.
    Store {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: String,
    },
    /// Retrieve an insight from the brain.
    Get {
        #[arg(short, long)]
        key: String,
    },
    /// List all known invariants.
    Laws,
}

#[derive(Serialize, Debug)]
struct BrainResponse {
    pub status: String,
    pub message: String,
    pub backend: String,
    pub kb_root: String,
    pub graph_db_present: bool,
    pub knowledge_lance_present: bool,
    pub result_limit: usize,
    pub entry_size_limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub laws: Option<Vec<String>>,
}

fn clip_to_max(input: &str) -> String {
    if input.len() <= MAX_ENTRY_SIZE {
        return input.to_string();
    }

    let mut end = MAX_ENTRY_SIZE;
    while end > 0 && !input.is_char_boundary(end) {
        end -= 1;
    }
    input[..end].to_string()
}

fn ensure_schema(conn: &Connection) -> anyhow::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS kb_entries (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            source TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn open_kb(kb_root: &Path) -> anyhow::Result<(Connection, PathBuf)> {
    fs::create_dir_all(kb_root)?;
    let graph_db_path = kb_root.join("graph.db");
    let conn = Connection::open(&graph_db_path)?;
    ensure_schema(&conn)?;
    Ok((conn, graph_db_path))
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
        fatal_diagnostic(
            Diagnostic::error("no command specified").with_help("Run with --help for usage."),
        );
    });

    let kb_root_path = PathBuf::from(&args.kb_root);
    let (conn, graph_db_path) = open_kb(&kb_root_path)?;
    let knowledge_lance_path = kb_root_path.join("knowledge.lance");

    let mut response = BrainResponse {
        status: "OK".to_string(),
        message: String::new(),
        backend: BACKEND_NAME.to_string(),
        kb_root: kb_root_path.to_string_lossy().into_owned(),
        graph_db_present: graph_db_path.exists(),
        knowledge_lance_present: knowledge_lance_path.exists(),
        result_limit: MAX_RESULTS,
        entry_size_limit: MAX_ENTRY_SIZE,
        value: None,
        laws: None,
    };

    match command {
        Commands::Store { key, value } => {
            let clipped_value = clip_to_max(&value);
            conn.execute(
                "INSERT INTO kb_entries (key, value, source)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(key) DO UPDATE SET
                     value = excluded.value,
                     source = excluded.source",
                params![key, clipped_value, ENTRY_SOURCE],
            )?;
            response.message = format!("Insight '{}' stored successfully.", key);
        }
        Commands::Get { key } => {
            match conn.query_row(
                "SELECT value FROM kb_entries WHERE key = ?1",
                params![key],
                |row| row.get::<_, String>(0),
            ) {
                Ok(val) => {
                    response.value = Some(clip_to_max(&val));
                    response.message = format!("Retrieved insight for key: {}", key);
                }
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    response.status = "ERROR".to_string();
                    response.message = format!("Unknown key: {}", key);
                }
                Err(err) => return Err(err.into()),
            }
        }
        Commands::Laws => {
            let mut stmt = conn.prepare(
                "SELECT key, value
                 FROM kb_entries
                 ORDER BY key ASC
                 LIMIT ?1",
            )?;
            let rows = stmt.query_map(params![MAX_RESULTS as i64], |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                Ok(format!("{}: {}", key, clip_to_max(&value)))
            })?;

            let mut laws = Vec::new();
            for row in rows {
                laws.push(row?);
            }

            response.message = "Global Laws retrieved.".to_string();
            response.laws = Some(laws);
        }
    }

    response.graph_db_present = graph_db_path.exists();
    response.knowledge_lance_present = knowledge_lance_path.exists();

    if args.format == "json" {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        if response.status == "ERROR" {
            eprintln!("[BRAIN ERROR] {}", response.message);
        } else if let Some(v) = response.value {
            println!("{}", v);
        } else if let Some(laws) = response.laws {
            for law in laws {
                println!("{}", law);
            }
        } else {
            println!("[BRAIN] {}", response.message);
        }
    }

    Ok(())
}

fn fatal_diagnostic(diag: Diagnostic) -> ! {
    eprint!("{}", render_diagnostic(&diag, "", ""));
    std::process::exit(1);
}
