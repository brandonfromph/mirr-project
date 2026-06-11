//! Bit-level pack/extract helpers, ALU op mapping, and validation.

#![forbid(unsafe_code)]

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::{AluOp, AluUnaryOp, PortId};
use crate::error::MirrError;

use super::opcodes::IMM10_MAX;
use crate::emit::rspu_isa::TargetSpec;

// ---------------------------------------------------------------------------
// Pack helpers (Dynamic based on TargetSpec)
// ---------------------------------------------------------------------------

pub(super) fn pack_r_type(
    opcode: u8,
    dst: u16,
    src1: u16,
    src2: u16,
    funct: u8,
    target: &TargetSpec,
) -> u64 {
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;
    let src2_shift = src1_shift - target.reg_bits;

    ((opcode as u64) << op_shift)
        | (((dst & target.reg_mask) as u64) << dst_shift)
        | (((src1 & target.reg_mask) as u64) << src1_shift)
        | (((src2 & target.reg_mask) as u64) << src2_shift)
        | (funct as u64 & 0x3FF)
}

pub(super) fn pack_i_type(opcode: u8, dst: u16, src: u16, imm: u32, target: &TargetSpec) -> u64 {
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src_shift = dst_shift - target.reg_bits;

    ((opcode as u64) << op_shift)
        | (((dst & target.reg_mask) as u64) << dst_shift)
        | (((src & target.reg_mask) as u64) << src_shift)
        | (imm as u64 & target.imm_mask)
}

pub(super) fn pack_g_type(
    opcode: u8,
    guard: u8,
    src_dst: u16,
    guard2: u8,
    funct: u32,
    target: &TargetSpec,
) -> u64 {
    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;
    let sd_shift = guard_shift - target.reg_bits;
    let guard2_shift = sd_shift - target.guard_bits;

    ((opcode as u64) << op_shift)
        | (((guard & target.guard_mask) as u64) << guard_shift)
        | (((src_dst & target.reg_mask) as u64) << sd_shift)
        | (((guard2 & target.guard_mask) as u64) << guard2_shift)
        | (funct as u64 & target.imm_mask)
}

pub(super) fn pack_s_type(opcode: u8, imm: u32, target: &TargetSpec) -> u64 {
    let op_shift = target.word_size - 6;
    ((opcode as u64) << op_shift) | (imm as u64 & 0x03FF_FFFF_FFFF_FFFF)
}

// ---------------------------------------------------------------------------
// Extract helpers (Dynamic based on TargetSpec)
// ---------------------------------------------------------------------------

pub fn extract_opcode(word: u64) -> u8 {
    // We assume opcode is always top 6 bits of the word size, but for now 64-bit word is our container.
    // If word_size is 32, opcode is at [31:26].
    if (word >> 58) != 0 {
        ((word >> 58) & 0x3F) as u8
    } else {
        ((word >> 26) & 0x3F) as u8
    }
}

pub(super) fn extract_r_fields(word: u64, target: &TargetSpec) -> (u16, u16, u16, u8) {
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src1_shift = dst_shift - target.reg_bits;
    let src2_shift = src1_shift - target.reg_bits;

    let dst = ((word >> dst_shift) & target.reg_mask as u64) as u16;
    let src1 = ((word >> src1_shift) & target.reg_mask as u64) as u16;
    let src2 = ((word >> src2_shift) & target.reg_mask as u64) as u16;
    let funct = (word & 0x3FF) as u8;
    (dst, src1, src2, funct)
}

pub(super) fn extract_i_fields(word: u64, target: &TargetSpec) -> (u16, u16, u32) {
    let op_shift = target.word_size - 6;
    let dst_shift = op_shift - target.reg_bits;
    let src_shift = dst_shift - target.reg_bits;

    let dst = ((word >> dst_shift) & target.reg_mask as u64) as u16;
    let src = ((word >> src_shift) & target.reg_mask as u64) as u16;
    let imm = (word & target.imm_mask) as u32;
    (dst, src, imm)
}

pub(super) fn extract_g_fields(word: u64, target: &TargetSpec) -> (u8, u16, u8, u32) {
    let op_shift = target.word_size - 6;
    let guard_shift = op_shift - target.guard_bits;
    let sd_shift = guard_shift - target.reg_bits;
    let guard2_shift = sd_shift - target.guard_bits;

    let guard = ((word >> guard_shift) & target.guard_mask as u64) as u8;
    let src_dst = ((word >> sd_shift) & target.reg_mask as u64) as u16;
    let guard2 = ((word >> guard2_shift) & target.guard_mask as u64) as u8;
    let funct = (word & target.imm_mask) as u32;
    (guard, src_dst, guard2, funct)
}

pub(super) fn alu_op_to_funct(op: AluOp) -> u8 {
    match op {
        AluOp::Add => 0,
        AluOp::Sub => 1,
        AluOp::Mul => 2,
        AluOp::And => 3,
        AluOp::Or => 4,
        AluOp::Xor => 5,
        AluOp::Shl => 6,
        AluOp::Shr => 7,
        AluOp::Eq => 8,
        AluOp::Ne => 9,
        AluOp::Lt => 10,
        AluOp::Le => 11,
        AluOp::Gt => 12,
        AluOp::Ge => 13,
    }
}

pub(super) fn funct_to_alu_op(f: u8) -> Result<AluOp, MirrError> {
    match f {
        0 => Ok(AluOp::Add),
        1 => Ok(AluOp::Sub),
        2 => Ok(AluOp::Mul),
        3 => Ok(AluOp::And),
        4 => Ok(AluOp::Or),
        5 => Ok(AluOp::Xor),
        6 => Ok(AluOp::Shl),
        7 => Ok(AluOp::Shr),
        8 => Ok(AluOp::Eq),
        9 => Ok(AluOp::Ne),
        10 => Ok(AluOp::Lt),
        11 => Ok(AluOp::Le),
        12 => Ok(AluOp::Gt),
        13 => Ok(AluOp::Ge),
        _ => Err(rspu_err(format!("{} unknown ALU funct code {f}", crate::error_codes::ec(707)))),
    }
}

pub(super) fn alu_unary_to_funct(op: AluUnaryOp) -> u8 {
    match op {
        AluUnaryOp::Not => 0,
        AluUnaryOp::Negate => 1,
        AluUnaryOp::ReductionOr => 2,
    }
}

pub(super) fn funct_to_alu_unary(f: u8) -> Result<AluUnaryOp, MirrError> {
    match f {
        0 => Ok(AluUnaryOp::Not),
        1 => Ok(AluUnaryOp::Negate),
        2 => Ok(AluUnaryOp::ReductionOr),
        _ => Err(rspu_err(format!(
            "{} unknown unary ALU funct code {f}",
            crate::error_codes::ec(707)
        ))),
    }
}

// ---------------------------------------------------------------------------
// Immediate overflow check
// ---------------------------------------------------------------------------

pub(super) fn check_imm10(value: u64, context: &str) -> Result<u16, MirrError> {
    if value > IMM10_MAX {
        return Err(rspu_err(format!(
            "{} {context} value {value} exceeds 10-bit immediate max ({IMM10_MAX})",
            crate::error_codes::ec(706)
        )));
    }
    Ok(value as u16)
}

pub(super) fn check_imm(value: u64, context: &str, target: &TargetSpec) -> Result<u32, MirrError> {
    if value > target.imm_mask {
        return Err(rspu_err(format!(
            "{} {context} value {value} exceeds target immediate max ({})",
            crate::error_codes::ec(706),
            target.imm_mask
        )));
    }
    Ok(value as u32)
}

pub(super) fn check_port(
    port: PortId,
    context: &str,
    target: &TargetSpec,
) -> Result<u32, MirrError> {
    if port as u64 > target.imm_mask {
        return Err(rspu_err(format!(
            "{} {context} port {port} exceeds target immediate max ({})",
            crate::error_codes::ec(706),
            target.imm_mask
        )));
    }
    Ok(port as u32)
}
