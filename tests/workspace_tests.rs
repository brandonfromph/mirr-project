#![forbid(unsafe_code)]

use mirrc::pipeline::PipelineConfig;
use mirrc::workspace::{Workspace, WorkspaceDependencyGraph, WorkspaceError};
use std::fs;
use tempfile::TempDir;

fn basic_config() -> PipelineConfig {
    PipelineConfig { temporal: false, rspu: false, mape_k: false, ..PipelineConfig::default() }
}

#[test]
fn test_workspace_topological_sort_and_cycle() {
    let mut graph = WorkspaceDependencyGraph::new();
    let file_a = std::path::PathBuf::from("a.mirr");
    let file_b = std::path::PathBuf::from("b.mirr");
    let file_c = std::path::PathBuf::from("c.mirr");

    // A -> B -> C
    graph.add_dependency(file_a.clone(), file_b.clone());
    graph.add_dependency(file_b.clone(), file_c.clone());

    assert_eq!(graph.dependency_count(), 2);
    let all = graph.all_files();
    assert_eq!(all.len(), 3);

    let sorted = graph.topological_sort().unwrap();
    // C should come before B, and B before A (or similar valid sort)
    // Actually the visit logic pushes children first, so dependencies appear before their parents
    assert!(sorted.contains(&file_a));
    assert!(sorted.contains(&file_b));
    assert!(sorted.contains(&file_c));

    // Now introduce a cycle: C -> A
    graph.add_dependency(file_c.clone(), file_a.clone());
    let cycle_result = graph.topological_sort();
    assert!(cycle_result.is_err());
}

#[test]
fn test_workspace_missing_import() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("main.mirr");
    fs::write(&root, "import \"missing.mirr\" as m;\nmodule main { }").unwrap();

    let mut workspace = Workspace::new(tmp.path());
    let result = workspace.compile_snapshot(&root, &basic_config());

    // Either Parse error or Import error depending on when it's caught
    assert!(result.is_err());
}

#[test]
fn test_workspace_security_violation() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("main.mirr");
    // Attempt absolute path out of the workspace root
    fs::write(&root, "import \"/etc/passwd\" as p;\nmodule main { }").unwrap();

    let mut workspace = Workspace::new(tmp.path());
    let result = workspace.compile_snapshot(&root, &basic_config());

    assert!(result.is_err());
    if let Err(e) = result {
        let err_str = e.to_string();
        assert!(
            err_str.contains("security violation") || err_str.contains("outside workspace root")
        );
    }
}

#[test]
fn test_workspace_caching_and_invalidation() {
    let tmp = TempDir::new().unwrap();
    let raw_root = tmp.path().join("main.mirr");
    fs::write(&raw_root, "module main { signal x: in bool; }").unwrap();
    let root = fs::canonicalize(&raw_root).unwrap_or(raw_root);

    let mut workspace = Workspace::new(tmp.path());
    let snap1 = workspace.compile_snapshot(&root, &basic_config()).unwrap();
    assert_eq!(snap1.imported_file_count(), 1);

    // Call again, should hit the cache (snapshot returned exactly)
    let snap2 = workspace.compile_snapshot(&root, &basic_config()).unwrap();
    assert_eq!(snap1.workspace_hash, snap2.workspace_hash);

    assert!(workspace.get_snapshot(&root).is_some());

    // Update the file through the workspace, which clears snapshots
    workspace.update_file(&root, "module main { signal y: in bool; }".to_string());
    assert!(workspace.get_snapshot(&root).is_none());

    // Should re-compile successfully
    let snap3 = workspace.compile_snapshot(&root, &basic_config()).unwrap();
    assert_ne!(snap1.workspace_hash, snap3.workspace_hash);
}

#[test]
fn test_workspace_pattern_merging() {
    let tmp = TempDir::new().unwrap();
    let sub = tmp.path().join("sub.mirr");
    let root = tmp.path().join("main.mirr");

    fs::write(
        &sub,
        "
        def helper(a: signal in bool) {
            reflect {
                property p { always(true); }
            }
        }
        module sub { }
    ",
    )
    .unwrap();

    fs::write(
        &root,
        "
        import \"sub.mirr\" as sub_lib;
        
        def local_pat() {
            reflect {
                property p { always(true); }
            }
        }
        module main { }
    ",
    )
    .unwrap();

    let mut workspace = Workspace::new(tmp.path());
    let snapshot = workspace.compile_snapshot(&root, &basic_config()).unwrap();

    let prog = &snapshot.pipeline.program.as_ref().unwrap();
    assert_eq!(prog.patterns.len(), 2);

    let names: Vec<_> = prog.patterns.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"local_pat"));
    assert!(names.contains(&"sub_lib::helper"));
}

#[test]
fn test_workspace_error_display() {
    let e1 = WorkspaceError::MissingSource(std::path::PathBuf::from("foo.mirr"));
    assert_eq!(e1.to_string(), "missing source for foo.mirr");

    let e2 = WorkspaceError::Io {
        path: std::path::PathBuf::from("foo.mirr"),
        message: "io err".to_string(),
    };
    assert_eq!(e2.to_string(), "I/O error for foo.mirr: io err");
}

#[test]
fn test_workspace_empty_import_path() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("main.mirr");
    fs::write(&root, "import \"\" as p;\nmodule main { }").unwrap();

    let mut workspace = Workspace::new(tmp.path());
    let result = workspace.compile_snapshot(&root, &basic_config());

    assert!(result.is_err());
}
