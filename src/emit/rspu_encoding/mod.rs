//! Binary encoding/decoding of R-SPU instructions.
//!
//! Bijective mapping between `RspuInstruction` and 32-bit machine words.

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

/// Encode a single R-SPU instruction into a 32-bit word.
pub fn encode(instr: &RspuInstruction) -> Result<EncodedInstruction, MirrError> {
    let word = match instr {
        RspuInstruction::TagBranch { tag_value, target_pc } => {
            let imm = ((*target_pc as u32) << 8) | (*tag_value as u32);
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
                    "{} ALU register b index {} exceeds 6-bit field max (63); \
                     use MOV to copy to a low register first",
                    crate::error_codes::ec(706),
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
                    "{} ALU_IMM alu_op {} exceeds 3-bit field max (7); \
                     comparison ops (Eq/Ne/Lt/Le/Gt/Ge) require the register ALU form",
                    crate::error_codes::ec(706),
                    op_code
                )));
            }
            let imm7 = if *imm > 127 {
                return Err(rspu_err(format!(
                    "{} ALU_IMM immediate {imm} exceeds 7-bit field max (127)",
                    crate::error_codes::ec(706)
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
                    "{} DEADLINE_SET cycles {} exceeds 26-bit immediate max",
                    crate::error_codes::ec(706),
                    cycles
                )));
            } else {
                *cycles
            };
            pack_s_type(OP_DEADLINE_SET, imm)
        }
        // MEGA-4: Totality Engine instructions
        RspuInstruction::Verify { cert_offset } => {
            let imm = if *cert_offset > 0x03FF_FFFF {
                return Err(rspu_err(format!(
                    "{} VERIFY cert_offset exceeds 26-bit immediate max",
                    crate::error_codes::ec(706)
                )));
            } else {
                *cert_offset
            };
            pack_s_type(OP_VERIFY, imm)
        }
        RspuInstruction::Certify { dst } => pack_r_type(OP_CERTIFY, *dst, 0, 0, 0),
        RspuInstruction::TotalCheck { expected_properties } => {
            let imm = if *expected_properties > 0x03FF_FFFF {
                return Err(rspu_err(format!(
                    "{} TOTAL_CHECK expected_properties exceeds 26-bit immediate max",
                    crate::error_codes::ec(706)
                )));
            } else {
                *expected_properties
            };
            pack_s_type(OP_TOTAL_CHECK, imm)
        }
        // MEGA-5: Symbolic Reasoning tier
        RspuInstruction::Match { dst, src, table_offset } => {
            if *table_offset > 0x03FF {
                return Err(rspu_err(format!(
                    "{} MATCH table_offset exceeds 10-bit immediate max",
                    crate::error_codes::ec(706)
                )));
            }
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

/// Decode a 32-bit word back into an R-SPU instruction.
pub fn emit_binary(program: &RspuProgram) -> Result<Vec<u32>, MirrError> {
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

    // MEGA-4: Totality Engine roundtrip tests
    #[test]
    fn test_v2_roundtrip_verify() {
        roundtrip(&RspuInstruction::Verify { cert_offset: 4096 });
    }

    #[test]
    fn test_v2_roundtrip_certify() {
        roundtrip(&RspuInstruction::Certify { dst: 192 });
    }

    #[test]
    fn test_v2_roundtrip_total_check() {
        roundtrip(&RspuInstruction::TotalCheck { expected_properties: 5 });
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
            certificate: None,
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
