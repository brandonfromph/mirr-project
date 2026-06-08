use mirrc::pipeline::PipelineConfig;
use mirrc::Workspace;
use std::path::Path;

fn main() {
    let root_file = "rspu_chip/rspu_top.mirr";
    let root_path = Path::new(root_file);
    let mut workspace = Workspace::new("rspu_chip");

    let source = std::fs::read_to_string(root_file).unwrap();
    workspace.update_file(root_file, source);

    let config = PipelineConfig { bootstrap_mode: true, rspu: true, ..Default::default() };

    match workspace.compile_snapshot(root_path, &config) {
        Ok(_) => println!("Compilation succeeded!"),
        Err(e) => println!("Compilation failed: {}", e),
    }
}
