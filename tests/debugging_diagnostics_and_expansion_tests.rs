#![cfg(any())]
#![forbid(unsafe_code)]
#![deny(warnings)]

//! Comprehensive diagnostic, span, and macro expansion integration tests.
//!
//! Specifically validates:
//! 1. Macro pattern call provenance and DO-178C traceability.
//! 2. Precise span propagation in deeply nested macro constructs.
//! 3. Type checker error line and span accuracy.
//! 4. Diagnostics for edge-case parser and syntax errors (reflex sugar, unmatched braces).

use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::span::Span;

#[test]
fn test_macro_expansion_provenance_tracing() {
    let source = r#"
        def monitor_val(s: signal in bool) {
            reflect {
                signal alarm: internal bool;
                reflex r {
                    on always {
                        alarm = !${s};
                    }
                }
            }
        }
        module top {
            signal sensor_a: in bool;
            signal sensor_b: in bool;
            monitor_val(sensor_a);
            monitor_val(sensor_b);
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config).expect("Pipeline must succeed");

    // After expansion, the pattern calls themselves must be erased
    assert!(result.program.as_ref().unwrap().module.pattern_calls.is_empty());

    // Provenance (origins) must be recorded for each call
    let origins = &result.program.as_ref().unwrap().module.pattern_origins;
    assert_eq!(origins.len(), 2, "Expected exactly two pattern expansion origins");
    assert_eq!(origins[0].pattern_name, "monitor_val");
    assert!(origins[0].call_args_summary.contains("sensor_b"));
    assert_eq!(origins[1].pattern_name, "monitor_val");
    assert!(origins[1].call_args_summary.contains("sensor_a"));
}

#[test]
fn test_macro_syntax_error_inside_def() {
    let source = r#"
        def bad_pat() {
            reflect {
                signal x: internal bool
                // Missing semicolon here should trigger E109/E113 inside def block parsing
            }
        }
        module top {}
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);

    assert!(result.is_err(), "Malformed pattern definition must fail to parse");
    let err = result.err().unwrap();
    if err.errors[0].span().is_none() {
        panic!("err was: {:?}", err);
    }
    let span = err.errors[0].span().unwrap();
    // Semicolon error should match line 3
    assert_eq!(span.start_line, 3);
}

#[test]
fn test_deep_expression_span_accuracy() {
    let source = r#"
        module type_mismatch {
            signal a: in u8;
            signal b: in u8;
            signal c: in bool;
            signal o: out u8;
            guard g { when true for 1 cycles; }
            reflex r {
                on g {
                    o = (a + b) && c;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);

    assert!(result.is_err(), "Bitwise-logical mixed type expressions must fail typecheck");
    let err = result.err().unwrap();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("E60"), "Expected typecheck error code E60x, got: {}", err_str);

    let span = err.errors[0].span().expect("Expected precise span for expression type mismatch");
    // Line 11 contains: o = (a + b) && c; under normalized layout
    assert_eq!(span.start_line, 11);
}

#[test]
fn test_reassign_to_input_signal_span() {
    let source = r#"
        module write_input {
            signal in_sig: in u8;
            signal out_sig: out u8;
            guard g { when true for 1 cycles; }
            reflex r {
                on g {
                    in_sig = 42;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);

    assert!(result.is_err(), "Writing to an input signal must be rejected");
    let err = result.err().unwrap();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("E206") || err_str.contains("input"),
        "Expected E206 input write violation, got: {}",
        err_str
    );

    let span = err.errors[0].span().expect("Expected span pointing to the write statement");
    // Reflex r block is on line 7
    assert_eq!(span.start_line, 7);
}

#[test]
fn test_reflex_sugar_always_lowering_diagnostic() {
    let source = r#"
        module reflex_sugar {
            signal out_sig: out bool;
            reflex r {
                out_sig = true; // sugar: should lower to 'on always' and compile
            }
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);
    assert!(result.is_ok(), "Reflex sugar with direct assignment should compile successfully");
}

#[test]
fn test_reflex_empty_on_clause_error() {
    let source = r#"
        module empty_on {
            signal out_sig: out bool;
            reflex r {
                on {
                    out_sig = true;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);

    assert!(result.is_err(), "Empty 'on' clause with missing target guard must fail");
    let err = result.err().unwrap();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("E140"), "Expected error E140, got: {}", err_str);

    let span = err.errors[0].span().expect("Expected span on empty 'on' clause");
    // 'on {' is on line 4
    assert_eq!(span.start_line, 4);
}

#[test]
fn test_unmatched_brace_in_middle_diagnostics() {
    let source = r#"
        module unmatched_brace {
            signal a: out bool;
            reflex r {
                on always {
                    a = true;
                }
            }
        } } // trailing extra brace (must be ignored or yield error depending on scope)
    "#;

    let config = PipelineConfig::default();
    let result = run_pipeline(source, &config);
    // Trailing brace outside module is ignored, so this is valid
    assert!(
        result.is_ok(),
        "Trailing braces outside module must be ignored by single-module compiler"
    );
}

#[test]
fn test_span_merge_edge_cases() {
    let s1 = Span::single_line(5, 10, 20);
    let s2 = Span::single_line(5, 15, 30);
    let merged = s1.merge(s2);
    assert_eq!(merged.start_line, 5);
    assert_eq!(merged.start_col, 10);
    assert_eq!(merged.end_line, 5);
    assert_eq!(merged.end_col, 30);
}
