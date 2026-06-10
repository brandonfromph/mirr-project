//! Extended R-SPU encoding tests (Part 3).

#![forbid(unsafe_code)]

// Unused import removed
use mirrc::emit::rspu_encoding::{encode, TargetSpec};
use mirrc::emit::rspu_isa::{AluUnaryOp, RspuInstruction};

#[test]
fn test_r_type_fields_no_overlap() {
    // We use a custom target with 16-bit registers to test "no overlap" at max values.
    let target = TargetSpec {
        name: "Max-16".to_string(),
        word_size: 64,
        reg_bits: 16,
        guard_bits: 8,
        reg_mask: 0xFFFF,
        guard_mask: 0xFF,
        imm_mask: 0x03FF_FFFF,
    };

    let instr = RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 65535, src: 65535 };
    let enc = encode(&instr, &target).expect("AluUnary max fields encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;

    let opcode = (word >> op_shift) & 0x3F;
    let dst = (word >> dst_shift) & 0xFFFF;
    let src1 = (word >> src1_shift) & 0xFFFF;
    let funct = word & 0x3FF;

    assert_eq!(opcode, 6, "AluUnary opcode should be 6");
    assert_eq!(dst, 65535, "dst at max should be 65535");
    assert_eq!(src1, 65535, "src at max should be 65535");
    assert_eq!(funct, 1, "Negate funct should be 1");
}

#[test]
fn test_i_type_fields_no_overlap() {
    let target = TargetSpec {
        name: "Max-16".to_string(),
        word_size: 64,
        reg_bits: 16,
        guard_bits: 8,
        reg_mask: 0xFFFF,
        guard_mask: 0xFF,
        imm_mask: 0x03FF_FFFF,
    };
    let instr = RspuInstruction::LoadImm { dst: 65535, value: 0x03FF_FFFF, width: 65535 };
    let enc = encode(&instr, &target).expect("LoadImm max fields encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src_shift = dst_shift - target.reg_bits;

    let opcode = (word >> op_shift) & 0x3F;
    let dst = (word >> dst_shift) & 0xFFFF;
    let src = (word >> src_shift) & 0xFFFF;
    let imm26 = word & target.imm_mask;

    assert_eq!(opcode, 3, "LoadImm opcode should be 3");
    assert_eq!(dst, 65535, "dst at max should be 65535");
    assert_eq!(src, 65535, "src (width) at max should be 65535");
    assert_eq!(imm26, 0x03FF_FFFF, "imm26 at max should be 0x03FF_FFFF");
}

#[test]
fn test_g_type_fields_no_overlap() {
    let target = TargetSpec {
        name: "Max-16".to_string(),
        word_size: 64,
        reg_bits: 16,
        guard_bits: 8,
        reg_mask: 0xFFFF,
        guard_mask: 0xFF,
        imm_mask: 0x03FF_FFFF,
    };
    let instr = RspuInstruction::ReflexIf { guard: 255, dst: 65535, src: 65535 };
    let enc = encode(&instr, &target).expect("ReflexIf max fields encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;
    let sd_shift = guard_shift - target.reg_bits;
    let guard2_shift = sd_shift - target.guard_bits;

    let opcode = (word >> op_shift) & 0x3F;
    let guard = (word >> guard_shift) & 0xFF;
    let src_dst = (word >> sd_shift) & 0xFFFF;
    let guard2 = (word >> guard2_shift) & 0xFF;
    let funct = word & target.imm_mask;

    assert_eq!(opcode, 15, "ReflexIf opcode should be 15");
    assert_eq!(guard, 255, "guard at max should be 255");
    assert_eq!(src_dst, 65535, "src_dst at max should be 65535");
    assert_eq!(guard2, 255, "guard2 (src) at max should be 255");
    assert_eq!(funct, 0, "ReflexIf funct should be 0");
}

#[test]
fn test_s_type_imm58_max() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::DeadlineSet { cycles: 0xFFFF_FFFF };
    let enc = encode(&instr, &target).expect("DeadlineSet max imm26 encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let opcode = (word >> op_shift) & 0x3F;
    let imm58 = word & 0x03FF_FFFF_FFFF_FFFF;

    assert_eq!(opcode, 29, "DeadlineSet opcode should be 29");
    assert_eq!(imm58, 0xFFFF_FFFF_u64, "imm58 at max should be 0xFFFF_FFFF, got {:#018X}", imm58);
}

#[test]
fn test_assert_always_field_packing() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::AssertAlways { cond: 0xABCD, property_id: 0x1234_5678 };
    let enc = encode(&instr, &target).expect("AssertAlways encode must succeed");
    let word = enc.0;

    let cond = (word >> 32) as u16;
    let id = (word & 0xFFFF_FFFF) as u32;

    assert_eq!(cond, 0xABCD, "AssertAlways cond packing");
    assert_eq!(id, 0x1234_5678, "AssertAlways property_id packing");
}

#[test]
fn test_decode_all_zero_word() {
    let target = TargetSpec::from_config(&None);
    let decoded = mirrc::emit::rspu_encoding::decode(0, &target)
        .expect("all-zero word should decode as LoadInput");
    assert_eq!(decoded, RspuInstruction::LoadInput { dst: 0, port: 0 });
}

#[test]
fn test_encode_decode_deterministic() {
    let target = TargetSpec::from_config(&None);
    let instr =
        RspuInstruction::Alu { op: mirrc::emit::rspu_isa::AluOp::Add, dst: 10, a: 20, b: 30 };
    let enc1 = encode(&instr, &target).expect("first encode should succeed");
    let enc2 = encode(&instr, &target).expect("second encode should succeed");
    assert_eq!(enc1.0, enc2.0);
}

#[test]
fn test_emit_binary_exceeds_max_instructions() {
    use mirrc::emit::rspu_isa::MAX_INSTRUCTIONS;
    let mut instructions = Vec::with_capacity(MAX_INSTRUCTIONS + 1);
    for _ in 0..MAX_INSTRUCTIONS + 1 {
        instructions.push(RspuInstruction::Nop);
    }
    let program = mirrc::emit::rspu_isa::RspuProgram {
        target: None,
        instructions,
        registers_used: 0,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
        certificate: None,
    };
    let result = mirrc::emit::rspu_encoding::emit_binary(&program);
    assert!(result.is_err());
    assert!(result.unwrap_err().message().contains("E706"));
}

#[test]
fn test_emit_binary_exactly_max_instructions() {
    use mirrc::emit::rspu_isa::MAX_INSTRUCTIONS;
    let mut instructions = Vec::with_capacity(MAX_INSTRUCTIONS);
    for _ in 0..MAX_INSTRUCTIONS {
        instructions.push(RspuInstruction::Nop);
    }
    let program = mirrc::emit::rspu_isa::RspuProgram {
        target: None,
        instructions,
        registers_used: 0,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
        certificate: None,
    };
    let result = mirrc::emit::rspu_encoding::emit_binary(&program);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), MAX_INSTRUCTIONS);
}
