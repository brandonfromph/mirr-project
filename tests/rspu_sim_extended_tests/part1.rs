use super::*;

#[test]
fn test_sim_initial_state() {
    let sim = RspuSimulator::new();
    assert_eq!(sim.pc, 0, "PC must start at 0");
    assert_eq!(sim.cycle, 0, "Cycle counter must start at 0");
    assert!(!sim.halted, "Simulator must not start halted");
    assert!(sim.deadline.is_none(), "No deadline set initially");
    assert_eq!(sim.guards.len(), MAX_GUARDS, "Guard array must have MAX_GUARDS entries");
    assert!(sim.properties.violations.is_empty(), "No property violations initially");
    assert_eq!(sim.exceptions.mode, ExecMode::Reflex, "Default mode must be Reflex");
}

#[test]
fn test_sim_all_guards_initially_false() {
    let sim = RspuSimulator::new();
    for i in 0..MAX_GUARDS {
        assert!(!sim.guards[i], "Guard {i} must be false initially");
    }
}

#[test]
fn test_sim_default_trait() {
    let sim = RspuSimulator::default();
    assert_eq!(sim.pc, 0, "Default simulator PC must be 0");
    assert!(!sim.halted, "Default simulator must not be halted");
}

// ---------------------------------------------------------------------------
// 2. Empty program
// ---------------------------------------------------------------------------

#[test]
fn test_empty_program_halts_immediately() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![]);
    let step = sim.step(&program).expect("step on empty program should not error");
    assert_eq!(step, StepResult::Halted, "Empty program must halt on first step");
    assert!(sim.halted, "Simulator must be halted after empty program step");
}

#[test]
fn test_empty_program_run() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![]);
    let result = sim.run(&program, 100).expect("run on empty program must succeed");
    assert!(result.halted, "SimResult must show halted for empty program");
    assert_eq!(
        result.cycles, 0,
        "Empty program run should execute 0 cycles (halts before any instruction)"
    );
}

// ---------------------------------------------------------------------------
// 3. Nop instruction
// ---------------------------------------------------------------------------

#[test]
fn test_nop_advances_pc() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Nop, RspuInstruction::Halt]);
    let step = sim.step(&program).expect("Nop step must succeed");
    assert_eq!(step, StepResult::Continue, "Nop must return Continue");
    assert_eq!(sim.pc, 1, "PC must advance past Nop");
    assert_eq!(sim.cycle, 1, "Cycle must increment after Nop");
}

#[test]
fn test_nop_sequence() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Nop sequence must succeed");
    assert!(result.halted, "Program must halt after Nop sequence");
    assert_eq!(result.cycles, 4, "3 Nops + 1 Halt = 4 cycles");
    assert_eq!(sim.pc, 3, "PC must be at the Halt instruction (index 3)");
}

// ---------------------------------------------------------------------------
// 4. Halt and EmergencyStop
// ---------------------------------------------------------------------------

#[test]
fn test_halt_stops_execution() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::Halt,
        RspuInstruction::Nop, // unreachable
    ]);
    let result = sim.run(&program, 100).expect("Halt must succeed");
    assert!(result.halted, "SimResult must show halted");
    assert_eq!(result.cycles, 1, "Only Halt instruction executed");
    assert_eq!(sim.pc, 0, "PC stays at Halt instruction");
}

#[test]
fn test_halt_sets_exception_state() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Halt]);
    let _result = sim.run(&program, 100).expect("Halt must succeed");
    assert!(sim.exceptions.halted, "Exception state must be halted after Halt");
}

#[test]
fn test_emergency_stop() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::EmergencyStop,
        RspuInstruction::Nop, // unreachable
    ]);
    let result = sim.run(&program, 100).expect("EmergencyStop must succeed");
    assert!(result.halted, "SimResult must show halted after EmergencyStop");
    assert_eq!(result.cycles, 2, "Nop + EmergencyStop = 2 cycles");
    assert_eq!(sim.pc, 1, "PC stays at EmergencyStop instruction");
}

#[test]
fn test_step_after_halt_returns_halted() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Halt]);
    let _ = sim.step(&program).expect("First step must succeed");
    let step2 = sim.step(&program).expect("Second step after halt must succeed");
    assert_eq!(step2, StepResult::Halted, "Step after halt must return Halted");
}

// ---------------------------------------------------------------------------
// 5. LoadImm instruction
// ---------------------------------------------------------------------------

#[test]
fn test_load_imm_unsigned() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0xDEAD, width: 16 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("LoadImm must succeed");
    let word = sim.registers.read(192);
    assert_eq!(word.value, 0xDEAD, "Register must hold loaded value");
    assert_eq!(word.tag, TypeTag::Unsigned { width: 16 }, "Tag must be Unsigned(16) for width=16");
}

#[test]
fn test_load_imm_bool_width() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("LoadImm bool must succeed");
    let word = sim.registers.read(192);
    assert_eq!(word.value, 1, "Bool register must hold 1");
    assert_eq!(word.tag, TypeTag::Bool, "Width 1 must produce Bool tag");
}

#[test]
fn test_load_imm_zero_width_is_bool() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("LoadImm width=0 must succeed");
    let word = sim.registers.read(192);
    assert_eq!(word.tag, TypeTag::Bool, "Width 0 maps to Bool");
}

// ---------------------------------------------------------------------------
// 6. Mov instruction
// ---------------------------------------------------------------------------

#[test]
fn test_mov_copies_value_and_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 99, width: 8 },
        RspuInstruction::Mov { dst: 193, src: 192 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Mov must succeed");
    let src = sim.registers.read(192);
    let dst = sim.registers.read(193);
    assert_eq!(dst.value, src.value, "Mov must copy value");
    assert_eq!(dst.tag, src.tag, "Mov must copy tag");
}

// ---------------------------------------------------------------------------
// 7. LoadInput / StoreOutput
// ---------------------------------------------------------------------------

#[test]
fn test_load_input_store_output_roundtrip() {
    let mut sim = sim_with_input(0, 42, TypeTag::Unsigned { width: 8 });
    let program = make_program(vec![
        RspuInstruction::LoadInput { dst: 192, port: 0 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("I/O roundtrip must succeed");
    assert!(result.halted, "Program must halt");
    let output = sim.read_output(0).expect("Output port 0 must exist");
    assert_eq!(output.value, 42, "Output value must match input");
    assert_eq!(output.tag, TypeTag::Unsigned { width: 8 }, "Output tag must match input");
}

#[test]
fn test_multiple_io_ports() {
    let mut sim = RspuSimulator::new();
    sim.set_input(0, 10, TypeTag::Unsigned { width: 8 });
    sim.set_input(1, 20, TypeTag::Unsigned { width: 8 });
    let program = make_program(vec![
        RspuInstruction::LoadInput { dst: 192, port: 0 },
        RspuInstruction::LoadInput { dst: 193, port: 1 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
        RspuInstruction::StoreOutput { src: 193, port: 1 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Multi-port I/O must succeed");
    assert!(result.halted, "Program must halt");
    assert_eq!(result.outputs.len(), 2, "Two output ports must be collected");
    let out0 = result.outputs.get(&0).expect("Port 0 must exist in outputs");
    let out1 = result.outputs.get(&1).expect("Port 1 must exist in outputs");
    assert_eq!(out0.value, 10, "Port 0 value must be 10");
    assert_eq!(out1.value, 20, "Port 1 value must be 20");
}

#[test]
fn test_read_output_out_of_range() {
    let sim = RspuSimulator::new();
    // Port 200 would compute REG_OUTPUT_BASE + 200 which overflows u8 (64+200=264 > 255).
    // With wrapping_add, 64 + 200 = 264 wraps to 8 (on u8), which IS in range.
    // Let's test a port that is clearly outside the output partition.
    // REG_OUTPUT_BASE=64, REG_OUTPUT_MAX=127, so 64 ports (0..63) are valid.
    // Port 64 would be 64+64=128, which is > 127, so out of range.
    let result = sim.read_output(64);
    assert!(result.is_none(), "read_output(64) must return None (out of output range)");
}

// ---------------------------------------------------------------------------
// 8. ALU binary operations
// ---------------------------------------------------------------------------

#[test]
fn test_alu_add() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 25, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ALU Add must succeed");
    assert_eq!(sim.registers.read(194).value, 35, "10 + 25 = 35");
}

#[test]
fn test_alu_sub() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 30, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 12, width: 8 },
        RspuInstruction::Alu { op: AluOp::Sub, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ALU Sub must succeed");
    assert_eq!(sim.registers.read(194).value, 18, "30 - 12 = 18");
}

#[test]
fn test_alu_mul() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 7, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 6, width: 8 },
        RspuInstruction::Alu { op: AluOp::Mul, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ALU Mul must succeed");
    assert_eq!(sim.registers.read(194).value, 42, "7 * 6 = 42");
}

#[test]
fn test_alu_bitwise_and() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0xFF, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0x0F, width: 8 },
        RspuInstruction::Alu { op: AluOp::And, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ALU And must succeed");
    assert_eq!(sim.registers.read(194).value, 0x0F, "0xFF & 0x0F = 0x0F");
}

#[test]
fn test_alu_bitwise_or() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0xF0, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0x0F, width: 8 },
        RspuInstruction::Alu { op: AluOp::Or, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ALU Or must succeed");
    assert_eq!(sim.registers.read(194).value, 0xFF, "0xF0 | 0x0F = 0xFF");
}

#[test]
fn test_alu_bitwise_xor() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0xAA, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0x55, width: 8 },
        RspuInstruction::Alu { op: AluOp::Xor, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ALU Xor must succeed");
    assert_eq!(sim.registers.read(194).value, 0xFF, "0xAA ^ 0x55 = 0xFF");
}

#[test]
fn test_alu_shl() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 4, width: 8 },
        RspuInstruction::Alu { op: AluOp::Shl, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ALU Shl must succeed");
    assert_eq!(sim.registers.read(194).value, 16, "1 << 4 = 16");
}

#[test]
fn test_alu_shr() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 128, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 3, width: 8 },
        RspuInstruction::Alu { op: AluOp::Shr, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ALU Shr must succeed");
    assert_eq!(sim.registers.read(194).value, 16, "128 >> 3 = 16");
}

#[test]
fn test_alu_comparisons() {
    let ops_and_expected: [(AluOp, u64, u64, u64); MAX_ALU_OPS] = [
        (AluOp::Add, 3, 4, 7),
        (AluOp::Sub, 10, 3, 7),
        (AluOp::Mul, 5, 3, 15),
        (AluOp::And, 0xFF, 0x0F, 0x0F),
        (AluOp::Or, 0xF0, 0x0F, 0xFF),
        (AluOp::Xor, 0xFF, 0xFF, 0),
        (AluOp::Shl, 2, 3, 16),
        (AluOp::Shr, 64, 2, 16),
        (AluOp::Eq, 5, 5, 1),
        (AluOp::Ne, 5, 6, 1),
        (AluOp::Lt, 3, 5, 1),
        (AluOp::Le, 5, 5, 1),
        (AluOp::Gt, 7, 3, 1),
        (AluOp::Ge, 5, 5, 1),
    ];
    for i in 0..MAX_ALU_OPS {
        let (op, a, b, expected) = ops_and_expected[i];
        let mut sim = RspuSimulator::new();
        let program = make_program(vec![
            RspuInstruction::LoadImm { dst: 192, value: a, width: 8 },
            RspuInstruction::LoadImm { dst: 193, value: b, width: 8 },
            RspuInstruction::Alu { op, dst: 194, a: 192, b: 193 },
            RspuInstruction::Halt,
        ]);
        let _result = sim.run(&program, 100).expect("ALU op must succeed");
        assert_eq!(
            sim.registers.read(194).value,
            expected,
            "ALU op {:?} with ({a}, {b}) must produce {expected}",
            op
        );
    }
}

// ---------------------------------------------------------------------------
// 9. ALU immediate operation
// ---------------------------------------------------------------------------

#[test]
fn test_alu_imm_add() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 193, a: 192, imm: 5 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("AluImm must succeed");
    assert_eq!(sim.registers.read(193).value, 15, "10 + imm(5) = 15");
}

#[test]
fn test_alu_imm_sub() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 50, width: 8 },
        RspuInstruction::AluImm { op: AluOp::Sub, dst: 193, a: 192, imm: 30 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("AluImm Sub must succeed");
    assert_eq!(sim.registers.read(193).value, 20, "50 - imm(30) = 20");
}

// ---------------------------------------------------------------------------
// 10. ALU unary operations
// ---------------------------------------------------------------------------

#[test]
fn test_alu_unary_not() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 193, src: 192 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("AluUnary Not must succeed");
    assert_eq!(sim.registers.read(193).value, !0u64, "NOT(0) must produce all-ones (u64::MAX)");
}

#[test]
fn test_alu_unary_negate() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 42, width: 8 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 193, src: 192 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("AluUnary Negate must succeed");
    let expected = (42i64).wrapping_neg() as u64;
    assert_eq!(
        sim.registers.read(193).value,
        expected,
        "Negate(42) must produce two's complement negation"
    );
}

