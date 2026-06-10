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

use mirrc::emit::rspu_encoding::{decode, encode};
use mirrc::emit::rspu_isa::{RspuInstruction, TargetSpec};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA Power-of-10)
// ---------------------------------------------------------------------------

// Unused constants removed for clippy

// ---------------------------------------------------------------------------
// Helper: roundtrip encode->decode with descriptive failure message
// ---------------------------------------------------------------------------

fn roundtrip_check(instr: &RspuInstruction, label: &str) {
    let target = TargetSpec::from_config(&None);
    let encoded = encode(instr, &target).unwrap_or_else(|e| {
        panic!("roundtrip_check({label}): encode failed: {}", e.message());
    });
    let decoded = decode(encoded.0, &target).unwrap_or_else(|e| {
        panic!("roundtrip_check({label}): decode failed: {}", e.message());
    });
    assert_eq!(
        &decoded, instr,
        "roundtrip_check({label}): decoded instruction does not match original"
    );
}

// Unused fn make_program removed for clippy

// ===========================================================================
// Section 1: Bit-level packing verification
// ===========================================================================

mod sub1;
mod sub2;
mod sub3;
