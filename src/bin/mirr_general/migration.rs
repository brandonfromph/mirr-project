#![forbid(unsafe_code)]

use std::io;
use std::path::{Path, PathBuf};
use std::{fs, vec};

#[cfg(not(test))]
use std::process::Command;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyScriptSpec {
    pub id: String,
    pub path: PathBuf,
    pub replacement_subcommand: String,
    pub parity_test_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationRecord {
    pub id: String,
    pub deleted: bool,
    pub detail: String,
}

fn parity_probe_succeeds(parity_test_name: &str) -> io::Result<bool> {
    #[cfg(test)]
    {
        let _ = parity_test_name;
        // Integration tests include this module directly; avoid recursive cargo
        // invocations that can deadlock on workspace file locks.
        Ok(false)
    }

    #[cfg(not(test))]
    {
        let status = Command::new("cargo").args(["nextest", "run", parity_test_name]).status()?;
        Ok(status.success())
    }
}

pub fn migrate_script(spec: &LegacyScriptSpec) -> io::Result<MigrationRecord> {
    if !spec.path.exists() {
        return Ok(MigrationRecord {
            id: spec.id.clone(),
            deleted: false,
            detail: "script already absent".to_string(),
        });
    }

    if !parity_probe_succeeds(&spec.parity_test_name)? {
        return Ok(MigrationRecord {
            id: spec.id.clone(),
            deleted: false,
            detail: format!("parity test failed: {}", spec.parity_test_name),
        });
    }

    fs::remove_file(&spec.path)?;

    Ok(MigrationRecord {
        id: spec.id.clone(),
        deleted: true,
        detail: format!("deleted and replaced by '{}'", spec.replacement_subcommand),
    })
}

pub fn build_script_inventory(repo_root: &Path) -> Vec<LegacyScriptSpec> {
    let entries = [
        ("SP001", "scripts/ci-local.sh", "run ci", "orchestrator_migration_tests::sp001"),
        (
            "SP002",
            "scripts/ci-local-fast.sh",
            "run ci --profile fast",
            "orchestrator_migration_tests::sp002",
        ),
        ("SP003", "run_wave_gates.ps1", "run ci", "orchestrator_migration_tests::sp003"),
        ("SP004", "run_wave_gates.sh", "run ci", "orchestrator_migration_tests::sp004"),
        ("SP005", "run_wave_gates.bat", "run ci", "orchestrator_migration_tests::sp005"),
        ("SP006", "execute-wave-gates.ps1", "run ci", "orchestrator_migration_tests::sp006"),
        ("SP007", "execute_all_gates.sh", "run ci", "orchestrator_migration_tests::sp007"),
        ("SP008", "run_critical_gates.sh", "run ci", "orchestrator_migration_tests::sp008"),
        ("SP009", "phase1_baseline.ps1", "run ci", "orchestrator_migration_tests::sp009"),
        ("SP010", "phase4_ci_steps.ps1", "run ci", "orchestrator_migration_tests::sp010"),
        ("SP011", "phase6_regression.ps1", "run ci", "orchestrator_migration_tests::sp011"),
        ("SP012", "test_gates.ps1", "run ci", "orchestrator_migration_tests::sp012"),
        ("SP013", "tests/eda/run_eda_tests.sh", "run ci", "orchestrator_migration_tests::sp013"),
        ("SP014", "tests/sim/run_sim.sh", "run ci", "orchestrator_migration_tests::sp014"),
        (
            "SP015",
            "crates/mirr-wasm/build.sh",
            "run ci --profile compile",
            "orchestrator_migration_tests::sp015",
        ),
        ("SP016", "build_selfhost.ps1", "run ci", "orchestrator_migration_tests::sp016"),
        ("SP017", "build_selfhost.sh", "run ci", "orchestrator_migration_tests::sp017"),
        ("SP018", "build_width.sh", "run ci", "orchestrator_migration_tests::sp018"),
        ("SP019", "build_proofs.bat", "run ci", "orchestrator_migration_tests::sp019"),
        ("SP020", "coqc.bat", "run ci", "orchestrator_migration_tests::sp020"),
        ("SP021", "run_coq.sh", "run ci", "orchestrator_migration_tests::sp021"),
        (
            "SP022",
            "run-mirr.ps1",
            "run ci --profile compile",
            "orchestrator_migration_tests::sp022",
        ),
        ("SP023", "scripts/repo_metrics.py", "run inspect", "orchestrator_migration_tests::sp023"),
        (
            "SP024",
            "scripts/review_coverage_gate.py",
            "run inspect",
            "orchestrator_migration_tests::sp024",
        ),
        (
            "SP025",
            "scripts/validate_proposals.py",
            "run inspect",
            "orchestrator_migration_tests::sp025",
        ),
        (
            "SP026",
            "scripts/run_upgraded_096_review.ps1",
            "run inspect",
            "orchestrator_migration_tests::sp026",
        ),
        ("SP027", "execute_p096.ps1", "run ci", "orchestrator_migration_tests::sp027"),
        ("SP028", "execute_p096.bat", "run ci", "orchestrator_migration_tests::sp028"),
        ("SP029", "mklink_coq.bat", "run ci", "orchestrator_migration_tests::sp029"),
        ("SP030", "do_split.py", "run inspect", "orchestrator_migration_tests::sp030"),
        ("SP031", "fix_unfold.py", "run ci", "orchestrator_migration_tests::sp031"),
        ("SP032", "fix_validate_module.py", "run inspect", "orchestrator_migration_tests::sp032"),
        ("SP033", "fix_rename.py", "run ci", "orchestrator_migration_tests::sp033"),
        ("SP034", "fix_scoping.py", "run ci", "orchestrator_migration_tests::sp034"),
        ("SP035", "fix_scoping2.py", "run ci", "orchestrator_migration_tests::sp035"),
        ("SP036", "fix_all.py", "run ci", "orchestrator_migration_tests::sp036"),
        ("SP037", "test.py", "run inspect", "orchestrator_migration_tests::sp037"),
        ("SP038", "test_write.py", "run inspect", "orchestrator_migration_tests::sp038"),
        ("SP039", "scripts/git-push.sh", "run ci", "orchestrator_migration_tests::sp039"),
    ];

    let mut specs = vec![];
    for (id, rel, replacement, parity_test_name) in entries {
        specs.push(LegacyScriptSpec {
            id: id.to_string(),
            path: repo_root.join(rel),
            replacement_subcommand: replacement.to_string(),
            parity_test_name: parity_test_name.to_string(),
        });
    }

    specs
}
