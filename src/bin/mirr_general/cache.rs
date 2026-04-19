#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

fn to_hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageSpec {
    pub name: String,
    pub source_files: Vec<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CacheEntry {
    pub package_name: String,
    pub fingerprint_hex: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CacheManifest {
    pub entries: BTreeMap<String, CacheEntry>,
}

pub fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let digest = hasher.finalize();
    Ok(to_hex_lower(digest.as_ref()))
}

pub fn compute_package_fingerprint(
    pkg: &PackageSpec,
    lock_hash: &str,
    rustc_version: &str,
) -> io::Result<String> {
    let mut path_hash_pairs: Vec<(String, String)> = Vec::new();

    for source_path in &pkg.source_files {
        let file_hash = hash_file(source_path)?;
        let key = source_path.to_string_lossy().to_string();
        path_hash_pairs.push((key, file_hash));
    }

    path_hash_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    let mut hasher = Sha256::new();
    for (path, file_hash) in path_hash_pairs {
        hasher.update(path.as_bytes());
        hasher.update([0]);
        hasher.update(file_hash.as_bytes());
        hasher.update([0]);
    }
    hasher.update(lock_hash.as_bytes());
    hasher.update([0]);
    hasher.update(rustc_version.as_bytes());

    let digest = hasher.finalize();
    Ok(to_hex_lower(digest.as_ref()))
}

pub fn load_manifest(path: &Path) -> io::Result<CacheManifest> {
    let text = fs::read_to_string(path)?;
    let mut entries = BTreeMap::new();

    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let (package_name, fingerprint_hex) = line.split_once('=').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid manifest line {}: '{}'", line_index + 1, line),
            )
        })?;

        if package_name.is_empty() || fingerprint_hex.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid manifest line {}: '{}'", line_index + 1, line),
            ));
        }

        entries.insert(
            package_name.to_string(),
            CacheEntry {
                package_name: package_name.to_string(),
                fingerprint_hex: fingerprint_hex.to_string(),
            },
        );
    }

    Ok(CacheManifest { entries })
}

pub fn should_skip_package(
    manifest: &CacheManifest,
    package_name: &str,
    fingerprint: &str,
) -> bool {
    match manifest.entries.get(package_name) {
        Some(existing) => existing.fingerprint_hex == fingerprint,
        None => false,
    }
}

pub fn upsert_manifest_entry(manifest: &mut CacheManifest, package_name: &str, fingerprint: &str) {
    manifest.entries.insert(
        package_name.to_string(),
        CacheEntry {
            package_name: package_name.to_string(),
            fingerprint_hex: fingerprint.to_string(),
        },
    );
}

pub fn write_manifest_atomic(path: &Path, manifest: &CacheManifest) -> io::Result<()> {
    let parent = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "manifest path must have a parent")
    })?;
    fs::create_dir_all(parent)?;
    let tmp_path = parent.join("cache.manifest.tmp");

    {
        let mut file = File::create(&tmp_path)?;
        for entry in manifest.entries.values() {
            writeln!(file, "{}={}", entry.package_name, entry.fingerprint_hex)?;
        }
        file.flush()?;
    }

    fs::rename(&tmp_path, path)?;
    Ok(())
}
