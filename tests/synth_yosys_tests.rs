//! Integration tests for synthesis-clean SystemVerilog emission.
//!
//! Verifies that `emit_sv_synthesis()` strips all SVA property blocks
//! and that `emit_sva_bind_file()` produces correct bind modules.
//! Optionally validates end-to-end Yosys synthesis when Yosys is in PATH.

#![forbid(unsafe_code)]

use nasa_rust_project::emit::verilog;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

/// Helper: compile a .mirr source string through the full pipeline.
fn compile(source: &str) -> nasa_rust_project::pipeline::PipelineResult {
    run_pipeline(source, &PipelineConfig::default()).expect("pipeline should succeed")
}

/// Helper: check if Yosys is available on this system.
fn yosys_available() -> bool {
    std::process::Command::new("yosys").arg("-V").output().is_ok()
}

/// SVA keywords that must not appear in synthesis-clean output.
const SVA_KEYWORDS: &[&str] = &["assert property", "assume property", "cover property"];

// -----------------------------------------------------------------------
// Strip SVA tests
// -----------------------------------------------------------------------

#[test]
fn synth_tmr_strips_all_sva() {
    let source = include_str!("../examples/tmr_sensor_fusion.mirr");
    let result = compile(source);
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    for kw in SVA_KEYWORDS {
        assert!(!sv.contains(kw), "synthesis output must not contain '{kw}'");
    }
}

#[test]
fn synth_tmr_preserves_rtl() {
    let source = include_str!("../examples/tmr_sensor_fusion.mirr");
    let result = compile(source);
    let sv = verilog::emit_sv_synthesis(&result, None, 0);

    assert!(sv.contains("module tmr_sensor_fusion"), "missing module declaration");
    assert!(sv.contains("endmodule"), "missing endmodule");
    assert!(sv.contains("always_ff"), "missing always_ff");
    assert!(sv.contains("always_comb"), "missing always_comb");
}

#[test]
fn synth_bind_file_has_bind_and_sva() {
    let source = include_str!("../examples/tmr_sensor_fusion.mirr");
    let result = compile(source);
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(!bind.is_empty(), "bind file should not be empty for TMR");
    assert!(bind.contains("bind tmr_sensor_fusion"), "missing bind statement");
    assert!(bind.contains("_sva"), "missing SVA wrapper module");
    assert!(bind.contains("assert property"), "bind file should contain SVA");
}

#[test]
fn synth_bind_file_empty_for_no_properties() {
    let source = include_str!("../examples/shift_register_guard.mirr");
    let result = compile(source);
    let bind = verilog::emit_sva_bind_file(&result);

    assert!(bind.is_empty(), "bind file should be empty when module has no properties");
}

// -----------------------------------------------------------------------
// Comprehensive strip test: all compilable examples
// -----------------------------------------------------------------------

/// All examples that compile successfully through the pipeline.
const COMPILABLE_EXAMPLES: &[(&str, &str)] = &[
    ("autonomous_vehicle", include_str!("../examples/autonomous_vehicle.mirr")),
    ("fir_filter", include_str!("../examples/fir_filter.mirr")),
    // flight_controller.mirr excluded: guard uses unsupported temporal form
    // flight_controller_signed.mirr excluded: guard 'nose_down' uses unsupported temporal form
    ("icu_monitor", include_str!("../examples/icu_monitor.mirr")),
    ("industrial_safety", include_str!("../examples/industrial_safety.mirr")),
    ("multi_guard_monitor", include_str!("../examples/multi_guard_monitor.mirr")),
    ("neonatal_respirator", include_str!("../examples/neonatal_respirator.mirr")),
    ("pattern_usage", include_str!("../examples/pattern_usage.mirr")),
    ("safety_property", include_str!("../examples/safety_property.mirr")),
    ("shift_register_guard", include_str!("../examples/shift_register_guard.mirr")),
    ("tmr_sensor_fusion", include_str!("../examples/tmr_sensor_fusion.mirr")),
];

#[test]
fn synth_all_examples_strip_sva() {
    for (name, source) in COMPILABLE_EXAMPLES {
        let result = compile(source);
        let sv = verilog::emit_sv_synthesis(&result, None, 0);

        for kw in SVA_KEYWORDS {
            assert!(!sv.contains(kw), "example '{name}': synthesis output must not contain '{kw}'");
        }
    }
}

// -----------------------------------------------------------------------
// Yosys end-to-end synthesis (skipped if Yosys not in PATH)
// -----------------------------------------------------------------------

#[test]
fn synth_yosys_all_examples() {
    if !yosys_available() {
        eprintln!("Yosys not found in PATH, skipping end-to-end synthesis test");
        return;
    }

    for (name, source) in COMPILABLE_EXAMPLES {
        let result = compile(source);
        let sv = verilog::emit_sv_synthesis(&result, None, 0);

        // Write to a temp file.
        let tmp_dir = std::env::temp_dir();
        let sv_path = tmp_dir.join(format!("mirr_synth_test_{name}.sv"));
        std::fs::write(&sv_path, &sv).expect("failed to write temp SV file");

        let module_name = &result.program.module.name;
        let yosys_cmd =
            format!("read_verilog -sv {}; synth -top {module_name}; stat", sv_path.display());

        let output = std::process::Command::new("yosys")
            .args(["-p", &yosys_cmd])
            .output()
            .expect("failed to run yosys");

        assert!(
            output.status.success(),
            "Yosys synthesis failed for example '{name}':\n{}",
            String::from_utf8_lossy(&output.stderr)
        );

        // Clean up.
        let _ = std::fs::remove_file(&sv_path);
    }
}
