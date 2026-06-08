use mirrc::pipeline::PipelineConfig;
use mirrc::Workspace;
use std::path::PathBuf;

#[test]
fn test_print_guards() {
    let root_path = PathBuf::from("rspu_chip/interconnect/noc_test_wrapper.mirr");
    let workspace_root = PathBuf::from("rspu_chip/interconnect");
    let mut workspace = Workspace::new(&workspace_root);
    let config = PipelineConfig { rspu: false, bootstrap_mode: true, ..Default::default() };

    let snapshot =
        workspace.compile_snapshot(&root_path, &config).expect("Workspace compilation failed");
    if let Some(net) = &snapshot.pipeline.temporal_netlist {
        println!("Generated {} guards", net.guards.len());
        for g in &net.guards {
            println!("Guard: {}", g.name());
        }
    }
}
