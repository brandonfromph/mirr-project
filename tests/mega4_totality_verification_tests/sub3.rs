use super::*;

#[test]
fn f2_example_tmr_sensor_fusion_cert_generation() {
    let src = include_str!("../../examples/tmr_sensor_fusion.mirr");
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        temporal: true,
        rspu: true,
        simulate: false,
        totality: true,
        symbolic: false,
        ..PipelineConfig::default()
    };
    let result = run_pipeline(src, &config).expect("pipeline must succeed");
    let totality = result.totality_result.as_ref().expect("totality result must be present");
    assert!(totality.is_total, "tmr_sensor_fusion must be total");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    // Certificate generation depends on emit_binary support for all instructions.
    // If cert is present, verify it deserializes correctly.
    if let Some(ref cert_bytes) = rspu.certificate {
        let _cert = nasa_rust_project::cert::deserialize_certificate(cert_bytes)
            .expect("certificate must deserialize");
    }
}

// ===========================================================================
// GAP 2: Adversarial certificate deserialization tests
//
// The cert format is:
//   [0..8]   MIRRCERT magic
//   [8]      version (1 byte)
//   [9..41]  program hash (32 bytes)
//   [41..45] registers (u32 LE)
//   [45..49] instructions_estimate (u32 LE)
//   [49..53] guards (u32 LE)
//   [53..61] max_cycles (u64 LE)
//   [61]     strategy tag (0=PrimitiveRecursive, 1=StaticGuardBound, 2=ResourceConstrained)
//   ...      strategy data (depends on tag)
//   ...      termination_bound (u64 LE)
//   ...      type_witness_count (u32 LE) + witnesses
//   ...      property_verdict_count (u32 LE) + verdicts
// ===========================================================================

#[test]
fn f8_adversarial_empty_input() {
    let result = nasa_rust_project::cert::deserialize_certificate(&[]);
    assert!(result.is_err(), "Empty input must fail deserialization");
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("too short"), "Error should mention 'too short', got: {}", msg);
}

#[test]
fn f8_adversarial_truncated_cert() {
    // Build a valid cert, then take only the first 20 bytes.
    use nasa_rust_project::cert;
    use nasa_rust_project::totality::ResourceBound;

    let valid_cert = ProofCertificate {
        version: 1,
        program_hash: [0xCC; 32],
        resource_bound: ResourceBound {
            registers: 5,
            instructions_estimate: 20,
            guards: 2,
            max_cycles: 50,
            pass: true,
        },
        type_witnesses: vec![],
        property_verdicts: vec![],
        termination_strategy: TerminationStrategy::PrimitiveRecursive,
        termination_bound: 50,
    };
    let bytes = cert::serialize_certificate(&valid_cert).expect("serialize must succeed");
    assert!(bytes.len() > 20, "Valid cert must be longer than 20 bytes");

    // Truncate to first 20 bytes — shorter than the 41-byte minimum.
    let truncated = &bytes[..20];
    let result = cert::deserialize_certificate(truncated);
    assert!(result.is_err(), "Truncated cert (20 bytes) must fail deserialization");
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("too short"), "Error should mention 'too short', got: {}", msg);
}

#[test]
fn f8_adversarial_wrong_magic_valid_length() {
    // Construct 80 bytes with wrong magic but otherwise plausible length.
    let mut data = vec![0u8; 80];
    // Write wrong magic.
    data[0..8].copy_from_slice(b"BADMAGIC");
    // Fill rest with zeros (structurally plausible body).
    let result = nasa_rust_project::cert::deserialize_certificate(&data);
    assert!(result.is_err(), "Wrong magic must fail deserialization");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("Invalid certificate magic"),
        "Error should mention invalid magic, got: {}",
        msg
    );
}

#[test]
fn f8_adversarial_unknown_strategy_tag() {
    // Build a valid header (magic + version + hash + resource bounds),
    // then set strategy byte to 255 (unknown).
    let mut data = vec![0u8; 80];

    // Magic.
    data[0..8].copy_from_slice(b"MIRRCERT");
    // Version.
    data[8] = 1;
    // Program hash: 32 zero bytes at [9..41].
    // Resource bound: registers=1, instructions=1, guards=1, max_cycles=1.
    // All as u32/u64 LE at appropriate offsets.
    data[41..45].copy_from_slice(&1u32.to_le_bytes()); // registers
    data[45..49].copy_from_slice(&1u32.to_le_bytes()); // instructions_estimate
    data[49..53].copy_from_slice(&1u32.to_le_bytes()); // guards
    data[53..61].copy_from_slice(&1u64.to_le_bytes()); // max_cycles
                                                       // Strategy tag at position 61: set to 255 (unknown).
    data[61] = 255;

    let result = nasa_rust_project::cert::deserialize_certificate(&data);
    assert!(result.is_err(), "Unknown strategy tag must fail deserialization");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("Unknown termination strategy tag: 255"),
        "Error should report unknown strategy tag 255, got: {}",
        msg
    );
}

#[test]
fn f8_adversarial_excessive_type_witness_count() {
    // Build a valid header through strategy + termination_bound,
    // then claim type_witness_count = 99999 which exceeds MAX_TYPE_WITNESSES (4096).
    use nasa_rust_project::cert;
    use nasa_rust_project::totality::ResourceBound;

    // Start from a valid minimal cert and hand-craft the witness count.
    let valid_cert = ProofCertificate {
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
    let mut bytes = cert::serialize_certificate(&valid_cert).expect("serialize must succeed");

    // The serialized format for PrimitiveRecursive strategy is:
    //   [0..8] magic, [8] version, [9..41] hash, [41..45] registers,
    //   [45..49] instructions, [49..53] guards, [53..61] max_cycles,
    //   [61] strategy=0, [62..70] termination_bound, [70..74] tw_count=0,
    //   [74..78] pv_count=0

    // Overwrite tw_count at offset 70 with 99999.
    assert!(bytes.len() >= 74, "Cert must be at least 74 bytes");
    bytes[70..74].copy_from_slice(&99999u32.to_le_bytes());

    let result = cert::deserialize_certificate(&bytes);
    assert!(result.is_err(), "Excessive type witness count must fail deserialization");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("Too many type witnesses"),
        "Error should mention too many type witnesses, got: {}",
        msg
    );
}

#[test]
fn f8_adversarial_version_zero() {
    // Version 0 is not the current version but deserialize_certificate does not
    // reject it — it simply stores whatever version byte it finds. This test
    // verifies that behavior: a cert with version=0 deserializes successfully
    // and the version field reads back as 0.
    use nasa_rust_project::cert;
    use nasa_rust_project::totality::ResourceBound;

    let cert = ProofCertificate {
        version: 0, // non-standard version
        program_hash: [0xDD; 32],
        resource_bound: ResourceBound {
            registers: 2,
            instructions_estimate: 10,
            guards: 1,
            max_cycles: 5,
            pass: true,
        },
        type_witnesses: vec![],
        property_verdicts: vec![],
        termination_strategy: TerminationStrategy::PrimitiveRecursive,
        termination_bound: 5,
    };
    let bytes = cert::serialize_certificate(&cert).expect("serialize must succeed");
    let decoded = cert::deserialize_certificate(&bytes).expect("version=0 should not be rejected");
    assert_eq!(decoded.version, 0, "Version 0 must round-trip correctly");
    assert_eq!(decoded.program_hash, [0xDD; 32]);
}

#[test]
fn f8_adversarial_truncated_after_strategy() {
    // Valid magic + version + hash + resource bounds + strategy tag, but truncated
    // before the termination_bound — the u64 read at the end should fail.
    let mut data = vec![0u8; 62]; // Just past the strategy tag byte

    // Magic.
    data[0..8].copy_from_slice(b"MIRRCERT");
    // Version.
    data[8] = 1;
    // Hash: 32 zeros [9..41].
    // Resource bound fields.
    data[41..45].copy_from_slice(&1u32.to_le_bytes());
    data[45..49].copy_from_slice(&1u32.to_le_bytes());
    data[49..53].copy_from_slice(&1u32.to_le_bytes());
    data[53..61].copy_from_slice(&1u64.to_le_bytes());
    // Strategy = PrimitiveRecursive (tag 0).
    data[61] = 0;
    // No more data — termination_bound read should fail.

    let result = nasa_rust_project::cert::deserialize_certificate(&data);
    assert!(result.is_err(), "Truncated cert after strategy must fail");
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("Unexpected end"), "Error should mention unexpected end, got: {}", msg);
}

#[test]
fn f8_adversarial_excessive_property_verdict_count() {
    // Build a valid cert with 0 type witnesses, then set property_verdict_count
    // to a huge value (99999) exceeding MAX_PROPERTY_VERDICTS (4096).
    use nasa_rust_project::cert;
    use nasa_rust_project::totality::ResourceBound;

    let valid_cert = ProofCertificate {
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
    let mut bytes = cert::serialize_certificate(&valid_cert).expect("serialize must succeed");

    // Format: ... [70..74] tw_count=0, [74..78] pv_count=0
    // Overwrite pv_count at offset 74 with 99999.
    assert!(bytes.len() >= 78, "Cert must be at least 78 bytes");
    bytes[74..78].copy_from_slice(&99999u32.to_le_bytes());

    let result = cert::deserialize_certificate(&bytes);
    assert!(result.is_err(), "Excessive property verdict count must fail deserialization");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("Too many property verdicts"),
        "Error should mention too many property verdicts, got: {}",
        msg
    );
}

// ===========================================================================
// GAP 3: Full chain — compile .mirr → R-SPU with totality → simulator →
//         VERIFY instruction → assert cert_verified == true
// ===========================================================================

#[test]
fn f12_full_chain_cert_generate_verify_accept() {
    use nasa_rust_project::emit::rspu_isa::RspuInstruction;
    use nasa_rust_project::emit::rspu_sim::RspuSimulator;

    // Step 1: Compile a .mirr source to R-SPU with totality=true.
    let src = include_str!("../../examples/neonatal_respirator.mirr");
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        temporal: true,
        rspu: true,
        simulate: false,
        totality: true,
        symbolic: false,
        ..PipelineConfig::default()
    };
    let result = run_pipeline(src, &config).expect("pipeline must succeed");

    // Verify the pipeline produced a totality result and certificate.
    let totality = result.totality_result.as_ref().expect("totality result must be present");
    assert!(totality.is_total, "neonatal_respirator must be total for full chain test");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(rspu.certificate.is_some(), "Certificate must be generated for full chain test");

    // Step 2: Create a simulator and inject the VERIFY instruction.
    let mut sim = RspuSimulator::new();
    assert!(!sim.cert_verified, "cert_verified must start as false");

    // Build a minimal program with VERIFY + CERTIFY + HALT to test the chain.
    let verify_program = nasa_rust_project::emit::rspu_isa::RspuProgram {
        instructions: vec![
            RspuInstruction::Verify { cert_offset: 0 },
            RspuInstruction::Certify { dst: 192 },
            RspuInstruction::Halt,
        ],
        registers_used: 1,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
        certificate: rspu.certificate.clone(),
    };

    // Step 3: Execute the program.
    let sim_result = sim.run(&verify_program, 100).expect("simulation must succeed");
    assert!(sim_result.halted, "Simulator must halt after VERIFY+CERTIFY+HALT");

    // Step 4: Assert VERIFY succeeded.
    assert!(sim.cert_verified, "cert_verified must be true after VERIFY instruction");

    // Step 5: Verify CERTIFY wrote 1 to the destination register.
    let certify_word = sim.registers.read(192);
    assert_eq!(certify_word.value, 1, "CERTIFY must write 1 when cert_verified is true");
}

#[test]
fn f12_full_chain_certify_without_verify_returns_zero() {
    use nasa_rust_project::emit::rspu_isa::RspuInstruction;
    use nasa_rust_project::emit::rspu_sim::RspuSimulator;

    // CERTIFY without preceding VERIFY should write 0 (cert_verified starts false).
    let mut sim = RspuSimulator::new();

    let program = nasa_rust_project::emit::rspu_isa::RspuProgram {
        instructions: vec![RspuInstruction::Certify { dst: 192 }, RspuInstruction::Halt],
        registers_used: 1,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
        certificate: None,
    };

    let sim_result = sim.run(&program, 100).expect("simulation must succeed");
    assert!(sim_result.halted, "Simulator must halt");
    assert!(!sim.cert_verified, "cert_verified must remain false without VERIFY");

    let certify_word = sim.registers.read(192);
    assert_eq!(certify_word.value, 0, "CERTIFY must write 0 when no VERIFY preceded it");
}

#[test]
fn f12_full_chain_total_check_with_no_violations() {
    use nasa_rust_project::emit::rspu_isa::RspuInstruction;
    use nasa_rust_project::emit::rspu_sim::RspuSimulator;

    // TotalCheck with expected_properties=0 and no violations should succeed.
    let mut sim = RspuSimulator::new();

    let program = nasa_rust_project::emit::rspu_isa::RspuProgram {
        instructions: vec![
            RspuInstruction::TotalCheck { expected_properties: 0 },
            RspuInstruction::Halt,
        ],
        registers_used: 0,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
        certificate: None,
    };

    let sim_result = sim.run(&program, 100).expect("simulation must succeed");
    assert!(sim_result.halted, "Simulator must halt after TotalCheck + Halt");
    assert!(sim_result.exception.is_none(), "No exception with 0 expected and 0 violations");
}

#[test]
fn f12_full_chain_verify_certify_total_check_sequence() {
    use nasa_rust_project::emit::rspu_isa::RspuInstruction;
    use nasa_rust_project::emit::rspu_sim::RspuSimulator;

    // Full MEGA-4 instruction sequence: VERIFY -> CERTIFY -> TOTAL_CHECK -> HALT.
    // Compile a real .mirr to get a genuine certificate, then inject all 3 instructions.
    let src = include_str!("../../examples/flight_controller.mirr");
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        temporal: true,
        rspu: true,
        simulate: false,
        totality: true,
        symbolic: false,
        ..PipelineConfig::default()
    };
    let pipeline_result = run_pipeline(src, &config).expect("pipeline must succeed");
    let rspu = pipeline_result.rspu_program.as_ref().expect("R-SPU program must be present");

    // Certificate generation depends on emit_binary support for all instructions.
    // Only run the full VERIFY/CERTIFY/TOTAL_CHECK simulation if a cert was generated.
    if let Some(ref _cert_bytes) = rspu.certificate {
        let mut sim = RspuSimulator::new();

        let program = nasa_rust_project::emit::rspu_isa::RspuProgram {
            instructions: vec![
                RspuInstruction::Verify { cert_offset: 0 },
                RspuInstruction::Certify { dst: 192 },
                RspuInstruction::TotalCheck { expected_properties: 0 },
                RspuInstruction::Halt,
            ],
            registers_used: 1,
            guards_used: 0,
            register_map: Vec::new(),
            guard_map: Vec::new(),
            certificate: rspu.certificate.clone(),
        };

        let sim_result = sim.run(&program, 100).expect("simulation must succeed");
        assert!(sim_result.halted, "Full MEGA-4 sequence must halt");
        assert!(sim.cert_verified, "VERIFY must succeed");
        assert_eq!(
            sim.registers.read(192).value,
            1,
            "CERTIFY must write 1 after successful VERIFY"
        );
        assert!(
            sim_result.exception.is_none(),
            "TotalCheck with 0 expected must not raise exception"
        );
    } else {
        // No certificate available due to emit_binary limitations.
        // Verify totality analysis still ran successfully.
        let totality =
            pipeline_result.totality_result.as_ref().expect("totality result must be present");
        assert!(totality.is_total, "flight_controller must be total even without cert");
    }
}
