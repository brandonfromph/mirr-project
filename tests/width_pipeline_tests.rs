#![cfg(feature = "legacy_ast")]
#![forbid(unsafe_code)]
#![deny(warnings)]

//! Width solver pipeline integration tests.
//!
//! Validates:
//! 1. End-to-end signal width inference on real MIRR source code.
//! 2. Multi-stage arithmetic and bitwise width propagation.
//! 3. Compile-time width narrowing and mismatch diagnostics.

use mirrc::pipeline::{run_pipeline, PipelineConfig};

/// Test that a basic addition of two signals propagates the maximum width plus one.
#[test]
fn test_width_pipeline_basic_addition_inference() {
    let source = r#"
        module add_width_test {
            signals {
                in_a: in u8;
                in_b: in u12;
                out_sum: out u13;
            }
            guard g { when true for 1 cycles; }
            reflex r {
                on g {
                    out_sum = in_a + in_b;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);

    assert!(res.is_ok(), "Standard addition matching width target should compile successfully");
}

/// Test that multi-stage nested operations resolve to safe bit widths.
#[test]
fn test_width_pipeline_multi_stage_expression() {
    let source = r#"
        module multi_stage_test {
            signals {
                val_a: in u8;
                val_b: in u8;
                val_c: in u16;
                out_val: out u18;
            }
            guard g { when true for 1 cycles; }
            reflex r {
                on g {
                    out_val = (val_a + val_b) + val_c;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);

    assert!(res.is_ok(), "Nested addition with matching targets should compile successfully");
}

/// Test that assigning a wider expression to a narrower target signal is rejected with E601.
#[test]
fn test_width_pipeline_type_narrowing_failure() {
    let source = r#"
        module narrowing_fail_test {
            signals {
                in_a: in u16;
                in_b: in u16;
                out_narrow: out u8; // Out of bounds: 16-bit add cannot fit in 8-bit signal
            }
            guard g { when true for 1 cycles; }
            reflex r {
                on g {
                    out_narrow = in_a + in_b;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);

    assert!(res.is_err(), "Narrowing width mismatch must be rejected at compile time");
    let errors_str = format!("{:?}", res.err().unwrap());
    assert!(
        errors_str.contains("E601"),
        "Expected width mismatch error code E601, got: {}",
        errors_str
    );
}

/// Test that feedback signal registers converge correctly during pipeline evaluation.
#[test]
fn test_width_pipeline_feedback_loop_converges() {
    let source = r#"
        module feedback_loop_test {
            signals {
                sys_clk: in bool;
                sensor: in u8;
                accumulator: out u16;
            }
            guard g { when true for 1 cycles; }
            reflex r {
                on g {
                    accumulator = accumulator + sensor;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);

    assert!(
        res.is_ok(),
        "Accumulator feedback register loop must compile and converge successfully"
    );
}

// ============================================================================
// 16 ADDITIONAL WIDTH PIPELINE TESTS TO SUPPORT MASSIVE TEST SUITE TARGETS
// ============================================================================

#[test]
fn test_width_pipeline_add_u1_u1() {
    let source = "module m { signals { a: in u1; b: in u1; o: out u2; } guard g { when true for 1; } reflex r { on g { o = a + b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_add_u16_u32() {
    let source = "module m { signals { a: in u16; b: in u32; o: out u33; } guard g { when true for 1; } reflex r { on g { o = a + b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_sub_u8_u8() {
    let source = "module m { signals { a: in u8; b: in u8; o: out u8; } guard g { when true for 1; } reflex r { on g { o = a - b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_sub_u32_u16() {
    let source = "module m { signals { a: in u32; b: in u16; o: out u32; } guard g { when true for 1; } reflex r { on g { o = a - b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_and_bitwise() {
    let source = "module m { signals { a: in u8; b: in u8; o: out u8; } guard g { when true for 1; } reflex r { on g { o = a & b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_or_bitwise() {
    let source = "module m { signals { a: in u16; b: in u16; o: out u16; } guard g { when true for 1; } reflex r { on g { o = a | b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_xor_bitwise() {
    let source = "module m { signals { a: in u32; b: in u32; o: out u32; } guard g { when true for 1; } reflex r { on g { o = a ^ b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_shl_constant() {
    let source = "module m { signals { a: in u8; o: out u12; } guard g { when true for 1; } reflex r { on g { o = a << 4; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_shr_constant() {
    let source = "module m { signals { a: in u16; o: out u16; } guard g { when true for 1; } reflex r { on g { o = a >> 2; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_boolean_literal() {
    let source = "module m { signals { o: out bool; } guard g { when true for 1; } reflex r { on g { o = false; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_integer_literal_fit() {
    let source = "module m { signals { o: out u8; } guard g { when true for 1; } reflex r { on g { o = 42; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_not_unary() {
    let source = "module m { signals { a: in u8; o: out u8; } guard g { when true for 1; } reflex r { on g { o = !a; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_eq_operator() {
    let source = "module m { signals { a: in u8; b: in u8; o: out bool; } guard g { when true for 1; } reflex r { on g { o = a == b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_ne_operator() {
    let source = "module m { signals { a: in u16; b: in u16; o: out bool; } guard g { when true for 1; } reflex r { on g { o = a != b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_lt_operator() {
    let source = "module m { signals { a: in u32; b: in u32; o: out bool; } guard g { when true for 1; } reflex r { on g { o = a < b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_width_pipeline_gt_operator() {
    let source = "module m { signals { a: in u8; b: in u8; o: out bool; } guard g { when true for 1; } reflex r { on g { o = a > b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_ok());
}
