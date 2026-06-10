//! Bit-level pack/extract helpers, ALU op mapping, and validation.

#![forbid(unsafe_code)]

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::{AluOp, AluUnaryOp, PortId};
use crate::error::MirrError;

use super::opcodes::IMM10_MAX;

// ---------------------------------------------------------------------------
// Pack helpers (R-SPU 2.0: 64-bit)
// ---------------------------------------------------------------------------

pub(super) fn pack_r_type(opcode: u8, dst: u16, src1: u16, src2: u16, funct: u8) -> u64 {
    ((opcode as u64) << 58)
        | ((dst as u64) << 42)
        | ((src1 as u64) << 26)
        | ((src2 as u64) << 10)
        | (funct as u64 & 0x3FF)
}

pub(super) fn pack_i_type(opcode: u8, dst: u16, src: u16, imm10: u16) -> u64 {
    ((opcode as u64) << 58)
        | ((dst as u64) << 42)
        | ((src as u64) << 26)
        | (imm10 as u64 & 0x03FF_FFFF)
}

pub(super) fn pack_g_type(opcode: u8, guard: u8, src_dst: u16, guard2: u8, funct: u8) -> u64 {
    ((opcode as u64) << 58)
        | ((guard as u64) << 50)
        | ((src_dst as u64) << 34)
        | ((guard2 as u64) << 26)
        | (funct as u64 & 0x03FF_FFFF)
}

pub(super) fn pack_s_type(opcode: u8, imm26: u32) -> u64 {
    ((opcode as u64) << 58) | (imm26 as u64 & 0x03FF_FFFF_FFFF_FFFF)
}

// ---------------------------------------------------------------------------
// Extract helpers (R-SPU 2.0: 64-bit)
// ---------------------------------------------------------------------------

pub(super) fn extract_opcode(word: u64) -> u8 {
    ((word >> 58) & 0x3F) as u8
}

pub(super) fn extract_r_fields(word: u64) -> (u16, u16, u16, u8) {
    let dst = ((word >> 42) & 0xFFFF) as u16;
    let src1 = ((word >> 26) & 0xFFFF) as u16;
    let src2 = ((word >> 10) & 0xFFFF) as u16;
    let funct = (word & 0x3FF) as u8;
    (dst, src1, src2, funct)
}

pub(super) fn extract_i_fields(word: u64) -> (u16, u16, u16) {
    let dst = ((word >> 42) & 0xFFFF) as u16;
    let src = ((word >> 26) & 0xFFFF) as u16;
    let imm = (word & 0x03FF_FFFF) as u16;
    (dst, src, imm)
}

pub(super) fn extract_g_fields(word: u64) -> (u8, u16, u8, u8) {
    let guard = ((word >> 50) & 0xFF) as u8;
    let src_dst = ((word >> 34) & 0xFFFF) as u16;
    let guard2 = ((word >> 26) & 0xFF) as u8;
    let funct = (word & 0x03FF_FFFF) as u8;
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
    }
}

pub(super) fn funct_to_alu_unary(f: u8) -> Result<AluUnaryOp, MirrError> {
    match f {
        0 => Ok(AluUnaryOp::Not),
        1 => Ok(AluUnaryOp::Negate),
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

pub(super) fn check_port10(port: PortId, context: &str) -> Result<u16, MirrError> {
    if port as u64 > IMM10_MAX {
        return Err(rspu_err(format!(
            "{} {context} port {port} exceeds 10-bit immediate max ({IMM10_MAX})",
            crate::error_codes::ec(706)
        )));
    }
    Ok(port)
}
