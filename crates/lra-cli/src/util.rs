#![forbid(unsafe_code)]

use std::path::Path;

/// Maximum file size for read_to_string calls (10 MB, NASA Power-of-10 bound).
pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

/// Read a file to string only if its size is within MAX_FILE_SIZE.
/// Returns empty string on any error or if the file exceeds the bound.
pub fn bounded_read_to_string(path: &Path) -> String {
    match std::fs::metadata(path) {
        Ok(meta) => {
            if meta.len() as usize > MAX_FILE_SIZE {
                eprintln!(
                    "Warning: {} exceeds {} bytes, skipping read",
                    path.display(),
                    MAX_FILE_SIZE
                );
                return String::new();
            }
        }
        Err(_) => return String::new(),
    }
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Parsed semantic version (major.minor.patch).
/// NASA Power-of-10: no heap allocation beyond the input string scan.
pub struct Semver {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

/// Maximum length for a version string (NASA Power-of-10 bound).
const MAX_VERSION_LEN: usize = 64;

/// Maximum digits per version component (guards against overflow).
const MAX_DIGITS: usize = 10;

/// Parse a semver string "X.Y.Z" into components. Returns None on invalid input.
pub fn parse_semver(s: &str) -> Option<Semver> {
    if s.len() > MAX_VERSION_LEN {
        return None;
    }
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut parts = [0u32; 3];
    let mut part_idx = 0;
    let mut digit_count = 0;
    let mut i = 0;
    while i < len && part_idx < 3 {
        let b = bytes[i];
        if b == b'.' {
            if digit_count == 0 {
                return None;
            }
            part_idx += 1;
            digit_count = 0;
        } else if b.is_ascii_digit() {
            if digit_count >= MAX_DIGITS {
                return None;
            }
            parts[part_idx] = parts[part_idx].checked_mul(10)?.checked_add((b - b'0') as u32)?;
            digit_count += 1;
        } else {
            return None;
        }
        i += 1;
    }
    if part_idx != 2 || digit_count == 0 {
        return None;
    }
    Some(Semver { major: parts[0], minor: parts[1], patch: parts[2] })
}
