//! R-SPU assembly emission helper functions.
//!
//! Extracted from `rspu.rs` to maintain strict 600-line file cap.

#![forbid(unsafe_code)]

use super::rspu::rspu_err;
use super::rspu_isa::*;
use super::rspu_regalloc::RegAllocResult;
use crate::ast::property::PropertyFormula;
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use crate::ast::MAX_EXPR_NODES;
use crate::emit::rspu_tagged::tag_from_signal_type;
use crate::error::MirrError;
use crate::temporal::low_level_ir::ConditionKind;

pub(crate) fn get_signal_tag_byte(name: &str, registry: &crate::ecs::Registry) -> u8 {
    if name == "true" {
        return 1; // Bool tag
    }

    for i in 0..registry.names.len() {
        if let Some(nc) = &registry.names[i] {
            if registry.resolve_name(nc.0) == name {
                if let Some(type_comp) = &registry.types[i] {
                    let tag = tag_from_signal_type(&type_comp.0.core);
                    match tag {
                        crate::emit::rspu_tagged::TypeTag::Uninitialized => return 0,
                        crate::emit::rspu_tagged::TypeTag::Bool => return 1,
                        crate::emit::rspu_tagged::TypeTag::Unsigned { width } => return width,
                        crate::emit::rspu_tagged::TypeTag::Signed { width } => {
                            return width.saturating_add(128)
                        }
                        crate::emit::rspu_tagged::TypeTag::Interval { .. } => return 2,
                    }
                }
            }
        }
    }

    16 // Default to u16
}

/// Map a condition kind to the register holding its boolean result.
pub(crate) fn condition_to_reg(
    cond: &ConditionKind,
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
    registry: &crate::ecs::Registry,
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
            let tag_byte = get_signal_tag_byte(signal, registry);
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
    expr_id: crate::ecs::EntityId,
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
    registry: &crate::ecs::Registry,
) -> Result<RegId, MirrError> {
    let mut result_stack: Vec<RegId> = Vec::with_capacity(32);
    let mut work: Vec<ExprWork> = Vec::with_capacity(32);
    work.push(ExprWork::Eval(expr_id));

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
            ExprWork::Eval(id) => {
                let idx = id.0 as usize;

                if let Some(crate::ecs::components::LiteralComponent(lit)) = &registry.literals[idx]
                {
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
                } else if let Some(crate::ecs::components::SignalRefComponent(sig_ent)) =
                    registry.signal_refs[idx]
                {
                    let sig_name = registry.names[sig_ent.0 as usize]
                        .map(|nc| registry.resolve_name(nc.0).to_string())
                        .unwrap_or_default();
                    let r = regs.map.get(sig_name.as_str()).copied().unwrap_or(0);
                    result_stack.push(r);
                } else if let Some(crate::ecs::components::PendingSignalRef(name)) =
                    &registry.pending_signal_refs[idx]
                {
                    let r = regs.map.get(name.as_str()).copied().unwrap_or(0);
                    result_stack.push(r);
                } else if let Some(crate::ecs::components::BinaryComponent { op, left, right }) =
                    &registry.binary_ops[idx]
                {
                    work.push(ExprWork::EmitBinary(*op));
                    work.push(ExprWork::Eval(*right));
                    work.push(ExprWork::Eval(*left));
                } else if let Some(crate::ecs::components::UnaryComponent { op, operand }) =
                    &registry.unary_ops[idx]
                {
                    work.push(ExprWork::EmitUnary(*op));
                    work.push(ExprWork::Eval(*operand));
                } else if let Some(crate::ecs::components::PrevComponent { signal, delay }) =
                    &registry.prev_ops[idx]
                {
                    let sig_name = if let Some(crate::ecs::components::SignalRefComponent(decl)) =
                        registry.signal_refs[signal.0 as usize]
                    {
                        registry.names[decl.0 as usize]
                            .map(|nc| registry.resolve_name(nc.0).to_string())
                            .unwrap_or_default()
                    } else if let Some(crate::ecs::components::PendingSignalRef(n)) =
                        &registry.pending_signal_refs[signal.0 as usize]
                    {
                        n.clone()
                    } else {
                        String::new()
                    };
                    let sig_reg = regs.map.get(sig_name.as_str()).copied().unwrap_or(0);
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
                } else if let Some(crate::ecs::components::ArrayIndexComponent { array, index }) =
                    &registry.array_indices[idx]
                {
                    // Quick check if the inner components are signal ref and literal
                    let arr_idx = array.0 as usize;
                    let ind_idx = index.0 as usize;

                    let mut arr_name = None;
                    if let Some(crate::ecs::components::SignalRefComponent(sig_ent)) =
                        registry.signal_refs[arr_idx]
                    {
                        arr_name = registry.names[sig_ent.0 as usize]
                            .map(|nc| registry.resolve_name(nc.0).to_string());
                    } else if let Some(crate::ecs::components::PendingSignalRef(n)) =
                        &registry.pending_signal_refs[arr_idx]
                    {
                        arr_name = Some(n.clone());
                    }

                    let mut lit_val = None;
                    if let Some(crate::ecs::components::LiteralComponent(LiteralValue::Integer(
                        v,
                    ))) = &registry.literals[ind_idx]
                    {
                        lit_val = Some(*v);
                    }

                    if let (Some(arr_name), Some(idx_val)) = (arr_name, lit_val) {
                        let flat_name = format!("{}[{}]", arr_name, idx_val);
                        let r = regs.map.get(flat_name.as_str()).copied().unwrap_or(0);
                        result_stack.push(r);
                    } else {
                        return Err(rspu_err(format!(
                            "{} R-SPU does not support dynamic array indexing.",
                            crate::error_codes::ec(720)
                        )));
                    }
                } else {
                    return Err(rspu_err(format!(
                        "{} R-SPU does not support composite or dynamic type expressions.",
                        crate::error_codes::ec(720)
                    )));
                }
            }
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

enum ExprWork {
    Eval(crate::ecs::EntityId),
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
    property_id: usize,
    prop: &crate::ecs::components::PropertyComponent,
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
    registry: &crate::ecs::Registry,
) -> Result<(), MirrError> {
    let temp_start = regs.next_temp;
    let prop_id = property_id as PropertyId;

    match &prop.formula {
        PropertyFormula::Always(_) => {
            let cond = emit_expr(prop.formula_exprs[0], regs, instrs, registry)?;
            instrs.push(RspuInstruction::AssertAlways { cond, property_id: prop_id });
        }
        PropertyFormula::Never(_) => {
            let cond = emit_expr(prop.formula_exprs[0], regs, instrs, registry)?;
            instrs.push(RspuInstruction::AssertNever { cond, property_id: prop_id });
        }
        PropertyFormula::AlwaysImplies { .. } => {
            let p = emit_expr(prop.formula_exprs[0], regs, instrs, registry)?;
            let q = emit_expr(prop.formula_exprs[1], regs, instrs, registry)?;
            let not_p = regs.alloc_temp().ok_or_else(|| {
                rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
            })?;
            instrs.push(RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: not_p, src: p });
            let implies = regs.alloc_temp().ok_or_else(|| {
                rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
            })?;
            instrs.push(RspuInstruction::Alu { op: AluOp::Or, dst: implies, a: not_p, b: q });
            instrs.push(RspuInstruction::AssertAlways { cond: implies, property_id: prop_id });
        }
        PropertyFormula::EventuallyWithin { cycles, .. } => {
            let cond = emit_expr(prop.formula_exprs[0], regs, instrs, registry)?;
            let gid = (MAX_GUARDS - 1 - property_id) as GuardId; // Allocate from top down for properties.
            instrs.push(RspuInstruction::SrInit { guard: gid, length: *cycles, cond });
            instrs.push(RspuInstruction::SrTick { guard: gid });
            let verified = regs.alloc_temp().ok_or_else(|| {
                rspu_err(format!("{} R-SPU temps exhausted.", crate::error_codes::ec(705)))
            })?;
            instrs.push(RspuInstruction::SrQuery { dst: verified, guard: gid });
            instrs.push(RspuInstruction::AssertAlways { cond: verified, property_id: prop_id });
        }
        PropertyFormula::AlwaysFollowedBy { delay_cycles, .. } => {
            let p = emit_expr(prop.formula_exprs[0], regs, instrs, registry)?;
            let q = emit_expr(prop.formula_exprs[1], regs, instrs, registry)?;
            let gid = (MAX_GUARDS - 1 - property_id) as GuardId;

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
            instrs.push(RspuInstruction::AssertAlways { cond: implies, property_id: prop_id });
        }
        _ => {}
    }

    regs.next_temp = temp_start;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::{ExtendedType, SignalKind, SignalType};
    use crate::ecs::components::{EntityKind, KindComponent, TypeComponent};
    use crate::ecs::Registry;

    #[test]
    fn test_get_signal_tag_byte() {
        let mut registry = Registry::new();
        let _mod_id = registry.create_entity("test_mod", KindComponent(EntityKind::MODULE));

        assert_eq!(get_signal_tag_byte("true", &registry), 1);

        let sig1 = registry
            .create_entity("sig_u8", KindComponent(EntityKind::SIGNAL(SignalKind::Internal)));
        registry.types[sig1.0 as usize] =
            Some(TypeComponent(ExtendedType::from_core(SignalType::Unsigned(8))));
        assert_eq!(get_signal_tag_byte("sig_u8", &registry), 8);

        let sig2 = registry
            .create_entity("sig_s16", KindComponent(EntityKind::SIGNAL(SignalKind::Internal)));
        registry.types[sig2.0 as usize] =
            Some(TypeComponent(ExtendedType::from_core(SignalType::Signed(16))));
        assert_eq!(get_signal_tag_byte("sig_s16", &registry), 144);

        let sig3 = registry
            .create_entity("sig_bool", KindComponent(EntityKind::SIGNAL(SignalKind::Internal)));
        registry.types[sig3.0 as usize] =
            Some(TypeComponent(ExtendedType::from_core(SignalType::Bool)));
        assert_eq!(get_signal_tag_byte("sig_bool", &registry), 1);

        assert_eq!(get_signal_tag_byte("unknown_sig", &registry), 16);
    }

    #[test]
    fn test_binary_to_alu() {
        assert_eq!(binary_to_alu(BinaryOp::Add), AluOp::Add);
        assert_eq!(binary_to_alu(BinaryOp::Sub), AluOp::Sub);
        assert_eq!(binary_to_alu(BinaryOp::Mul), AluOp::Mul);
        assert_eq!(binary_to_alu(BinaryOp::BitwiseOr), AluOp::Or);
        assert_eq!(binary_to_alu(BinaryOp::BitwiseAnd), AluOp::And);
        assert_eq!(binary_to_alu(BinaryOp::And), AluOp::And);
        assert_eq!(binary_to_alu(BinaryOp::Or), AluOp::Or);
        assert_eq!(binary_to_alu(BinaryOp::Xor), AluOp::Xor);
        assert_eq!(binary_to_alu(BinaryOp::Shl), AluOp::Shl);
        assert_eq!(binary_to_alu(BinaryOp::Shr), AluOp::Shr);
        assert_eq!(binary_to_alu(BinaryOp::Eq), AluOp::Eq);
        assert_eq!(binary_to_alu(BinaryOp::Ne), AluOp::Ne);
        assert_eq!(binary_to_alu(BinaryOp::Lt), AluOp::Lt);
        assert_eq!(binary_to_alu(BinaryOp::Le), AluOp::Le);
        assert_eq!(binary_to_alu(BinaryOp::Gt), AluOp::Gt);
        assert_eq!(binary_to_alu(BinaryOp::Ge), AluOp::Ge);
    }
}
