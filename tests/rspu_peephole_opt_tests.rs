//! R-SPU peephole optimizer – extended edge-case and branch coverage tests.
//!
//! Exercises `src/emit/rspu_opt.rs` paths not reached by inline tests:
//!   - commutative ALU_IMM rewrite (lhs is the immediate register)
//!   - unary literal fold with ReductionOr
//!   - mixed sequences where peephole windows overlap
//!   - passthrough of non-optimisable instructions

#![forbid(unsafe_code)]

use mirrc::emit::rspu_isa::{AluOp, AluUnaryOp, RspuInstruction};
use mirrc::emit::rspu_opt::peephole_optimize;

// -----------------------------------------------------------------------
// Commutative rewrite: lhs is the immediate register
// -----------------------------------------------------------------------
#[test]
fn commutative_rewrite_swaps_lhs_immediate_to_alu_imm() {
    // LoadImm r200 = 5; ALU ADD r201 = r200 + r12  →  AluImm ADD r201, r12, #5
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 200, b: 12 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(
        output,
        vec![RspuInstruction::AluImm { op: AluOp::Add, dst: 201, a: 12, imm: 5 }]
    );
}

#[test]
fn commutative_rewrite_works_for_mul() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 3, width: 8 },
        RspuInstruction::Alu { op: AluOp::Mul, dst: 201, a: 200, b: 10 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(
        output,
        vec![RspuInstruction::AluImm { op: AluOp::Mul, dst: 201, a: 10, imm: 3 }]
    );
}

#[test]
fn non_commutative_sub_does_not_swap_lhs_immediate() {
    // Sub is NOT commutative → if lhs is the imm reg, we must NOT rewrite
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Sub, dst: 201, a: 200, b: 12 },
    ];
    let output = peephole_optimize(&input);
    // Should be left as-is (no rewrite)
    assert_eq!(output, input);
}

#[test]
fn non_commutative_shl_does_not_swap_lhs_immediate() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 2, width: 8 },
        RspuInstruction::Alu { op: AluOp::Shl, dst: 201, a: 200, b: 10 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

// -----------------------------------------------------------------------
// ReductionOr unary fold
// -----------------------------------------------------------------------
#[test]
fn folds_unary_reduction_or_nonzero_to_one() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 42, width: 8 },
        RspuInstruction::AluUnary { op: AluUnaryOp::ReductionOr, dst: 193, src: 192 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 193, value: 1, width: 8 }]);
}

#[test]
fn folds_unary_reduction_or_zero_to_zero() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
        RspuInstruction::AluUnary { op: AluUnaryOp::ReductionOr, dst: 193, src: 192 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 193, value: 0, width: 8 }]);
}

// -----------------------------------------------------------------------
// Binary literal fold with comparison ops
// -----------------------------------------------------------------------
#[test]
fn folds_binary_comparison_eq_true() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 7, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 7, width: 8 },
        RspuInstruction::Alu { op: AluOp::Eq, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    // 7 == 7 = 1, result width 1 for comparison
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 1, width: 1 }]);
}

#[test]
fn folds_binary_comparison_lt_false() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Lt, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    // 10 < 5 = 0
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 0, width: 1 }]);
}

#[test]
fn folds_binary_shift_preserves_lhs_width() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 16 },
        RspuInstruction::LoadImm { dst: 193, value: 3, width: 8 },
        RspuInstruction::Alu { op: AluOp::Shl, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    // 1 << 3 = 8, width = lhs_width = 16
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 8, width: 16 }]);
}

// -----------------------------------------------------------------------
// Unary fold overflow guard: NOT of a small value produces a huge value → no fold
// -----------------------------------------------------------------------
#[test]
fn does_not_fold_unary_not_when_result_overflows_encoding() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 0, width: 64 },
        RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: 193, src: 192 },
    ];
    let output = peephole_optimize(&input);
    // !0 = 0xFFFF_FFFF_FFFF_FFFF >> does NOT fit in 10-bit encoding → passthrough
    assert_eq!(output, input);
}

// -----------------------------------------------------------------------
// Mixed sequence: binary fold + self-move elimination
// -----------------------------------------------------------------------
#[test]
fn mixed_binary_fold_then_self_move_removal() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 3, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 4, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::Mov { dst: 10, src: 10 },
        RspuInstruction::Halt,
    ];
    let output = peephole_optimize(&input);
    assert_eq!(
        output,
        vec![
            RspuInstruction::LoadImm { dst: 194, value: 7, width: 8 },
            RspuInstruction::Halt,
        ]
    );
}

// -----------------------------------------------------------------------
// Passthrough of non-optimisable instructions
// -----------------------------------------------------------------------
#[test]
fn passthrough_fence_nop_trap_deadlineset() {
    let input = vec![
        RspuInstruction::Fence,
        RspuInstruction::Nop,
        RspuInstruction::Trap { code: 42 },
        RspuInstruction::DeadlineSet { cycles: 100 },
        RspuInstruction::EmergencyStop,
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

// -----------------------------------------------------------------------
// Binary fold blocked by register reuse
// -----------------------------------------------------------------------
#[test]
fn binary_fold_blocked_when_temp_register_reused_later() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 2, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 3, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
    ];
    let expected = vec![
        RspuInstruction::LoadImm { dst: 192, value: 2, width: 8 },
        RspuInstruction::AluImm { op: AluOp::Add, dst: 194, a: 192, imm: 3 },
        RspuInstruction::StoreOutput { src: 192, port: 0 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, expected);
}

// -----------------------------------------------------------------------
// Binary fold blocked by same src/dst registers  
// -----------------------------------------------------------------------
#[test]
fn binary_fold_blocked_when_src_regs_match() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::LoadImm { dst: 192, value: 3, width: 8 }, // same dst as first
        RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 192 },
    ];
    let output = peephole_optimize(&input);
    // lhs_reg == rhs_reg → no fold
    assert_eq!(output, input);
}

// -----------------------------------------------------------------------
// ALU_IMM rewrite blocked by unsupported op (comparison)
// -----------------------------------------------------------------------
#[test]
fn alu_imm_rewrite_blocked_for_comparison_ops() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Eq, dst: 201, a: 10, b: 200 },
    ];
    let output = peephole_optimize(&input);
    // Eq is not in supports_alu_imm_op → no rewrite
    assert_eq!(output, input);
}

// -----------------------------------------------------------------------
// RefexIf self-move removal  
// -----------------------------------------------------------------------
#[test]
fn removes_redundant_reflex_if_self_assignment() {
    let input = vec![
        RspuInstruction::ReflexIf { guard: 1, dst: 42, src: 42 },
        RspuInstruction::Nop,
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::Nop]);
}

// -----------------------------------------------------------------------
// Exhaustive eval_binary coverage via binary fold
// -----------------------------------------------------------------------
#[test]
fn folds_xor_bitwise() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 0xFF, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0x0F, width: 8 },
        RspuInstruction::Alu { op: AluOp::Xor, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 0xF0, width: 8 }]);
}

#[test]
fn folds_and_bitwise() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 0xFF, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0x0F, width: 8 },
        RspuInstruction::Alu { op: AluOp::And, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 0x0F, width: 8 }]);
}

#[test]
fn folds_or_bitwise() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 0xF0, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 0x0F, width: 8 },
        RspuInstruction::Alu { op: AluOp::Or, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 0xFF, width: 8 }]);
}

#[test]
fn folds_ne_comparison() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 1, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 2, width: 8 },
        RspuInstruction::Alu { op: AluOp::Ne, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 1, width: 1 }]);
}

#[test]
fn folds_le_comparison() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Le, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 1, width: 1 }]);
}

#[test]
fn folds_gt_comparison() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 10, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Gt, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 1, width: 1 }]);
}

#[test]
fn folds_ge_comparison() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 5, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Ge, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 1, width: 1 }]);
}

#[test]
fn folds_shr_shift() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 16, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 2, width: 8 },
        RspuInstruction::Alu { op: AluOp::Shr, dst: 194, a: 192, b: 193 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 4, width: 8 }]);
}

#[test]
fn folds_sub_underflow_prevents_fold() {
    // 3 - 5 wraps to large u64 → overflow prevents fold
    let input = vec![
        RspuInstruction::LoadImm { dst: 192, value: 3, width: 8 },
        RspuInstruction::LoadImm { dst: 193, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Sub, dst: 194, a: 192, b: 193 },
    ];
    let expected = vec![
        RspuInstruction::LoadImm { dst: 192, value: 3, width: 8 },
        RspuInstruction::AluImm { op: AluOp::Sub, dst: 194, a: 192, imm: 5 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, expected);
}

// -----------------------------------------------------------------------
// instruction_mentions_reg coverage for rarely-hit variants
// -----------------------------------------------------------------------
#[test]
fn reg_mention_blocks_fold_via_sr_init_cond() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::SrInit { guard: 0, length: 100, cond: 200 },
    ];
    let output = peephole_optimize(&input);
    // r200 is mentioned later by SrInit → no rewrite
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_ctr_init_cond() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::CtrInit { guard: 0, target: 100, cond: 200 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_prev() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::Prev { dst: 202, signal: 200, delay: 1 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_assert_always() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::AssertAlways { cond: 200, property_id: 0 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_assert_never() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::AssertNever { cond: 200, property_id: 0 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_trap_if() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::TrapIf { cond: 200, code: 0 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_tag_load_dst() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::TagLoad { dst: 200, tag: 1 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_tag_check_src() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::TagCheck { src: 200, expected: 0 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_tag_read() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::TagRead { dst: 202, src: 200 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_certify() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::Certify { dst: 200 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_match() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::Match { dst: 202, src: 200, table_offset: 0 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_interval_lo() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::IntervalLo { dst: 200, src: 10 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_interval_hi() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::IntervalHi { dst: 200, src: 10 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

#[test]
fn reg_mention_blocks_fold_via_interval_check() {
    let input = vec![
        RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
        RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
        RspuInstruction::IntervalCheck { src: 200, bounds: 10 },
    ];
    let output = peephole_optimize(&input);
    assert_eq!(output, input);
}

// -----------------------------------------------------------------------
// Empty input  
// -----------------------------------------------------------------------
#[test]
fn empty_input_produces_empty_output() {
    let output = peephole_optimize(&[]);
    assert!(output.is_empty());
}
