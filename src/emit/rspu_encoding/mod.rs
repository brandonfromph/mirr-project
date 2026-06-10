//! Binary encoding/decoding of R-SPU instructions.
//!
//! Bijective mapping between `RspuInstruction` and 64-bit machine words (R-SPU 2.0).

#![forbid(unsafe_code)]

mod decode;
mod format;
pub mod opcodes;

pub use decode::decode;
pub use opcodes::*;

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::*;
use crate::error::MirrError;

use format::*;

// ---------------------------------------------------------------------------
// Encode
// ---------------------------------------------------------------------------

/// A packed R-SPU instruction word (64-bit for R-SPU 2.0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EncodedInstruction(pub u64);

/// Encode a single R-SPU instruction into a 64-bit word.
pub fn encode(instr: &RspuInstruction) -> Result<EncodedInstruction, MirrError> {
    let word = match instr {
        RspuInstruction::TagBranch { tag_value, target_pc } => {
            let imm = (*target_pc << 8) | (*tag_value as u32);
            pack_s_type(OP_TAG_BRANCH, imm)
        }
        // I-type
        RspuInstruction::LoadInput { dst, port } => {
            let p = check_port10(*port, "LOAD_INPUT")?;
            pack_i_type(OP_LOAD_INPUT, *dst, 0, p)
        }
        RspuInstruction::StoreOutput { src, port } => {
            let p = check_port10(*port, "STORE_OUTPUT")?;
            pack_i_type(OP_STORE_OUTPUT, 0, *src, p)
        }
        // R-type
        RspuInstruction::Mov { dst, src } => pack_r_type(OP_MOV, *dst, *src, 0, 0),
        // I-type
        RspuInstruction::LoadImm { dst, value, width } => {
            let imm = check_imm10(*value, "LOAD_IMM")?;
            // Encode width in src field (max 255 fits u8, but we have 16 bits now)
            pack_i_type(OP_LOAD_IMM, *dst, *width as u16, imm)
        }
        // R-type with ALU funct
        RspuInstruction::Alu { op, dst, a, b } => {
            let op_funct = alu_op_to_funct(*op);
            pack_r_type(OP_ALU, *dst, *a, *b, op_funct)
        }
        // I-type
        RspuInstruction::AluImm { op, dst, a, imm } => {
            let op_code = alu_op_to_funct(*op) as u16;
            let packed = (op_code << 10) | (*imm as u16 & 0x3FF);
            pack_i_type(OP_ALU_IMM, *dst, *a, packed)
        }
        // R-type
        RspuInstruction::AluUnary { op, dst, src } => {
            pack_r_type(OP_ALU_UNARY, *dst, *src, 0, alu_unary_to_funct(*op))
        }
        // I-type
        RspuInstruction::SrInit { guard, length, cond } => {
            let imm = check_imm10(*length as u64, "SR_INIT")?;
            pack_i_type(OP_SR_INIT, *guard as u16, *cond, imm)
        }
        // G-type
        RspuInstruction::SrTick { guard } => pack_g_type(OP_SR_TICK, *guard, 0, 0, 0),
        // G-type
        RspuInstruction::SrQuery { dst, guard } => pack_g_type(OP_SR_QUERY, *guard, *dst, 0, 0),
        // I-type
        RspuInstruction::CtrInit { guard, target, cond } => {
            let imm = check_imm10(*target, "CTR_INIT")?;
            pack_i_type(OP_CTR_INIT, *guard as u16, *cond, imm)
        }
        // G-type
        RspuInstruction::CtrTick { guard } => pack_g_type(OP_CTR_TICK, *guard, 0, 0, 0),
        // G-type
        RspuInstruction::CtrQuery { dst, guard } => pack_g_type(OP_CTR_QUERY, *guard, *dst, 0, 0),
        // R-type
        RspuInstruction::GuardAnd { dst, a, b } => pack_r_type(OP_GUARD_AND, *dst as u16, *a as u16, *b as u16, 0),
        RspuInstruction::GuardOr { dst, a, b } => pack_r_type(OP_GUARD_OR, *dst as u16, *a as u16, *b as u16, 0),
        // G-type
        RspuInstruction::ReflexIf { guard, dst, src } => {
            pack_g_type(OP_REFLEX_IF, *guard, *dst, *src as u8, 0)
        }
        // I-type
        RspuInstruction::Prev { dst, signal, delay } => {
            let imm = check_imm10(*delay as u64, "PREV")?;
            pack_i_type(OP_PREV, *dst, *signal, imm)
        }
        // S-type
        RspuInstruction::EmergencyStop => pack_s_type(OP_EMERGENCY_STOP, 0),
        RspuInstruction::AssertAlways { cond, property_id } => {
            // Pack cond into higher bits for R-SPU 2.0
            let op_word = pack_s_type(OP_ASSERT_ALWAYS, *property_id);
            op_word | ((*cond as u64) << 32)
        }
        RspuInstruction::AssertNever { cond, property_id } => {
            let op_word = pack_s_type(OP_ASSERT_NEVER, *property_id);
            op_word | ((*cond as u64) << 32)
        }
        RspuInstruction::Trap { code } => pack_s_type(OP_TRAP, *code as u32),
        RspuInstruction::TrapIf { cond, code } => {
            let imm = ((*cond as u32) << 8) | (*code as u32);
            pack_s_type(OP_TRAP_IF, imm)
        }
        RspuInstruction::Halt => pack_s_type(OP_HALT, 0),
        RspuInstruction::ModeSwitch { mode } => pack_s_type(OP_MODE_SWITCH, *mode as u32),
        RspuInstruction::TagLoad { dst, tag } => pack_i_type(OP_TAG_LOAD, *dst, *tag as u16, 0),
        RspuInstruction::TagCheck { src, expected } => {
            pack_i_type(OP_TAG_CHECK, *src, *expected as u16, 0)
        }
        RspuInstruction::TagRead { dst, src } => pack_r_type(OP_TAG_READ, *dst, *src, 0, 0),
        RspuInstruction::Nop => pack_s_type(OP_NOP, 0),
        RspuInstruction::Fence => pack_s_type(OP_FENCE, 0),
        RspuInstruction::DeadlineSet { cycles } => pack_s_type(OP_DEADLINE_SET, *cycles),
        RspuInstruction::Verify { cert_offset } => pack_s_type(OP_VERIFY, *cert_offset),
        RspuInstruction::Certify { dst } => pack_r_type(OP_CERTIFY, *dst, 0, 0, 0),
        RspuInstruction::TotalCheck { expected_properties } => {
            pack_s_type(OP_TOTAL_CHECK, *expected_properties)
        }
        RspuInstruction::Match { dst, src, table_offset } => {
            pack_i_type(OP_MATCH, *dst, *src, *table_offset)
        }
        RspuInstruction::IntervalLo { dst, src } => pack_r_type(OP_INTERVAL_LO, *dst, *src, 0, 0),
        RspuInstruction::IntervalHi { dst, src } => pack_r_type(OP_INTERVAL_HI, *dst, *src, 0, 0),
        RspuInstruction::IntervalCheck { src, bounds } => {
            pack_r_type(OP_INTERVAL_CHECK, 0, *src, *bounds, 0)
        }
    };
    Ok(EncodedInstruction(word))
}

// ---------------------------------------------------------------------------
// Decode
// ---------------------------------------------------------------------------

/// Emit a sequence of 64-bit words representing an R-SPU program.
pub fn emit_binary(program: &RspuProgram) -> Result<Vec<u64>, MirrError> {
    if program.instructions.len() > MAX_INSTRUCTIONS {
        return Err(rspu_err(format!(
            "{} program has {} instructions, exceeds MAX_INSTRUCTIONS ({MAX_INSTRUCTIONS})",
            crate::error_codes::ec(706),
            program.instructions.len()
        )));
    }
    let mut words = Vec::with_capacity(program.instructions.len());
    for (i, instr) in program.instructions.iter().enumerate() {
        let encoded = encode(instr).map_err(|e| {
            rspu_err(format!(
                "{} encoding instruction {i} ({}): {}",
                crate::error_codes::ec(706),
                instr.mnemonic(),
                e.message()
            ))
        })?;
        words.push(encoded.0);
    }
    Ok(words)
}
