//! R-SPU instruction decoder: 32-bit word to `RspuInstruction`.

#![forbid(unsafe_code)]

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::*;
use crate::error::MirrError;

use super::format::*;
use super::opcodes::*;

pub fn decode(word: u32) -> Result<RspuInstruction, MirrError> {
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
            let (dst, a, imm10) = extract_i_fields(word);
            let b = ((imm10 >> 4) & 0x3F) as u8;
            let op_funct = (imm10 & 0xF) as u8;
            let op = funct_to_alu_op(op_funct)?;
            Ok(RspuInstruction::Alu { op, dst, a, b })
        }
        OP_ALU_IMM => {
            let (dst, a, packed) = extract_i_fields(word);
            let op_funct = ((packed >> 7) & 0x7) as u8;
            let imm7 = (packed & 0x7F) as u64;
            let op = funct_to_alu_op(op_funct)?;
            Ok(RspuInstruction::AluImm { op, dst, a, imm: imm7 })
        }
        OP_ALU_UNARY => {
            let (dst, src, _, funct) = extract_r_fields(word);
            let op = funct_to_alu_unary(funct)?;
            Ok(RspuInstruction::AluUnary { op, dst, src })
        }
        OP_SR_INIT => {
            let (guard, cond, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::SrInit { guard, length: imm10 as u32, cond })
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
            Ok(RspuInstruction::CtrInit { guard, target: imm10 as u64, cond })
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
            Ok(RspuInstruction::GuardAnd { dst, a, b })
        }
        OP_GUARD_OR => {
            let (dst, a, b, _) = extract_r_fields(word);
            Ok(RspuInstruction::GuardOr { dst, a, b })
        }
        OP_REFLEX_IF => {
            let (guard, dst, src, _) = extract_g_fields(word);
            Ok(RspuInstruction::ReflexIf { guard, dst, src })
        }
        OP_PREV => {
            let (dst, signal, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::Prev { dst, signal, delay: imm10 as u32 })
        }
        OP_EMERGENCY_STOP => Ok(RspuInstruction::EmergencyStop),
        OP_ASSERT_ALWAYS => {
            let imm26 = extract_s_imm26(word);
            let cond = ((imm26 >> 18) & 0xFF) as RegId;
            let property_id = imm26 & 0x3_FFFF;
            Ok(RspuInstruction::AssertAlways { cond, property_id })
        }
        OP_ASSERT_NEVER => {
            let imm26 = extract_s_imm26(word);
            let cond = ((imm26 >> 18) & 0xFF) as RegId;
            let property_id = imm26 & 0x3_FFFF;
            Ok(RspuInstruction::AssertNever { cond, property_id })
        }
        OP_TRAP => {
            let imm26 = extract_s_imm26(word);
            Ok(RspuInstruction::Trap { code: imm26 as u8 })
        }
        OP_TRAP_IF => {
            let imm26 = extract_s_imm26(word);
            let cond = ((imm26 >> 8) & 0xFF) as RegId;
            let code = (imm26 & 0xFF) as u8;
            Ok(RspuInstruction::TrapIf { cond, code })
        }
        OP_HALT => Ok(RspuInstruction::Halt),
        OP_MODE_SWITCH => {
            let imm26 = extract_s_imm26(word);
            Ok(RspuInstruction::ModeSwitch { mode: imm26 as u8 })
        }
        OP_TAG_LOAD => {
            let (dst, tag, _imm) = extract_i_fields(word);
            Ok(RspuInstruction::TagLoad { dst, tag })
        }
        OP_TAG_CHECK => {
            let (src, expected, _imm) = extract_i_fields(word);
            Ok(RspuInstruction::TagCheck { src, expected })
        }
        OP_TAG_READ => {
            let (dst, src, _, _) = extract_r_fields(word);
            Ok(RspuInstruction::TagRead { dst, src })
        }
        OP_NOP => Ok(RspuInstruction::Nop),
        OP_FENCE => Ok(RspuInstruction::Fence),
        OP_DEADLINE_SET => {
            let imm26 = extract_s_imm26(word);
            Ok(RspuInstruction::DeadlineSet { cycles: imm26 })
        }
        // MEGA-4: Totality Engine instructions
        OP_VERIFY => {
            let imm26 = extract_s_imm26(word);
            Ok(RspuInstruction::Verify { cert_offset: imm26 })
        }
        OP_CERTIFY => {
            let (dst, _, _, _) = extract_r_fields(word);
            Ok(RspuInstruction::Certify { dst })
        }
        OP_TOTAL_CHECK => {
            let imm26 = extract_s_imm26(word);
            Ok(RspuInstruction::TotalCheck { expected_properties: imm26 })
        }
        // MEGA-5: Symbolic Reasoning instructions
        OP_MATCH => {
            let (dst, src, imm10) = extract_i_fields(word);
            Ok(RspuInstruction::Match { dst, src, table_offset: imm10 })
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
            let imm26 = extract_s_imm26(word);
            let target_pc = (imm26 >> 8) as u32;
            let tag_value = (imm26 & 0xFF) as u8;
            Ok(RspuInstruction::TagBranch { tag_value, target_pc })
        }
        _ => Err(rspu_err(format!("{} unknown opcode {opcode}", crate::error_codes::ec(707)))),
    }
}
