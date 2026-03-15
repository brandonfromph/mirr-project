//! Extended integration tests for R-SPU instruction encoding and decoding.
//!
//! Covers all four instruction formats (R-type, I-type, G-type, S-type),
//! opcode encoding/decoding roundtrips for every assigned opcode, field
//! extraction correctness, immediate encoding boundaries, ALU funct code
//! mapping, error paths (E706 overflow, E707 unknown opcode), and
//! program-level `emit_binary` correctness.
//!
//! NASA Power-of-10 compliance:
//! - `#![forbid(unsafe_code)]`
//! - All loops use explicit `MAX_*` bounded iteration constants.
//! - No recursion in any test helper.
//! - Every `assert!` has a descriptive message string.

#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop, clippy::clone_on_copy)]

use nasa_rust_project::emit::rspu_encoding::{decode, emit_binary, encode, EncodedInstruction};
use nasa_rust_project::emit::rspu_isa::{
    AluOp, AluUnaryOp, RspuInstruction, RspuProgram, MAX_INSTRUCTIONS,
};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

/// Maximum register values to iterate over in parametric tests.
const MAX_REG_TEST_VALS: usize = 16;

/// Maximum immediate values to iterate over in parametric tests.
const MAX_IMM_TEST_VALS: usize = 32;

/// Maximum opcodes to iterate over in unknown-opcode tests.
const MAX_OPCODE_SCAN: usize = 64;

/// Maximum instructions in emit_binary stress tests.
const MAX_EMIT_STRESS: usize = 128;

// ---------------------------------------------------------------------------
// Helper: roundtrip encode->decode with descriptive failure message
// ---------------------------------------------------------------------------

fn roundtrip_check(instr: &RspuInstruction, label: &str) {
    let encoded = encode(instr).unwrap_or_else(|e| {
        panic!("roundtrip_check({label}): encode failed: {}", e.message());
    });
    let decoded = decode(encoded.0).unwrap_or_else(|e| {
        panic!("roundtrip_check({label}): decode failed: {}", e.message());
    });
    assert_eq!(
        &decoded, instr,
        "roundtrip_check({label}): decoded instruction does not match original"
    );
}

fn make_program(instructions: Vec<RspuInstruction>) -> RspuProgram {
    RspuProgram {
        instructions,
        registers_used: 256,
        guards_used: 64,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    }
}

// ===========================================================================
// Section 1: Bit-level packing verification
// ===========================================================================

#[test]
fn test_opcode_field_position() {
    // The opcode occupies bits [31:26]. Encoding a NOP (opcode 27) with zero
    // payload must produce exactly 27 << 26.
    let enc = encode(&RspuInstruction::Nop).expect("NOP encode must succeed");
    let expected: u32 = 27 << 26;
    assert_eq!(
        enc.0, expected,
        "NOP word should be opcode 27 shifted to bits [31:26], got {:#010X}",
        enc.0
    );
}

#[test]
fn test_opcode_field_extracted_correctly_for_all_assigned() {
    // Verify that every assigned opcode roundtrips through its position.
    // We use representative instructions for each opcode.
    let test_instrs: [(u8, RspuInstruction); 30] = [
        (0, RspuInstruction::LoadInput { dst: 0, port: 0 }),
        (1, RspuInstruction::StoreOutput { src: 0, port: 0 }),
        (2, RspuInstruction::Mov { dst: 0, src: 0 }),
        (3, RspuInstruction::LoadImm { dst: 0, value: 0, width: 0 }),
        (4, RspuInstruction::Alu { op: AluOp::Add, dst: 0, a: 0, b: 0 }),
        (5, RspuInstruction::AluImm { op: AluOp::Add, dst: 0, a: 0, imm: 0 }),
        (6, RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 0, src: 0 }),
        (7, RspuInstruction::SrInit { guard: 0, length: 0, cond: 0 }),
        (8, RspuInstruction::SrTick { guard: 0 }),
        (9, RspuInstruction::SrQuery { dst: 0, guard: 0 }),
        (10, RspuInstruction::CtrInit { guard: 0, target: 0, cond: 0 }),
        (11, RspuInstruction::CtrTick { guard: 0 }),
        (12, RspuInstruction::CtrQuery { dst: 0, guard: 0 }),
        (13, RspuInstruction::GuardAnd { dst: 0, a: 0, b: 0 }),
        (14, RspuInstruction::GuardOr { dst: 0, a: 0, b: 0 }),
        (15, RspuInstruction::ReflexIf { guard: 0, dst: 0, src: 0 }),
        (16, RspuInstruction::Prev { dst: 0, signal: 0, delay: 0 }),
        (17, RspuInstruction::EmergencyStop),
        (18, RspuInstruction::AssertAlways { cond: 0, property_id: 0 }),
        (19, RspuInstruction::AssertNever { cond: 0, property_id: 0 }),
        (20, RspuInstruction::Trap { code: 0 }),
        (21, RspuInstruction::TrapIf { cond: 0, code: 0 }),
        (22, RspuInstruction::Halt),
        (23, RspuInstruction::ModeSwitch { mode: 0 }),
        (24, RspuInstruction::TagLoad { dst: 0, tag: 0 }),
        (25, RspuInstruction::TagCheck { src: 0, expected: 0 }),
        (26, RspuInstruction::TagRead { dst: 0, src: 0 }),
        (27, RspuInstruction::Nop),
        (28, RspuInstruction::Fence),
        (29, RspuInstruction::DeadlineSet { cycles: 0 }),
    ];

    for (expected_opcode, instr) in &test_instrs {
        let enc = encode(instr).unwrap_or_else(|e| {
            panic!(
                "encoding opcode {} ({}) failed: {}",
                expected_opcode,
                instr.mnemonic(),
                e.message()
            );
        });
        let extracted = (enc.0 >> 26) & 0x3F;
        assert_eq!(
            extracted,
            *expected_opcode as u32,
            "opcode mismatch for {}: expected {}, got {}",
            instr.mnemonic(),
            expected_opcode,
            extracted
        );
    }
}

#[test]
fn test_r_type_field_layout() {
    // R-type: [31:26] opcode | [25:18] dst | [17:10] src1 | [9:2] src2 | [1:0] funct
    // MOV uses R-type: opcode=2, dst, src1=src, src2=0, funct=0
    let instr = RspuInstruction::Mov { dst: 0xAB, src: 0xCD };
    let enc = encode(&instr).expect("MOV encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let dst = (word >> 18) & 0xFF;
    let src1 = (word >> 10) & 0xFF;
    let src2 = (word >> 2) & 0xFF;
    let funct = word & 0x3;

    assert_eq!(opcode, 2, "MOV opcode should be 2");
    assert_eq!(dst, 0xAB, "MOV dst field should be 0xAB");
    assert_eq!(src1, 0xCD, "MOV src1 field should be 0xCD");
    assert_eq!(src2, 0, "MOV src2 field should be 0 (unused)");
    assert_eq!(funct, 0, "MOV funct field should be 0");
}

#[test]
fn test_i_type_field_layout() {
    // I-type: [31:26] opcode | [25:18] dst | [17:10] src | [9:0] imm10
    // LoadInput uses I-type: opcode=0, dst, src=0, imm10=port
    let instr = RspuInstruction::LoadInput { dst: 0x55, port: 0x2AB };
    let enc = encode(&instr).expect("LoadInput encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let dst = (word >> 18) & 0xFF;
    let src = (word >> 10) & 0xFF;
    let imm10 = word & 0x3FF;

    assert_eq!(opcode, 0, "LoadInput opcode should be 0");
    assert_eq!(dst, 0x55, "LoadInput dst field should be 0x55");
    assert_eq!(src, 0, "LoadInput src field should be 0 (unused for LoadInput)");
    assert_eq!(imm10, 0x2AB, "LoadInput imm10 field should be 0x2AB");
}

#[test]
fn test_g_type_field_layout() {
    // G-type: [31:26] opcode | [25:18] guard | [17:10] src_dst | [9:2] guard2 | [1:0] funct
    // ReflexIf uses G-type: opcode=15, guard, src_dst=dst, guard2=src, funct=0
    let instr = RspuInstruction::ReflexIf { guard: 0x11, dst: 0x22, src: 0x33 };
    let enc = encode(&instr).expect("ReflexIf encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let guard = (word >> 18) & 0xFF;
    let src_dst = (word >> 10) & 0xFF;
    let guard2 = (word >> 2) & 0xFF;
    let funct = word & 0x3;

    assert_eq!(opcode, 15, "ReflexIf opcode should be 15");
    assert_eq!(guard, 0x11, "ReflexIf guard field should be 0x11");
    assert_eq!(src_dst, 0x22, "ReflexIf src_dst field should be 0x22");
    assert_eq!(guard2, 0x33, "ReflexIf guard2 field should be 0x33");
    assert_eq!(funct, 0, "ReflexIf funct field should be 0");
}

#[test]
fn test_s_type_field_layout() {
    // S-type: [31:26] opcode | [25:0] imm26
    // EmergencyStop uses S-type: opcode=17, imm26=0
    let enc = encode(&RspuInstruction::EmergencyStop).expect("EmergencyStop encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let imm26 = word & 0x03FF_FFFF;

    assert_eq!(opcode, 17, "EmergencyStop opcode should be 17");
    assert_eq!(imm26, 0, "EmergencyStop imm26 should be 0");
}

#[test]
fn test_s_type_deadline_set_imm26() {
    // DeadlineSet encodes cycles in imm26 field.
    let cycles_val: u32 = 0x03AB_CDEF;
    let instr = RspuInstruction::DeadlineSet { cycles: cycles_val };
    let enc = encode(&instr).expect("DeadlineSet encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let imm26 = word & 0x03FF_FFFF;

    assert_eq!(opcode, 29, "DeadlineSet opcode should be 29");
    assert_eq!(
        imm26, cycles_val,
        "DeadlineSet imm26 should contain the cycles value {:#010X}",
        cycles_val
    );
}

// ===========================================================================
// Section 2: R-type roundtrip tests (parametric)
// ===========================================================================

#[test]
fn test_mov_parametric_registers() {
    // Test MOV with various dst/src register combinations.
    let reg_vals: [u8; MAX_REG_TEST_VALS] =
        [0, 1, 2, 10, 32, 63, 64, 100, 127, 128, 150, 191, 192, 200, 254, 255];
    for i in 0..MAX_REG_TEST_VALS {
        for j in 0..MAX_REG_TEST_VALS {
            let instr = RspuInstruction::Mov { dst: reg_vals[i], src: reg_vals[j] };
            roundtrip_check(&instr, &format!("MOV dst=R{}, src=R{}", reg_vals[i], reg_vals[j]));
        }
    }
}

#[test]
fn test_guard_and_parametric() {
    let guard_vals: [u8; 8] = [0, 1, 2, 5, 10, 20, 50, 63];
    for i in 0..8 {
        for j in 0..8 {
            for k in 0..8 {
                let instr = RspuInstruction::GuardAnd {
                    dst: guard_vals[i],
                    a: guard_vals[j],
                    b: guard_vals[k],
                };
                roundtrip_check(
                    &instr,
                    &format!(
                        "GUARD_AND dst=G{}, a=G{}, b=G{}",
                        guard_vals[i], guard_vals[j], guard_vals[k]
                    ),
                );
            }
        }
    }
}

#[test]
fn test_guard_or_parametric() {
    let guard_vals: [u8; 8] = [0, 1, 3, 7, 15, 31, 48, 63];
    for i in 0..8 {
        for j in 0..8 {
            let instr =
                RspuInstruction::GuardOr { dst: guard_vals[i], a: guard_vals[j], b: guard_vals[i] };
            roundtrip_check(
                &instr,
                &format!(
                    "GUARD_OR dst=G{}, a=G{}, b=G{}",
                    guard_vals[i], guard_vals[j], guard_vals[i]
                ),
            );
        }
    }
}

#[test]
fn test_tag_read_parametric() {
    let reg_vals: [u8; 8] = [0, 1, 64, 128, 192, 200, 254, 255];
    for i in 0..8 {
        for j in 0..8 {
            let instr = RspuInstruction::TagRead { dst: reg_vals[i], src: reg_vals[j] };
            roundtrip_check(
                &instr,
                &format!("TAG_READ dst=R{}, src=R{}", reg_vals[i], reg_vals[j]),
            );
        }
    }
}

#[test]
fn test_alu_unary_not_parametric() {
    let reg_vals: [u8; 8] = [0, 5, 64, 128, 192, 200, 254, 255];
    for i in 0..8 {
        for j in 0..8 {
            let instr = RspuInstruction::AluUnary {
                op: AluUnaryOp::Not,
                dst: reg_vals[i],
                src: reg_vals[j],
            };
            roundtrip_check(
                &instr,
                &format!("ALU_UNARY NOT dst=R{}, src=R{}", reg_vals[i], reg_vals[j]),
            );
        }
    }
}

#[test]
fn test_alu_unary_negate_parametric() {
    let reg_vals: [u8; 8] = [0, 1, 63, 64, 127, 192, 254, 255];
    for i in 0..8 {
        for j in 0..8 {
            let instr = RspuInstruction::AluUnary {
                op: AluUnaryOp::Negate,
                dst: reg_vals[i],
                src: reg_vals[j],
            };
            roundtrip_check(
                &instr,
                &format!("ALU_UNARY NEG dst=R{}, src=R{}", reg_vals[i], reg_vals[j]),
            );
        }
    }
}

// ===========================================================================
// Section 3: I-type roundtrip tests (parametric)
// ===========================================================================

#[test]
fn test_load_input_parametric_ports() {
    // Port is 10-bit (0..1023). Test representative values.
    let port_vals: [u16; MAX_IMM_TEST_VALS] = [
        0, 1, 2, 3, 4, 5, 10, 15, 16, 31, 32, 63, 64, 127, 128, 255, 256, 300, 400, 500, 511, 512,
        600, 700, 800, 900, 950, 1000, 1010, 1020, 1022, 1023,
    ];
    for i in 0..MAX_IMM_TEST_VALS {
        let instr = RspuInstruction::LoadInput { dst: 5, port: port_vals[i] };
        roundtrip_check(&instr, &format!("LoadInput port={}", port_vals[i]));
    }
}

#[test]
fn test_store_output_parametric_ports() {
    let port_vals: [u16; 8] = [0, 1, 42, 127, 255, 512, 1000, 1023];
    for i in 0..8 {
        let instr = RspuInstruction::StoreOutput { src: 64, port: port_vals[i] };
        roundtrip_check(&instr, &format!("StoreOutput port={}", port_vals[i]));
    }
}

#[test]
fn test_load_imm_parametric_values() {
    // value is 10-bit immediate (0..1023), width is stored in src field (8-bit).
    let val_pairs: [(u64, u32); 12] = [
        (0, 1),
        (1, 1),
        (0xFF, 8),
        (100, 8),
        (255, 8),
        (256, 16),
        (511, 16),
        (512, 32),
        (700, 32),
        (1023, 64),
        (0, 255),
        (42, 128),
    ];
    for i in 0..12 {
        let (value, width) = val_pairs[i];
        let instr = RspuInstruction::LoadImm { dst: 3, value, width };
        roundtrip_check(&instr, &format!("LoadImm value={}, width={}", value, width));
    }
}

#[test]
fn test_sr_init_parametric_length() {
    // length is 10-bit immediate (0..1023).
    let lengths: [u32; 8] = [0, 1, 5, 10, 100, 255, 512, 1023];
    for i in 0..8 {
        let instr = RspuInstruction::SrInit { guard: 0, length: lengths[i], cond: 10 };
        roundtrip_check(&instr, &format!("SrInit length={}", lengths[i]));
    }
}

#[test]
fn test_ctr_init_parametric_target() {
    // target is stored as 10-bit immediate (0..1023).
    let targets: [u64; 8] = [0, 1, 10, 50, 100, 500, 999, 1023];
    for i in 0..8 {
        let instr = RspuInstruction::CtrInit { guard: 1, target: targets[i], cond: 5 };
        roundtrip_check(&instr, &format!("CtrInit target={}", targets[i]));
    }
}

#[test]
fn test_prev_parametric_delay() {
    // delay is stored as 10-bit immediate (0..1023).
    let delays: [u32; 8] = [0, 1, 2, 3, 10, 100, 512, 1023];
    for i in 0..8 {
        let instr = RspuInstruction::Prev { dst: 192, signal: 5, delay: delays[i] };
        roundtrip_check(&instr, &format!("Prev delay={}", delays[i]));
    }
}

#[test]
fn test_tag_load_parametric() {
    let tag_vals: [u8; 8] = [0, 1, 2, 5, 10, 50, 127, 255];
    for i in 0..8 {
        let instr = RspuInstruction::TagLoad { dst: 192, tag: tag_vals[i] };
        roundtrip_check(&instr, &format!("TagLoad tag={}", tag_vals[i]));
    }
}

#[test]
fn test_tag_check_parametric() {
    let expected_vals: [u8; 8] = [0, 1, 2, 3, 10, 50, 200, 255];
    for i in 0..8 {
        let instr = RspuInstruction::TagCheck { src: 5, expected: expected_vals[i] };
        roundtrip_check(&instr, &format!("TagCheck expected={}", expected_vals[i]));
    }
}

// ===========================================================================
// Section 4: G-type roundtrip tests (parametric)
// ===========================================================================

#[test]
fn test_sr_tick_parametric_guard() {
    let guard_vals: [u8; 8] = [0, 1, 2, 5, 10, 31, 50, 63];
    for i in 0..8 {
        let instr = RspuInstruction::SrTick { guard: guard_vals[i] };
        roundtrip_check(&instr, &format!("SrTick guard={}", guard_vals[i]));
    }
}

#[test]
fn test_sr_query_parametric() {
    let guard_vals: [u8; 4] = [0, 1, 31, 63];
    let reg_vals: [u8; 4] = [0, 64, 192, 255];
    for i in 0..4 {
        for j in 0..4 {
            let instr = RspuInstruction::SrQuery { dst: reg_vals[j], guard: guard_vals[i] };
            roundtrip_check(
                &instr,
                &format!("SrQuery guard={}, dst=R{}", guard_vals[i], reg_vals[j]),
            );
        }
    }
}

#[test]
fn test_ctr_tick_parametric_guard() {
    let guard_vals: [u8; 8] = [0, 1, 2, 10, 20, 40, 50, 63];
    for i in 0..8 {
        let instr = RspuInstruction::CtrTick { guard: guard_vals[i] };
        roundtrip_check(&instr, &format!("CtrTick guard={}", guard_vals[i]));
    }
}

#[test]
fn test_ctr_query_parametric() {
    let guard_vals: [u8; 4] = [0, 10, 32, 63];
    let reg_vals: [u8; 4] = [0, 64, 128, 255];
    for i in 0..4 {
        for j in 0..4 {
            let instr = RspuInstruction::CtrQuery { dst: reg_vals[j], guard: guard_vals[i] };
            roundtrip_check(
                &instr,
                &format!("CtrQuery guard={}, dst=R{}", guard_vals[i], reg_vals[j]),
            );
        }
    }
}

#[test]
fn test_reflex_if_parametric() {
    let guard_vals: [u8; 4] = [0, 1, 31, 63];
    let reg_vals: [u8; 4] = [0, 64, 128, 255];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let instr = RspuInstruction::ReflexIf {
                    guard: guard_vals[i],
                    dst: reg_vals[j],
                    src: reg_vals[k],
                };
                roundtrip_check(
                    &instr,
                    &format!(
                        "ReflexIf guard={}, dst=R{}, src=R{}",
                        guard_vals[i], reg_vals[j], reg_vals[k]
                    ),
                );
            }
        }
    }
}

// ===========================================================================
// Section 5: S-type roundtrip tests (parametric)
// ===========================================================================

#[test]
fn test_assert_always_parametric() {
    // cond is 8-bit register, property_id is 18-bit (0..0x3FFFF).
    let cond_vals: [u8; 4] = [0, 10, 128, 255];
    let prop_vals: [u32; 4] = [0, 1, 0x1_0000, 0x3_FFFF];
    for i in 0..4 {
        for j in 0..4 {
            let instr =
                RspuInstruction::AssertAlways { cond: cond_vals[i], property_id: prop_vals[j] };
            roundtrip_check(
                &instr,
                &format!("AssertAlways cond=R{}, prop={}", cond_vals[i], prop_vals[j]),
            );
        }
    }
}

#[test]
fn test_assert_never_parametric() {
    let cond_vals: [u8; 4] = [0, 5, 100, 255];
    let prop_vals: [u32; 4] = [0, 42, 0x2_0000, 0x3_FFFF];
    for i in 0..4 {
        for j in 0..4 {
            let instr =
                RspuInstruction::AssertNever { cond: cond_vals[i], property_id: prop_vals[j] };
            roundtrip_check(
                &instr,
                &format!("AssertNever cond=R{}, prop={}", cond_vals[i], prop_vals[j]),
            );
        }
    }
}

#[test]
fn test_trap_parametric_codes() {
    // code is u8 (0..255).
    let code_vals: [u8; 8] = [0, 1, 5, 42, 100, 128, 200, 255];
    for i in 0..8 {
        let instr = RspuInstruction::Trap { code: code_vals[i] };
        roundtrip_check(&instr, &format!("Trap code={}", code_vals[i]));
    }
}

#[test]
fn test_trap_if_parametric() {
    let cond_vals: [u8; 4] = [0, 10, 128, 255];
    let code_vals: [u8; 4] = [0, 1, 42, 255];
    for i in 0..4 {
        for j in 0..4 {
            let instr = RspuInstruction::TrapIf { cond: cond_vals[i], code: code_vals[j] };
            roundtrip_check(
                &instr,
                &format!("TrapIf cond=R{}, code={}", cond_vals[i], code_vals[j]),
            );
        }
    }
}

#[test]
fn test_mode_switch_parametric() {
    let mode_vals: [u8; 8] = [0, 1, 2, 5, 10, 50, 128, 255];
    for i in 0..8 {
        let instr = RspuInstruction::ModeSwitch { mode: mode_vals[i] };
        roundtrip_check(&instr, &format!("ModeSwitch mode={}", mode_vals[i]));
    }
}

#[test]
fn test_deadline_set_parametric() {
    // cycles is 26-bit (0..0x03FF_FFFF).
    let cycle_vals: [u32; 8] =
        [0, 1, 1000, 0xFFFF, 0x1_0000, 0x00FF_FFFF, 0x03AB_CDEF, 0x03FF_FFFF];
    for i in 0..8 {
        let instr = RspuInstruction::DeadlineSet { cycles: cycle_vals[i] };
        roundtrip_check(&instr, &format!("DeadlineSet cycles={}", cycle_vals[i]));
    }
}

#[test]
fn test_s_type_zero_payload_instructions() {
    // EmergencyStop, Halt, Nop, Fence all encode with zero payload.
    let instrs = [
        RspuInstruction::EmergencyStop,
        RspuInstruction::Halt,
        RspuInstruction::Nop,
        RspuInstruction::Fence,
    ];
    for i in 0..4 {
        let enc = encode(&instrs[i]).unwrap_or_else(|e| {
            panic!("encoding {} failed: {}", instrs[i].mnemonic(), e.message());
        });
        let imm26 = enc.0 & 0x03FF_FFFF;
        assert_eq!(
            imm26,
            0,
            "{} should have zero imm26 payload, got {:#010X}",
            instrs[i].mnemonic(),
            imm26
        );
        roundtrip_check(&instrs[i], instrs[i].mnemonic());
    }
}

// ===========================================================================
// Section 6: ALU operation encoding/decoding roundtrips
// ===========================================================================

#[test]
fn test_alu_all_binary_ops_roundtrip() {
    // All 14 binary ALU ops. b must be <= 63 for the 6-bit field.
    let ops: [AluOp; 14] = [
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
        roundtrip_check(&instr, &format!("ALU op_index={}", i));
    }
}

#[test]
fn test_alu_funct_code_ordering() {
    // Verify the funct codes are assigned in order: Add=0, Sub=1, ..., Ge=13.
    let ops: [AluOp; 14] = [
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
        // Encode ALU with this op, then extract the funct bits from imm10[3:0].
        let instr = RspuInstruction::Alu { op: ops[i], dst: 0, a: 0, b: 0 };
        let enc = encode(&instr).expect("ALU encode must succeed");
        let imm10 = enc.0 & 0x3FF;
        let op_funct = imm10 & 0xF;
        assert_eq!(
            op_funct, i as u32,
            "ALU op index {} should have funct code {}, got {}",
            i, i, op_funct
        );
    }
}

#[test]
fn test_alu_b_register_field_encoding() {
    // ALU encodes b in imm10[9:4] (6 bits). Verify b is correctly placed.
    let b_vals: [u8; 8] = [0, 1, 10, 31, 32, 48, 62, 63];
    for i in 0..8 {
        let instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: b_vals[i] };
        let enc = encode(&instr).expect("ALU encode must succeed");
        let imm10 = enc.0 & 0x3FF;
        let extracted_b = ((imm10 >> 4) & 0x3F) as u8;
        assert_eq!(
            extracted_b, b_vals[i],
            "ALU b register: expected {}, extracted {}",
            b_vals[i], extracted_b
        );
    }
}

#[test]
fn test_alu_imm_ops_0_through_7_roundtrip() {
    // AluImm only supports ops with funct code 0..7 (Add through Shr).
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
    // AluImm packs: imm10 = (op_code << 7) | (imm & 0x7F).
    // Verify the encoding manually.
    let instr = RspuInstruction::AluImm { op: AluOp::Sub, dst: 192, a: 0, imm: 100 };
    let enc = encode(&instr).expect("ALU_IMM encode must succeed");
    let packed_imm10 = (enc.0 & 0x3FF) as u16;

    // Sub has funct code 1, imm is 100.
    let expected = (1_u16 << 7) | 100;
    assert_eq!(
        packed_imm10, expected,
        "ALU_IMM packed imm10: expected {:#06X}, got {:#06X}",
        expected, packed_imm10
    );
}

#[test]
fn test_alu_imm_boundary_values() {
    // Test immediate boundary: 0 and 127 (max for 7-bit field).
    let instr_zero = RspuInstruction::AluImm { op: AluOp::Add, dst: 192, a: 0, imm: 0 };
    roundtrip_check(&instr_zero, "ALU_IMM imm=0");

    let instr_max = RspuInstruction::AluImm { op: AluOp::Add, dst: 192, a: 0, imm: 127 };
    roundtrip_check(&instr_max, "ALU_IMM imm=127");
}

#[test]
fn test_alu_unary_funct_codes() {
    // Not=0, Negate=1 in the funct field [1:0].
    let instr_not = RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 0, src: 0 };
    let enc_not = encode(&instr_not).expect("ALU_UNARY NOT encode must succeed");
    let funct_not = enc_not.0 & 0x3;
    assert_eq!(funct_not, 0, "AluUnary NOT funct should be 0, got {}", funct_not);

    let instr_neg = RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 0, src: 0 };
    let enc_neg = encode(&instr_neg).expect("ALU_UNARY NEGATE encode must succeed");
    let funct_neg = enc_neg.0 & 0x3;
    assert_eq!(funct_neg, 1, "AluUnary NEGATE funct should be 1, got {}", funct_neg);
}

// ===========================================================================
// Section 7: Error path tests — E706 overflow
// ===========================================================================

#[test]
fn test_e706_load_imm_overflow() {
    // value 1024 exceeds 10-bit max (1023).
    let instr = RspuInstruction::LoadImm { dst: 0, value: 1024, width: 16 };
    let result = encode(&instr);
    assert!(result.is_err(), "LoadImm value=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "LoadImm overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_load_input_port_overflow() {
    // port 1024 exceeds 10-bit max (1023).
    let instr = RspuInstruction::LoadInput { dst: 0, port: 1024 };
    let result = encode(&instr);
    assert!(result.is_err(), "LoadInput port=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "LoadInput port overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_store_output_port_overflow() {
    let instr = RspuInstruction::StoreOutput { src: 64, port: 1024 };
    let result = encode(&instr);
    assert!(result.is_err(), "StoreOutput port=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(
        msg.contains("E706"),
        "StoreOutput port overflow error should contain E706, got: {msg}"
    );
}

#[test]
fn test_e706_alu_b_register_overflow() {
    // Register b=64 exceeds the 6-bit field max (63).
    let instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 64 };
    let result = encode(&instr);
    assert!(result.is_err(), "ALU b=64 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "ALU b overflow error should contain E706, got: {msg}");
    assert!(
        msg.contains("register b index"),
        "ALU b overflow error should mention 'register b index', got: {msg}"
    );
}

#[test]
fn test_e706_alu_b_register_max_boundary() {
    // b=63 should succeed (max for 6-bit field), b=64 should fail.
    let ok_instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 63 };
    assert!(encode(&ok_instr).is_ok(), "ALU b=63 (max 6-bit) should succeed");

    let fail_instr = RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 64 };
    assert!(encode(&fail_instr).is_err(), "ALU b=64 (exceeds 6-bit) should fail");
}

#[test]
fn test_e706_alu_imm_comparison_ops_rejected() {
    // Comparison ops (Eq=8, Ne=9, Lt=10, Le=11, Gt=12, Ge=13) have funct > 7
    // and cannot be used with AluImm (only 3-bit op field).
    let cmp_ops: [AluOp; 6] = [AluOp::Eq, AluOp::Ne, AluOp::Lt, AluOp::Le, AluOp::Gt, AluOp::Ge];
    for i in 0..6 {
        let instr = RspuInstruction::AluImm { op: cmp_ops[i], dst: 192, a: 0, imm: 1 };
        let result = encode(&instr);
        assert!(result.is_err(), "AluImm with comparison op index {} should fail with E706", i);
        let msg = result.unwrap_err().message().to_string();
        assert!(msg.contains("E706"), "AluImm comparison op error should contain E706, got: {msg}");
    }
}

#[test]
fn test_e706_alu_imm_immediate_overflow() {
    // imm=128 exceeds the 7-bit field max (127).
    let instr = RspuInstruction::AluImm { op: AluOp::Add, dst: 192, a: 0, imm: 128 };
    let result = encode(&instr);
    assert!(result.is_err(), "AluImm imm=128 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(
        msg.contains("E706"),
        "AluImm immediate overflow error should contain E706, got: {msg}"
    );
}

#[test]
fn test_e706_sr_init_length_overflow() {
    // length 1024 exceeds 10-bit max.
    let instr = RspuInstruction::SrInit { guard: 0, length: 1024, cond: 0 };
    let result = encode(&instr);
    assert!(result.is_err(), "SrInit length=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "SrInit length overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_ctr_init_target_overflow() {
    // target 1024 exceeds 10-bit max.
    let instr = RspuInstruction::CtrInit { guard: 0, target: 1024, cond: 0 };
    let result = encode(&instr);
    assert!(result.is_err(), "CtrInit target=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "CtrInit target overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_prev_delay_overflow() {
    // delay 1024 exceeds 10-bit max.
    let instr = RspuInstruction::Prev { dst: 192, signal: 5, delay: 1024 };
    let result = encode(&instr);
    assert!(result.is_err(), "Prev delay=1024 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "Prev delay overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_deadline_set_overflow() {
    // cycles exceeding 26-bit max.
    let instr = RspuInstruction::DeadlineSet { cycles: 0x0400_0000 };
    let result = encode(&instr);
    assert!(result.is_err(), "DeadlineSet cycles=0x0400_0000 should fail with E706");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "DeadlineSet overflow error should contain E706, got: {msg}");
}

#[test]
fn test_e706_deadline_set_boundary() {
    // Max valid: 0x03FF_FFFF should succeed.
    let ok_instr = RspuInstruction::DeadlineSet { cycles: 0x03FF_FFFF };
    assert!(
        encode(&ok_instr).is_ok(),
        "DeadlineSet cycles=0x03FF_FFFF (max 26-bit) should succeed"
    );

    // One over: 0x0400_0000 should fail.
    let fail_instr = RspuInstruction::DeadlineSet { cycles: 0x0400_0000 };
    assert!(encode(&fail_instr).is_err(), "DeadlineSet cycles=0x0400_0000 should fail");
}

// ===========================================================================
// Section 8: Error path tests — E707 unknown opcode
// ===========================================================================

#[test]
fn test_e707_unknown_opcodes_30_through_63() {
    // Opcodes 30..32 are assigned (MEGA-4: VERIFY, CERTIFY, TOTAL_CHECK).
    // Opcodes 33..36 are assigned (MEGA-5: MATCH, INTERVAL_LO, INTERVAL_HI, INTERVAL_CHECK).
    // Opcodes 37..63 are unassigned. Decoding them should produce E707.
    for opcode in 37..MAX_OPCODE_SCAN {
        let word: u32 = (opcode as u32) << 26;
        let result = decode(word);
        assert!(result.is_err(), "decoding unassigned opcode {} should fail with E707", opcode);
        let msg = result.unwrap_err().message().to_string();
        assert!(
            msg.contains("E707"),
            "unassigned opcode {} error should contain E707, got: {msg}",
            opcode
        );
    }
}

#[test]
fn test_e707_max_opcode_63() {
    let word: u32 = 63 << 26;
    let result = decode(word);
    assert!(result.is_err(), "decoding opcode 63 should fail with E707");
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E707"), "opcode 63 error should contain E707, got: {msg}");
}

// ===========================================================================
// Section 9: EncodedInstruction newtype tests
// ===========================================================================

#[test]
fn test_encoded_instruction_equality() {
    let a = EncodedInstruction(0x1234_5678);
    let b = EncodedInstruction(0x1234_5678);
    let c = EncodedInstruction(0x0000_0000);
    assert_eq!(a, b, "identical EncodedInstruction values should be equal");
    assert_ne!(a, c, "different EncodedInstruction values should not be equal");
}

#[test]
fn test_encoded_instruction_copy_clone() {
    let a = EncodedInstruction(0xDEAD_BEEF);
    let b = a; // Copy
    let c = a.clone();
    assert_eq!(a.0, b.0, "EncodedInstruction should be Copy: a and b should have same value");
    assert_eq!(a.0, c.0, "EncodedInstruction Clone should produce equal value");
}

#[test]
fn test_encoded_instruction_debug_format() {
    let enc = EncodedInstruction(42);
    let debug_str = format!("{:?}", enc);
    assert!(
        debug_str.contains("EncodedInstruction"),
        "Debug format should contain 'EncodedInstruction', got: {debug_str}"
    );
    assert!(
        debug_str.contains("42"),
        "Debug format should contain the inner value '42', got: {debug_str}"
    );
}

// ===========================================================================
// Section 10: Comprehensive roundtrip — all 30 opcodes
// ===========================================================================

#[test]
fn test_roundtrip_every_opcode_comprehensive() {
    // One representative instruction per opcode, covering all 30 assigned opcodes.
    let all_instrs: [RspuInstruction; 30] = [
        RspuInstruction::LoadInput { dst: 5, port: 42 },
        RspuInstruction::StoreOutput { src: 64, port: 7 },
        RspuInstruction::Mov { dst: 10, src: 20 },
        RspuInstruction::LoadImm { dst: 3, value: 255, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 },
        RspuInstruction::AluImm { op: AluOp::Sub, dst: 192, a: 5, imm: 10 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 192, src: 5 },
        RspuInstruction::SrInit { guard: 0, length: 5, cond: 10 },
        RspuInstruction::SrTick { guard: 2 },
        RspuInstruction::SrQuery { dst: 192, guard: 2 },
        RspuInstruction::CtrInit { guard: 1, target: 100, cond: 5 },
        RspuInstruction::CtrTick { guard: 1 },
        RspuInstruction::CtrQuery { dst: 192, guard: 1 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::GuardOr { dst: 3, a: 1, b: 2 },
        RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 5 },
        RspuInstruction::Prev { dst: 192, signal: 5, delay: 3 },
        RspuInstruction::EmergencyStop,
        RspuInstruction::AssertAlways { cond: 10, property_id: 42 },
        RspuInstruction::AssertNever { cond: 11, property_id: 99 },
        RspuInstruction::Trap { code: 5 },
        RspuInstruction::TrapIf { cond: 10, code: 3 },
        RspuInstruction::Halt,
        RspuInstruction::ModeSwitch { mode: 1 },
        RspuInstruction::TagLoad { dst: 192, tag: 2 },
        RspuInstruction::TagCheck { src: 5, expected: 2 },
        RspuInstruction::TagRead { dst: 192, src: 5 },
        RspuInstruction::Nop,
        RspuInstruction::Fence,
        RspuInstruction::DeadlineSet { cycles: 1000 },
    ];
    for i in 0..30 {
        roundtrip_check(&all_instrs[i], &format!("opcode_{}_comprehensive", i));
    }
}

// ===========================================================================
// Section 11: Program-level emit_binary tests
// ===========================================================================

#[test]
fn test_emit_binary_empty_program() {
    let program = make_program(vec![]);
    let words = emit_binary(&program).expect("emit_binary on empty program should succeed");
    assert_eq!(words.len(), 0, "empty program should produce zero words");
}

#[test]
fn test_emit_binary_single_instruction() {
    let program = make_program(vec![RspuInstruction::Nop]);
    let words = emit_binary(&program).expect("emit_binary single NOP should succeed");
    assert_eq!(words.len(), 1, "single instruction should produce one word");
    let decoded = decode(words[0]).expect("decode of emitted word should succeed");
    assert_eq!(decoded, RspuInstruction::Nop, "decoded word should be NOP");
}

#[test]
fn test_emit_binary_multi_instruction_roundtrip() {
    let instrs = vec![
        RspuInstruction::LoadInput { dst: 0, port: 0 },
        RspuInstruction::LoadInput { dst: 1, port: 1 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
        RspuInstruction::EmergencyStop,
    ];
    let program = make_program(instrs.clone());
    let words = emit_binary(&program).expect("emit_binary should succeed");
    assert_eq!(words.len(), instrs.len(), "word count should match instruction count");
    for i in 0..words.len() {
        let decoded = decode(words[i]).unwrap_or_else(|e| {
            panic!("decode of word {} failed: {}", i, e.message());
        });
        assert_eq!(decoded, instrs[i], "instruction {} mismatch after emit_binary roundtrip", i);
    }
}

#[test]
fn test_emit_binary_stress_bounded() {
    // Generate MAX_EMIT_STRESS instructions (all NOPs) and verify roundtrip.
    let mut instrs = Vec::with_capacity(MAX_EMIT_STRESS);
    for _i in 0..MAX_EMIT_STRESS {
        instrs.push(RspuInstruction::Nop);
    }
    let program = make_program(instrs);
    let words = emit_binary(&program).expect("emit_binary stress should succeed");
    assert_eq!(
        words.len(),
        MAX_EMIT_STRESS,
        "stress test should produce {} words",
        MAX_EMIT_STRESS
    );
    for i in 0..MAX_EMIT_STRESS {
        let decoded = decode(words[i]).expect("decode in stress test should succeed");
        assert_eq!(decoded, RspuInstruction::Nop, "stress test word {} should decode to NOP", i);
    }
}

#[test]
fn test_emit_binary_exceeds_max_instructions() {
    // A program with MAX_INSTRUCTIONS + 1 instructions should fail.
    let mut instrs = Vec::with_capacity(MAX_INSTRUCTIONS + 1);
    for _i in 0..MAX_INSTRUCTIONS + 1 {
        instrs.push(RspuInstruction::Nop);
    }
    let program = make_program(instrs);
    let result = emit_binary(&program);
    assert!(
        result.is_err(),
        "emit_binary with {} instructions should fail (MAX_INSTRUCTIONS={})",
        MAX_INSTRUCTIONS + 1,
        MAX_INSTRUCTIONS
    );
    let msg = result.unwrap_err().message().to_string();
    assert!(msg.contains("E706"), "emit_binary overflow error should contain E706, got: {msg}");
}

#[test]
fn test_emit_binary_exactly_max_instructions() {
    // Exactly MAX_INSTRUCTIONS should succeed.
    let mut instrs = Vec::with_capacity(MAX_INSTRUCTIONS);
    for _i in 0..MAX_INSTRUCTIONS {
        instrs.push(RspuInstruction::Nop);
    }
    let program = make_program(instrs);
    let result = emit_binary(&program);
    assert!(result.is_ok(), "emit_binary with exactly MAX_INSTRUCTIONS should succeed");
    let words = result.unwrap();
    assert_eq!(
        words.len(),
        MAX_INSTRUCTIONS,
        "emit_binary should produce exactly MAX_INSTRUCTIONS words"
    );
}

// ===========================================================================
// Section 12: Bit-isolation tests — verify fields do not overlap
// ===========================================================================

#[test]
fn test_r_type_fields_no_overlap() {
    // Encode with all fields at max to verify no bit collisions.
    // R-type: dst=0xFF, src1=0xFF, src2=0xFF, funct=0x3
    // Using AluUnary (R-type) with dst=255, src=255, funct=1 (Negate).
    let instr = RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 255, src: 255 };
    let enc = encode(&instr).expect("AluUnary max fields encode must succeed");
    let word = enc.0;

    // Verify each field independently.
    let opcode = (word >> 26) & 0x3F;
    let dst = (word >> 18) & 0xFF;
    let src1 = (word >> 10) & 0xFF;
    let funct = word & 0x3;

    assert_eq!(opcode, 6, "AluUnary opcode should be 6");
    assert_eq!(dst, 255, "dst at max should be 255");
    assert_eq!(src1, 255, "src at max should be 255");
    assert_eq!(funct, 1, "Negate funct should be 1");

    // Verify roundtrip preserves values.
    roundtrip_check(&instr, "R-type max fields");
}

#[test]
fn test_i_type_fields_no_overlap() {
    // I-type: dst=0xFF, src=0xFF, imm10=0x3FF
    // LoadImm: dst=255, width(src)=255, value(imm10)=1023.
    let instr = RspuInstruction::LoadImm { dst: 255, value: 1023, width: 255 };
    let enc = encode(&instr).expect("LoadImm max fields encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let dst = (word >> 18) & 0xFF;
    let src = (word >> 10) & 0xFF;
    let imm10 = word & 0x3FF;

    assert_eq!(opcode, 3, "LoadImm opcode should be 3");
    assert_eq!(dst, 255, "dst at max should be 255");
    assert_eq!(src, 255, "src (width) at max should be 255");
    assert_eq!(imm10, 1023, "imm10 at max should be 1023");

    roundtrip_check(&instr, "I-type max fields");
}

#[test]
fn test_g_type_fields_no_overlap() {
    // G-type: guard=0xFF, src_dst=0xFF, guard2=0xFF, funct=0x3
    // We cannot directly encode all-max G-type through the public API since
    // the funct is always 0 for most G-type instructions. Test with SrQuery
    // which uses guard and src_dst.
    let instr = RspuInstruction::ReflexIf { guard: 255, dst: 255, src: 255 };
    let enc = encode(&instr).expect("ReflexIf max fields encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let guard = (word >> 18) & 0xFF;
    let src_dst = (word >> 10) & 0xFF;
    let guard2 = (word >> 2) & 0xFF;
    let funct = word & 0x3;

    assert_eq!(opcode, 15, "ReflexIf opcode should be 15");
    assert_eq!(guard, 255, "guard at max should be 255");
    assert_eq!(src_dst, 255, "src_dst at max should be 255");
    assert_eq!(guard2, 255, "guard2 (src) at max should be 255");
    assert_eq!(funct, 0, "ReflexIf funct should be 0");

    roundtrip_check(&instr, "G-type max fields");
}

#[test]
fn test_s_type_imm26_max() {
    // S-type with max 26-bit immediate.
    let instr = RspuInstruction::DeadlineSet { cycles: 0x03FF_FFFF };
    let enc = encode(&instr).expect("DeadlineSet max imm26 encode must succeed");
    let word = enc.0;

    let opcode = (word >> 26) & 0x3F;
    let imm26 = word & 0x03FF_FFFF;

    assert_eq!(opcode, 29, "DeadlineSet opcode should be 29");
    assert_eq!(imm26, 0x03FF_FFFF, "imm26 at max should be 0x03FF_FFFF, got {:#010X}", imm26);
    roundtrip_check(&instr, "S-type max imm26");
}

// ===========================================================================
// Section 13: Assert encoding — cond and property_id packing
// ===========================================================================

#[test]
fn test_assert_always_field_packing() {
    // AssertAlways packs: imm26 = (cond << 18) | (property_id & 0x3_FFFF).
    let instr = RspuInstruction::AssertAlways { cond: 0xAB, property_id: 0x1_2345 };
    let enc = encode(&instr).expect("AssertAlways encode must succeed");
    let imm26 = enc.0 & 0x03FF_FFFF;

    let expected_imm = (0xAB_u32 << 18) | 0x1_2345;
    assert_eq!(
        imm26, expected_imm,
        "AssertAlways imm26 packing: expected {:#010X}, got {:#010X}",
        expected_imm, imm26
    );
    roundtrip_check(&instr, "AssertAlways field packing");
}

#[test]
fn test_assert_never_field_packing() {
    let instr = RspuInstruction::AssertNever { cond: 0xFF, property_id: 0x3_FFFF };
    let enc = encode(&instr).expect("AssertNever encode must succeed");
    let imm26 = enc.0 & 0x03FF_FFFF;

    let expected_imm = (0xFF_u32 << 18) | 0x3_FFFF;
    assert_eq!(
        imm26, expected_imm,
        "AssertNever imm26 packing: expected {:#010X}, got {:#010X}",
        expected_imm, imm26
    );
    roundtrip_check(&instr, "AssertNever max fields");
}

// ===========================================================================
// Section 14: TrapIf encoding — cond and code packing
// ===========================================================================

#[test]
fn test_trap_if_field_packing() {
    // TrapIf packs: imm26 = (cond << 8) | code.
    let instr = RspuInstruction::TrapIf { cond: 0xAB, code: 0xCD };
    let enc = encode(&instr).expect("TrapIf encode must succeed");
    let imm26 = enc.0 & 0x03FF_FFFF;

    let expected_imm = (0xAB_u32 << 8) | 0xCD;
    assert_eq!(
        imm26, expected_imm,
        "TrapIf imm26 packing: expected {:#010X}, got {:#010X}",
        expected_imm, imm26
    );
    roundtrip_check(&instr, "TrapIf field packing");
}

// ===========================================================================
// Section 15: Decode stability — random-looking bit patterns
// ===========================================================================

#[test]
fn test_decode_all_zero_word() {
    // Word 0x0000_0000 is opcode 0 (LoadInput) with all fields zero.
    let decoded = decode(0x0000_0000).expect("all-zero word should decode as LoadInput");
    assert_eq!(
        decoded,
        RspuInstruction::LoadInput { dst: 0, port: 0 },
        "all-zero word should decode to LoadInput dst=0 port=0"
    );
}

#[test]
fn test_decode_known_bit_patterns() {
    // Manually construct known words and verify their decoding.
    // NOP: opcode=27 (0x1B), zero payload -> word = 0x1B << 26 = 0x6C00_0000.
    let nop_word: u32 = 27 << 26;
    let decoded = decode(nop_word).expect("NOP word should decode");
    assert_eq!(decoded, RspuInstruction::Nop, "manually constructed NOP word should decode to Nop");

    // FENCE: opcode=28 (0x1C), zero payload -> word = 0x1C << 26 = 0x7000_0000.
    let fence_word: u32 = 28 << 26;
    let decoded = decode(fence_word).expect("FENCE word should decode");
    assert_eq!(
        decoded,
        RspuInstruction::Fence,
        "manually constructed FENCE word should decode to Fence"
    );

    // HALT: opcode=22 (0x16), zero payload -> word = 0x16 << 26 = 0x5800_0000.
    let halt_word: u32 = 22 << 26;
    let decoded = decode(halt_word).expect("HALT word should decode");
    assert_eq!(
        decoded,
        RspuInstruction::Halt,
        "manually constructed HALT word should decode to Halt"
    );
}

#[test]
fn test_encode_decode_deterministic() {
    // Encoding the same instruction twice must produce the same word.
    let instr = RspuInstruction::Alu { op: AluOp::Xor, dst: 200, a: 10, b: 20 };
    let enc1 = encode(&instr).expect("first encode should succeed");
    let enc2 = encode(&instr).expect("second encode should succeed");
    assert_eq!(enc1.0, enc2.0, "encoding the same instruction twice must produce identical words");
}

// ===========================================================================
// Section 16: Mixed-format program roundtrip
// ===========================================================================

#[test]
fn test_emit_binary_all_formats_program() {
    // A program that includes at least one instruction from each format
    // (R-type, I-type, G-type, S-type) and all ISA v2 extensions.
    let instrs = vec![
        // I-type
        RspuInstruction::LoadInput { dst: 0, port: 100 },
        RspuInstruction::LoadInput { dst: 1, port: 200 },
        RspuInstruction::LoadImm { dst: 2, value: 42, width: 8 },
        // R-type
        RspuInstruction::Mov { dst: 3, src: 0 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 193, src: 192 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::GuardOr { dst: 3, a: 0, b: 2 },
        RspuInstruction::TagRead { dst: 194, src: 0 },
        // I-type
        RspuInstruction::AluImm { op: AluOp::Add, dst: 195, a: 0, imm: 10 },
        RspuInstruction::SrInit { guard: 0, length: 5, cond: 10 },
        RspuInstruction::CtrInit { guard: 1, target: 100, cond: 5 },
        RspuInstruction::Prev { dst: 196, signal: 0, delay: 2 },
        RspuInstruction::StoreOutput { src: 192, port: 50 },
        RspuInstruction::TagLoad { dst: 197, tag: 3 },
        RspuInstruction::TagCheck { src: 197, expected: 3 },
        // G-type
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 198, guard: 0 },
        RspuInstruction::CtrTick { guard: 1 },
        RspuInstruction::CtrQuery { dst: 199, guard: 1 },
        RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 192 },
        // S-type
        RspuInstruction::AssertAlways { cond: 10, property_id: 1 },
        RspuInstruction::AssertNever { cond: 11, property_id: 2 },
        RspuInstruction::Trap { code: 5 },
        RspuInstruction::TrapIf { cond: 10, code: 3 },
        RspuInstruction::ModeSwitch { mode: 1 },
        RspuInstruction::DeadlineSet { cycles: 5000 },
        RspuInstruction::Nop,
        RspuInstruction::Fence,
        RspuInstruction::Halt,
        RspuInstruction::EmergencyStop,
    ];
    let program = make_program(instrs.clone());
    let words = emit_binary(&program).expect("emit_binary all-formats should succeed");
    assert_eq!(
        words.len(),
        instrs.len(),
        "word count should match instruction count ({})",
        instrs.len()
    );
    for i in 0..words.len() {
        let decoded = decode(words[i]).unwrap_or_else(|e| {
            panic!("decode of word {} ({}) failed: {}", i, instrs[i].mnemonic(), e.message());
        });
        assert_eq!(
            decoded,
            instrs[i],
            "instruction {} ({}) mismatch after emit_binary roundtrip",
            i,
            instrs[i].mnemonic()
        );
    }
}

// ===========================================================================
// Section 17: Edge cases and corner values
// ===========================================================================

#[test]
fn test_load_imm_zero_value_zero_width() {
    let instr = RspuInstruction::LoadImm { dst: 0, value: 0, width: 0 };
    roundtrip_check(&instr, "LoadImm value=0, width=0");
}

#[test]
fn test_load_imm_max_width_clamped() {
    // Width > 255 is clamped to 255 during encoding (u8 field).
    // After roundtrip, width will be 255 regardless of input.
    let instr_in = RspuInstruction::LoadImm { dst: 0, value: 0, width: 1000 };
    let enc = encode(&instr_in).expect("LoadImm width>255 should encode (clamped)");
    let decoded = decode(enc.0).expect("LoadImm decode should succeed");
    if let RspuInstruction::LoadImm { width, .. } = decoded {
        assert_eq!(
            width, 255,
            "LoadImm width > 255 should be clamped to 255 after roundtrip, got {}",
            width
        );
    } else {
        panic!("decoded instruction should be LoadImm");
    }
}

#[test]
fn test_mov_same_register() {
    // MOV R5, R5 (no-op move) should still roundtrip correctly.
    let instr = RspuInstruction::Mov { dst: 5, src: 5 };
    roundtrip_check(&instr, "MOV same register (R5, R5)");
}

#[test]
fn test_guard_and_same_operands() {
    let instr = RspuInstruction::GuardAnd { dst: 0, a: 0, b: 0 };
    roundtrip_check(&instr, "GUARD_AND all same guard");
}

#[test]
fn test_guard_or_same_operands() {
    let instr = RspuInstruction::GuardOr { dst: 0, a: 0, b: 0 };
    roundtrip_check(&instr, "GUARD_OR all same guard");
}

#[test]
fn test_alu_all_zero_operands() {
    let instr = RspuInstruction::Alu { op: AluOp::Add, dst: 0, a: 0, b: 0 };
    roundtrip_check(&instr, "ALU Add all-zero");
}

#[test]
fn test_alu_imm_zero_immediate() {
    let instr = RspuInstruction::AluImm { op: AluOp::Add, dst: 0, a: 0, imm: 0 };
    roundtrip_check(&instr, "ALU_IMM Add imm=0");
}

#[test]
fn test_trap_code_zero() {
    let instr = RspuInstruction::Trap { code: 0 };
    roundtrip_check(&instr, "Trap code=0");
}

#[test]
fn test_trap_code_max() {
    let instr = RspuInstruction::Trap { code: 255 };
    roundtrip_check(&instr, "Trap code=255");
}

#[test]
fn test_mode_switch_zero() {
    let instr = RspuInstruction::ModeSwitch { mode: 0 };
    roundtrip_check(&instr, "ModeSwitch mode=0");
}

#[test]
fn test_mode_switch_max() {
    let instr = RspuInstruction::ModeSwitch { mode: 255 };
    roundtrip_check(&instr, "ModeSwitch mode=255");
}

#[test]
fn test_assert_always_zero_property() {
    let instr = RspuInstruction::AssertAlways { cond: 0, property_id: 0 };
    roundtrip_check(&instr, "AssertAlways cond=0, prop=0");
}

#[test]
fn test_assert_never_max_property() {
    let instr = RspuInstruction::AssertNever { cond: 255, property_id: 0x3_FFFF };
    roundtrip_check(&instr, "AssertNever max cond and property");
}

#[test]
fn test_deadline_set_zero() {
    let instr = RspuInstruction::DeadlineSet { cycles: 0 };
    roundtrip_check(&instr, "DeadlineSet cycles=0");
}

#[test]
fn test_prev_zero_delay() {
    let instr = RspuInstruction::Prev { dst: 192, signal: 0, delay: 0 };
    roundtrip_check(&instr, "Prev delay=0");
}

#[test]
fn test_prev_max_delay() {
    let instr = RspuInstruction::Prev { dst: 192, signal: 5, delay: 1023 };
    roundtrip_check(&instr, "Prev max delay=1023");
}

// ===========================================================================
// Section 18: 10-bit immediate boundary sweep
// ===========================================================================

#[test]
fn test_imm10_boundary_sweep_load_input() {
    // Test the exact boundary: 1023 succeeds, 1024 fails.
    let ok_instr = RspuInstruction::LoadInput { dst: 0, port: 1023 };
    assert!(encode(&ok_instr).is_ok(), "LoadInput port=1023 (10-bit max) should succeed");
    roundtrip_check(&ok_instr, "LoadInput port=1023 boundary");

    let fail_instr = RspuInstruction::LoadInput { dst: 0, port: 1024 };
    assert!(encode(&fail_instr).is_err(), "LoadInput port=1024 should fail");
}

#[test]
fn test_imm10_boundary_sweep_sr_init() {
    let ok_instr = RspuInstruction::SrInit { guard: 0, length: 1023, cond: 0 };
    assert!(encode(&ok_instr).is_ok(), "SrInit length=1023 should succeed");
    roundtrip_check(&ok_instr, "SrInit length=1023 boundary");

    let fail_instr = RspuInstruction::SrInit { guard: 0, length: 1024, cond: 0 };
    assert!(encode(&fail_instr).is_err(), "SrInit length=1024 should fail");
}

#[test]
fn test_imm10_boundary_sweep_ctr_init() {
    let ok_instr = RspuInstruction::CtrInit { guard: 0, target: 1023, cond: 0 };
    assert!(encode(&ok_instr).is_ok(), "CtrInit target=1023 should succeed");
    roundtrip_check(&ok_instr, "CtrInit target=1023 boundary");

    let fail_instr = RspuInstruction::CtrInit { guard: 0, target: 1024, cond: 0 };
    assert!(encode(&fail_instr).is_err(), "CtrInit target=1024 should fail");
}

// ===========================================================================
// Section 19: Word-level encoding uniqueness
// ===========================================================================

#[test]
fn test_distinct_opcodes_produce_distinct_words() {
    // Two instructions with different opcodes but otherwise identical fields
    // must produce different 32-bit words.
    let nop_word = encode(&RspuInstruction::Nop).expect("NOP encode").0;
    let fence_word = encode(&RspuInstruction::Fence).expect("FENCE encode").0;
    let halt_word = encode(&RspuInstruction::Halt).expect("HALT encode").0;
    let estop_word = encode(&RspuInstruction::EmergencyStop).expect("ESTOP encode").0;

    assert_ne!(nop_word, fence_word, "NOP and FENCE should produce different words");
    assert_ne!(nop_word, halt_word, "NOP and HALT should produce different words");
    assert_ne!(nop_word, estop_word, "NOP and EMERGENCY_STOP should produce different words");
    assert_ne!(fence_word, halt_word, "FENCE and HALT should produce different words");
    assert_ne!(fence_word, estop_word, "FENCE and EMERGENCY_STOP should produce different words");
    assert_ne!(halt_word, estop_word, "HALT and EMERGENCY_STOP should produce different words");
}

#[test]
fn test_different_register_values_produce_different_words() {
    let enc1 = encode(&RspuInstruction::Mov { dst: 0, src: 0 }).expect("MOV 0,0 encode");
    let enc2 = encode(&RspuInstruction::Mov { dst: 1, src: 0 }).expect("MOV 1,0 encode");
    let enc3 = encode(&RspuInstruction::Mov { dst: 0, src: 1 }).expect("MOV 0,1 encode");

    assert_ne!(enc1.0, enc2.0, "MOV dst=0 and MOV dst=1 should differ");
    assert_ne!(enc1.0, enc3.0, "MOV src=0 and MOV src=1 should differ");
    assert_ne!(enc2.0, enc3.0, "MOV(1,0) and MOV(0,1) should differ");
}

#[test]
fn test_different_alu_ops_produce_different_words() {
    let enc_add = encode(&RspuInstruction::Alu { op: AluOp::Add, dst: 192, a: 0, b: 1 })
        .expect("ALU Add encode");
    let enc_sub = encode(&RspuInstruction::Alu { op: AluOp::Sub, dst: 192, a: 0, b: 1 })
        .expect("ALU Sub encode");
    let enc_mul = encode(&RspuInstruction::Alu { op: AluOp::Mul, dst: 192, a: 0, b: 1 })
        .expect("ALU Mul encode");

    assert_ne!(
        enc_add.0, enc_sub.0,
        "ALU Add and Sub with same operands should produce different words"
    );
    assert_ne!(
        enc_add.0, enc_mul.0,
        "ALU Add and Mul with same operands should produce different words"
    );
    assert_ne!(
        enc_sub.0, enc_mul.0,
        "ALU Sub and Mul with same operands should produce different words"
    );
}
