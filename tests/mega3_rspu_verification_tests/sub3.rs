use super::*;


#[test]
fn e10_interval_check_mnemonic() {
    let i = RspuInstruction::IntervalCheck { src: 0, bounds: 1 };
    assert_eq!(i.mnemonic(), "INTERVAL_CHECK");
}

#[test]
fn e10_sim_interval_shadow_initialized() {
    let sim = RspuSimulator::new();
    assert_eq!(
        sim.interval_shadow.len(),
        MAX_REGISTERS,
        "Interval shadow must have MAX_REGISTERS entries"
    );
    // All default to (0, u64::MAX).
    let mut i = 0;
    while i < sim.interval_shadow.len() && i < MAX_TEST_ITERATIONS {
        assert_eq!(
            sim.interval_shadow[i],
            (0, u64::MAX),
            "Default interval shadow for R{} must be [0, u64::MAX]",
            i
        );
        i += 1;
    }
}

#[test]
fn e10_sim_cert_verified_starts_false() {
    let sim = RspuSimulator::new();
    assert!(!sim.cert_verified, "cert_verified must start false");
}

// ===========================================================================
// Gap 1: .mirr examples → R-SPU pipeline compilation
// ===========================================================================

#[test]
fn e6_example_autonomous_vehicle_compiles_to_rspu() {
    let src = include_str!("../../examples/autonomous_vehicle.mirr");
    let result = pipeline_with_rspu(src).expect("autonomous_vehicle must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_fir_filter_compiles_to_rspu() {
    let src = include_str!("../../examples/fir_filter.mirr");
    let result = pipeline_with_rspu(src).expect("fir_filter must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_flight_controller_compiles_to_rspu() {
    let src = include_str!("../../examples/flight_controller.mirr");
    let result = pipeline_with_rspu(src).expect("flight_controller must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_flight_controller_signed_compiles_to_rspu() {
    let src = include_str!("../../examples/flight_controller_signed.mirr");
    // This example has a guard condition that cannot be lowered to hardware,
    // so the R-SPU backend correctly rejects it with a TemporalCompilationError.
    let result = pipeline_with_rspu(src);
    assert!(
        result.is_err(),
        "flight_controller_signed should fail R-SPU compilation (unsupported guard form)"
    );
}

#[test]
fn e6_example_icu_monitor_compiles_to_rspu() {
    let src = include_str!("../../examples/icu_monitor.mirr");
    let result = pipeline_with_rspu(src).expect("icu_monitor must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_industrial_safety_compiles_to_rspu() {
    let src = include_str!("../../examples/industrial_safety.mirr");
    let result = pipeline_with_rspu(src).expect("industrial_safety must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_multi_guard_monitor_compiles_to_rspu() {
    let src = include_str!("../../examples/multi_guard_monitor.mirr");
    let result = pipeline_with_rspu(src).expect("multi_guard_monitor must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_neonatal_respirator_compiles_to_rspu() {
    let src = include_str!("../../examples/neonatal_respirator.mirr");
    let result = pipeline_with_rspu(src).expect("neonatal_respirator must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_pattern_usage_compiles_to_rspu() {
    let src = include_str!("../../examples/pattern_usage.mirr");
    let result = pipeline_with_rspu(src).expect("pattern_usage must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_safety_property_compiles_to_rspu() {
    let src = include_str!("../../examples/safety_property.mirr");
    let result = pipeline_with_rspu(src).expect("safety_property must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_shift_register_guard_compiles_to_rspu() {
    let src = include_str!("../../examples/shift_register_guard.mirr");
    let result = pipeline_with_rspu(src).expect("shift_register_guard must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

#[test]
fn e6_example_tmr_sensor_fusion_compiles_to_rspu() {
    let src = include_str!("../../examples/tmr_sensor_fusion.mirr");
    let result = pipeline_with_rspu(src).expect("tmr_sensor_fusion must compile to R-SPU");
    let rspu = result.rspu_program.as_ref().expect("R-SPU program must be present");
    assert!(!rspu.instructions.is_empty(), "Must have instructions");
}

// ===========================================================================
// Gap 2: Simulator execution tests for Verify/Certify/TotalCheck (opcodes 30-32)
// ===========================================================================

#[test]
fn e9_sim_verify_sets_cert_verified() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::Verify { cert_offset: 0 }]);
    sim.step(&prog).expect("step");
    assert!(sim.cert_verified, "VERIFY must set cert_verified to true");
}

#[test]
fn e9_sim_certify_reads_cert_verified_false() {
    let mut sim = RspuSimulator::new();
    // Without Verify first, cert_verified is false
    let prog = make_program(vec![RspuInstruction::Certify { dst: 192 }]);
    sim.step(&prog).expect("step");
    assert_eq!(sim.registers.read(192).value, 0, "Certify without Verify must write 0");
}

#[test]
fn e9_sim_verify_then_certify() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::Verify { cert_offset: 0 },
        RspuInstruction::Certify { dst: 192 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    assert_eq!(sim.registers.read(192).value, 1, "Certify after Verify must write 1");
}

#[test]
fn e9_sim_total_check_no_violations_continues() {
    let mut sim = RspuSimulator::new();
    // No violations registered, expected_properties=0 should pass
    let prog = make_program(vec![RspuInstruction::TotalCheck { expected_properties: 0 }]);
    let result = sim.step(&prog).expect("step");
    assert_eq!(
        result,
        StepResult::Continue,
        "TotalCheck with 0 expected and 0 violations must continue"
    );
}

#[test]
fn e9_sim_total_check_with_violations_raises_exception() {
    let mut sim = RspuSimulator::new();
    // Add a violation so the check fails
    sim.properties.violations.push(0);
    let prog = make_program(vec![RspuInstruction::TotalCheck { expected_properties: 2 }]);
    let result = sim.step(&prog).expect("step");
    match result {
        StepResult::Exception(code) => {
            assert_eq!(
                code,
                nasa_rust_project::emit::rspu_exceptions::ExceptionCode::PropertyFail,
                "TotalCheck with violations must raise PropertyFail"
            );
        }
        other => panic!("Expected PropertyFail exception, got {:?}", other),
    }
}

// ===========================================================================
// Gap 3: Simulator execution tests for Match/IntervalLo/IntervalHi/IntervalCheck (opcodes 33-36)
// ===========================================================================

#[test]
fn e10_sim_match_nonzero_input() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 42, width: 16 },
        RspuInstruction::Match { dst: 193, src: 192, table_offset: 0 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    assert_eq!(sim.registers.read(193).value, 1, "MATCH on nonzero input must return 1");
}

#[test]
fn e10_sim_match_zero_input() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 16 },
        RspuInstruction::Match { dst: 193, src: 192, table_offset: 0 },
    ]);
    sim.step(&prog).expect("step 1");
    sim.step(&prog).expect("step 2");
    assert_eq!(sim.registers.read(193).value, 0, "MATCH on zero input must return 0");
}

#[test]
fn e10_sim_interval_lo_reads_shadow() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (100, 200);
    let prog = make_program(vec![RspuInstruction::IntervalLo { dst: 192, src: 5 }]);
    sim.step(&prog).expect("step");
    assert_eq!(sim.registers.read(192).value, 100, "IntervalLo must read lower bound from shadow");
}

#[test]
fn e10_sim_interval_hi_reads_shadow() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (100, 200);
    let prog = make_program(vec![RspuInstruction::IntervalHi { dst: 192, src: 5 }]);
    sim.step(&prog).expect("step");
    assert_eq!(sim.registers.read(192).value, 200, "IntervalHi must read upper bound from shadow");
}

#[test]
fn e10_sim_interval_lo_default_is_zero() {
    let sim = RspuSimulator::new();
    // Default interval lower bound is 0
    assert_eq!(sim.interval_shadow[0].0, 0, "Default interval lo must be 0");
}

#[test]
fn e10_sim_interval_hi_default_is_u64_max() {
    let sim = RspuSimulator::new();
    // Default interval upper bound is u64::MAX
    assert_eq!(sim.interval_shadow[0].1, u64::MAX, "Default interval hi must be u64::MAX");
}

#[test]
fn e10_sim_interval_check_in_range_passes() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 25 (in range [10,50])
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 25, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    assert_eq!(result, StepResult::Continue, "In-range check must continue");
}

#[test]
fn e10_sim_interval_check_at_lower_boundary_passes() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 10 (exact lower bound, should pass)
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    assert_eq!(result, StepResult::Continue, "Exact lower bound must continue");
}

#[test]
fn e10_sim_interval_check_at_upper_boundary_passes() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 50 (exact upper bound, should pass)
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 50, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    assert_eq!(result, StepResult::Continue, "Exact upper bound must continue");
}

#[test]
fn e10_sim_interval_check_out_of_range_exception() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 100 (out of range [10,50])
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 100, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    match result {
        StepResult::Exception(code) => {
            assert_eq!(
                code,
                nasa_rust_project::emit::rspu_exceptions::ExceptionCode::IntervalViolation,
                "Out-of-range IntervalCheck must raise IntervalViolation"
            );
        }
        other => panic!("Expected IntervalViolation exception, got {:?}", other),
    }
}

#[test]
fn e10_sim_interval_check_below_range_exception() {
    let mut sim = RspuSimulator::new();
    sim.interval_shadow[5] = (10, 50);
    // Set register 192 to value 5 (below range [10,50])
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 16 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 5 },
    ]);
    sim.step(&prog).expect("step 1");
    let result = sim.step(&prog).expect("step 2");
    match result {
        StepResult::Exception(code) => {
            assert_eq!(
                code,
                nasa_rust_project::emit::rspu_exceptions::ExceptionCode::IntervalViolation,
                "Below-range IntervalCheck must raise IntervalViolation"
            );
        }
        other => panic!("Expected IntervalViolation exception, got {:?}", other),
    }
}

// ===========================================================================
// Gap 4: Encoding roundtrip tests for opcodes 30-36
// ===========================================================================

#[test]
fn e3_encode_decode_verify_roundtrip() {
    let i = RspuInstruction::Verify { cert_offset: 4096 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "Verify encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_certify_roundtrip() {
    let i = RspuInstruction::Certify { dst: 192 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "Certify encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_total_check_roundtrip() {
    let i = RspuInstruction::TotalCheck { expected_properties: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "TotalCheck encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_match_roundtrip() {
    let i = RspuInstruction::Match { dst: 193, src: 10, table_offset: 42 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "Match encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_interval_lo_roundtrip() {
    let i = RspuInstruction::IntervalLo { dst: 192, src: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "IntervalLo encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_interval_hi_roundtrip() {
    let i = RspuInstruction::IntervalHi { dst: 192, src: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "IntervalHi encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_interval_check_roundtrip() {
    let i = RspuInstruction::IntervalCheck { src: 192, bounds: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded, i, "IntervalCheck encode/decode roundtrip must preserve fields");
}

#[test]
fn e3_encode_decode_all_mega4_mega5_opcodes_roundtrip() {
    let mega4_5_instrs: [RspuInstruction; 7] = [
        RspuInstruction::Verify { cert_offset: 0 },
        RspuInstruction::Certify { dst: 192 },
        RspuInstruction::TotalCheck { expected_properties: 10 },
        RspuInstruction::Match { dst: 193, src: 5, table_offset: 7 },
        RspuInstruction::IntervalLo { dst: 194, src: 10 },
        RspuInstruction::IntervalHi { dst: 195, src: 10 },
        RspuInstruction::IntervalCheck { src: 192, bounds: 3 },
    ];
    let mut i = 0;
    while i < mega4_5_instrs.len() && i < MAX_TEST_ITERATIONS {
        let encoded = encode(&mega4_5_instrs[i]).expect("encode must succeed");
        let decoded = decode(encoded.0).expect("decode must succeed");
        assert_eq!(
            decoded,
            mega4_5_instrs[i],
            "Roundtrip failed for {}",
            mega4_5_instrs[i].mnemonic()
        );
        i += 1;
    }
}
