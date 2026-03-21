//! Integration tests for ARM Thumb-2 emission backend.

#![forbid(unsafe_code)]
#![deny(warnings)]

use nasa_rust_project::emit::arm::emit_arm_asm;
use nasa_rust_project::emit::rspu_isa::*;

fn empty_program() -> RspuProgram {
    RspuProgram {
        instructions: vec![],
        registers_used: 0,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    }
}

fn simple_program() -> RspuProgram {
    RspuProgram {
        instructions: vec![
            RspuInstruction::LoadInput { dst: 0, port: 0 },
            RspuInstruction::StoreOutput { src: 1, port: 0 },
            RspuInstruction::Halt,
        ],
        registers_used: 2,
        guards_used: 0,
        register_map: vec![("input_0".into(), 0), ("output_0".into(), 1)],
        guard_map: vec![],
        certificate: None,
    }
}

#[test]
fn arm_emit_empty_program() {
    let program = empty_program();
    let result = emit_arm_asm(&program);
    assert!(result.is_ok(), "[E1202] ARM emission failed: {:?}", result.err());
    let asm = result.unwrap();
    assert!(asm.contains(".syntax unified"));
    assert!(asm.contains(".thumb"));
    assert!(asm.contains("_start:"));
}

#[test]
fn arm_emit_load_input() {
    let program = simple_program();
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("ldr r0"), "LOAD_INPUT should produce ldr instruction");
    assert!(asm.contains("0x40000000"), "LOAD_INPUT should use MMIO base address");
}

#[test]
fn arm_emit_store_output() {
    let program = simple_program();
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("str r1"), "STORE_OUTPUT should produce str instruction");
}

#[test]
fn arm_emit_alu_operations() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::Alu { op: AluOp::Add, dst: 2, a: 0, b: 1 },
            RspuInstruction::Alu { op: AluOp::Sub, dst: 3, a: 0, b: 1 },
            RspuInstruction::Alu { op: AluOp::And, dst: 4, a: 0, b: 1 },
            RspuInstruction::Alu { op: AluOp::Or, dst: 5, a: 0, b: 1 },
        ],
        registers_used: 6,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("add r2"), "ADD should produce add instruction");
    assert!(asm.contains("sub r3"), "SUB should produce sub instruction");
    assert!(asm.contains("and r4"), "AND should produce and instruction");
    assert!(asm.contains("orr r5"), "OR should produce orr instruction");
}

#[test]
fn arm_emit_shift_register() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::SrInit { guard: 0, length: 4, cond: 0 },
            RspuInstruction::SrTick { guard: 0 },
            RspuInstruction::SrQuery { dst: 2, guard: 0 },
        ],
        registers_used: 3,
        guards_used: 1,
        register_map: vec![("input".into(), 0)],
        guard_map: vec![("delay_guard".into(), 0)],
        certificate: None,
    };
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("sr_g0"), "SR should reference guard storage");
}

#[test]
fn arm_emit_counter() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::CtrInit { guard: 1, target: 100, cond: 0 },
            RspuInstruction::CtrTick { guard: 1 },
            RspuInstruction::CtrQuery { dst: 2, guard: 1 },
        ],
        registers_used: 3,
        guards_used: 1,
        register_map: vec![("input".into(), 0)],
        guard_map: vec![("counter_guard".into(), 1)],
        certificate: None,
    };
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("guard_g1"), "CTR should reference guard storage");
}

#[test]
fn arm_emit_reflex_if() {
    let program = RspuProgram {
        instructions: vec![RspuInstruction::ReflexIf { guard: 0, dst: 1, src: 0 }],
        registers_used: 2,
        guards_used: 1,
        register_map: vec![("input".into(), 0), ("output".into(), 1)],
        guard_map: vec![("g".into(), 0)],
        certificate: None,
    };
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("it ne"), "REFLEX_IF should produce IT block");
}

#[test]
fn arm_emit_assertions() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::AssertAlways { cond: 0, property_id: 1 },
            RspuInstruction::AssertNever { cond: 1, property_id: 2 },
        ],
        registers_used: 2,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("bkpt"), "Assertions should produce bkpt traps");
}

#[test]
fn arm_emit_full_program() {
    let program = simple_program();
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("_start:"));
    assert!(asm.contains("instr_halt:"));
    assert!(asm.contains(".section .data"));
}

#[test]
fn arm_thumb2_syntax() {
    let program = simple_program();
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains(".syntax unified"), "Should use unified syntax");
    assert!(asm.contains(".thumb"), "Should use Thumb encoding");
}

#[test]
fn arm_it_blocks() {
    let program = RspuProgram {
        instructions: vec![RspuInstruction::Alu { op: AluOp::Eq, dst: 2, a: 0, b: 1 }],
        registers_used: 3,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("cmp"), "EQ should produce cmp");
    assert!(asm.contains("it eq"), "EQ should produce IT block");
}

#[test]
fn arm_halt_produces_bkpt() {
    let program = RspuProgram {
        instructions: vec![RspuInstruction::Halt],
        registers_used: 0,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("bkpt"), "HALT should produce bkpt");
}

#[test]
fn arm_nop_produces_nop() {
    let program = RspuProgram {
        instructions: vec![RspuInstruction::Nop],
        registers_used: 0,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_arm_asm(&program).expect("[E1202] ARM emission failed");
    assert!(asm.contains("nop"), "NOP should produce nop instruction");
}
