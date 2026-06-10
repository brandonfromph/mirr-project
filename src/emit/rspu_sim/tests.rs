//! Unit tests for the R-SPU simulator.

#![forbid(unsafe_code)]

use super::*;
use crate::emit::rspu_exceptions::{ExceptionCode, ExecMode};
use crate::emit::rspu_isa::{AluOp, RspuInstruction, RspuProgram, MAX_GUARDS};
use crate::emit::rspu_tagged::TypeTag;

/// Helper to create a minimal program from a list of instructions.
fn make_program(instructions: Vec<RspuInstruction>) -> RspuProgram {
    RspuProgram {
        target: None,
        instructions,
        registers_used: 0,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
        certificate: None,
    }
}

#[test]
fn test_simulator_new() {
    let sim = RspuSimulator::new();
    assert_eq!(sim.pc, 0);
    assert_eq!(sim.cycle, 0);
    assert!(!sim.halted);
    assert!(sim.deadline.is_none());
    assert_eq!(sim.guards.len(), MAX_GUARDS);
    // All guards must be false.
    for i in 0..MAX_GUARDS {
        assert!(!sim.read_guard_bool(i as u8));
    }
    assert!(sim.properties.violations.is_empty());
    assert_eq!(sim.exceptions.mode, ExecMode::Reflex);
}

#[test]
fn test_set_input_read_output() {
    let mut sim = RspuSimulator::new();

    // Set input on port 0 (register R0).
    sim.set_input(0, 42, TypeTag::Unsigned { width: 8 });

    // Build a program that loads input port 0 into R192 (temp),
    // then stores it to output port 0 (R64).
    let program = make_program(vec![
        RspuInstruction::LoadInput { dst: 192, port: 0 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
        RspuInstruction::Halt,
    ]);

    let result = sim.run(&program, 100).unwrap();
    assert!(result.halted);
    assert_eq!(result.cycles, 1);

    // Read output port 0.
    let output = sim.read_output(0).unwrap();
    assert_eq!(output.value, 42);
    assert_eq!(output.tag, TypeTag::Unsigned { width: 8 });
}

#[test]
fn test_alu_add() {
    let mut sim = RspuSimulator::new();

    // Load two immediates and add them.
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 25, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);

    let result = sim.run(&program, 100).unwrap();
    assert!(result.halted);

    let word = sim.registers.read(194);
    assert_eq!(word.value, 35);
}

#[test]
fn test_halt_stops() {
    let mut sim = RspuSimulator::new();

    let program = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::Halt,
        RspuInstruction::Nop, // should not be reached
    ]);

    let result = sim.run(&program, 100).unwrap();
    assert!(result.halted);
    // Entire program execution = 1 cycle.
    assert_eq!(result.cycles, 1);
    // PC should stay at the Halt instruction (index 1).
    assert_eq!(sim.pc, 1);
}

#[test]
fn test_emergency_stop() {
    let mut sim = RspuSimulator::new();

    let program = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::EmergencyStop,
        RspuInstruction::Nop, // should not be reached
    ]);

    let result = sim.run(&program, 100).unwrap();
    assert!(result.halted);
    assert_eq!(result.cycles, 1);
}

#[test]
fn test_assert_always_violation() {
    let mut sim = RspuSimulator::new();

    // Load 0 into R192 (represents a false condition), then assert always.
    // MEGA-4: AssertAlways now raises PropertyFail exception on violation.
    let program = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 7 },
        RspuInstruction::Halt,
    ]);

    let result = sim.run(&program, 100).unwrap();
    assert_eq!(result.exception, Some(ExceptionCode::PropertyFail));
    assert_eq!(result.property_violations, vec![7]);
}
