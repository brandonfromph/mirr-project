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
// Constants (NASA P10 bounded-resource model)
// ---------------------------------------------------------------------------

/// Maximum number of cycles of register history to track for `Prev`.
pub const MAX_REG_HISTORY: usize = 64;

/// Cycle-accurate simulator for R-SPU programs.
///
/// Maintains the full architectural state: register file, guard array,
/// program counter, cycle counter, exception state, property tracking,
/// and optional deadline.
pub struct RspuSimulator {
    /// Tagged register file (256 entries).
    pub registers: RegisterFile,
    /// Double-buffered guard state array (`MAX_GUARDS` entries).
    pub guards: Vec<DoubleBufferedGuard>,
    /// Program counter (index into instruction vector).
    pub pc: usize,
    /// Current cycle count.
    pub cycle: u64,
    /// Exception handling state machine.
    pub exceptions: ExceptionState,
    /// Property assertion tracking.
    pub properties: PropertyState,
    /// Optional hard real-time deadline (absolute cycle count).
    pub deadline: Option<u32>,
    /// Whether the simulator has been halted.
    pub halted: bool,
    /// Whether the last VERIFY instruction succeeded.
    pub cert_verified: bool,
    /// Shadow interval register file.
    pub interval_shadow: Vec<(u64, u64)>,
    /// Current active type tag register.
    pub tag_register: RegId,
    /// Circular buffer for register history `[cycle][reg]`.
    /// Size: MAX_REGISTERS * MAX_REG_HISTORY
    pub history: Vec<TaggedWord>,
    /// Index of the most recent cycle in the history buffer.
    pub history_cursor: usize,
    /// Number of valid cycles currently in the history buffer.
    pub history_valid_count: usize,
}

impl RspuSimulator {
    /// Create a new simulator with all state initialized to defaults.
    ///
    /// - All registers are uninitialized.
    /// - All guards are false.
    /// - PC is 0, cycle is 0, no deadline.
    pub fn new() -> Self {
        let mut guards = Vec::with_capacity(MAX_GUARDS);
        for _i in 0..MAX_GUARDS {
            guards.push(DoubleBufferedGuard::default());
        }
        // MEGA-5: Initialize interval shadow with full range for every register.
        // Bounded: exactly MAX_REGISTERS iterations.
        let mut interval_shadow = Vec::with_capacity(MAX_REGISTERS);
        for _i in 0..MAX_REGISTERS {
            interval_shadow.push((0, u64::MAX));
        }
        let mut sim = Self {
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
            history: vec![TaggedWord::uninitialized(); MAX_REGISTERS * MAX_REG_HISTORY],
            history_cursor: 0,
            history_valid_count: 0,
        };
        // MEGA-4: Record Cycle 0 state into history immediately.
        sim.push_history();
        sim
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
    /// Execute exactly one instruction from the program.
    ///
    /// This advances the program counter (PC) and updates register state.
    /// Returns StepResult to indicate if execution should continue or halt.
    pub fn step(&mut self, program: &RspuProgram) -> Result<StepResult, MirrError> {
        if self.halted {
            return Ok(StepResult::Halted);
        }
        if self.pc >= program.instructions.len() {
            self.halted = true;
            return Ok(StepResult::Halted);
        }

        let instr = &program.instructions[self.pc];
        let old_pc = self.pc;
        self.pc += 1;

        let result = self.execute_instruction(instr)?;

        // If it halted or faulted, leave PC at the instruction that caused it.
        match result {
            StepResult::Halted | StepResult::EmergencyStop | StepResult::Exception(_) => {
                self.pc = old_pc;
            }
            _ => {}
        }

        Ok(result)
    }

    /// Run a single combinatorial cycle (one full program execution until halt).
    ///
    /// This represents one "clock tick" in the hardware model.
    pub fn run_cycle(&mut self, program: &RspuProgram) -> Result<StepResult, MirrError> {
        // 1. Prepare "next" state for the new cycle.
        for g in &mut self.guards {
            g.next = g.current;
        }

        // 2. Reset PC for the start of the combinatorial cycle.
        self.pc = 0;
        self.halted = false;
        let mut inst_count = 0;
        let mut last_result: StepResult;

        // 3. Execute instructions until termination for this cycle.
        loop {
            inst_count += 1;
            if inst_count > MAX_PROGRAM_ITERATIONS {
                return Err(rspu_err(format!(
                    "{} cycle execution exceeded iteration limit",
                    crate::error_codes::ec(713)
                )));
            }

            let result = self.step(program)?;
            last_result = result.clone();
            match result {
                StepResult::Continue => {}
                _ => break,
            }
        }

        // 4. Commit state changes at the "clock edge" (end of program).
        for g in &mut self.guards {
            g.commit();
        }
        self.cycle += 1;

        // Check deadline expiry.
        if let Some(deadline_cycles) = self.deadline {
            if self.cycle >= deadline_cycles as u64 {
                self.deadline = None;
                return Ok(StepResult::Exception(ExceptionCode::DeadlineMiss));
            }
        }

        // 5. Update register history for future Prev queries.
        self.push_history();

        Ok(last_result)
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
                    return Err(rspu_err(format!(
                        "{} tag violation: unary operand is uninitialized",
                        crate::error_codes::ec(708)
                    )));
                }
                let result_val = execute_alu_unary(*op, word.value);
                self.registers.write(*dst, TaggedWord::from_computed(result_val, word.tag));
                Ok(StepResult::Continue)
            }

            // -- Temporal tier (shift register) -------------------------
            RspuInstruction::SrInit { guard, length, cond } => {
                let idx = *guard as usize;
                if idx < self.guards.len() {
                    let val = self.registers.read(*cond).value != 0;
                    if *length == 1 {
                        // Combinatorial guard (immediate)
                        let unit = GuardUnit::ShiftRegister {
                            data: if val { 1 } else { 0 },
                            length: *length,
                            input_reg: *cond,
                        };
                        self.guards[idx].current = unit;
                        self.guards[idx].next = unit;
                    } else {
                        // Sequential guard (delayed)
                        let prev_guard = self.guards[idx].current;
                        let unit = match prev_guard {
                            GuardUnit::ShiftRegister { data, .. } => {
                                if !val {
                                    // Reset on false
                                    GuardUnit::ShiftRegister {
                                        data: 0,
                                        length: *length,
                                        input_reg: *cond,
                                    }
                                } else {
                                    // Preserve on true
                                    GuardUnit::ShiftRegister {
                                        data,
                                        length: *length,
                                        input_reg: *cond,
                                    }
                                }
                            }
                            _ => {
                                // Initialize to 0 on startup
                                GuardUnit::ShiftRegister {
                                    data: 0,
                                    length: *length,
                                    input_reg: *cond,
                                }
                            }
                        };
                        self.guards[idx].current = unit;
                        self.guards[idx].next = unit;
                    }
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::SrTick { guard } => {
                let idx = *guard as usize;
                if idx < self.guards.len() {
                    if let GuardUnit::ShiftRegister { data, length, input_reg } =
                        self.guards[idx].current
                    {
                        let val = self.registers.read(input_reg).value;
                        let next_bit = if val != 0 { 1u64 } else { 0u64 };
                        let next_data = ((data << 1) | next_bit) & ((1 << length) - 1);
                        self.guards[idx].next =
                            GuardUnit::ShiftRegister { data: next_data, length, input_reg };
                    }
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::SrQuery { dst, guard } => {
                let val = if self.read_guard_current_bool(*guard) { 1u64 } else { 0u64 };
                self.registers.write(*dst, TaggedWord::from_computed(val, TypeTag::Bool));
                Ok(StepResult::Continue)
            }

            // -- Temporal tier (counter) --------------------------------
            RspuInstruction::CtrInit { guard, target, cond } => {
                let val = self.registers.read(*cond).value;
                let idx = *guard as usize;
                if idx < self.guards.len() {
                    let prev_guard = self.guards[idx].current;
                    let unit = match prev_guard {
                        GuardUnit::Counter { current, .. } => {
                            if val == 0 {
                                // If condition is false, reset count to 0
                                GuardUnit::Counter { current: 0, target: *target, input_reg: *cond }
                            } else {
                                // If condition is true, preserve existing count!
                                GuardUnit::Counter { current, target: *target, input_reg: *cond }
                            }
                        }
                        _ => {
                            // If uninitialized or other type, initialize to 0
                            GuardUnit::Counter { current: 0, target: *target, input_reg: *cond }
                        }
                    };
                    self.guards[idx].current = unit;
                    self.guards[idx].next = unit;
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::CtrTick { guard } => {
                let idx = *guard as usize;
                if idx < self.guards.len() {
                    if let GuardUnit::Counter { current, target, input_reg } =
                        self.guards[idx].current
                    {
                        let val = self.registers.read(input_reg).value;
                        let next_count = if val != 0 {
                            if current < target {
                                current + 1
                            } else {
                                current
                            }
                        } else {
                            0
                        };
                        self.guards[idx].next =
                            GuardUnit::Counter { current: next_count, target, input_reg };
                    }
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::CtrQuery { dst, guard } => {
                let val = if self.read_guard_current_bool(*guard) { 1u64 } else { 0u64 };
                self.registers.write(*dst, TaggedWord::from_computed(val, TypeTag::Bool));
                Ok(StepResult::Continue)
            }

            // -- Guard combinators --------------------------------------
            RspuInstruction::GuardAnd { dst, a, b } => {
                let result = self.read_guard_current_bool(*a) && self.read_guard_current_bool(*b);
                self.set_guard_bool(*dst, result);
                Ok(StepResult::Continue)
            }

            RspuInstruction::GuardOr { dst, a, b } => {
                let result = self.read_guard_current_bool(*a) || self.read_guard_current_bool(*b);
                self.set_guard_bool(*dst, result);
                Ok(StepResult::Continue)
            }

            // -- Reflex tier --------------------------------------------
            RspuInstruction::ReflexIf { guard, dst, src } => {
                if self.read_guard_current_bool(*guard) {
                    let word = self.registers.read(*src).clone();
                    self.registers.write(*dst, word);
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::Prev { dst, signal, delay } => {
                let val = self.get_prev_value(*signal, *delay)?;
                println!(
                    "DEBUG: Prev cycle={} signal={} delay={} val={:?}",
                    self.cycle, signal, delay, val
                );
                self.registers.write(*dst, val);
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
                } else {
                    self.properties.record_satisfaction(*property_id);
                }
                Ok(StepResult::Continue)
            }

            RspuInstruction::AssertNever { cond, property_id } => {
                let val = self.registers.read(*cond).value;
                if val != 0 {
                    self.properties.record_violation(*property_id);
                    let _action = self.exceptions.raise_exception(ExceptionCode::PropertyFail)?;
                    return Ok(StepResult::Exception(ExceptionCode::PropertyFail));
                } else {
                    self.properties.record_satisfaction(*property_id);
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
                        return Err(rspu_err(format!(
                            "{} invalid mode value: {other}",
                            crate::error_codes::ec(714),
                        )));
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
                println!("DEBUG: TagBranch tag_reg={} tag_val={} current_tag={:?} current_val={} match={} target={}", self.tag_register, tag_value, current_tag, current_tag_val, current_tag_val == *tag_value, target_pc);
                if current_tag_val == *tag_value {
                    self.pc = *target_pc as usize;
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
                // Count satisfied properties.
                let satisfied = self
                    .properties
                    .statuses
                    .values()
                    .filter(|s| **s == PropertyStatus::Satisfied)
                    .count();
                if satisfied < *expected_properties as usize {
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

    /// Read a guard boolean value, returning false if the index is out of bounds.
    pub fn read_guard_bool(&self, id: GuardId) -> bool {
        let idx = id as usize;
        if idx >= self.guards.len() {
            return false;
        }
        match self.guards[idx].current {
            GuardUnit::ShiftRegister { data, length, .. } => {
                if length == 0 {
                    return true;
                }
                let mask = if length >= 64 { !0u64 } else { (1u64 << length) - 1 };
                (data & mask) == mask
            }
            GuardUnit::Counter { current, target, .. } => current >= target,
            GuardUnit::Combinatorial(b) => b,
            GuardUnit::Uninitialized => false,
        }
    }

    /// Set a combinatorial guard value in the next-state buffer.
    /// Read a guard boolean value from the CURRENT state (beginning of cycle).
    fn read_guard_current_bool(&self, id: GuardId) -> bool {
        let idx = id as usize;
        if idx >= self.guards.len() {
            return false;
        }
        match self.guards[idx].current {
            GuardUnit::ShiftRegister { data, length, .. } => {
                if length == 0 {
                    return true;
                }
                let mask = if length >= 64 { !0u64 } else { (1u64 << length) - 1 };
                (data & mask) == mask
            }
            GuardUnit::Counter { current, target, .. } => current >= target,
            GuardUnit::Combinatorial(b) => b,
            GuardUnit::Uninitialized => false,
        }
    }

    fn set_guard_bool(&mut self, id: GuardId, value: bool) {
        let idx = id as usize;
        if idx < self.guards.len() {
            let unit = GuardUnit::Combinatorial(value);
            self.guards[idx].current = unit;
            self.guards[idx].next = unit;
        }
    }

    /// Snapshot all registers into the history buffer.
    fn push_history(&mut self) {
        let values = self.registers.get_all_values();
        // If history_valid_count > 0, move cursor to next slot.
        if self.history_valid_count > 0 {
            self.history_cursor = (self.history_cursor + 1) % MAX_REG_HISTORY;
        } else {
            self.history_cursor = 0;
        }

        let start = self.history_cursor * MAX_REGISTERS;
        for (i, val) in values.iter().enumerate() {
            self.history[start + i] = val.clone();
        }

        if self.history_valid_count < MAX_REG_HISTORY {
            self.history_valid_count += 1;
        }
    }

    /// Retrieve a historical register value.
    fn get_prev_value(&self, reg: RegId, delay: u32) -> Result<TaggedWord, MirrError> {
        if delay == 0 {
            return Ok(self.registers.read(reg).clone());
        }
        if delay as usize > self.history_valid_count {
            return Err(rspu_err(format!(
                "{} Prev delay {} exceeds available history {}",
                crate::error_codes::ec(716),
                delay,
                self.history_valid_count
            )));
        }
        // delay=1 is the most recent (last finished cycle).
        // That is at self.history_cursor.
        let offset = (delay - 1) as usize;
        let index = (self.history_cursor + MAX_REG_HISTORY - offset) % MAX_REG_HISTORY;
        // println!("DEBUG: get_prev_value delay={} cursor={} index={}", delay, self.history_cursor, index);
        let start = index * MAX_REGISTERS;
        Ok(self.history[start + reg as usize].clone())
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
        let mut cycles_executed: u64 = 0;

        self.halted = false;
        let mut final_halted = false;
        while cycles_executed < effective_max && !self.halted && terminating_exception.is_none() {
            let result = self.run_cycle(program)?;
            cycles_executed += 1;

            match result {
                StepResult::Continue => {
                    final_halted = false;
                }
                StepResult::Halted => {
                    final_halted = true;
                }
                StepResult::EmergencyStop => {
                    self.halted = true;
                    final_halted = true;
                    break;
                }
                StepResult::Exception(code) => {
                    terminating_exception = Some(code);
                    break;
                }
            }
        }

        // If an exception occurred, we still return the result but with the exception field set.
        // We only return Err for internal simulator errors (like budget exceeded).

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
            cycles: cycles_executed,
            outputs,
            property_violations: self.properties.get_violations(),
            exception: terminating_exception,
            halted: final_halted,
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
