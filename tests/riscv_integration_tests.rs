//! Integration tests for RISC-V RV32I emission backend.

#![forbid(unsafe_code)]
#![deny(warnings)]

use nasa_rust_project::emit::rspu_isa::*;
use nasa_rust_project::emit::riscv::emit_riscv_asm;

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
            RspuInstruction::StoreOutput { src: 64, port: 0 },
            RspuInstruction::Halt,
        ],
        registers_used: 2,
        guards_used: 0,
        register_map: vec![("input_0".into(), 0), ("output_0".into(), 64)],
        guard_map: vec![],
        certificate: None,
    }
}

#[test]
fn riscv_emit_empty_program() {
    let program = empty_program();
    let result = emit_riscv_asm(&program);
    assert!(result.is_ok(), "[E1201] RISC-V emission failed: {:?}", result.err());
    let asm = result.unwrap();
    assert!(asm.contains(".section .text"));
    assert!(asm.contains("_start:"));
}

#[test]
fn riscv_emit_load_input() {
    let program = simple_program();
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("lw x0"), "LOAD_INPUT should produce lw instruction");
    assert!(asm.contains("0x10000000"), "LOAD_INPUT should use MMIO base address");
}

#[test]
fn riscv_emit_store_output() {
    let program = simple_program();
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("sw x64"), "STORE_OUTPUT should produce sw instruction");
}

#[test]
fn riscv_emit_alu_operations() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::Alu { op: AluOp::Add, dst: 10, a: 1, b: 2 },
            RspuInstruction::Alu { op: AluOp::Sub, dst: 11, a: 3, b: 4 },
            RspuInstruction::Alu { op: AluOp::And, dst: 12, a: 5, b: 6 },
            RspuInstruction::Alu { op: AluOp::Or, dst: 13, a: 7, b: 8 },
        ],
        registers_used: 14,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("add x10"), "ADD should produce add instruction");
    assert!(asm.contains("sub x11"), "SUB should produce sub instruction");
    assert!(asm.contains("and x12"), "AND should produce and instruction");
    assert!(asm.contains("or x13"), "OR should produce or instruction");
}

#[test]
fn riscv_emit_shift_register() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::SrInit { guard: 0, length: 4, cond: 0 },
            RspuInstruction::SrTick { guard: 0 },
            RspuInstruction::SrQuery { dst: 192, guard: 0 },
        ],
        registers_used: 1,
        guards_used: 1,
        register_map: vec![("input".into(), 0)],
        guard_map: vec![("delay_guard".into(), 0)],
        certificate: None,
    };
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("sr_g0"), "SR should reference guard storage");
}

#[test]
fn riscv_emit_counter() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::CtrInit { guard: 1, target: 100, cond: 0 },
            RspuInstruction::CtrTick { guard: 1 },
            RspuInstruction::CtrQuery { dst: 192, guard: 1 },
        ],
        registers_used: 1,
        guards_used: 1,
        register_map: vec![("input".into(), 0)],
        guard_map: vec![("counter_guard".into(), 1)],
        certificate: None,
    };
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("guard_g1"), "CTR should reference guard storage");
}

#[test]
fn riscv_emit_reflex_if() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::ReflexIf { guard: 0, dst: 64, src: 0 },
        ],
        registers_used: 2,
        guards_used: 1,
        register_map: vec![("input".into(), 0), ("output".into(), 64)],
        guard_map: vec![("g".into(), 0)],
        certificate: None,
    };
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("beq"), "REFLEX_IF should produce conditional branch");
}

#[test]
fn riscv_emit_assertions() {
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
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("ecall"), "Assertions should produce ecall traps");
}

#[test]
fn riscv_emit_full_program() {
    let program = simple_program();
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("_start:"));
    assert!(asm.contains("instr_halt:"));
    assert!(asm.contains(".section .data"));
}

#[test]
fn riscv_register_bounds() {
    let program = RspuProgram {
        instructions: vec![
            RspuInstruction::Mov { dst: 255, src: 0 },
        ],
        registers_used: 256,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("x255"), "Register 255 should be valid");
}

#[test]
fn riscv_halt_produces_ecall() {
    let program = RspuProgram {
        instructions: vec![RspuInstruction::Halt],
        registers_used: 0,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("ecall"), "HALT should produce ecall");
}

#[test]
fn riscv_nop_produces_nop() {
    let program = RspuProgram {
        instructions: vec![RspuInstruction::Nop],
        registers_used: 0,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };
    let asm = emit_riscv_asm(&program).expect("[E1201] RISC-V emission failed");
    assert!(asm.contains("nop"), "NOP should produce nop instruction");
}