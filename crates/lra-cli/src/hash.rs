#![forbid(unsafe_code)]

use sha2::{Digest, Sha256};
use std::path::Path;

use crate::util::MAX_FILE_SIZE;

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
    Ok(format!("{:x}", result))
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
