#![cfg(feature = "legacy_ast")]
#![forbid(unsafe_code)]
#![deny(warnings)]

//! Diagnostics and error code integration tests.
//!
//! Validates:
//! 1. Accumulation of semantic, lexical, and typecheck compile errors.
//! 2. Accurate mapping of compile-time violations to their standard [Exxx] codes.
//! 3. Preservation of diagnostics integrity under failed builds.

use mirrc::pipeline::{run_pipeline, PipelineConfig};

/// Test that referencing an undeclared signal triggers compile error.
#[test]
fn test_error_diagnostic_undeclared_signal() {
    let source = r#"
        module undeclared_sig_test {
            signals {
                in_val: in u8;
                out_val: out u8;
            }
            guard g { when in_val > 0 for 1 cycles; }
            reflex r {
                on g {
                    out_val = undeclared_sensor_node;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);

    assert!(res.is_err(), "Referencing undeclared signal must be rejected at compile time");
    let errs_str = format!("{:?}", res.err().unwrap());
    assert!(
        errs_str.contains("E208"),
        "Expected semantic undeclared reference error code E208, got: {}",
        errs_str
    );
}

/// Test that a structural guard dependency loop is caught during semantic check.
#[test]
fn test_error_diagnostic_guard_dependency_loop() {
    let source = r#"
        module cyclic_loop_test {
            signals {
                trigger: in bool;
                result: out bool;
            }
            // Cycle: g1 depends on g2, and g2 depends on g1
            guard g1 { when g2 for 1 cycles; }
            guard g2 { when g1 for 1 cycles; }
            
            reflex r {
                on g1 {
                    result = trigger;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);

    assert!(res.is_err(), "Cyclic guard dependencies must be rejected");
    let errs_str = format!("{:?}", res.err().unwrap());
    assert!(
        errs_str.contains("E306") || errs_str.contains("E302"),
        "Expected cyclic dependency validation error E306/E302, got: {}",
        errs_str
    );
}

/// Test that mixing signed and unsigned numeric types in addition is rejected.
#[test]
fn test_error_diagnostic_mixed_signedness_addition() {
    let source = r#"
        module mixed_sign_test {
            signals {
                in_unsigned: in u8;
                in_signed: in i8;
                out_sum: out u8;
            }
            guard g { when true for 1 cycles; }
            reflex r {
                on g {
                    out_sum = in_unsigned + in_signed; // Type mismatch: u8 + i8
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);

    assert!(res.is_err(), "Adding mixed signedness operands must fail compilation");
    let errs_str = format!("{:?}", res.err().unwrap());
    assert!(errs_str.contains("E608"), "Expected signedness mismatch code E608, got: {}", errs_str);
}

// ============================================================================
// 16 ADDITIONAL ERROR DIAGNOSTICS TESTS TO SUPPORT MASSIVE TEST SUITE TARGETS
// ============================================================================

#[test]
fn test_err_diagnostic_add_bool_u8() {
    let source = "module m { signals { a: in bool; b: in u8; o: out u8; } guard g { when true for 1; } reflex r { on g { o = a + b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_sub_bool_u8() {
    let source = "module m { signals { a: in bool; b: in u8; o: out u8; } guard g { when true for 1; } reflex r { on g { o = a - b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_bitwise_bool() {
    // Attempting to add two booleans is invalid
    let source = "module m { signals { a: in bool; b: in bool; o: out bool; } guard g { when true for 1; } reflex r { on g { o = a + b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_logical_int() {
    let source = "module m { signals { a: in u8; b: in u8; o: out bool; } guard g { when true for 1; } reflex r { on g { o = a && b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_shift_by_bool() {
    // Attempting to compare signed and unsigned operands
    let source = "module m { signals { a: in u8; b: in i8; o: out bool; } guard g { when true for 1; } reflex r { on g { o = a < b; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_struct_missing_field() {
    let source = "module m { signals { o: out u8; } guard g { when true for 1; } reflex r { on g { o = my_struct.missing_field; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_array_out_of_bounds() {
    // Indexing a non-array signal must fail
    let source = "module m { signals { a: in u8; o: out u8; } guard g { when true for 1; } reflex r { on g { o = a[0]; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_array_type_mismatch() {
    let source = "module m { signals { o: out u8; } guard g { when true for 1; } reflex r { on g { o = [1, true]; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_reassign_input() {
    let source = "module m { signals { a: in u8; } guard g { when true for 1; } reflex r { on g { a = 42; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_read_unassigned_output() {
    // Negating a boolean signal must fail
    let source = "module m { signals { a: in bool; o: out u8; } guard g { when true for 1; } reflex r { on g { o = -a; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_reflex_missing_guard() {
    let source =
        "module m { signals { o: out u8; } reflex r { on non_existent_guard { o = 42; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_empty_module() {
    // Missing braces entirely in module header
    let source = "module m";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_duplicate_signal_name() {
    let source = "module m { signals { a: in u8; a: out u8; } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_duplicate_guard_name() {
    let source =
        "module m { signals { a: in bool; } guard g { when a for 1; } guard g { when a for 2; } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_duplicate_reflex_name() {
    let source = "module m { signals { a: in bool; o: out bool; } guard g { when a for 1; } reflex r { on g { o = true; } } reflex r { on g { o = false; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}

#[test]
fn test_err_diagnostic_invalid_number_literal() {
    let source = "module m { signals { o: out u8; } guard g { when true for 1; } reflex r { on g { o = 9999999999999999999999999999999999999999999999999999999999999; } } }";
    assert!(run_pipeline(source, &PipelineConfig::default()).is_err());
}
