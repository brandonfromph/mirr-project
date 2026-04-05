use std::collections::BTreeMap;
use std::fs;
use std::io;

use tempfile::tempdir;

#[path = "../src/bin/mirr_general/cache.rs"]
mod cache;

use cache::{
    compute_package_fingerprint, load_manifest, should_skip_package, upsert_manifest_entry,
    write_manifest_atomic, CacheEntry, CacheManifest, PackageSpec,
};

#[test]
fn orchestrator_cache_tests_fingerprint_changes_when_source_file_content_changes() -> io::Result<()>
{
    let temp = tempdir()?;
    let source_path = temp.path().join("pkg1.rs");

    fs::write(&source_path, "alpha")?;
    let pkg = PackageSpec { name: "pkg1".to_string(), source_files: vec![source_path.clone()] };

    let first = compute_package_fingerprint(&pkg, "lockhash", "rustc 1.80.0")?;

    fs::write(&source_path, "beta")?;
    let second = compute_package_fingerprint(&pkg, "lockhash", "rustc 1.80.0")?;

    assert_ne!(first, second);
    Ok(())
}

#[test]
fn orchestrator_cache_tests_fingerprint_changes_when_rustc_version_changes() -> io::Result<()> {
    let temp = tempdir()?;
    let source_path = temp.path().join("pkg2.rs");
    fs::write(&source_path, "stable content")?;

    let pkg = PackageSpec { name: "pkg2".to_string(), source_files: vec![source_path] };

    let first = compute_package_fingerprint(&pkg, "lockhash", "rustc 1.80.0")?;
    let second = compute_package_fingerprint(&pkg, "lockhash", "rustc 1.81.0")?;

    assert_ne!(first, second);
    Ok(())
}

#[test]
fn orchestrator_cache_tests_should_skip_returns_true_on_matching_fingerprint() {
    let mut entries = BTreeMap::new();
    entries.insert(
        "pkg3".to_string(),
        CacheEntry { package_name: "pkg3".to_string(), fingerprint_hex: "abc123".to_string() },
    );
    let manifest = CacheManifest { entries };

    assert!(should_skip_package(&manifest, "pkg3", "abc123"));
}

#[test]
fn orchestrator_cache_tests_should_skip_returns_false_on_mismatched_fingerprint() {
    let mut entries = BTreeMap::new();
    entries.insert(
        "pkg4".to_string(),
        CacheEntry { package_name: "pkg4".to_string(), fingerprint_hex: "abc123".to_string() },
    );
    let manifest = CacheManifest { entries };

    assert!(!should_skip_package(&manifest, "pkg4", "different"));
}

#[test]
fn orchestrator_cache_tests_write_manifest_atomic_and_load_manifest_round_trip_produces_identical_entries(
) -> io::Result<()> {
    let temp = tempdir()?;
    let manifest_path = temp.path().join("cache.manifest");

    let entries = BTreeMap::new();
    let mut manifest = CacheManifest { entries };
    upsert_manifest_entry(&mut manifest, "pkgA", "fingerprintA");
    upsert_manifest_entry(&mut manifest, "pkgB", "fingerprintB");

    write_manifest_atomic(&manifest_path, &manifest)?;
    let loaded = load_manifest(&manifest_path)?;

    assert_eq!(manifest.entries, loaded.entries);
    Ok(())
}

#[test]
fn orchestrator_cache_tests_load_manifest_returns_err_on_invalid_line_format() -> io::Result<()> {
    let temp = tempdir()?;
    let manifest_path = temp.path().join("cache.manifest");
    fs::write(&manifest_path, "valid=ok\ninvalid_line_without_equals\n")?;

    let loaded = load_manifest(&manifest_path);
    assert!(loaded.is_err());
    Ok(())
}

#[test]
fn write_manifest_creates_parent_directory_if_missing() -> io::Result<()> {
    let root = std::env::temp_dir().join("mirr_cache_parent_create").join("deeply").join("nested");
    let path = root.join("cache.manifest");
    let mut manifest = CacheManifest::default();
    upsert_manifest_entry(&mut manifest, "workspace", "abc");
    write_manifest_atomic(&path, &manifest)?;
    assert!(path.exists());
    Ok(())
}
