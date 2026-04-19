#![forbid(unsafe_code)]

use sha2::{Digest, Sha256};
use std::path::Path;

use crate::util::MAX_FILE_SIZE;

fn to_hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

/// Compute SHA-256 of a file. Returns 64-char lowercase hex string.
/// Bounded: refuses files larger than MAX_FILE_SIZE.
pub fn sha256_file(path: &Path) -> Result<String, String> {
    let meta =
        std::fs::metadata(path).map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;
    if meta.len() as usize > MAX_FILE_SIZE {
        return Err(format!("{} exceeds {} bytes", path.display(), MAX_FILE_SIZE));
    }
    let bytes =
        std::fs::read(path).map_err(|e| format!("Cannot read {}: {}", path.display(), e))?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let result = hasher.finalize();
    Ok(to_hex_lower(result.as_ref()))
}

/// Public entry point. Prints the SHA-256 hash of the given file values. Returns exit code.
pub fn run(path: &str) -> i32 {
    match sha256_file(Path::new(path)) {
        Ok(hash) => {
            println!("sha256:{}", hash);
            0
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    }
}
