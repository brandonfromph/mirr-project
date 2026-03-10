#![forbid(unsafe_code)]

//! Cycle-accurate ISA simulator for R-SPU programs.
//!
//! Executes an [`RspuProgram`] instruction by instruction, tracking register
//! state, guard state, property assertions, exceptions, and deadlines.
//!
//! The primary entry point is [`RspuSimulator`], which provides:
//! - [`RspuSimulator::step`]: single-instruction execution
//! - [`RspuSimulator::run`]: full-program execution to completion
//!
//! All loops are bounded by `MAX_*` constants (NASA Power-of-10).
//! No recursion.  No unsafe code.

use std::collections::HashMap;

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_exceptions::{ExceptionCode, ExceptionState, ExecMode};
use crate::emit::rspu_isa::{
    AluOp, AluUnaryOp, GuardId, PortId, PropertyId, RegId, RspuInstruction, RspuProgram,
    MAX_GUARDS, MAX_SIM_CYCLES, REG_OUTPUT_BASE, REG_OUTPUT_MAX,
};
use crate::emit::rspu_tagged::{check_alu_tags, RegisterFile, TaggedWord, TypeTag};
use crate::error::MirrError;

// ---------------------------------------------------------------------------
// Constants (NASA P10 bounded-resource model)
// ---------------------------------------------------------------------------

/// Maximum number of property violations tracked before saturation.
const MAX_PROPERTY_VIOLATIONS: usize = 1024;

// ---------------------------------------------------------------------------
// StepResult
// ---------------------------------------------------------------------------

/// Result of executing a single instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepResult {
    /// Execution should continue to the next instruction.
    Continue,
    /// The processor has been halted (graceful stop).
    Halted,
    /// Emergency stop (immediate abort, safety-critical).
    EmergencyStop,
    /// An exception was raised with the given code.
    Exception(ExceptionCode),
}

// ---------------------------------------------------------------------------
// SimResult
// ---------------------------------------------------------------------------

/// Result of a complete simulation run.
#[derive(Debug, Clone)]
pub struct SimResult {
    /// Total cycles executed.
    pub cycles: u64,
    /// Output port values (scanned from the output register partition).
    pub outputs: HashMap<PortId, TaggedWord>,
    /// Property IDs that were violated during execution.
    pub property_violations: Vec<PropertyId>,
    /// Exception that terminated execution, if any.
    pub exception: Option<ExceptionCode>,
    /// Whether the simulator halted normally.
    pub halted: bool,
}

// ---------------------------------------------------------------------------
// PropertyState
// ---------------------------------------------------------------------------

/// Tracks property assertion violations during simulation.
#[derive(Debug, Clone)]
pub struct PropertyState {
    /// List of property IDs that have been violated.
    pub violations: Vec<PropertyId>,
}

impl PropertyState {
    /// Create a new property state with no violations.
    pub fn new() -> Self {
        Self { violations: Vec::new() }
    }

    /// Record a property violation, respecting the saturation bound.
    fn record_violation(&mut self, id: PropertyId) {
        if self.violations.len() < MAX_PROPERTY_VIOLATIONS {
            self.violations.push(id);
        }
    }
}

impl Default for PropertyState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// RspuSimulator
// ---------------------------------------------------------------------------

/// Cycle-accurate simulator for R-SPU programs.
///
/// Maintains the full architectural state: register file, guard array,
/// program counter, cycle counter, exception state, property tracking,
/// and optional deadline.
pub struct RspuSimulator {
    /// Tagged register file (256 entries).
    pub registers: RegisterFile,
    /// Guard state array (`MAX_GUARDS` entries, initialized to false).
    pub guards: Vec<bool>,
    /// Program counter (index into instruction vector).
    pub pc: usize,
    /// Current cycle count.
    pub cycle: u64,
    /// Exception handling state machine.
    pub exceptions: ExceptionState,
    /// Property assertion tracking.
    pub properties: PropertyState,
    /// Optional hard real-time deadline (absolute cycle count).
    /// When set, the simulator raises `DeadlineMiss` if the cycle counter
    /// reaches this value.
    pub deadline: Option<u32>,
    /// Whether the simulator has been halted.
    pub halted: bool,
}

impl RspuSimulator {
    /// Create a new simulator with all state initialized to defaults.
    ///
    /// - All registers are uninitialized.
    /// - All guards are false.
    /// - PC is 0, cycle is 0, no deadline.
    pub fn new() -> Self {
        let mut guards = Vec::with_capacity(MAX_GUARDS);
        // Bounded: exactly MAX_GUARDS iterations.
        for _i in 0..MAX_GUARDS {
            guards.push(false);
        }
        Self {
            registers: RegisterFile::new(),
            guards,
            pc: 0,
            cycle: 0,
            exceptions: ExceptionState::new(),
            properties: PropertyState::new(),
            deadline: None,
            halted: false,
        }
    }

    /// Write an input value to the register file at the input partition.
    ///
    /// `port` selects the register in the R0-R63 input range.  The value
    /// is tagged with the given `TypeTag`.
    pub fn set_input(&mut self, port: PortId, value: u64, tag: TypeTag) {
        let reg = port as RegId;
        let word = TaggedWord::from_input(value, tag, port);
        self.registers.write(reg, word);
    }

    /// Read an output value from the register file output partition (R64-R127).
    ///
    /// Returns `None` if the port index would exceed the output range.
    pub fn read_output(&self, port: PortId) -> Option<&TaggedWord> {
        let reg = REG_OUTPUT_BASE.wrapping_add(port as RegId);
        if !(REG_OUTPUT_BASE..=REG_OUTPUT_MAX).contains(&reg) {
            return None;
        }
        Some(self.registers.read(reg))
    }

    // -----------------------------------------------------------------------
    // Single-step execution
    // -----------------------------------------------------------------------

    /// Execute one instruction and advance the simulator state.
    ///
    /// Returns `StepResult::Halted` if the simulator is already halted or
    /// the PC has run past the end of the program.
    ///
    /// Bounded: each call executes exactly one instruction.
    pub fn step(&mut self, program: &RspuProgram) -> Result<StepResult, MirrError> {
        // Already halted?
        if self.halted {
            return Ok(StepResult::Halted);
        }

        // PC past end of program?
        if self.pc >= program.instructions.len() {
            self.halted = true;
            return Ok(StepResult::Halted);
        }

        // Fetch instruction.
        let instr = program.instructions[self.pc].clone();

        // Execute.
        let result = self.execute_instruction(&instr)?;

        // If the instruction did not halt or raise an exception, advance PC.
        match result {
            StepResult::Continue => {
                self.pc += 1;
            }
            StepResult::Halted | StepResult::EmergencyStop | StepResult::Exception(_) => {
                // PC stays at the halting/faulting instruction.
            }
        }

        // Advance cycle counter.
        self.cycle += 1;

        // Check deadline expiry (after incrementing cycle).
        if let Some(deadline_cycles) = self.deadline {
            if self.cycle >= deadline_cycles as u64 {
                self.deadline = None;
                return Ok(StepResult::Exception(ExceptionCode::DeadlineMiss));
            }
        }

        Ok(result)
    }

    /// Execute a single instruction, updating simulator state.
    ///
    /// This is the core dispatch function.  It matches on all 30 instruction
    /// variants and performs the appropriate register/guard/state updates.
    fn execute_instruction(&mut self, instr: &RspuInstruction) -> Result<StepResult, MirrError> {
        match instr {
            // -- Register tier ------------------------------------------
            RspuInstruction::LoadInput { dst, port } => {
                // Copy the value from the input register (port maps to R0-R63).
                let src_reg = *port as RegId;
                let word = self.registers.read(src_reg).clone();
                self.registers.write(*dst, word);
                Ok(StepResult::Continue)
            }

            RspuInstruction::StoreOutput { src, port } => {
                // Copy from src register into the output partition.
                let word = self.registers.read(*src).clone();
                let out_reg = REG_OUTPUT_BASE.wrapping_add(*port as RegId);
                self.registers.write(out_reg, word);
                Ok(StepResult::Continue)
            }

            RspuInstruction::Mov { dst, src } => {
                let word = self.registers.read(*src).clone();
                self.registers.write(*dst, word);
                Ok(StepResult::Continue)
            }

            RspuInstruction::LoadImm { dst, value, width } => {
                let tag = width_to_type_tag(*width);
                let word = TaggedWord::from_literal(*value, tag);
                self.registers.write(*dst, word);
                Ok(StepResult::Continue)
            }

            // -- ALU tier -----------------------------------------------
            RspuInstruction::Alu { op, dst, a, b } => {
                let word_a = self.registers.read(*a).clone();
                let word_b = self.registers.read(*b).clone();
                let result_tag = check_alu_tags(&word_a, &word_b, *op)?;
                let result_val = execute_alu(*op, word_a.value, word_b.value);
                self.registers.write(*dst, TaggedWord::from_computed(result_val, result_tag));
                Ok(StepResult::Continue)
            }

            RspuInstruction::AluImm { op, dst, a, imm } => {
                let word_a = self.registers.read(*a).clone();
                // Create a literal tagged word for the immediate, using
                // the same tag as operand a so tag checking succeeds.
                let word_b = TaggedWord::from_literal(*imm, word_a.tag);
                let result_tag = check_alu_tags(&word_a, &word_b, *op)?;
                let result_val = execute_alu(*op, word_a.value, *imm);
                self.registers.write(*dst, TaggedWord::from_computed(result_val, result_tag));
                Ok(StepResult::Continue)
            }

            RspuInstruction::AluUnary { op, dst, src } => {
                let word = self.registers.read(*src).clone();
                if word.tag == TypeTag::Uninitialized {
                    return Err(rspu_err("[E708] tag violation: unary operand is uninitialized"));
                }
                let result_val = execute_alu_unary(*op, word.value);
                self.registers.write(*dst, TaggedWord::from_computed(result_val, word.tag));
                Ok(StepResult::Continue)
            }

            // -- Temporal tier (shift register) -------------------------
            RspuInstruction::SrInit { guard, length: _, cond } => {
                // Simplified simulation: set the guard to true if the
                // condition register is nonzero.
                let cond_val = self.registers.read(*cond).value;
                self.set_guard(*guard, cond_val != 0);
                Ok(StepResult::Continue)
            }

            RspuInstruction::SrTick { guard: _ } => {
                // In the single-tick simulation model, SR tick is a no-op.
                Ok(StepResult::Continue)
            }

            RspuInstruction::SrQuery { dst, guard } => {
                let active = self.read_guard(*guard);
                let val = u64::from(active);
                self.registers.write(*dst, TaggedWord::from_computed(val, TypeTag::Bool));
                Ok(StepResult::Continue)
            }

            // -- Temporal tier (counter) --------------------------------
            RspuInstruction::CtrInit { guard, target: _, cond } => {
                // Simplified: set guard active if condition is nonzero.
                let cond_val = self.registers.read(*cond).value;
                self.set_guard(*guard, cond_val != 0);
                Ok(StepResult::Continue)
            }

            RspuInstruction::CtrTick { guard: _ } => {
                // Counter tick is a no-op in the single-tick model.
                Ok(StepResult::Continue)
            }

            RspuInstruction::CtrQuery { dst, guard } => {
                let active = self.read_guard(*guard);
                let val = u64::from(active);
                self.registers.write(*dst, TaggedWord::from_computed(val, TypeTag::Bool));
                Ok(StepResult::Continue)
            }

            // -- Guard combinators --------------------------------------
            RspuInstruction::GuardAnd { dst, a, b } => {
                let result = self.read_guard(*a) && self.read_guard(*b);
                self.set_guard(*dst, result);
                Ok(StepResult::Continue)
            }

            RspuInstruction::GuardOr { dst, a, b } => {
                let result = self.read_guard(*a) || self.read_guard(*b);
                self.set_guard(*dst, result);
                Ok(StepResult::Continue)
            }

            // -- Reflex tier --------------------------------------------
            RspuInstruction::ReflexIf { guard, dst, src } => {
                if self.read_guard(*guard) {
                    let word = self.registers.read(*src).clone();
                    self.registers.write(*dst, word);
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::Prev { dst, signal, delay: _ } => {
                // Simplified single-tick model: copy signal to dst.
                // Full delay tracking requires multi-cycle state not modeled here.
                let word = self.registers.read(*signal).clone();
                self.registers.write(*dst, word);
                Ok(StepResult::Continue)
            }

            // -- Safety tier --------------------------------------------
            RspuInstruction::EmergencyStop => {
                self.halted = true;
                Ok(StepResult::EmergencyStop)
            }

            // -- LTL Assertion tier -------------------------------------
            RspuInstruction::AssertAlways { cond, property_id } => {
                let val = self.registers.read(*cond).value;
                if val == 0 {
                    self.properties.record_violation(*property_id);
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::AssertNever { cond, property_id } => {
                let val = self.registers.read(*cond).value;
                if val != 0 {
                    self.properties.record_violation(*property_id);
                }
                Ok(StepResult::Continue)
            }

            // -- ISA v2: Exception tier ---------------------------------
            RspuInstruction::Trap { code: _code } => {
                let _action = self.exceptions.raise_exception(ExceptionCode::SoftwareTrap)?;
                Ok(StepResult::Exception(ExceptionCode::SoftwareTrap))
            }

            RspuInstruction::TrapIf { cond, code: _code } => {
                let val = self.registers.read(*cond).value;
                if val != 0 {
                    let _action = self.exceptions.raise_exception(ExceptionCode::SoftwareTrap)?;
                    return Ok(StepResult::Exception(ExceptionCode::SoftwareTrap));
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::Halt => {
                self.halted = true;
                self.exceptions.halt();
                Ok(StepResult::Halted)
            }

            // -- ISA v2: Control tier -----------------------------------
            RspuInstruction::ModeSwitch { mode } => {
                let new_mode = match mode {
                    0 => ExecMode::Reflex,
                    1 => ExecMode::Host,
                    other => {
                        return Err(rspu_err(format!("[E714] invalid mode value: {other}",)));
                    }
                };
                // Tolerate same-mode switch in simulation by silently
                // succeeding rather than faulting.
                if self.exceptions.mode != new_mode {
                    self.exceptions.switch_mode(new_mode)?;
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::Nop => Ok(StepResult::Continue),

            RspuInstruction::Fence => {
                // Ordering guarantee -- no-op in sequential simulation.
                Ok(StepResult::Continue)
            }

            // -- ISA v2: Tagged tier ------------------------------------
            RspuInstruction::TagLoad { dst, tag } => {
                // Set the type tag on the dst register from the raw u8.
                let new_tag = u8_to_type_tag(*tag);
                let current = self.registers.read(*dst).clone();
                self.registers.write(*dst, TaggedWord::from_computed(current.value, new_tag));
                Ok(StepResult::Continue)
            }

            RspuInstruction::TagCheck { src, expected } => {
                let actual_tag = self.registers.read_tag(*src);
                let expected_tag = u8_to_type_tag(*expected);
                if actual_tag != expected_tag {
                    let _action = self.exceptions.raise_exception(ExceptionCode::TagViolation)?;
                    return Ok(StepResult::Exception(ExceptionCode::TagViolation));
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::TagRead { dst, src } => {
                let src_tag = self.registers.read_tag(*src);
                let tag_val = type_tag_to_u8(&src_tag) as u64;
                self.registers.write(
                    *dst,
                    TaggedWord::from_computed(tag_val, TypeTag::Unsigned { width: 8 }),
                );
                Ok(StepResult::Continue)
            }

            // -- ISA v2: Temporal extension -----------------------------
            RspuInstruction::DeadlineSet { cycles } => {
                // Set the absolute deadline.
                self.deadline = Some(*cycles);
                Ok(StepResult::Continue)
            }
        }
    }

    // -----------------------------------------------------------------------
    // Guard helpers (bounded by MAX_GUARDS)
    // -----------------------------------------------------------------------

    /// Read a guard value, returning false if the index is out of bounds.
    fn read_guard(&self, id: GuardId) -> bool {
        let idx = id as usize;
        if idx < self.guards.len() {
            self.guards[idx]
        } else {
            false
        }
    }

    /// Set a guard value.  No-op if the index is out of bounds.
    fn set_guard(&mut self, id: GuardId, value: bool) {
        let idx = id as usize;
        if idx < self.guards.len() {
            self.guards[idx] = value;
        }
    }

    // -----------------------------------------------------------------------
    // Full program execution
    // -----------------------------------------------------------------------

    /// Run the program to completion or until a termination condition.
    ///
    /// Execution stops when:
    /// - The processor halts (`Halt` or `EmergencyStop` instruction).
    /// - An exception is raised.
    /// - `max_cycles` is reached (returns `Err` with `[E712]`).
    ///
    /// Bounded: at most `effective_max` iterations (capped by `MAX_SIM_CYCLES`).
    pub fn run(&mut self, program: &RspuProgram, max_cycles: u64) -> Result<SimResult, MirrError> {
        let effective_max = max_cycles.min(MAX_SIM_CYCLES);
        let mut terminating_exception: Option<ExceptionCode> = None;

        // Bounded loop: at most effective_max iterations.
        let mut steps: u64 = 0;
        while steps < effective_max {
            let result = self.step(program)?;
            steps += 1;

            match result {
                StepResult::Continue => {}
                StepResult::Halted => break,
                StepResult::EmergencyStop => break,
                StepResult::Exception(code) => {
                    terminating_exception = Some(code);
                    break;
                }
            }
        }

        // If we exhausted the cycle budget without terminating, report error.
        if steps >= effective_max && !self.halted && terminating_exception.is_none() {
            return Err(rspu_err(format!(
                "[E712] simulation exceeded {effective_max} cycles without halting",
            )));
        }

        // Collect outputs by scanning the output register partition (R64-R127).
        let mut outputs = HashMap::new();
        // Bounded: exactly (REG_OUTPUT_MAX - REG_OUTPUT_BASE + 1) = 64 iterations.
        let mut reg = REG_OUTPUT_BASE;
        loop {
            let word = self.registers.read(reg).clone();
            if word.tag != TypeTag::Uninitialized {
                let port = (reg - REG_OUTPUT_BASE) as PortId;
                outputs.insert(port, word);
            }
            if reg == REG_OUTPUT_MAX {
                break;
            }
            reg += 1;
        }

        Ok(SimResult {
            cycles: self.cycle,
            outputs,
            property_violations: self.properties.violations.clone(),
            exception: terminating_exception,
            halted: self.halted,
        })
    }
}

impl Default for RspuSimulator {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ALU execution helpers
// ---------------------------------------------------------------------------

/// Execute a binary ALU operation on raw 64-bit values.
///
/// All arithmetic uses wrapping semantics to avoid overflow panics.
fn execute_alu(op: AluOp, a: u64, b: u64) -> u64 {
    match op {
        AluOp::Add => a.wrapping_add(b),
        AluOp::Sub => a.wrapping_sub(b),
        AluOp::Mul => a.wrapping_mul(b),
        AluOp::And => a & b,
        AluOp::Or => a | b,
        AluOp::Xor => a ^ b,
        AluOp::Shl => a.wrapping_shl(b as u32),
        AluOp::Shr => a.wrapping_shr(b as u32),
        AluOp::Eq => u64::from(a == b),
        AluOp::Ne => u64::from(a != b),
        AluOp::Lt => u64::from(a < b),
        AluOp::Le => u64::from(a <= b),
        AluOp::Gt => u64::from(a > b),
        AluOp::Ge => u64::from(a >= b),
    }
}

/// Execute a unary ALU operation on a raw 64-bit value.
fn execute_alu_unary(op: AluUnaryOp, a: u64) -> u64 {
    match op {
        AluUnaryOp::Not => !a,
        AluUnaryOp::Negate => (a as i64).wrapping_neg() as u64,
    }
}

// ---------------------------------------------------------------------------
// Type tag conversion helpers
// ---------------------------------------------------------------------------

/// Convert a u8 encoding to a `TypeTag`.
///
/// Encoding scheme:
/// - 0 => Uninitialized
/// - 1 => Bool
/// - 2..=127 => Unsigned { width: n }
/// - 128..=255 => Signed { width: n - 128 }
fn u8_to_type_tag(tag: u8) -> TypeTag {
    match tag {
        0 => TypeTag::Uninitialized,
        1 => TypeTag::Bool,
        n if n >= 128 => TypeTag::Signed { width: n.wrapping_sub(128) },
        n => TypeTag::Unsigned { width: n },
    }
}

/// Convert a `TypeTag` to its u8 encoding.
fn type_tag_to_u8(tag: &TypeTag) -> u8 {
    match tag {
        TypeTag::Uninitialized => 0,
        TypeTag::Bool => 1,
        TypeTag::Unsigned { width } => *width,
        TypeTag::Signed { width } => width.wrapping_add(128),
    }
}

/// Convert a width in bits to a `TypeTag`.
///
/// Widths of 0 or 1 map to `Bool`; all others map to `Unsigned`.
fn width_to_type_tag(width: u32) -> TypeTag {
    if width <= 1 {
        TypeTag::Bool
    } else if width <= 127 {
        TypeTag::Unsigned { width: width as u8 }
    } else {
        TypeTag::Unsigned { width: 127 }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::rspu_isa::{AluOp, RspuInstruction, RspuProgram, MAX_GUARDS};
    use crate::emit::rspu_tagged::TypeTag;

    /// Helper to create a minimal program from a list of instructions.
    fn make_program(instructions: Vec<RspuInstruction>) -> RspuProgram {
        RspuProgram {
            instructions,
            registers_used: 0,
            guards_used: 0,
            register_map: Vec::new(),
            guard_map: Vec::new(),
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
            assert!(!sim.guards[i]);
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
        assert_eq!(result.cycles, 3);

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
        // Nop + Halt = 2 instructions executed = 2 cycles.
        assert_eq!(result.cycles, 2);
        // PC should be at the Halt instruction (index 1), not advanced past it.
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
        assert_eq!(result.cycles, 2);
    }

    #[test]
    fn test_assert_always_violation() {
        let mut sim = RspuSimulator::new();

        // Load 0 into R192 (represents a false condition), then assert always.
        let program = make_program(vec![
            RspuInstruction::LoadImm { dst: 192, value: 0, width: 2 },
            RspuInstruction::AssertAlways { cond: 192, property_id: 7 },
            RspuInstruction::Halt,
        ]);

        let result = sim.run(&program, 100).unwrap();
        assert!(result.halted);
        assert_eq!(result.property_violations, vec![7]);
    }
}
