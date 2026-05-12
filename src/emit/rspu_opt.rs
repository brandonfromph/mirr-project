//! Bounded peephole optimization for emitted R-SPU instructions.
//!
//! This pass only applies local, semantics-preserving rewrites:
//! - fold adjacent literal ALU chains into a single `LOAD_IMM`
//! - rewrite `LOAD_IMM` + `ALU` into `ALU_IMM` where safe
//! - drop redundant self-moves (`MOV` / `REFLEX_IF` with identical src/dst)

#![forbid(unsafe_code)]

use crate::emit::rspu_isa::{AluOp, AluUnaryOp, RegId, RspuInstruction};

const LOAD_IMM_ENCODE_MAX: u64 = 0x3FF;
const ALU_IMM_ENCODE_MAX: u64 = 127;

pub(crate) fn peephole_optimize(instrs: &[RspuInstruction]) -> Vec<RspuInstruction> {
    let mut out = Vec::with_capacity(instrs.len());
    let mut i = 0usize;

    while i < instrs.len() {
        if let Some((instr, consumed)) = try_fold_binary_literals(instrs, i) {
            out.push(instr);
            i += consumed;
            continue;
        }

        if let Some((instr, consumed)) = try_fold_unary_literal(instrs, i) {
            out.push(instr);
            i += consumed;
            continue;
        }

        if let Some((instr, consumed)) = try_rewrite_alu_imm(instrs, i) {
            out.push(instr);
            i += consumed;
            continue;
        }

        match &instrs[i] {
            RspuInstruction::Mov { dst, src } if dst == src => {
                i += 1;
            }
            RspuInstruction::ReflexIf { dst, src, .. } if dst == src => {
                i += 1;
            }
            other => {
                out.push(other.clone());
                i += 1;
            }
        }
    }

    out
}

fn try_fold_binary_literals(
    instrs: &[RspuInstruction],
    i: usize,
) -> Option<(RspuInstruction, usize)> {
    if i + 2 >= instrs.len() {
        return None;
    }

    let (lhs_reg, lhs_value, lhs_width) = match &instrs[i] {
        RspuInstruction::LoadImm { dst, value, width } => (*dst, *value, *width),
        _ => return None,
    };

    let (rhs_reg, rhs_value, rhs_width) = match &instrs[i + 1] {
        RspuInstruction::LoadImm { dst, value, width } => (*dst, *value, *width),
        _ => return None,
    };

    let (op, dst, a, b) = match &instrs[i + 2] {
        RspuInstruction::Alu { op, dst, a, b } => (*op, *dst, *a, *b),
        _ => return None,
    };

    if lhs_reg == rhs_reg {
        return None;
    }

    if a != lhs_reg || b != rhs_reg {
        return None;
    }

    if reg_mentioned_after(instrs, i + 3, lhs_reg) || reg_mentioned_after(instrs, i + 3, rhs_reg) {
        return None;
    }

    let folded_value = eval_binary(op, lhs_value, rhs_value);
    if folded_value > LOAD_IMM_ENCODE_MAX {
        return None;
    }
    let folded_width = binary_result_width(op, lhs_width, rhs_width);
    let folded = RspuInstruction::LoadImm { dst, value: folded_value, width: folded_width };
    Some((folded, 3))
}

fn try_fold_unary_literal(
    instrs: &[RspuInstruction],
    i: usize,
) -> Option<(RspuInstruction, usize)> {
    if i + 1 >= instrs.len() {
        return None;
    }

    let (src_reg, value, width) = match &instrs[i] {
        RspuInstruction::LoadImm { dst, value, width } => (*dst, *value, *width),
        _ => return None,
    };

    let (op, dst, src) = match &instrs[i + 1] {
        RspuInstruction::AluUnary { op, dst, src } => (*op, *dst, *src),
        _ => return None,
    };

    if src != src_reg {
        return None;
    }

    if reg_mentioned_after(instrs, i + 2, src_reg) {
        return None;
    }

    let folded_value = eval_unary(op, value);
    if folded_value > LOAD_IMM_ENCODE_MAX {
        return None;
    }

    let folded = RspuInstruction::LoadImm { dst, value: folded_value, width };
    Some((folded, 2))
}

fn try_rewrite_alu_imm(instrs: &[RspuInstruction], i: usize) -> Option<(RspuInstruction, usize)> {
    if i + 1 >= instrs.len() {
        return None;
    }

    let (imm_reg, imm_value) = match &instrs[i] {
        RspuInstruction::LoadImm { dst, value, .. } => (*dst, *value),
        _ => return None,
    };

    let (op, dst, a, b) = match &instrs[i + 1] {
        RspuInstruction::Alu { op, dst, a, b } => (*op, *dst, *a, *b),
        _ => return None,
    };

    if reg_mentioned_after(instrs, i + 2, imm_reg) {
        return None;
    }

    if !supports_alu_imm_op(op) || imm_value > ALU_IMM_ENCODE_MAX {
        return None;
    }

    if b == imm_reg && a != imm_reg {
        return Some((RspuInstruction::AluImm { op, dst, a, imm: imm_value }, 2));
    }

    if a == imm_reg && b != imm_reg && is_commutative(op) {
        return Some((RspuInstruction::AluImm { op, dst, a: b, imm: imm_value }, 2));
    }

    None
}

fn reg_mentioned_after(instrs: &[RspuInstruction], start: usize, reg: RegId) -> bool {
    let mut idx = start;
    while idx < instrs.len() {
        if instruction_mentions_reg(&instrs[idx], reg) {
            return true;
        }
        idx += 1;
    }
    false
}

fn instruction_mentions_reg(instr: &RspuInstruction, reg: RegId) -> bool {
    match instr {
        RspuInstruction::TagBranch { .. } => todo!(),
        RspuInstruction::LoadInput { dst, .. } => *dst == reg,
        RspuInstruction::StoreOutput { src, .. } => *src == reg,
        RspuInstruction::Mov { dst, src } => *dst == reg || *src == reg,
        RspuInstruction::LoadImm { dst, .. } => *dst == reg,
        RspuInstruction::Alu { dst, a, b, .. } => *dst == reg || *a == reg || *b == reg,
        RspuInstruction::AluImm { dst, a, .. } => *dst == reg || *a == reg,
        RspuInstruction::AluUnary { dst, src, .. } => *dst == reg || *src == reg,
        RspuInstruction::SrInit { cond, .. } => *cond == reg,
        RspuInstruction::SrTick { .. } => false,
        RspuInstruction::SrQuery { dst, .. } => *dst == reg,
        RspuInstruction::CtrInit { cond, .. } => *cond == reg,
        RspuInstruction::CtrTick { .. } => false,
        RspuInstruction::CtrQuery { dst, .. } => *dst == reg,
        RspuInstruction::GuardAnd { .. } => false,
        RspuInstruction::GuardOr { .. } => false,
        RspuInstruction::ReflexIf { dst, src, .. } => *dst == reg || *src == reg,
        RspuInstruction::Prev { dst, signal, .. } => *dst == reg || *signal == reg,
        RspuInstruction::EmergencyStop => false,
        RspuInstruction::AssertAlways { cond, .. } => *cond == reg,
        RspuInstruction::AssertNever { cond, .. } => *cond == reg,
        RspuInstruction::Trap { .. } => false,
        RspuInstruction::TrapIf { cond, .. } => *cond == reg,
        RspuInstruction::Halt => false,
        RspuInstruction::ModeSwitch { .. } => false,
        RspuInstruction::Nop => false,
        RspuInstruction::Fence => false,
        RspuInstruction::TagLoad { dst, .. } => *dst == reg,
        RspuInstruction::TagCheck { src, .. } => *src == reg,
        RspuInstruction::TagRead { dst, src } => *dst == reg || *src == reg,
        RspuInstruction::DeadlineSet { .. } => false,
        RspuInstruction::Verify { .. } => false,
        RspuInstruction::Certify { dst } => *dst == reg,
        RspuInstruction::TotalCheck { .. } => false,
        RspuInstruction::Match { dst, src, .. } => *dst == reg || *src == reg,
        RspuInstruction::IntervalLo { dst, src } => *dst == reg || *src == reg,
        RspuInstruction::IntervalHi { dst, src } => *dst == reg || *src == reg,
        RspuInstruction::IntervalCheck { src, bounds } => *src == reg || *bounds == reg,
    }
}

fn is_commutative(op: AluOp) -> bool {
    matches!(
        op,
        AluOp::Add | AluOp::Mul | AluOp::And | AluOp::Or | AluOp::Xor | AluOp::Eq | AluOp::Ne
    )
}

fn supports_alu_imm_op(op: AluOp) -> bool {
    matches!(
        op,
        AluOp::Add
            | AluOp::Sub
            | AluOp::Mul
            | AluOp::And
            | AluOp::Or
            | AluOp::Xor
            | AluOp::Shl
            | AluOp::Shr
    )
}

fn eval_binary(op: AluOp, lhs: u64, rhs: u64) -> u64 {
    match op {
        AluOp::Add => lhs.wrapping_add(rhs),
        AluOp::Sub => lhs.wrapping_sub(rhs),
        AluOp::Mul => lhs.wrapping_mul(rhs),
        AluOp::And => lhs & rhs,
        AluOp::Or => lhs | rhs,
        AluOp::Xor => lhs ^ rhs,
        AluOp::Shl => lhs.wrapping_shl(rhs as u32),
        AluOp::Shr => lhs.wrapping_shr(rhs as u32),
        AluOp::Eq => u64::from(lhs == rhs),
        AluOp::Ne => u64::from(lhs != rhs),
        AluOp::Lt => u64::from(lhs < rhs),
        AluOp::Le => u64::from(lhs <= rhs),
        AluOp::Gt => u64::from(lhs > rhs),
        AluOp::Ge => u64::from(lhs >= rhs),
    }
}

fn eval_unary(op: AluUnaryOp, value: u64) -> u64 {
    match op {
        AluUnaryOp::Not => !value,
        AluUnaryOp::Negate => (value as i64).wrapping_neg() as u64,
    }
}

fn binary_result_width(op: AluOp, lhs_width: u32, rhs_width: u32) -> u32 {
    match op {
        AluOp::Eq | AluOp::Ne | AluOp::Lt | AluOp::Le | AluOp::Gt | AluOp::Ge => 1,
        AluOp::Shl | AluOp::Shr => lhs_width,
        AluOp::Add | AluOp::Sub | AluOp::Mul | AluOp::And | AluOp::Or | AluOp::Xor => {
            lhs_width.max(rhs_width)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_binary_literals_into_single_load_imm() {
        let input = vec![
            RspuInstruction::LoadImm { dst: 192, value: 6, width: 8 },
            RspuInstruction::LoadImm { dst: 193, value: 7, width: 8 },
            RspuInstruction::Alu { op: AluOp::Mul, dst: 194, a: 192, b: 193 },
        ];

        let output = peephole_optimize(&input);

        assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 194, value: 42, width: 8 }]);
    }

    #[test]
    fn folds_unary_literal_into_single_load_imm() {
        let input = vec![
            RspuInstruction::LoadImm { dst: 192, value: 0, width: 8 },
            RspuInstruction::AluUnary { op: AluUnaryOp::Negate, dst: 193, src: 192 },
        ];

        let output = peephole_optimize(&input);

        assert_eq!(output, vec![RspuInstruction::LoadImm { dst: 193, value: 0, width: 8 }]);
    }

    #[test]
    fn rewrites_literal_rhs_alu_to_alu_imm() {
        let input = vec![
            RspuInstruction::LoadImm { dst: 200, value: 7, width: 8 },
            RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 12, b: 200 },
        ];

        let output = peephole_optimize(&input);

        assert_eq!(
            output,
            vec![RspuInstruction::AluImm { op: AluOp::Add, dst: 201, a: 12, imm: 7 }]
        );
    }

    #[test]
    fn keeps_literal_load_when_register_is_used_later() {
        let input = vec![
            RspuInstruction::LoadImm { dst: 200, value: 5, width: 8 },
            RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 10, b: 200 },
            RspuInstruction::Mov { dst: 30, src: 200 },
        ];

        let output = peephole_optimize(&input);

        assert_eq!(output, input);
    }

    #[test]
    fn does_not_rewrite_alu_imm_for_large_immediate() {
        let input = vec![
            RspuInstruction::LoadImm { dst: 200, value: 255, width: 8 },
            RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 12, b: 200 },
        ];

        let output = peephole_optimize(&input);

        assert_eq!(output, input);
    }

    #[test]
    fn does_not_rewrite_when_both_operands_use_immediate_register() {
        let input = vec![
            RspuInstruction::LoadImm { dst: 200, value: 7, width: 8 },
            RspuInstruction::Alu { op: AluOp::Add, dst: 201, a: 200, b: 200 },
        ];

        let output = peephole_optimize(&input);

        assert_eq!(output, input);
    }

    #[test]
    fn does_not_fold_binary_literals_when_folded_immediate_overflows_encoding() {
        let input = vec![
            RspuInstruction::LoadImm { dst: 192, value: 1000, width: 10 },
            RspuInstruction::LoadImm { dst: 193, value: 1000, width: 10 },
            RspuInstruction::Alu { op: AluOp::Add, dst: 194, a: 192, b: 193 },
        ];

        let output = peephole_optimize(&input);

        assert_eq!(output, input);
    }

    #[test]
    fn removes_redundant_self_moves() {
        let input = vec![
            RspuInstruction::Mov { dst: 5, src: 5 },
            RspuInstruction::ReflexIf { guard: 0, dst: 7, src: 7 },
            RspuInstruction::Halt,
        ];

        let output = peephole_optimize(&input);

        assert_eq!(output, vec![RspuInstruction::Halt]);
    }
}
