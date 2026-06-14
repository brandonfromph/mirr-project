//! Phase 6: Structural Trace & Internal Consistency.
//!
//! Deep-state verification of the Registry to ensure no dangling
//! entities or semantic gaps exist after expansion.

#![forbid(unsafe_code)]

use mirrc::ecs::components::EntityKind;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn test_ecs_span_linkage_integrity() {
    let source = r#"
        module trace_test {
            signal a: in u32;
            signal b: in u32;
            signal c: out u32;
            guard g { when true for 1 cycles; }
            reflex r { on g { c = a + b; } }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config).expect("Pipeline failed");
    let reg = res.ecs_registry.as_ref().expect("Registry required");

    // Every signal entity in the Registry MUST have a valid Span
    let next_id = reg.active_entities();
    let mut signal_count = 0;
    for i in 0..next_id {
        let idx = i as usize;
        if let Some(kind) = &reg.kinds[idx] {
            if let EntityKind::SIGNAL(_) = kind.0 {
                signal_count += 1;
                assert!(
                    reg.spans[idx].is_some(),
                    "Signal Entity {} is missing a Span reference (Line/Col mapping)",
                    i
                );
            }
        }
    }
    assert!(signal_count >= 3, "Expected at least 3 signals (a, b, c), found {}", signal_count);
}

#[test]
fn test_diagnostic_precision_whitebox() {
    // Intentionally create a width mismatch to verify the diagnostic message
    let source = r#"
        module bad_width {
            signal a: in u8;
            signal b: in u16;
            signal c: out u8;
            guard g { when true for 1 cycles; }
            reflex r { on g { c = a ^ b; } }
        }
    "#;

    let res = run_pipeline(source, &PipelineConfig::default());
    assert!(res.is_err(), "Pipeline should fail for bitwise width mismatch");

    let errs = res.err().unwrap();
    let found_e607 = errs.errors.iter().any(|e| match e {
        mirrc::error::MirrError::TypeError { message, .. } => {
            message.contains("[E607]") && message.contains("requires matching types")
        }
        _ => false,
    });
    assert!(found_e607, "Expected E607 width mismatch error, got: {:?}", errs);
}

#[test]
fn test_internal_wiring_consistency() {
    let source = r#"
        module wiring_test {
            signal a: in u8;
            signal b: in u8;
            signal c: out u8;
            guard g { when true for 1 cycles; }
            reflex r { on g { c = a & b; } }
        }
    "#;

    let res = run_pipeline(source, &PipelineConfig::default()).expect("Pipeline failed");
    let reg = res.ecs_registry.as_ref().expect("Registry required");

    // Verify binary operator wiring
    let next_id = reg.active_entities();
    let mut bin_op_found = false;
    for i in 0..next_id {
        let idx = i as usize;
        if let Some(bin) = &reg.binary_ops[idx] {
            bin_op_found = true;
            // Verify that 'left' and 'right' point to valid SignalRef entities
            let left_idx = bin.left.0 as usize;
            let right_idx = bin.right.0 as usize;

            assert!(
                reg.signal_refs[left_idx].is_some() || reg.pending_signal_refs[left_idx].is_some(),
                "Binary op left child is dangling"
            );
            assert!(
                reg.signal_refs[right_idx].is_some()
                    || reg.pending_signal_refs[right_idx].is_some(),
                "Binary op right child is dangling"
            );
        }
    }
    assert!(bin_op_found, "Binary operator '&' was not found in Registry");
}
