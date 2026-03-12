//! 32-bit fixed-width binary encoding for R-SPU instructions.
//!
//! Every R-SPU instruction encodes to exactly one 32-bit word:
//!
//! ```text
//! [31:26] opcode (6 bits — 64 slots, 30 used)
//! [25:0]  payload (format-specific)
//! ```
//!
//! Four instruction formats:
//!
//! | Format | Layout (bits 25..0)                                            |
//! |--------|---------------------------------------------------------------|
//! | R-type | dst(8) \| src1(8) \| src2(8) \| funct(2)                      |
//! | I-type | dst(8) \| src(8) \| imm10(10)                                 |
//! | G-type | guard(8) \| src_dst(8) \| guard2(8) \| funct(2)               |
//! | S-type | imm26(26)                                                     |
//!
//! All loops are bounded by `MAX_INSTRUCTIONS` (NASA Power-of-10).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::*;
use crate::error::MirrError;

// ---------------------------------------------------------------------------
// Opcode constants (6-bit, 0–63)
// ---------------------------------------------------------------------------

pub const OP_LOAD_INPUT: u8 = 0;
pub const OP_STORE_OUTPUT: u8 = 1;
pub const OP_MOV: u8 = 2;
pub const OP_LOAD_IMM: u8 = 3;
pub const OP_ALU: u8 = 4;
pub const OP_ALU_IMM: u8 = 5;
pub const OP_ALU_UNARY: u8 = 6;
pub const OP_SR_INIT: u8 = 7;
pub const OP_SR_TICK: u8 = 8;
pub const OP_SR_QUERY: u8 = 9;
pub const OP_CTR_INIT: u8 = 10;
pub const OP_CTR_TICK: u8 = 11;
pub const OP_CTR_QUERY: u8 = 12;
pub const OP_GUARD_AND: u8 = 13;
pub const OP_GUARD_OR: u8 = 14;
pub const OP_REFLEX_IF: u8 = 15;
pub const OP_PREV: u8 = 16;
pub const OP_EMERGENCY_STOP: u8 = 17;
pub const OP_ASSERT_ALWAYS: u8 = 18;
pub const OP_ASSERT_NEVER: u8 = 19;
// Reserved for Wave 2 new instructions:
pub const OP_TRAP: u8 = 20;
pub const OP_TRAP_IF: u8 = 21;
pub const OP_HALT: u8 = 22;
pub const OP_MODE_SWITCH: u8 = 23;
pub const OP_TAG_LOAD: u8 = 24;
pub const OP_TAG_CHECK: u8 = 25;
pub const OP_TAG_READ: u8 = 26;
pub const OP_NOP: u8 = 27;
pub const OP_FENCE: u8 = 28;
pub const OP_DEADLINE_SET: u8 = 29;

/// Total number of assigned opcodes (used + reserved).
pub const TOTAL_OPCODES: usize = 30;

/// Maximum value for a 10-bit immediate field.
const IMM10_MAX: u64 = 0x3FF;

// ---------------------------------------------------------------------------
// Encoded instruction newtype
// ---------------------------------------------------------------------------

/// A 32-bit encoded R-SPU instruction word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodedInstruction(pub u32);

// ---------------------------------------------------------------------------
// Pack helpers
// ---------------------------------------------------------------------------

fn pack_r_type(opcode: u8, dst: u8, src1: u8, src2: u8, funct: u8) -> u32 {
    ((opcode as u32) << 26)
        | ((dst as u32) << 18)
        | ((src1 as u32) << 10)
        | ((src2 as u32) << 2)
        | (funct as u32 & 0x3)
}

fn pack_i_type(opcode: u8, dst: u8, src: u8, imm10: u16) -> u32 {
    ((opcode as u32) << 26) | ((dst as u32) << 18) | ((src as u32) << 10) | (imm10 as u32 & 0x3FF)
}

fn pack_g_type(opcode: u8, guard: u8, src_dst: u8, guard2: u8, funct: u8) -> u32 {
    ((opcode as u32) << 26)
        | ((guard as u32) << 18)
        | ((src_dst as u32) << 10)
        | ((guard2 as u32) << 2)
        | (funct as u32 & 0x3)
}

fn pack_s_type(opcode: u8, imm26: u32) -> u32 {
    ((opcode as u32) << 26) | (imm26 & 0x03FF_FFFF)
}

// ---------------------------------------------------------------------------
// Extract helpers
// ---------------------------------------------------------------------------

fn extract_opcode(word: u32) -> u8 {
    ((word >> 26) & 0x3F) as u8
}

fn extract_r_fields(word: u32) -> (u8, u8, u8, u8) {
    let dst = ((word >> 18) & 0xFF) as u8;
    let src1 = ((word >> 10) & 0xFF) as u8;
    let src2 = ((word >> 2) & 0xFF) as u8;
    let funct = (word & 0x3) as u8;
    (dst, src1, src2, funct)
}

fn extract_i_fields(word: u32) -> (u8, u8, u16) {
    let dst = ((word >> 18) & 0xFF) as u8;
    let src = ((word >> 10) & 0xFF) as u8;
    let imm10 = (word & 0x3FF) as u16;
    (dst, src, imm10)
}

fn extract_g_fields(word: u32) -> (u8, u8, u8, u8) {
    let guard = ((word >> 18) & 0xFF) as u8;
    let src_dst = ((word >> 10) & 0xFF) as u8;
    let guard2 = ((word >> 2) & 0xFF) as u8;
    let funct = (word & 0x3) as u8;
    (guard, src_dst, guard2, funct)
}

fn extract_s_imm26(word: u32) -> u32 {
    word & 0x03FF_FFFF
}

// ---------------------------------------------------------------------------
// ALU op encoding
// ---------------------------------------------------------------------------

fn alu_op_to_funct(op: AluOp) -> u8 {
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

fn funct_to_alu_op(f: u8) -> Result<AluOp, MirrError> {
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

fn alu_unary_to_funct(op: AluUnaryOp) -> u8 {
    match op {
        AluUnaryOp::Not => 0,
        AluUnaryOp::Negate => 1,
    }
}

fn funct_to_alu_unary(f: u8) -> Result<AluUnaryOp, MirrError> {
    match f {
        0 => Ok(AluUnaryOp::Not),
        1 => Ok(AluUnaryOp::Negate),
        _ => Err(rspu_err(format!("[E707] unknown unary ALU funct code {f}"))),
    }
}

// ---------------------------------------------------------------------------
// Immediate overflow check
// ---------------------------------------------------------------------------

fn check_imm10(value: u64, context: &str) -> Result<u16, MirrError> {
    if value > IMM10_MAX {
        return Err(rspu_err(format!(
            "[E706] {context} value {value} exceeds 10-bit immediate max ({IMM10_MAX})"
        )));
    }
    Ok(value as u16)
}

fn check_port10(port: PortId, context: &str) -> Result<u16, MirrError> {
    if port as u64 > IMM10_MAX {
        return Err(rspu_err(format!(
            "[E706] {context} port {port} exceeds 10-bit immediate max ({IMM10_MAX})"
        )));
    }
    Ok(port)
}

// ---------------------------------------------------------------------------
// Encode
// ---------------------------------------------------------------------------

/// Encode a single R-SPU instruction into a 32-bit word.
pub fn encode(instr: &RspuInstruction) -> Result<EncodedInstruction, MirrError> {
    let word = match instr {
        // I-type
        RspuInstruction::LoadInput { dst, port } => {
            let p = check_port10(*port, "LOAD_INPUT")?;
            pack_i_type(OP_LOAD_INPUT, *dst, 0, p)
        }
        RspuInstruction::StoreOutput { src, port } => {
            let p = check_port10(*port, "STORE_OUTPUT")?;
            pack_i_type(OP_STORE_OUTPUT, 0, *src, p)
        }
        // R-type (src2=0, funct=0)
        RspuInstruction::Mov { dst, src } => pack_r_type(OP_MOV, *dst, *src, 0, 0),
        // I-type
        RspuInstruction::LoadImm { dst, value, width } => {
            let imm = check_imm10(*value, "LOAD_IMM")?;
            // Encode width in src field (max 255 fits u8)
            let w = if *width > 255 { 255 } else { *width as u8 };
            pack_i_type(OP_LOAD_IMM, *dst, w, imm)
        }
        // R-type with ALU funct in lower bits
        // ALU has 14 ops but only 2 funct bits — encode op in src2 field instead
        RspuInstruction::Alu { op, dst, a, b } => {
            // I-type encoding: dst=dst, src=a, imm10 = (b << 4) | op_code
            // op_code occupies 4 bits [3:0], b occupies 6 bits [9:4].
            // Validate b fits in 6 bits — registers >= 64 would be silently
            // truncated, producing incorrect binary output.
            let op_code = alu_op_to_funct(*op);
            if *b > 63 {
                return Err(rspu_err(format!(
                    "[E706] ALU register b index {} exceeds 6-bit field max (63); \
                     use MOV to copy to a low register first",
                    b
                )));
            }
            let imm = ((*b as u16) << 4) | (op_code as u16);
            pack_i_type(OP_ALU, *dst, *a, imm)
        }
        // I-type
        RspuInstruction::AluImm { op, dst, a, imm } => {
            // Encode: dst=dst, src=a, imm10 = (alu_op << 7) | (imm & 0x7F)
            // This gives 7 bits for immediate (0-127) and 3 bits for op.
            // For larger immediates, user must use LoadImm + ALU register form.
            let op_code = alu_op_to_funct(*op) as u16;
            if op_code > 7 {
                return Err(rspu_err(format!(
                    "[E706] ALU_IMM alu_op {} exceeds 3-bit field max (7); \
                     comparison ops (Eq/Ne/Lt/Le/Gt/Ge) require the register ALU form",
                    op_code
                )));
            }
            let imm7 = if *imm > 127 {
                return Err(rspu_err(format!(
                    "[E706] ALU_IMM immediate {imm} exceeds 7-bit field max (127)"
                )));
            } else {
                *imm as u16
            };
            let packed = (op_code << 7) | imm7;
            pack_i_type(OP_ALU_IMM, *dst, *a, packed)
        }
        // R-type
        RspuInstruction::AluUnary { op, dst, src } => {
            pack_r_type(OP_ALU_UNARY, *dst, *src, 0, alu_unary_to_funct(*op))
        }
        // I-type
        RspuInstruction::SrInit { guard, length, cond } => {
            let imm = check_imm10(*length as u64, "SR_INIT")?;
            pack_i_type(OP_SR_INIT, *guard, *cond, imm)
        }
        // G-type
        RspuInstruction::SrTick { guard } => pack_g_type(OP_SR_TICK, *guard, 0, 0, 0),
        // G-type
        RspuInstruction::SrQuery { dst, guard } => pack_g_type(OP_SR_QUERY, *guard, *dst, 0, 0),
        // I-type
        RspuInstruction::CtrInit { guard, target, cond } => {
            let imm = check_imm10(*target, "CTR_INIT")?;
            pack_i_type(OP_CTR_INIT, *guard, *cond, imm)
        }
        // G-type
        RspuInstruction::CtrTick { guard } => pack_g_type(OP_CTR_TICK, *guard, 0, 0, 0),
        // G-type
        RspuInstruction::CtrQuery { dst, guard } => pack_g_type(OP_CTR_QUERY, *guard, *dst, 0, 0),
        // R-type (guards as register-like IDs)
        RspuInstruction::GuardAnd { dst, a, b } => pack_r_type(OP_GUARD_AND, *dst, *a, *b, 0),
        RspuInstruction::GuardOr { dst, a, b } => pack_r_type(OP_GUARD_OR, *dst, *a, *b, 0),
        // G-type
        RspuInstruction::ReflexIf { guard, dst, src } => {
            pack_g_type(OP_REFLEX_IF, *guard, *dst, *src, 0)
        }
        // I-type: dst=dst, src=signal, imm10=delay
        RspuInstruction::Prev { dst, signal, delay } => {
            let imm = check_imm10(*delay as u64, "PREV")?;
            pack_i_type(OP_PREV, *dst, *signal, imm)
        }
        // S-type
        RspuInstruction::EmergencyStop => pack_s_type(OP_EMERGENCY_STOP, 0),
        // S-type: imm26 = (cond << 18) | (property_id & 0x3FFFF)
        RspuInstruction::AssertAlways { cond, property_id } => {
            let imm = ((*cond as u32) << 18) | (*property_id & 0x3_FFFF);
            pack_s_type(OP_ASSERT_ALWAYS, imm)
        }
        RspuInstruction::AssertNever { cond, property_id } => {
            let imm = ((*cond as u32) << 18) | (*property_id & 0x3_FFFF);
            pack_s_type(OP_ASSERT_NEVER, imm)
        }
        // ISA v2 extensions
        RspuInstruction::Trap { code } => pack_s_type(OP_TRAP, *code as u32),
        RspuInstruction::TrapIf { cond, code } => {
            let imm = ((*cond as u32) << 8) | (*code as u32);
            pack_s_type(OP_TRAP_IF, imm)
        }
        RspuInstruction::Halt => pack_s_type(OP_HALT, 0),
        RspuInstruction::ModeSwitch { mode } => pack_s_type(OP_MODE_SWITCH, *mode as u32),
        RspuInstruction::TagLoad { dst, tag } => pack_i_type(OP_TAG_LOAD, *dst, *tag, 0),
        RspuInstruction::TagCheck { src, expected } => {
            pack_i_type(OP_TAG_CHECK, *src, *expected, 0)
        }
        RspuInstruction::TagRead { dst, src } => pack_r_type(OP_TAG_READ, *dst, *src, 0, 0),
        RspuInstruction::Nop => pack_s_type(OP_NOP, 0),
        RspuInstruction::Fence => pack_s_type(OP_FENCE, 0),
        RspuInstruction::DeadlineSet { cycles } => {
            let imm = if *cycles > 0x03FF_FFFF {
                return Err(rspu_err(format!(
                    "[E706] DEADLINE_SET cycles {} exceeds 26-bit immediate max",
                    cycles
                )));
            } else {
                *cycles
            };
            pack_s_type(OP_DEADLINE_SET, imm)
        }
    };
    Ok(EncodedInstruction(word))
}

// ---------------------------------------------------------------------------
// Decode
// ---------------------------------------------------------------------------

/// Decode a 32-bit word back into an R-SPU instruction.
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
        _ => Err(rspu_err(format!("[E707] unknown opcode {opcode}"))),
    }
}

// ---------------------------------------------------------------------------
// Program-level encoding
// ---------------------------------------------------------------------------

/// Encode all instructions in an R-SPU program to raw 32-bit words.
///
/// Bounded by `MAX_INSTRUCTIONS` (NASA P10).
pub fn emit_binary(program: &RspuProgram) -> Result<Vec<u32>, MirrError> {
    if program.instructions.len() > MAX_INSTRUCTIONS {
        return Err(rspu_err(format!(
            "[E706] program has {} instructions, exceeds MAX_INSTRUCTIONS ({MAX_INSTRUCTIONS})",
            program.instructions.len()
        )));
    }
    let mut words = Vec::with_capacity(program.instructions.len());
    for (i, instr) in program.instructions.iter().enumerate() {
        let encoded = encode(instr).map_err(|e| {
            rspu_err(format!(
                "[E706] encoding instruction {i} ({}): {}",
                instr.mnemonic(),
                e.message()
            ))
        })?;
        words.push(encoded.0);
        if i >= MAX_INSTRUCTIONS {
            break;
        }
    }
    Ok(words)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(instr: &RspuInstruction) {
        let encoded = encode(instr).expect("encode should succeed");
        let decoded = decode(encoded.0).expect("decode should succeed");
        assert_eq!(&decoded, instr, "roundtrip failed for {}", instr.mnemonic());
    }

    #[test]
    fn test_encode_decode_load_input() {
        roundtrip(&RspuInstruction::LoadInput { dst: 5, port: 42 });
    }

    #[test]
    fn test_encode_decode_store_output() {
        roundtrip(&RspuInstruction::StoreOutput { src: 64, port: 7 });
    }

    #[test]
    fn test_encode_decode_mov() {
        roundtrip(&RspuInstruction::Mov { dst: 10, src: 20 });
    }

    #[test]
    fn test_encode_decode_load_imm() {
        roundtrip(&RspuInstruction::LoadImm { dst: 3, value: 255, width: 8 });
    }

    #[test]
    fn test_encode_decode_alu_add() {
        roundtrip(&RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 });
    }

    #[test]
    fn test_encode_decode_alu_unary_not() {
        roundtrip(&RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 192, src: 5 });
    }

    #[test]
    fn test_encode_decode_sr_init() {
        roundtrip(&RspuInstruction::SrInit { guard: 0, length: 5, cond: 10 });
    }

    #[test]
    fn test_encode_decode_sr_tick() {
        roundtrip(&RspuInstruction::SrTick { guard: 2 });
    }

    #[test]
    fn test_encode_decode_sr_query() {
        roundtrip(&RspuInstruction::SrQuery { dst: 192, guard: 2 });
    }

    #[test]
    fn test_encode_decode_emergency_stop() {
        roundtrip(&RspuInstruction::EmergencyStop);
    }

    #[test]
    fn test_encode_decode_reflex_if() {
        roundtrip(&RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 5 });
    }

    #[test]
    fn test_encode_decode_assert_always() {
        roundtrip(&RspuInstruction::AssertAlways { cond: 10, property_id: 42 });
    }

    #[test]
    fn test_encode_decode_assert_never() {
        roundtrip(&RspuInstruction::AssertNever { cond: 11, property_id: 99 });
    }

    #[test]
    fn test_encode_decode_guard_and() {
        roundtrip(&RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 });
    }

    #[test]
    fn test_encode_decode_guard_or() {
        roundtrip(&RspuInstruction::GuardOr { dst: 3, a: 1, b: 2 });
    }

    #[test]
    fn test_encode_decode_ctr_init() {
        roundtrip(&RspuInstruction::CtrInit { guard: 1, target: 100, cond: 5 });
    }

    #[test]
    fn test_encode_decode_ctr_tick() {
        roundtrip(&RspuInstruction::CtrTick { guard: 1 });
    }

    #[test]
    fn test_encode_decode_ctr_query() {
        roundtrip(&RspuInstruction::CtrQuery { dst: 192, guard: 1 });
    }

    #[test]
    fn test_encode_decode_prev() {
        roundtrip(&RspuInstruction::Prev { dst: 192, signal: 5, delay: 3 });
    }

    #[test]
    fn test_encode_decode_alu_imm() {
        roundtrip(&RspuInstruction::AluImm { op: AluOp::Add, dst: 192, a: 0, imm: 42 });
    }

    #[test]
    fn test_roundtrip_all_existing_opcodes() {
        let instructions: Vec<RspuInstruction> = vec![
            RspuInstruction::LoadInput { dst: 0, port: 0 },
            RspuInstruction::StoreOutput { src: 64, port: 0 },
            RspuInstruction::Mov { dst: 1, src: 0 },
            RspuInstruction::LoadImm { dst: 2, value: 100, width: 8 },
            RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 },
            RspuInstruction::AluImm { op: AluOp::Sub, dst: 192, a: 0, imm: 10 },
            RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 192, src: 0 },
            RspuInstruction::SrInit { guard: 0, length: 5, cond: 0 },
            RspuInstruction::SrTick { guard: 0 },
            RspuInstruction::SrQuery { dst: 192, guard: 0 },
            RspuInstruction::CtrInit { guard: 1, target: 100, cond: 5 },
            RspuInstruction::CtrTick { guard: 1 },
            RspuInstruction::CtrQuery { dst: 192, guard: 1 },
            RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
            RspuInstruction::GuardOr { dst: 3, a: 0, b: 1 },
            RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 0 },
            RspuInstruction::Prev { dst: 192, signal: 5, delay: 2 },
            RspuInstruction::EmergencyStop,
            RspuInstruction::AssertAlways { cond: 10, property_id: 1 },
            RspuInstruction::AssertNever { cond: 11, property_id: 2 },
        ];
        for instr in &instructions {
            roundtrip(instr);
        }
    }

    #[test]
    fn test_overflow_returns_e706() {
        let instr = RspuInstruction::LoadImm { dst: 0, value: 2048, width: 16 };
        let result = encode(&instr);
        assert!(result.is_err());
        let msg = result.unwrap_err().message().to_string();
        assert!(msg.contains("E706"), "expected E706, got: {msg}");
    }

    #[test]
    fn test_unknown_opcode_returns_e707() {
        // Opcode 63 is unassigned.
        let word: u32 = 63 << 26;
        let result = decode(word);
        assert!(result.is_err());
        let msg = result.unwrap_err().message().to_string();
        assert!(msg.contains("E707"), "expected E707, got: {msg}");
    }

    #[test]
    fn test_v2_roundtrip_trap() {
        // OP_TRAP (20) is now implemented.
        roundtrip(&RspuInstruction::Trap { code: 5 });
    }

    #[test]
    fn test_v2_roundtrip_halt() {
        roundtrip(&RspuInstruction::Halt);
    }

    #[test]
    fn test_v2_roundtrip_nop() {
        roundtrip(&RspuInstruction::Nop);
    }

    #[test]
    fn test_v2_roundtrip_fence() {
        roundtrip(&RspuInstruction::Fence);
    }

    #[test]
    fn test_v2_roundtrip_trap_if() {
        roundtrip(&RspuInstruction::TrapIf { cond: 10, code: 3 });
    }

    #[test]
    fn test_v2_roundtrip_mode_switch() {
        roundtrip(&RspuInstruction::ModeSwitch { mode: 1 });
    }

    #[test]
    fn test_v2_roundtrip_tag_load() {
        roundtrip(&RspuInstruction::TagLoad { dst: 192, tag: 2 });
    }

    #[test]
    fn test_v2_roundtrip_tag_check() {
        roundtrip(&RspuInstruction::TagCheck { src: 5, expected: 2 });
    }

    #[test]
    fn test_v2_roundtrip_tag_read() {
        roundtrip(&RspuInstruction::TagRead { dst: 192, src: 5 });
    }

    #[test]
    fn test_v2_roundtrip_deadline_set() {
        roundtrip(&RspuInstruction::DeadlineSet { cycles: 1000 });
    }

    #[test]
    fn test_alu_b_register_overflow_returns_e706() {
        // Register b=192 exceeds the 6-bit field (max 63).
        let instr = RspuInstruction::Alu { op: AluOp::Add, dst: 10, a: 0, b: 192 };
        let result = encode(&instr);
        assert!(result.is_err());
        let msg = result.unwrap_err().message().to_string();
        assert!(msg.contains("E706"), "expected E706, got: {msg}");
        assert!(msg.contains("register b index"), "expected register b message, got: {msg}");
    }

    #[test]
    fn test_alu_imm_comparison_op_overflow_returns_e706() {
        // AluOp::Eq has funct code 8, which exceeds the 3-bit field (max 7).
        let instr = RspuInstruction::AluImm { op: AluOp::Eq, dst: 10, a: 0, imm: 1 };
        let result = encode(&instr);
        assert!(result.is_err());
        let msg = result.unwrap_err().message().to_string();
        assert!(msg.contains("E706"), "expected E706, got: {msg}");
        assert!(msg.contains("alu_op"), "expected alu_op overflow message, got: {msg}");
    }

    #[test]
    fn test_emit_binary_program() {
        let program = RspuProgram {
            instructions: vec![
                RspuInstruction::LoadInput { dst: 0, port: 0 },
                RspuInstruction::StoreOutput { src: 64, port: 0 },
                RspuInstruction::EmergencyStop,
            ],
            registers_used: 2,
            guards_used: 0,
            register_map: vec![],
            guard_map: vec![],
        };
        let words = emit_binary(&program).expect("emit_binary should succeed");
        assert_eq!(words.len(), 3);
        // Verify each word decodes back.
        for (i, word) in words.iter().enumerate() {
            let decoded = decode(*word).expect("decode should succeed");
            assert_eq!(decoded, program.instructions[i]);
        }
    }
}
