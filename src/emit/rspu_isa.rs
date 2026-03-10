//! R-SPU Instruction Set Architecture (ISA) definition.
//!
//! The **Reflex Signal Processing Unit** is a safety-critical instruction-level
//! target that maps 1:1 to MIRR's three primitives:
//!
//! | MIRR Primitive | R-SPU Tier | Instructions |
//! |---------------|-----------|-------------|
//! | Signal        | Register  | `LOAD_INPUT`, `STORE_OUTPUT`, `MOV`, `LOAD_IMM` |
//! | Guard         | Temporal  | `SR_INIT`, `SR_TICK`, `SR_QUERY`, `CTR_*`, `GUARD_AND/OR` |
//! | Reflex        | Execution | `REFLEX_IF`, `PREV` |
//!
//! Plus an ALU tier for expression evaluation and a safety tier for MAPE-K.
//!
//! All resource limits are bounded constants (NASA P10).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Resource limits (NASA P10 bounded-resource model)
// ---------------------------------------------------------------------------

/// Maximum allocatable registers.
pub const MAX_REGISTERS: usize = 256;

/// Maximum temporal guard hardware units.
pub const MAX_GUARDS: usize = 64;

/// Maximum instructions in a single R-SPU program.
pub const MAX_INSTRUCTIONS: usize = 4096;

/// Maximum cycle count for the ISA simulator.
pub const MAX_SIM_CYCLES: u64 = 1_000_000;

/// Maximum trap handlers in the exception table.
pub const MAX_TRAP_HANDLERS: usize = 16;

/// Maximum nested exception depth.
pub const MAX_EXCEPTION_DEPTH: usize = 8;

// ---------------------------------------------------------------------------
// Register and port identifiers
// ---------------------------------------------------------------------------

/// A register index in the R-SPU register file.
///
/// Partitions:
/// - R0–R63: input ports
/// - R64–R127: output ports
/// - R128–R191: internal signals
/// - R192–R255: expression temporaries
pub type RegId = u8;

/// A port index (maps to a physical I/O pad).
pub type PortId = u16;

/// A guard hardware unit index.
pub type GuardId = u8;

/// A property assertion index.
pub type PropertyId = u32;

// ---------------------------------------------------------------------------
// Register partition ranges
// ---------------------------------------------------------------------------

/// First register for input ports.
pub const REG_INPUT_BASE: RegId = 0;
/// Last register for input ports (inclusive).
pub const REG_INPUT_MAX: RegId = 63;

/// First register for output ports.
pub const REG_OUTPUT_BASE: RegId = 64;
/// Last register for output ports (inclusive).
pub const REG_OUTPUT_MAX: RegId = 127;

/// First register for internal signals.
pub const REG_INTERNAL_BASE: RegId = 128;
/// Last register for internal signals (inclusive).
pub const REG_INTERNAL_MAX: RegId = 191;

/// First register for expression temporaries.
pub const REG_TEMP_BASE: RegId = 192;
/// Last register for expression temporaries (inclusive).
pub const REG_TEMP_MAX: RegId = 255;

// ---------------------------------------------------------------------------
// ALU operation codes
// ---------------------------------------------------------------------------

/// Binary ALU operations (maps to `BinaryOp`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AluOp {
    /// Addition.
    Add,
    /// Subtraction.
    Sub,
    /// Multiplication.
    Mul,
    /// Logical/bitwise AND.
    And,
    /// Logical/bitwise OR.
    Or,
    /// Bitwise XOR.
    Xor,
    /// Left shift.
    Shl,
    /// Right shift.
    Shr,
    /// Equality comparison.
    Eq,
    /// Inequality comparison.
    Ne,
    /// Less than.
    Lt,
    /// Less than or equal.
    Le,
    /// Greater than.
    Gt,
    /// Greater than or equal.
    Ge,
}

/// Unary ALU operations (maps to `UnaryOp`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AluUnaryOp {
    /// Logical/bitwise NOT.
    Not,
    /// Arithmetic negation (two's complement).
    Negate,
}

// ---------------------------------------------------------------------------
// R-SPU instruction set
// ---------------------------------------------------------------------------

/// A single R-SPU instruction.
///
/// Every variant executes in a single cycle (deterministic timing model).
/// The instruction set is organized into five tiers matching the MIRR
/// compilation pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RspuInstruction {
    // -- Register tier --------------------------------------------------
    /// Load an input port value into a register.
    LoadInput { dst: RegId, port: PortId },
    /// Store a register value to an output port.
    StoreOutput { src: RegId, port: PortId },
    /// Copy one register to another.
    Mov { dst: RegId, src: RegId },
    /// Load an immediate value into a register.
    LoadImm { dst: RegId, value: u64, width: u32 },

    // -- ALU tier -------------------------------------------------------
    /// Binary ALU operation: `dst = a op b`.
    Alu { op: AluOp, dst: RegId, a: RegId, b: RegId },
    /// Binary ALU with immediate: `dst = a op imm`.
    AluImm { op: AluOp, dst: RegId, a: RegId, imm: u64 },
    /// Unary ALU operation: `dst = op(src)`.
    AluUnary { op: AluUnaryOp, dst: RegId, src: RegId },

    // -- Temporal tier --------------------------------------------------
    /// Initialize a shift-register guard unit.
    SrInit { guard: GuardId, length: u32, cond: RegId },
    /// Advance a shift-register guard by one tick.
    SrTick { guard: GuardId },
    /// Query a shift-register guard result into a register.
    SrQuery { dst: RegId, guard: GuardId },

    /// Initialize a counter guard unit.
    CtrInit { guard: GuardId, target: u64, cond: RegId },
    /// Advance a counter guard by one tick.
    CtrTick { guard: GuardId },
    /// Query a counter guard result into a register.
    CtrQuery { dst: RegId, guard: GuardId },

    /// Combine two guards with AND: dst = a & b.
    GuardAnd { dst: GuardId, a: GuardId, b: GuardId },
    /// Combine two guards with OR: dst = a | b.
    GuardOr { dst: GuardId, a: GuardId, b: GuardId },

    // -- Reflex tier ----------------------------------------------------
    /// Conditional move: if guard is active, dst = src.
    ReflexIf { guard: GuardId, dst: RegId, src: RegId },
    /// Previous-tick register: dst = signal value at t - delay.
    Prev { dst: RegId, signal: RegId, delay: u32 },

    // -- Safety tier (MAPE-K actions) -----------------------------------
    /// Halt the R-SPU immediately (non-recoverable safety action).
    EmergencyStop,

    // -- LTL Assertion tier (verification only) -------------------------
    /// Assert that cond is always true (verification register, no datapath).
    AssertAlways { cond: RegId, property_id: PropertyId },
    /// Assert that cond is never true (verification register, no datapath).
    AssertNever { cond: RegId, property_id: PropertyId },

    // -- Exception tier (MEGA-3) -----------------------------------------
    /// Raise software trap with error code.
    Trap { code: u8 },
    /// Conditional trap: if cond register != 0, raise trap with code.
    TrapIf { cond: RegId, code: u8 },
    /// Graceful halt: quiesce and stop (recoverable).
    Halt,

    // -- Control tier (MEGA-3) -------------------------------------------
    /// Switch between reflex and host execution modes.
    ModeSwitch { mode: u8 },
    /// No operation (pipeline alignment / padding).
    Nop,
    /// Memory/pipeline fence (ordering barrier).
    Fence,

    // -- Tagged tier (MEGA-3) --------------------------------------------
    /// Set type tag on a register.
    TagLoad { dst: RegId, tag: u8 },
    /// Trap E708 if register's tag doesn't match expected.
    TagCheck { src: RegId, expected: u8 },
    /// Copy type tag from src register into dst as integer value.
    TagRead { dst: RegId, src: RegId },

    // -- Temporal extension (MEGA-3) -------------------------------------
    /// Set deadline counter; trap E715 on expiry.
    DeadlineSet { cycles: u32 },
}

impl RspuInstruction {
    /// Return the R-SPU assembly mnemonic for this instruction.
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::LoadInput { .. } => "LOAD_INPUT",
            Self::StoreOutput { .. } => "STORE_OUTPUT",
            Self::Mov { .. } => "MOV",
            Self::LoadImm { .. } => "LOAD_IMM",
            Self::Alu { .. } => "ALU",
            Self::AluImm { .. } => "ALU_IMM",
            Self::AluUnary { .. } => "ALU_UNARY",
            Self::SrInit { .. } => "SR_INIT",
            Self::SrTick { .. } => "SR_TICK",
            Self::SrQuery { .. } => "SR_QUERY",
            Self::CtrInit { .. } => "CTR_INIT",
            Self::CtrTick { .. } => "CTR_TICK",
            Self::CtrQuery { .. } => "CTR_QUERY",
            Self::GuardAnd { .. } => "GUARD_AND",
            Self::GuardOr { .. } => "GUARD_OR",
            Self::ReflexIf { .. } => "REFLEX_IF",
            Self::Prev { .. } => "PREV",
            Self::EmergencyStop => "EMERGENCY_STOP",
            Self::AssertAlways { .. } => "ASSERT_ALWAYS",
            Self::AssertNever { .. } => "ASSERT_NEVER",
            Self::Trap { .. } => "TRAP",
            Self::TrapIf { .. } => "TRAP_IF",
            Self::Halt => "HALT",
            Self::ModeSwitch { .. } => "MODE_SWITCH",
            Self::TagLoad { .. } => "TAG_LOAD",
            Self::TagCheck { .. } => "TAG_CHECK",
            Self::TagRead { .. } => "TAG_READ",
            Self::Nop => "NOP",
            Self::Fence => "FENCE",
            Self::DeadlineSet { .. } => "DEADLINE_SET",
        }
    }
}

// ---------------------------------------------------------------------------
// R-SPU program
// ---------------------------------------------------------------------------

/// A complete R-SPU program: a bounded sequence of instructions with
/// resource metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RspuProgram {
    /// Ordered instruction sequence.
    pub instructions: Vec<RspuInstruction>,
    /// Number of registers used (across all partitions).
    pub registers_used: usize,
    /// Number of guard hardware units used.
    pub guards_used: usize,
    /// Human-readable register map: signal_name -> RegId.
    pub register_map: Vec<(String, RegId)>,
    /// Human-readable guard map: guard_name -> GuardId.
    pub guard_map: Vec<(String, GuardId)>,
}

impl RspuProgram {
    /// Emit R-SPU assembly as a text string.
    ///
    /// Each line is one instruction with operands.
    /// Bounded: iterates over instructions vec (max `MAX_INSTRUCTIONS`).
    pub fn emit_asm(&self) -> String {
        let mut out = String::with_capacity(self.instructions.len() * 40);

        out.push_str("; R-SPU Assembly — generated by MIRR compiler\n");
        out.push_str(&format!("; Registers used: {}\n", self.registers_used));
        out.push_str(&format!("; Guards used:    {}\n", self.guards_used));
        out.push_str(&format!("; Instructions:   {}\n", self.instructions.len()));
        out.push_str(";\n; Register map:\n");

        for (name, reg) in &self.register_map {
            out.push_str(&format!(";   R{reg:<3} = {name}\n"));
        }
        out.push_str(";\n; Guard map:\n");
        for (name, gid) in &self.guard_map {
            out.push_str(&format!(";   G{gid:<3} = {name}\n"));
        }
        out.push('\n');

        for (i, instr) in self.instructions.iter().enumerate() {
            out.push_str(&format!("{:>4}:  {}\n", i, format_instruction(instr)));
        }

        out
    }
}

/// Format a single instruction as R-SPU assembly text.
fn format_instruction(instr: &RspuInstruction) -> String {
    match instr {
        RspuInstruction::LoadInput { dst, port } => {
            format!("LOAD_INPUT  R{dst}, P{port}")
        }
        RspuInstruction::StoreOutput { src, port } => {
            format!("STORE_OUTPUT R{src}, P{port}")
        }
        RspuInstruction::Mov { dst, src } => {
            format!("MOV         R{dst}, R{src}")
        }
        RspuInstruction::LoadImm { dst, value, width } => {
            format!("LOAD_IMM    R{dst}, {value} (w{width})")
        }
        RspuInstruction::Alu { op, dst, a, b } => {
            format!("ALU         R{dst}, R{a}, R{b}, {}", alu_op_str(*op))
        }
        RspuInstruction::AluImm { op, dst, a, imm } => {
            format!("ALU_IMM     R{dst}, R{a}, {imm}, {}", alu_op_str(*op))
        }
        RspuInstruction::AluUnary { op, dst, src } => {
            format!("ALU_UNARY   R{dst}, R{src}, {}", alu_unary_str(*op))
        }
        RspuInstruction::SrInit { guard, length, cond } => {
            format!("SR_INIT     G{guard}, {length}, R{cond}")
        }
        RspuInstruction::SrTick { guard } => {
            format!("SR_TICK     G{guard}")
        }
        RspuInstruction::SrQuery { dst, guard } => {
            format!("SR_QUERY    R{dst}, G{guard}")
        }
        RspuInstruction::CtrInit { guard, target, cond } => {
            format!("CTR_INIT    G{guard}, {target}, R{cond}")
        }
        RspuInstruction::CtrTick { guard } => {
            format!("CTR_TICK    G{guard}")
        }
        RspuInstruction::CtrQuery { dst, guard } => {
            format!("CTR_QUERY   R{dst}, G{guard}")
        }
        RspuInstruction::GuardAnd { dst, a, b } => {
            format!("GUARD_AND   G{dst}, G{a}, G{b}")
        }
        RspuInstruction::GuardOr { dst, a, b } => {
            format!("GUARD_OR    G{dst}, G{a}, G{b}")
        }
        RspuInstruction::ReflexIf { guard, dst, src } => {
            format!("REFLEX_IF   G{guard}, R{dst}, R{src}")
        }
        RspuInstruction::Prev { dst, signal, delay } => {
            format!("PREV        R{dst}, R{signal}, {delay}")
        }
        RspuInstruction::EmergencyStop => "EMERGENCY_STOP".to_string(),
        RspuInstruction::AssertAlways { cond, property_id } => {
            format!("ASSERT_ALWAYS R{cond}, #{property_id}")
        }
        RspuInstruction::AssertNever { cond, property_id } => {
            format!("ASSERT_NEVER R{cond}, #{property_id}")
        }
        RspuInstruction::Trap { code } => format!("TRAP        {code}"),
        RspuInstruction::TrapIf { cond, code } => format!("TRAP_IF     R{cond}, {code}"),
        RspuInstruction::Halt => "HALT".to_string(),
        RspuInstruction::ModeSwitch { mode } => format!("MODE_SWITCH {mode}"),
        RspuInstruction::Nop => "NOP".to_string(),
        RspuInstruction::Fence => "FENCE".to_string(),
        RspuInstruction::TagLoad { dst, tag } => format!("TAG_LOAD    R{dst}, T{tag}"),
        RspuInstruction::TagCheck { src, expected } => format!("TAG_CHECK   R{src}, T{expected}"),
        RspuInstruction::TagRead { dst, src } => format!("TAG_READ    R{dst}, R{src}"),
        RspuInstruction::DeadlineSet { cycles } => format!("DEADLINE_SET {cycles}"),
    }
}

fn alu_op_str(op: AluOp) -> &'static str {
    match op {
        AluOp::Add => "ADD",
        AluOp::Sub => "SUB",
        AluOp::Mul => "MUL",
        AluOp::And => "AND",
        AluOp::Or => "OR",
        AluOp::Xor => "XOR",
        AluOp::Shl => "SHL",
        AluOp::Shr => "SHR",
        AluOp::Eq => "EQ",
        AluOp::Ne => "NE",
        AluOp::Lt => "LT",
        AluOp::Le => "LE",
        AluOp::Gt => "GT",
        AluOp::Ge => "GE",
    }
}

fn alu_unary_str(op: AluUnaryOp) -> &'static str {
    match op {
        AluUnaryOp::Not => "NOT",
        AluUnaryOp::Negate => "NEG",
    }
}
