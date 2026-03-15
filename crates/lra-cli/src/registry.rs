#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Maximum entries in a single registry manifest (NASA Power-of-10 bound).
const MAX_REGISTRY_ENTRIES: usize = 10_000;

/// Maximum registry file size (5 MB).
const MAX_REGISTRY_SIZE: usize = 5 * 1024 * 1024;

/// Maximum search results returned.
const MAX_SEARCH_RESULTS: usize = 100;

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub version: String,
    pub updated: String,
    pub entries: Vec<RegistryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub title: String,
    pub authors: Vec<String>,
    pub url: String,
    pub hash: String,
    pub capability: String,
    pub keywords: Vec<String>,
    pub license: String,
    pub version: String,
    pub depends: Vec<String>,
    pub registered: String,
    #[serde(default)]
    pub peers: Vec<String>,
    #[serde(default)]
    pub verifications: Option<Verifications>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Verifications {
    pub last_verified: Option<String>,
    pub total_verifications: u64,
    pub total_challenges: u64,
}

/// Parse a registry manifest from JSON string.
/// Bounded: refuses files > MAX_REGISTRY_SIZE, entries > MAX_REGISTRY_ENTRIES.
pub fn parse_registry(json: &str) -> Result<Registry, String> {
    if json.len() > MAX_REGISTRY_SIZE {
        return Err(format!("Registry exceeds {} bytes", MAX_REGISTRY_SIZE));
    }
    let reg: Registry =
        serde_json::from_str(json).map_err(|e| format!("Invalid registry JSON: {}", e))?;
    if reg.entries.len() > MAX_REGISTRY_ENTRIES {
        return Err(format!("Registry exceeds {} entries", MAX_REGISTRY_ENTRIES));
    }
    Ok(reg)
}

/// Search registry entries by keyword (case-insensitive substring match).
/// Returns matching entries up to MAX_SEARCH_RESULTS.
pub fn search_entries<'a>(reg: &'a Registry, query: &str) -> Vec<&'a RegistryEntry> {
    let q = query.to_lowercase();
    let mut results = Vec::new();
    let mut i = 0;
    let max = reg.entries.len();
    while i < max && results.len() < MAX_SEARCH_RESULTS {
        let entry = &reg.entries[i];
        let matches = entry.title.to_lowercase().contains(&q)
            || entry.keywords.iter().any(|k| k.to_lowercase().contains(&q))
            || entry.capability.to_lowercase().contains(&q)
            || entry.authors.iter().any(|a| a.to_lowercase().contains(&q));
        if matches {
            results.push(entry);
        }
        i += 1;
    }
    results
}
