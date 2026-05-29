#![forbid(unsafe_code)]
//! Pipeline validation + temporal rejection tests.

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

const DUPLICATE_SIGNAL: &str = r#"
module dup {
    signal x: in bool;
    signal x: out bool;
}
"#;

const XOR_GUARD: &str = r#"
module xor_guard {
    signal a: in bool;
    signal b: in bool;
    signal out: out bool;

    guard g {
        when a ^ b
        for 5 cycles;
    }

    reflex r {
        on g {
            out = true;
        }
    }
}
"#;

#[test]
fn duplicate_signal_is_rejected_at_validation() {
    let err = match run_pipeline(DUPLICATE_SIGNAL, &PipelineConfig::default()) {
        Ok(_) => panic!("duplicate signal should fail validation"),
        Err(e) => e,
    };
    assert!(err.to_string().contains("[E201]"));
}

#[test]
fn xor_guard_is_rejected_by_temporal_lowering() {
    let err = match run_pipeline(XOR_GUARD, &PipelineConfig::default()) {
        Ok(_) => panic!("xor guard should fail temporal lowering"),
        Err(e) => e,
    };
    let msg = err.to_string();
    assert!(msg.contains("guard 'g'"), "expected guard context, got: {msg}");
    assert!(msg.contains("unsupported"), "expected unsupported-condition detail, got: {msg}");
}

#[test]
fn xor_guard_passes_when_temporal_is_disabled() {
    let config = PipelineConfig { temporal: false, ..PipelineConfig::default() };
    run_pipeline(XOR_GUARD, &config).expect("xor guard should pass when temporal is disabled");
}
