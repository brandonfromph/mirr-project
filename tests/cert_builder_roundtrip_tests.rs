//! Proof certificate builder and serialization tests.
//!
//! Exercises `src/cert/mod.rs::build_certificate` and related paths:
//!   - build_certificate with Input/Output/Internal signals
//!   - build_certificate with property summaries → PrimitiveRecursive strategy
//!   - build_certificate with temporal guard → StaticGuardBound strategy
//!   - cert/serialize.rs: overflow guard for MAX_CERT_SIZE

#![forbid(unsafe_code)]

use mirrc::cert::{
    build_certificate, serialize_certificate, deserialize_certificate,
    ProofCertificate, TerminationStrategy, TypeWitness, PropertyVerdict,
};
use mirrc::totality::{
    TotalityResult, ResourceBound, OutputCompletenessResult,
    GuardCoverageResult, TemporalBoundResult, AcyclicityResult,
    PropertySummary,
};
use mirrc::ecs::Registry;
use mirrc::ast::types::{SignalKind, SignalType, ExtendedType};
use mirrc::ecs::components::{EntityKind, KindComponent, NameComponent, TypeComponent};

fn make_totality(max_guard_cycles: u64, properties: Vec<PropertySummary>) -> TotalityResult {
    TotalityResult {
        resource_bound: ResourceBound {
            registers: 8,
            instructions_estimate: 20,
            guards: 2,
            max_cycles: 100,
            pass: true,
        },
        output_completeness: OutputCompletenessResult {
            undriven_outputs: vec![],
            pass: true,
        },
        guard_coverage: GuardCoverageResult {
            covered_outputs: 2,
            total_outputs: 2,
            pass: true,
        },
        temporal_bound: TemporalBoundResult {
            max_guard_cycles,
            max_prev_delay: 3,
            worst_case_latency: max_guard_cycles + 3,
            pass: true,
        },
        acyclicity: AcyclicityResult {
            pass: true,
            cycle_witness: None,
        },
        property_summary: properties,
        is_total: true,
    }
}

fn registry_with_signals() -> Registry {
    let mut reg = Registry::new();

    // Input signal
    let e0 = reg.create_entity("clk", KindComponent(EntityKind::SIGNAL(SignalKind::Input)));
    reg.types[e0.0 as usize] = Some(TypeComponent(ExtendedType::from(SignalType::Bool)));

    // Output signal
    let e1 = reg.create_entity("out_data", KindComponent(EntityKind::SIGNAL(SignalKind::Output)));
    reg.types[e1.0 as usize] = Some(TypeComponent(ExtendedType::from(SignalType::Unsigned(8))));

    // Internal signal (signed)
    let e2 = reg.create_entity("temp", KindComponent(EntityKind::SIGNAL(SignalKind::Internal)));
    reg.types[e2.0 as usize] = Some(TypeComponent(ExtendedType::from(SignalType::Signed(16))));

    reg
}

// -----------------------------------------------------------------------
// build_certificate with signals and StaticGuardBound
// -----------------------------------------------------------------------
#[test]
fn build_certificate_with_guard_cycles_uses_static_guard_bound() {
    let reg = registry_with_signals();
    let totality = make_totality(50, vec![
        PropertySummary { name: "stability".to_string(), kind: "always".to_string() },
    ]);
    let binary: Vec<u64> = vec![0xDEAD, 0xBEEF, 0xCAFE];

    let cert = build_certificate(&totality, &binary, &reg);

    // Should be StaticGuardBound
    assert_eq!(
        cert.termination_strategy,
        TerminationStrategy::StaticGuardBound { max_guard_cycles: 50 }
    );
    assert_eq!(cert.termination_bound, 53); // 50 + 3

    // Type witnesses should have 3 signals
    assert_eq!(cert.type_witnesses.len(), 3);
    assert_eq!(cert.type_witnesses[0].name, "clk");
    assert_eq!(cert.type_witnesses[0].kind, 0); // Input
    assert_eq!(cert.type_witnesses[0].width, 1);
    assert_eq!(cert.type_witnesses[0].signed, 0);

    assert_eq!(cert.type_witnesses[1].name, "out_data");
    assert_eq!(cert.type_witnesses[1].kind, 1); // Output
    assert_eq!(cert.type_witnesses[1].width, 8);

    assert_eq!(cert.type_witnesses[2].name, "temp");
    assert_eq!(cert.type_witnesses[2].kind, 2); // Internal
    assert_eq!(cert.type_witnesses[2].width, 16);
    assert_eq!(cert.type_witnesses[2].signed, 1); // Signed

    // Property verdicts
    assert_eq!(cert.property_verdicts.len(), 1);
    assert_eq!(cert.property_verdicts[0].name, "stability");
    assert!(cert.property_verdicts[0].verified);
}

// -----------------------------------------------------------------------
// build_certificate with zero guard cycles → PrimitiveRecursive
// -----------------------------------------------------------------------
#[test]
fn build_certificate_zero_guard_cycles_uses_primitive_recursive() {
    let reg = registry_with_signals();
    let totality = make_totality(0, vec![]);
    let binary: Vec<u64> = vec![0x1234];

    let cert = build_certificate(&totality, &binary, &reg);

    assert_eq!(cert.termination_strategy, TerminationStrategy::PrimitiveRecursive);
    assert_eq!(cert.termination_bound, 3); // 0 + 3
}

// -----------------------------------------------------------------------
// Full roundtrip: build → serialize → deserialize
// -----------------------------------------------------------------------
#[test]
fn build_certificate_roundtrip_through_serialization() {
    let reg = registry_with_signals();
    let totality = make_totality(100, vec![
        PropertySummary { name: "p1".to_string(), kind: "always".to_string() },
        PropertySummary { name: "p2".to_string(), kind: "never".to_string() },
    ]);
    let binary: Vec<u64> = vec![1, 2, 3, 4, 5];

    let cert = build_certificate(&totality, &binary, &reg);
    let bytes = serialize_certificate(&cert).expect("serialize should succeed");
    let restored = deserialize_certificate(&bytes).expect("deserialize should succeed");

    assert_eq!(restored.version, cert.version);
    assert_eq!(restored.program_hash, cert.program_hash);
    assert_eq!(restored.type_witnesses.len(), cert.type_witnesses.len());
    assert_eq!(restored.property_verdicts.len(), 2);
    assert_eq!(restored.property_verdicts[0].name, "p1");
    assert_eq!(restored.property_verdicts[1].name, "p2");
    assert_eq!(restored.termination_strategy, cert.termination_strategy);
}

// -----------------------------------------------------------------------
// Multiple properties
// -----------------------------------------------------------------------
#[test]
fn build_certificate_multiple_properties() {
    let reg = Registry::new();
    let mut props = Vec::new();
    for i in 0..10 {
        props.push(PropertySummary {
            name: format!("prop_{}", i),
            kind: if i % 2 == 0 { "always".to_string() } else { "never".to_string() },
        });
    }
    let totality = make_totality(0, props);
    let cert = build_certificate(&totality, &[], &reg);
    assert_eq!(cert.property_verdicts.len(), 10);
    for pv in &cert.property_verdicts {
        assert!(pv.verified); // is_total = true
    }
}
