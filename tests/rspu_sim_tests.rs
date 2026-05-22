//! Integration tests for the R-SPU simulator and binary encoding roundtrips.
//!
//! Tests cover the full RspuSimulator API: register/ALU instructions, temporal
//! guards, reflex dispatch, safety assertions, exceptions, tagged words, deadline
//! enforcement, and end-to-end binary encoding via emit_binary + decode.
//!
//! All loops are bounded (NASA Power-of-10).  No recursion.  No unsafe code.

#![forbid(unsafe_code)]

use nasa_rust_project::emit::rspu_encoding::{decode, emit_binary, encode};
use nasa_rust_project::emit::rspu_exceptions::{ExceptionCode, ExecMode};
use nasa_rust_project::emit::rspu_isa::{AluOp, RspuInstruction, RspuProgram};
use nasa_rust_project::emit::rspu_sim::RspuSimulator;
use nasa_rust_project::emit::rspu_tagged::TypeTag;

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Simulator tests
// ---------------------------------------------------------------------------

#[test]
fn test_sim_nop_program() {
    let prog = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert_eq!(result.cycles, 1);
    assert!(result.halted);
    assert!(result.exception.is_none());
    assert!(result.property_violations.is_empty());
}

#[test]
fn test_sim_load_store_identity() {
    let prog = make_program(vec![
        RspuInstruction::LoadInput { dst: 192, port: 0 },
        RspuInstruction::Mov { dst: 64, src: 192 },
        RspuInstruction::StoreOutput { src: 64, port: 0 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    sim.set_input(0, 0xCAFE, TypeTag::Unsigned { width: 16 });
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    let out = result.outputs.get(&0).expect("output port 0 should exist");
    assert_eq!(out.value, 0xCAFE);
    assert_eq!(out.tag, TypeTag::Unsigned { width: 16 });
}

#[test]
fn test_sim_alu_chain() {
    // R0 = 10, R1 = 20, R192 = R0 + R1, store R192 to output port 0
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 20, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::Mov { dst: 64, src: 194 },
        RspuInstruction::StoreOutput { src: 64, port: 0 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    let out = result.outputs.get(&0).expect("output port 0 should exist");
    assert_eq!(out.value, 30);
}

#[test]
fn test_sim_alu_subtract() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 50, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 17, width: 8 },
        RspuInstruction::Alu { op: AluOp::Sub, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    assert_eq!(sim.registers.read(194).value, 33);
}

#[test]
fn test_sim_alu_multiply() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 7, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 6, width: 8 },
        RspuInstruction::Alu { op: AluOp::Mul, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    assert_eq!(sim.registers.read(194).value, 42);
}

#[test]
fn test_sim_alu_comparison_eq() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 99, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 99, width: 8 },
        RspuInstruction::Alu { op: AluOp::Eq, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    let word = sim.registers.read(194);
    assert_eq!(word.value, 1, "equal values should produce 1");
    assert_eq!(word.tag, TypeTag::Bool);
}

#[test]
fn test_sim_alu_comparison_lt() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 10, width: 8 },
        RspuInstruction::Alu { op: AluOp::Lt, dst: 194, a: 192, b: 193 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    let word = sim.registers.read(194);
    assert_eq!(word.value, 1, "5 < 10 should produce 1");
    assert_eq!(word.tag, TypeTag::Bool);
}

#[test]
fn test_sim_guard_sr_init_query() {
    // Load a nonzero condition, init guard G0, tick, then query into R194.
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 194, guard: 0 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    // Cycle 1: SrInit + SrTick. Query should see 1 (immediate visibility for Init).
    assert_eq!(sim.registers.read(194).value, 1);

    // Cycle 2: Just Query. Should see 1.
    let prog_q =
        make_program(vec![RspuInstruction::SrQuery { dst: 194, guard: 0 }, RspuInstruction::Halt]);
    let _ = sim.run(&prog_q, 1000).expect("Cycle 2 should succeed");
    assert_eq!(sim.registers.read(194).value, 1);
}

#[test]
fn test_sim_reflex_if_active() {
    // Guard G0 is active; ReflexIf should copy src to dst.
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::LoadImm { dst: 193, value: 42, width: 8 },
        RspuInstruction::LoadImm { dst: 194, value: 0, width: 8 },
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    // Cycle 1: SrInit is immediate. Guard is now 1.
    assert_eq!(sim.registers.read(194).value, 42);

    // Now tick it and check in next cycle.
    let prog_tick = make_program(vec![RspuInstruction::SrTick { guard: 0 }, RspuInstruction::Halt]);
    sim.run(&prog_tick, 1000).unwrap();

    let prog_reflex = make_program(vec![
        RspuInstruction::LoadImm { dst: 193, value: 42, width: 8 },
        RspuInstruction::LoadImm { dst: 194, value: 0, width: 8 },
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        RspuInstruction::Halt,
    ]);
    sim.run(&prog_reflex, 1000).unwrap();
    assert_eq!(sim.registers.read(194).value, 42);
}

#[test]
fn test_sim_reflex_if_inactive() {
    // Guard G0 is inactive (cond=0); ReflexIf should NOT copy.
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 1 },
        RspuInstruction::SrInit { guard: 0, length: 1, cond: 192 },
        RspuInstruction::LoadImm { dst: 193, value: 42, width: 8 },
        RspuInstruction::LoadImm { dst: 194, value: 99, width: 8 },
        RspuInstruction::ReflexIf { guard: 0, dst: 194, src: 193 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    assert_eq!(sim.registers.read(194).value, 99, "ReflexIf should not copy when guard inactive");
}

#[test]
fn test_sim_emergency_stop() {
    let prog = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::EmergencyStop,
        RspuInstruction::Nop, // should not be reached
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    // EmergencyStop sets halted = true but it is an emergency, not a normal halt.
    assert!(result.halted);
    // The exception field is None because EmergencyStop uses StepResult::EmergencyStop,
    // not StepResult::Exception.
    assert!(result.exception.is_none());
    assert_eq!(result.cycles, 1);
}

#[test]
fn test_sim_assert_always_pass() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 0 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    assert!(result.property_violations.is_empty(), "no violation when cond is nonzero");
}

#[test]
fn test_sim_assert_always_fail() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 1 },
        RspuInstruction::AssertAlways { cond: 192, property_id: 7 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    // MEGA-4: AssertAlways raises PropertyFail exception on violation.
    assert_eq!(result.exception, Some(ExceptionCode::PropertyFail));
    assert_eq!(result.property_violations, vec![7], "violation when cond is zero");
}

#[test]
fn test_sim_assert_never_pass() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 1 },
        RspuInstruction::AssertNever { cond: 192, property_id: 3 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    assert!(result.property_violations.is_empty(), "no violation when cond is zero");
}

#[test]
fn test_sim_assert_never_fail() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::AssertNever { cond: 192, property_id: 5 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    // MEGA-4: AssertNever raises PropertyFail exception on violation.
    assert_eq!(result.exception, Some(ExceptionCode::PropertyFail));
    assert_eq!(result.property_violations, vec![5], "violation when cond is nonzero");
}

#[test]
fn test_sim_trap() {
    let prog = make_program(vec![
        RspuInstruction::Nop,
        RspuInstruction::Trap { code: 5 },
        RspuInstruction::Halt, // should not be reached
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(!result.halted, "trap does not set halted flag");
    assert_eq!(result.exception, Some(ExceptionCode::SoftwareTrap));
}

#[test]
fn test_sim_trap_if_true() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 1 },
        RspuInstruction::TrapIf { cond: 192, code: 3 },
        RspuInstruction::Halt, // should not be reached
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(!result.halted);
    assert_eq!(result.exception, Some(ExceptionCode::SoftwareTrap));
}

#[test]
fn test_sim_trap_if_false() {
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 1 },
        RspuInstruction::TrapIf { cond: 192, code: 3 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted, "trap should not fire when cond is zero");
    assert!(result.exception.is_none());
}

#[test]
fn test_sim_mode_switch() {
    let prog = make_program(vec![RspuInstruction::ModeSwitch { mode: 1 }, RspuInstruction::Halt]);
    let mut sim = RspuSimulator::new();
    assert_eq!(sim.exceptions.mode, ExecMode::Reflex);
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    assert_eq!(sim.exceptions.mode, ExecMode::Host);
}

#[test]
fn test_sim_tag_load_and_check() {
    // TagLoad sets Unsigned{width:8} (tag byte = 8), TagCheck expects the same.
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 42, width: 8 },
        RspuInstruction::TagLoad { dst: 192, tag: 8 },
        RspuInstruction::TagCheck { src: 192, expected: 8 },
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    assert!(result.exception.is_none(), "matching tag should not raise exception");
}

#[test]
fn test_sim_tag_check_violation() {
    // TagLoad sets Unsigned{width:8} (tag=8), TagCheck expects Bool (tag=1) => violation.
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::TagLoad { dst: 192, tag: 8 },
        RspuInstruction::TagCheck { src: 192, expected: 1 },
        RspuInstruction::Halt, // should not be reached
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(!result.halted);
    assert_eq!(result.exception, Some(ExceptionCode::TagViolation));
}

#[test]
fn test_sim_deadline_set_no_expiry() {
    // Set a deadline of 1000 cycles; program runs ~4 cycles, no expiry.
    let prog = make_program(vec![
        RspuInstruction::DeadlineSet { cycles: 1000 },
        RspuInstruction::Nop,
        RspuInstruction::Nop,
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 2000).expect("sim should succeed");
    assert!(result.halted);
    assert!(result.exception.is_none(), "deadline should not expire");
    assert_eq!(result.cycles, 1);
}

#[test]
fn test_sim_fence_is_noop() {
    // Fence should not alter register state or halt.
    let prog = make_program(vec![
        RspuInstruction::LoadImm { dst: 192, value: 55, width: 8 },
        RspuInstruction::Fence,
        RspuInstruction::Halt,
    ]);
    let mut sim = RspuSimulator::new();
    let result = sim.run(&prog, 1000).expect("sim should succeed");
    assert!(result.halted);
    assert_eq!(sim.registers.read(192).value, 55, "Fence should not change register state");
}

// ---------------------------------------------------------------------------
// Encoding roundtrip integration tests
// ---------------------------------------------------------------------------

#[test]
fn test_encoding_roundtrip_full_program() {
    let instructions = vec![
        RspuInstruction::LoadInput { dst: 0, port: 0 },
        RspuInstruction::LoadImm { dst: 192, value: 100, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 0 },
        RspuInstruction::StoreOutput { src: 64, port: 0 },
        RspuInstruction::SrInit { guard: 0, length: 5, cond: 192 },
        RspuInstruction::SrTick { guard: 0 },
        RspuInstruction::SrQuery { dst: 195, guard: 0 },
        RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 194 },
        RspuInstruction::Mov { dst: 193, src: 192 },
        RspuInstruction::Nop,
        RspuInstruction::Fence,
        RspuInstruction::Trap { code: 1 },
        RspuInstruction::Halt,
    ];
    let prog = make_program(instructions.clone());
    let words = emit_binary(&prog).expect("emit_binary should succeed");
    assert_eq!(words.len(), instructions.len());

    // Decode every word and verify against the original instruction.
    // Bounded: at most instructions.len() iterations.
    for (i, word) in words.iter().enumerate() {
        let decoded = decode(*word).expect("decode should succeed");
        assert_eq!(
            decoded,
            instructions[i],
            "roundtrip mismatch at instruction {i}: {}",
            instructions[i].mnemonic()
        );
    }
}

#[test]
fn test_encoding_all_alu_ops() {
    // Verify encode/decode roundtrip for every AluOp variant.
    let all_ops = [
        AluOp::Add,
        AluOp::Sub,
        AluOp::Mul,
        AluOp::And,
        AluOp::Or,
        AluOp::Xor,
        AluOp::Shl,
        AluOp::Shr,
        AluOp::Eq,
        AluOp::Ne,
        AluOp::Lt,
        AluOp::Le,
        AluOp::Gt,
        AluOp::Ge,
    ];
    // Bounded: exactly 14 iterations.
    for op in &all_ops {
        // b must fit in 6-bit field (max 63) due to I-type ALU encoding.
        let instr = RspuInstruction::Alu { op: *op, dst: 194, a: 192, b: 1 };
        let encoded = encode(&instr).expect("encode should succeed");
        let decoded = decode(encoded.0).expect("decode should succeed");
        assert_eq!(decoded, instr, "roundtrip failed for ALU op {:?}", op);
    }
}
