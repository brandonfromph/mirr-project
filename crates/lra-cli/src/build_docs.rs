#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};

use pulldown_cmark::{html, Options, Parser};

const MAX_INPUT_SIZE: usize = 2 * 1024 * 1024;
const MAX_FILES: usize = 200;

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
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
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
        let (title, nav_order) = extract_meta(&source);
        let (_, html_name) = match html_filename(&path) {
            Some(pair) => pair,
            None => continue,
        };
        nav_entries.push(NavEntry { title, html_name, nav_order });
    }
    nav_entries.sort_by_key(|e| e.nav_order);

    // Only show files with explicit nav_order in the nav (exclude legacy/hidden files)
    let visible_nav: Vec<&NavEntry> = nav_entries.iter().filter(|e| e.nav_order < 900).collect();

    // Build the nav HTML once
    let nav_html = build_nav(&visible_nav);

    // Second pass: render each file
    let mut built = 0;
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

        let (title, _) = extract_meta(&source);
        let body = strip_frontmatter(&source);
        let body_html = render_markdown(body);

        let (_, html_name) = match html_filename(&path) {
            Some(pair) => pair,
            None => continue,
        };

        let page_html = assemble_page(&title, &html_name, &nav_html, &body_html, css_path);

        let out_path = output.join(&html_name);
        if let Err(e) = fs::write(&out_path, &page_html) {
            eprintln!("Error writing {}: {}", out_path.display(), e);
            return 1;
        }
        built += 1;
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

/// Compute the HTML filename from a Markdown source path.
/// Returns `(stem, html_name)` or `None` if the path has no file stem.
fn html_filename(path: &Path) -> Option<(String, String)> {
    let stem = path.file_stem()?.to_string_lossy().to_string();
    let html_name =
        if stem == "home" { "index.html".to_string() } else { format!("{}.html", stem) };
    Some((stem, html_name))
}

/// Extract title and nav_order from YAML frontmatter.
fn extract_meta(source: &str) -> (String, i32) {
    let mut title = String::new();
    let mut nav_order = 999;

    let trimmed = source.trim_start();
    if !trimmed.starts_with("---") {
        // No frontmatter — use first heading as title
        for line in source.lines().take(20) {
            if let Some(h) = line.strip_prefix("# ") {
                title = h.trim().to_string();
                break;
            }
        }
        return (title, nav_order);
    }

    let after = &trimmed[3..];
    let closing = match after.find("\n---") {
        Some(c) => c,
        None => return (title, nav_order),
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
        }
    }

    (title, nav_order)
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
    postprocess_callouts(&html_output)
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
    css_path: &str,
) -> String {
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

  <main class="site-content">
{body}
  </main>

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
        body = body_html,
    )
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
