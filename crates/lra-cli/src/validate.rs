#![forbid(unsafe_code)]

use std::path::Path;

use crate::util::bounded_read_to_string;

/// Compliance tier result.
#[derive(Debug, PartialEq, Eq)]
pub enum Tier {
    None,
    Bronze,
    Silver,
    Gold,
}

impl std::fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tier::None => write!(f, "NONE"),
            Tier::Bronze => write!(f, "BRONZE"),
            Tier::Silver => write!(f, "SILVER"),
            Tier::Gold => write!(f, "GOLD"),
        }
    }
}

struct Check {
    id: &'static str,
    label: &'static str,
    pass: bool,
}

/// Run all 23 LRA-1.0 compliance checks against the given HTML and directory.
fn run_checks(html: &str, dir: &Path) -> Vec<Check> {
    let license_path = dir.join("LICENSE");
    let license_text = bounded_read_to_string(&license_path);
    let paperjs_path = dir.join("paper.js");
    let citation_path = dir.join("CITATION.cff");
    let citation_text = bounded_read_to_string(&citation_path);

    // WASM glob: bounded directory walk (NASA Power-of-10)
    let has_wasm = has_wasm_file(dir);

    // H2+H3: Check for external resource loading in HTML and paper.js
    let paperjs_text = bounded_read_to_string(&paperjs_path);
    let no_external = !has_external_resources(html) && !has_external_resources(&paperjs_text);

    vec![
        // Bronze (16)
        Check { id: "B1", label: "html lang attribute", pass: html.contains("<html lang=") },
        Check {
            id: "B2",
            label: "charset meta",
            pass: html.contains("<meta") && html.contains("charset"),
        },
        Check {
            id: "B3",
            label: "viewport meta",
            pass: html.contains("<meta") && html.contains("viewport"),
        },
        Check {
            id: "B4",
            label: "description meta",
            pass: html.contains("<meta") && html.contains("description"),
        },
        Check { id: "B5", label: "title element", pass: html.contains("<title>") },
        Check { id: "B6", label: "abstract section", pass: html.contains("id=\"abstract\"") },
        Check { id: "B7", label: "claims section", pass: html.contains("id=\"claims\"") },
        Check { id: "B8", label: "references section", pass: html.contains("id=\"references\"") },
        Check { id: "B9", label: "citation section", pass: html.contains("id=\"citation\"") },
        Check {
            id: "B10",
            label: "data-lra-claim attribute",
            pass: html.contains("data-lra-claim"),
        },
        Check { id: "B11", label: "LICENSE file exists", pass: license_path.exists() },
        Check {
            id: "B12",
            label: "LICENSE contains Apache",
            pass: license_text.contains("GNU General Public License"),
        },
        Check { id: "B13", label: "CITATION.cff exists", pass: citation_path.exists() },
        Check { id: "B14", label: "lra:version meta", pass: html.contains("lra:version") },
        Check {
            id: "B15",
            label: "CITATION.cff has GPL-3.0-or-later license",
            pass: citation_text.contains("license:") && citation_text.contains("GPL-3.0-or-later"),
        },
        Check { id: "B16", label: "paper.css exists", pass: dir.join("paper.css").exists() },
        // Silver (3)
        Check { id: "S1", label: "demo section", pass: html.contains("class=\"demo\"") },
        Check { id: "S2", label: "noscript fallback", pass: html.contains("<noscript>") },
        Check { id: "S3", label: "paper.js exists", pass: paperjs_path.exists() },
        // Gold (4)
        Check { id: "G1", label: "WASM module present", pass: has_wasm },
        Check { id: "G2", label: "evidence link", pass: html.contains("data-lra-evidence") },
        Check { id: "G3", label: "no external resources", pass: no_external },
        Check { id: "G4", label: "aria-live region", pass: html.contains("aria-live") },
    ]
}

/// Check if text contains external resource loading patterns (H2+H3).
fn has_external_resources(text: &str) -> bool {
    const PATTERNS: [&str; 6] = [
        "fetch(\"http",
        "fetch('http",
        "<script src=\"http",
        "<link href=\"http",
        "@import url(\"http",
        "@import url('http",
    ];
    const MAX_PATTERNS: usize = 6;
    let mut i = 0;
    while i < MAX_PATTERNS {
        if text.contains(PATTERNS[i]) {
            return true;
        }
        i += 1;
    }
    false
}

/// Bounded directory walk to find any .wasm file.
/// MAX_DIR_DEPTH = 8, MAX_DIR_ENTRIES = 10_000 (NASA Power-of-10).
fn has_wasm_file(dir: &Path) -> bool {
    const MAX_DIR_DEPTH: usize = 8;
    const MAX_DIR_ENTRIES: usize = 10_000;

    let mut stack: Vec<(std::path::PathBuf, usize)> = vec![(dir.to_path_buf(), 0)];
    let mut entries_visited: usize = 0;

    while let Some((current, depth)) = stack.pop() {
        if depth > MAX_DIR_DEPTH || entries_visited > MAX_DIR_ENTRIES {
            break;
        }
        let read_dir = match std::fs::read_dir(&current) {
            Ok(rd) => rd,
            Err(_) => continue,
        };
        for entry in read_dir {
            entries_visited += 1;
            if entries_visited > MAX_DIR_ENTRIES {
                break;
            }
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "wasm" {
                        return true;
                    }
                }
            } else if path.is_dir() && depth < MAX_DIR_DEPTH {
                stack.push((path, depth + 1));
            }
        }
    }
    false
}

/// Determine compliance tier from check results.
fn determine_tier(checks: &[Check]) -> Tier {
    let bronze_pass = checks.iter().filter(|c| c.id.starts_with('B')).all(|c| c.pass);
    let silver_pass = checks.iter().filter(|c| c.id.starts_with('S')).all(|c| c.pass);
    let gold_pass = checks.iter().filter(|c| c.id.starts_with('G')).all(|c| c.pass);

    if bronze_pass && silver_pass && gold_pass {
        Tier::Gold
    } else if bronze_pass && silver_pass {
        Tier::Silver
    } else if bronze_pass {
        Tier::Bronze
    } else {
        Tier::None
    }
}

/// Public entry point. Returns exit code.
pub fn run(path: &str) -> i32 {
    let html_path = Path::new(path);
    let html = bounded_read_to_string(html_path);
    if html.is_empty() {
        eprintln!("Error: cannot read {}", path);
        return 1;
    }

    let dir = html_path.parent().unwrap_or(Path::new("."));
    let checks = run_checks(&html, dir);
    let tier = determine_tier(&checks);

    let total_pass = checks.iter().filter(|c| c.pass).count();
    let total = checks.len();

    println!("LRA Validate — {}\n", path);

    let sections = [("Bronze", "B"), ("Silver", "S"), ("Gold", "G")];
    for (name, prefix) in &sections {
        let tier_checks: Vec<&Check> = checks.iter().filter(|c| c.id.starts_with(prefix)).collect();
        let pass_count = tier_checks.iter().filter(|c| c.pass).count();
        println!("{} ({}/{}):", name, pass_count, tier_checks.len());
        for c in tier_checks {
            let status = if c.pass { "PASS" } else { "FAIL" };
            println!("  [{}] {}  {}", status, c.id, c.label);
        }
        println!();
    }

    println!("Result: {} ({}/{} checks passed)", tier, total_pass, total);

    if tier == Tier::None {
        1
    } else {
        0
    }
}

/// Returns the tier as a string for badge.rs.
pub fn tier_for(path: &str) -> Tier {
    let html_path = Path::new(path);
    let html = bounded_read_to_string(html_path);
    if html.is_empty() {
        return Tier::None;
    }
    let dir = html_path.parent().unwrap_or(Path::new("."));
    let checks = run_checks(&html, dir);
    determine_tier(&checks)
}
