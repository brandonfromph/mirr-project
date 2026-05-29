//! R-SPU opcode constants and EncodedInstruction type.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Opcode constants (6-bit, 0–63)
// ---------------------------------------------------------------------------

pub const OP_LOAD_INPUT: u8 = 0;
pub const OP_STORE_OUTPUT: u8 = 1;
pub const OP_MOV: u8 = 2;
pub const OP_LOAD_IMM: u8 = 3;
pub const OP_ALU: u8 = 4;
pub const OP_ALU_IMM: u8 = 5;
pub const OP_ALU_UNARY: u8 = 6;
pub const OP_SR_INIT: u8 = 7;
pub const OP_SR_TICK: u8 = 8;
pub const OP_SR_QUERY: u8 = 9;
pub const OP_CTR_INIT: u8 = 10;
pub const OP_CTR_TICK: u8 = 11;
pub const OP_CTR_QUERY: u8 = 12;
pub const OP_GUARD_AND: u8 = 13;
pub const OP_GUARD_OR: u8 = 14;
pub const OP_REFLEX_IF: u8 = 15;
pub const OP_PREV: u8 = 16;
pub const OP_EMERGENCY_STOP: u8 = 17;
pub const OP_ASSERT_ALWAYS: u8 = 18;
pub const OP_ASSERT_NEVER: u8 = 19;
// Reserved for Wave 2 new instructions:
pub const OP_TRAP: u8 = 20;
pub const OP_TRAP_IF: u8 = 21;
pub const OP_HALT: u8 = 22;
pub const OP_MODE_SWITCH: u8 = 23;
pub const OP_TAG_LOAD: u8 = 24;
pub const OP_TAG_CHECK: u8 = 25;
pub const OP_TAG_READ: u8 = 26;
pub const OP_NOP: u8 = 27;
pub const OP_FENCE: u8 = 28;
pub const OP_DEADLINE_SET: u8 = 29;
// MEGA-4: Totality Engine instructions
pub const OP_VERIFY: u8 = 30;
pub const OP_CERTIFY: u8 = 31;
pub const OP_TOTAL_CHECK: u8 = 32;
// MEGA-5: Symbolic Reasoning instructions
pub const OP_MATCH: u8 = 33;
pub const OP_INTERVAL_LO: u8 = 34;
pub const OP_INTERVAL_HI: u8 = 35;
pub const OP_INTERVAL_CHECK: u8 = 36;
pub const OP_TAG_BRANCH: u8 = 37;

/// Total number of assigned opcodes (used + reserved).
pub const TOTAL_OPCODES: usize = 38;

/// Maximum value for a 10-bit immediate field.
pub(super) const IMM10_MAX: u64 = 0x3FF;

// ---------------------------------------------------------------------------
// Encoded instruction newtype
// ---------------------------------------------------------------------------

/// A 32-bit encoded R-SPU instruction word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodedInstruction(pub u32);
