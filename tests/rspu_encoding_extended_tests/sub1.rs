use super::*;

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