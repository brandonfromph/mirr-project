//! MEGA-3 Subsystem Verification Test Suite — R-SPU Backend.
//!
//! NASA-style verification tests for the R-SPU instruction set architecture,
//! tagged-word register file, binary encoding, exception model, and simulator.
//!
//! Covers:
//! - E1: ISA instruction variants (37 opcodes, 5 tiers)
//! - E2: Tagged-word type safety (TypeTag, Provenance, RegisterFile)
//! - E3: Binary encoding/decoding roundtrip (32-bit fixed-width)
//! - E4: Exception model (ExceptionCode, ExecMode, ExceptionState)
//! - E5: Simulator step semantics (register, ALU, temporal, reflex, safety)
//! - E6: Full pipeline → R-SPU → simulate E2E
//! - E7: Resource limits (MAX_REGISTERS, MAX_GUARDS, MAX_INSTRUCTIONS)
//! - E8: MEGA-3 new instructions (Trap, TrapIf, Halt, ModeSwitch, Tag*, Fence)
//! - E9: MEGA-4 totality instructions (Verify, Certify, TotalCheck)
//! - E10: MEGA-5 symbolic instructions (Match, IntervalLo, IntervalHi, IntervalCheck)
//!
//! Every loop is bounded by a MAX_* constant. No recursion. No unsafe code.

#![forbid(unsafe_code)]

use mirrc::emit::rspu_encoding::{decode, encode, EncodedInstruction};
use mirrc::emit::rspu_isa::{
    AluOp, AluUnaryOp, RegId, RspuInstruction, RspuProgram, MAX_GUARDS, MAX_INSTRUCTIONS,
    MAX_REGISTERS, MAX_SIM_CYCLES, REG_INPUT_BASE, REG_INPUT_MAX, REG_INTERNAL_BASE,
    REG_INTERNAL_MAX, REG_OUTPUT_BASE, REG_OUTPUT_MAX, REG_TEMP_BASE, REG_TEMP_MAX,
};
use mirrc::emit::rspu_sim::{RspuSimulator, StepResult};
use mirrc::emit::rspu_tagged::{RegisterFile, TaggedWord, TypeTag};
use mirrc::pipeline::{run_pipeline, PipelineConfig};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum test iterations in any bounded loop.
const MAX_TEST_ITERATIONS: usize = 256;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a minimal R-SPU program from a list of instructions.
fn make_program(instrs: Vec<RspuInstruction>) -> RspuProgram {
    RspuProgram {
        instructions: instrs,
        registers_used: MAX_REGISTERS,
        guards_used: 0,
        register_map: Vec::new(),
        guard_map: Vec::new(),
        certificate: None,
    }
}

/// Run pipeline with R-SPU emission on the given MIRR source.
fn pipeline_with_rspu(src: &str) -> Result<mirrc::PipelineResult, mirrc::PipelineErrors> {
    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        temporal: true,
        rspu: true,
        simulate: false,
        totality: false,
        symbolic: false,
        ..PipelineConfig::default()
    };
    run_pipeline(src, &config)
}

// ===========================================================================
// E1: ISA instruction variants
// ===========================================================================

mod sub1;
mod sub2;
mod sub3;
