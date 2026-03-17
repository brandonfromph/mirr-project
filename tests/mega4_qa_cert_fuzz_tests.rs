#![forbid(unsafe_code)]

use nasa_rust_project::cert::{
    deserialize_certificate, serialize_certificate, ProofCertificate, PropertyVerdict,
    TerminationStrategy, TypeWitness,
};
use nasa_rust_project::totality::ResourceBound;

const MAX_FUZZ_ITERATIONS: usize = 8192;

fn make_cert(strategy: TerminationStrategy) -> ProofCertificate {
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
        termination_strategy: strategy,
        termination_bound: 100,
    }
}

fn push_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn push_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

fn build_raw_cert_primitive(
    version: u8,
    hash: [u8; 32],
    reg: u32,
    instr: u32,
    guards: u32,
    cycles: u64,
    term_bound: u64,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"MIRRCERT");
    buf.push(version);
    buf.extend_from_slice(&hash);
    push_u32(&mut buf, reg);
    push_u32(&mut buf, instr);
    push_u32(&mut buf, guards);
    push_u64(&mut buf, cycles);
    buf.push(0); // PrimitiveRecursive tag
    push_u64(&mut buf, term_bound);
    push_u32(&mut buf, 0); // 0 type witnesses
    push_u32(&mut buf, 0); // 0 property verdicts
    buf
}

// Test 1: Version 0 roundtrip
#[test]
fn qa_cert_fuzz_version_zero() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.version = 0;
    let bytes = serialize_certificate(&cert).expect("serialize version=0");
    let restored = deserialize_certificate(&bytes).expect("deserialize version=0");
    assert_eq!(restored.version, 0, "Version 0 must survive roundtrip");
}

// Test 2: Version 255 roundtrip
#[test]
fn qa_cert_fuzz_version_255() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.version = 255;
    let bytes = serialize_certificate(&cert).expect("serialize version=255");
    let restored = deserialize_certificate(&bytes).expect("deserialize version=255");
    assert_eq!(restored.version, 255, "Version 255 must survive roundtrip");
}

// Test 3: All-zero program hash roundtrip
#[test]
fn qa_cert_fuzz_all_zero_hash() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.program_hash = [0x00; 32];
    let bytes = serialize_certificate(&cert).expect("serialize all-zero hash");
    let restored = deserialize_certificate(&bytes).expect("deserialize all-zero hash");
    assert_eq!(restored.program_hash, [0x00; 32], "All-zero hash must survive roundtrip");
}

// Test 4: All-0xFF program hash roundtrip
#[test]
fn qa_cert_fuzz_all_ff_hash() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.program_hash = [0xFF; 32];
    let bytes = serialize_certificate(&cert).expect("serialize all-0xFF hash");
    let restored = deserialize_certificate(&bytes).expect("deserialize all-0xFF hash");
    assert_eq!(restored.program_hash, [0xFF; 32], "All-0xFF hash must survive roundtrip");
}

// Test 5: Alternating 0xAA/0x55 hash pattern roundtrip
#[test]
fn qa_cert_fuzz_alternating_hash() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    let mut hash = [0u8; 32];
    let mut i = 0;
    let bound32 = MAX_FUZZ_ITERATIONS.min(32);
    while i < bound32 {
        hash[i] = if i % 2 == 0 { 0xAA } else { 0x55 };
        i += 1;
    }
    cert.program_hash = hash;
    let bytes = serialize_certificate(&cert).expect("serialize alternating hash");
    let restored = deserialize_certificate(&bytes).expect("deserialize alternating hash");
    let mut j = 0;
    while j < bound32 {
        let expected = if j % 2 == 0 { 0xAA } else { 0x55 };
        assert_eq!(
            restored.program_hash[j], expected,
            "Hash byte {} mismatch: expected 0x{:02X}, got 0x{:02X}",
            j, expected, restored.program_hash[j]
        );
        j += 1;
    }
}

// Test 6: Max u32 registers roundtrip
#[test]
fn qa_cert_fuzz_max_u32_registers() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.resource_bound.registers = u32::MAX;
    cert.resource_bound.instructions_estimate = u32::MAX;
    cert.resource_bound.guards = u32::MAX;
    let bytes = serialize_certificate(&cert).expect("serialize max u32 registers");
    let restored = deserialize_certificate(&bytes).expect("deserialize max u32 registers");
    assert_eq!(restored.resource_bound.registers, u32::MAX, "registers=u32::MAX must roundtrip");
    assert_eq!(
        restored.resource_bound.instructions_estimate,
        u32::MAX,
        "instructions_estimate=u32::MAX must roundtrip"
    );
    assert_eq!(restored.resource_bound.guards, u32::MAX, "guards=u32::MAX must roundtrip");
}

// Test 7: Max u64 max_cycles roundtrip
#[test]
fn qa_cert_fuzz_max_u64_max_cycles() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.resource_bound.max_cycles = u64::MAX;
    let bytes = serialize_certificate(&cert).expect("serialize max u64 max_cycles");
    let restored = deserialize_certificate(&bytes).expect("deserialize max u64 max_cycles");
    assert_eq!(restored.resource_bound.max_cycles, u64::MAX, "max_cycles=u64::MAX must roundtrip");
}

// Test 8: All resource bounds zero roundtrip
#[test]
fn qa_cert_fuzz_zero_resources() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.resource_bound.registers = 0;
    cert.resource_bound.instructions_estimate = 0;
    cert.resource_bound.guards = 0;
    cert.resource_bound.max_cycles = 0;
    cert.termination_bound = 0;
    let bytes = serialize_certificate(&cert).expect("serialize zero resources");
    let restored = deserialize_certificate(&bytes).expect("deserialize zero resources");
    assert_eq!(restored.resource_bound.registers, 0);
    assert_eq!(restored.resource_bound.instructions_estimate, 0);
    assert_eq!(restored.resource_bound.guards, 0);
    assert_eq!(restored.resource_bound.max_cycles, 0);
    assert_eq!(restored.termination_bound, 0);
}

// Test 9: Single type witness with width=u32::MAX
#[test]
fn qa_cert_fuzz_single_type_witness_max_width() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.type_witnesses.push(TypeWitness {
        name: "max_w".to_string(),
        kind: 0,
        width: u32::MAX,
        signed: 0,
    });
    let bytes = serialize_certificate(&cert).expect("serialize max-width witness");
    let restored = deserialize_certificate(&bytes).expect("deserialize max-width witness");
    assert_eq!(restored.type_witnesses.len(), 1);
    assert_eq!(restored.type_witnesses[0].name, "max_w");
    assert_eq!(restored.type_witnesses[0].width, u32::MAX, "width=u32::MAX must roundtrip");
}

// Test 10: Type witnesses with all kind values (0, 1, 2)
#[test]
fn qa_cert_fuzz_type_witness_all_kinds() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.type_witnesses.push(TypeWitness { name: "inp".to_string(), kind: 0, width: 1, signed: 0 });
    cert.type_witnesses.push(TypeWitness {
        name: "outp".to_string(),
        kind: 1,
        width: 8,
        signed: 0,
    });
    cert.type_witnesses.push(TypeWitness {
        name: "intl".to_string(),
        kind: 2,
        width: 16,
        signed: 1,
    });
    let bytes = serialize_certificate(&cert).expect("serialize all-kinds witnesses");
    let restored = deserialize_certificate(&bytes).expect("deserialize all-kinds witnesses");
    assert_eq!(restored.type_witnesses.len(), 3);
    assert_eq!(restored.type_witnesses[0].kind, 0, "kind=0 input");
    assert_eq!(restored.type_witnesses[1].kind, 1, "kind=1 output");
    assert_eq!(restored.type_witnesses[2].kind, 2, "kind=2 internal");
    assert_eq!(restored.type_witnesses[0].name, "inp");
    assert_eq!(restored.type_witnesses[1].name, "outp");
    assert_eq!(restored.type_witnesses[2].name, "intl");
}

// Test 11: Type witnesses with signed=0 and signed=1
#[test]
fn qa_cert_fuzz_type_witness_signed_and_unsigned() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.type_witnesses.push(TypeWitness {
        name: "usig".to_string(),
        kind: 0,
        width: 32,
        signed: 0,
    });
    cert.type_witnesses.push(TypeWitness {
        name: "ssig".to_string(),
        kind: 1,
        width: 32,
        signed: 1,
    });
    let bytes = serialize_certificate(&cert).expect("serialize signed/unsigned witnesses");
    let restored = deserialize_certificate(&bytes).expect("deserialize signed/unsigned witnesses");
    assert_eq!(restored.type_witnesses.len(), 2);
    assert_eq!(restored.type_witnesses[0].signed, 0, "unsigned witness must have signed=0");
    assert_eq!(restored.type_witnesses[1].signed, 1, "signed witness must have signed=1");
}

// Test 12: Property verdict with empty name
#[test]
fn qa_cert_fuzz_empty_property_name() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.property_verdicts.push(PropertyVerdict {
        name: String::new(),
        kind: "always".to_string(),
        verified: true,
    });
    let bytes = serialize_certificate(&cert).expect("serialize empty-name verdict");
    let restored = deserialize_certificate(&bytes).expect("deserialize empty-name verdict");
    assert_eq!(restored.property_verdicts.len(), 1);
    assert_eq!(
        restored.property_verdicts[0].name, "",
        "Empty property name must roundtrip as empty string"
    );
    assert!(restored.property_verdicts[0].verified);
}

// Test 13: Type witness name exactly 255 chars long
#[test]
fn qa_cert_fuzz_255_char_witness_name() {
    let long_name: String = {
        let mut s = String::new();
        let mut i = 0;
        let bound = MAX_FUZZ_ITERATIONS.min(255);
        while i < bound {
            s.push('W');
            i += 1;
        }
        s
    };
    assert_eq!(long_name.len(), 255);
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.type_witnesses.push(TypeWitness {
        name: long_name.clone(),
        kind: 2,
        width: 64,
        signed: 1,
    });
    let bytes = serialize_certificate(&cert).expect("serialize 255-char witness name");
    let restored = deserialize_certificate(&bytes).expect("deserialize 255-char witness name");
    assert_eq!(restored.type_witnesses.len(), 1);
    assert_eq!(
        restored.type_witnesses[0].name.len(),
        255,
        "255-char name must preserve exact length"
    );
    assert_eq!(restored.type_witnesses[0].name, long_name);
}

// Test 14: Multiple verdicts with mixed verified values
#[test]
fn qa_cert_fuzz_multiple_verdicts_mixed() {
    let mut cert = make_cert(TerminationStrategy::PrimitiveRecursive);
    cert.property_verdicts.push(PropertyVerdict {
        name: "prop_a".to_string(),
        kind: "always".to_string(),
        verified: true,
    });
    cert.property_verdicts.push(PropertyVerdict {
        name: "prop_b".to_string(),
        kind: "eventually".to_string(),
        verified: false,
    });
    cert.property_verdicts.push(PropertyVerdict {
        name: "prop_c".to_string(),
        kind: "invariant".to_string(),
        verified: true,
    });
    let bytes = serialize_certificate(&cert).expect("serialize mixed verdicts");
    let restored = deserialize_certificate(&bytes).expect("deserialize mixed verdicts");
    assert_eq!(restored.property_verdicts.len(), 3);
    assert!(restored.property_verdicts[0].verified, "prop_a must be verified=true");
    assert!(!restored.property_verdicts[1].verified, "prop_b must be verified=false");
    assert!(restored.property_verdicts[2].verified, "prop_c must be verified=true");
    assert_eq!(restored.property_verdicts[0].name, "prop_a");
    assert_eq!(restored.property_verdicts[1].kind, "eventually");
    assert_eq!(restored.property_verdicts[2].name, "prop_c");
}

// Test 15: StaticGuardBound with max_guard_cycles=0
#[test]
fn qa_cert_fuzz_static_guard_bound_zero() {
    let cert = make_cert(TerminationStrategy::StaticGuardBound { max_guard_cycles: 0 });
    let bytes = serialize_certificate(&cert).expect("serialize SGB zero");
    let restored = deserialize_certificate(&bytes).expect("deserialize SGB zero");
    assert_eq!(
        restored.termination_strategy,
        TerminationStrategy::StaticGuardBound { max_guard_cycles: 0 },
        "StaticGuardBound with max_guard_cycles=0 must roundtrip"
    );
}

// Test 16: StaticGuardBound with max_guard_cycles=u64::MAX
#[test]
fn qa_cert_fuzz_static_guard_bound_max() {
    let cert = make_cert(TerminationStrategy::StaticGuardBound { max_guard_cycles: u64::MAX });
    let bytes = serialize_certificate(&cert).expect("serialize SGB max");
    let restored = deserialize_certificate(&bytes).expect("deserialize SGB max");
    assert_eq!(
        restored.termination_strategy,
        TerminationStrategy::StaticGuardBound { max_guard_cycles: u64::MAX },
        "StaticGuardBound with max_guard_cycles=u64::MAX must roundtrip"
    );
}

// Test 17: ResourceConstrained with zero values
#[test]
fn qa_cert_fuzz_resource_constrained_zeros() {
    let cert = make_cert(TerminationStrategy::ResourceConstrained {
        max_instructions: 0,
        max_registers: 0,
    });
    let bytes = serialize_certificate(&cert).expect("serialize RC zeros");
    let restored = deserialize_certificate(&bytes).expect("deserialize RC zeros");
    assert_eq!(
        restored.termination_strategy,
        TerminationStrategy::ResourceConstrained { max_instructions: 0, max_registers: 0 },
        "ResourceConstrained with zeros must roundtrip"
    );
}

// Test 18: ResourceConstrained with u32::MAX values
#[test]
fn qa_cert_fuzz_resource_constrained_maxes() {
    let cert = make_cert(TerminationStrategy::ResourceConstrained {
        max_instructions: u32::MAX,
        max_registers: u32::MAX,
    });
    let bytes = serialize_certificate(&cert).expect("serialize RC maxes");
    let restored = deserialize_certificate(&bytes).expect("deserialize RC maxes");
    assert_eq!(
        restored.termination_strategy,
        TerminationStrategy::ResourceConstrained {
            max_instructions: u32::MAX,
            max_registers: u32::MAX,
        },
        "ResourceConstrained with u32::MAX fields must roundtrip"
    );
}

// Test 19: Larger certificate with 10 witnesses + 5 verdicts
#[test]
fn qa_cert_fuzz_roundtrip_10_witnesses_5_verdicts() {
    let mut cert = make_cert(TerminationStrategy::StaticGuardBound { max_guard_cycles: 9999 });
    cert.version = 42;
    cert.program_hash = [0xDE; 32];
    cert.resource_bound.registers = 256;
    cert.resource_bound.instructions_estimate = 1024;
    cert.resource_bound.guards = 16;
    cert.resource_bound.max_cycles = 500_000;
    cert.termination_bound = 999_999;

    let mut wi = 0;
    let wi_bound = MAX_FUZZ_ITERATIONS.min(10);
    while wi < wi_bound {
        let name_char = (b'a' + (wi as u8 % 26)) as char;
        let mut name = String::new();
        let mut ci = 0;
        while ci < MAX_FUZZ_ITERATIONS && ci < (wi + 1) {
            name.push(name_char);
            ci += 1;
        }
        cert.type_witnesses.push(TypeWitness {
            name,
            kind: (wi % 3) as u8,
            width: ((wi + 1) * 8) as u32,
            signed: (wi % 2) as u8,
        });
        wi += 1;
    }

    let mut vi = 0;
    let vi_bound = MAX_FUZZ_ITERATIONS.min(5);
    while vi < vi_bound {
        cert.property_verdicts.push(PropertyVerdict {
            name: format!("prop_{}", vi),
            kind: format!("kind_{}", vi),
            verified: vi % 2 == 0,
        });
        vi += 1;
    }

    assert_eq!(cert.type_witnesses.len(), 10);
    assert_eq!(cert.property_verdicts.len(), 5);

    let bytes = serialize_certificate(&cert).expect("serialize large cert");
    let restored = deserialize_certificate(&bytes).expect("deserialize large cert");

    assert_eq!(restored.version, 42);
    assert_eq!(restored.program_hash, [0xDE; 32]);
    assert_eq!(restored.resource_bound.registers, 256);
    assert_eq!(restored.resource_bound.instructions_estimate, 1024);
    assert_eq!(restored.resource_bound.guards, 16);
    assert_eq!(restored.resource_bound.max_cycles, 500_000);
    assert_eq!(restored.termination_bound, 999_999);
    assert_eq!(
        restored.termination_strategy,
        TerminationStrategy::StaticGuardBound { max_guard_cycles: 9999 }
    );
    assert_eq!(restored.type_witnesses.len(), 10);
    assert_eq!(restored.property_verdicts.len(), 5);

    let mut check_wi = 0;
    let check_wi_bound = MAX_FUZZ_ITERATIONS.min(10);
    while check_wi < check_wi_bound {
        assert_eq!(
            restored.type_witnesses[check_wi].kind,
            (check_wi % 3) as u8,
            "Witness {} kind mismatch",
            check_wi
        );
        assert_eq!(
            restored.type_witnesses[check_wi].width,
            ((check_wi + 1) * 8) as u32,
            "Witness {} width mismatch",
            check_wi
        );
        assert_eq!(
            restored.type_witnesses[check_wi].signed,
            (check_wi % 2) as u8,
            "Witness {} signed mismatch",
            check_wi
        );
        check_wi += 1;
    }

    let mut check_vi = 0;
    let check_vi_bound = MAX_FUZZ_ITERATIONS.min(5);
    while check_vi < check_vi_bound {
        assert_eq!(restored.property_verdicts[check_vi].name, format!("prop_{}", check_vi));
        assert_eq!(restored.property_verdicts[check_vi].kind, format!("kind_{}", check_vi));
        assert_eq!(
            restored.property_verdicts[check_vi].verified,
            check_vi % 2 == 0,
            "Verdict {} verified mismatch",
            check_vi
        );
        check_vi += 1;
    }
}

// Test 20: Raw bytes with wrong magic -- deserialize must fail
#[test]
fn qa_cert_fuzz_raw_bytes_magic_mismatch() {
    let mut raw = build_raw_cert_primitive(1, [0xBB; 32], 4, 10, 2, 50, 100);
    assert!(deserialize_certificate(&raw).is_ok(), "Valid raw cert must parse before corruption");
    raw[0] = b'X';
    let result = deserialize_certificate(&raw);
    assert!(result.is_err(), "Corrupted magic (first byte) must fail deserialization");
    raw[0] = b'M';
    raw[7] = b'Z'; // 'T' -> 'Z'
    let result2 = deserialize_certificate(&raw);
    assert!(result2.is_err(), "Corrupted magic (last byte) must fail deserialization");
    let mut zeroed_magic = raw.clone();
    let mut zi = 0;
    let zi_bound = MAX_FUZZ_ITERATIONS.min(8);
    while zi < zi_bound {
        zeroed_magic[zi] = 0;
        zi += 1;
    }
    let result3 = deserialize_certificate(&zeroed_magic);
    assert!(result3.is_err(), "All-zero magic must fail deserialization");
}
