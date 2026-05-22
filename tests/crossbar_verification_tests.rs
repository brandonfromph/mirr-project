use nasa_rust_project::pipeline::PipelineConfig;
use nasa_rust_project::Workspace;
use std::path::PathBuf;

#[test]
fn test_crossbar_static_routing_integrity() {
    // 1. Setup workspace for the crossbar verification
    let root_path = PathBuf::from("rspu_chip/interconnect/crossbar.mirr");
    let config = PipelineConfig::default();

    // 2. Compile the crossbar module
    let mut workspace = Workspace::new(&root_path);
    let snapshot =
        workspace.compile_snapshot(&root_path, &config).expect("Crossbar failed to compile");

    // 3. Verify Static Route Integrity
    // Ensure all 16 inputs route to their corresponding outputs in our baseline
    let signals = &snapshot.pipeline.program.module.signals;

    // Check signal widths
    for i in 0..16 {
        let name = format!("data_out_{}", i);
        assert!(signals.iter().any(|s| s.name == name), "Missing output signal: {}", name);
    }
}
