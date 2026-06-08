#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

const MAX_INPUT_SIZE: usize = 2 * 1024 * 1024; // 2 MB
const MAX_FRONTMATTER_LINES: usize = 200;
const MAX_CLAIMS: usize = 100;
const MAX_REFERENCES: usize = 500;

/// Build a Markdown file into LRA-compliant HTML. Returns exit code.
pub fn run(input: &str, output: &str) -> i32 {
    let input_path = Path::new(input);
    if !input_path.exists() {
        eprintln!("Error: file not found: {}", input);
        return 1;
    }

    let source = match fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", input, e);
            return 1;
        }
    };

    if source.len() > MAX_INPUT_SIZE {
        eprintln!("Error: input too large ({} bytes, limit {})", source.len(), MAX_INPUT_SIZE);
        return 1;
    }

    let (frontmatter, body) = match split_frontmatter(&source) {
        Some(pair) => pair,
        None => {
            eprintln!("Error: missing YAML frontmatter (file must start with ---)");
            return 1;
        }
    };

    let meta = match parse_frontmatter(frontmatter) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error in frontmatter: {}", e);
            return 1;
        }
    };

    let html_body = render_markdown(body);
    let html = assemble(&meta, &html_body);

    if let Err(e) = fs::write(output, &html) {
        eprintln!("Error writing {}: {}", output, e);
        return 1;
    }

    println!("Built: {} -> {} ({} bytes)", input, output, html.len());
    0
}

// ── Frontmatter parsing ──────────────────────────────────────────────

struct Meta {
    title: String,
    author: String,
    date: String,
    description: String,
    url: String,
    license: String,
    keywords: Vec<String>,
    capability: String,
    claims: Vec<Claim>,
}

struct Claim {
    text: String,
    evidence: String,
}

fn split_frontmatter(source: &str) -> Option<(&str, &str)> {
    let trimmed = source.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }

    // Find the closing ---
    let after_first = &trimmed[3..];
    let closing = after_first.find("\n---")?;
    let frontmatter = &after_first[..closing];
    let body_start = closing + 4; // skip \n---
    let body = after_first[body_start..].trim_start_matches(['\n', '\r']);
    Some((frontmatter, body))
}

fn parse_frontmatter(fm: &str) -> Result<Meta, String> {
    // Simple key: value parser (no full YAML library needed)
    let mut meta = Meta {
        title: String::new(),
        author: String::new(),
        date: String::new(),
        description: String::new(),
        url: String::new(),
        license: "Apache-2.0".to_string(),
        keywords: Vec::new(),
        capability: String::new(),
        claims: Vec::new(),
    };

    let mut in_claims = false;
    let mut current_claim_text = String::new();
    let mut current_claim_evidence = String::new();
    let mut line_count = 0;

    for line in fm.lines() {
        line_count += 1;
        if line_count > MAX_FRONTMATTER_LINES {
            return Err("frontmatter exceeds maximum line count".to_string());
        }

        let trimmed = line.trim();

        // Claims array parsing
        if in_claims {
            if trimmed.starts_with("- text:") {
                // Flush previous claim
                if !current_claim_text.is_empty() {
                    if meta.claims.len() >= MAX_CLAIMS {
                        return Err("too many claims".to_string());
                    }
                    meta.claims.push(Claim {
                        text: current_claim_text.clone(),
                        evidence: current_claim_evidence.clone(),
                    });
                }
                current_claim_text = unquote(trimmed.trim_start_matches("- text:").trim());
                current_claim_evidence.clear();
            } else if trimmed.starts_with("evidence:") {
                current_claim_evidence = unquote(trimmed.trim_start_matches("evidence:").trim());
            } else if !trimmed.starts_with('-')
                && !trimmed.starts_with("evidence:")
                && !trimmed.is_empty()
            {
                // End of claims section — new top-level key
                if trimmed.contains(':') && !trimmed.starts_with('#') {
                    in_claims = false;
                    // Flush last claim
                    if !current_claim_text.is_empty() {
                        if meta.claims.len() >= MAX_CLAIMS {
                            return Err("too many claims".to_string());
                        }
                        meta.claims.push(Claim {
                            text: current_claim_text.clone(),
                            evidence: current_claim_evidence.clone(),
                        });
                        current_claim_text.clear();
                        current_claim_evidence.clear();
                    }
                    // Fall through to parse this line as a regular key
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }

        if in_claims {
            continue;
        }

        // Top-level key: value
        if let Some((key, value)) = trimmed.split_once(':') {
            let key = key.trim();
            let value = unquote(value.trim());

            match key {
                "title" => meta.title = value,
                "author" => meta.author = value,
                "date" => meta.date = value,
                "description" => meta.description = value,
                "url" => meta.url = value,
                "license" => meta.license = value,
                "capability" => meta.capability = value,
                "keywords" => {
                    meta.keywords = parse_yaml_array(&value);
                }
                "claims" => {
                    in_claims = true;
                }
                _ => {} // Ignore unknown keys
            }
        }
    }

    // Flush last claim
    if !current_claim_text.is_empty() {
        if meta.claims.len() >= MAX_CLAIMS {
            return Err("too many claims".to_string());
        }
        meta.claims.push(Claim { text: current_claim_text, evidence: current_claim_evidence });
    }

    if meta.title.is_empty() {
        return Err("missing required field: title".to_string());
    }
    if meta.author.is_empty() {
        return Err("missing required field: author".to_string());
    }

    Ok(meta)
}

fn unquote(s: &str) -> String {
    let trimmed = s.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        trimmed[1..trimmed.len() - 1].to_string()
    } else {
        trimmed.to_string()
    }
}

fn parse_yaml_array(s: &str) -> Vec<String> {
    // Handle ["a", "b", "c"] style
    let trimmed = s.trim();
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let inner = &trimmed[1..trimmed.len() - 1];
        return inner
            .split(',')
            .map(|item| unquote(item.trim()))
            .filter(|item| !item.is_empty())
            .collect();
    }
    // Single value
    if !trimmed.is_empty() {
        vec![trimmed.to_string()]
    } else {
        Vec::new()
    }
}

// ── Markdown rendering ───────────────────────────────────────────────

fn render_markdown(body: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // Pre-process: wrap $...$ in <span class="math"> markers
    let processed = preprocess_math(body);
    // Pre-process: resolve [@key] citation markers
    let (processed, citations) = preprocess_citations(&processed);

    let parser = Parser::new_ext(&processed, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Append auto-generated reference list if citations were found
    if !citations.is_empty() {
        html_output.push_str(
            "\n<section id=\"references\">\n<h2>References</h2>\n<ol class=\"references\">\n",
        );
        for (i, key) in citations.iter().enumerate() {
            html_output.push_str(&format!(
                "  <li id=\"ref-{}\">[{}] {}</li>\n",
                i + 1,
                i + 1,
                escape_html(key)
            ));
        }
        html_output.push_str("</ol>\n</section>\n");
    }

    html_output
}

fn preprocess_math(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    let mut i = 0;

    // Bounded iteration (NASA Power-of-10)
    const MAX_CHARS: usize = 4_000_000;
    let effective_len = if len > MAX_CHARS { MAX_CHARS } else { len };

    while i < effective_len {
        if chars[i] == '$' {
            // Check for display math $$...$$
            if i + 1 < effective_len && chars[i + 1] == '$' {
                let start = i + 2;
                let mut end = start;
                while end + 1 < effective_len && !(chars[end] == '$' && chars[end + 1] == '$') {
                    end += 1;
                }
                if end + 1 < effective_len {
                    let math: String = chars[start..end].iter().collect();
                    result.push_str("<span class=\"math display\">");
                    result.push_str(&escape_html(&math));
                    result.push_str("</span>");
                    i = end + 2;
                    continue;
                }
            }
            // Inline math $...$
            let start = i + 1;
            let mut end = start;
            while end < effective_len && chars[end] != '$' {
                end += 1;
            }
            if end < effective_len && end > start {
                let math: String = chars[start..end].iter().collect();
                result.push_str("<span class=\"math\">");
                result.push_str(&escape_html(&math));
                result.push_str("</span>");
                i = end + 1;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

fn preprocess_citations(input: &str) -> (String, Vec<String>) {
    let mut result = String::with_capacity(input.len());
    let mut citations: Vec<String> = Vec::new();
    let len = input.len();

    const MAX_CHARS: usize = 4_000_000;
    let effective = if len > MAX_CHARS { &input[..MAX_CHARS] } else { input };

    let mut chars = effective.char_indices().peekable();
    while let Some((_i, c)) = chars.next() {
        // Look for [@key]
        if c == '[' {
            if let Some(&(_, '@')) = chars.peek() {
                chars.next(); // consume '@'
                let start = if let Some(&(s, _)) = chars.peek() { s } else { break };
                let mut end = start;
                let mut found = false;
                while let Some(&(e, ch)) = chars.peek() {
                    if ch == ']' {
                        end = e;
                        found = true;
                        chars.next(); // consume ']'
                        break;
                    }
                    chars.next();
                }
                if found {
                    let key = &effective[start..end];
                    let num = if let Some(pos) = citations.iter().position(|c| c == key) {
                        pos + 1
                    } else {
                        if citations.len() < MAX_REFERENCES {
                            citations.push(key.to_string());
                        }
                        citations.len()
                    };
                    result.push_str(&format!(
                        "<a href=\"#ref-{}\" class=\"citation\">[{}]</a>",
                        num, num
                    ));
                    continue;
                }
                // Malformed — output the consumed chars literally
                result.push('[');
                result.push('@');
                result.push_str(&effective[start..end]);
                continue;
            }
        }
        result.push(c);
    }

    (result, citations)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

// ── HTML assembly ────────────────────────────────────────────────────

fn assemble(meta: &Meta, body_html: &str) -> String {
    let keywords_json: Vec<String> =
        meta.keywords.iter().map(|k| format!("\"{}\"", escape_html(k))).collect();
    let keywords_csv = meta.keywords.join(", ");

    let capability_meta = if meta.capability.is_empty() {
        String::new()
    } else {
        format!("  <meta name=\"lra:capability\" content=\"{}\">\n", escape_html(&meta.capability))
    };

    let claims_html = if meta.claims.is_empty() {
        String::new()
    } else {
        let mut s = String::from("<section id=\"claims\">\n  <h2>Claims</h2>\n  <p>Each claim below is linked to evidence within this paper.</p>\n  <ol>\n");
        for (i, claim) in meta.claims.iter().enumerate() {
            s.push_str(&format!("    <li id=\"claim-{}\" data-lra-claim=\"{}\">", i + 1, i + 1));
            s.push_str(&escape_html(&claim.text));
            if !claim.evidence.is_empty() {
                s.push_str(&format!(
                    " <a href=\"{}\" data-lra-evidence=\"{}\">[Evidence &darr;]</a>",
                    escape_html(&claim.evidence),
                    escape_html(claim.evidence.trim_start_matches('#'))
                ));
            }
            s.push_str("</li>\n");
        }
        s.push_str("  </ol>\n</section>\n");
        s
    };

    let bibtex_key = meta
        .title
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>()
        .to_lowercase();
    let year = if meta.date.len() >= 4 { &meta.date[..4] } else { &meta.date };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title}</title>
  <link rel="stylesheet" href="paper.css">
  <link rel="icon" href="data:,">
  <meta name="lra:version" content="1.0">
{capability}  <meta name="lra:license" content="{license}">
  <meta name="description" content="{description}">
  <meta property="og:title" content="{title}">
  <meta property="og:description" content="{description}">
  <meta property="og:image" content="lra-card.svg">
  <meta property="og:image:width" content="1200">
  <meta property="og:image:height" content="630">
  <meta property="og:type" content="article">
  <meta name="twitter:card" content="summary_large_image">
  <meta name="lra:abstract" content="{description}">
  <meta name="lra:keywords" content="{keywords_csv}">
  <script type="application/ld+json">
  {{
    "@context": "https://schema.org",
    "@type": "ScholarlyArticle",
    "name": "{title}",
    "author": {{ "@type": "Person", "name": "{author}" }},
    "datePublished": "{date}",
    "description": "{description}",
    "license": "https://www.gnu.org/licenses/gpl-3.0.html",
    "url": "{url}",
    "keywords": [{keywords_json}]
  }}
  </script>
</head>
<body>

<header>
  <h1>{title}</h1>
  <p class="meta">
    {author} &middot; {date} &middot;
    <a href="LICENSE">{license}</a> &middot;
    <a href="CITATION.cff">CITATION.cff</a>
  </p>
</header>

<noscript>
  <p>This paper includes interactive demos that require JavaScript.
  All text, tables, and figures are readable without JavaScript.</p>
</noscript>

<main>

{body}

{claims}
<section id="citation">
  <h2>How to Cite This Work</h2>
  <pre id="citation-block">
@software{{{bibtex_key},
  title  = {{{title}}},
  author = {{{author}}},
  year   = {{{year}}},
  url    = {{{url}}},
  license = {{{license}}}
}}
  </pre>
  <p>
    Or use <a href="CITATION.cff">CITATION.cff</a> for
    citation managers. Cite by commit hash for exact reproducibility.
  </p>
</section>
</main>

<footer>
  <p>
    Living Research Artifact &middot; {license} &middot;
    Built from commit <code id="commit-hash">dev</code> &middot;
    No external dependencies &middot; No server required
  </p>
  <p>
    Standard: <a href="spec/LRA-1.0.md">LRA-1.0</a>
  </p>
</footer>

<script src="paper.js"></script>
</body>
</html>
"#,
        title = escape_html(&meta.title),
        author = escape_html(&meta.author),
        date = escape_html(&meta.date),
        description = escape_html(&meta.description),
        url = escape_html(&meta.url),
        license = escape_html(&meta.license),
        capability = capability_meta,
        keywords_csv = escape_html(&keywords_csv),
        keywords_json = keywords_json.join(", "),
        body = body_html,
        claims = claims_html,
        bibtex_key = bibtex_key,
        year = year,
    )
}
