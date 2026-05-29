// build.rs — MIRR compiler build script
//
// Responsibilities:
//   1. Warn on raw `[Ennn]` string literals in production source.
//      (Phase 4: flip to `error!` to make the build fail.)
//   2. Verify that `src/error_codes.rs` exists (the typed registry).
//   3. Print Cargo rerun triggers.
//
// Raw-string detection heuristic:
//   Any `"[E` followed by digits and `]"` in a `.rs` file under `src/`
//   that is NOT inside a `#[cfg(test)]` block and NOT in `error_codes.rs`
//   or `build.rs` itself is flagged.
//
// NOTE: When Phase 4 is ready, change `cargo:warning=` to
//   `eprintln!("cargo:error=...")` and `std::process::exit(1)`.

use std::fs;
use std::path::Path;

fn main() {
    // Rerun only when source changes.
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");

    // 1. Ensure the typed registry exists.
    if !Path::new("src/error_codes.rs").exists() {
        eprintln!(
            "cargo:error=src/error_codes.rs is missing — the typed ErrorCode registry is required."
        );
        std::process::exit(1);
    }

    // 2. Scan for raw [Ennn] string literals in production code.
    scan_raw_error_codes("src");
}

/// Walk every `.rs` file under `root`, skip test-only files and the
/// registry itself, and warn on any raw `[Ennn]` string literal found.
fn scan_raw_error_codes(root: &str) {
    let skip_files =
        ["error_codes.rs", "build.rs", "diagnostic_builder.rs", "error.rs", "mape_k/error.rs"];

    walk_rs(root, &skip_files, &mut |path: &Path, content: &str| {
        // Skip test modules: crude but effective for our codebase.
        // We look for `#[cfg(test)]` in the file; if found we only check
        // lines that appear before that marker.
        let test_boundary = content.find("#[cfg(test)]").unwrap_or(content.len());
        let production = &content[..test_boundary];

        scan_for_raw_codes(path, production);
    });
}

fn scan_for_raw_codes(path: &Path, content: &str) {
    // Pattern: `"[E` followed by 1–4 digits and `]`
    // We scan byte-by-byte (bounded per file).
    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;

    // NASA Power-of-10: bounded loop.
    while i + 4 < len {
        // Look for the sequence: `"[E`
        if bytes[i] == b'"' && bytes[i + 1] == b'[' && bytes[i + 2] == b'E' {
            // Count digits after E.
            let mut j = i + 3;
            while j < len.min(i + 8) && bytes[j].is_ascii_digit() {
                j += 1;
            }
            let digit_count = j - (i + 3);
            if digit_count >= 1 && j < len && bytes[j] == b']' {
                // Found a raw [Ennn] literal.
                let line_no = content[..i].chars().filter(|&c| c == '\n').count() + 1;
                let code_slice = &content[i + 1..j + 1]; // "[E123]"

                // Phase 4: Hard gate enabled.
                eprintln!(
                    "cargo:error=RAW-ERROR-CODE: {}:{} — use ErrorCode enum instead of raw string {}",
                    path.display(),
                    line_no,
                    code_slice,
                );
                std::process::exit(1);
            }
        }
        i += 1;
    }
}

/// Recursively walk `.rs` files under `root`, skipping files whose name
/// matches any entry in `skip_files`.
fn walk_rs(root: &str, skip_files: &[&str], cb: &mut dyn FnMut(&Path, &str)) {
    let dir = match fs::read_dir(root) {
        Ok(d) => d,
        Err(_) => return,
    };

    // NASA Power-of-10: bounded iteration (max 4096 entries per dir).
    let mut count = 0usize;
    for entry in dir {
        count += 1;
        if count > 4096 {
            break;
        }
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();

        if path.is_dir() {
            if let Some(s) = path.to_str() {
                walk_rs(s, skip_files, cb);
            }
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if skip_files.contains(&file_name) {
                continue;
            }
            if let Ok(content) = fs::read_to_string(&path) {
                cb(&path, &content);
            }
        }
    }
}
