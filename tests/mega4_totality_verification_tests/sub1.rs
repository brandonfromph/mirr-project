use super::*;

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
    use mirrc::cert;
    use mirrc::totality::ResourceBound;

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
    use mirrc::cert;
    use mirrc::totality::ResourceBound;

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
    let result = mirrc::cert::deserialize_certificate(bad_bytes);
    assert!(result.is_err(), "Invalid magic must fail deserialization");
}

// ===========================================================================
// F9: TerminationStrategy variants
// ===========================================================================
