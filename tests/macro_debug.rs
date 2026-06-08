// test file

#[test]
fn debug_macro_expansion() {
    let root_path = std::path::PathBuf::from("rspu_chip/core/alu_test_wrapper.mirr");
    let workspace_root = std::path::PathBuf::from("rspu_chip/core");
    let mut workspace = mirrc::Workspace::new(&workspace_root);
    let config =
        mirrc::pipeline::PipelineConfig { rspu: true, bootstrap_mode: true, ..Default::default() };
    match workspace.compile_snapshot(&root_path, &config) {
        Ok(snapshot) => {
            if let Some(prog) = &snapshot.pipeline.rspu_program {
                println!("REGISTER MAP: {:#?}", prog.register_map);
                println!("INSTRUCTIONS: {:#?}", prog.instructions);
            }
        }
        Err(e) => {
            println!("COMPILATION FAILED: {:?}", e);
        }
    }
}
