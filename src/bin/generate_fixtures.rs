use mirrc::{parse_mirr, MirrAstJson, TemporalGuardCompiler, TemporalNetlistJson};
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let examples_dir = Path::new("examples");
    let ast_out_dir = Path::new("tests/fixtures/ast");
    let netlist_out_dir = Path::new("tests/fixtures/netlist");

    fs::create_dir_all(ast_out_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create ast output dir: {}", e))?;
    fs::create_dir_all(netlist_out_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create netlist output dir: {}", e))?;

    if let Ok(entries) = fs::read_dir(examples_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("mirr") {
                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| anyhow::anyhow!("Failed to get file stem for {:?}", path))?;

                println!("Processing {}...", file_stem);

                let src = fs::read_to_string(&path).map_err(|e| {
                    anyhow::anyhow!("Failed to read example source {:?}: {}", path, e)
                })?;

                let program = match parse_mirr(&src) {
                    Ok(p) => p,
                    Err(e) => {
                        println!("Skipping {} due to parse error: {:?}", file_stem, e);
                        continue;
                    }
                };

                // AST
                let ast_json = MirrAstJson::from_program(&program);
                let ast_json_str = serde_json::to_string_pretty(&ast_json).map_err(|e| {
                    anyhow::anyhow!("Failed to serialize AST JSON for {}: {}", file_stem, e)
                })?;
                let ast_out_path = ast_out_dir.join(format!("{}.json", file_stem));
                fs::write(&ast_out_path, ast_json_str).map_err(|e| {
                    anyhow::anyhow!("Failed to write AST JSON to {:?}: {}", ast_out_path, e)
                })?;

                // Netlist
                let netlist =
                    match TemporalGuardCompiler::new().compile_temporal_guards(&program.module) {
                        Ok(n) => n,
                        Err(e) => {
                            println!(
                                "Skipping netlist for {} due to compile error: {:?}",
                                file_stem, e
                            );
                            continue;
                        }
                    };
                let netlist_json = TemporalNetlistJson::from_netlist(&netlist);
                let netlist_json_str =
                    serde_json::to_string_pretty(&netlist_json).map_err(|e| {
                        anyhow::anyhow!("Failed to serialize Netlist JSON for {}: {}", file_stem, e)
                    })?;
                let netlist_out_path = netlist_out_dir.join(format!("{}.json", file_stem));
                fs::write(&netlist_out_path, netlist_json_str).map_err(|e| {
                    anyhow::anyhow!("Failed to write Netlist JSON to {:?}: {}", netlist_out_path, e)
                })?;
            }
        }
    }

    println!("Fixture generation complete.");
    Ok(())
}
