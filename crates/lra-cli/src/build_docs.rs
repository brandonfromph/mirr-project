#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Options, Parser};

const MAX_INPUT_SIZE: usize = 2 * 1024 * 1024;
const MAX_FILES: usize = 200;
const MAX_HIGHLIGHT_TOKENS: usize = 10_000;
const MAX_TOC_HEADINGS: usize = 100;
const MAX_SEARCH_INDEX_ENTRIES: usize = 1000;
const MAX_SEARCH_TERM_LENGTH: usize = 200;

/// Build all Markdown files in a docs directory into static HTML pages.
/// Returns exit code.
pub fn run(input_dir: &str, output_dir: &str, css_path: &str) -> i32 {
    let input = Path::new(input_dir);
    let output = Path::new(output_dir);

    if !input.is_dir() {
        eprintln!("Error: not a directory: {}", input_dir);
        return 1;
    }

    if let Err(e) = fs::create_dir_all(output) {
        eprintln!("Error creating output dir: {}", e);
        return 1;
    }

    // Collect all .md files
    let mut md_files: Vec<_> = match fs::read_dir(input) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension() == Some(std::ffi::OsStr::new("md")))
            .collect(),
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            return 1;
        }
    };

    if md_files.len() > MAX_FILES {
        eprintln!("Error: too many files ({}, limit {})", md_files.len(), MAX_FILES);
        return 1;
    }

    // Sort for deterministic output
    md_files.sort_by_key(|e| e.file_name());

    // First pass: collect nav entries (title + filename)
    let mut nav_entries: Vec<NavEntry> = Vec::new();
    for entry in &md_files {
        let path = entry.path();
        let source = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        if source.len() > MAX_INPUT_SIZE {
            eprintln!("Error: {} too large ({} bytes)", path.display(), source.len());
            return 1;
        }
        let meta = extract_meta(&source);
        let (_, html_name) = match html_filename(&path) {
            Some(pair) => pair,
            None => continue,
        };
        nav_entries.push(NavEntry { title: meta.title, html_name, nav_order: meta.nav_order });
    }
    nav_entries.sort_by_key(|e| e.nav_order);

    // Only show files with explicit nav_order in the nav (exclude legacy/hidden files)
    let visible_nav: Vec<&NavEntry> = nav_entries.iter().filter(|e| e.nav_order < 900).collect();

    // Build the nav HTML once
    let nav_html = build_nav(&visible_nav);

    // Second pass: render each file
    let mut built = 0;
    let mut search_entries: Vec<SearchEntry> = Vec::new();
    for entry in &md_files {
        let path = entry.path();
        let source = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {}: {}", path.display(), e);
                return 1;
            }
        };

        if source.len() > MAX_INPUT_SIZE {
            eprintln!("Error: {} too large ({} bytes)", path.display(), source.len());
            return 1;
        }

        let meta = extract_meta(&source);
        let body = strip_frontmatter(&source);
        let body_html = render_markdown(body);

        let (_, html_name) = match html_filename(&path) {
            Some(pair) => pair,
            None => continue,
        };

        // Extract TOC entries and build TOC HTML
        let toc_entries = extract_toc_entries(body);
        let toc_html = if meta.toc_enabled && !toc_entries.is_empty() {
            build_toc_html(&toc_entries)
        } else {
            String::new()
        };

        // Inject heading IDs into rendered HTML
        let body_html = inject_heading_ids(&body_html);

        // Collect search index entry
        if search_entries.len() < MAX_SEARCH_INDEX_ENTRIES {
            let headings: Vec<String> = toc_entries.iter().map(|e| e.title.clone()).collect();
            let snippet = extract_first_paragraph(&body_html);
            search_entries.push(SearchEntry {
                title: meta.title.clone(),
                url: html_name.clone(),
                headings,
                snippet,
            });
        }

        let page_html =
            assemble_page(&meta.title, &html_name, &nav_html, &body_html, &toc_html, css_path);

        let out_path = output.join(&html_name);
        if let Err(e) = fs::write(&out_path, &page_html) {
            eprintln!("Error writing {}: {}", out_path.display(), e);
            return 1;
        }
        built += 1;
    }

    // Write search index
    if let Err(e) = write_search_index(output, &search_entries) {
        eprintln!("Warning: failed to write search index: {}", e);
    }

    // Copy CSS file if it exists in the input directory
    let css_src = input.join("style.css");
    if css_src.exists() {
        let css_dst = output.join("style.css");
        if let Err(e) = fs::copy(&css_src, &css_dst) {
            eprintln!("Warning: failed to copy style.css: {}", e);
        }
    }

    // Copy assets directory if it exists
    let assets_src = input.join("assets");
    if assets_src.is_dir() {
        if let Err(e) = copy_dir_recursive(&assets_src, &output.join("assets")) {
            eprintln!("Warning: failed to copy assets: {}", e);
        }
    }

    println!("Built {} docs pages -> {}", built, output_dir);
    0
}

struct NavEntry {
    title: String,
    html_name: String,
    nav_order: i32,
}

struct PageMeta {
    title: String,
    nav_order: i32,
    toc_enabled: bool,
}

struct TocEntry {
    level: u8,
    id: String,
    title: String,
}

struct SearchEntry {
    title: String,
    url: String,
    headings: Vec<String>,
    snippet: String,
}

/// Compute the HTML filename from a Markdown source path.
/// Returns `(stem, html_name)` or `None` if the path has no file stem.
fn html_filename(path: &Path) -> Option<(String, String)> {
    let stem = path.file_stem()?.to_string_lossy().to_string();
    let html_name =
        if stem == "home" { "index.html".to_string() } else { format!("{}.html", stem) };
    Some((stem, html_name))
}

/// Extract title, nav_order, and toc_enabled from YAML frontmatter.
fn extract_meta(source: &str) -> PageMeta {
    let mut title = String::new();
    let mut nav_order = 999;
    let mut toc_enabled = true;

    let trimmed = source.trim_start();
    if !trimmed.starts_with("---") {
        // No frontmatter — use first heading as title
        for line in source.lines().take(20) {
            if let Some(h) = line.strip_prefix("# ") {
                title = h.trim().to_string();
                break;
            }
        }
        return PageMeta { title, nav_order, toc_enabled };
    }

    let after = &trimmed[3..];
    let closing = match after.find("\n---") {
        Some(c) => c,
        None => return PageMeta { title, nav_order, toc_enabled },
    };
    let fm = &after[..closing];

    const MAX_FRONTMATTER_LINES: usize = 50;

    for line in fm.lines().take(MAX_FRONTMATTER_LINES) {
        let line = line.trim();
        if let Some(val) = line.strip_prefix("title:") {
            title = unquote(val.trim());
        } else if let Some(val) = line.strip_prefix("nav_order:") {
            if let Ok(n) = val.trim().parse::<i32>() {
                nav_order = n;
            }
        } else if let Some(val) = line.strip_prefix("toc:") {
            if val.trim() == "false" {
                toc_enabled = false;
            }
        }
    }

    PageMeta { title, nav_order, toc_enabled }
}

fn unquote(s: &str) -> String {
    let t = s.trim();
    if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
        t[1..t.len() - 1].to_string()
    } else {
        t.to_string()
    }
}

/// Strip YAML frontmatter, returning only the body.
fn strip_frontmatter(source: &str) -> &str {
    let trimmed = source.trim_start();
    if !trimmed.starts_with("---") {
        return source;
    }
    let after = &trimmed[3..];
    match after.find("\n---") {
        Some(pos) => {
            let rest = &after[pos + 4..];
            rest.trim_start_matches(['\n', '\r'])
        }
        None => source,
    }
}

/// Render Markdown to HTML via pulldown-cmark.
fn render_markdown(body: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // Resolve internal .md links to .html (e.g., [Foo](roadmap) -> roadmap.html)
    let processed = resolve_internal_links(body);

    let parser = Parser::new_ext(&processed, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Convert kramdown callouts {: .note } etc. to styled divs
    let with_callouts = postprocess_callouts(&html_output);

    // Apply MIRR syntax highlighting to ```mirr code blocks
    postprocess_mirr_highlight(&with_callouts)
}

/// Convert bare page links like [text](page-name) to [text](page-name.html).
/// Only converts links that don't already have an extension or protocol.
fn resolve_internal_links(body: &str) -> String {
    let mut result = String::with_capacity(body.len());
    let mut chars = body.char_indices().peekable();

    const MAX_ITER: usize = 4_000_000;
    let mut iter_count = 0;

    while let Some((i, c)) = chars.next() {
        iter_count += 1;
        if iter_count > MAX_ITER {
            result.push_str(&body[i..]);
            break;
        }

        if c == ']' {
            result.push(c);
            // Check for (link) immediately after ]
            if let Some(&(_, '(')) = chars.peek() {
                chars.next();
                result.push('(');

                // Collect the link target
                let mut link = String::new();
                let mut depth = 1;
                while let Some(&(_, ch)) = chars.peek() {
                    if ch == ')' {
                        depth -= 1;
                        if depth == 0 {
                            chars.next();
                            break;
                        }
                    } else if ch == '(' {
                        depth += 1;
                    }
                    link.push(ch);
                    chars.next();
                }

                // Resolve: if link is a bare name (no /, no ., no http)
                // convert to .html
                let resolved = if !link.contains('/')
                    && !link.contains('.')
                    && !link.contains(':')
                    && !link.starts_with('#')
                    && !link.is_empty()
                {
                    if link == "home" {
                        "index.html".to_string()
                    } else {
                        format!("{}.html", link)
                    }
                } else {
                    link
                };

                result.push_str(&resolved);
                result.push(')');
            }
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert kramdown-style callouts into styled HTML divs.
fn postprocess_callouts(html: &str) -> String {
    // Pattern: <p>{: .note }</p> followed by <blockquote>
    // Scoped replacement: only the blockquote paired with a callout marker
    // is converted — other blockquotes are left untouched.
    let mut result = html.to_string();

    const MAX_CALLOUT_PASSES: usize = 50;

    for class in &["note", "tip", "warning", "important"] {
        let marker = format!("<p>{{: .{} }}</p>\n<blockquote>", class);
        let replacement_open = format!(
            "<div class=\"callout {}\"><div class=\"callout-title\">{}</div>\n<div>",
            class,
            capitalize(class)
        );

        let mut passes = 0;
        while let Some(pos) = result.find(&marker) {
            passes += 1;
            if passes > MAX_CALLOUT_PASSES {
                break;
            }

            // Replace the marker + <blockquote> open tag
            result =
                format!("{}{}{}", &result[..pos], replacement_open, &result[pos + marker.len()..]);

            // Find the NEXT </blockquote> after the insertion and replace only that one
            let search_start = pos + replacement_open.len();
            if let Some(close_offset) = result[search_start..].find("</blockquote>") {
                let close_pos = search_start + close_offset;
                let close_tag = "</blockquote>";
                result = format!(
                    "{}{}{}",
                    &result[..close_pos],
                    "</div></div>",
                    &result[close_pos + close_tag.len()..]
                );
            }
        }

        // Strip any remaining inline markers for this class
        let inline_marker = format!("{{: .{} }}", class);
        result = result.replace(&inline_marker, "");
    }

    result
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Build sidebar navigation HTML from nav entries.
fn build_nav(entries: &[&NavEntry]) -> String {
    let mut nav = String::from("<nav class=\"site-nav\" aria-label=\"Documentation\">\n");

    for entry in entries {
        nav.push_str(&format!(
            "    <a href=\"{}\">{}</a>\n",
            escape_html(&entry.html_name),
            escape_html(&entry.title)
        ));
    }

    // Paper link
    nav.push_str("    <div class=\"paper-link\"><a href=\"../paper/index.html\">Interactive Paper &rarr;</a></div>\n");
    nav.push_str("  </nav>");
    nav
}

/// Assemble a full HTML page.
fn assemble_page(
    title: &str,
    _html_name: &str,
    nav_html: &str,
    body_html: &str,
    toc_html: &str,
    css_path: &str,
) -> String {
    let toc_section = if toc_html.is_empty() {
        String::new()
    } else {
        format!("  <aside class=\"page-toc-container\">\n{}\n  </aside>\n", toc_html)
    };
    let content_class =
        if toc_html.is_empty() { "site-content" } else { "site-content content-with-toc" };
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} — MIRR Documentation</title>
  <link rel="icon" type="image/svg+xml" href="assets/images/mirr_logo.svg">
  <link rel="stylesheet" href="{css}">
</head>
<body>
<div class="site">
  <header class="site-header">
    <img src="assets/images/mirr_logo.svg" alt="MIRR" class="logo">
    <h1>MIRR <span>Documentation</span></h1>
    <button class="menu-toggle" onclick="document.querySelector('.site-nav').classList.toggle('open')">Menu</button>
    <a class="github-link" href="https://github.com/brandonfromph/mirr-project">GitHub &rarr;</a>
  </header>

  {nav}

  <div class="content-wrapper">
{toc_section}  <main class="{content_class}">
{body}
  </main>
  </div>

  <footer class="site-footer">MIRR &mdash; Safety-critical hardware, zero dependencies.</footer>
</div>
<script>
(function() {{
  var path = location.pathname.split('/').pop() || 'index.html';
  var links = document.querySelectorAll('.site-nav a');
  for (var i = 0; i < links.length; i++) {{
    links[i].classList.remove('active');
    if (links[i].getAttribute('href') === path) {{ links[i].classList.add('active'); }}
  }}
}})();
</script>
</body>
</html>
"#,
        title = escape_html(title),
        css = css_path,
        nav = nav_html,
        toc_section = toc_section,
        content_class = content_class,
        body = body_html,
    )
}

/// Post-process rendered HTML to apply MIRR syntax highlighting.
/// Finds `<code class="language-mirr">...</code>` blocks and wraps tokens
/// in CSS class spans.
fn postprocess_mirr_highlight(html: &str) -> String {
    let open_tag = "<code class=\"language-mirr\">";
    let close_tag = "</code>";
    let mut result = String::with_capacity(html.len());
    let mut cursor = 0;

    const MAX_BLOCKS: usize = 200;
    let mut block_count = 0;

    while let Some(start) = html[cursor..].find(open_tag) {
        block_count += 1;
        if block_count > MAX_BLOCKS {
            result.push_str(&html[cursor..]);
            break;
        }

        let abs_start = cursor + start;
        result.push_str(&html[cursor..abs_start]);
        result.push_str(open_tag);

        let content_start = abs_start + open_tag.len();
        let content_end = match html[content_start..].find(close_tag) {
            Some(pos) => content_start + pos,
            None => {
                result.push_str(&html[content_start..]);
                return result;
            }
        };

        let raw_code = &html[content_start..content_end];
        // Unescape HTML entities before highlighting, then re-escape per token
        let code = raw_code
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"");
        result.push_str(&highlight_mirr(&code));
        result.push_str(close_tag);

        cursor = content_end + close_tag.len();
    }

    result.push_str(&html[cursor..]);
    result
}

/// Classify and wrap MIRR source code tokens with CSS classes.
fn highlight_mirr(code: &str) -> String {
    let mut out = String::with_capacity(code.len() * 2);
    let mut token_count = 0;

    for line in code.lines() {
        token_count += 1;
        if token_count > MAX_HIGHLIGHT_TOKENS {
            out.push_str(&escape_html(line));
            out.push('\n');
            continue;
        }

        let trimmed = line.trim_start();
        // Full-line comment
        if trimmed.starts_with("//") {
            let leading = &line[..line.len() - trimmed.len()];
            out.push_str(leading);
            out.push_str("<span class=\"mirr-cmt\">");
            out.push_str(&escape_html(trimmed));
            out.push_str("</span>\n");
            continue;
        }

        // Token-by-token highlighting
        let mut chars = line.chars().peekable();
        while let Some(&ch) = chars.peek() {
            token_count += 1;
            if token_count > MAX_HIGHLIGHT_TOKENS {
                let rest: String = chars.collect();
                out.push_str(&escape_html(&rest));
                break;
            }

            if ch == '/' {
                chars.next();
                if chars.peek() == Some(&'/') {
                    // Inline comment to end of line
                    out.push_str("<span class=\"mirr-cmt\">/");
                    let rest: String = chars.collect();
                    out.push_str(&escape_html(&rest));
                    out.push_str("</span>");
                    break;
                }
                out.push_str("<span class=\"mirr-op\">/</span>");
                continue;
            }

            if ch == '@' {
                // Annotation
                chars.next();
                let mut ann = String::from("@");
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ann.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                out.push_str("<span class=\"mirr-ann\">");
                out.push_str(&escape_html(&ann));
                out.push_str("</span>");
                continue;
            }

            if ch == '#' {
                // Tag
                chars.next();
                let mut tag = String::from("#");
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        tag.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                out.push_str("<span class=\"mirr-tag\">");
                out.push_str(&escape_html(&tag));
                out.push_str("</span>");
                continue;
            }

            if ch.is_ascii_digit() {
                // Number
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit()
                        || c == '_'
                        || c == 'x'
                        || c == 'b'
                        || c == 'o'
                        || (c.is_ascii_hexdigit() && !c.is_ascii_alphabetic())
                    {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                out.push_str("<span class=\"mirr-num\">");
                out.push_str(&escape_html(&num));
                out.push_str("</span>");
                continue;
            }

            if ch.is_alphabetic() || ch == '_' {
                // Identifier or keyword
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let class = classify_mirr_token(&ident);
                out.push_str("<span class=\"");
                out.push_str(class);
                out.push_str("\">");
                out.push_str(&escape_html(&ident));
                out.push_str("</span>");
                continue;
            }

            if is_mirr_operator(ch) {
                chars.next();
                out.push_str("<span class=\"mirr-op\">");
                out.push_str(&escape_html(&ch.to_string()));
                out.push_str("</span>");
                continue;
            }

            // Whitespace / punctuation — pass through
            chars.next();
            out.push(ch);
        }
        out.push('\n');
    }

    // Remove trailing newline added by last iteration
    if out.ends_with('\n') {
        out.pop();
    }

    out
}

fn classify_mirr_token(token: &str) -> &'static str {
    match token {
        "module" | "input" | "output" | "wire" | "reg" | "always" | "assign" | "temporal"
        | "require" | "ensure" | "reflex" | "if" | "else" | "for" | "let" | "fn" | "struct"
        | "enum" | "match" | "return" | "guard" | "signal" | "when" | "on" | "cycles"
        | "property" | "pattern" | "prev" | "use" => "mirr-kw",
        "in" | "out" | "internal" => "mirr-dir",
        "u1" | "u2" | "u3" | "u4" | "u5" | "u6" | "u7" | "u8" | "u9" | "u10" | "u11" | "u12"
        | "u13" | "u14" | "u15" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "bool"
        | "bit" | "clock" | "reset" => "mirr-type",
        "true" | "false" => "mirr-bool",
        _ => "mirr-name",
    }
}

fn is_mirr_operator(ch: char) -> bool {
    matches!(ch, '+' | '-' | '*' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '~' | '%')
}

/// Generate a URL-safe slug from heading text.
/// Lowercase, spaces to hyphens, strip non-alphanumeric (except hyphens).
fn heading_to_slug(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    const MAX_SLUG_CHARS: usize = 200;
    let mut count = 0;
    for ch in text.chars() {
        count += 1;
        if count > MAX_SLUG_CHARS {
            break;
        }
        if ch.is_alphanumeric() {
            for lower in ch.to_lowercase() {
                slug.push(lower);
            }
        } else if (ch == ' ' || ch == '-') && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    // Trim trailing hyphens
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

/// Extract TOC entries (h2 and h3 headings) from Markdown source.
fn extract_toc_entries(body: &str) -> Vec<TocEntry> {
    let mut entries = Vec::new();
    for line in body.lines() {
        if entries.len() >= MAX_TOC_HEADINGS {
            break;
        }
        let trimmed = line.trim_start();
        if let Some(text) = trimmed.strip_prefix("### ") {
            let title = text.trim().to_string();
            let id = heading_to_slug(&title);
            if !id.is_empty() {
                entries.push(TocEntry { level: 3, id, title });
            }
        } else if let Some(text) = trimmed.strip_prefix("## ") {
            let title = text.trim().to_string();
            let id = heading_to_slug(&title);
            if !id.is_empty() {
                entries.push(TocEntry { level: 2, id, title });
            }
        }
    }
    entries
}

/// Build a `<nav class="toc">` HTML block from TOC entries.
fn build_toc_html(entries: &[TocEntry]) -> String {
    let mut html = String::from("    <nav class=\"toc\" aria-label=\"Table of Contents\">\n");
    html.push_str("      <h2 class=\"toc-title\">Contents</h2>\n");
    html.push_str("      <ul>\n");
    for entry in entries {
        let indent = if entry.level == 3 { "          " } else { "        " };
        let class = if entry.level == 3 { " class=\"toc-h3\"" } else { "" };
        html.push_str(&format!(
            "{}<li{}><a href=\"#{}\">{}</a></li>\n",
            indent,
            class,
            escape_html(&entry.id),
            escape_html(&entry.title)
        ));
    }
    html.push_str("      </ul>\n");
    html.push_str("    </nav>");
    html
}

/// Inject `id` attributes into `<h2>` and `<h3>` tags in rendered HTML.
fn inject_heading_ids(html: &str) -> String {
    let mut result = String::with_capacity(html.len() + 512);
    let mut cursor = 0;

    const MAX_HEADING_INJECTIONS: usize = 200;
    let mut injection_count = 0;

    let tags: &[&str] = &["<h2>", "<h3>"];
    let close_tags: &[&str] = &["</h2>", "</h3>"];

    while cursor < html.len() {
        injection_count += 1;
        if injection_count > MAX_HEADING_INJECTIONS {
            result.push_str(&html[cursor..]);
            break;
        }

        // Find the next <h2> or <h3> tag
        let mut best_pos = None;
        let mut best_tag_idx = 0;
        for (idx, tag) in tags.iter().enumerate() {
            if let Some(pos) = html[cursor..].find(tag) {
                let abs_pos = cursor + pos;
                if best_pos.is_none() || abs_pos < best_pos.unwrap_or(usize::MAX) {
                    best_pos = Some(abs_pos);
                    best_tag_idx = idx;
                }
            }
        }

        let tag_pos = match best_pos {
            Some(p) => p,
            None => {
                result.push_str(&html[cursor..]);
                break;
            }
        };

        let open_tag = tags[best_tag_idx];
        let close_tag = close_tags[best_tag_idx];
        let tag_name = &open_tag[1..open_tag.len() - 1]; // "h2" or "h3"

        // Copy everything before this tag
        result.push_str(&html[cursor..tag_pos]);

        // Find closing tag
        let content_start = tag_pos + open_tag.len();
        let content_end = match html[content_start..].find(close_tag) {
            Some(pos) => content_start + pos,
            None => {
                result.push_str(&html[tag_pos..]);
                break;
            }
        };

        // Extract text content (strip any inner HTML tags for slug generation)
        let inner_html = &html[content_start..content_end];
        let text_content = strip_html_tags(inner_html);
        let slug = heading_to_slug(&text_content);

        // Write the tag with id attribute
        result.push_str(&format!("<{} id=\"{}\">", tag_name, escape_html(&slug)));
        result.push_str(inner_html);
        result.push_str(close_tag);

        cursor = content_end + close_tag.len();
    }

    result
}

/// Strip HTML tags from a string, returning only text content.
fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    const MAX_STRIP_CHARS: usize = 100_000;
    let mut count = 0;
    for ch in html.chars() {
        count += 1;
        if count > MAX_STRIP_CHARS {
            break;
        }
        if ch == '<' {
            in_tag = true;
        } else if ch == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(ch);
        }
    }
    result
}

/// Extract the first paragraph's text content from rendered HTML.
/// Returns plain text truncated to MAX_SEARCH_TERM_LENGTH.
fn extract_first_paragraph(html: &str) -> String {
    let open = "<p>";
    let close = "</p>";
    let start = match html.find(open) {
        Some(pos) => pos + open.len(),
        None => return String::new(),
    };
    let end = match html[start..].find(close) {
        Some(pos) => start + pos,
        None => return String::new(),
    };
    let inner = &html[start..end];
    let text = strip_html_tags(inner);
    let trimmed = text.trim();
    if trimmed.len() <= MAX_SEARCH_TERM_LENGTH {
        trimmed.to_string()
    } else {
        // Truncate at a char boundary
        let mut end_idx = MAX_SEARCH_TERM_LENGTH;
        while end_idx > 0 && !trimmed.is_char_boundary(end_idx) {
            end_idx -= 1;
        }
        format!("{}...", &trimmed[..end_idx])
    }
}

/// Escape a string for embedding in a JSON string value.
fn escape_json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    const MAX_JSON_ESCAPE_CHARS: usize = 100_000;
    let mut count = 0;
    for ch in s.chars() {
        count += 1;
        if count > MAX_JSON_ESCAPE_CHARS {
            break;
        }
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                // Control characters
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}

/// Write the search-index.json file to the output directory.
fn write_search_index(output: &Path, entries: &[SearchEntry]) -> std::io::Result<()> {
    let mut json = String::from("[\n");
    let count = entries.len().min(MAX_SEARCH_INDEX_ENTRIES);
    for (i, entry) in entries.iter().enumerate().take(count) {
        json.push_str("  {\n");
        json.push_str(&format!("    \"title\": \"{}\",\n", escape_json_string(&entry.title)));
        json.push_str(&format!("    \"url\": \"{}\",\n", escape_json_string(&entry.url)));

        // Headings array
        json.push_str("    \"headings\": [");
        let max_headings = entry.headings.len().min(MAX_TOC_HEADINGS);
        for (j, heading) in entry.headings.iter().enumerate().take(max_headings) {
            if j > 0 {
                json.push_str(", ");
            }
            json.push('"');
            json.push_str(&escape_json_string(heading));
            json.push('"');
        }
        json.push_str("],\n");

        json.push_str(&format!("    \"snippet\": \"{}\"\n", escape_json_string(&entry.snippet)));
        json.push_str("  }");
        if i + 1 < count {
            json.push(',');
        }
        json.push('\n');
    }
    json.push(']');
    fs::write(output.join("search-index.json"), json)
}

/// Iteratively copy a directory tree.
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    const MAX_DEPTH: usize = 16;
    const MAX_TOTAL_ENTRIES: usize = 10_000;

    let mut stack: Vec<(PathBuf, PathBuf)> = vec![(src.to_path_buf(), dst.to_path_buf())];
    let mut total: usize = 0;

    while let Some((src_dir, dst_dir)) = stack.pop() {
        fs::create_dir_all(&dst_dir)?;

        let depth = match dst_dir.strip_prefix(dst) {
            Ok(rel) => rel.components().count(),
            Err(_) => 0,
        };
        if depth > MAX_DEPTH {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "directory nesting too deep",
            ));
        }

        for entry in fs::read_dir(&src_dir)? {
            total += 1;
            if total > MAX_TOTAL_ENTRIES {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "too many directory entries",
                ));
            }

            let entry = entry?;
            let ty = entry.file_type()?;
            let dst_path = dst_dir.join(entry.file_name());

            if ty.is_dir() {
                stack.push((entry.path(), dst_path));
            } else {
                fs::copy(entry.path(), dst_path)?;
            }
        }
    }
    Ok(())
}
