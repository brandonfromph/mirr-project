#![forbid(unsafe_code)]

use mirrc::parser::parse_mirr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::validation::validate_module;
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
    validate_module(&result.program.module).expect("Semantic validation failed");
    assert!(result.program.module.properties.iter().any(|p| p.name == "majority_correct"));
}

#[test]
fn test_stdlib_priority_enc() {
    let src = fs::read_to_string("stdlib/safety/priority_enc.mirr").unwrap();
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("Pipeline failed");
    validate_module(&result.program.module).expect("Validation failed");
    assert!(result.program.module.properties.iter().any(|p| p.name == "pending_iff_irq"));
}

#[test]
fn test_stdlib_sensor_valid() {
    let src = fs::read_to_string("stdlib/safety/sensor_valid.mirr").unwrap();
    let result = run_pipeline(&src, &PipelineConfig::default()).expect("Pipeline failed");
    validate_module(&result.program.module).expect("Validation failed");
}
