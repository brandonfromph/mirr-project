//! R-SPU fixed-point simulator.
//!
//! Cycle-accurate executor for R-SPU programs, used for validation.

#![forbid(unsafe_code)]

mod helpers;
mod sim_types;

pub use sim_types::*;

use std::collections::HashMap;

use crate::emit::rspu::rspu_err;
use crate::emit::rspu_exceptions::{ExceptionCode, ExceptionState, ExecMode};
use crate::emit::rspu_isa::*;
use crate::emit::rspu_tagged::{check_alu_tags, RegisterFile, TaggedWord, TypeTag};
use crate::error::MirrError;

use helpers::*;

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
    /// Whether the last VERIFY instruction succeeded (MEGA-4 totality).
    pub cert_verified: bool,
    /// Shadow interval register file for MEGA-5 symbolic reasoning.
    /// Each register has (lo, hi) bounds — default = (0, u64::MAX).
    pub interval_shadow: Vec<(u64, u64)>,
    /// Current active type tag register.
    pub tag_register: RegId,
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
        // MEGA-5: Initialize interval shadow with full range for every register.
        // Bounded: exactly MAX_REGISTERS iterations.
        let mut interval_shadow = Vec::with_capacity(MAX_REGISTERS);
        for _i in 0..MAX_REGISTERS {
            interval_shadow.push((0, u64::MAX));
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
            cert_verified: false,
            interval_shadow,
            tag_register: 0,
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
                    let _action = self.exceptions.raise_exception(ExceptionCode::PropertyFail)?;
                    return Ok(StepResult::Exception(ExceptionCode::PropertyFail));
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::AssertNever { cond, property_id } => {
                let val = self.registers.read(*cond).value;
                if val != 0 {
                    self.properties.record_violation(*property_id);
                    let _action = self.exceptions.raise_exception(ExceptionCode::PropertyFail)?;
                    return Ok(StepResult::Exception(ExceptionCode::PropertyFail));
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

            RspuInstruction::TagBranch { tag_value, target_pc } => {
                let current_tag = self.registers.read_tag(self.tag_register);
                let current_tag_val = type_tag_to_u8(&current_tag);
                if current_tag_val == *tag_value {
                    self.pc = *target_pc as usize;
                } else {
                    self.pc = self.pc.wrapping_add(1);
                }
                Ok(StepResult::Continue)
            }

            // -- ISA v2: Temporal extension -----------------------------
            RspuInstruction::DeadlineSet { cycles } => {
                // Set the absolute deadline.
                self.deadline = Some(*cycles);
                Ok(StepResult::Continue)
            }

            // -- MEGA-4: Totality Engine tier ------------------------------
            RspuInstruction::Verify { cert_offset: _ } => {
                // In simulation, VERIFY validates that a certificate is
                // present.  Hardware performs SHA-256 hash comparison;
                // the simulator trusts the host environment.
                self.cert_verified = true;
                Ok(StepResult::Continue)
            }

            RspuInstruction::Certify { dst } => {
                // Write 1 if the last VERIFY succeeded, 0 otherwise.
                let val = if self.cert_verified { 1u64 } else { 0u64 };
                self.registers
                    .write(*dst, TaggedWord::from_computed(val, TypeTag::Unsigned { width: 1 }));
                Ok(StepResult::Continue)
            }

            RspuInstruction::TotalCheck { expected_properties } => {
                // Count verified properties (those NOT in the violations list).
                // If fewer than expected passed, raise PropertyFail.
                let verified = (*expected_properties as usize)
                    .saturating_sub(self.properties.violations.len());
                if verified < *expected_properties as usize {
                    let _action = self.exceptions.raise_exception(ExceptionCode::PropertyFail)?;
                    return Ok(StepResult::Exception(ExceptionCode::PropertyFail));
                }
                Ok(StepResult::Continue)
            }

            // -- MEGA-5: Symbolic Reasoning tier ---------------------------------
            RspuInstruction::Match { dst, src, table_offset: _ } => {
                // In simulation, MATCH performs a simplified stub: write the
                // source register value as the match result (pattern ID 0).
                // Full hardware match-unit logic requires a pattern table not
                // modeled in the sequential simulator.
                let val = self.registers.read(*src).value;
                let result_id = if val == 0 { 0u64 } else { 1u64 };
                self.registers.write(
                    *dst,
                    TaggedWord::from_computed(result_id, TypeTag::Unsigned { width: 32 }),
                );
                Ok(StepResult::Continue)
            }

            RspuInstruction::IntervalLo { dst, src } => {
                let idx = *src as usize;
                let lo =
                    if idx < self.interval_shadow.len() { self.interval_shadow[idx].0 } else { 0 };
                self.registers
                    .write(*dst, TaggedWord::from_computed(lo, TypeTag::Unsigned { width: 64 }));
                Ok(StepResult::Continue)
            }

            RspuInstruction::IntervalHi { dst, src } => {
                let idx = *src as usize;
                let hi = if idx < self.interval_shadow.len() {
                    self.interval_shadow[idx].1
                } else {
                    u64::MAX
                };
                self.registers
                    .write(*dst, TaggedWord::from_computed(hi, TypeTag::Unsigned { width: 64 }));
                Ok(StepResult::Continue)
            }

            RspuInstruction::IntervalCheck { src, bounds } => {
                let val = self.registers.read(*src).value;
                let bounds_idx = *bounds as usize;
                let (lo, hi) = if bounds_idx < self.interval_shadow.len() {
                    self.interval_shadow[bounds_idx]
                } else {
                    (0, u64::MAX)
                };
                if val < lo || val > hi {
                    let _action =
                        self.exceptions.raise_exception(ExceptionCode::IntervalViolation)?;
                    return Ok(StepResult::Exception(ExceptionCode::IntervalViolation));
                }
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
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
