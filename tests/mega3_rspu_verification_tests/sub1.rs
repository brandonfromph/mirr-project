use super::*;

#[test]
fn e1_load_input_mnemonic() {
    let i = RspuInstruction::LoadInput { dst: 0, port: 0 };
    assert_eq!(i.mnemonic(), "LOAD_INPUT", "LoadInput mnemonic must match ISA spec");
}

#[test]
fn e1_store_output_mnemonic() {
    let i = RspuInstruction::StoreOutput { src: 64, port: 0 };
    assert_eq!(i.mnemonic(), "STORE_OUTPUT");
}

#[test]
fn e1_mov_mnemonic() {
    let i = RspuInstruction::Mov { dst: 1, src: 0 };
    assert_eq!(i.mnemonic(), "MOV");
}

#[test]
fn e1_load_imm_mnemonic() {
    let i = RspuInstruction::LoadImm { dst: 0, value: 42, width: 16 };
    assert_eq!(i.mnemonic(), "LOAD_IMM");
}

#[test]
fn e1_alu_mnemonic() {
    let i = RspuInstruction::Alu { op: AluOp::Add, dst: 2, a: 0, b: 1 };
    assert_eq!(i.mnemonic(), "ALU");
}

#[test]
fn e1_alu_imm_mnemonic() {
    let i = RspuInstruction::AluImm { op: AluOp::Sub, dst: 2, a: 0, imm: 1 };
    assert_eq!(i.mnemonic(), "ALU_IMM");
}

#[test]
fn e1_alu_unary_mnemonic() {
    let i = RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 1, src: 0 };
    assert_eq!(i.mnemonic(), "ALU_UNARY");
}

#[test]
fn e1_sr_init_mnemonic() {
    let i = RspuInstruction::SrInit { guard: 0, length: 4, cond: 0 };
    assert_eq!(i.mnemonic(), "SR_INIT");
}

#[test]
fn e1_sr_tick_mnemonic() {
    let i = RspuInstruction::SrTick { guard: 0 };
    assert_eq!(i.mnemonic(), "SR_TICK");
}

#[test]
fn e1_sr_query_mnemonic() {
    let i = RspuInstruction::SrQuery { dst: 1, guard: 0 };
    assert_eq!(i.mnemonic(), "SR_QUERY");
}

#[test]
fn e1_ctr_init_mnemonic() {
    let i = RspuInstruction::CtrInit { guard: 0, target: 100, cond: 0 };
    assert_eq!(i.mnemonic(), "CTR_INIT");
}

#[test]
fn e1_ctr_tick_mnemonic() {
    let i = RspuInstruction::CtrTick { guard: 0 };
    assert_eq!(i.mnemonic(), "CTR_TICK");
}

#[test]
fn e1_ctr_query_mnemonic() {
    let i = RspuInstruction::CtrQuery { dst: 1, guard: 0 };
    assert_eq!(i.mnemonic(), "CTR_QUERY");
}

#[test]
fn e1_guard_and_mnemonic() {
    let i = RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 };
    assert_eq!(i.mnemonic(), "GUARD_AND");
}

#[test]
fn e1_guard_or_mnemonic() {
    let i = RspuInstruction::GuardOr { dst: 2, a: 0, b: 1 };
    assert_eq!(i.mnemonic(), "GUARD_OR");
}

#[test]
fn e1_reflex_if_mnemonic() {
    let i = RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 0 };
    assert_eq!(i.mnemonic(), "REFLEX_IF");
}

#[test]
fn e1_prev_mnemonic() {
    let i = RspuInstruction::Prev { dst: 1, signal: 0, delay: 1 };
    assert_eq!(i.mnemonic(), "PREV");
}

#[test]
fn e1_emergency_stop_mnemonic() {
    assert_eq!(RspuInstruction::EmergencyStop.mnemonic(), "EMERGENCY_STOP");
}

#[test]
fn e1_assert_always_mnemonic() {
    let i = RspuInstruction::AssertAlways { cond: 0, property_id: 0 };
    assert_eq!(i.mnemonic(), "ASSERT_ALWAYS");
}

#[test]
fn e1_assert_never_mnemonic() {
    let i = RspuInstruction::AssertNever { cond: 0, property_id: 0 };
    assert_eq!(i.mnemonic(), "ASSERT_NEVER");
}

#[test]
fn e1_all_37_opcodes_have_unique_mnemonics() {
    let instrs: Vec<RspuInstruction> = vec![
        RspuInstruction::LoadInput { dst: 0, port: 0 },
        RspuInstruction::StoreOutput { src: 0, port: 0 },
        RspuInstruction::Mov { dst: 0, src: 0 },
        RspuInstruction::LoadImm { dst: 0, value: 0, width: 1 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 0, a: 0, b: 0 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 0, a: 0, imm: 0 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 0, src: 0 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 0 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 0, guard: 0 },
        RspuInstruction::CtrInit { guard: 0, target: 1, cond: 0 },
        RspuInstruction::CtrTick { guard: 0 },
        RspuInstruction::CtrQuery { dst: 0, guard: 0 },
        RspuInstruction::GuardAnd { dst: 0, a: 0, b: 0 },
        RspuInstruction::GuardOr { dst: 0, a: 0, b: 0 },
        RspuInstruction::ReflexIf { guard: 0, dst: 0, src: 0 },
        RspuInstruction::Prev { dst: 0, signal: 0, delay: 0 },
        RspuInstruction::EmergencyStop,
        RspuInstruction::AssertAlways { cond: 0, property_id: 0 },
        RspuInstruction::AssertNever { cond: 0, property_id: 0 },
        RspuInstruction::Trap { code: 0 },
        RspuInstruction::TrapIf { cond: 0, code: 0 },
        RspuInstruction::Halt,
        RspuInstruction::ModeSwitch { mode: 0 },
        RspuInstruction::TagLoad { dst: 0, tag: 0 },
        RspuInstruction::TagCheck { src: 0, expected: 0 },
        RspuInstruction::TagRead { dst: 0, src: 0 },
        RspuInstruction::Nop,
        RspuInstruction::Fence,
        RspuInstruction::DeadlineSet { cycles: 0 },
        RspuInstruction::Verify { cert_offset: 0 },
        RspuInstruction::Certify { dst: 0 },
        RspuInstruction::TotalCheck { expected_properties: 0 },
        RspuInstruction::Match { dst: 0, src: 0, table_offset: 0 },
        RspuInstruction::IntervalLo { dst: 0, src: 0 },
        RspuInstruction::IntervalHi { dst: 0, src: 0 },
        RspuInstruction::IntervalCheck { src: 0, bounds: 0 },
    ];

    // Verify all 37 instruction variants exist.
    assert_eq!(instrs.len(), 37, "R-SPU ISA must have exactly 37 instruction variants");

    // Verify no two instructions share a mnemonic.
    let mut mnemonics: Vec<&str> = Vec::with_capacity(MAX_TEST_ITERATIONS);
    let mut i = 0;
    while i < instrs.len() && i < MAX_TEST_ITERATIONS {
        let m = instrs[i].mnemonic();
        let mut j = 0;
        while j < mnemonics.len() && j < MAX_TEST_ITERATIONS {
            assert_ne!(mnemonics[j], m, "Duplicate mnemonic: {}", m);
            j += 1;
        }
        mnemonics.push(m);
        i += 1;
    }
}

// ===========================================================================
// E2: Tagged-word type safety
// ===========================================================================

#[test]
fn e2_tagged_word_uninitialized_default() {
    let w = TaggedWord::uninitialized();
    assert_eq!(w.tag, TypeTag::Uninitialized, "Default register must be Uninitialized");
    assert_eq!(w.value, 0);
}

#[test]
fn e2_register_file_256_entries() {
    let rf = RegisterFile::new();
    // All 256 registers must be readable.
    let mut i: usize = 0;
    while i < MAX_REGISTERS {
        let w = rf.read(i as RegId);
        assert_eq!(w.tag, TypeTag::Uninitialized, "Register R{} must start Uninitialized", i);
        i += 1;
    }
}

#[test]
fn e2_register_file_write_read_roundtrip() {
    let mut rf = RegisterFile::new();
    let w = TaggedWord::from_literal(42, TypeTag::Unsigned { width: 16 });
    rf.write(10, w.clone());
    assert_eq!(rf.read(10).value, 42);
    assert_eq!(rf.read(10).tag, TypeTag::Unsigned { width: 16 });
}

#[test]
fn e2_register_partition_input() {
    assert_eq!(REG_INPUT_BASE, 0);
    assert_eq!(REG_INPUT_MAX, 63);
}

#[test]
fn e2_register_partition_output() {
    assert_eq!(REG_OUTPUT_BASE, 64);
    assert_eq!(REG_OUTPUT_MAX, 127);
}

#[test]
fn e2_register_partition_internal() {
    assert_eq!(REG_INTERNAL_BASE, 128);
    assert_eq!(REG_INTERNAL_MAX, 191);
}

#[test]
fn e2_register_partition_temp() {
    assert_eq!(REG_TEMP_BASE, 192);
    assert_eq!(REG_TEMP_MAX, 255);
}

#[test]
fn e2_type_tag_display_bool() {
    assert_eq!(format!("{}", TypeTag::Bool), "bool");
}

#[test]
fn e2_type_tag_display_unsigned() {
    assert_eq!(format!("{}", TypeTag::Unsigned { width: 16 }), "u16");
}

#[test]
fn e2_type_tag_display_signed() {
    assert_eq!(format!("{}", TypeTag::Signed { width: 32 }), "i32");
}

#[test]
fn e2_type_tag_display_uninitialized() {
    assert_eq!(format!("{}", TypeTag::Uninitialized), "<uninitialized>");
}

#[test]
fn e2_type_tag_display_interval() {
    assert_eq!(format!("{}", TypeTag::Interval { lo: 0, hi: 255 }), "interval[0, 255]");
}

// ===========================================================================
// E3: Binary encoding/decoding roundtrip
// ===========================================================================

#[test]
fn e3_encode_decode_load_input() {
    let i = RspuInstruction::LoadInput { dst: 5, port: 3 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "LOAD_INPUT");
}

#[test]
fn e3_encode_decode_store_output() {
    let i = RspuInstruction::StoreOutput { src: 64, port: 1 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "STORE_OUTPUT");
}

#[test]
fn e3_encode_decode_mov() {
    let i = RspuInstruction::Mov { dst: 10, src: 5 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "MOV");
}

#[test]
fn e3_encode_decode_alu_add() {
    let i = RspuInstruction::Alu { op: AluOp::Add, dst: 2, a: 0, b: 1 };
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "ALU");
}

#[test]
fn e3_encode_decode_emergency_stop() {
    let i = RspuInstruction::EmergencyStop;
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "EMERGENCY_STOP");
}

#[test]
fn e3_encode_decode_halt() {
    let i = RspuInstruction::Halt;
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "HALT");
}

#[test]
fn e3_encode_decode_nop() {
    let i = RspuInstruction::Nop;
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "NOP");
}

#[test]
fn e3_encode_decode_fence() {
    let i = RspuInstruction::Fence;
    let encoded = encode(&i).expect("encode must succeed");
    let decoded = decode(encoded.0).expect("decode must succeed");
    assert_eq!(decoded.mnemonic(), "FENCE");
}

#[test]
fn e3_encoded_instruction_is_32_bits() {
    let i = RspuInstruction::Nop;
    let _encoded = encode(&i).expect("encode must succeed");
    assert_eq!(std::mem::size_of::<EncodedInstruction>(), 4, "Encoded instruction must be 32 bits");
}

#[test]
fn e3_binary_roundtrip_all_zero_arg_instructions() {
    let zero_arg_instrs = [
        RspuInstruction::EmergencyStop,
        RspuInstruction::Halt,
        RspuInstruction::Nop,
        RspuInstruction::Fence,
    ];
    let mut i = 0;
    while i < zero_arg_instrs.len() && i < MAX_TEST_ITERATIONS {
        let encoded = encode(&zero_arg_instrs[i]).expect("encode must succeed");
        let decoded = decode(encoded.0).expect("decode must succeed");
        assert_eq!(
            decoded.mnemonic(),
            zero_arg_instrs[i].mnemonic(),
            "Roundtrip failed for {}",
            zero_arg_instrs[i].mnemonic()
        );
        i += 1;
    }
}

// ===========================================================================
// E4: Exception model
// ===========================================================================

#[test]
fn e4_exception_code_tag_violation_is_zero() {
    // ExceptionCode::TagViolation repr = 0
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::TagViolation;
    assert_eq!(code as u8, 0);
}

#[test]
fn e4_exception_code_deadline_miss_is_one() {
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::DeadlineMiss;
    assert_eq!(code as u8, 1);
}

#[test]
fn e4_exception_code_property_fail_is_two() {
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::PropertyFail;
    assert_eq!(code as u8, 2);
}

#[test]
fn e4_exception_code_software_trap_is_five() {
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::SoftwareTrap;
    assert_eq!(code as u8, 5);
}

#[test]
fn e4_exception_code_interval_violation_is_seven() {
    let code = nasa_rust_project::emit::rspu_exceptions::ExceptionCode::IntervalViolation;
    assert_eq!(code as u8, 7);
}

#[test]
fn e4_exec_mode_reflex_display() {
    let mode = nasa_rust_project::emit::rspu_exceptions::ExecMode::Reflex;
    assert_eq!(format!("{}", mode), "Reflex");
}

#[test]
fn e4_exec_mode_host_display() {
    let mode = nasa_rust_project::emit::rspu_exceptions::ExecMode::Host;
    assert_eq!(format!("{}", mode), "Host");
}

// ===========================================================================
// E5: Simulator step semantics
// ===========================================================================

#[test]
fn e5_sim_new_starts_at_pc_zero() {
    let sim = RspuSimulator::new();
    assert_eq!(sim.pc, 0, "Simulator must start at PC=0");
    assert_eq!(sim.cycle, 0, "Simulator must start at cycle=0");
    assert!(!sim.halted, "Simulator must not be halted at start");
}

#[test]
fn e5_sim_empty_program_halts_immediately() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![]);
    let result = sim.step(&prog).expect("step must not error");
    assert_eq!(result, StepResult::Halted, "Empty program must halt on first step");
}

#[test]
fn e5_sim_nop_advances_pc() {
    let mut sim = RspuSimulator::new();
    let prog = make_program(vec![RspuInstruction::Nop]);
    let result = sim.step(&prog).expect("step must not error");
    assert_eq!(result, StepResult::Continue);
    assert_eq!(sim.pc, 1, "PC must advance after NOP");
    assert_eq!(sim.cycle, 1, "Cycle must advance after NOP");
}
