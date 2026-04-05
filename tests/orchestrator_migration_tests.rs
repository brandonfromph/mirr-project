#![forbid(unsafe_code)]

#[path = "../src/bin/mirr_general/migration.rs"]
mod migration;

use migration::{build_script_inventory, migrate_script, LegacyScriptSpec};
use std::fs;
use std::io;
use std::path::PathBuf;

fn write_script(path: &PathBuf) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, b"echo legacy script\n")?;
    Ok(())
}

#[test]
fn migrate_script_does_not_delete_when_parity_test_name_missing() -> io::Result<()> {
    let root = std::env::temp_dir().join("orchestrator_migration_missing_test_1");
    let script_path = root.join("scripts/ci-local.sh");
    write_script(&script_path)?;

    let spec = LegacyScriptSpec {
        id: "SP001".to_string(),
        path: script_path.clone(),
        replacement_subcommand: "run ci".to_string(),
        parity_test_name: "definitely_missing_nextest_test_name_xyz".to_string(),
    };

    let result = migrate_script(&spec)?;
    assert!(!result.deleted);
    assert!(script_path.exists());
    Ok(())
}

#[test]
fn migrate_script_returns_script_already_absent_when_path_missing() -> io::Result<()> {
    let root = std::env::temp_dir().join("orchestrator_migration_absent_2");
    let script_path = root.join("scripts/run_wave_gates.sh");
    if script_path.exists() {
        fs::remove_file(&script_path)?;
    }

    let spec = LegacyScriptSpec {
        id: "SP004".to_string(),
        path: script_path,
        replacement_subcommand: "run ci".to_string(),
        parity_test_name: "unused_when_missing".to_string(),
    };

    let result = migrate_script(&spec)?;
    assert!(!result.deleted);
    assert_eq!(result.detail, "script already absent");
    Ok(())
}

#[test]
fn migrate_script_record_preserves_id_verbatim() -> io::Result<()> {
    let root = std::env::temp_dir().join("orchestrator_migration_id_3");
    let script_path = root.join("scripts/test_gates.ps1");
    write_script(&script_path)?;

    let spec = LegacyScriptSpec {
        id: "SP012".to_string(),
        path: script_path,
        replacement_subcommand: "run ci --phase tests".to_string(),
        parity_test_name: "missing_parity_name_123".to_string(),
    };

    let result = migrate_script(&spec)?;
    assert_eq!(result.id, "SP012");
    Ok(())
}

#[test]
fn migrate_script_never_deletes_without_proof_of_parity() -> io::Result<()> {
    let root = std::env::temp_dir().join("orchestrator_migration_no_delete_4");
    let script_path = root.join("scripts/execute_all_gates.sh");
    write_script(&script_path)?;

    let spec = LegacyScriptSpec {
        id: "SP007".to_string(),
        path: script_path.clone(),
        replacement_subcommand: "run ci".to_string(),
        parity_test_name: "missing_parity_name_456".to_string(),
    };

    let result = migrate_script(&spec)?;
    assert!(!result.deleted);
    assert!(script_path.exists());
    Ok(())
}

#[test]
fn migrate_script_detail_contains_parity_test_failed_on_nextest_failure() -> io::Result<()> {
    let root = std::env::temp_dir().join("orchestrator_migration_detail_5");
    let script_path = root.join("scripts/phase1_baseline.ps1");
    write_script(&script_path)?;

    let spec = LegacyScriptSpec {
        id: "SP009".to_string(),
        path: script_path,
        replacement_subcommand: "run ci --phase baseline".to_string(),
        parity_test_name: "missing_parity_name_789".to_string(),
    };

    let result = migrate_script(&spec)?;
    assert!(!result.deleted);
    assert!(result.detail.contains("parity test failed"));
    Ok(())
}

#[test]
fn build_script_inventory_returns_non_empty_with_sp001_id() {
    let repo_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let inventory = build_script_inventory(&repo_root);
    assert!(!inventory.is_empty());
    assert_eq!(inventory[0].id, "SP001");
}
