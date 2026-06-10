//! Binary encoding/decoding of R-SPU instructions.
//!
//! Bijective mapping between `RspuInstruction` and 64-bit machine words (R-SPU 2.0).

#![forbid(unsafe_code)]

mod decode;
mod format;
pub mod opcodes;

pub use decode::decode;
pub use format::extract_opcode;
pub use opcodes::*;

pub use crate::emit::rspu_isa::TargetSpec;

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::*;
use crate::error::MirrError;
use format::*;

// ---------------------------------------------------------------------------
// Encode
// ---------------------------------------------------------------------------

/// A packed R-SPU instruction word (variable size, stored in u64).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EncodedInstruction(pub u64);

/// Encode a single R-SPU instruction into a machine word using a target spec.
pub fn encode(
    instr: &RspuInstruction,
    target: &TargetSpec,
) -> Result<EncodedInstruction, MirrError> {
    let word = match instr {
        RspuInstruction::TagBranch { tag_value, target_pc } => {
            let imm = (*target_pc << 8) | (*tag_value as u32);
            pack_s_type(OP_TAG_BRANCH, imm, target)
        }
        // I-type
        RspuInstruction::LoadInput { dst, port } => {
            let p = check_port(*port, "LOAD_INPUT", target)?;
            pack_i_type(OP_LOAD_INPUT, *dst, 0, p, target)
        }
        RspuInstruction::StoreOutput { src, port } => {
            let p = check_port(*port, "STORE_OUTPUT", target)?;
            pack_i_type(OP_STORE_OUTPUT, 0, *src, p, target)
        }
        // R-type
        RspuInstruction::Mov { dst, src } => pack_r_type(OP_MOV, *dst, *src, 0, 0, target),
        // I-type
        RspuInstruction::LoadImm { dst, value, width } => {
            let imm = check_imm(*value, "LOAD_IMM", target)?;
            // Encode width in src field
            pack_i_type(OP_LOAD_IMM, *dst, *width as u16, imm, target)
        }
        // R-type with ALU funct
        RspuInstruction::Alu { op, dst, a, b } => {
            let op_funct = alu_op_to_funct(*op);
            pack_r_type(OP_ALU, *dst, *a, *b, op_funct, target)
        }
        // I-type
        RspuInstruction::AluImm { op, dst, a, imm } => {
            let imm10 = check_imm10(*imm, "ALU_IMM")?; // ALU_IMM is always 10-bit in the ISA spec
            let op_code = alu_op_to_funct(*op) as u16;
            let packed = (op_code << 10) | (imm10 & 0x3FF);
            pack_i_type(OP_ALU_IMM, *dst, *a, packed as u32, target)
        }
        // R-type
        RspuInstruction::AluUnary { op, dst, src } => {
            pack_r_type(OP_ALU_UNARY, *dst, *src, 0, alu_unary_to_funct(*op), target)
        }
        // I-type
        RspuInstruction::SrInit { guard, length, cond } => {
            let imm = check_imm(*length as u64, "SR_INIT", target)?;
            pack_i_type(OP_SR_INIT, *guard as u16, *cond, imm, target)
        }
        // G-type
        RspuInstruction::SrTick { guard } => pack_g_type(OP_SR_TICK, *guard, 0, 0, 0, target),
        // G-type
        RspuInstruction::SrQuery { dst, guard } => {
            pack_g_type(OP_SR_QUERY, *guard, *dst, 0, 0, target)
        }
        // I-type
        RspuInstruction::CtrInit { guard, target: tgt, cond } => {
            let imm = check_imm(*tgt, "CTR_INIT", target)?;
            pack_i_type(OP_CTR_INIT, *guard as u16, *cond, imm, target)
        }
        // G-type
        RspuInstruction::CtrTick { guard } => pack_g_type(OP_CTR_TICK, *guard, 0, 0, 0, target),
        // G-type
        RspuInstruction::CtrQuery { dst, guard } => {
            pack_g_type(OP_CTR_QUERY, *guard, *dst, 0, 0, target)
        }
        // R-type
        RspuInstruction::GuardAnd { dst, a, b } => {
            pack_r_type(OP_GUARD_AND, *dst as u16, *a as u16, *b as u16, 0, target)
        }
        RspuInstruction::GuardOr { dst, a, b } => {
            pack_r_type(OP_GUARD_OR, *dst as u16, *a as u16, *b as u16, 0, target)
        }
        // G-type
        RspuInstruction::ReflexIf { guard, dst, src } => {
            pack_g_type(OP_REFLEX_IF, *guard, *dst, 0, *src as u32, target)
        }
        // I-type
        RspuInstruction::Prev { dst, signal, delay } => {
            let imm = check_imm(*delay as u64, "PREV", target)?;
            pack_i_type(OP_PREV, *dst, *signal, imm, target)
        }
        // S-type
        RspuInstruction::EmergencyStop => pack_s_type(OP_EMERGENCY_STOP, 0, target),
        RspuInstruction::AssertAlways { cond, property_id } => {
            let op_word = pack_s_type(OP_ASSERT_ALWAYS, *property_id, target);
            op_word | ((*cond as u64) << 32)
        }
        RspuInstruction::AssertNever { cond, property_id } => {
            let op_word = pack_s_type(OP_ASSERT_NEVER, *property_id, target);
            op_word | ((*cond as u64) << 32)
        }
        RspuInstruction::Trap { code } => pack_s_type(OP_TRAP, *code as u32, target),
        RspuInstruction::TrapIf { cond, code } => {
            let imm = ((*cond as u32) << 8) | (*code as u32);
            pack_s_type(OP_TRAP_IF, imm, target)
        }
        RspuInstruction::Halt => pack_s_type(OP_HALT, 0, target),
        RspuInstruction::ModeSwitch { mode } => pack_s_type(OP_MODE_SWITCH, *mode as u32, target),
        RspuInstruction::TagLoad { dst, tag } => {
            pack_i_type(OP_TAG_LOAD, *dst, *tag as u16, 0, target)
        }
        RspuInstruction::TagCheck { src, expected } => {
            pack_i_type(OP_TAG_CHECK, *src, *expected as u16, 0, target)
        }
        RspuInstruction::TagRead { dst, src } => pack_r_type(OP_TAG_READ, *dst, *src, 0, 0, target),
        RspuInstruction::Nop => pack_s_type(OP_NOP, 0, target),
        RspuInstruction::Fence => pack_s_type(OP_FENCE, 0, target),
        RspuInstruction::DeadlineSet { cycles } => pack_s_type(OP_DEADLINE_SET, *cycles, target),
        RspuInstruction::Verify { cert_offset } => pack_s_type(OP_VERIFY, *cert_offset, target),
        RspuInstruction::Certify { dst } => pack_r_type(OP_CERTIFY, *dst, 0, 0, 0, target),
        RspuInstruction::TotalCheck { expected_properties } => {
            pack_s_type(OP_TOTAL_CHECK, *expected_properties, target)
        }
        RspuInstruction::Match { dst, src, table_offset } => {
            pack_i_type(OP_MATCH, *dst, *src, *table_offset as u32, target)
        }
        RspuInstruction::IntervalLo { dst, src } => {
            pack_r_type(OP_INTERVAL_LO, *dst, *src, 0, 0, target)
        }
        RspuInstruction::IntervalHi { dst, src } => {
            pack_r_type(OP_INTERVAL_HI, *dst, *src, 0, 0, target)
        }
        RspuInstruction::IntervalCheck { src, bounds } => {
            pack_r_type(OP_INTERVAL_CHECK, 0, *src, *bounds, 0, target)
        }
    };
    Ok(EncodedInstruction(word))
}

// ---------------------------------------------------------------------------
// Decode
// ---------------------------------------------------------------------------

/// Emit a sequence of machine words representing an R-SPU program.
pub fn emit_binary(program: &RspuProgram) -> Result<Vec<u64>, MirrError> {
    if program.instructions.len() > MAX_INSTRUCTIONS {
        return Err(rspu_err(format!(
            "{} program has {} instructions, exceeds MAX_INSTRUCTIONS ({MAX_INSTRUCTIONS})",
            crate::error_codes::ec(706),
            program.instructions.len()
        )));
    }
    let target = TargetSpec::from_config(&program.target);
    let mut words = Vec::with_capacity(program.instructions.len());
    for (i, instr) in program.instructions.iter().enumerate() {
        let encoded = encode(instr, &target).map_err(|e| {
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
