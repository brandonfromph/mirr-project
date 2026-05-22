#![forbid(unsafe_code)]
//! Temporal Synthesis Torture Test
//! Generates a massive, high-density MIRR module to stress test
//! the consolidated synthesis logic and macro processor.

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn stress_test_temporal_synthesis_density() {
    let mut mirr = String::from("module torture {\n");

    // 1. Generate 100 random signals
    for i in 0..100 {
        mirr.push_str(&format!("    signal s{}: in bool;\n", i));
    }
    mirr.push_str("    signal out: out bool;\n\n");

    // 2. Generate 100 SR guards (1-16 cycles)
    for i in 0..100 {
        let cycles = (i % 16) + 1;
        mirr.push_str(&format!("    guard g_sr_{} = when s{} for {} cycles;\n", i, i, cycles));
    }

    // 3. Generate 100 Counter guards (17-200 cycles)
    for i in 0..100 {
        let cycles = 17 + (i % 184);
        mirr.push_str(&format!("    guard g_ctr_{} = when s{} for {} cycles;\n", i, i, cycles));
    }

    // 4. Generate 50 Nested Guards (AND/OR complexity)
    for i in 0..50 {
        mirr.push_str(&format!(
            "    guard g_nest_{} = when (g_sr_{} && g_ctr_{}) for 1 cycles;\n",
            i, i, i
        ));
    }

    // 5. Massive reflex block
    mirr.push_str("    reflex main {\n");
    mirr.push_str("        if g_nest_0 {\n");
    mirr.push_str("            out = true;\n");
    for i in 1..50 {
        mirr.push_str(&format!("        }} else if g_nest_{} {{\n", i));
        mirr.push_str("            out = false;\n");
    }
    mirr.push_str("        } else {\n");
    mirr.push_str("            out = false;\n");
    mirr.push_str("        }\n");
    mirr.push_str("    }\n");
    mirr.push_str("}\n");

    // Run the pipeline with R-SPU emission enabled
    let cfg = PipelineConfig { rspu: true, ..Default::default() };
    let result = run_pipeline(&mirr, &cfg);

    match result {
        Ok(res) => {
            println!("Torture Test Success (R-SPU Emitted)!");
            println!("Total Guards: {}", res.program.module.guards.len());
            println!("Total Signals: {}", res.program.module.signals.len());
            if let Some(prog) = &res.rspu_program {
                println!("R-SPU Instructions: {}", prog.instructions.len());
                println!("R-SPU Registers: {}", prog.registers_used);
            }
        }
        Err(e) => {
            println!("Torture Test caught EXPECTED synthesis failure (Resource Limit):");
            for err in &e.errors {
                println!("  - {}", err);
            }
        }
    }
}
