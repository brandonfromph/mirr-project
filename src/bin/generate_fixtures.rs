use mirrc::{parse_mirr, MirrAstJson, TemporalGuardCompiler, TemporalNetlistJson};
use std::fs;
use std::path::Path;

fn main() {
    let examples_dir = Path::new("examples");
    let ast_out_dir = Path::new("tests/fixtures/ast");
    let netlist_out_dir = Path::new("tests/fixtures/netlist");

    fs::create_dir_all(ast_out_dir).expect("Failed to create ast output dir");
    fs::create_dir_all(netlist_out_dir).expect("Failed to create netlist output dir");

    if let Ok(entries) = fs::read_dir(examples_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("mirr") {
                let file_name = path.file_stem().unwrap().to_str().unwrap();
                println!("Processing {}...", file_name);

                let src = fs::read_to_string(&path).expect("Failed to read example source");

                let program = match parse_mirr(&src) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Skipping {} due to parse error: {:?}", file_name, e);
                        continue;
                    }
                };

                // AST
                let ast_json = MirrAstJson::from_program(&program);
                let ast_json_str = serde_json::to_string_pretty(&ast_json).unwrap();
                let ast_out_path = ast_out_dir.join(format!("{}.json", file_name));
                fs::write(&ast_out_path, ast_json_str).expect("Failed to write AST JSON");

                // Netlist
                let netlist =
                    match TemporalGuardCompiler::new().compile_temporal_guards(&program.module) {
                        Ok(n) => n,
                        Err(e) => {
                            println!(
                                "Skipping netlist for {} due to compile error: {:?}",
                                file_name, e
                            );
                            continue;
                        }
                    };
                let netlist_json = TemporalNetlistJson::from_netlist(&netlist);
                let netlist_json_str = serde_json::to_string_pretty(&netlist_json).unwrap();
                let netlist_out_path = netlist_out_dir.join(format!("{}.json", file_name));
                fs::write(&netlist_out_path, netlist_json_str)
                    .expect("Failed to write Netlist JSON");
            }
        }
    }
    println!("Fixture generation complete.");
}
