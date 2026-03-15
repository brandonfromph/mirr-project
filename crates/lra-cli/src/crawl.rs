#![forbid(unsafe_code)]

use crate::registry;
use crate::util;

/// Maximum nodes to crawl (NASA Power-of-10 bound).
const MAX_CRAWL_NODES: usize = 100;

/// Maximum HTTP timeout in seconds.
const MAX_TIMEOUT_SECS: u64 = 10;

/// Maximum HTML response size (10 MB).
const MAX_HTML_SIZE: usize = 10 * 1024 * 1024;

/// Maximum meta tags to scan per page.
const MAX_META_SCAN: usize = 200;

pub fn run(seed: &str, registry_path: &str) -> i32 {
    println!("LRA Network Crawl \u{2014} seed: {}\n", seed);

    let agent = ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(MAX_TIMEOUT_SECS))
        .timeout_read(std::time::Duration::from_secs(MAX_TIMEOUT_SECS))
        .build();

    // Load registry for metadata enrichment
    let reg_path = std::path::Path::new(registry_path);
    let reg_json = util::bounded_read_to_string(reg_path);
    let reg = registry::parse_registry(&reg_json).ok();

    // Crawl queue: start with seed
    let mut queue: Vec<String> = vec![seed.to_string()];
    let mut visited: Vec<String> = Vec::new();
    let mut nodes: Vec<CrawlNode> = Vec::new();

    let mut qi = 0;
    while qi < queue.len() && visited.len() < MAX_CRAWL_NODES {
        let url = queue[qi].clone();
        qi += 1;

        // Skip already visited
        if visited.iter().any(|v| v == &url) {
            continue;
        }
        visited.push(url.clone());

        // Fetch the page (single fetch — reuse body for all checks)
        let body = match agent.get(&url).call() {
            Ok(response) => {
                if response.status() != 200 {
                    continue;
                }
                match response.into_string() {
                    Ok(b) if b.len() <= MAX_HTML_SIZE => b,
                    _ => continue,
                }
            }
            Err(_) => continue,
        };

        // Check if this is an LRA node (must have lra:capability meta tag)
        let capability = extract_meta(&body, "lra:capability");
        if capability.is_none() {
            continue;
        }

        // Extract metadata
        let version = extract_meta(&body, "lra:version").unwrap_or_default();
        let claim_count = count_claims(&body);
        let depends = extract_depends(&body);

        // Enrich from registry
        let reg_entry = reg.as_ref().and_then(|r| {
            let mut i = 0;
            while i < r.entries.len() && i < MAX_CRAWL_NODES {
                if r.entries[i].url == url {
                    return Some(&r.entries[i]);
                }
                i += 1;
            }
            None
        });

        let title =
            reg_entry.map(|e| e.title.clone()).unwrap_or_else(|| "(unknown title)".to_string());

        nodes.push(CrawlNode {
            url: url.clone(),
            title,
            capability: capability.unwrap_or_default(),
            version,
            claims: claim_count,
            depends_count: depends.len(),
        });

        // Add dependencies to crawl queue (if they look like URLs)
        let mut di = 0;
        while di < depends.len() && di < MAX_CRAWL_NODES {
            if depends[di].starts_with("http://") || depends[di].starts_with("https://") {
                if queue.len() < MAX_CRAWL_NODES && !queue.iter().any(|q| q == &depends[di]) {
                    queue.push(depends[di].clone());
                }
            }
            di += 1;
        }
    }

    // Print results
    println!("  Discovered {} node(s):\n", nodes.len());
    let mut ni = 0;
    while ni < nodes.len() && ni < MAX_CRAWL_NODES {
        let node = &nodes[ni];
        println!("  [{}] {}", ni + 1, node.title);
        println!("      URL:        {}", node.url);
        println!("      Capability: {}", node.capability);
        println!("      Version:    {}", node.version);
        println!("      Claims:     {}", node.claims);
        println!("      Depends:    {}", node.depends_count);
        println!();
        ni += 1;
    }

    println!("Crawl complete: {} node(s), {} URL(s) visited", nodes.len(), visited.len());
    0
}

struct CrawlNode {
    url: String,
    title: String,
    capability: String,
    version: String,
    claims: usize,
    depends_count: usize,
}

/// Extract a meta tag content value by name from HTML (bounded scan).
fn extract_meta(html: &str, name: &str) -> Option<String> {
    let pattern = format!("name=\"{}\"", name);
    let pos = html.find(&pattern)?;
    let search_start = pos;
    let search_end = (search_start + 500).min(html.len());
    let chunk = &html[search_start..search_end];
    let content_marker = "content=\"";
    let content_pos = chunk.find(content_marker)?;
    let value_start = content_pos + content_marker.len();
    let remaining = &chunk[value_start..];
    let end = remaining.find('"')?;
    Some(remaining[..end].to_string())
}

/// Count data-lra-claim attributes in HTML (bounded).
fn count_claims(html: &str) -> usize {
    let marker = "data-lra-claim";
    let mut count = 0;
    let mut pos = 0;
    while pos < html.len() && count < MAX_META_SCAN {
        match html[pos..].find(marker) {
            Some(offset) => {
                count += 1;
                pos += offset + marker.len();
            }
            None => break,
        }
    }
    count
}

/// Extract lra:depends meta tag values from HTML (bounded).
fn extract_depends(html: &str) -> Vec<String> {
    let mut results = Vec::new();
    let marker = "lra:depends";
    let mut pos = 0;
    while pos < html.len() && results.len() < MAX_META_SCAN {
        match html[pos..].find(marker) {
            Some(offset) => {
                let chunk_start = pos + offset;
                let chunk_end = (chunk_start + 500).min(html.len());
                let chunk = &html[chunk_start..chunk_end];
                if let Some(content_pos) = chunk.find("content=\"") {
                    let value_start = content_pos + "content=\"".len();
                    if let Some(end) = chunk[value_start..].find('"') {
                        results.push(chunk[value_start..value_start + end].to_string());
                    }
                }
                pos = chunk_start + marker.len();
            }
            None => break,
        }
    }
    results
}
