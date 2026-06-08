//! Extended integration tests for R-SPU instruction encoding and decoding.
//!
//! Covers all four instruction formats (R-type, I-type, G-type, S-type),
//! opcode encoding/decoding roundtrips for every assigned opcode, field
//! extraction correctness, immediate encoding boundaries, ALU funct code
//! mapping, error paths (E706 overflow, E707 unknown opcode), and
//! program-level `emit_binary` correctness.
//!
//! NASA Power-of-10 compliance:
//! - `#![forbid(unsafe_code)]`
//! - All loops use explicit `MAX_*` bounded iteration constants.
//! - No recursion in any test helper.
//! - Every `assert!` has a descriptive message string.

#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop, clippy::clone_on_copy)]

use mirrc::emit::rspu_encoding::{decode, emit_binary, encode, EncodedInstruction};
use mirrc::emit::rspu_isa::{
    AluOp, AluUnaryOp, RspuInstruction, RspuProgram, MAX_INSTRUCTIONS,
};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

/// Maximum register values to iterate over in parametric tests.
const MAX_REG_TEST_VALS: usize = 16;

/// Maximum immediate values to iterate over in parametric tests.
const MAX_IMM_TEST_VALS: usize = 32;

/// Maximum opcodes to iterate over in unknown-opcode tests.
const MAX_OPCODE_SCAN: usize = 64;

/// Maximum instructions in emit_binary stress tests.
const MAX_EMIT_STRESS: usize = 128;

// ---------------------------------------------------------------------------
// Helper: roundtrip encode->decode with descriptive failure message
// ---------------------------------------------------------------------------

fn roundtrip_check(instr: &RspuInstruction, label: &str) {
    let encoded = encode(instr).unwrap_or_else(|e| {
        panic!("roundtrip_check({label}): encode failed: {}", e.message());
    });
    let decoded = decode(encoded.0).unwrap_or_else(|e| {
        panic!("roundtrip_check({label}): decode failed: {}", e.message());
    });
    assert_eq!(
        &decoded, instr,
        "roundtrip_check({label}): decoded instruction does not match original"
    );
}

fn make_program(instructions: Vec<RspuInstruction>) -> RspuProgram {
    RspuProgram {
        instructions,
        registers_used: 256,
        guards_used: 64,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    }
}

// ===========================================================================
// Section 1: Bit-level packing verification
// ===========================================================================

mod sub1;
mod sub2;
mod sub3;
