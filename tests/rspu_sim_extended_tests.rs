//! Extended integration tests for R-SPU cycle-accurate simulator.
//!
//! Covers all instruction execution paths (register, ALU, temporal, guard,
//! reflex, safety, LTL assertion, exception, control, tagged, deadline),
//! register file operations, program counter behavior, halt/emergency stop
//! semantics, property checking during simulation, SimResult output format,
//! and edge cases (empty program, max cycles, wrapping overflow).
//!
//! NASA Power-of-10 compliance:
//! - `#![forbid(unsafe_code)]`
//! - All loops use explicit `MAX_*` bounded iteration constants.
//! - No recursion in any test helper.
//! - Every `assert!` has a descriptive message string.

#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

use nasa_rust_project::emit::rspu_exceptions::{ExceptionCode, ExecMode};
use nasa_rust_project::emit::rspu_isa::{
    AluOp, AluUnaryOp, RspuInstruction, RspuProgram, MAX_GUARDS, MAX_SIM_CYCLES, REG_OUTPUT_BASE,
};
use nasa_rust_project::emit::rspu_sim::{RspuSimulator, StepResult};
use nasa_rust_project::emit::rspu_tagged::TypeTag;

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

/// Maximum ALU ops to iterate in parametric tests.
const MAX_ALU_OPS: usize = 14;

/// Maximum guards to check in guard-init tests.
const MAX_GUARD_TEST: usize = 8;

/// Maximum property violations to inject in saturation tests.
const MAX_PROP_TEST: usize = 32;

/// Maximum registers to scan in output collection tests.
const MAX_OUTPUT_SCAN: usize = 16;

/// Maximum instructions in stress test programs.
const MAX_STRESS_INSTRS: usize = 128;

// ---------------------------------------------------------------------------
// Helper: construct a minimal RspuProgram from instructions
// ---------------------------------------------------------------------------

/// Build an `RspuProgram` from a vec of instructions with zeroed metadata.
/// No recursion. Single bounded pass.
fn make_program(instructions: Vec<RspuInstruction>) -> RspuProgram {
    RspuProgram {
        instructions,
        registers_used: 0,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
    }
}

/// Build a simulator with a single input port pre-loaded.
/// No recursion.
fn sim_with_input(port: u16, value: u64, tag: TypeTag) -> RspuSimulator {
    let mut sim = RspuSimulator::new();
    sim.set_input(port, value, tag);
    sim
}

// ---------------------------------------------------------------------------
// 1. Simulator initialization
// ---------------------------------------------------------------------------

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

#[test]
fn test_alu_unary_on_uninitialized_errors() {
    let mut sim = RspuSimulator::new();
    // R192 is uninitialized -- unary op should error with E708.
    let program = make_program(vec![
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 193, src: 192 },
        RspuInstruction::Halt,
    ]);
    let err = sim.step(&program).expect_err("Unary on uninitialized must error");
    let msg = err.to_string();
    assert!(msg.contains("E708"), "Error must contain E708 tag violation code, got: {msg}");
}

// ---------------------------------------------------------------------------
// 11. Temporal tier: Shift register
// ---------------------------------------------------------------------------

#[test]
fn test_sr_init_activates_guard_on_nonzero_cond() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 4, cond: 192 },
        RspuInstruction::SrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("SrInit must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        1,
        "Guard 0 must be active after SrInit with nonzero cond"
    );
}

#[test]
fn test_sr_init_inactive_on_zero_cond() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 4, cond: 192 },
        RspuInstruction::SrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("SrInit zero cond must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        0,
        "Guard 0 must be inactive after SrInit with zero cond"
    );
}

#[test]
fn test_sr_tick_is_noop() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 4, cond: 192 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("SrTick must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        1,
        "Guard remains active after SrTick (no-op in single-tick model)"
    );
}

// ---------------------------------------------------------------------------
// 12. Temporal tier: Counter
// ---------------------------------------------------------------------------

#[test]
fn test_ctr_init_activates_guard_on_nonzero_cond() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::CtrInit { guard: 1, target: 10, cond: 192 },
        RspuInstruction::CtrQuery { dst: 193, guard: 1 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("CtrInit must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        1,
        "Guard 1 must be active after CtrInit with nonzero cond"
    );
}

#[test]
fn test_ctr_tick_is_noop() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::CtrInit { guard: 2, target: 5, cond: 192 },
        RspuInstruction::CtrTick { guard: 2 },
        RspuInstruction::CtrQuery { dst: 193, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("CtrTick must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        1,
        "Guard remains active after CtrTick (no-op in single-tick model)"
    );
}

// ---------------------------------------------------------------------------
// 13. Guard combinators
// ---------------------------------------------------------------------------

#[test]
fn test_guard_and_both_true() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrInit { guard: 1, length: 1, cond: 192 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::SrQuery { dst: 193, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("GuardAnd must succeed");
    assert_eq!(sim.registers.read(193).value, 1, "AND(true, true) must be true");
}

#[test]
fn test_guard_and_one_false() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrInit { guard: 1, length: 1, cond: 193 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::SrQuery { dst: 194, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("GuardAnd one false must succeed");
    assert_eq!(sim.registers.read(194).value, 0, "AND(true, false) must be false");
}

#[test]
fn test_guard_or_one_true() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrInit { guard: 1, length: 1, cond: 193 },
        RspuInstruction::GuardOr { dst: 2, a: 0, b: 1 },
        RspuInstruction::SrQuery { dst: 194, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("GuardOr must succeed");
    assert_eq!(sim.registers.read(194).value, 1, "OR(true, false) must be true");
}

#[test]
fn test_guard_or_both_false() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrInit { guard: 1, length: 1, cond: 192 },
        RspuInstruction::GuardOr { dst: 2, a: 0, b: 1 },
        RspuInstruction::SrQuery { dst: 193, guard: 2 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("GuardOr both false must succeed");
    assert_eq!(sim.registers.read(193).value, 0, "OR(false, false) must be false");
}

// ---------------------------------------------------------------------------
// 14. ReflexIf instruction
// ---------------------------------------------------------------------------

#[test]
fn test_reflex_if_guard_active() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::LoadImm { dst: 193, value: 99, width: 8 },
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ReflexIf active must succeed");
    assert_eq!(sim.registers.read(194).value, 99, "ReflexIf must copy when guard is active");
}

#[test]
fn test_reflex_if_guard_inactive() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::LoadImm { dst: 193, value: 99, width: 8 },
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ReflexIf inactive must succeed");
    assert_eq!(
        sim.registers.read(194).tag,
        TypeTag::Uninitialized,
        "ReflexIf must NOT copy when guard is inactive"
    );
}

// ---------------------------------------------------------------------------
// 15. Prev instruction
// ---------------------------------------------------------------------------

#[test]
fn test_prev_copies_signal() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 77, width: 8 },
        RspuInstruction::Prev { dst: 193, signal: 192, delay: 1 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Prev must succeed");
    assert_eq!(
        sim.registers.read(193).value,
        77,
        "Prev must copy signal to dst in single-tick model"
    );
}

// ---------------------------------------------------------------------------
// 16. AssertAlways / AssertNever
// ---------------------------------------------------------------------------

#[test]
fn test_assert_always_no_violation() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 2 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 10 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("AssertAlways no violation must succeed");
    assert!(result.property_violations.is_empty(), "No violations when cond is nonzero");
}

#[test]
fn test_assert_always_violation() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 7 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("AssertAlways violation must succeed");
    assert_eq!(result.property_violations, vec![7], "Property 7 must be violated when cond is 0");
}

#[test]
fn test_assert_never_no_violation() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
        RspuInstruction::AssertNever { cond: 192, property_id: 20 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("AssertNever no violation must succeed");
    assert!(result.property_violations.is_empty(), "No violations when cond is 0 for AssertNever");
}

#[test]
fn test_assert_never_violation() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 2 },
        RspuInstruction::AssertNever { cond: 192, property_id: 33 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("AssertNever violation must succeed");
    assert_eq!(
        result.property_violations,
        vec![33],
        "Property 33 must be violated when cond is nonzero for AssertNever"
    );
}

#[test]
fn test_multiple_property_violations() {
    let mut sim = RspuSimulator::new();
    let mut instrs = Vec::new();
    instrs.push(RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 });
    instrs.push(RspuInstruction::LoadImm { dst: 193, value: 1, width: 2 });
    for i in 0..MAX_PROP_TEST {
        instrs.push(RspuInstruction::AssertAlways { cond: 192, property_id: i as u32 });
    }
    for i in 0..MAX_PROP_TEST {
        instrs.push(RspuInstruction::AssertNever { cond: 193, property_id: (100 + i) as u32 });
    }
    instrs.push(RspuInstruction::Halt);
    let program = make_program(instrs);
    let result = sim.run(&program, 1000).expect("Multiple violations must succeed");
    assert_eq!(
        result.property_violations.len(),
        MAX_PROP_TEST * 2,
        "Must have {expected} violations",
        expected = MAX_PROP_TEST * 2
    );
}

// ---------------------------------------------------------------------------
// 17. Trap and TrapIf
// ---------------------------------------------------------------------------

#[test]
fn test_trap_raises_software_trap() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Trap { code: 1 }, RspuInstruction::Halt]);
    let result = sim.run(&program, 100).expect("Trap must succeed");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::SoftwareTrap),
        "Trap must produce SoftwareTrap exception"
    );
}

#[test]
fn test_trap_if_condition_true() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::TrapIf { cond: 192, code: 2 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("TrapIf cond true must succeed");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::SoftwareTrap),
        "TrapIf with nonzero cond must trap"
    );
}

#[test]
fn test_trap_if_condition_false() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::TrapIf { cond: 192, code: 2 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("TrapIf cond false must succeed");
    assert!(result.halted, "TrapIf with zero cond must continue to Halt");
    assert!(result.exception.is_none(), "TrapIf with zero cond must not produce exception");
}

// ---------------------------------------------------------------------------
// 18. ModeSwitch
// ---------------------------------------------------------------------------

#[test]
fn test_mode_switch_to_host() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::ModeSwitch { mode: 1 }, // Host
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("ModeSwitch to Host must succeed");
    assert_eq!(sim.exceptions.mode, ExecMode::Host, "Mode must be Host after ModeSwitch(1)");
}

#[test]
fn test_mode_switch_same_mode_tolerant() {
    let mut sim = RspuSimulator::new();
    // Reflex -> Reflex should be tolerated (no error).
    let program = make_program(vec![
        RspuInstruction::ModeSwitch { mode: 0 }, // Reflex (same as default)
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Same-mode switch must be tolerated");
    assert!(result.halted, "Program must halt normally after same-mode switch");
}

#[test]
fn test_mode_switch_invalid_mode() {
    let mut sim = RspuSimulator::new();
    let program =
        make_program(vec![RspuInstruction::ModeSwitch { mode: 99 }, RspuInstruction::Halt]);
    let err = sim.run(&program, 100).expect_err("Invalid mode must error");
    let msg = err.to_string();
    assert!(msg.contains("E714"), "Invalid mode error must contain E714, got: {msg}");
}

// ---------------------------------------------------------------------------
// 19. TagLoad, TagCheck, TagRead
// ---------------------------------------------------------------------------

#[test]
fn test_tag_load() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 42, width: 8 },
        RspuInstruction::TagLoad { dst: 192, tag: 1 }, // 1 = Bool
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("TagLoad must succeed");
    assert_eq!(sim.registers.read(192).tag, TypeTag::Bool, "TagLoad(1) must set tag to Bool");
    assert_eq!(sim.registers.read(192).value, 42, "TagLoad must preserve value");
}

#[test]
fn test_tag_check_pass() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::TagCheck { src: 192, expected: 8 }, // 8 = Unsigned{width:8}
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("TagCheck pass must succeed");
    assert!(result.halted, "TagCheck pass must continue to Halt");
    assert!(result.exception.is_none(), "TagCheck pass must not raise exception");
}

#[test]
fn test_tag_check_fail() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::TagCheck { src: 192, expected: 1 }, // 1 = Bool, actual is Unsigned(8)
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("TagCheck fail must succeed (returns exception)");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::TagViolation),
        "TagCheck mismatch must raise TagViolation"
    );
}

#[test]
fn test_tag_read() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 1 }, // Bool
        RspuInstruction::TagRead { dst: 193, src: 192 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("TagRead must succeed");
    // Bool encodes as u8 = 1.
    assert_eq!(sim.registers.read(193).value, 1, "TagRead of Bool must produce 1");
    assert_eq!(
        sim.registers.read(193).tag,
        TypeTag::Unsigned { width: 8 },
        "TagRead result must be Unsigned(8)"
    );
}

// ---------------------------------------------------------------------------
// 20. DeadlineSet
// ---------------------------------------------------------------------------

#[test]
fn test_deadline_set_no_miss() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::DeadlineSet { cycles: 100 },
        RspuInstruction::Nop,
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 200).expect("Deadline no miss must succeed");
    assert!(result.halted, "Program must halt before deadline");
    assert!(result.exception.is_none(), "No exception when deadline not reached");
}

#[test]
fn test_deadline_miss() {
    let mut sim = RspuSimulator::new();
    // Deadline at cycle 2; we execute 3 Nops before Halt so cycle will reach 2.
    let program = make_program(vec![
        RspuInstruction::DeadlineSet { cycles: 2 },
        RspuInstruction::Nop,
        RspuInstruction::Nop, // After this step, cycle=3 >= deadline(2)
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Deadline miss must succeed (returns exception)");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::DeadlineMiss),
        "Must report DeadlineMiss when cycle reaches deadline"
    );
}

// ---------------------------------------------------------------------------
// 21. Fence instruction
// ---------------------------------------------------------------------------

#[test]
fn test_fence_is_noop() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Fence, RspuInstruction::Halt]);
    let result = sim.run(&program, 100).expect("Fence must succeed");
    assert!(result.halted, "Fence must not prevent program from halting");
    assert_eq!(result.cycles, 2, "Fence + Halt = 2 cycles");
}

// ---------------------------------------------------------------------------
// 22. Program counter behavior
// ---------------------------------------------------------------------------

#[test]
fn test_pc_advances_on_continue() {
    let mut sim = RspuSimulator::new();
    let program =
        make_program(vec![RspuInstruction::Nop, RspuInstruction::Nop, RspuInstruction::Halt]);
    let _ = sim.step(&program).expect("Step 0 must succeed");
    assert_eq!(sim.pc, 1, "PC must be 1 after first Nop");
    let _ = sim.step(&program).expect("Step 1 must succeed");
    assert_eq!(sim.pc, 2, "PC must be 2 after second Nop");
}

#[test]
fn test_pc_stays_on_halt() {
    let mut sim = RspuSimulator::new();
    let program =
        make_program(vec![RspuInstruction::Nop, RspuInstruction::Halt, RspuInstruction::Nop]);
    let result = sim.run(&program, 100).expect("Program must succeed");
    assert_eq!(sim.pc, 1, "PC must stay at Halt instruction index");
    assert!(result.halted, "SimResult must show halted");
}

#[test]
fn test_pc_past_end_halts() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Nop]);
    let s1 = sim.step(&program).expect("First step must succeed");
    assert_eq!(s1, StepResult::Continue, "Nop returns Continue");
    assert_eq!(sim.pc, 1, "PC advances past the single Nop");
    let s2 = sim.step(&program).expect("Second step (past end) must succeed");
    assert_eq!(s2, StepResult::Halted, "PC past end must produce Halted");
    assert!(sim.halted, "Simulator must be halted after PC passes end");
}

// ---------------------------------------------------------------------------
// 23. Max cycles exceeded
// ---------------------------------------------------------------------------

#[test]
fn test_max_cycles_exceeded_error() {
    let mut sim = RspuSimulator::new();
    // Infinite Nop loop (no Halt) with tiny max_cycles budget.
    let mut instrs = Vec::new();
    for _i in 0..MAX_STRESS_INSTRS {
        instrs.push(RspuInstruction::Nop);
    }
    // No Halt instruction -- program never terminates.
    // But we also need PC to loop. Actually, with only MAX_STRESS_INSTRS nops
    // and no branch, PC will fall off the end and halt naturally.
    // Instead, set max_cycles to something smaller than instruction count.
    let program = make_program(instrs);
    let err = sim.run(&program, 5).expect_err("Must error when max_cycles exceeded without halt");
    let msg = err.to_string();
    assert!(msg.contains("E712"), "Max cycles error must contain E712, got: {msg}");
}

#[test]
fn test_max_sim_cycles_cap() {
    // Verify the MAX_SIM_CYCLES constant is reasonable.
    assert_eq!(MAX_SIM_CYCLES, 1_000_000, "MAX_SIM_CYCLES must be 1_000_000, got {MAX_SIM_CYCLES}");
}

// ---------------------------------------------------------------------------
// 24. Wrapping overflow
// ---------------------------------------------------------------------------

#[test]
fn test_add_wrapping_overflow() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: u64::MAX, width: 64 },
        RspuInstruction::LoadImm { dst: 193, value: 1, width: 64 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Wrapping add must succeed");
    assert_eq!(sim.registers.read(194).value, 0, "u64::MAX + 1 must wrap to 0");
}

#[test]
fn test_sub_wrapping_underflow() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 1, width: 8 },
        RspuInstruction::Alu { op: AluOp::Sub, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Wrapping sub must succeed");
    assert_eq!(sim.registers.read(194).value, u64::MAX, "0 - 1 must wrap to u64::MAX");
}

#[test]
fn test_mul_wrapping_overflow() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: u64::MAX, width: 64 },
        RspuInstruction::LoadImm { dst: 193, value: 2, width: 64 },
        RspuInstruction::Alu { op: AluOp::Mul, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Wrapping mul must succeed");
    let expected = u64::MAX.wrapping_mul(2);
    assert_eq!(sim.registers.read(194).value, expected, "u64::MAX * 2 must wrap correctly");
}

// ---------------------------------------------------------------------------
// 25. SimResult output collection
// ---------------------------------------------------------------------------

#[test]
fn test_sim_result_collects_outputs() {
    let mut sim = RspuSimulator::new();
    let mut instrs = Vec::new();
    for i in 0..MAX_OUTPUT_SCAN {
        instrs.push(RspuInstruction::LoadImm { dst: 192, value: (i * 10) as u64, width: 8 });
        instrs.push(RspuInstruction::StoreOutput { src: 192, port: i as u16 });
    }
    instrs.push(RspuInstruction::Halt);
    let program = make_program(instrs);
    let result = sim.run(&program, 1000).expect("Output collection must succeed");
    assert_eq!(
        result.outputs.len(),
        MAX_OUTPUT_SCAN,
        "Must collect {MAX_OUTPUT_SCAN} output ports"
    );
    for i in 0..MAX_OUTPUT_SCAN {
        let word =
            result.outputs.get(&(i as u16)).unwrap_or_else(|| panic!("Output port {i} must exist"));
        assert_eq!(
            word.value,
            (i * 10) as u64,
            "Output port {i} must have value {expected}",
            expected = i * 10
        );
    }
}

#[test]
fn test_sim_result_no_outputs_when_none_written() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![RspuInstruction::Nop, RspuInstruction::Halt]);
    let result = sim.run(&program, 100).expect("No-output program must succeed");
    assert!(
        result.outputs.is_empty(),
        "SimResult outputs must be empty when no StoreOutput executed"
    );
}

#[test]
fn test_sim_result_fields() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 42 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("SimResult fields test must succeed");
    assert_eq!(result.cycles, 3, "cycles must be 3 (LoadImm + AssertAlways + Halt)");
    assert!(result.halted, "halted must be true");
    assert!(result.exception.is_none(), "exception must be None");
    assert_eq!(result.property_violations, vec![42], "property_violations must contain [42]");
}

// ---------------------------------------------------------------------------
// 26. Set input and read output partition
// ---------------------------------------------------------------------------

#[test]
fn test_set_input_registers() {
    let mut sim = RspuSimulator::new();
    sim.set_input(5, 0xBEEF, TypeTag::Unsigned { width: 16 });
    let word = sim.registers.read(5); // Port 5 maps to R5
    assert_eq!(word.value, 0xBEEF, "Input register R5 must hold 0xBEEF");
    assert_eq!(word.tag, TypeTag::Unsigned { width: 16 }, "Input tag must be Unsigned(16)");
}

#[test]
fn test_read_output_partition() {
    let mut sim = RspuSimulator::new();
    // Directly write to the output partition register R64 (port 0).
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 555, width: 16 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Output partition test must succeed");
    let out = sim.read_output(0).expect("Output port 0 must be readable");
    assert_eq!(out.value, 555, "Output port 0 must hold 555");
    // Verify the register is indeed at REG_OUTPUT_BASE.
    let direct = sim.registers.read(REG_OUTPUT_BASE);
    assert_eq!(direct.value, 555, "REG_OUTPUT_BASE register must match output port 0");
}

// ---------------------------------------------------------------------------
// 27. Guard bounds checking
// ---------------------------------------------------------------------------

#[test]
fn test_guard_out_of_bounds_reads_false() {
    let mut sim = RspuSimulator::new();
    // Query a guard that is within array but never set -- must be false.
    let program = make_program(vec![
        RspuInstruction::SrQuery { dst: 192, guard: 63 }, // MAX_GUARDS-1
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Guard bounds query must succeed");
    assert_eq!(sim.registers.read(192).value, 0, "Unset guard at max index must read as false");
}

// ---------------------------------------------------------------------------
// 28. Multiple guards initialization
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_guards_independent() {
    let mut sim = RspuSimulator::new();
    let mut instrs = Vec::new();
    instrs.push(RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 });
    instrs.push(RspuInstruction::LoadImm { dst: 193, value: 0, width: 8 });
    for i in 0..MAX_GUARD_TEST {
        let cond = if i % 2 == 0 { 192 } else { 193 };
        instrs.push(RspuInstruction::SrInit { guard: i as u8, length: 1, cond });
    }
    for i in 0..MAX_GUARD_TEST {
        instrs.push(RspuInstruction::SrQuery { dst: (200 + i) as u8, guard: i as u8 });
    }
    instrs.push(RspuInstruction::Halt);
    let program = make_program(instrs);
    let _result = sim.run(&program, 200).expect("Multiple guards must succeed");
    for i in 0..MAX_GUARD_TEST {
        let expected = if i % 2 == 0 { 1u64 } else { 0u64 };
        assert_eq!(
            sim.registers.read((200 + i) as u8).value,
            expected,
            "Guard {i} must be {expected_str}",
            expected_str = if expected == 1 { "active" } else { "inactive" }
        );
    }
}

// ---------------------------------------------------------------------------
// 29. Cycle counter accuracy
// ---------------------------------------------------------------------------

#[test]
fn test_cycle_counter_increments_per_step() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Cycle counter test must succeed");
    assert_eq!(result.cycles, 5, "4 Nops + 1 Halt = 5 cycles");
    assert_eq!(sim.cycle, 5, "Simulator cycle counter must match SimResult");
}

// ---------------------------------------------------------------------------
// 30. Stress test: many instructions
// ---------------------------------------------------------------------------

#[test]
fn test_stress_many_nops() {
    let mut sim = RspuSimulator::new();
    let mut instrs = Vec::new();
    for _i in 0..MAX_STRESS_INSTRS {
        instrs.push(RspuInstruction::Nop);
    }
    instrs.push(RspuInstruction::Halt);
    let program = make_program(instrs);
    let result = sim.run(&program, 1000).expect("Stress test must succeed");
    assert!(result.halted, "Stress test must halt");
    assert_eq!(
        result.cycles,
        (MAX_STRESS_INSTRS + 1) as u64,
        "Stress test must execute {expected} cycles",
        expected = MAX_STRESS_INSTRS + 1
    );
}

// ---------------------------------------------------------------------------
// 31. ALU tag mismatch errors
// ---------------------------------------------------------------------------

#[test]
fn test_alu_tag_mismatch_unsigned_signed() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        // Manually tag R193 as signed.
        RspuInstruction::LoadImm { dst: 193, value: 20, width: 8 },
        RspuInstruction::TagLoad { dst: 193, tag: 136 }, // 128+8 = Signed{width:8}
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let err = sim.run(&program, 100).expect_err("Mismatched tags must error");
    let msg = err.to_string();
    assert!(msg.contains("E708"), "Tag mismatch must produce E708, got: {msg}");
}

// ---------------------------------------------------------------------------
// 32. Comparison result is Bool
// ---------------------------------------------------------------------------

#[test]
fn test_comparison_produces_bool_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 10, width: 8 },
        RspuInstruction::Alu { op: AluOp::Lt, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Comparison must succeed");
    assert_eq!(sim.registers.read(194).value, 1, "5 < 10 must be true (1)");
    assert_eq!(sim.registers.read(194).tag, TypeTag::Bool, "Comparison result tag must be Bool");
}

#[test]
fn test_eq_false() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 10, width: 8 },
        RspuInstruction::Alu { op: AluOp::Eq, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Eq comparison must succeed");
    assert_eq!(sim.registers.read(194).value, 0, "5 == 10 must be false (0)");
}

// ---------------------------------------------------------------------------
// 33. Exception terminates run
// ---------------------------------------------------------------------------

#[test]
fn test_exception_terminates_run() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::Trap { code: 1 },
        RspuInstruction::Nop,  // unreachable
        RspuInstruction::Halt, // unreachable
    ]);
    let result = sim.run(&program, 100).expect("Exception must terminate run");
    assert!(!result.halted, "Halted flag must be false when exception terminates");
    assert_eq!(
        result.exception,
        Some(ExceptionCode::SoftwareTrap),
        "Exception must be SoftwareTrap"
    );
    assert_eq!(result.cycles, 2, "Nop + Trap = 2 cycles before exception");
}

// ---------------------------------------------------------------------------
// 34. Register file default state
// ---------------------------------------------------------------------------

#[test]
fn test_uninitialized_register_is_zero_valued() {
    let sim = RspuSimulator::new();
    let word = sim.registers.read(192);
    assert_eq!(word.value, 0, "Uninitialized register value must be 0");
    assert_eq!(
        word.tag,
        TypeTag::Uninitialized,
        "Uninitialized register tag must be Uninitialized"
    );
}

// ---------------------------------------------------------------------------
// 35. LoadImm large width clamped
// ---------------------------------------------------------------------------

#[test]
fn test_load_imm_large_width_clamped() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 200 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Large width LoadImm must succeed");
    // Width > 127 is clamped to 127 by width_to_type_tag.
    assert_eq!(
        sim.registers.read(192).tag,
        TypeTag::Unsigned { width: 127 },
        "Width > 127 must clamp to Unsigned(127)"
    );
}

// ---------------------------------------------------------------------------
// 36. AluImm preserves tag from operand
// ---------------------------------------------------------------------------

#[test]
fn test_alu_imm_preserves_operand_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 100, width: 16 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 193, a: 192, imm: 50 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("AluImm tag preservation must succeed");
    assert_eq!(
        sim.registers.read(193).tag,
        TypeTag::Unsigned { width: 16 },
        "AluImm result tag must match operand's Unsigned(16)"
    );
    assert_eq!(sim.registers.read(193).value, 150, "100 + imm(50) = 150");
}

// ---------------------------------------------------------------------------
// 37. SrQuery produces Bool tag
// ---------------------------------------------------------------------------

#[test]
fn test_sr_query_produces_bool_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("SrQuery Bool tag must succeed");
    assert_eq!(sim.registers.read(193).tag, TypeTag::Bool, "SrQuery result must have Bool tag");
}

// ---------------------------------------------------------------------------
// 38. CtrQuery produces Bool tag
// ---------------------------------------------------------------------------

#[test]
fn test_ctr_query_produces_bool_tag() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::CtrInit { guard: 0, target: 5, cond: 192 },
        RspuInstruction::CtrQuery { dst: 193, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("CtrQuery Bool tag must succeed");
    assert_eq!(sim.registers.read(193).tag, TypeTag::Bool, "CtrQuery result must have Bool tag");
}

// ---------------------------------------------------------------------------
// 39. Full datapath: input -> ALU -> output
// ---------------------------------------------------------------------------

#[test]
fn test_full_datapath_input_alu_output() {
    let mut sim = RspuSimulator::new();
    sim.set_input(0, 100, TypeTag::Unsigned { width: 16 });
    sim.set_input(1, 50, TypeTag::Unsigned { width: 16 });
    let program = make_program(vec![
        RspuInstruction::LoadInput { dst: 192, port: 0 },
        RspuInstruction::LoadInput { dst: 193, port: 1 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::StoreOutput { src: 194, port: 0 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Full datapath must succeed");
    assert!(result.halted, "Datapath program must halt");
    let out = result.outputs.get(&0).expect("Output port 0 must exist");
    assert_eq!(out.value, 150, "100 + 50 = 150");
}

// ---------------------------------------------------------------------------
// 40. Conditional datapath with guard
// ---------------------------------------------------------------------------

#[test]
fn test_conditional_datapath_with_guard() {
    let mut sim = RspuSimulator::new();
    sim.set_input(0, 1, TypeTag::Unsigned { width: 8 }); // condition = true
    let program = make_program(vec![
        // Load condition and init guard.
        RspuInstruction::LoadInput { dst: 192, port: 0 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        // Prepare a value.
        RspuInstruction::LoadImm { dst: 193, value: 42, width: 8 },
        // Conditional move: only if guard active.
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        // Output the result.
        RspuInstruction::StoreOutput { src: 194, port: 0 },
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Conditional datapath must succeed");
    assert!(result.halted, "Conditional datapath must halt");
    let out = result.outputs.get(&0).expect("Output port 0 must exist");
    assert_eq!(out.value, 42, "Output must be 42 when guard is active");
}

// ---------------------------------------------------------------------------
// 41. Deadline cleared after miss
// ---------------------------------------------------------------------------

#[test]
fn test_deadline_cleared_after_miss() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::DeadlineSet { cycles: 1 },
        RspuInstruction::Nop, // cycle becomes 2 after this, >= deadline(1)
        RspuInstruction::Halt,
    ]);
    let result = sim.run(&program, 100).expect("Deadline cleared test must succeed");
    assert_eq!(result.exception, Some(ExceptionCode::DeadlineMiss), "Must report DeadlineMiss");
    // After the deadline fires, the deadline field is cleared.
    assert!(sim.deadline.is_none(), "Deadline must be cleared after miss");
}

// ---------------------------------------------------------------------------
// 42. Signed arithmetic via TagLoad
// ---------------------------------------------------------------------------

#[test]
fn test_signed_arithmetic() {
    let mut sim = RspuSimulator::new();
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::TagLoad { dst: 192, tag: 136 }, // Signed{width:8}
        RspuInstruction::LoadImm { dst: 193, value: 3, width: 8 },
        RspuInstruction::TagLoad { dst: 193, tag: 136 }, // Signed{width:8}
        RspuInstruction::Alu { op: AluOp::Sub, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let _result = sim.run(&program, 100).expect("Signed arithmetic must succeed");
    assert_eq!(sim.registers.read(194).value, 7, "10 - 3 = 7");
    assert_eq!(
        sim.registers.read(194).tag,
        TypeTag::Signed { width: 8 },
        "Result tag must be Signed(8)"
    );
}
