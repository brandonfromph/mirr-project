//! Bit-level pack/extract helpers, ALU op mapping, and validation.

#![forbid(unsafe_code)]

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::{AluOp, AluUnaryOp, PortId};
use crate::error::MirrError;

use super::opcodes::IMM10_MAX;

// ---------------------------------------------------------------------------
// Pack helpers
// ---------------------------------------------------------------------------

pub(super) fn pack_r_type(opcode: u8, dst: u8, src1: u8, src2: u8, funct: u8) -> u32 {
    ((opcode as u32) << 26)
        | ((dst as u32) << 18)
        | ((src1 as u32) << 10)
        | ((src2 as u32) << 2)
        | (funct as u32 & 0x3)
}

pub(super) fn pack_i_type(opcode: u8, dst: u8, src: u8, imm10: u16) -> u32 {
    ((opcode as u32) << 26) | ((dst as u32) << 18) | ((src as u32) << 10) | (imm10 as u32 & 0x3FF)
}

pub(super) fn pack_g_type(opcode: u8, guard: u8, src_dst: u8, guard2: u8, funct: u8) -> u32 {
    ((opcode as u32) << 26)
        | ((guard as u32) << 18)
        | ((src_dst as u32) << 10)
        | ((guard2 as u32) << 2)
        | (funct as u32 & 0x3)
}

pub(super) fn pack_s_type(opcode: u8, imm26: u32) -> u32 {
    ((opcode as u32) << 26) | (imm26 & 0x03FF_FFFF)
}

// ---------------------------------------------------------------------------
// Extract helpers
// ---------------------------------------------------------------------------

pub(super) fn extract_opcode(word: u32) -> u8 {
    ((word >> 26) & 0x3F) as u8
}

pub(super) fn extract_r_fields(word: u32) -> (u8, u8, u8, u8) {
    let dst = ((word >> 18) & 0xFF) as u8;
    let src1 = ((word >> 10) & 0xFF) as u8;
    let src2 = ((word >> 2) & 0xFF) as u8;
    let funct = (word & 0x3) as u8;
    (dst, src1, src2, funct)
}

pub(super) fn extract_i_fields(word: u32) -> (u8, u8, u16) {
    let dst = ((word >> 18) & 0xFF) as u8;
    let src = ((word >> 10) & 0xFF) as u8;
    let imm10 = (word & 0x3FF) as u16;
    (dst, src, imm10)
}

pub(super) fn extract_g_fields(word: u32) -> (u8, u8, u8, u8) {
    let guard = ((word >> 18) & 0xFF) as u8;
    let src_dst = ((word >> 10) & 0xFF) as u8;
    let guard2 = ((word >> 2) & 0xFF) as u8;
    let funct = (word & 0x3) as u8;
    (guard, src_dst, guard2, funct)
}

pub(super) fn extract_s_imm26(word: u32) -> u32 {
    word & 0x03FF_FFFF
}

// ---------------------------------------------------------------------------
// ALU op encoding
// ---------------------------------------------------------------------------

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
        _ => Err(rspu_err(format!("[E707] unknown ALU funct code {f}"))),
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
        _ => Err(rspu_err(format!("[E707] unknown unary ALU funct code {f}"))),
    }
}

// ---------------------------------------------------------------------------
// Immediate overflow check
// ---------------------------------------------------------------------------

pub(super) fn check_imm10(value: u64, context: &str) -> Result<u16, MirrError> {
    if value > IMM10_MAX {
        return Err(rspu_err(format!(
            "[E706] {context} value {value} exceeds 10-bit immediate max ({IMM10_MAX})"
        )));
    }
    Ok(value as u16)
}

pub(super) fn check_port10(port: PortId, context: &str) -> Result<u16, MirrError> {
    if port as u64 > IMM10_MAX {
        return Err(rspu_err(format!(
            "[E706] {context} port {port} exceeds 10-bit immediate max ({IMM10_MAX})"
        )));
    }
    Ok(port)
}
