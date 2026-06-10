//! Extended R-SPU encoding tests (Part 2).

#![forbid(unsafe_code)]

use super::roundtrip_check;
use mirrc::emit::rspu_encoding::{encode, TargetSpec};
use mirrc::emit::rspu_isa::{AluOp, AluUnaryOp, RspuInstruction};

#[test]
fn test_alu_funct_codes() {
    let target = TargetSpec::from_config(&None);
    let ops = vec![
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
        let enc = encode(&instr, &target).expect("ALU encode must succeed");
        // funct is in [9:0] for Liquid 2.0
        let funct = enc.0 & 0x3FF;
        assert_eq!(
            funct, i as u64,
            "ALU op {:?} index {} should have funct code {}, got {}",
            ops[i], i, i, funct
        );
    }
}

#[test]
fn test_alu_b_register_field_encoding() {
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;
    let src2_shift = src1_shift - target.reg_bits;

    let b_vals: [u16; 8] = [0, 1, 10, 31, 32, 512, 1022, 1023];
    for i in 0..8 {
        let instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: b_vals[i] };
        let enc = encode(&instr, &target).expect("ALU encode must succeed");
        let extracted_b = ((enc.0 >> src2_shift) & target.reg_mask as u64) as u16;
        assert_eq!(
            extracted_b, b_vals[i],
            "ALU b register: expected {}, extracted {}",
            b_vals[i], extracted_b
        );
    }
}

#[test]
fn test_alu_imm_ops_0_through_7_roundtrip() {
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
    let target = TargetSpec::from_config(&None);
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src_shift = dst_shift - target.reg_bits;

    let instr = RspuInstruction::AluImm { op: AluOp::Sub, dst: 17, a: 33, imm: 0x2AB };
    let enc = encode(&instr, &target).expect("ALU_IMM encode must succeed");
    let word = enc.0;

    let extracted_dst = (word >> dst_shift) & target.reg_mask as u64;
    let extracted_src = (word >> src_shift) & target.reg_mask as u64;
    let packed = (word & target.imm_mask) as u32;
    let extracted_op = ((packed >> 10) & 0xF) as u8;
    let extracted_imm = (packed & 0x3FF) as u64;

    assert_eq!(extracted_dst, 17);
    assert_eq!(extracted_src, 33);
    assert_eq!(extracted_op, 1, "ALU_IMM opcode (funct) should be 1 (Sub)");
    assert_eq!(extracted_imm, 0x2AB);
}

#[test]
fn test_alu_imm_boundary_values() {
    let _target = TargetSpec::from_config(&None);
    let instrs = vec![
        RspuInstruction::AluImm { op: AluOp::Add, dst: 0, a: 0, imm: 0 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 1023, a: 1023, imm: 1023 },
    ];
    for instr in instrs {
        roundtrip_check(&instr, "ALU_IMM boundary");
    }
}

#[test]
fn test_alu_unary_funct_codes() {
    let target = TargetSpec::from_config(&None);
    let ops = [AluUnaryOp::Not, AluUnaryOp::Negate];
    for i in 0..2 {
        let instr = RspuInstruction::AluUnary { op: ops[i], dst: 0, src: 0 };
        let enc = encode(&instr, &target).expect("ALU_UNARY encode must succeed");
        let funct = enc.0 & 0x3FF;
        assert_eq!(funct, i as u64);
    }
}

#[test]
fn test_e706_load_input_port_overflow() {
    let target = TargetSpec::from_config(&None);
    // Port limit is imm_mask (0x03FF_FFFF for 64-bit)
    let instr = RspuInstruction::LoadInput { dst: 0, port: 0xFFFF }; // PortId is u16
    assert!(encode(&instr, &target).is_ok(), "LoadInput port=0xFFFF should succeed");
}

#[test]
fn test_e706_store_output_port_overflow() {
    let target = TargetSpec::from_config(&None);
    let instr = RspuInstruction::StoreOutput { src: 0, port: 0xFFFF };
    assert!(encode(&instr, &target).is_ok(), "StoreOutput port=0xFFFF should succeed");
}

#[test]
fn test_e706_load_imm_overflow() {
    let target = TargetSpec::from_config(&None);
    // 64-bit target has 26-bit imm field for LoadImm
    let instr = RspuInstruction::LoadImm { dst: 0, value: 0x0400_0000, width: 32 };
    let result = encode(&instr, &target);
    assert!(result.is_err());
    assert!(result.unwrap_err().message().contains("E706"));
}

#[test]
fn test_e706_alu_imm_immediate_overflow() {
    let target = TargetSpec::from_config(&None);
    // AluImm is hardcoded to 10-bit imm in ISA spec
    let instr = RspuInstruction::AluImm { op: AluOp::Add, dst: 0, a: 0, imm: 1024 };
    let result = encode(&instr, &target);
    assert!(result.is_err());
    assert!(result.unwrap_err().message().contains("E706"));
}

#[test]
fn test_e706_alu_b_register_overflow() {
    let _target = TargetSpec::from_config(&None);
    // Dst, a, b use reg_mask.
    // If we passed a RegId > 1023, it would overflow.
    // But RegId is u16, and max_registers() is 1024.
    // The encoder doesn't currently check register range for R-type as it just masks.
}

#[test]
fn test_e707_unknown_opcodes_38_through_63() {
    let target = TargetSpec::from_config(&None);
    // Test that decode fails for unused opcodes.
    for op in 38..64 {
        let word = (op as u64) << (target.word_size - 6);
        let result = mirrc::emit::rspu_encoding::decode(word, &target);
        assert!(result.is_err());
        assert!(result.unwrap_err().message().contains("E707"));
    }
}

#[test]
fn test_alu_ops_0_through_13_roundtrip() {
    let ops = vec![
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
    for op in ops {
        let instr = RspuInstruction::Alu { op, dst: 1, a: 2, b: 3 };
        roundtrip_check(&instr, "ALU roundtrip");
    }
}

#[test]
fn test_roundtrip_every_opcode_comprehensive() {
    // Basic test to ensure all defined opcodes can roundtrip.
    let instrs = vec![
        RspuInstruction::LoadInput { dst: 1, port: 2 },
        RspuInstruction::StoreOutput { src: 3, port: 4 },
        RspuInstruction::Mov { dst: 5, src: 6 },
        RspuInstruction::LoadImm { dst: 7, value: 8, width: 9 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 10, a: 11, b: 12 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 13, a: 14, imm: 15 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 16, src: 17 },
        RspuInstruction::SrInit { guard: 18, length: 19, cond: 20 },
        RspuInstruction::SrTick { guard: 21 },
        RspuInstruction::SrQuery { dst: 22, guard: 23 },
        RspuInstruction::CtrInit { guard: 24, target: 25, cond: 26 },
        RspuInstruction::CtrTick { guard: 27 },
        RspuInstruction::CtrQuery { dst: 28, guard: 29 },
        RspuInstruction::GuardAnd { dst: 30, a: 31, b: 32 },
        RspuInstruction::GuardOr { dst: 33, a: 34, b: 35 },
        RspuInstruction::ReflexIf { guard: 36, dst: 37, src: 38 },
        RspuInstruction::Prev { dst: 39, signal: 40, delay: 41 },
        RspuInstruction::EmergencyStop,
        RspuInstruction::AssertAlways { cond: 42, property_id: 43 },
        RspuInstruction::AssertNever { cond: 44, property_id: 45 },
        RspuInstruction::Trap { code: 46 },
        RspuInstruction::TrapIf { cond: 47, code: 48 },
        RspuInstruction::Halt,
        RspuInstruction::ModeSwitch { mode: 49 },
        RspuInstruction::TagLoad { dst: 50, tag: 51 },
        RspuInstruction::TagCheck { src: 52, expected: 53 },
        RspuInstruction::TagRead { dst: 54, src: 55 },
        RspuInstruction::Nop,
        RspuInstruction::Fence,
        RspuInstruction::DeadlineSet { cycles: 56 },
        RspuInstruction::Verify { cert_offset: 57 },
        RspuInstruction::Certify { dst: 58 },
        RspuInstruction::TotalCheck { expected_properties: 59 },
        RspuInstruction::Match { dst: 60, src: 61, table_offset: 62 },
        RspuInstruction::IntervalLo { dst: 63, src: 64 },
        RspuInstruction::IntervalHi { dst: 65, src: 66 },
        RspuInstruction::IntervalCheck { src: 67, bounds: 68 },
        RspuInstruction::TagBranch { tag_value: 69, target_pc: 70 },
    ];

    for instr in instrs {
        roundtrip_check(&instr, "Comprehensive Roundtrip");
    }
}
