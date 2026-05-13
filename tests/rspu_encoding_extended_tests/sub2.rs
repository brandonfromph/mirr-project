use super::*;

#[test]
fn test_mode_switch_parametric() {
    let mode_vals: [u8; 8] = [0, 1, 2, 5, 10, 50, 128, 255];
    for i in 0..8 {
        let instr = RspuInstruction::ModeSwitch { mode: mode_vals[i] };
        roundtrip_check(&instr, &format!("ModeSwitch mode={}", mode_vals[i]));
    }
}

#[test]
fn test_deadline_set_parametric() {
    // cycles is 26-bit (0..0x03FF_FFFF).
    let cycle_vals: [u32; 8] =
        [0, 1, 1000, 0xFFFF, 0x1_0000, 0x00FF_FFFF, 0x03AB_CDEF, 0x03FF_FFFF];
    for i in 0..8 {
        let instr = RspuInstruction::DeadlineSet { cycles: cycle_vals[i] };
        roundtrip_check(&instr, &format!("DeadlineSet cycles={}", cycle_vals[i]));
    }
}

#[test]
fn test_s_type_zero_payload_instructions() {
    // EmergencyStop, Halt, Nop, Fence all encode with zero payload.
    let instrs = [
        RspuInstruction::EmergencyStop,
        RspuInstruction::Halt,
        RspuInstruction::Nop,
        RspuInstruction::Fence,
    ];
    for i in 0..4 {
        let enc = encode(&instrs[i]).unwrap_or_else(|e| {
            panic!("encoding {} failed: {}", instrs[i].mnemonic(), e.message());
        });
        let imm26 = enc.0 & 0x03FF_FFFF;
        assert_eq!(
            imm26,
            0,
            "{} should have zero imm26 payload, got {:#010X}",
            instrs[i].mnemonic(),
            imm26
        );
        roundtrip_check(&instrs[i], instrs[i].mnemonic());
    }
}

// ===========================================================================
// Section 6: ALU operation encoding/decoding roundtrips
// ===========================================================================

#[test]
fn test_alu_all_binary_ops_roundtrip() {
    // All 14 binary ALU ops. b must be <= 63 for the 6-bit field.
    let ops: [AluOp; 14] = [
        AluOp::Add,
        AluOp::Sub,
        AluOp::Mul,
        AluOp::And,
        AluOp::Or,
        AluOp::Xor,
        AluOp::Shl,
        AluOp::Shr,
        AluOp::Eq,
        AluOp::Ne,
        AluOp::Lt,
        AluOp::Le,
        AluOp::Gt,
        AluOp::Ge,
    ];
    for i in 0..14 {
        let instr = RspuInstruction::Alu { op: ops[i], dst: 192, a: 0, b: 1 };
        roundtrip_check(&instr, &format!("ALU op_index={}", i));
    }
}

#[test]
fn test_alu_funct_code_ordering() {
    // Verify the funct codes are assigned in order: Add=0, Sub=1, ..., Ge=13.
    let ops: [AluOp; 14] = [
        AluOp::Add,
        AluOp::Sub,
        AluOp::Mul,
        AluOp::And,
        AluOp::Or,
        AluOp::Xor,
        AluOp::Shl,
        AluOp::Shr,
        AluOp::Eq,
        AluOp::Ne,
        AluOp::Lt,
        AluOp::Le,
        AluOp::Gt,
        AluOp::Ge,
    ];
    for i in 0..14 {
        // Encode ALU with this op, then extract the funct bits from imm10[3:0].
        let instr = RspuInstruction::Alu { op: ops[i], dst: 0, a: 0, b: 0 };
        let enc = encode(&instr).expect("ALU encode must succeed");
        let imm10 = enc.0 & 0x3FF;
        let op_funct = imm10 & 0xF;
        assert_eq!(
            op_funct, i as u32,
            "ALU op index {} should have funct code {}, got {}",
            i, i, op_funct
        );
    }
}

#[test]
fn test_alu_b_register_field_encoding() {
    // ALU encodes b in imm10[9:4] (6 bits). Verify b is correctly placed.
    let b_vals: [u8; 8] = [0, 1, 10, 31, 32, 48, 62, 63];
    for i in 0..8 {
        let instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: b_vals[i] };
        let enc = encode(&instr).expect("ALU encode must succeed");
        let imm10 = enc.0 & 0x3FF;
        let extracted_b = ((imm10 >> 4) & 0x3F) as u8;
        assert_eq!(
            extracted_b, b_vals[i],
            "ALU b register: expected {}, extracted {}",
            b_vals[i], extracted_b
        );
    }
}

#[test]
fn test_alu_imm_ops_0_through_7_roundtrip() {
    // AluImm only supports ops with funct code 0..7 (Add through Shr).
    let ops: [AluOp; 8] = [
        AluOp::Add,
        AluOp::Sub,
        AluOp::Mul,
        AluOp::And,
        AluOp::Or,
        AluOp::Xor,
        AluOp::Shl,
        AluOp::Shr,
    ];
    for i in 0..8 {
        let instr = RspuInstruction::AluImm { op: ops[i], dst: 192, a: 0, imm: 42 };
        roundtrip_check(&instr, &format!("ALU_IMM op_index={}", i));
    }
}

#[test]
fn test_alu_imm_field_encoding() {
    // AluImm packs: imm10 = (op_code << 7) | (imm & 0x7F).
    // Verify the encoding manually.
    let instr = RspuInstruction::AluImm { op: AluOp::Sub, dst: 192, a: 0, imm: 100 };
    let enc = encode(&instr).expect("ALU_IMM encode must succeed");
    let packed_imm10 = (enc.0 & 0x3FF) as u16;

    // Sub has funct code 1, imm is 100.
    let expected = (1_u16 << 7) | 100;
    assert_eq!(
        packed_imm10, expected,
        "ALU_IMM packed imm10: expected {:#06X}, got {:#06X}",
        expected, packed_imm10
    );
}

#[test]
fn test_alu_imm_boundary_values() {
    // Test immediate boundary: 0 and 127 (max for 7-bit field).
    let instr_zero = RspuInstruction::AluImm { op: AluOp::Add, dst: 192, a: 0, imm: 0 };
    roundtrip_check(&instr_zero, "ALU_IMM imm=0");

    let instr_max = RspuInstruction::AluImm { op: AluOp::Add, dst: 192, a: 0, imm: 127 };
    roundtrip_check(&instr_max, "ALU_IMM imm=127");
}

#[test]
fn test_alu_unary_funct_codes() {
    // Not=0, Negate=1 in the funct field [1:0].
    let instr_not = RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 0, src: 0 };
    let enc_not = encode(&instr_not).expect("ALU_UNARY NOT encode must succeed");
    let funct_not = enc_not.0 & 0x3;
    assert_eq!(funct_not, 0, "AluUnary NOT funct should be 0, got {}", funct_not);

    let instr_neg = RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 0, src: 0 };
    let enc_neg = encode(&instr_neg).expect("ALU_UNARY NEGATE encode must succeed");
    let funct_neg = enc_neg.0 & 0x3;
    assert_eq!(funct_neg, 1, "AluUnary NEGATE funct should be 1, got {}", funct_neg);
}

// ===========================================================================
// Section 7: Error path tests — E706 overflow
// ===========================================================================

#[test]
fn test_e706_load_imm_overflow() {
    // value 1024 exceeds 10-bit max (1023).
    let instr = RspuInstruction::LoadImm { dst: 0, value: 1024, width: 16 };
    let result = encode(&instr);
    assert!(result.is_err(), "LoadImm value=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "LoadImm overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_load_input_port_overflow() {
    // port 1024 exceeds 10-bit max (1023).
    let instr = RspuInstruction::LoadInput { dst: 0, port: 1024 };
    let result = encode(&instr);
    assert!(result.is_err(), "LoadInput port=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "LoadInput port overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_store_output_port_overflow() {
    let instr = RspuInstruction::StoreOutput { src: 64, port: 1024 };
    let result = encode(&instr);
    assert!(result.is_err(), "StoreOutput port=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(
        msg.contains("E706"),
        "StoreOutput port overflow error should contain E706, got: {msg}"
    );
}

#[test]
fn test_e706_alu_b_register_overflow() {
    // Register b=64 exceeds the 6-bit field max (63).
    let instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 64 };
    let result = encode(&instr);
    assert!(result.is_err(), "ALU b=64 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "ALU b overflow error should contain E706, got: {msg}");
    assert!(
        msg.contains("register b index"),
        "ALU b overflow error should mention 'register b index', got: {msg}"
    );
}

#[test]
fn test_e706_alu_b_register_max_boundary() {
    // b=63 should succeed (max for 6-bit field), b=64 should fail.
    let ok_instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 63 };
    assert!(encode(&ok_instr).is_ok(), "ALU b=63 (max 6-bit) should succeed");

    let fail_instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 64 };
    assert!(encode(&fail_instr).is_err(), "ALU b=64 (exceeds 6-bit) should fail");
}

#[test]
fn test_e706_alu_imm_comparison_ops_rejected() {
    // Comparison ops (Eq=8, Ne=9, Lt=10, Le=11, Gt=12, Ge=13) have funct > 7
    // and cannot be used with AluImm (only 3-bit op field).
    let cmp_ops: [AluOp; 6] = [AluOp::Eq, AluOp::Ne, AluOp::Lt, AluOp::Le, AluOp::Gt, AluOp::Ge];
    for i in 0..6 {
        let instr = RspuInstruction::AluImm { op: cmp_ops[i], dst: 192, a: 0, imm: 1 };
        let result = encode(&instr);
        assert!(result.is_err(), "AluImm with comparison op index {} should fail with E706", i);
        let msg = result.unwrap_err().message().to_string();
        assert!(msg.contains("E706"), "AluImm comparison op error should contain E706, got: {msg}");
    }
}

#[test]
fn test_e706_alu_imm_immediate_overflow() {
    // imm=128 exceeds the 7-bit field max (127).
    let instr = RspuInstruction::AluImm { op: AluOp::Add, dst: 192, a: 0, imm: 128 };
    let result = encode(&instr);
    assert!(result.is_err(), "AluImm imm=128 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(
        msg.contains("E706"),
        "AluImm immediate overflow error should contain E706, got: {msg}"
    );
}

#[test]
fn test_e706_sr_init_length_overflow() {
    // length 1024 exceeds 10-bit max.
    let instr = RspuInstruction::SrInit { guard: 0, length: 1024, cond: 0 };
    let result = encode(&instr);
    assert!(result.is_err(), "SrInit length=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "SrInit length overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_ctr_init_target_overflow() {
    // target 1024 exceeds 10-bit max.
    let instr = RspuInstruction::CtrInit { guard: 0, target: 1024, cond: 0 };
    let result = encode(&instr);
    assert!(result.is_err(), "CtrInit target=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "CtrInit target overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_prev_delay_overflow() {
    // delay 1024 exceeds 10-bit max.
    let instr = RspuInstruction::Prev { dst: 192, signal: 5, delay: 1024 };
    let result = encode(&instr);
    assert!(result.is_err(), "Prev delay=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "Prev delay overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_deadline_set_overflow() {
    // cycles exceeding 26-bit max.
    let instr = RspuInstruction::DeadlineSet { cycles: 0x0400_0000 };
    let result = encode(&instr);
    assert!(result.is_err(), "DeadlineSet cycles=0x0400_0000 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "DeadlineSet overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_deadline_set_boundary() {
    // Max valid: 0x03FF_FFFF should succeed.
    let ok_instr = RspuInstruction::DeadlineSet { cycles: 0x03FF_FFFF };
    assert!(
        encode(&ok_instr).is_ok(),
        "DeadlineSet cycles=0x03FF_FFFF (max 26-bit) should succeed"
    );

    // One over: 0x0400_0000 should fail.
    let fail_instr = RspuInstruction::DeadlineSet { cycles: 0x0400_0000 };
    assert!(encode(&fail_instr).is_err(), "DeadlineSet cycles=0x0400_0000 should fail");
}

// ===========================================================================
// Section 8: Error path tests — E707 unknown opcode
// ===========================================================================

#[test]
fn test_e707_unknown_opcodes_30_through_63() {
    // Opcodes 30..32 are assigned (MEGA-4: VERIFY, CERTIFY, TOTAL_CHECK).
    // Opcodes 33..36 are assigned (MEGA-5: MATCH, INTERVAL_LO, INTERVAL_HI, INTERVAL_CHECK).
    // Opcode 37 is assigned (OP_TAG_BRANCH).
    // Opcodes 38..63 are unassigned. Decoding them should produce E707.
    for opcode in 38..MAX_OPCODE_SCAN {
        let word: u32 = (opcode as u32) << 26;
        let result = decode(word);
        assert!(result.is_err(), "decoding unassigned opcode {} should fail with E707", opcode);
        let msg = result.unwrap_err().message().to_string();
        assert!(
            msg.contains("E707"),
            "unassigned opcode {} error should contain E707, got: {msg}",
            opcode
        );
    }
}

#[test]
fn test_e707_max_opcode_63() {
    let word: u32 = 63 << 26;
    let result = decode(word);
    assert!(result.is_err(), "decoding opcode 63 should fail with E707");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E707"), "opcode 63 error should contain E707, got: {msg}");
}

// ===========================================================================
// Section 9: EncodedInstruction newtype tests
// ===========================================================================

#[test]
fn test_encoded_instruction_equality() {
    let a = EncodedInstruction(0x1234_5678);
    let b = EncodedInstruction(0x1234_5678);
    let c = EncodedInstruction(0x0000_0000);
    assert_eq!(a, b, "identical EncodedInstruction values should be equal");
    assert_ne!(a, c, "different EncodedInstruction values should not be equal");
}

#[test]
fn test_encoded_instruction_copy_clone() {
    let a = EncodedInstruction(0xDEAD_BEEF);
    let b = a; // Copy
    let c = a.clone();
    assert_eq!(a.0, b.0, "EncodedInstruction should be Copy: a and b should have same value");
    assert_eq!(a.0, c.0, "EncodedInstruction Clone should produce equal value");
}

#[test]
fn test_encoded_instruction_debug_format() {
    let enc = EncodedInstruction(42);
    let debug_str = format!("{:?}", enc);
    assert!(
        debug_str.contains("EncodedInstruction"),
        "Debug format should contain 'EncodedInstruction', got: {debug_str}"
    );
    assert!(
        debug_str.contains("42"),
        "Debug format should contain the inner value '42', got: {debug_str}"
    );
}

// ===========================================================================
// Section 10: Comprehensive roundtrip — all 30 opcodes
// ===========================================================================

#[test]
fn test_roundtrip_every_opcode_comprehensive() {
    // One representative instruction per opcode, covering all 30 assigned opcodes.
    let all_instrs: [RspuInstruction; 30] = [
        RspuInstruction::LoadInput { dst: 5, port: 42 },
        RspuInstruction::StoreOutput { src: 64, port: 7 },
        RspuInstruction::Mov { dst: 10, src: 20 },
        RspuInstruction::LoadImm { dst: 3, value: 255, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 },
        RspuInstruction::AluImm { op: AluOp::Sub, dst: 192, a: 5, imm: 10 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 192, src: 5 },
        RspuInstruction::SrInit { guard: 0, length: 5, cond: 10 },
        RspuInstruction::SrTick { guard: 2 },
        RspuInstruction::SrQuery { dst: 192, guard: 2 },
        RspuInstruction::CtrInit { guard: 1, target: 100, cond: 5 },
        RspuInstruction::CtrTick { guard: 1 },
        RspuInstruction::CtrQuery { dst: 192, guard: 1 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::GuardOr { dst: 3, a: 1, b: 2 },
        RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 5 },
        RspuInstruction::Prev { dst: 192, signal: 5, delay: 3 },
        RspuInstruction::EmergencyStop,
        RspuInstruction::AssertAlways { cond: 10, property_id: 42 },
        RspuInstruction::AssertNever { cond: 11, property_id: 99 },
        RspuInstruction::Trap { code: 5 },
        RspuInstruction::TrapIf { cond: 10, code: 3 },
        RspuInstruction::Halt,
        RspuInstruction::ModeSwitch { mode: 1 },
        RspuInstruction::TagLoad { dst: 192, tag: 2 },
        RspuInstruction::TagCheck { src: 5, expected: 2 },
        RspuInstruction::TagRead { dst: 192, src: 5 },
        RspuInstruction::Nop,
        RspuInstruction::Fence,
        RspuInstruction::DeadlineSet { cycles: 1000 },
    ];
    for i in 0..30 {
        roundtrip_check(&all_instrs[i], &format!("opcode_{}_comprehensive", i));
    }
}

// ===========================================================================
// Section 11: Program-level emit_binary tests
// ===========================================================================

#[test]
fn test_emit_binary_empty_program() {
    let program = make_program(vec![]);
    let words = emit_binary(&program).expect("emit_binary on empty program should succeed");
    assert_eq!(words.len(), 0, "empty program should produce zero words");
}

#[test]
fn test_emit_binary_single_instruction() {
    let program = make_program(vec![RspuInstruction::Nop]);
    let words = emit_binary(&program).expect("emit_binary single NOP should succeed");
    assert_eq!(words.len(), 1, "single instruction should produce one word");
    let decoded = decode(words[0]).expect("decode of emitted word should succeed");
    assert_eq!(decoded, RspuInstruction::Nop, "decoded word should be NOP");
}

#[test]
fn test_emit_binary_multi_instruction_roundtrip() {
    let instrs = vec![
        RspuInstruction::LoadInput { dst: 0, port: 0 },
        RspuInstruction::LoadInput { dst: 1, port: 1 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
        RspuInstruction::EmergencyStop,
    ];
    let program = make_program(instrs.clone());
    let words = emit_binary(&program).expect("emit_binary should succeed");
    assert_eq!(words.len(), instrs.len(), "word count should match instruction count");
    for i in 0..words.len() {
        let decoded = decode(words[i]).unwrap_or_else(|e| {
            panic!("decode of word {} failed: {}", i, e.message());
        });
        assert_eq!(decoded, instrs[i], "instruction {} mismatch after emit_binary roundtrip", i);
    }
}

#[test]
fn test_emit_binary_stress_bounded() {
    // Generate MAX_EMIT_STRESS instructions (all NOPs) and verify roundtrip.
    let mut instrs = Vec::with_capacity(MAX_EMIT_STRESS);
    for _i in 0..MAX_EMIT_STRESS {
        instrs.push(RspuInstruction::Nop);
    }
    let program = make_program(instrs);
    let words = emit_binary(&program).expect("emit_binary stress should succeed");
    assert_eq!(
        words.len(),
        MAX_EMIT_STRESS,
        "stress test should produce {} words",
        MAX_EMIT_STRESS
    );
    for i in 0..MAX_EMIT_STRESS {
        let decoded = decode(words[i]).expect("decode in stress test should succeed");
        assert_eq!(decoded, RspuInstruction::Nop, "stress test word {} should decode to NOP", i);
    }
}
