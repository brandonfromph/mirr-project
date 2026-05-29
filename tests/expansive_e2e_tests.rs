#![forbid(unsafe_code)]
#![deny(warnings)]

use nasa_rust_project::pipeline::PipelineConfig;
use std::path::PathBuf;

#[test]
fn test_e2e_expansive_width_inference_to_verilog_pipeline() {
    // 1. Parse a valid MIRR module containing an accumulator loop and guard.
    // The target signal 'acc' is declared as u16.
    // The addition 'prev(acc, 1) + 2' will naturally infer to 17 bits (MaxPlusOne).
    // An assignment from u17 back to u16 normally triggers a truncation error,
    // but because 'acc' belongs to an expansive SCC loop, Phase 4b suppresses
    // the truncation error, allowing compilation to succeed cleanly.
    let source = r#"
module industrial_integrator {
    signal acc: internal u16;
    signal clock: internal bool;

    guard bound_guard {
        when clock
        for 30 cycles;
    }

    reflex integrate {
        on bound_guard {
            acc = prev(acc, 1) + 2;
        }
    }
}
"#;

    let program =
        nasa_rust_project::parse_mirr(source).expect("E2E: Failed to parse industrial_integrator");

    // 2. Configure the pipeline with all standard checks enabled (including typecheck and width inference)
    let config = PipelineConfig {
        typecheck: true,
        temporal: true,
        width: true,
        base_dir: Some(PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()),
        )),
        ..Default::default()
    };

    println!("Running full compiler pipeline on standard MIRR program...");

    // Convert program back to in-memory module validation
    let res = nasa_rust_project::pipeline::run_pipeline_on_program(program, &config);
    if let Err(ref e) = res {
        panic!("E2E pipeline execution failed: {:?}", e);
    }

    let pipeline_res = res.unwrap();

    // 3. Assertions on E2E compilation results:
    // Verify that the width solver successfully ran, suppressed the truncation error,
    // and correctly produced a successful compiled output.
    let emitted_verilog = nasa_rust_project::emit::verilog::emit_sv(&pipeline_res);

    println!("Emitted E2E Verilog:\n{}", emitted_verilog);

    // Verify 'acc' is declared as a 16-bit signal (logic [15:0] acc)
    assert!(
        emitted_verilog.contains("[15:0]") && emitted_verilog.contains("acc"),
        "E2E emitted Verilog does not contain correct 16-bit width declaration [15:0] for 'acc'!"
    );

    // Verify guard and retiming constructs were generated
    assert!(
        pipeline_res.temporal_netlist.is_some(),
        "E2E: Expected temporal retiming netlist generation"
    );

    println!("E2E expansive width pipeline verified successfully!");
}
