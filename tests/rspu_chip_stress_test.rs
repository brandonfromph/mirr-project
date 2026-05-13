#![forbid(unsafe_code)]

use nasa_rust_project::pipeline::PipelineConfig;
use nasa_rust_project::Workspace;
use std::path::PathBuf;

#[test]
fn test_rspu_chip_workspace_compilation() {
    let root_path = PathBuf::from("rspu_chip/rspu_top.mirr");
    let workspace_root = PathBuf::from("rspu_chip");
    
    // We already generated the files via python script in the workspace root.
    assert!(root_path.exists(), "RSPU top file does not exist, run generate_rspu.py first");

    let mut workspace = Workspace::new(&workspace_root);
    
    // Compile the multi-file project, resolving all imports across all subdirectories
    let mut config = PipelineConfig::default();
    config.temporal = false; // Disable temporal lowering if unsupported forms are used
    config.rspu = false;
    
    println!("Loading multi-file RSPU project from {}...", root_path.display());
    let snapshot = workspace.compile_snapshot(&root_path, &config).expect("Workspace compilation failed");
    
    println!("Compiled snapshot with hash: {}", snapshot.workspace_hash);
    println!("Total imported files: {}", snapshot.imported_file_count());
    
    // Basic structural checks to prove all files merged
    assert!(snapshot.imported_file_count() > 30, "Should have imported dozens of files");
    assert!(snapshot.pipeline.program.patterns.len() > 100, "Should have merged hundreds of patterns");
}
