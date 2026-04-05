#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_text(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {}: {}", path.display(), e))
}

fn list_file_names_with_ext(dir: &Path, ext: &str) -> Vec<String> {
    let mut out = Vec::new();
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("failed to read directory {}: {}", dir.display(), e));

    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("failed to read directory entry: {}", e));
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(ext) {
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_else(|| panic!("invalid utf-8 file name in {}", dir.display()));
            out.push(name.to_string());
        }
    }

    out.sort();
    out
}

fn discover_fuzz_targets(root: &Path) -> Vec<String> {
    let mut targets = Vec::new();
    let target_dir = root.join("fuzz").join("fuzz_targets");
    let files = list_file_names_with_ext(&target_dir, "rs");

    for file in files {
        let stem = file
            .strip_suffix(".rs")
            .unwrap_or_else(|| panic!("unexpected fuzz target filename: {}", file));
        targets.push(stem.to_string());
    }

    targets.sort();
    targets
}

fn parse_workspace_fuzz_targets(workspace_cargo: &str) -> BTreeSet<String> {
    const MAX_METADATA_LINES: usize = 2048;
    let mut targets = BTreeSet::new();
    let mut in_section = false;
    let mut in_targets = false;

    for line in workspace_cargo.lines().take(MAX_METADATA_LINES) {
        let trimmed = line.trim();

        if trimmed.starts_with('[') {
            if trimmed == "[workspace.metadata.mirr.fuzz]" {
                in_section = true;
                in_targets = false;
                continue;
            }

            if in_section {
                break;
            }
        }

        if !in_section {
            continue;
        }

        if !in_targets {
            if let Some(pos) = trimmed.find("targets") {
                if pos == 0 && trimmed.contains('[') {
                    in_targets = true;
                    let start = trimmed.find('[').unwrap_or(0);
                    parse_quoted_targets(&trimmed[start + 1..], &mut targets);
                    if trimmed.contains(']') {
                        break;
                    }
                }
            }
            continue;
        }

        parse_quoted_targets(trimmed, &mut targets);
        if trimmed.contains(']') {
            break;
        }
    }

    targets
}

fn parse_quoted_targets(fragment: &str, out: &mut BTreeSet<String>) {
    let bytes = fragment.as_bytes();
    let mut i = 0usize;

    while i < bytes.len() {
        if bytes[i] != b'"' {
            i += 1;
            continue;
        }

        i += 1;
        let start = i;
        while i < bytes.len() && bytes[i] != b'"' {
            i += 1;
        }

        if i > start && i <= bytes.len() {
            out.insert(fragment[start..i].to_string());
        }

        if i < bytes.len() {
            i += 1;
        }
    }
}

#[test]
fn language_dfa_has_no_admitted_proofs() {
    let root = repo_root();
    let dfa = read_text(&root.join("proofs").join("language").join("DFA.v"));
    assert!(!dfa.contains("Admitted."), "proofs/language/DFA.v must not contain Admitted.");
}

#[test]
fn language_makefile_manifest_includes_all_language_theorems() {
    let root = repo_root();
    let language_dir = root.join("proofs").join("language");
    let makefile = read_text(&language_dir.join("Makefile"));
    let theorem_files = list_file_names_with_ext(&language_dir, "v");

    assert!(makefile.contains("VFILES :="), "proofs/language/Makefile must define VFILES");

    for theorem in theorem_files {
        assert!(
            makefile.contains(&theorem),
            "proofs/language/Makefile VFILES must include {}",
            theorem
        );
    }
}

#[test]
fn fuzz_matrix_entries_match_targets_and_seed_health_policy() {
    let root = repo_root();
    let targets = discover_fuzz_targets(&root);
    assert!(!targets.is_empty(), "fuzz/fuzz_targets must contain at least one target");

    let fuzz_cargo = read_text(&root.join("fuzz").join("Cargo.toml"));
    let corpus_root = root.join("fuzz").join("corpus");

    for target in &targets {
        let cargo_marker = format!("name = \"{}\"", target);
        assert!(
            fuzz_cargo.contains(&cargo_marker),
            "fuzz/Cargo.toml must declare target {}",
            target
        );

        let corpus_dir = corpus_root.join(target);
        assert!(
            corpus_dir.is_dir(),
            "fuzz corpus directory missing for target {} at {}",
            target,
            corpus_dir.display()
        );

        let mut seed_count = 0usize;
        let entries = fs::read_dir(&corpus_dir)
            .unwrap_or_else(|e| panic!("failed to read corpus {}: {}", corpus_dir.display(), e));
        for entry in entries {
            let entry = entry.unwrap_or_else(|e| panic!("failed to read seed entry: {}", e));
            if entry.path().is_file() {
                seed_count += 1;
            }
        }

        assert!(
            seed_count >= 1,
            "seed-health policy requires at least one seed file for {}",
            target
        );
    }

    let target_set: BTreeSet<String> = targets.into_iter().collect();
    let corpus_dirs = fs::read_dir(&corpus_root)
        .unwrap_or_else(|e| panic!("failed to read corpus root {}: {}", corpus_root.display(), e));
    for entry in corpus_dirs {
        let entry = entry.unwrap_or_else(|e| panic!("failed to read corpus root entry: {}", e));
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_else(|| {
                panic!("invalid utf-8 corpus directory name: {}", path.display())
            });
            assert!(
                target_set.contains(dir_name),
                "corpus directory {} does not map to a declared fuzz target",
                dir_name
            );
        }
    }
}

#[test]
fn workspace_fuzz_governance_metadata_matches_discovered_targets() {
    let root = repo_root();
    let workspace_cargo = read_text(&root.join("Cargo.toml"));
    let discovered_targets: BTreeSet<String> = discover_fuzz_targets(&root).into_iter().collect();
    let metadata_targets = parse_workspace_fuzz_targets(&workspace_cargo);

    assert!(
        workspace_cargo.contains("[workspace.metadata.mirr.fuzz]"),
        "workspace Cargo.toml must define [workspace.metadata.mirr.fuzz]"
    );
    assert!(
        workspace_cargo.contains("required_seed_files_per_target = 1"),
        "workspace fuzz governance must pin required_seed_files_per_target = 1"
    );

    assert_eq!(
        metadata_targets, discovered_targets,
        "workspace fuzz metadata targets must exactly match discovered fuzz target set"
    );
}
