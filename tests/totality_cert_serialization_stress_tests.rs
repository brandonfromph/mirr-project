//! MEGA-4 QA Stress Tests — Certificate Serialization/Deserialization Edge Cases.
//!
//! 20 stress/edge-case tests targeting bug detection in the MIRR proof certificate
//! binary format. Covers boundary values, truncation, malformed input, and
//! round-trip fidelity for extreme inputs.
//!
//! Every loop is bounded by a MAX_* constant. No recursion. No unsafe code.

#![forbid(unsafe_code)]

use nasa_rust_project::cert::{
    deserialize_certificate, serialize_certificate, ProofCertificate, PropertyVerdict,
    TerminationStrategy, TypeWitness,
};
use nasa_rust_project::totality::ResourceBound;

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA P10)
// ---------------------------------------------------------------------------

const MAX_TEST_ITERATIONS: usize = 8192;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Magic bytes for the MIRR certificate format.
const CERT_MAGIC: &[u8; 8] = b"MIRRCERT";

/// Build a minimal valid ProofCertificate with PrimitiveRecursive strategy.
fn minimal_cert() -> ProofCertificate {
    ProofCertificate {
        version: 1,
        program_hash: [0xAA; 32],
        resource_bound: ResourceBound {
            registers: 4,
            instructions_estimate: 10,
            guards: 2,
            max_cycles: 50,
            pass: true,
        },
        type_witnesses: vec![],
        property_verdicts: vec![],
        termination_strategy: TerminationStrategy::PrimitiveRecursive,
        termination_bound: 100,
    }
}

/// Push a u32 in little-endian to a byte buffer.
fn push_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

/// Push a u64 in little-endian to a byte buffer.
fn push_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

/// Push a length-prefixed string (1 byte len + bytes) to a byte buffer.
fn push_string(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(255) as u8;
    buf.push(len);
    buf.extend_from_slice(&bytes[..len as usize]);
}

/// Build a raw binary header up to and including the resource bound fields.
/// Caller must append strategy, termination bound, witnesses, verdicts.
fn build_raw_header(
    version: u8,
    hash: [u8; 32],
    reg: u32,
    instr: u32,
    guards: u32,
    cycles: u64,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(CERT_MAGIC);
    buf.push(version);
    buf.extend_from_slice(&hash);
    push_u32(&mut buf, reg);
    push_u32(&mut buf, instr);
    push_u32(&mut buf, guards);
    push_u64(&mut buf, cycles);
    buf
}

// ===========================================================================
// QA1: 255-character signal name roundtrips
// ===========================================================================

#[test]
fn qa_cert_max_length_signal_name() {
    let mut cert = minimal_cert();
    let long_name: String = {
        let mut s = String::new();
        let mut i = 0;
        let bound = MAX_TEST_ITERATIONS.min(255);
        while i < bound {
            s.push('x');
            i += 1;
        }
        s
    };
    assert_eq!(long_name.len(), 255);
    cert.type_witnesses.push(TypeWitness { name: long_name.clone(), kind: 0, width: 8, signed: 0 });

    let bytes = serialize_certificate(&cert).expect("serialize 255-char name");
    let restored = deserialize_certificate(&bytes).expect("deserialize 255-char name");
    assert_eq!(restored.type_witnesses.len(), 1);
    assert_eq!(restored.type_witnesses[0].name, long_name);
    assert_eq!(restored.type_witnesses[0].name.len(), 255);
}

// ===========================================================================
// QA2: Overlength signal name truncated to 255
// ===========================================================================

#[test]
fn qa_cert_overlength_signal_name_truncated() {
    let mut cert = minimal_cert();
    let name_300: String = {
        let mut s = String::new();
        let mut i = 0;
        let bound = MAX_TEST_ITERATIONS.min(300);
        while i < bound {
            s.push('A');
            i += 1;
        }
        s
    };
    assert_eq!(name_300.len(), 300);
    cert.type_witnesses.push(TypeWitness { name: name_300, kind: 1, width: 16, signed: 1 });

    let bytes = serialize_certificate(&cert).expect("serialize 300-char name");
    let restored = deserialize_certificate(&bytes).expect("deserialize truncated name");
    assert_eq!(restored.type_witnesses.len(), 1);
    // Name must be truncated to 255 bytes
    assert_eq!(restored.type_witnesses[0].name.len(), 255);
}

// ===========================================================================
// QA3: Empty signal name roundtrips
// ===========================================================================

#[test]
fn qa_cert_empty_signal_name() {
    let mut cert = minimal_cert();
    cert.type_witnesses.push(TypeWitness { name: String::new(), kind: 2, width: 1, signed: 0 });

    let bytes = serialize_certificate(&cert).expect("serialize empty name");
    let restored = deserialize_certificate(&bytes).expect("deserialize empty name");
    assert_eq!(restored.type_witnesses.len(), 1);
    assert_eq!(restored.type_witnesses[0].name, "");
    assert_eq!(restored.type_witnesses[0].kind, 2);
}

// ===========================================================================
// QA4: Unicode (multi-byte) signal name
// ===========================================================================

#[test]
fn qa_cert_unicode_signal_name() {
    let mut cert = minimal_cert();
    // Multi-byte UTF-8: each char is 3 bytes
    let unicode_name = "\u{2603}\u{2764}\u{263A}"; // snowman, heart, smiley
    assert!(unicode_name.len() > 3); // multi-byte
    cert.type_witnesses.push(TypeWitness {
        name: unicode_name.to_string(),
        kind: 0,
        width: 32,
        signed: 0,
    });

    let bytes = serialize_certificate(&cert).expect("serialize unicode name");
    let restored = deserialize_certificate(&bytes).expect("deserialize unicode name");
    assert_eq!(restored.type_witnesses.len(), 1);
    assert_eq!(restored.type_witnesses[0].name, unicode_name);
}

// ===========================================================================
// QA5: Exactly 4096 type witnesses (boundary)
// ===========================================================================

#[test]
fn qa_cert_max_type_witnesses_boundary() {
    let mut cert = minimal_cert();
    let mut i = 0;
    let bound = MAX_TEST_ITERATIONS.min(4096);
    while i < bound {
        cert.type_witnesses.push(TypeWitness {
            name: "s".to_string(),
            kind: 0,
            width: 1,
            signed: 0,
        });
        i += 1;
    }
    assert_eq!(cert.type_witnesses.len(), 4096);

    let bytes = serialize_certificate(&cert).expect("serialize 4096 witnesses");
    let restored = deserialize_certificate(&bytes).expect("deserialize 4096 witnesses");
    assert_eq!(restored.type_witnesses.len(), 4096);
}

// ===========================================================================
// QA6: Exactly 4096 property verdicts (boundary)
// ===========================================================================

#[test]
fn qa_cert_max_property_verdicts_boundary() {
    let mut cert = minimal_cert();
    let mut i = 0;
    let bound = MAX_TEST_ITERATIONS.min(4096);
    while i < bound {
        cert.property_verdicts.push(PropertyVerdict {
            name: "p".to_string(),
            kind: "a".to_string(),
            verified: true,
        });
        i += 1;
    }
    assert_eq!(cert.property_verdicts.len(), 4096);

    let bytes = serialize_certificate(&cert).expect("serialize 4096 verdicts");
    let restored = deserialize_certificate(&bytes).expect("deserialize 4096 verdicts");
    assert_eq!(restored.property_verdicts.len(), 4096);
}

// ===========================================================================
// QA7: All-zero program hash roundtrips
// ===========================================================================

#[test]
fn qa_cert_all_zero_program_hash() {
    let mut cert = minimal_cert();
    cert.program_hash = [0u8; 32];

    let bytes = serialize_certificate(&cert).expect("serialize zero hash");
    let restored = deserialize_certificate(&bytes).expect("deserialize zero hash");
    assert_eq!(restored.program_hash, [0u8; 32]);
}

// ===========================================================================
// QA8: u64::MAX termination bound roundtrips
// ===========================================================================

#[test]
fn qa_cert_max_u64_termination_bound() {
    let mut cert = minimal_cert();
    cert.termination_bound = u64::MAX;

    let bytes = serialize_certificate(&cert).expect("serialize u64::MAX bound");
    let restored = deserialize_certificate(&bytes).expect("deserialize u64::MAX bound");
    assert_eq!(restored.termination_bound, u64::MAX);
}

// ===========================================================================
// QA9: u32::MAX registers roundtrips
// ===========================================================================

#[test]
fn qa_cert_max_u32_registers() {
    let mut cert = minimal_cert();
    cert.resource_bound.registers = u32::MAX;

    let bytes = serialize_certificate(&cert).expect("serialize u32::MAX registers");
    let restored = deserialize_certificate(&bytes).expect("deserialize u32::MAX registers");
    assert_eq!(restored.resource_bound.registers, u32::MAX);
}

// ===========================================================================
// QA10: Version 255 roundtrips
// ===========================================================================

#[test]
fn qa_cert_version_255() {
    let mut cert = minimal_cert();
    cert.version = 255;

    let bytes = serialize_certificate(&cert).expect("serialize version 255");
    let restored = deserialize_certificate(&bytes).expect("deserialize version 255");
    assert_eq!(restored.version, 255);
}

// ===========================================================================
// QA11: ResourceConstrained with zero values
// ===========================================================================

#[test]
fn qa_cert_resource_constrained_zero_values() {
    let mut cert = minimal_cert();
    cert.termination_strategy =
        TerminationStrategy::ResourceConstrained { max_instructions: 0, max_registers: 0 };

    let bytes = serialize_certificate(&cert).expect("serialize RC zeros");
    let restored = deserialize_certificate(&bytes).expect("deserialize RC zeros");
    assert_eq!(
        restored.termination_strategy,
        TerminationStrategy::ResourceConstrained { max_instructions: 0, max_registers: 0 }
    );
}

// ===========================================================================
// QA12: StaticGuardBound with zero cycles
// ===========================================================================

#[test]
fn qa_cert_static_guard_bound_zero_cycles() {
    let mut cert = minimal_cert();
    cert.termination_strategy = TerminationStrategy::StaticGuardBound { max_guard_cycles: 0 };

    let bytes = serialize_certificate(&cert).expect("serialize SGB zero");
    let restored = deserialize_certificate(&bytes).expect("deserialize SGB zero");
    assert_eq!(
        restored.termination_strategy,
        TerminationStrategy::StaticGuardBound { max_guard_cycles: 0 }
    );
}

// ===========================================================================
// QA13: Duplicate type witness names
// ===========================================================================

#[test]
fn qa_cert_duplicate_type_witness_names() {
    let mut cert = minimal_cert();
    cert.type_witnesses.push(TypeWitness {
        name: "dup_sig".to_string(),
        kind: 0,
        width: 8,
        signed: 0,
    });
    cert.type_witnesses.push(TypeWitness {
        name: "dup_sig".to_string(),
        kind: 1,
        width: 16,
        signed: 1,
    });

    let bytes = serialize_certificate(&cert).expect("serialize duplicate names");
    let restored = deserialize_certificate(&bytes).expect("deserialize duplicate names");
    assert_eq!(restored.type_witnesses.len(), 2);
    assert_eq!(restored.type_witnesses[0].name, "dup_sig");
    assert_eq!(restored.type_witnesses[1].name, "dup_sig");
    // Distinguished by kind/width/signed
    assert_eq!(restored.type_witnesses[0].kind, 0);
    assert_eq!(restored.type_witnesses[1].kind, 1);
    assert_eq!(restored.type_witnesses[0].width, 8);
    assert_eq!(restored.type_witnesses[1].width, 16);
}

// ===========================================================================
// QA14: Exactly 41 bytes — valid header then immediate EOF after hash
// ===========================================================================

#[test]
fn qa_cert_deser_exactly_41_bytes_truncated() {
    let mut buf = Vec::new();
    buf.extend_from_slice(CERT_MAGIC); // 8 bytes
    buf.push(1); // version: 1 byte
    buf.extend_from_slice(&[0u8; 32]); // hash: 32 bytes
                                       // Total: 41 bytes — valid header but no resource bound data
    assert_eq!(buf.len(), 41);

    let result = deserialize_certificate(&buf);
    assert!(result.is_err(), "41 bytes has no resource bound, must fail");
}

// ===========================================================================
// QA15: Truncated mid-resource-bound
// ===========================================================================

#[test]
fn qa_cert_deser_truncated_mid_resource_bound() {
    let mut buf = Vec::new();
    buf.extend_from_slice(CERT_MAGIC);
    buf.push(1);
    buf.extend_from_slice(&[0u8; 32]);
    // Only first u32 of resource bound (registers)
    push_u32(&mut buf, 10);
    // Missing: instructions_estimate, guards, max_cycles
    assert_eq!(buf.len(), 45);

    let result = deserialize_certificate(&buf);
    assert!(result.is_err(), "Truncated resource bound must fail");
}

// ===========================================================================
// QA16: Strategy tag 1 (StaticGuardBound) but no data follows
// ===========================================================================

#[test]
fn qa_cert_deser_strategy_tag1_no_data() {
    let mut buf = build_raw_header(1, [0u8; 32], 1, 1, 0, 0);
    // Strategy tag 1 needs 8 more bytes for max_guard_cycles
    buf.push(1);
    // No data follows — EOF

    let result = deserialize_certificate(&buf);
    assert!(result.is_err(), "Strategy tag 1 without data must fail");
}

// ===========================================================================
// QA17: Strategy tag 2 (ResourceConstrained) with only 4 bytes (needs 8)
// ===========================================================================

#[test]
fn qa_cert_deser_strategy_tag2_partial_data() {
    let mut buf = build_raw_header(1, [0u8; 32], 1, 1, 0, 0);
    // Strategy tag 2 needs 8 bytes (4 + 4)
    buf.push(2);
    push_u32(&mut buf, 100); // max_instructions only
                             // Missing: max_registers — EOF

    let result = deserialize_certificate(&buf);
    assert!(result.is_err(), "Strategy tag 2 with partial data must fail");
}

// ===========================================================================
// QA18: Type witness count exceeds available data
// ===========================================================================

#[test]
fn qa_cert_deser_type_witness_count_exceeds_data() {
    let mut buf = build_raw_header(1, [0u8; 32], 1, 1, 0, 0);
    buf.push(0); // PrimitiveRecursive
    push_u64(&mut buf, 0); // termination bound
                           // Claim 10 type witnesses but provide only 1 worth of data
    push_u32(&mut buf, 10);
    // One witness: 1 byte name_len + 1 byte name + 1 byte kind + 4 bytes width + 1 byte signed = 8 bytes
    push_string(&mut buf, "x");
    buf.push(0); // kind
    push_u32(&mut buf, 8); // width
    buf.push(0); // signed
                 // No more data for the remaining 9 witnesses

    let result = deserialize_certificate(&buf);
    assert!(result.is_err(), "Claimed 10 witnesses but only 1 provided — must fail");
}

// ===========================================================================
// QA19: String length byte exceeds remaining data
// ===========================================================================

#[test]
fn qa_cert_deser_string_length_exceeds_remaining() {
    let mut buf = build_raw_header(1, [0u8; 32], 1, 1, 0, 0);
    buf.push(0); // PrimitiveRecursive
    push_u64(&mut buf, 0); // termination bound
                           // 1 type witness
    push_u32(&mut buf, 1);
    // String length byte claims 200, but only 10 bytes of actual string data remain
    buf.push(200);
    let mut i = 0;
    let bound = MAX_TEST_ITERATIONS.min(10);
    while i < bound {
        buf.push(b'z');
        i += 1;
    }
    // EOF after 10 bytes — but length says 200

    let result = deserialize_certificate(&buf);
    assert!(result.is_err(), "String length 200 with only 10 bytes remaining must fail");
}

// ===========================================================================
// QA20: pass field always true on deserialization
// ===========================================================================

#[test]
fn qa_cert_pass_field_always_true_on_deser() {
    // Serialize a cert with pass=false
    let mut cert = minimal_cert();
    cert.resource_bound.pass = false;

    let bytes = serialize_certificate(&cert).expect("serialize pass=false cert");
    let restored = deserialize_certificate(&bytes).expect("deserialize pass=false cert");

    // The deserializer hardcodes pass=true regardless of the original value
    // (pass is not serialized in the binary format)
    assert!(restored.resource_bound.pass, "Deserialized cert must always have pass=true");
}
