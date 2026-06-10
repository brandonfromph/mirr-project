//! Binary decoding of R-SPU instructions.

#![forbid(unsafe_code)]

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_encoding::format::*;
use crate::emit::rspu_encoding::opcodes::*;
use crate::emit::rspu_isa::*;
use crate::error::MirrError;

/// Decode a 64-bit word back into an R-SPU instruction (R-SPU 2.0).
pub fn decode(word: u64) -> Result<RspuInstruction, MirrError> {
    let opcode = extract_opcode(word);

    match opcode {
        OP_LOAD_INPUT => {
            let (dst, _src, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::LoadInput { dst, port: imm10 })
        }
        OP_STORE_OUTPUT => {
            let (_dst, src, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::StoreOutput { src, port: imm10 })
        }
        OP_MOV => {
            let (dst, src, _, _) = extract_r_fields(word);
            Ok(RspuInstruction::Mov { dst, src })
        }
        OP_LOAD_IMM => {
            let (dst, w, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::LoadImm { dst, value: imm10 as u64, width: w as u32 })
        }
        OP_ALU => {
            let (dst, a, b, op_funct) = extract_r_fields(word);
            let op = funct_to_alu_op(op_funct)?;
            Ok(RspuInstruction::Alu { op, dst, a, b })
        }
        OP_ALU_IMM => {
            let (dst, a, packed) = extract_i_fields(word);
            let op_funct = ((packed >> 10) & 0xF) as u8;
            let imm = (packed & 0x3FF) as u64;
            let op = funct_to_alu_op(op_funct)?;
            Ok(RspuInstruction::AluImm { op, dst, a, imm })
        }
        OP_ALU_UNARY => {
            let (dst, src, _, funct) = extract_r_fields(word);
            let op = funct_to_alu_unary(funct)?;
            Ok(RspuInstruction::AluUnary { op, dst, src })
        }
        OP_SR_INIT => {
            let (guard, cond, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::SrInit { guard: guard as u8, length: imm10 as u32, cond })
        }
        OP_SR_TICK => {
            let (guard, _, _, _) = extract_g_fields(word);
            Ok(RspuInstruction::SrTick { guard })
        }
        OP_SR_QUERY => {
            let (guard, dst, _, _) = extract_g_fields(word);
            Ok(RspuInstruction::SrQuery { dst, guard })
        }
        OP_CTR_INIT => {
            let (guard, cond, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::CtrInit { guard: guard as u8, target: imm10 as u64, cond })
        }
        OP_CTR_TICK => {
            let (guard, _, _, _) = extract_g_fields(word);
            Ok(RspuInstruction::CtrTick { guard })
        }
        OP_CTR_QUERY => {
            let (guard, dst, _, _) = extract_g_fields(word);
            Ok(RspuInstruction::CtrQuery { dst, guard })
        }
        OP_GUARD_AND => {
            let (dst, a, b, _) = extract_r_fields(word);
            Ok(RspuInstruction::GuardAnd { dst: dst as u8, a: a as u8, b: b as u8 })
        }
        OP_GUARD_OR => {
            let (dst, a, b, _) = extract_r_fields(word);
            Ok(RspuInstruction::GuardOr { dst: dst as u8, a: a as u8, b: b as u8 })
        }
        OP_REFLEX_IF => {
            let (guard, dst, src, _) = extract_g_fields(word);
            Ok(RspuInstruction::ReflexIf { guard, dst, src: src as u16 })
        }
        OP_PREV => {
            let (dst, signal, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::Prev { dst, signal, delay: imm10 as u32 })
        }
        OP_EMERGENCY_STOP => Ok(RspuInstruction::EmergencyStop),
        OP_ASSERT_ALWAYS => {
            let cond = (word >> 32) as RegId;
            let property_id = (word & 0xFFFF_FFFF) as u32;
            Ok(RspuInstruction::AssertAlways { cond, property_id })
        }
        OP_ASSERT_NEVER => {
            let cond = (word >> 32) as RegId;
            let property_id = (word & 0xFFFF_FFFF) as u32;
            Ok(RspuInstruction::AssertNever { cond, property_id })
        }
        OP_TRAP => {
            let code = (word & 0xFF) as u8;
            Ok(RspuInstruction::Trap { code })
        }
        OP_TRAP_IF => {
            let cond = ((word >> 8) & 0xFFFF) as RegId;
            let code = (word & 0xFF) as u8;
            Ok(RspuInstruction::TrapIf { cond, code })
        }
        OP_HALT => Ok(RspuInstruction::Halt),
        OP_MODE_SWITCH => {
            let mode = (word & 0xFF) as u8;
            Ok(RspuInstruction::ModeSwitch { mode })
        }
        OP_TAG_LOAD => {
            let (dst, tag, _) = extract_i_fields(word);
            Ok(RspuInstruction::TagLoad { dst, tag: tag as u8 })
        }
        OP_TAG_CHECK => {
            let (src, expected, _) = extract_i_fields(word);
            Ok(RspuInstruction::TagCheck { src, expected: expected as u8 })
        }
        OP_TAG_READ => {
            let (dst, src, _, _) = extract_r_fields(word);
            Ok(RspuInstruction::TagRead { dst, src })
        }
        OP_NOP => Ok(RspuInstruction::Nop),
        OP_FENCE => Ok(RspuInstruction::Fence),
        OP_DEADLINE_SET => {
            let cycles = (word & 0xFFFF_FFFF) as u32;
            Ok(RspuInstruction::DeadlineSet { cycles })
        }
        OP_VERIFY => {
            let cert_offset = (word & 0xFFFF_FFFF) as u32;
            Ok(RspuInstruction::Verify { cert_offset })
        }
        OP_CERTIFY => {
            let (dst, _, _, _) = extract_r_fields(word);
            Ok(RspuInstruction::Certify { dst })
        }
        OP_TOTAL_CHECK => {
            let expected_properties = (word & 0xFFFF_FFFF) as u32;
            Ok(RspuInstruction::TotalCheck { expected_properties })
        }
        OP_MATCH => {
            let (dst, src, table_offset) = extract_i_fields(word);
            Ok(RspuInstruction::Match { dst, src, table_offset })
        }
        OP_INTERVAL_LO => {
            let (dst, src1, _, _) = extract_r_fields(word);
            Ok(RspuInstruction::IntervalLo { dst, src: src1 })
        }
        OP_INTERVAL_HI => {
            let (dst, src1, _, _) = extract_r_fields(word);
            Ok(RspuInstruction::IntervalHi { dst, src: src1 })
        }
        OP_INTERVAL_CHECK => {
            let (_dst, src1, src2, _) = extract_r_fields(word);
            Ok(RspuInstruction::IntervalCheck { src: src1, bounds: src2 })
        }
        OP_TAG_BRANCH => {
            let target_pc = (word >> 8) as u32;
            let tag_value = (word & 0xFF) as u8;
            Ok(RspuInstruction::TagBranch { tag_value, target_pc })
        }
        _ => Err(rspu_err(format!("{} unknown opcode {opcode}", crate::error_codes::ec(707)))),
    }
}
