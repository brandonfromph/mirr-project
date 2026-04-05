#![forbid(unsafe_code)]

use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageManifest {
    pub members: Vec<String>,
}

pub fn load_package_manifest(path: &Path) -> io::Result<PackageManifest> {
    let content = fs::read_to_string(path)?;
    let mut members = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        members.push(trimmed.to_string());
    }

    Ok(PackageManifest { members })
}
