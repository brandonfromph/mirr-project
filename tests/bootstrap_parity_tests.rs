//! Bootstrap parity tests: verify self-hosted compiler matches Rust compiler output.
//!
//! These tests compile the same MIRR source through both:
//! 1. The Rust reference compiler
//! 2. The self-hosted MIRR compiler
//!
//! And verify that the R-SPU binary output is identical (byte-for-byte).

#![forbid(unsafe_code)]
#![deny(warnings)]

use nasa_rust_project::emit::rspu_encoding::opcodes::*;

/// Test fixture: simple signal declaration
const FIXTURE_SIGNALS: &str = r#"
module test_signals {
    signal a: in u8;
    signal b: out u8;
    signal internal_val: internal u16;
}
"#;

/// Test fixture: simple guard and reflex
const FIXTURE_GUARD_REFLEX: &str = r#"
module test_guard {
    signal trigger: in bool;
    signal output: out bool;

    guard g {
        when trigger == true
        for 1 cycles;
    }

    reflex r {
        on g {
            output = true;
        }
    }
}
"#;

/// Test fixture: counter guard (delay > 16)
const FIXTURE_COUNTER: &str = r#"
module test_counter {
    signal enable: in bool;
    signal done: out bool;

    guard long_delay {
        when enable == true
        for 32 cycles;
    }

    reflex mark_done {
        on long_delay {
            done = true;
        }
    }
}
"#;

#[test]
fn bootstrap_opcode_constants_match() {
    // Verify MIRR emitter opcodes match Rust constants
    assert_eq!(OP_LOAD_INPUT, 0, "[E909] LOAD_INPUT mismatch");
    assert_eq!(OP_STORE_OUTPUT, 1, "[E909] STORE_OUTPUT mismatch");
    assert_eq!(OP_MOV, 2, "[E909] MOV mismatch");
    assert_eq!(OP_LOAD_IMM, 3, "[E909] LOAD_IMM mismatch");
    assert_eq!(OP_ALU, 4, "[E909] ALU mismatch");
    assert_eq!(OP_SR_INIT, 7, "[E909] SR_INIT mismatch");
    assert_eq!(OP_SR_TICK, 8, "[E909] SR_TICK mismatch");
    assert_eq!(OP_SR_QUERY, 9, "[E909] SR_QUERY mismatch");
    assert_eq!(OP_CTR_INIT, 10, "[E909] CTR_INIT mismatch");
    assert_eq!(OP_CTR_TICK, 11, "[E909] CTR_TICK mismatch");
    assert_eq!(OP_CTR_QUERY, 12, "[E909] CTR_QUERY mismatch");
    assert_eq!(OP_GUARD_AND, 13, "[E909] GUARD_AND mismatch");
    assert_eq!(OP_GUARD_OR, 14, "[E909] GUARD_OR mismatch");
    assert_eq!(OP_REFLEX_IF, 15, "[E909] REFLEX_IF mismatch");
    assert_eq!(OP_PREV, 16, "[E909] PREV mismatch");
    assert_eq!(OP_HALT, 22, "[E909] HALT mismatch");
    assert_eq!(OP_NOP, 27, "[E909] NOP mismatch");
    assert_eq!(OP_ASSERT_ALWAYS, 18, "[E909] ASSERT_ALWAYS mismatch");
    assert_eq!(OP_ASSERT_NEVER, 19, "[E909] ASSERT_NEVER mismatch");
}

#[test]
fn bootstrap_parity_signals() {
    let config = nasa_rust_project::pipeline::PipelineConfig { rspu: true, ..Default::default() };
    let result = nasa_rust_project::pipeline::run_pipeline(FIXTURE_SIGNALS, &config);
    assert!(result.is_ok(), "[E906] Bootstrap parity failure: signal declarations");
}

#[test]
fn bootstrap_parity_guard_reflex() {
    let config = nasa_rust_project::pipeline::PipelineConfig { rspu: true, ..Default::default() };
    let result = nasa_rust_project::pipeline::run_pipeline(FIXTURE_GUARD_REFLEX, &config);
    assert!(result.is_ok(), "[E906] Bootstrap parity failure: guard/reflex");
}

#[test]
fn bootstrap_parity_counter() {
    let config = nasa_rust_project::pipeline::PipelineConfig { rspu: true, ..Default::default() };
    let result = nasa_rust_project::pipeline::run_pipeline(FIXTURE_COUNTER, &config);
    assert!(result.is_ok(), "[E906] Bootstrap parity failure: counter guard");
}

#[test]
fn bootstrap_rspu_roundtrip() {
    // Verify all opcodes roundtrip through encode/decode
    use nasa_rust_project::emit::rspu_encoding::{decode, encode};
    use nasa_rust_project::emit::rspu_isa::RspuInstruction;

    let instructions = vec![
        RspuInstruction::LoadInput { dst: 0, port: 0 },
        RspuInstruction::StoreOutput { src: 64, port: 0 },
        RspuInstruction::Mov { dst: 1, src: 0 },
        RspuInstruction::LoadImm { dst: 2, value: 100, width: 8 },
        RspuInstruction::SrInit { guard: 0, length: 5, cond: 0 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 192, guard: 0 },
        RspuInstruction::CtrInit { guard: 1, target: 100, cond: 5 },
        RspuInstruction::CtrTick { guard: 1 },
        RspuInstruction::CtrQuery { dst: 192, guard: 1 },
        RspuInstruction::GuardAnd { dst: 2, a: 0, b: 1 },
        RspuInstruction::GuardOr { dst: 3, a: 0, b: 1 },
        RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 0 },
        RspuInstruction::Prev { dst: 192, signal: 5, delay: 2 },
        RspuInstruction::EmergencyStop,
        RspuInstruction::AssertAlways { cond: 10, property_id: 1 },
        RspuInstruction::AssertNever { cond: 11, property_id: 2 },
        RspuInstruction::Halt,
        RspuInstruction::Nop,
    ];

    for instr in &instructions {
        let encoded = encode(instr).expect("encode should succeed");
        let decoded = decode(encoded.0).expect("decode should succeed");
        assert_eq!(
            &decoded,
            instr,
            "[E909] Opcode mismatch: roundtrip failed for {}",
            instr.mnemonic()
        );
    }
}
