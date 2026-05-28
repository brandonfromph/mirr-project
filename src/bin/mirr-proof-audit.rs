use anyhow::Result;
use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about = "Audits MIRR compiler proofs coverage", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "src/ast")]
    ast_dir: PathBuf,

    #[arg(short, long, default_value = "src/emit")]
    emit_dir: PathBuf,

    #[arg(short, long, default_value = "src/mape_k")]
    mape_k_dir: PathBuf,

    #[arg(short, long, default_value = "src/cert")]
    cert_dir: PathBuf,

    #[arg(short, long, default_value = "proofs")]
    proofs_dir: PathBuf,

    #[arg(short, long, default_value = "json")]
    format: String,
}

struct Symbol {
    name: String,
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let rust_symbols =
        collect_rust_symbols(&args.ast_dir, &args.emit_dir, &args.mape_k_dir, &args.cert_dir)?;
    let proof_symbols = collect_proof_symbols(&args.proofs_dir)?;

    let mut coverage = Vec::new();
    let mut covered_count = 0;

    for symbol in &rust_symbols {
        let normalized_rust = normalize_name(&symbol.name);
        let mut found = false;
        for proof_sym in &proof_symbols {
            if normalize_name(proof_sym) == normalized_rust {
                found = true;
                break;
            }
        }

        if found {
            covered_count += 1;
        }

        coverage.push((symbol.name.clone(), symbol.file.display().to_string(), found));
    }

    if args.format == "json" {
        let out = serde_json::json!({
            "total_symbols": rust_symbols.len(),
            "covered_symbols": covered_count,
            "coverage_percent": if rust_symbols.is_empty() { 0.0 } else { (covered_count as f64 / rust_symbols.len() as f64) * 100.0 },
            "details": coverage.iter().map(|(name, file, covered)| {
                serde_json::json!({
                    "name": name,
                    "file": file,
                    "covered": covered
                })
            }).collect::<Vec<_>>()
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("MIRR Proof Audit Report");
        println!("=======================");
        println!("Total Rust Symbols:   {}", rust_symbols.len());
        println!("Covered Symbols:       {}", covered_count);
        println!(
            "Coverage Percentage:   {:.2}%",
            if rust_symbols.is_empty() {
                0.0
            } else {
                (covered_count as f64 / rust_symbols.len() as f64) * 100.0
            }
        );
        println!("\nMissing Proofs:");
        for (name, file, covered) in &coverage {
            if !covered {
                println!("  - {} ({})", name, file);
            }
        }
    }

    Ok(())
}

fn normalize_name(name: &str) -> String {
    let mut normalized = name.to_lowercase().replace("_", "");
    for suffix in &["ok", "valid", "proof"] {
        if normalized.ends_with(suffix) && normalized.len() > suffix.len() {
            normalized = normalized[..normalized.len() - suffix.len()].to_string();
        }
    }
    normalized
}

fn collect_rust_symbols(
    ast_dir: &Path,
    emit_dir: &Path,
    mape_k_dir: &Path,
    cert_dir: &Path,
) -> Result<Vec<Symbol>> {
    let mut symbols = Vec::new();
    let re = Regex::new(r"^\s*(?:pub\s+)?(?:struct|enum)\s+([A-Z][a-zA-Z0-9_]*)").unwrap();

    let mut scanned = HashSet::new();

    for dir in &[ast_dir, emit_dir, mape_k_dir, cert_dir] {
        if !dir.exists() {
            continue;
        }
        let canonical = match dir.canonicalize() {
            Ok(p) => p,
            Err(_) => dir.to_path_buf(),
        };
        if !scanned.insert(canonical) {
            continue;
        }
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file()
                && entry.path().extension().is_some_and(|ext| ext == "rs")
            {
                let content = std::fs::read_to_string(entry.path())?;
                for line in content.lines() {
                    if let Some(caps) = re.captures(line) {
                        symbols.push(Symbol {
                            name: caps[1].to_string(),
                            file: entry.path().to_path_buf(),
                        });
                    }
                }
            }
        }
    }
    Ok(symbols)
}

fn collect_proof_symbols(proofs_dir: &Path) -> Result<HashSet<String>> {
    let mut symbols = HashSet::new();
    let re = Regex::new(
        r"^\s*(?:Theorem|Lemma|Definition|Inductive|Fixpoint|Record|with)\s+([a-zA-Z0-9_]*)",
    )
    .unwrap();

    if !proofs_dir.exists() {
        return Ok(symbols);
    }
    for entry in WalkDir::new(proofs_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|ext| ext == "v") {
            let content = std::fs::read_to_string(entry.path())?;
            for line in content.lines() {
                if let Some(caps) = re.captures(line) {
                    symbols.insert(caps[1].to_string());
                }
            }
        }
    }
    Ok(symbols)
}
