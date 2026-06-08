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

use mirrc::emit::rspu_exceptions::{ExceptionCode, ExecMode};
use mirrc::emit::rspu_isa::{
    AluOp, AluUnaryOp, RspuInstruction, RspuProgram, MAX_GUARDS, MAX_SIM_CYCLES, REG_OUTPUT_BASE,
};
use mirrc::emit::rspu_sim::{RspuSimulator, StepResult};
use mirrc::emit::rspu_tagged::TypeTag;

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

/// Maximum ALU ops to iterate in parametric tests.
const MAX_ALU_OPS: usize = 14;

/// Maximum guards to check in guard-init tests.
const MAX_GUARD_TEST: usize = 8;

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
        certificate: None,
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

mod part1;
mod part2;
mod part3;
