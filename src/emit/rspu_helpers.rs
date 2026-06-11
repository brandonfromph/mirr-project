//! R-SPU assembly emission helper functions.
//!
//! Extracted from `rspu.rs` to maintain strict 600-line file cap.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::property::{PropertyDecl, PropertyFormula};
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use crate::ast::MAX_EXPR_NODES;
use crate::error::MirrError;
use crate::temporal::low_level_ir::ConditionKind;

use super::rspu::{rspu_err, rspu_err_ref};
use super::rspu_isa::*;
use super::rspu_regalloc::RegAllocResult;
use crate::emit::rspu_tagged::tag_from_signal_type;

pub(crate) fn get_signal_tag_byte(name: &str, module: &crate::ast::program::Module) -> u8 {
    if name == "true" {
        return 1; // Bool tag
    }
    if let Some(sig) = module.signals.iter().find(|s| s.name == name) {
        let tag = tag_from_signal_type(&sig.ty.core);
        match tag {
            crate::emit::rspu_tagged::TypeTag::Uninitialized => 0,
            crate::emit::rspu_tagged::TypeTag::Bool => 1,
            crate::emit::rspu_tagged::TypeTag::Unsigned { width } => width,
            crate::emit::rspu_tagged::TypeTag::Signed { width } => width.saturating_add(128),
            crate::emit::rspu_tagged::TypeTag::Interval { .. } => 2,
        }
    } else {
        16 // Default to u16
    }
}

/// Map a condition kind to the register holding its boolean result.
pub(crate) fn condition_to_reg(
    cond: &ConditionKind,
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
    module: &crate::ast::program::Module,
) -> Result<RegId, MirrError> {
    match cond {
        ConditionKind::SimpleSignal(sig) => Ok(regs.map.get(sig.as_str()).copied().unwrap_or(0)),
        ConditionKind::NegatedSignal(sig) => {
            let sig_reg = regs.map.get(sig.as_str()).copied().unwrap_or(0);
            let tmp = regs.alloc_temp().ok_or_else(|| {
                rspu_err(format!(
                    "{} R-SPU temporary registers exhausted in negated guard condition.",
                    crate::error_codes::ec(708)
                ))
            })?;
            instrs.push(RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: tmp, src: sig_reg });
            Ok(tmp)
        }
        ConditionKind::PrevSignal { signal, delay } => {
            let sig_reg = regs.map.get(signal.as_str()).copied().unwrap_or(0);
            let tmp = regs.alloc_temp().ok_or_else(|| {
                rspu_err(format!(
                    "{} R-SPU temporary registers exhausted in guard condition.",
                    crate::error_codes::ec(705)
                ))
            })?;
            instrs.push(RspuInstruction::Prev { dst: tmp, signal: sig_reg, delay: *delay as u32 });
            Ok(tmp)
        }
        ConditionKind::Comparison { signal, op, value } => {
            let sig_reg = regs.map.get(signal.as_str()).copied().unwrap_or(0);

            // 1. Allocate a temporary register for the immediate value
            let imm_reg = regs.alloc_temp().ok_or_else(|| {
                rspu_err(format!(
                    "{} R-SPU temporary registers exhausted in guard condition.",
                    crate::error_codes::ec(705)
                ))
            })?;

            // 2. Load the immediate value into the temporary register
            let imm = match value {
                crate::ast::types::LiteralValue::Integer(n) => *n,
                crate::ast::types::LiteralValue::Bool(b) => {
                    if *b {
                        1
                    } else {
                        0
                    }
                }
            };

            // 3. Find the signal's tag and width to tag the immediate properly.
            let tag_byte = get_signal_tag_byte(signal, module);
            let width = if tag_byte >= 128 { tag_byte - 128 } else { tag_byte };
            let width = if width == 0 { 16 } else { width };

            instrs.push(RspuInstruction::LoadImm { dst: imm_reg, value: imm, width: width as u32 });
            instrs.push(RspuInstruction::TagLoad { dst: imm_reg, tag: tag_byte });

            // 4. Allocate a temporary register for the comparison result
            let tmp = regs.alloc_temp().ok_or_else(|| {
                rspu_err(format!(
                    "{} R-SPU temporary registers exhausted in guard condition.",
                    crate::error_codes::ec(705)
                ))
            })?;

            // 5. Emit the register-register ALU instruction
            let alu_op = binary_to_alu(*op);
            instrs.push(RspuInstruction::Alu { op: alu_op, dst: tmp, a: imm_reg, b: sig_reg });

            Ok(tmp)
        }
        ConditionKind::AlwaysTrue => {
            // Reuse the pre-initialized `true` constant register if available.
            Ok(regs.map.get("true").copied().unwrap_or(0))
        }
    }
}

/// Emit instructions to evaluate an expression, returning the register
/// holding the result.
pub(crate) fn emit_expr(
    expr: &Expr,
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
) -> Result<RegId, MirrError> {
    let mut result_stack: Vec<RegId> = Vec::with_capacity(32);
    let mut work: Vec<ExprWork> = Vec::with_capacity(32);
    work.push(ExprWork::Eval(expr));

    let mut visited = 0usize;

    while let Some(item) = work.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            return Err(rspu_err(format!(
                "{} R-SPU expression exceeds maximum node count.",
                crate::error_codes::ec(704)
            )));
        }

        match item {
            ExprWork::Eval(e) => match e {
                Expr::Signal(name) => {
                    let r = regs.map.get(name.as_str()).copied().unwrap_or(0);
                    result_stack.push(r);
                }
                Expr::Literal(lit) => {
                    let tmp = regs.alloc_temp().ok_or_else(|| {
                        rspu_err(format!(
                            "{} R-SPU temporary registers exhausted.",
                            crate::error_codes::ec(705)
                        ))
                    })?;
                    match lit {
                        LiteralValue::Integer(n) => {
                            instrs.push(RspuInstruction::LoadImm {
                                dst: tmp,
                                value: *n,
                                width: 64,
                            });
                        }
                        LiteralValue::Bool(b) => {
                            instrs.push(RspuInstruction::LoadImm {
                                dst: tmp,
                                value: if *b { 1 } else { 0 },
                                width: 1,
                            });
                        }
                    }
                    result_stack.push(tmp);
                }
                Expr::Prev { signal, delay } => {
                    let sig_reg = regs.map.get(signal.as_str()).copied().unwrap_or(0);
                    let tmp = regs.alloc_temp().ok_or_else(|| {
                        rspu_err(format!(
                            "{} R-SPU temporary registers exhausted.",
                            crate::error_codes::ec(705)
                        ))
                    })?;
                    instrs.push(RspuInstruction::Prev {
                        dst: tmp,
                        signal: sig_reg,
                        delay: *delay as u32,
                    });
                    result_stack.push(tmp);
                }
                Expr::Unary { op, operand } => {
                    work.push(ExprWork::EmitUnary(*op));
                    work.push(ExprWork::Eval(operand));
                }
                Expr::Binary { op, left, right } => {
                    work.push(ExprWork::EmitBinary(*op));
                    work.push(ExprWork::Eval(right));
                    work.push(ExprWork::Eval(left));
                }
                Expr::ArrayIndex { array, index } => {
                    if let (Expr::Signal(arr_name), Expr::Literal(LiteralValue::Integer(idx))) =
                        (array.as_ref(), index.as_ref())
                    {
                        let flat_name = format!("{}[{}]", arr_name, idx);
                        let r = regs.map.get(flat_name.as_str()).copied().unwrap_or(0);
                        result_stack.push(r);
                    } else {
                        return Err(rspu_err(format!(
                            "{} R-SPU does not support dynamic array indexing.",
                            crate::error_codes::ec(720)
                        )));
                    }
                }
                Expr::FieldAccess { .. } | Expr::ArrayLiteral(_) | Expr::StructLiteral { .. } => {
                    return Err(rspu_err(format!(
                        "{} R-SPU does not support composite type expressions.",
                        crate::error_codes::ec(720)
                    )));
                }
                Expr::UnfoldIndex(name) => {
                    return Err(rspu_err_ref(
                        crate::error_codes::ec(721),
                        format!("UnfoldIndex '{}' reached R-SPU emitter", name),
                    ));
                }
            },
            ExprWork::EmitUnary(op) => {
                let src = result_stack.pop().unwrap_or(0);
                let tmp = regs.alloc_temp().ok_or_else(|| {
                    rspu_err(format!(
                        "{} R-SPU temporary registers exhausted.",
                        crate::error_codes::ec(705)
                    ))
                })?;
                let alu_op = match op {
                    UnaryOp::Not => AluUnaryOp::Not,
                    UnaryOp::Negate => AluUnaryOp::Negate,
                    UnaryOp::ReductionOr => AluUnaryOp::ReductionOr,
                };
                instrs.push(RspuInstruction::AluUnary { op: alu_op, dst: tmp, src });
                result_stack.push(tmp);
            }
            ExprWork::EmitBinary(op) => {
                let rhs = result_stack.pop().unwrap_or(0);
                let lhs = result_stack.pop().unwrap_or(0);
                let tmp = regs.alloc_temp().ok_or_else(|| {
                    rspu_err(format!(
                        "{} R-SPU temporary registers exhausted.",
                        crate::error_codes::ec(705)
                    ))
                })?;
                let alu_op = binary_to_alu(op);
                instrs.push(RspuInstruction::Alu { op: alu_op, dst: tmp, a: lhs, b: rhs });
                result_stack.push(tmp);
            }
        }
    }

    Ok(result_stack.pop().unwrap_or(0))
}

enum ExprWork<'a> {
    Eval(&'a Expr),
    EmitUnary(UnaryOp),
    EmitBinary(BinaryOp),
}

pub(crate) fn binary_to_alu(op: BinaryOp) -> AluOp {
    match op {
        BinaryOp::Add => AluOp::Add,
        BinaryOp::Sub => AluOp::Sub,
        BinaryOp::Mul => AluOp::Mul,
        BinaryOp::BitwiseOr => AluOp::Or,
        BinaryOp::BitwiseAnd => AluOp::And,
        BinaryOp::And => AluOp::And,
        BinaryOp::Or => AluOp::Or,
        BinaryOp::Xor => AluOp::Xor,
        BinaryOp::Shl => AluOp::Shl,
        BinaryOp::Shr => AluOp::Shr,
        BinaryOp::Eq => AluOp::Eq,
        BinaryOp::Ne => AluOp::Ne,
        BinaryOp::Lt => AluOp::Lt,
        BinaryOp::Le => AluOp::Le,
        BinaryOp::Gt => AluOp::Gt,
        BinaryOp::Ge => AluOp::Ge,
    }
}

/// Emit LTL assertion tier instructions for properties.
pub(crate) fn emit_properties(
    properties: &[PropertyDecl],
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
) -> Result<(), MirrError> {
    let temp_start = regs.next_temp;

    for (idx, prop) in properties.iter().enumerate() {
        regs.next_temp = temp_start;
        let property_id = idx as PropertyId;

        match &prop.formula {
            PropertyFormula::Always(expr) => {
                let cond = emit_expr(expr, regs, instrs)?;
                instrs.push(RspuInstruction::AssertAlways { cond, property_id });
            }
            PropertyFormula::Never(expr) => {
                let cond = emit_expr(expr, regs, instrs)?;
                instrs.push(RspuInstruction::AssertNever { cond, property_id });
            }
            PropertyFormula::AlwaysImplies { antecedent, consequent } => {
                let p = emit_expr(antecedent, regs, instrs)?;
                let q = emit_expr(consequent, regs, instrs)?;
                let not_p = regs.alloc_temp().ok_or_else(|| {
                    rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
                })?;
                instrs.push(RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: not_p, src: p });
                let implies = regs.alloc_temp().ok_or_else(|| {
                    rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
                })?;
                instrs.push(RspuInstruction::Alu { op: AluOp::Or, dst: implies, a: not_p, b: q });
                instrs.push(RspuInstruction::AssertAlways { cond: implies, property_id });
            }
            PropertyFormula::EventuallyWithin { expr, cycles } => {
                let cond = emit_expr(expr, regs, instrs)?;
                let gid = (MAX_GUARDS - 1 - idx) as GuardId; // Allocate from top down for properties.
                instrs.push(RspuInstruction::SrInit { guard: gid, length: *cycles, cond });
                instrs.push(RspuInstruction::SrTick { guard: gid });
                let verified = regs.alloc_temp().ok_or_else(|| {
                    rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
                })?;
                instrs.push(RspuInstruction::SrQuery { dst: verified, guard: gid });
                instrs.push(RspuInstruction::AssertAlways { cond: verified, property_id });
            }
            PropertyFormula::AlwaysFollowedBy { trigger, response, delay_cycles } => {
                let p = emit_expr(trigger, regs, instrs)?;
                let q = emit_expr(response, regs, instrs)?;
                let gid = (MAX_GUARDS - 1 - idx) as GuardId;

                instrs.push(RspuInstruction::SrInit { guard: gid, length: *delay_cycles, cond: p });
                instrs.push(RspuInstruction::SrTick { guard: gid });

                let delayed_p = regs.alloc_temp().ok_or_else(|| {
                    rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
                })?;
                instrs.push(RspuInstruction::SrQuery { dst: delayed_p, guard: gid });

                let not_p = regs.alloc_temp().ok_or_else(|| {
                    rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
                })?;
                instrs.push(RspuInstruction::AluUnary {
                    op: AluUnaryOp::Not,
                    dst: not_p,
                    src: delayed_p,
                });
                let implies = regs.alloc_temp().ok_or_else(|| {
                    rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
                })?;
                instrs.push(RspuInstruction::Alu { op: AluOp::Or, dst: implies, a: not_p, b: q });
                instrs.push(RspuInstruction::AssertAlways { cond: implies, property_id });
            }
            _ => {}
        }
    }
    regs.next_temp = temp_start;
    Ok(())
}
