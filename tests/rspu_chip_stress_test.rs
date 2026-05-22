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
    let config = PipelineConfig {
        temporal: false, // Disable temporal lowering if unsupported forms are used
        rspu: false,
        ..Default::default()
    };

    println!("Loading multi-file RSPU project from {}...", root_path.display());
    let snapshot =
        workspace.compile_snapshot(&root_path, &config).expect("Workspace compilation failed");

    println!("Compiled snapshot with hash: {}", snapshot.workspace_hash);
    println!("Total imported files: {}", snapshot.imported_file_count());

    // Structural checks for the RS-16 Liquid Architecture
    assert!(snapshot.imported_file_count() >= 2, "Should have imported ALU and RAM modules");

    // Verify the core alu_core pattern is merged into the global namespace
    let has_alu = snapshot.pipeline.program.patterns.iter().any(|p| p.name.contains("alu_core"));
    assert!(has_alu, "RS-16 should contain the alu_core pattern");
}
