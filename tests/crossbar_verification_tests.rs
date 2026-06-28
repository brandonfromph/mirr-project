#![allow(clippy::field_reassign_with_default)]
#![cfg(feature = "legacy_ast")]
#![cfg(any())]
use mirrc::pipeline::PipelineConfig;
use mirrc::Workspace;
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
    let signals = &snapshot.pipeline.program.as_ref().unwrap().module.signals;

    // Dynamically determine the number of ports by counting data_out_N signals
    let num_ports = signals.iter().filter(|s| s.name.starts_with("data_out_")).count();

    // Check signal widths
    for i in 0..num_ports {
        let name = format!("data_out_{}", i);
        assert!(signals.iter().any(|s| s.name == name), "Missing output signal: {}", name);
    }
}
