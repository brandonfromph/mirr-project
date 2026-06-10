//! Extended R-SPU encoding tests (Part 1).

#![forbid(unsafe_code)]

use mirrc::emit::rspu_encoding::{encode, extract_opcode, TargetSpec};
use mirrc::emit::rspu_isa::RspuInstruction;

#[test]
fn test_opcode_field_position() {
    // Opcode is always top 6 bits of the word size.
    // For 64-bit target, bits [63:58].
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::Nop;
    let enc = encode(&instr, &target).expect("NOP encode must succeed");
    let word = enc.0;

    let opcode = extract_opcode(word);
    assert_eq!(opcode, 27, "NOP opcode should be 27");
}

#[test]
fn test_opcode_field_extracted_correctly_for_all_assigned() {
    let target = TargetSpec::from_config(&None);
    let instrs = vec![
        (RspuInstruction::LoadInput { dst: 0, port: 0 }, 0),
        (RspuInstruction::StoreOutput { src: 0, port: 0 }, 1),
        (RspuInstruction::Mov { dst: 0, src: 0 }, 2),
        (RspuInstruction::LoadImm { dst: 0, value: 0, width: 0 }, 3),
        (RspuInstruction::Halt, 22),
        (RspuInstruction::Nop, 27),
    ];

    for (instr, expected_opcode) in instrs {
        let enc = encode(&instr, &target).expect("encode must succeed");
        let extracted = extract_opcode(enc.0);
        assert_eq!(extracted, expected_opcode, "Opcode mismatch for mnemonic {}", instr.mnemonic());
    }
}

#[test]
fn test_r_type_field_layout() {
    // R-type: [63:58] opcode | [57:48] dst | [47:38] src1 | [37:28] src2 | [9:0] funct
    // (Assuming Liquid profile: word=64, reg=10)
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::Mov { dst: 0xAB, src: 0xCD };
    let enc = encode(&instr, &target).expect("MOV encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;
    let src2_shift = src1_shift - target.reg_bits;

    let opcode = (word >> op_shift) & 0x3F;
    let dst = (word >> dst_shift) & target.reg_mask as u64;
    let src1 = (word >> src1_shift) & target.reg_mask as u64;
    let src2 = (word >> src2_shift) & target.reg_mask as u64;
    let funct = word & 0x3FF;

    assert_eq!(opcode, 2, "MOV opcode should be 2");
    assert_eq!(dst, 0xAB, "MOV dst field should be 0xAB");
    assert_eq!(src1, 0xCD, "MOV src1 field should be 0xCD");
    assert_eq!(src2, 0, "MOV src2 field should be 0 (unused)");
    assert_eq!(funct, 0, "MOV funct field should be 0");
}

#[test]
fn test_i_type_field_layout() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::LoadInput { dst: 0x55, port: 0x2AB };
    let enc = encode(&instr, &target).expect("LoadInput encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src_shift = dst_shift - target.reg_bits;

    let opcode = (word >> op_shift) & 0x3F;
    let dst = (word >> dst_shift) & target.reg_mask as u64;
    let src = (word >> src_shift) & target.reg_mask as u64;
    let imm26 = word & target.imm_mask;

    assert_eq!(opcode, 0, "LoadInput opcode should be 0");
    assert_eq!(dst, 0x55, "LoadInput dst field should be 0x55");
    assert_eq!(src, 0, "LoadInput src field should be 0 (unused for LoadInput)");
    assert_eq!(imm26, 0x2AB, "LoadInput imm26 field should be 0x2AB");
}

#[test]
fn test_g_type_field_layout() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::ReflexIf { guard: 0x11, dst: 0x22, src: 0x33 };
    let enc = encode(&instr, &target).expect("ReflexIf encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;
    let sd_shift = guard_shift - target.reg_bits;
    let guard2_shift = sd_shift - target.guard_bits;

    let opcode = (word >> op_shift) & 0x3F;
    let guard = (word >> guard_shift) & target.guard_mask as u64;
    let src_dst = (word >> sd_shift) & target.reg_mask as u64;
    let guard2 = (word >> guard2_shift) & target.guard_mask as u64;
    let funct = word & target.imm_mask;

    assert_eq!(opcode, 15, "ReflexIf opcode should be 15");
    assert_eq!(guard, 0x11, "ReflexIf guard field should be 0x11");
    assert_eq!(src_dst, 0x22, "ReflexIf src_dst field should be 0x22");
    assert_eq!(guard2, 0x33, "ReflexIf guard2 field should be 0x33");
    assert_eq!(funct, 0, "ReflexIf funct field should be 0");
}

#[test]
fn test_s_type_field_layout() {
    let target = TargetSpec::from_config(&None);
    let enc = encode(&RspuInstruction::EmergencyStop, &target)
        .expect("EmergencyStop encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let opcode = (word >> op_shift) & 0x3F;
    let imm58 = word & 0x03FF_FFFF_FFFF_FFFF;

    assert_eq!(opcode, 17, "EmergencyStop opcode should be 17");
    assert_eq!(imm58, 0, "EmergencyStop imm58 field should be 0");
}

#[test]
fn test_s_type_deadline_set_imm26() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::DeadlineSet { cycles: 0x1234567 };
    let enc = encode(&instr, &target).expect("DeadlineSet encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let opcode = (word >> op_shift) & 0x3F;
    let imm = word & 0x03FF_FFFF; // Use 26-bit mask as that's what we pack

    assert_eq!(opcode, 29, "DeadlineSet opcode should be 29");
    assert_eq!(imm, 0x1234567, "DeadlineSet imm field should be 0x1234567");
}

#[test]
fn test_i_type_field_layout_prev() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::Prev { dst: 0xAA, signal: 0xBB, delay: 0xCC };
    let enc = encode(&instr, &target).expect("Prev encode must succeed");
    let word = enc.0;

    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src_shift = dst_shift - target.reg_bits;

    let opcode = (word >> op_shift) & 0x3F;
    let dst = (word >> dst_shift) & target.reg_mask as u64;
    let src = (word >> src_shift) & target.reg_mask as u64;
    let imm26 = word & target.imm_mask;

    assert_eq!(opcode, 16, "Prev opcode should be 16");
    assert_eq!(dst, 0xAA, "Prev dst field should be 0xAA");
    assert_eq!(src, 0xBB, "Prev signal field should be 0xBB");
    assert_eq!(imm26, 0xCC, "Prev delay field should be 0xCC");
}

#[test]
fn test_load_input_parametric_ports() {
    let target = TargetSpec::from_config(&None);
    for port in [0, 1, 10, 255, 1023] {
        let instr = RspuInstruction::LoadInput { dst: 0, port };
        let enc = encode(&instr, &target).expect("LoadInput encode must succeed");
        let word = enc.0;
        let imm = word & target.imm_mask;
        assert_eq!(imm, port as u64, "Port value mismatch in LoadInput immediate");
    }
}

#[test]
fn test_store_output_parametric_ports() {
    let target = TargetSpec::from_config(&None);
    for port in [0, 1, 10, 255, 1023] {
        let instr = RspuInstruction::StoreOutput { src: 0, port };
        let enc = encode(&instr, &target).expect("StoreOutput encode must succeed");
        let word = enc.0;
        let imm = word & target.imm_mask;
        assert_eq!(imm, port as u64, "Port value mismatch in StoreOutput immediate");
    }
}

#[test]
fn test_mov_parametric_registers() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;

    for (dst, src) in [(0, 1), (1, 0), (255, 512), (1023, 1022)] {
        let instr = RspuInstruction::Mov { dst, src };
        let enc = encode(&instr, &target).expect("Mov encode must succeed");
        let word = enc.0;
        let extracted_dst = (word >> dst_shift) & target.reg_mask as u64;
        let extracted_src = (word >> src1_shift) & target.reg_mask as u64;
        assert_eq!(extracted_dst, dst as u64, "Dst register mismatch in Mov");
        assert_eq!(extracted_src, src as u64, "Src register mismatch in Mov");
    }
}

#[test]
fn test_load_imm_parametric_values() {
    let target = TargetSpec::from_config(&None);
    for value in [0, 1, 0xFFF, 0x3FFFFFF] {
        let instr = RspuInstruction::LoadImm { dst: 0, value, width: 32 };
        let enc = encode(&instr, &target).expect("LoadImm encode must succeed");
        let word = enc.0;
        let imm = word & target.imm_mask;
        assert_eq!(imm, value, "Value mismatch in LoadImm");
    }
}

#[test]
fn test_alu_unary_not_parametric() {
    let target = TargetSpec::from_config(&None);
    let instr =
        RspuInstruction::AluUnary { op: mirrc::emit::rspu_isa::AluUnaryOp::Not, dst: 5, src: 10 };
    let enc = encode(&instr, &target).expect("AluUnary Not encode must succeed");
    let funct = enc.0 & 0x3FF;
    assert_eq!(funct, 0, "AluUnary Not funct should be 0");
}

#[test]
fn test_alu_unary_negate_parametric() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::AluUnary {
        op: mirrc::emit::rspu_isa::AluUnaryOp::Negate,
        dst: 5,
        src: 10,
    };
    let enc = encode(&instr, &target).expect("AluUnary Negate encode must succeed");
    let funct = enc.0 & 0x3FF;
    assert_eq!(funct, 1, "AluUnary Negate funct should be 1");
}

#[test]
fn test_sr_init_parametric_length() {
    let target = TargetSpec::from_config(&None);
    for length in [1, 8, 32, 64] {
        let instr = RspuInstruction::SrInit { guard: 5, length, cond: 10 };
        let enc = encode(&instr, &target).expect("SrInit encode must succeed");
        let word = enc.0;
        let imm = word & target.imm_mask;
        assert_eq!(imm, length as u64, "Length value mismatch in SrInit");
    }
}

#[test]
fn test_sr_tick_parametric_guard() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;

    for guard in [0, 1, 15, 63] {
        let instr = RspuInstruction::SrTick { guard };
        let enc = encode(&instr, &target).expect("SrTick encode must succeed");
        let word = enc.0;
        let extracted_guard = (word >> guard_shift) & target.guard_mask as u64;
        assert_eq!(extracted_guard, guard as u64, "Guard mismatch in SrTick");
    }
}

#[test]
fn test_sr_query_parametric() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;
    let sd_shift = guard_shift - target.reg_bits;

    let instr = RspuInstruction::SrQuery { dst: 100, guard: 5 };
    let enc = encode(&instr, &target).expect("SrQuery encode must succeed");
    let word = enc.0;
    let extracted_guard = (word >> guard_shift) & target.guard_mask as u64;
    let extracted_dst = (word >> sd_shift) & target.reg_mask as u64;
    assert_eq!(extracted_guard, 5);
    assert_eq!(extracted_dst, 100);
}

#[test]
fn test_ctr_init_parametric_target() {
    let target = TargetSpec::from_config(&None);
    for tgt in [1, 1000, 0x3FFFFFF] {
        let instr = RspuInstruction::CtrInit { guard: 2, target: tgt, cond: 5 };
        let enc = encode(&instr, &target).expect("CtrInit encode must succeed");
        let word = enc.0;
        let imm = word & target.imm_mask;
        assert_eq!(imm, tgt, "Target value mismatch in CtrInit");
    }
}

#[test]
fn test_ctr_tick_parametric_guard() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;

    let instr = RspuInstruction::CtrTick { guard: 12 };
    let enc = encode(&instr, &target).expect("CtrTick encode must succeed");
    let extracted_guard = (enc.0 >> guard_shift) & target.guard_mask as u64;
    assert_eq!(extracted_guard, 12);
}

#[test]
fn test_ctr_query_parametric() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;
    let sd_shift = guard_shift - target.reg_bits;

    let instr = RspuInstruction::CtrQuery { dst: 255, guard: 7 };
    let enc = encode(&instr, &target).expect("CtrQuery encode must succeed");
    let extracted_guard = (enc.0 >> guard_shift) & target.guard_mask as u64;
    let extracted_dst = (enc.0 >> sd_shift) & target.reg_mask as u64;
    assert_eq!(extracted_guard, 7);
    assert_eq!(extracted_dst, 255);
}

#[test]
fn test_guard_and_parametric() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;
    let src2_shift = src1_shift - target.reg_bits;

    let instr = RspuInstruction::GuardAnd { dst: 5, a: 1, b: 2 };
    let enc = encode(&instr, &target).expect("GuardAnd encode must succeed");
    let word = enc.0;
    let extracted_dst = (word >> dst_shift) & target.reg_mask as u64;
    let extracted_a = (word >> src1_shift) & target.reg_mask as u64;
    let extracted_b = (word >> src2_shift) & target.reg_mask as u64;
    assert_eq!(extracted_dst, 5);
    assert_eq!(extracted_a, 1);
    assert_eq!(extracted_b, 2);
}

#[test]
fn test_guard_or_parametric() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;
    let src2_shift = src1_shift - target.reg_bits;

    let instr = RspuInstruction::GuardOr { dst: 5, a: 1, b: 2 };
    let enc = encode(&instr, &target).expect("GuardOr encode must succeed");
    let word = enc.0;
    let extracted_dst = (word >> dst_shift) & target.reg_mask as u64;
    let extracted_a = (word >> src1_shift) & target.reg_mask as u64;
    let extracted_b = (word >> src2_shift) & target.reg_mask as u64;
    assert_eq!(extracted_dst, 5);
    assert_eq!(extracted_a, 1);
    assert_eq!(extracted_b, 2);
}

#[test]
fn test_reflex_if_parametric() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;
    let sd_shift = guard_shift - target.reg_bits;

    let instr = RspuInstruction::ReflexIf { guard: 5, dst: 100, src: 200 };
    let enc = encode(&instr, &target).expect("ReflexIf encode must succeed");
    let word = enc.0;
    let extracted_guard = (word >> guard_shift) & target.guard_mask as u64;
    let extracted_dst = (word >> sd_shift) & target.reg_mask as u64;
    let imm = word & target.imm_mask;
    assert_eq!(extracted_guard, 5);
    assert_eq!(extracted_dst, 100);
    assert_eq!(imm, 200);
}

#[test]
fn test_tag_load_parametric() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src_shift = dst_shift - target.reg_bits;

    let instr = RspuInstruction::TagLoad { dst: 10, tag: 5 };
    let enc = encode(&instr, &target).expect("TagLoad encode must succeed");
    let word = enc.0;
    let extracted_dst = (word >> dst_shift) & target.reg_mask as u64;
    let extracted_tag = (word >> src_shift) & target.reg_mask as u64;
    assert_eq!(extracted_dst, 10);
    assert_eq!(extracted_tag, 5);
}

#[test]
fn test_tag_check_parametric() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src_shift = dst_shift - target.reg_bits;

    let instr = RspuInstruction::TagCheck { src: 10, expected: 5 };
    let enc = encode(&instr, &target).expect("TagCheck encode must succeed");
    let word = enc.0;
    let extracted_src = (word >> dst_shift) & target.reg_mask as u64;
    let extracted_expected = (word >> src_shift) & target.reg_mask as u64;
    assert_eq!(extracted_src, 10);
    assert_eq!(extracted_expected, 5);
}

#[test]
fn test_tag_read_parametric() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;

    let instr = RspuInstruction::TagRead { dst: 10, src: 20 };
    let enc = encode(&instr, &target).expect("TagRead encode must succeed");
    let word = enc.0;
    let extracted_dst = (word >> dst_shift) & target.reg_mask as u64;
    let extracted_src = (word >> src1_shift) & target.reg_mask as u64;
    assert_eq!(extracted_dst, 10);
    assert_eq!(extracted_src, 20);
}

#[test]
fn test_prev_parametric_delay() {
    let target = TargetSpec::from_config(&None);
    for delay in [1, 10, 64, 0x3FFFFFF] {
        let instr = RspuInstruction::Prev { dst: 0, signal: 0, delay };
        let enc = encode(&instr, &target).expect("Prev encode must succeed");
        let imm = enc.0 & target.imm_mask;
        assert_eq!(imm, delay as u64, "Delay value mismatch in Prev");
    }
}

#[test]
fn test_trap_parametric_codes() {
    let target = TargetSpec::from_config(&None);
    for code in [0, 1, 127, 255] {
        let instr = RspuInstruction::Trap { code };
        let enc = encode(&instr, &target).expect("Trap encode must succeed");
        let imm = enc.0 & 0xFF;
        assert_eq!(imm, code as u64, "Trap code mismatch");
    }
}

#[test]
fn test_trap_if_parametric() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::TrapIf { cond: 100, code: 42 };
    let enc = encode(&instr, &target).expect("TrapIf encode must succeed");
    let imm = enc.0 & 0xFFFFFF;
    let extracted_cond = (imm >> 8) & 0xFFFF;
    let extracted_code = imm & 0xFF;
    assert_eq!(extracted_cond, 100);
    assert_eq!(extracted_code, 42);
}

#[test]
fn test_assert_always_parametric() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::AssertAlways { cond: 10, property_id: 99 };
    let enc = encode(&instr, &target).expect("AssertAlways encode must succeed");
    let cond = (enc.0 >> 32) as u16;
    let id = (enc.0 & 0xFFFF_FFFF) as u32;
    assert_eq!(cond, 10);
    assert_eq!(id, 99);
}

#[test]
fn test_assert_never_parametric() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::AssertNever { cond: 10, property_id: 99 };
    let enc = encode(&instr, &target).expect("AssertNever encode must succeed");
    let cond = (enc.0 >> 32) as u16;
    let id = (enc.0 & 0xFFFF_FFFF) as u32;
    assert_eq!(cond, 10);
    assert_eq!(id, 99);
}
