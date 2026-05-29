#![forbid(unsafe_code)]

use nasa_rust_project::emit::arm::emit_arm_asm;
use nasa_rust_project::emit::riscv::emit_riscv_asm;
use nasa_rust_project::emit::rspu_encoding::{decode, encode};
use nasa_rust_project::emit::rspu_isa::RspuInstruction;
use nasa_rust_project::emit::rspu_isa::RspuProgram;

#[test]
fn test_tag_branch_encoding_roundtrip() {
    let instr = RspuInstruction::TagBranch { tag_value: 42, target_pc: 1024 };
    let encoded = encode(&instr).expect("encode should succeed");
    let decoded = decode(encoded.0).expect("decode should succeed");
    assert_eq!(decoded, instr, "roundtrip failed for TagBranch");
}

#[test]
fn test_tag_branch_formatting() {
    // Just ensuring format_instruction and emitting doesn't panic.
    let program = RspuProgram {
        instructions: vec![RspuInstruction::TagBranch { tag_value: 42, target_pc: 1024 }],
        registers_used: 0,
        guards_used: 0,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    };

    // Ensure ARM emitter doesn't panic and outputs the expected comment.
    let arm_asm = emit_arm_asm(&program).expect("ARM emission failed");
    assert!(arm_asm.contains("TAG_BRANCH 42, 1024"), "ARM missing TagBranch comment");

    // Ensure RISC-V emitter doesn't panic and outputs the expected comment.
    let riscv_asm = emit_riscv_asm(&program).expect("RISCV emission failed");
    assert!(riscv_asm.contains("TAG_BRANCH 42, 1024"), "RISC-V missing TagBranch comment");
}
