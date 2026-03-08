//! Phase 6 Integration Tests
//!
//! End-to-end pipeline tests verifying parse -> simplify -> width -> temporal -> emit.

use nasa_rust_project::emit;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

// ---------------------------------------------------------------------------
// Test fixtures
// ---------------------------------------------------------------------------

const NEONATAL_MIRR: &str = r#"
module neonatal_respirator {
    signal airway_pressure: in u16;
    signal clamp_valve: out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for 1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;

const MINIMAL_MIRR: &str = r#"
module minimal {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    reflex r {
        on g {
            b = a;
        }
    }
}
"#;

const ARITHMETIC_MIRR: &str = r#"
module arith_test {
    signal sensor: in u8;
    signal offset: in u8;
    signal result: out u16;

    guard check_sensor {
        when sensor > 10
        for 5 cycles;
    }

    reflex compute {
        on check_sensor {
            result = sensor + offset;
        }
    }
}
"#;

const EMPTY_GUARDS_MIRR: &str = r#"
module no_guards {
    signal x: in bool;
    signal y: out bool;
}
"#;

// ---------------------------------------------------------------------------
// Pipeline stage tests
// ---------------------------------------------------------------------------

#[test]
fn pipeline_full_neonatal() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).expect("pipeline should succeed");

    assert_eq!(result.program.module.name, "neonatal_respirator");
    assert!(result.simplify_stats.is_some());
    assert!(result.width_result.is_some());
    assert!(result.temporal_netlist.is_some());
}

#[test]
fn pipeline_parse_only() {
    let config = PipelineConfig { typecheck: false, simplify: false, width: false, temporal: false, rspu: false };
    let result = run_pipeline(MINIMAL_MIRR, &config).expect("pipeline should succeed");

    assert_eq!(result.program.module.name, "minimal");
    assert!(result.simplify_stats.is_none());
    assert!(result.width_result.is_none());
    assert!(result.temporal_netlist.is_none());
}

#[test]
fn pipeline_simplify_without_width() {
    let config = PipelineConfig { typecheck: true, simplify: true, width: false, temporal: false, rspu: false };
    let result = run_pipeline(MINIMAL_MIRR, &config).expect("pipeline should succeed");

    assert!(result.simplify_stats.is_some());
    assert!(result.width_result.is_none());
}

#[test]
fn pipeline_width_without_temporal() {
    let config = PipelineConfig { typecheck: true, simplify: true, width: true, temporal: false, rspu: false };
    let result = run_pipeline(ARITHMETIC_MIRR, &config).expect("pipeline should succeed");

    assert!(result.width_result.is_some());
    assert!(result.temporal_netlist.is_none());
}

#[test]
fn pipeline_rejects_invalid_source() {
    let config = PipelineConfig::default();
    let result = run_pipeline("this is not valid mirr", &config);
    assert!(result.is_err());
}

#[test]
fn pipeline_empty_module_no_guards() {
    let config = PipelineConfig::default();
    let result = run_pipeline(EMPTY_GUARDS_MIRR, &config).expect("pipeline should succeed");

    assert!(result.temporal_netlist.is_some());
    let netlist = result.temporal_netlist.as_ref().unwrap();
    assert_eq!(netlist.guards.len(), 0);
}

// ---------------------------------------------------------------------------
// DOT emitter tests
// ---------------------------------------------------------------------------

#[test]
fn dot_output_contains_digraph() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.starts_with("digraph "));
    assert!(dot.contains("neonatal_respirator"));
    assert!(dot.ends_with("}\n"));
}

#[test]
fn dot_output_contains_signal_nodes() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.contains("airway_pressure"));
    assert!(dot.contains("clamp_valve"));
    assert!(dot.contains("u16"));
    assert!(dot.contains("bool"));
}

#[test]
fn dot_output_contains_guard_diamond() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.contains("shape=diamond"));
    assert!(dot.contains("sustained_pressure_drop"));
    assert!(dot.contains("1000c"));
}

#[test]
fn dot_output_contains_temporal_subgraph() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.contains("cluster_temporal"));
    assert!(dot.contains("Temporal Hardware"));
}

#[test]
fn dot_output_contains_reflex_edges() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.contains("emergency_clamp"));
}

#[test]
fn dot_expr_detail_produces_subgraphs() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let dot = emit::dot::emit_expr_dot(&result);

    assert!(dot.contains("_expr"));
    assert!(dot.contains("cluster_guard_"));
    assert!(dot.contains("Lt"));
}

// ---------------------------------------------------------------------------
// SystemVerilog emitter tests
// ---------------------------------------------------------------------------

#[test]
fn sv_output_contains_module_declaration() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("module neonatal_respirator ("));
    assert!(sv.contains("endmodule"));
}

#[test]
fn sv_output_contains_port_directions() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("input "));
    assert!(sv.contains("output"));
    assert!(sv.contains("airway_pressure"));
    assert!(sv.contains("clamp_valve"));
}

#[test]
fn sv_output_contains_always_ff() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("always_ff"));
    assert!(sv.contains("posedge clk"));
    assert!(sv.contains("rst_n"));
}

#[test]
fn sv_output_contains_always_comb() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("always_comb"));
}

#[test]
fn sv_output_contains_width_annotations() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    // u16 = logic [15:0]
    assert!(sv.contains("[15:0]"));
}

#[test]
fn sv_counter_guard_emits_counter_logic() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    // 1000 cycles guard uses counter strategy
    assert!(sv.contains("counter"));
    assert!(sv.contains("1000"));
}

// ---------------------------------------------------------------------------
// JSON netlist emitter tests
// ---------------------------------------------------------------------------

#[test]
fn json_output_parses_as_valid_json() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let json_str = emit::json_netlist::emit_json(&result).expect("json emission should succeed");

    let parsed: serde_json::Value =
        serde_json::from_str(&json_str).expect("output should be valid JSON");
    assert!(parsed.is_object());
}

#[test]
fn json_output_contains_ir_version() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let json_str = emit::json_netlist::emit_json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["ir_version"], "1.0");
}

#[test]
fn json_output_contains_program_module() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let json_str = emit::json_netlist::emit_json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed["program"]["module"]["name"], "neonatal_respirator");
}

#[test]
fn json_output_contains_simplify_stats() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let json_str = emit::json_netlist::emit_json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert!(parsed["simplify_stats"].is_object());
    assert!(parsed["simplify_stats"]["nodes_before"].is_number());
}

#[test]
fn json_output_contains_width_stats() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let json_str = emit::json_netlist::emit_json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert!(parsed["width_stats"].is_object());
    assert!(parsed["width_stats"]["nodes_analyzed"].is_number());
}

#[test]
fn json_output_contains_temporal_netlist() {
    let config = PipelineConfig::default();
    let result = run_pipeline(NEONATAL_MIRR, &config).unwrap();
    let json_str = emit::json_netlist::emit_json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert!(parsed["temporal"].is_object());
    assert!(parsed["temporal"]["guards"].is_array());
}

#[test]
fn json_output_null_when_stages_skipped() {
    let config = PipelineConfig { typecheck: false, simplify: false, width: false, temporal: false, rspu: false };
    let result = run_pipeline(MINIMAL_MIRR, &config).unwrap();
    let json_str = emit::json_netlist::emit_json(&result).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    assert!(parsed["simplify_stats"].is_null());
    assert!(parsed["width_stats"].is_null());
    assert!(parsed["temporal"].is_null());
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn pipeline_arithmetic_no_width_errors() {
    let config = PipelineConfig::default();
    let result = run_pipeline(ARITHMETIC_MIRR, &config).unwrap();

    // u8 + u8 = u9, target is u16, no truncation
    assert!(!result.has_width_errors());
}

#[test]
fn dot_empty_module_still_valid() {
    let config = PipelineConfig::default();
    let result = run_pipeline(EMPTY_GUARDS_MIRR, &config).unwrap();
    let dot = emit::dot::emit_module_dot(&result);

    assert!(dot.starts_with("digraph "));
    assert!(dot.ends_with("}\n"));
}

#[test]
fn sv_empty_module_still_valid() {
    let config = PipelineConfig::default();
    let result = run_pipeline(EMPTY_GUARDS_MIRR, &config).unwrap();
    let sv = emit::verilog::emit_sv(&result);

    assert!(sv.contains("module no_guards"));
    assert!(sv.contains("endmodule"));
}
