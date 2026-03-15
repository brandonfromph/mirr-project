//! MEGA-4 Subsystem Verification Test Suite — Totality Engine + Proof Certificates.
//!
//! NASA-style verification tests for the MIRR totality engine (5 analyses),
//! proof certificate format (serialize/deserialize), and pipeline integration.
//!
//! Covers:
//! - F1: Resource bounds analysis (check_resource_bounds)
//! - F2: Output completeness (check_output_completeness)
//! - F3: Guard coverage (check_guard_coverage)
//! - F4: Temporal bound (check_temporal_bound)
//! - F5: Dependency acyclicity (check_dependency_acyclicity)
//! - F6: Aggregate totality (run_totality_check — all 5 pass)
//! - F7: Property summary (build_property_summary)
//! - F8: Proof certificate serialize/deserialize roundtrip
//! - F9: TerminationStrategy variants
//! - F10: Pipeline integration (totality flag in PipelineConfig)
//! - F11: TotalityError variant in MirrError
//!
//! Every loop is bounded by a MAX_* constant. No recursion. No unsafe code.

#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{ExtendedType, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::cert::{ProofCertificate, TerminationStrategy};
use nasa_rust_project::emit::rspu_isa::{MAX_GUARDS, MAX_INSTRUCTIONS, MAX_REGISTERS};
use nasa_rust_project::error::MirrError;
use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::totality::run_totality_check;

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum test iterations in any bounded loop.
const _MAX_TEST_ITERATIONS: usize = 256;

// ---------------------------------------------------------------------------
// AST Helpers — build Module directly for unit tests
// ---------------------------------------------------------------------------

fn make_signal(name: &str, kind: SignalKind) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(SignalType::Bool),
        origin: None,
        span: None,
    }
}

fn _make_signal_u16(name: &str, kind: SignalKind) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(SignalType::Unsigned(16)),
        origin: None,
        span: None,
    }
}

fn make_guard(name: &str, cycles: u64) -> Guard {
    Guard {
        name: name.to_string(),
        condition: Expr::Literal(LiteralValue::Bool(true)),
        cycles,
        origin: None,
        span: None,
    }
}

fn _make_guard_on_signal(name: &str, signal: &str, cycles: u64) -> Guard {
    Guard {
        name: name.to_string(),
        condition: Expr::Signal(signal.to_string()),
        cycles,
        origin: None,
        span: None,
    }
}

fn make_reflex(name: &str, guard: &str, target: &str) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec![guard.to_string()],
        assignments: vec![Assignment {
            target: target.to_string(),
            value: Expr::Literal(LiteralValue::Bool(true)),
            span: None,
        }],
        origin: None,
        span: None,
    }
}

fn make_reflex_with_expr(name: &str, guard: &str, target: &str, value: Expr) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: vec![guard.to_string()],
        assignments: vec![Assignment { target: target.to_string(), value, span: None }],
        origin: None,
        span: None,
    }
}

fn make_module(signals: Vec<SignalDecl>, guards: Vec<Guard>, reflexes: Vec<Reflex>) -> Module {
    Module {
        name: "test".to_string(),
        signals,
        guards,
        reflexes,
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

fn make_module_with_properties(
    signals: Vec<SignalDecl>,
    guards: Vec<Guard>,
    reflexes: Vec<Reflex>,
    properties: Vec<PropertyDecl>,
) -> Module {
    Module {
        name: "test".to_string(),
        signals,
        guards,
        reflexes,
        properties,
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

/// Shorthand: a well-formed total module with 1 input, 1 output, 1 guard, 1 reflex.
fn total_module() -> Module {
    make_module(
        vec![
            make_signal("input_a", SignalKind::Input),
            make_signal("output_b", SignalKind::Output),
        ],
        vec![make_guard("g1", 3)],
        vec![make_reflex("r1", "g1", "output_b")],
    )
}

// ===========================================================================
// F1: Resource bounds analysis
// ===========================================================================

#[test]
fn f1_total_module_passes_resource_bounds() {
    let m = total_module();
    let result = run_totality_check(&m);
    assert!(result.resource_bound.pass, "Total module must pass resource bounds");
}

#[test]
fn f1_registers_count_matches_signal_count() {
    let m = make_module(
        vec![
            make_signal("a", SignalKind::Input),
            make_signal("b", SignalKind::Input),
            make_signal("c", SignalKind::Output),
        ],
        vec![make_guard("g", 1)],
        vec![make_reflex("r", "g", "c")],
    );
    let result = run_totality_check(&m);
    assert_eq!(result.resource_bound.registers, 3, "3 signals need 3 registers");
}

#[test]
fn f1_guard_count_matches_guard_decls() {
    let m = make_module(
        vec![make_signal("a", SignalKind::Input), make_signal("out", SignalKind::Output)],
        vec![make_guard("g1", 1), make_guard("g2", 5), make_guard("g3", 10)],
        vec![make_reflex("r1", "g1", "out")],
    );
    let result = run_totality_check(&m);
    assert_eq!(result.resource_bound.guards, 3, "3 guard decls need 3 guard units");
}

#[test]
fn f1_max_cycles_is_highest_guard() {
    let m = make_module(
        vec![make_signal("a", SignalKind::Input), make_signal("out", SignalKind::Output)],
        vec![make_guard("g1", 5), make_guard("g2", 100), make_guard("g3", 50)],
        vec![make_reflex("r1", "g1", "out")],
    );
    let result = run_totality_check(&m);
    assert_eq!(
        result.resource_bound.max_cycles, 100,
        "Max cycles must be highest guard cycle count"
    );
}

#[test]
fn f1_resource_limits_constants() {
    assert_eq!(MAX_REGISTERS, 256, "MAX_REGISTERS hardware limit");
    assert_eq!(MAX_GUARDS, 64, "MAX_GUARDS hardware limit");
    assert_eq!(MAX_INSTRUCTIONS, 4096, "MAX_INSTRUCTIONS hardware limit");
}

// ===========================================================================
// F2: Output completeness
// ===========================================================================

#[test]
fn f2_all_outputs_driven_passes() {
    let m = total_module();
    let result = run_totality_check(&m);
    assert!(result.output_completeness.pass, "All outputs driven must pass");
    assert!(result.output_completeness.undriven_outputs.is_empty(), "No undriven outputs expected");
}

#[test]
fn f2_undriven_output_fails() {
    let m = make_module(
        vec![
            make_signal("a", SignalKind::Input),
            make_signal("out1", SignalKind::Output),
            make_signal("out2", SignalKind::Output), // undriven
        ],
        vec![make_guard("g1", 1)],
        vec![make_reflex("r1", "g1", "out1")], // only drives out1
    );
    let result = run_totality_check(&m);
    assert!(!result.output_completeness.pass, "Undriven output must fail completeness");
    assert_eq!(
        result.output_completeness.undriven_outputs,
        vec!["out2"],
        "out2 must be reported as undriven"
    );
}

#[test]
fn f2_multiple_undriven_outputs() {
    let m = make_module(
        vec![
            make_signal("a", SignalKind::Input),
            make_signal("out1", SignalKind::Output),
            make_signal("out2", SignalKind::Output),
            make_signal("out3", SignalKind::Output),
        ],
        vec![make_guard("g1", 1)],
        vec![make_reflex("r1", "g1", "out1")],
    );
    let result = run_totality_check(&m);
    assert!(!result.output_completeness.pass);
    assert_eq!(result.output_completeness.undriven_outputs.len(), 2, "out2 and out3 undriven");
}

#[test]
fn f2_input_signals_not_checked() {
    // Module with only inputs and no outputs — completeness trivially passes.
    let m = make_module(
        vec![make_signal("a", SignalKind::Input), make_signal("b", SignalKind::Input)],
        vec![],
        vec![],
    );
    let result = run_totality_check(&m);
    assert!(result.output_completeness.pass, "No outputs means completeness passes");
}

#[test]
fn f2_internal_signals_not_checked() {
    let m = make_module(
        vec![make_signal("a", SignalKind::Input), make_signal("state", SignalKind::Internal)],
        vec![],
        vec![],
    );
    let result = run_totality_check(&m);
    assert!(result.output_completeness.pass, "Internal signals not subject to output completeness");
}

// ===========================================================================
// F3: Guard coverage
// ===========================================================================

#[test]
fn f3_covered_outputs_pass() {
    let m = total_module();
    let result = run_totality_check(&m);
    assert!(result.guard_coverage.pass, "All outputs with guards must pass coverage");
    assert_eq!(result.guard_coverage.covered_outputs, 1);
    assert_eq!(result.guard_coverage.total_outputs, 1);
}

#[test]
fn f3_no_outputs_means_trivially_covered() {
    let m = make_module(vec![make_signal("a", SignalKind::Input)], vec![], vec![]);
    let result = run_totality_check(&m);
    assert!(result.guard_coverage.pass, "No outputs means coverage passes trivially");
}

// ===========================================================================
// F4: Temporal bound
// ===========================================================================

#[test]
fn f4_temporal_bound_is_max_guard_cycles() {
    let m = make_module(
        vec![make_signal("a", SignalKind::Input), make_signal("out", SignalKind::Output)],
        vec![make_guard("g1", 10), make_guard("g2", 25)],
        vec![make_reflex("r1", "g1", "out")],
    );
    let result = run_totality_check(&m);
    assert_eq!(result.temporal_bound.max_guard_cycles, 25);
    assert!(result.temporal_bound.pass, "Temporal bound always passes (finite cycles)");
}

#[test]
fn f4_prev_delay_contributes_to_latency() {
    // Reflex uses prev(input_a, 5) — adds 5 to worst-case latency.
    let m = make_module(
        vec![make_signal("input_a", SignalKind::Input), make_signal("out", SignalKind::Output)],
        vec![make_guard("g1", 10)],
        vec![make_reflex_with_expr(
            "r1",
            "g1",
            "out",
            Expr::Prev { signal: "input_a".to_string(), delay: 5 },
        )],
    );
    let result = run_totality_check(&m);
    assert_eq!(result.temporal_bound.max_prev_delay, 5);
    assert_eq!(result.temporal_bound.worst_case_latency, 15, "10 guard + 5 prev = 15");
}

#[test]
fn f4_zero_guards_zero_latency() {
    let m = make_module(vec![make_signal("a", SignalKind::Input)], vec![], vec![]);
    let result = run_totality_check(&m);
    assert_eq!(result.temporal_bound.max_guard_cycles, 0);
    assert_eq!(result.temporal_bound.worst_case_latency, 0);
}

// ===========================================================================
// F5: Dependency acyclicity
// ===========================================================================

#[test]
fn f5_acyclic_module_passes() {
    let m = total_module();
    let result = run_totality_check(&m);
    assert!(result.acyclicity.pass, "Acyclic module must pass");
    assert!(result.acyclicity.cycle_witness.is_none());
}

#[test]
fn f5_self_referencing_output_is_cycle() {
    let m = make_module(
        vec![make_signal("out", SignalKind::Output)],
        vec![make_guard("g1", 1)],
        vec![make_reflex_with_expr(
            "r_cycle",
            "g1",
            "out",
            Expr::Signal("out".to_string()), // self-reference
        )],
    );
    let result = run_totality_check(&m);
    assert!(!result.acyclicity.pass, "Self-referencing signal must be a cycle");
    assert_eq!(
        result.acyclicity.cycle_witness.as_deref(),
        Some("out"),
        "Cycle witness must name the cycling signal"
    );
}

#[test]
fn f5_prev_breaks_cycle() {
    let m = make_module(
        vec![make_signal("out", SignalKind::Output)],
        vec![make_guard("g1", 1)],
        vec![make_reflex_with_expr(
            "r_prev",
            "g1",
            "out",
            Expr::Prev { signal: "out".to_string(), delay: 1 },
        )],
    );
    let result = run_totality_check(&m);
    assert!(result.acyclicity.pass, "Prev must break the combinational cycle");
}

#[test]
fn f5_empty_module_acyclic() {
    let m = make_module(vec![], vec![], vec![]);
    let result = run_totality_check(&m);
    assert!(result.acyclicity.pass, "Empty module has no cycles");
}

#[test]
fn f5_input_to_output_no_cycle() {
    let m = make_module(
        vec![make_signal("in_a", SignalKind::Input), make_signal("out_b", SignalKind::Output)],
        vec![make_guard("g1", 1)],
        vec![make_reflex_with_expr("r1", "g1", "out_b", Expr::Signal("in_a".to_string()))],
    );
    let result = run_totality_check(&m);
    assert!(result.acyclicity.pass, "Input→output is not a cycle");
}

// ===========================================================================
// F6: Aggregate totality
// ===========================================================================

#[test]
fn f6_total_module_is_total() {
    let m = total_module();
    let result = run_totality_check(&m);
    assert!(result.is_total, "Well-formed module must pass all 5 totality checks");
}

#[test]
fn f6_undriven_output_makes_not_total() {
    let m = make_module(
        vec![
            make_signal("a", SignalKind::Input),
            make_signal("out1", SignalKind::Output),
            make_signal("out2", SignalKind::Output),
        ],
        vec![make_guard("g1", 1)],
        vec![make_reflex("r1", "g1", "out1")],
    );
    let result = run_totality_check(&m);
    assert!(!result.is_total, "Undriven output must make module non-total");
}

#[test]
fn f6_cycle_makes_not_total() {
    let m = make_module(
        vec![make_signal("s", SignalKind::Output)],
        vec![make_guard("g", 1)],
        vec![make_reflex_with_expr("r", "g", "s", Expr::Signal("s".to_string()))],
    );
    let result = run_totality_check(&m);
    assert!(!result.is_total, "Cyclic module must not be total");
}

// ===========================================================================
// F7: Property summary
// ===========================================================================

#[test]
fn f7_properties_captured_in_summary() {
    let props = vec![
        PropertyDecl {
            name: "p1".to_string(),
            formula: PropertyFormula::Always(Expr::Literal(LiteralValue::Bool(true))),
            directive: PropertyDirective::Assert,
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "p2".to_string(),
            formula: PropertyFormula::Never(Expr::Literal(LiteralValue::Bool(false))),
            directive: PropertyDirective::Assert,
            origin: None,
            span: None,
        },
    ];
    let m = make_module_with_properties(
        vec![make_signal("a", SignalKind::Input)],
        vec![],
        vec![],
        props,
    );
    let result = run_totality_check(&m);
    assert_eq!(result.property_summary.len(), 2, "Two properties must be summarized");
    assert_eq!(result.property_summary[0].name, "p1");
    assert_eq!(result.property_summary[0].kind, "always");
    assert_eq!(result.property_summary[1].name, "p2");
    assert_eq!(result.property_summary[1].kind, "never");
}

#[test]
fn f7_empty_properties_returns_empty_summary() {
    let m = total_module();
    let result = run_totality_check(&m);
    assert!(result.property_summary.is_empty(), "No properties means empty summary");
}

#[test]
fn f7_all_property_kinds_recognized() {
    let props = vec![
        PropertyDecl {
            name: "always_prop".to_string(),
            formula: PropertyFormula::Always(Expr::Literal(LiteralValue::Bool(true))),
            directive: PropertyDirective::Assert,
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "never_prop".to_string(),
            formula: PropertyFormula::Never(Expr::Literal(LiteralValue::Bool(false))),
            directive: PropertyDirective::Assert,
            origin: None,
            span: None,
        },
        PropertyDecl {
            name: "eventually_prop".to_string(),
            formula: PropertyFormula::EventuallyWithin {
                cycles: 10,
                expr: Expr::Literal(LiteralValue::Bool(true)),
            },
            directive: PropertyDirective::Cover,
            origin: None,
            span: None,
        },
    ];
    let m = make_module_with_properties(
        vec![make_signal("a", SignalKind::Input)],
        vec![],
        vec![],
        props,
    );
    let result = run_totality_check(&m);
    let kinds: Vec<&str> = result.property_summary.iter().map(|p| p.kind.as_str()).collect();
    assert!(kinds.contains(&"always"), "Must recognize 'always' kind");
    assert!(kinds.contains(&"never"), "Must recognize 'never' kind");
    assert!(kinds.contains(&"eventually_within"), "Must recognize 'eventually_within' kind");
}

// ===========================================================================
// F8: Proof certificate serialize/deserialize roundtrip
// ===========================================================================

#[test]
fn f8_cert_serialize_deserialize_roundtrip() {
    use nasa_rust_project::cert;
    use nasa_rust_project::totality::ResourceBound;

    let cert = ProofCertificate {
        version: 1,
        program_hash: [0xAB; 32],
        resource_bound: ResourceBound {
            registers: 10,
            instructions_estimate: 50,
            guards: 3,
            max_cycles: 100,
            pass: true,
        },
        type_witnesses: vec![],
        property_verdicts: vec![],
        termination_strategy: TerminationStrategy::PrimitiveRecursive,
        termination_bound: 100,
    };

    let bytes = cert::serialize_certificate(&cert).expect("serialize must succeed");
    assert!(!bytes.is_empty(), "Serialized cert must not be empty");

    let decoded = cert::deserialize_certificate(&bytes).expect("deserialize must succeed");
    assert_eq!(decoded.version, 1);
    assert_eq!(decoded.program_hash, [0xAB; 32]);
    assert_eq!(decoded.resource_bound.registers, 10);
    assert_eq!(decoded.termination_bound, 100);
}

#[test]
fn f8_cert_magic_bytes() {
    use nasa_rust_project::cert;
    use nasa_rust_project::totality::ResourceBound;

    let cert = ProofCertificate {
        version: 1,
        program_hash: [0; 32],
        resource_bound: ResourceBound {
            registers: 1,
            instructions_estimate: 1,
            guards: 1,
            max_cycles: 1,
            pass: true,
        },
        type_witnesses: vec![],
        property_verdicts: vec![],
        termination_strategy: TerminationStrategy::PrimitiveRecursive,
        termination_bound: 1,
    };

    let bytes = cert::serialize_certificate(&cert).expect("serialize");
    // First 8 bytes must be MIRRCERT magic.
    assert!(bytes.len() >= 8, "Cert must be at least 8 bytes");
    assert_eq!(&bytes[0..8], b"MIRRCERT", "Certificate must start with MIRRCERT magic");
}

#[test]
fn f8_cert_invalid_magic_fails() {
    let bad_bytes = b"INVALID_MAGIC_PADDING_DATA_HERE!!";
    let result = nasa_rust_project::cert::deserialize_certificate(bad_bytes);
    assert!(result.is_err(), "Invalid magic must fail deserialization");
}

// ===========================================================================
// F9: TerminationStrategy variants
// ===========================================================================

#[test]
fn f9_primitive_recursive_strategy() {
    let s = TerminationStrategy::PrimitiveRecursive;
    assert_eq!(s, TerminationStrategy::PrimitiveRecursive);
}

#[test]
fn f9_static_guard_bound_strategy() {
    let s = TerminationStrategy::StaticGuardBound { max_guard_cycles: 100 };
    match s {
        TerminationStrategy::StaticGuardBound { max_guard_cycles } => {
            assert_eq!(max_guard_cycles, 100);
        }
        _ => panic!("Expected StaticGuardBound"),
    }
}

#[test]
fn f9_resource_constrained_strategy() {
    let s = TerminationStrategy::ResourceConstrained { max_instructions: 4096, max_registers: 256 };
    match s {
        TerminationStrategy::ResourceConstrained { max_instructions, max_registers } => {
            assert_eq!(max_instructions, 4096);
            assert_eq!(max_registers, 256);
        }
        _ => panic!("Expected ResourceConstrained"),
    }
}

// ===========================================================================
// F10: Pipeline integration
// ===========================================================================

#[test]
fn f10_pipeline_with_totality_flag() {
    let src = r#"module simple {
    signal enable: in bool;
    signal out_val: out bool;

    guard g {
        when enable
        for 1 cycles;
    }

    reflex r {
        on g {
            out_val = true;
        }
    }
}"#;
    let config = PipelineConfig { totality: true, ..PipelineConfig::default() };
    let result = run_pipeline(src, &config).expect("Pipeline with totality must succeed");
    assert!(result.totality_result.is_some(), "Totality result must be present when flag is set");
    let tr = result.totality_result.as_ref().unwrap();
    assert!(tr.is_total, "Simple valid module must be total");
}

#[test]
fn f10_pipeline_without_totality_flag() {
    let src = r#"module simple {
    signal enable: in bool;
    signal out_val: out bool;

    guard g {
        when enable
        for 1 cycles;
    }

    reflex r {
        on g {
            out_val = true;
        }
    }
}"#;
    let config = PipelineConfig { totality: false, ..PipelineConfig::default() };
    let result = run_pipeline(src, &config).expect("Pipeline without totality must succeed");
    assert!(result.totality_result.is_none(), "Totality result must be absent when flag is unset");
}

// ===========================================================================
// F11: TotalityError variant in MirrError
// ===========================================================================

#[test]
fn f11_totality_error_display() {
    let err = MirrError::TotalityError { message: "test error".to_string(), span: None };
    let display = format!("{}", err);
    assert!(display.contains("E1100"), "TotalityError display must contain E1100");
    assert!(display.contains("test error"), "TotalityError display must contain message");
}

#[test]
fn f11_totality_error_message_accessor() {
    let err = MirrError::TotalityError { message: "resource overflow".to_string(), span: None };
    assert_eq!(err.message(), "resource overflow");
}

#[test]
fn f11_totality_error_code() {
    let err = MirrError::TotalityError { message: "any".to_string(), span: None };
    assert_eq!(
        err.error_code(),
        Some("E1100".to_string()),
        "TotalityError must have error code E1100"
    );
}

// ===========================================================================
// Integration: parse MIRR → totality check
// ===========================================================================

#[test]
fn integration_parse_and_totality_check() {
    let src = r#"module integ_test {
    signal enable: in bool;
    signal pressure: in u16;
    signal alarm: out bool;
    signal valve: out bool;

    guard enabled {
        when enable
        for 1 cycles;
    }

    guard high_pressure {
        when pressure > 4000
        for 4 cycles;
    }

    reflex trip_alarm {
        on high_pressure {
            alarm = true;
        }
    }

    reflex open_valve {
        on enabled {
            valve = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("Must parse");
    let result = run_totality_check(&parsed.module);
    assert!(result.is_total, "Well-formed parsed module must be total");
    assert_eq!(result.resource_bound.registers, 4, "4 signals = 4 registers");
    assert_eq!(result.resource_bound.guards, 2, "2 guards");
    assert_eq!(result.temporal_bound.max_guard_cycles, 4, "Max guard is 4 cycles");
}

#[test]
fn integration_undriven_output_from_parse() {
    let src = r#"module partial {
    signal a: in bool;
    signal b: out bool;
    signal c: out bool;

    guard g {
        when a
        for 1 cycles;
    }

    reflex r {
        on g {
            b = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).expect("Must parse");
    let result = run_totality_check(&parsed.module);
    assert!(!result.is_total, "Module with undriven output c must not be total");
    assert!(
        result.output_completeness.undriven_outputs.contains(&"c".to_string()),
        "Signal c must be reported as undriven"
    );
}
