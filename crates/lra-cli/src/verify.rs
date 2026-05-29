#![forbid(unsafe_code)]

use crate::registry;
use crate::util;
use sha2::{Digest, Sha256};

/// Maximum HTML response size (10 MB, NASA Power-of-10 bound).
const MAX_HTML_SIZE: usize = 10 * 1024 * 1024;

/// Maximum number of claims to extract (NASA Power-of-10 bound).
const MAX_CLAIMS_EXTRACT: usize = 100;

/// Maximum HTTP timeout in seconds.
const MAX_TIMEOUT_SECS: u64 = 10;

/// Maximum number of verification checks.
const MAX_CHECKS: usize = 20;

fn to_hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

pub fn run(target: &str, registry_path: &str, receipt_path: Option<&str>) -> i32 {
    println!("LRA Verify — {}\n", target);

    // Step 1: Try to resolve target from registry (by URL or hash)
    let registry_result = resolve_from_registry(target, registry_path);

    // Step 2: Determine the URL to fetch
    let url = if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else if let Some(ref entry) = registry_result {
        entry.url.clone()
    } else {
        eprintln!("  [FAIL] Target is not a URL and not found in registry: {}", target);
        return 1;
    };

    // Step 3: Fetch the paper HTML
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(MAX_TIMEOUT_SECS))
        .timeout_read(std::time::Duration::from_secs(MAX_TIMEOUT_SECS))
        .build();

    let response = match agent.get(&url).call() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("  [FAIL] Cannot reach: {}", e);
            return 1;
        }
    };

    let status = response.status();
    if status != 200 {
        eprintln!("  [FAIL] HTTP {}", status);
        return 1;
    }

    let body = match response.into_string() {
        Ok(b) => {
            if b.len() > MAX_HTML_SIZE {
                eprintln!("  [FAIL] Response too large ({} bytes)", b.len());
                return 1;
            }
            b
        }
        Err(e) => {
            eprintln!("  [FAIL] Cannot read response: {}", e);
            return 1;
        }
    };

    println!("  Fetched {} bytes from {}\n", body.len(), url);

    // Step 4: Content integrity check (SHA-256)
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    let hash_bytes = hasher.finalize();
    let hash_hex = to_hex_lower(hash_bytes.as_ref());
    let live_hash = format!("sha256:{hash_hex}");

    let mut pass_count = 0;
    let mut check_count = 0;

    if let Some(ref entry) = registry_result {
        check_count += 1;
        if live_hash == entry.hash {
            println!("  [PASS] Content integrity — hash matches registry");
            pass_count += 1;
        } else {
            println!("  [FAIL] Content integrity — hash mismatch");
            println!("         Live:     {}", live_hash);
            println!("         Registry: {}", entry.hash);
        }
    } else {
        println!("  [INFO] Content hash: {}", live_hash);
        println!("         (not in registry — cannot verify integrity)");
    }

    // Step 5: Extract and list claims from HTML markup
    let claims = extract_claims(&body);
    println!("\n  Claims found: {}", claims.len());
    let mut ci = 0;
    while ci < claims.len() && ci < MAX_CLAIMS_EXTRACT {
        let (ref id, ref text, has_evidence) = claims[ci];
        let evidence_tag = if has_evidence { "executable" } else { "assertion" };
        println!("    {} [{}]: {}", id, evidence_tag, truncate(text, 80));
        ci += 1;
    }

    // Step 6: Structural completeness checks
    println!();
    let structural_checks: [(&str, bool); 4] = [
        ("LRA version tag (lra:version)", body.contains("lra:version")),
        ("Service Worker reference", body.contains("sw.js") || body.contains("serviceWorker")),
        ("Claims markup (data-lra-claim)", !claims.is_empty()),
        ("Capability tag (lra:capability)", body.contains("lra:capability")),
    ];

    let mut si = 0;
    while si < structural_checks.len() && si < MAX_CHECKS {
        let (label, pass) = structural_checks[si];
        let tag = if pass { "PASS" } else { "FAIL" };
        println!("  [{}] {}", tag, label);
        if pass {
            pass_count += 1;
        }
        check_count += 1;
        si += 1;
    }

    println!("\n  Result: {}/{} checks passed", pass_count, check_count);

    // Step 7: Optionally produce a JSON verification receipt
    if let Some(rpath) = receipt_path {
        let integrity = if let Some(ref _entry) = registry_result {
            if live_hash == _entry.hash {
                "match"
            } else {
                "mismatch"
            }
        } else {
            "not_in_registry"
        };

        let registry_hash_str = registry_result.as_ref().map(|e| e.hash.as_str()).unwrap_or("null");

        let receipt = serde_json::json!({
            "target_url": url,
            "target_hash": live_hash,
            "registry_hash": registry_hash_str,
            "integrity": integrity,
            "claims_found": claims.len(),
            "structural_checks": {
                "lra_version": structural_checks[0].1,
                "sw_reference": structural_checks[1].1,
                "claims_markup": structural_checks[2].1,
                "capability_tag": structural_checks[3].1
            },
            "timestamp": chrono_iso8601(),
            "verifier_version": env!("CARGO_PKG_VERSION")
        });

        match serde_json::to_string_pretty(&receipt) {
            Ok(json) => {
                if let Err(e) = std::fs::write(rpath, &json) {
                    eprintln!("  Warning: Cannot write receipt: {}", e);
                } else {
                    println!("  Receipt: {}", rpath);
                }
            }
            Err(e) => eprintln!("  Warning: Cannot serialize receipt: {}", e),
        }
    }

    if pass_count == check_count {
        0
    } else {
        1
    }
}

/// Resolve a target (URL or hash) from the registry file.
fn resolve_from_registry(target: &str, registry_path: &str) -> Option<registry::RegistryEntry> {
    let path = std::path::Path::new(registry_path);
    if !path.exists() {
        return None;
    }
    let json = util::bounded_read_to_string(path);
    let reg = match registry::parse_registry(&json) {
        Ok(r) => r,
        Err(_) => return None,
    };
    let mut i = 0;
    while i < reg.entries.len() && i < MAX_CHECKS {
        if reg.entries[i].url == target || reg.entries[i].hash == target {
            return Some(reg.entries[i].clone());
        }
        i += 1;
    }
    None
}

/// Extract claims from HTML `data-lra-claim` attributes (bounded).
fn extract_claims(html: &str) -> Vec<(String, String, bool)> {
    let mut claims = Vec::new();
    let marker = "data-lra-claim=\"";
    let mut pos = 0;
    let bytes = html.as_bytes();
    while pos < bytes.len() && claims.len() < MAX_CLAIMS_EXTRACT {
        match html[pos..].find(marker) {
            Some(offset) => {
                let start = pos + offset + marker.len();
                if let Some(end) = html[start..].find('"') {
                    let claim_id = &html[start..start + end];
                    // Look for nearby text content (simplified extraction)
                    let text = extract_nearby_text(html, start + end);
                    let has_evidence = html[pos..pos + offset + 200.min(html.len() - pos - offset)]
                        .contains("evidence");
                    claims.push((claim_id.to_string(), text, has_evidence));
                    pos = start + end + 1;
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    claims
}

/// Extract text content near a position in HTML (simplified, bounded).
fn extract_nearby_text(html: &str, start: usize) -> String {
    let search_end = (start + 500).min(html.len());
    let chunk = &html[start..search_end];
    // Find text between > and <
    let mut result = String::new();
    let mut in_tag = false;
    let mut ci = 0;
    let chunk_bytes = chunk.as_bytes();
    while ci < chunk_bytes.len() && result.len() < 200 {
        let ch = chunk_bytes[ci];
        if ch == b'<' {
            in_tag = true;
            if !result.is_empty() {
                break;
            }
        } else if ch == b'>' {
            in_tag = false;
        } else if !in_tag {
            result.push(ch as char);
        }
        ci += 1;
    }
    result.trim().to_string()
}

/// Truncate a string to a maximum length (bounded).
fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max])
    }
}

/// Produce an ISO-8601 timestamp without a chrono dependency.
/// Format: YYYY-MM-DDTHH:MM:SSZ (always UTC from SystemTime).
fn chrono_iso8601() -> String {
    let dur =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = dur.as_secs();
    // Bounded arithmetic — no recursion, no unbounded iteration
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let mins = (time_secs % 3600) / 60;
    let s = time_secs % 60;
    // Simplified date calculation (good through 2099)
    let mut y = 1970u64;
    let mut remaining_days = days;
    let max_years = 200;
    let mut year_iter = 0;
    while year_iter < max_years {
        let days_in_year =
            if y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400)) {
                366
            } else {
                365
            };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        y += 1;
        year_iter += 1;
    }
    let month_days: [u64; 12] =
        if y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400)) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
    let mut m = 0usize;
    while m < 12 {
        if remaining_days < month_days[m] {
            break;
        }
        remaining_days -= month_days[m];
        m += 1;
    }
    let d = remaining_days + 1;
    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", y, m + 1, d, hours, mins, s)
}
