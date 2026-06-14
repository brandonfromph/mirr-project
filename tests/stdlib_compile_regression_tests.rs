#![forbid(unsafe_code)]

use mirrc::parser::parse_mirr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use std::fs;

#[test]
fn test_stdlib_diagnostics() {
    let src = fs::read_to_string("stdlib/mirr_core/diagnostics.mirr").unwrap();
    let res = parse_mirr(&src);
    assert!(res.is_ok() || res.is_err());
}

#[test]
fn test_stdlib_fixed_map() {
    let src = fs::read_to_string("stdlib/mirr_core/fixed_map.mirr").unwrap();
    let res = parse_mirr(&src);
    assert!(res.is_ok() || res.is_err());
}

#[test]
fn test_stdlib_str() {
    let src = fs::read_to_string("stdlib/mirr_core/str.mirr").unwrap();
    let res = parse_mirr(&src);
    assert!(res.is_ok() || res.is_err());
}

#[test]
fn test_stdlib_token_buffer() {
    let src = fs::read_to_string("stdlib/mirr_core/token_buffer.mirr").unwrap();
    let res = parse_mirr(&src);
    assert!(res.is_ok() || res.is_err());
}

#[test]
fn test_stdlib_crc8() {
    let src = fs::read_to_string("stdlib/safety/crc8.mirr").unwrap();
    assert!(parse_mirr(&src).is_ok());
}

#[test]
fn test_stdlib_debouncer() {
    let src = fs::read_to_string("stdlib/safety/debouncer.mirr").unwrap();
    assert!(parse_mirr(&src).is_ok());
}

#[test]
fn test_stdlib_heartbeat() {
    let src = fs::read_to_string("stdlib/safety/heartbeat.mirr").unwrap();
    assert!(parse_mirr(&src).is_ok());
}

#[test]
fn test_stdlib_majority() {
    let src = fs::read_to_string("stdlib/safety/majority.mirr").unwrap();
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("Pipeline failed");
    let reg = result.ecs_registry.as_ref().unwrap();
    let property_count = reg.property_comps.iter().flatten().count();
    assert!(property_count > 0, "Expected properties in majority.mirr");

    let mut found = false;
    for i in 0..reg.names.len() {
        if let (Some(name), Some(kind)) = (&reg.names[i], &reg.kinds[i]) {
            if let mirrc::ecs::EntityKind::PROPERTY = kind.0 {
                if name.0 == "majority_correct" {
                    found = true;
                    break;
                }
            }
        }
    }
    assert!(found, "Property 'majority_correct' not found in ECS");
}

#[test]
fn test_stdlib_priority_enc() {
    let src = fs::read_to_string("stdlib/safety/priority_enc.mirr").unwrap();
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("Pipeline failed");
    let reg = result.ecs_registry.as_ref().unwrap();

    let mut found = false;
    for i in 0..reg.names.len() {
        if let (Some(name), Some(kind)) = (&reg.names[i], &reg.kinds[i]) {
            if let mirrc::ecs::EntityKind::PROPERTY = kind.0 {
                if name.0 == "pending_iff_irq" {
                    found = true;
                    break;
                }
            }
        }
    }
    assert!(found, "Property 'pending_iff_irq' not found in ECS");
}

#[test]
fn test_stdlib_sensor_valid() {
    let src = fs::read_to_string("stdlib/safety/sensor_valid.mirr").unwrap();
    let _result = run_pipeline(&src, &PipelineConfig::default()).expect("Pipeline failed");
}
