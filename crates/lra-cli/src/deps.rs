#![forbid(unsafe_code)]

use std::path::Path;

use crate::hash;
use crate::registry;
use crate::util::bounded_read_to_string;

/// Maximum dependency tree depth (NASA Power-of-10 bound).
const MAX_DEP_DEPTH: usize = 16;

/// Maximum number of meta tag searches (NASA Power-of-10 bound).
const MAX_META_SEARCH: usize = 1_000_000;

pub fn run(path: &str, registry_path: &str) -> i32 {
    let html = bounded_read_to_string(Path::new(path));
    if html.is_empty() {
        eprintln!("Error: cannot read {}", path);
        return 1;
    }
    // Compute this paper's hash
    let my_hash = match hash::sha256_file(Path::new(path)) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 1;
        }
    };
    // Extract lra:depends meta tags
    let depends = extract_depends(&html);

    println!("LRA Dependency Graph \u{2014} {}\n", path);
    println!("  This paper: sha256:{}\n", &my_hash[..my_hash.len().min(16)]);

    if depends.is_empty() {
        println!("  No dependencies declared.");
        println!(
            "\n  Tip: Add <meta name=\"lra:depends\" content=\"sha256:...\"> to declare dependencies."
        );
        return 0;
    }

    // Load registry (optional — if available, resolve hashes to titles)
    let reg_json = bounded_read_to_string(Path::new(registry_path));
    let registry =
        if reg_json.is_empty() { None } else { registry::parse_registry(&reg_json).ok() };

    println!("  Dependencies ({}):", depends.len());
    let mut i = 0;
    let max = depends.len().min(MAX_DEP_DEPTH);
    while i < max {
        let dep_hash = &depends[i];
        let title = registry.as_ref().and_then(|reg| {
            let mut j = 0;
            let n = reg.entries.len();
            while j < n {
                if reg.entries[j].hash == *dep_hash {
                    return Some(reg.entries[j].title.clone());
                }
                j += 1;
            }
            None
        });
        let short_hash = &dep_hash[..dep_hash.len().min(23)];
        match title {
            Some(t) => println!("    +-- {} ({})", t, short_hash),
            None => println!("    +-- {} (unknown \u{2014} not in registry)", short_hash),
        }
        i += 1;
    }
    println!();
    0
}

/// Extract SHA-256 hashes from <meta name="lra:depends" content="sha256:..."> tags.
/// Bounded: MAX_DEPENDS = 100.
fn extract_depends(html: &str) -> Vec<String> {
    const MAX_DEPENDS: usize = 100;
    let pattern = "lra:depends";
    let mut results = Vec::new();
    let mut search_from = 0;
    let mut iter_count = 0;
    while iter_count < MAX_META_SEARCH && results.len() < MAX_DEPENDS {
        iter_count += 1;
        let pos = match html[search_from..].find(pattern) {
            Some(p) => search_from + p,
            None => break,
        };
        // Find the content="..." attribute after this position
        let after = &html[pos..];
        if let Some(content_start) = after.find("content=\"") {
            let value_start = pos + content_start + 9; // len of `content="`
            if value_start < html.len() {
                if let Some(value_end) = html[value_start..].find('"') {
                    let value = &html[value_start..value_start + value_end];
                    if let Some(hash) = value.strip_prefix("sha256:") {
                        results.push(format!("sha256:{}", hash));
                    }
                }
            }
        }
        search_from = pos + pattern.len();
    }
    results
}
