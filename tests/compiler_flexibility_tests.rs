#![cfg(feature = "legacy_ast")]
#![forbid(unsafe_code)]

//! Integration tests for validating MIRR compiler flexibility and robustness.
//!
//! Tests the pipeline's capability to ingest, parse, validate, simplify, width-solve,
//! temporally-lower, and compile complex nested logical/arithmetic expressions,
//! temporal `prev` operations, and advanced composite types.

use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_compiler_pipeline_flexibility_temporal_arithmetic() {
    let source = r#"
module FlexibilityTemporal {
    signal clk_en: in bool;
    signal sensor_a: in u16;
    signal sensor_b: in u16;
    signal temp: in u16;
    
    signal threshold_reached: out bool;
    signal safety_trigger: out bool;
    
    guard g_critical {
        when (sensor_a > 100) && (sensor_b <= 500) && prev(clk_en, 2)
        for 4 cycles;
    }
    
    reflex trigger_control {
        on g_critical {
            threshold_reached = true;
            safety_trigger = temp > 400;
        }
    }
}
"#;

    let config = PipelineConfig {
        temporal: true,
        width: true,
        typecheck: true,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(source, &config);
    assert!(result.is_ok(), "Failed to run pipeline: {:?}", result.err());

    let res = result.unwrap();
    assert!(res.ecs_registry.is_some(), "Expected type checker map");
    assert!(res.width_stats.is_some(), "Expected width inference result");
    assert!(res.temporal_netlist.is_some(), "Expected temporal synthesis netlist");

    let netlist = res.temporal_netlist.unwrap();
    assert_eq!(netlist.guards.len(), 1, "Expected exactly 1 compiled guard");
}

#[test]
fn test_compiler_front_end_flexibility_composites() {
    let source = r#"
struct Location {
    x: u16;
    y: u16;
}

struct Telemetry {
    loc: struct Location;
    samples: u8[4];
    valid: bool;
}

module FlexibilityComposites {
    signal bus: in struct Telemetry;
    signal alert: out bool;
    signal sensor_val: out u8;
    
    guard check_alert {
        when bus.valid && (bus.loc.x > 10)
        for 1 cycles;
    }
    
    reflex actuate {
        on check_alert {
            alert = true;
            sensor_val = bus.samples[0];
        }
    }
}
"#;

    // Validate parsing and semantic validation for composite types
    let config = PipelineConfig {
        temporal: false,
        width: false,
        typecheck: true,
        ..PipelineConfig::default()
    };

    let result = run_pipeline(source, &config);
    assert!(result.is_ok(), "Failed to run pipeline for composites: {:?}", result.err());
}
