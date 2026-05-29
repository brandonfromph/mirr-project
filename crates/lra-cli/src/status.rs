#![forbid(unsafe_code)]

use crate::registry;
use crate::util;
use sha2::{Digest, Sha256};

/// Maximum HTML response size (10 MB, NASA Power-of-10 bound).
const MAX_HTML_SIZE: usize = 10 * 1024 * 1024;

/// Maximum HTTP timeout in seconds.
const MAX_TIMEOUT_SECS: u64 = 10;

/// Maximum registry entries to check (NASA Power-of-10 bound).
const MAX_STATUS_ENTRIES: usize = 100;

/// Maximum structural checks per entry.
const MAX_CHECKS: usize = 10;

fn to_hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

pub fn run(registry_path: &str) -> i32 {
    let path = std::path::Path::new(registry_path);
    let json = util::bounded_read_to_string(path);
    if json.is_empty() {
        eprintln!("Error: Cannot read registry: {}", registry_path);
        return 1;
    }

    let reg = match registry::parse_registry(&json) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 1;
        }
    };

    let count = reg.entries.len().min(MAX_STATUS_ENTRIES);
    println!("LRA Network Status \u{2014} {} paper(s)\n", count);

    let agent = ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(MAX_TIMEOUT_SECS))
        .timeout_read(std::time::Duration::from_secs(MAX_TIMEOUT_SECS))
        .build();

    let mut live_count = 0;
    let mut stale_count = 0;
    let mut unreachable_count = 0;
    let mut i = 0;

    while i < count {
        let entry = &reg.entries[i];
        println!("  {}", entry.title);
        println!("    URL:     {}", entry.url);

        // Single fetch per paper — reuse body for both marker checks and SHA-256
        let body = match agent.get(&entry.url).call() {
            Ok(response) => {
                if response.status() != 200 {
                    println!("    Status:  [UNREACHABLE] (HTTP {})", response.status());
                    unreachable_count += 1;
                    println!();
                    i += 1;
                    continue;
                }
                match response.into_string() {
                    Ok(b) => {
                        if b.len() > MAX_HTML_SIZE {
                            println!("    Status:  [UNREACHABLE] (response too large)");
                            unreachable_count += 1;
                            println!();
                            i += 1;
                            continue;
                        }
                        b
                    }
                    Err(e) => {
                        println!("    Status:  [UNREACHABLE] ({})", e);
                        unreachable_count += 1;
                        println!();
                        i += 1;
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("    Status:  [UNREACHABLE] ({})", e);
                unreachable_count += 1;
                println!();
                i += 1;
                continue;
            }
        };

        // Structural marker checks (reuse fetched body)
        let checks: [(&str, bool); 4] = [
            ("lra:version", body.contains("lra:version")),
            ("sw.js", body.contains("sw.js") || body.contains("serviceWorker")),
            ("data-lra-claim", body.contains("data-lra-claim")),
            ("lra:capability", body.contains("lra:capability")),
        ];

        let mut marker_pass = 0;
        let mut ci = 0;
        while ci < checks.len() && ci < MAX_CHECKS {
            if checks[ci].1 {
                marker_pass += 1;
            }
            ci += 1;
        }

        // SHA-256 integrity check (reuse same fetched body)
        let mut hasher = Sha256::new();
        hasher.update(body.as_bytes());
        let hash_bytes = hasher.finalize();
        let hash_hex = to_hex_lower(hash_bytes.as_ref());
        let live_hash = format!("sha256:{hash_hex}");

        let hash_match = live_hash == entry.hash;

        if marker_pass == checks.len() && hash_match {
            println!(
                "    Status:  [LIVE] (HTTP 200, {}/{} markers, hash match)",
                marker_pass,
                checks.len()
            );
            live_count += 1;
        } else if marker_pass == checks.len() {
            println!(
                "    Status:  [STALE] ({}/{} markers, hash mismatch)",
                marker_pass,
                checks.len()
            );
            stale_count += 1;
        } else {
            println!("    Status:  [STALE] ({}/{} markers)", marker_pass, checks.len());
            stale_count += 1;
        }

        println!("    Hash:    {}", live_hash);
        match util::parse_semver(&entry.version) {
            Some(sv) => println!("    Version: {}.{}.{}", sv.major, sv.minor, sv.patch),
            None => println!("    Version: {} (invalid semver)", entry.version),
        }

        if let Some(ref v) = entry.verifications {
            println!(
                "    Verifs:  {} total, {} challenges",
                v.total_verifications, v.total_challenges
            );
        }

        if entry.depends.is_empty() {
            println!("    Depends: none");
        } else {
            println!("    Depends: {} paper(s)", entry.depends.len());
        }

        println!();
        i += 1;
    }

    println!(
        "Summary: {} live, {} stale, {} unreachable",
        live_count, stale_count, unreachable_count
    );
    0
}
