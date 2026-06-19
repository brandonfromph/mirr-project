#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::pipeline::PipelineConfig;
use std::path::PathBuf;

#[test]
fn test_e2e_expansive_width_inference_to_verilog_pipeline() {
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

    let res = mirrc::pipeline::run_pipeline_with_file(source, "test.mirr", &config);
    if let Err(ref e) = res {
        panic!("E2E pipeline execution failed: {:?}", e);
    }

    let pipeline_res = res.unwrap();

    let emitted_verilog = mirrc::emit::verilog::emit_sv(&pipeline_res);
    println!("Emitted E2E Verilog:\n{}", emitted_verilog);

    assert!(
        emitted_verilog.contains("[15:0]") && emitted_verilog.contains("acc"),
        "E2E emitted Verilog does not contain correct 16-bit width declaration [15:0] for 'acc'!"
    );

    assert!(
        pipeline_res.temporal_netlist.is_some(),
        "E2E: Expected temporal retiming netlist generation"
    );

    println!("E2E expansive width pipeline verified successfully!");
}
