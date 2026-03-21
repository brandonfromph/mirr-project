use super::*;

#[test]
fn test_emit_binary_exceeds_max_instructions() {
    // A program with MAX_INSTRUCTIONS + 1 instructions should fail.
    let mut instrs = Vec::with_capacity(MAX_INSTRUCTIONS + 1);
    for _i in 0..MAX_INSTRUCTIONS + 1 {
        instrs.push(RspuInstruction::Nop);
    }
    let program = make_program(instrs);
    let result = emit_binary(&program);
    assert!(
        result.is_err(),
        "emit_binary with {} instructions should fail (MAX_INSTRUCTIONS={})",
        MAX_INSTRUCTIONS + 1,
        MAX_INSTRUCTIONS
    );
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "emit_binary overflow error should contain E706, got: {msg}");
}

#[test]
fn test_emit_binary_exactly_max_instructions() {
    // Exactly MAX_INSTRUCTIONS should succeed.
    let mut instrs = Vec::with_capacity(MAX_INSTRUCTIONS);
    for _i in 0..MAX_INSTRUCTIONS {
        instrs.push(RspuInstruction::Nop);
    }
    let program = make_program(instrs);
    let result = emit_binary(&program);
    assert!(result.is_ok(), "emit_binary with exactly MAX_INSTRUCTIONS should succeed");
    let words = result.unwrap();
    assert_eq!(
        words.len(),
        MAX_INSTRUCTIONS,
        "emit_binary should produce exactly MAX_INSTRUCTIONS words"
    );
}

// ===========================================================================
// Section 12: Bit-isolation tests — verify fields do not overlap
// ===========================================================================

#[test]
fn test_r_type_fields_no_overlap() {
    // Encode with all fields at max to verify no bit collisions.
    // R-type: dst=0xFF, src1=0xFF, src2=0xFF, funct=0x3
    // Using AluUnary (R-type) with dst=255, src=255, funct=1 (Negate).
    let instr = RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 255, src: 255 };
    let enc = encode(&instr).expect("AluUnary max fields encode must succeed");
    let word = enc.0;

    // Verify each field independently.
    let opcode = (word >> 26) & 0x3F;
    let dst = (word >> 18) & 0xFF;
    let src1 = (word >> 10) & 0xFF;
    let funct = word & 0x3;

    assert_eq!(opcode, 6, "AluUnary opcode should be 6");
    assert_eq!(dst, 255, "dst at max should be 255");
    assert_eq!(src1, 255, "src at max should be 255");
    assert_eq!(funct, 1, "Negate funct should be 1");

    // Verify roundtrip preserves values.
    roundtrip_check(&instr, "R-type max fields");
}

#[test]
fn test_i_type_fields_no_overlap() {
    // I-type: dst=0xFF, src=0xFF, imm10=0x3FF
    // LoadImm: dst=255, width(src)=255, value(imm10)=1023.
    let instr = RspuInstruction::LoadImm { dst: 255, value: 1023, width: 255 };
    let enc = encode(&instr).expect("LoadImm max fields encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let dst = (word >> 18) & 0xFF;
    let src = (word >> 10) & 0xFF;
    let imm10 = word & 0x3FF;

    assert_eq!(opcode, 3, "LoadImm opcode should be 3");
    assert_eq!(dst, 255, "dst at max should be 255");
    assert_eq!(src, 255, "src (width) at max should be 255");
    assert_eq!(imm10, 1023, "imm10 at max should be 1023");

    roundtrip_check(&instr, "I-type max fields");
}

#[test]
fn test_g_type_fields_no_overlap() {
    // G-type: guard=0xFF, src_dst=0xFF, guard2=0xFF, funct=0x3
    // We cannot directly encode all-max G-type through the public API since
    // the funct is always 0 for most G-type instructions. Test with SrQuery
    // which uses guard and src_dst.
    let instr = RspuInstruction::ReflexIf { guard: 255, dst: 255, src: 255 };
    let enc = encode(&instr).expect("ReflexIf max fields encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let guard = (word >> 18) & 0xFF;
    let src_dst = (word >> 10) & 0xFF;
    let guard2 = (word >> 2) & 0xFF;
    let funct = word & 0x3;

    assert_eq!(opcode, 15, "ReflexIf opcode should be 15");
    assert_eq!(guard, 255, "guard at max should be 255");
    assert_eq!(src_dst, 255, "src_dst at max should be 255");
    assert_eq!(guard2, 255, "guard2 (src) at max should be 255");
    assert_eq!(funct, 0, "ReflexIf funct should be 0");

    roundtrip_check(&instr, "G-type max fields");
}

#[test]
fn test_s_type_imm26_max() {
    // S-type with max 26-bit immediate.
    let instr = RspuInstruction::DeadlineSet { cycles: 0x03FF_FFFF };
    let enc = encode(&instr).expect("DeadlineSet max imm26 encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let imm26 = word & 0x03FF_FFFF;

    assert_eq!(opcode, 29, "DeadlineSet opcode should be 29");
    assert_eq!(imm26, 0x03FF_FFFF, "imm26 at max should be 0x03FF_FFFF, got {:#010X}", imm26);
    roundtrip_check(&instr, "S-type max imm26");
}

// ===========================================================================
// Section 13: Assert encoding — cond and property_id packing
// ===========================================================================

#[test]
fn test_assert_always_field_packing() {
    // AssertAlways packs: imm26 = (cond << 18) | (property_id & 0x3_FFFF).
    let instr = RspuInstruction::AssertAlways { cond: 0xAB, property_id: 0x1_2345 };
    let enc = encode(&instr).expect("AssertAlways encode must succeed");
    let imm26 = enc.0 & 0x03FF_FFFF;

    let expected_imm = (0xAB_u32 << 18) | 0x1_2345;
    assert_eq!(
        imm26, expected_imm,
        "AssertAlways imm26 packing: expected {:#010X}, got {:#010X}",
        expected_imm, imm26
    );
    roundtrip_check(&instr, "AssertAlways field packing");
}

#[test]
fn test_assert_never_field_packing() {
    let instr = RspuInstruction::AssertNever { cond: 0xFF, property_id: 0x3_FFFF };
    let enc = encode(&instr).expect("AssertNever encode must succeed");
    let imm26 = enc.0 & 0x03FF_FFFF;

    let expected_imm = (0xFF_u32 << 18) | 0x3_FFFF;
    assert_eq!(
        imm26, expected_imm,
        "AssertNever imm26 packing: expected {:#010X}, got {:#010X}",
        expected_imm, imm26
    );
    roundtrip_check(&instr, "AssertNever max fields");
}

// ===========================================================================
// Section 14: TrapIf encoding — cond and code packing
// ===========================================================================

#[test]
fn test_trap_if_field_packing() {
    // TrapIf packs: imm26 = (cond << 8) | code.
    let instr = RspuInstruction::TrapIf { cond: 0xAB, code: 0xCD };
    let enc = encode(&instr).expect("TrapIf encode must succeed");
    let imm26 = enc.0 & 0x03FF_FFFF;

    let expected_imm = (0xAB_u32 << 8) | 0xCD;
    assert_eq!(
        imm26, expected_imm,
        "TrapIf imm26 packing: expected {:#010X}, got {:#010X}",
        expected_imm, imm26
    );
    roundtrip_check(&instr, "TrapIf field packing");
}

// ===========================================================================
// Section 15: Decode stability — random-looking bit patterns
// ===========================================================================

#[test]
fn test_decode_all_zero_word() {
    // Word 0x0000_0000 is opcode 0 (LoadInput) with all fields zero.
    let decoded = decode(0x0000_0000).expect("all-zero word should decode as LoadInput");
    assert_eq!(
        decoded,
        RspuInstruction::LoadInput { dst: 0, port: 0 },
        "all-zero word should decode to LoadInput dst=0 port=0"
    );
}

#[test]
fn test_decode_known_bit_patterns() {
    // Manually construct known words and verify their decoding.
    // NOP: opcode=27 (0x1B), zero payload -> word = 0x1B << 26 = 0x6C00_0000.
    let nop_word: u32 = 27 << 26;
    let decoded = decode(nop_word).expect("NOP word should decode");
    assert_eq!(decoded, RspuInstruction::Nop, "manually constructed NOP word should decode to Nop");

    // FENCE: opcode=28 (0x1C), zero payload -> word = 0x1C << 26 = 0x7000_0000.
    let fence_word: u32 = 28 << 26;
    let decoded = decode(fence_word).expect("FENCE word should decode");
    assert_eq!(
        decoded,
        RspuInstruction::Fence,
        "manually constructed FENCE word should decode to Fence"
    );

    // HALT: opcode=22 (0x16), zero payload -> word = 0x16 << 26 = 0x5800_0000.
    let halt_word: u32 = 22 << 26;
    let decoded = decode(halt_word).expect("HALT word should decode");
    assert_eq!(
        decoded,
        RspuInstruction::Halt,
        "manually constructed HALT word should decode to Halt"
    );
}

#[test]
fn test_encode_decode_deterministic() {
    // Encoding the same instruction twice must produce the same word.
    let instr = RspuInstruction::Alu { op: AluOp::Xor, dst: 200, a: 10, b: 20 };
    let enc1 = encode(&instr).expect("first encode should succeed");
    let enc2 = encode(&instr).expect("second encode should succeed");
    assert_eq!(enc1.0, enc2.0, "encoding the same instruction twice must produce identical words");
}

// ===========================================================================
// Section 16: Mixed-format program roundtrip
// ===========================================================================

#[test]
fn test_emit_binary_all_formats_program() {
    // A program that includes at least one instruction from each format
    // (R-type, I-type, G-type, S-type) and all ISA v2 extensions.
    let instrs = vec![
        // I-type
        RspuInstruction::LoadInput { dst: 0, port: 100 },
        RspuInstruction::LoadInput { dst: 1, port: 200 },
        RspuInstruction::LoadImm { dst: 2, value: 42, width: 8 },
        // R-type
        RspuInstruction::Mov { dst: 3, src: 0 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 193, src: 192 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::GuardOr { dst: 3, a: 0, b: 2 },
        RspuInstruction::TagRead { dst: 194, src: 0 },
        // I-type
        RspuInstruction::AluImm { op: AluOp::Add, dst: 195, a: 0, imm: 10 },
        RspuInstruction::SrInit { guard: 0, length: 5, cond: 10 },
        RspuInstruction::CtrInit { guard: 1, target: 100, cond: 5 },
        RspuInstruction::Prev { dst: 196, signal: 0, delay: 2 },
        RspuInstruction::StoreOutput { src: 192, port: 50 },
        RspuInstruction::TagLoad { dst: 197, tag: 3 },
        RspuInstruction::TagCheck { src: 197, expected: 3 },
        // G-type
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 198, guard: 0 },
        RspuInstruction::CtrTick { guard: 1 },
        RspuInstruction::CtrQuery { dst: 199, guard: 1 },
        RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 192 },
        // S-type
        RspuInstruction::AssertAlways { cond: 10, property_id: 1 },
        RspuInstruction::AssertNever { cond: 11, property_id: 2 },
        RspuInstruction::Trap { code: 5 },
        RspuInstruction::TrapIf { cond: 10, code: 3 },
        RspuInstruction::ModeSwitch { mode: 1 },
        RspuInstruction::DeadlineSet { cycles: 5000 },
        RspuInstruction::Nop,
        RspuInstruction::Fence,
        RspuInstruction::Halt,
        RspuInstruction::EmergencyStop,
    ];
    let program = make_program(instrs.clone());
    let words = emit_binary(&program).expect("emit_binary all-formats should succeed");
    assert_eq!(
        words.len(),
        instrs.len(),
        "word count should match instruction count ({})",
        instrs.len()
    );
    for i in 0..words.len() {
        let decoded = decode(words[i]).unwrap_or_else(|e| {
            panic!("decode of word {} ({}) failed: {}", i, instrs[i].mnemonic(), e.message());
        });
        assert_eq!(
            decoded,
            instrs[i],
            "instruction {} ({}) mismatch after emit_binary roundtrip",
            i,
            instrs[i].mnemonic()
        );
    }
}

// ===========================================================================
// Section 17: Edge cases and corner values
// ===========================================================================

#[test]
fn test_load_imm_zero_value_zero_width() {
    let instr = RspuInstruction::LoadImm { dst: 0, value: 0, width: 0 };
    roundtrip_check(&instr, "LoadImm value=0, width=0");
}

#[test]
fn test_load_imm_max_width_clamped() {
    // Width > 255 is clamped to 255 during encoding (u8 field).
    // After roundtrip, width will be 255 regardless of input.
    let instr_in = RspuInstruction::LoadImm { dst: 0, value: 0, width: 1000 };
    let enc = encode(&instr_in).expect("LoadImm width>255 should encode (clamped)");
    let decoded = decode(enc.0).expect("LoadImm decode should succeed");
    if let RspuInstruction::LoadImm { width, .. } = decoded {
        assert_eq!(
            width, 255,
            "LoadImm width > 255 should be clamped to 255 after roundtrip, got {}",
            width
        );
    } else {
        panic!("decoded instruction should be LoadImm");
    }
}

#[test]
fn test_mov_same_register() {
    // MOV R5, R5 (no-op move) should still roundtrip correctly.
    let instr = RspuInstruction::Mov { dst: 5, src: 5 };
    roundtrip_check(&instr, "MOV same register (R5, R5)");
}

#[test]
fn test_guard_and_same_operands() {
    let instr = RspuInstruction::GuardAnd { dst: 0, a: 0, b: 0 };
    roundtrip_check(&instr, "GUARD_AND all same guard");
}

#[test]
fn test_guard_or_same_operands() {
    let instr = RspuInstruction::GuardOr { dst: 0, a: 0, b: 0 };
    roundtrip_check(&instr, "GUARD_OR all same guard");
}

#[test]
fn test_alu_all_zero_operands() {
    let instr = RspuInstruction::Alu { op: AluOp::Add, dst: 0, a: 0, b: 0 };
    roundtrip_check(&instr, "ALU Add all-zero");
}

#[test]
fn test_alu_imm_zero_immediate() {
    let instr = RspuInstruction::AluImm { op: AluOp::Add, dst: 0, a: 0, imm: 0 };
    roundtrip_check(&instr, "ALU_IMM Add imm=0");
}

#[test]
fn test_trap_code_zero() {
    let instr = RspuInstruction::Trap { code: 0 };
    roundtrip_check(&instr, "Trap code=0");
}

#[test]
fn test_trap_code_max() {
    let instr = RspuInstruction::Trap { code: 255 };
    roundtrip_check(&instr, "Trap code=255");
}

#[test]
fn test_mode_switch_zero() {
    let instr = RspuInstruction::ModeSwitch { mode: 0 };
    roundtrip_check(&instr, "ModeSwitch mode=0");
}

#[test]
fn test_mode_switch_max() {
    let instr = RspuInstruction::ModeSwitch { mode: 255 };
    roundtrip_check(&instr, "ModeSwitch mode=255");
}

#[test]
fn test_assert_always_zero_property() {
    let instr = RspuInstruction::AssertAlways { cond: 0, property_id: 0 };
    roundtrip_check(&instr, "AssertAlways cond=0, prop=0");
}

#[test]
fn test_assert_never_max_property() {
    let instr = RspuInstruction::AssertNever { cond: 255, property_id: 0x3_FFFF };
    roundtrip_check(&instr, "AssertNever max cond and property");
}

#[test]
fn test_deadline_set_zero() {
    let instr = RspuInstruction::DeadlineSet { cycles: 0 };
    roundtrip_check(&instr, "DeadlineSet cycles=0");
}

#[test]
fn test_prev_zero_delay() {
    let instr = RspuInstruction::Prev { dst: 192, signal: 0, delay: 0 };
    roundtrip_check(&instr, "Prev delay=0");
}

#[test]
fn test_prev_max_delay() {
    let instr = RspuInstruction::Prev { dst: 192, signal: 5, delay: 1023 };
    roundtrip_check(&instr, "Prev max delay=1023");
}

// ===========================================================================
// Section 18: 10-bit immediate boundary sweep
// ===========================================================================

#[test]
fn test_imm10_boundary_sweep_load_input() {
    // Test the exact boundary: 1023 succeeds, 1024 fails.
    let ok_instr = RspuInstruction::LoadInput { dst: 0, port: 1023 };
    assert!(encode(&ok_instr).is_ok(), "LoadInput port=1023 (10-bit max) should succeed");
    roundtrip_check(&ok_instr, "LoadInput port=1023 boundary");

    let fail_instr = RspuInstruction::LoadInput { dst: 0, port: 1024 };
    assert!(encode(&fail_instr).is_err(), "LoadInput port=1024 should fail");
}

#[test]
fn test_imm10_boundary_sweep_sr_init() {
    let ok_instr = RspuInstruction::SrInit { guard: 0, length: 1023, cond: 0 };
    assert!(encode(&ok_instr).is_ok(), "SrInit length=1023 should succeed");
    roundtrip_check(&ok_instr, "SrInit length=1023 boundary");

    let fail_instr = RspuInstruction::SrInit { guard: 0, length: 1024, cond: 0 };
    assert!(encode(&fail_instr).is_err(), "SrInit length=1024 should fail");
}

#[test]
fn test_imm10_boundary_sweep_ctr_init() {
    let ok_instr = RspuInstruction::CtrInit { guard: 0, target: 1023, cond: 0 };
    assert!(encode(&ok_instr).is_ok(), "CtrInit target=1023 should succeed");
    roundtrip_check(&ok_instr, "CtrInit target=1023 boundary");

    let fail_instr = RspuInstruction::CtrInit { guard: 0, target: 1024, cond: 0 };
    assert!(encode(&fail_instr).is_err(), "CtrInit target=1024 should fail");
}

// ===========================================================================
// Section 19: Word-level encoding uniqueness
// ===========================================================================

#[test]
fn test_distinct_opcodes_produce_distinct_words() {
    // Two instructions with different opcodes but otherwise identical fields
    // must produce different 32-bit words.
    let nop_word = encode(&RspuInstruction::Nop).expect("NOP encode").0;
    let fence_word = encode(&RspuInstruction::Fence).expect("FENCE encode").0;
    let halt_word = encode(&RspuInstruction::Halt).expect("HALT encode").0;
    let estop_word = encode(&RspuInstruction::EmergencyStop).expect("ESTOP encode").0;

    assert_ne!(nop_word, fence_word, "NOP and FENCE should produce different words");
    assert_ne!(nop_word, halt_word, "NOP and HALT should produce different words");
    assert_ne!(nop_word, estop_word, "NOP and EMERGENCY_STOP should produce different words");
    assert_ne!(fence_word, halt_word, "FENCE and HALT should produce different words");
    assert_ne!(fence_word, estop_word, "FENCE and EMERGENCY_STOP should produce different words");
    assert_ne!(halt_word, estop_word, "HALT and EMERGENCY_STOP should produce different words");
}

#[test]
fn test_different_register_values_produce_different_words() {
    let enc1 = encode(&RspuInstruction::Mov { dst: 0, src: 0 }).expect("MOV 0,0 encode");
    let enc2 = encode(&RspuInstruction::Mov { dst: 1, src: 0 }).expect("MOV 1,0 encode");
    let enc3 = encode(&RspuInstruction::Mov { dst: 0, src: 1 }).expect("MOV 0,1 encode");

    assert_ne!(enc1.0, enc2.0, "MOV dst=0 and MOV dst=1 should differ");
    assert_ne!(enc1.0, enc3.0, "MOV src=0 and MOV src=1 should differ");
    assert_ne!(enc2.0, enc3.0, "MOV(1,0) and MOV(0,1) should differ");
}

#[test]
fn test_different_alu_ops_produce_different_words() {
    let enc_add = encode(&RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 })
        .expect("ALU Add encode");
    let enc_sub = encode(&RspuInstruction::Alu { op: AluOp::Sub, dst: 192, a: 0, b: 1 })
        .expect("ALU Sub encode");
    let enc_mul = encode(&RspuInstruction::Alu { op: AluOp::Mul, dst: 192, a: 0, b: 1 })
        .expect("ALU Mul encode");

    assert_ne!(
        enc_add.0, enc_sub.0,
        "ALU Add and Sub with same operands should produce different words"
    );
    assert_ne!(
        enc_add.0, enc_mul.0,
        "ALU Add and Mul with same operands should produce different words"
    );
    assert_ne!(
        enc_sub.0, enc_mul.0,
        "ALU Sub and Mul with same operands should produce different words"
    );
}
