//! R-SPU assembly emission backend.
//!
//! Follows the same pattern as the four existing backends (verilog, firrtl,
//! json_netlist, dot):
//!
//! 1. Accept `&PipelineResult`
//! 2. Allocate registers for all signals
//! 3. Walk `TemporalNetlist.guards` → emit `SR_INIT`/`CTR_INIT` + tick/query
//! 4. Walk `module.reflexes` → emit `REFLEX_IF` with expression preambles
//! 5. Walk `module.properties` → emit `ASSERT_ALWAYS`/`ASSERT_NEVER`
//! 6. Emit `LOAD_INPUT` preamble and `STORE_OUTPUT` postamble per tick
//! 7. Return `RspuProgram`
//!
//! All walks are bounded by existing pipeline limits.

#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::ast::expr::Expr;
use crate::ast::property::{PropertyDecl, PropertyFormula};
use crate::ast::types::{BinaryOp, LiteralValue, SignalKind, SignalType, UnaryOp};
use crate::error::MirrError;
use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::{CompiledGuard, ConditionKind, TemporalNetlist};

use super::rspu_isa::*;
use super::rspu_regalloc::{allocate_registers, RegAllocResult};

/// Maximum expression nodes to walk (same as verilog emitter).
const MAX_EXPR_NODES: usize = 512;

/// Emit an R-SPU program from pipeline results.
///
/// Requires temporal lowering to have run (`temporal_netlist` must be `Some`).
/// Returns `Err(E702)` if instruction budget is exceeded.
pub fn emit_rspu(result: &PipelineResult) -> Result<RspuProgram, MirrError> {
    let module = &result.program.module;
    let netlist = result.temporal_netlist.as_ref();

    // Step 1: Register allocation.
    let mut regs = allocate_registers(module)?;

    // Step 2: Guard allocation.
    let (guard_map_vec, guard_map) = allocate_guards(netlist)?;

    // Instruction accumulator.
    let mut instrs: Vec<RspuInstruction> = Vec::with_capacity(256);

    // Step 3: Load inputs (tick preamble).
    let mut port_idx: PortId = 0;
    for sig in &module.signals {
        if sig.kind == SignalKind::Input {
            let r = regs.reg(&sig.name);
            instrs.push(RspuInstruction::LoadInput { dst: r, port: port_idx });
            port_idx += 1;
        }
    }

    // Step 4: Temporal guard emission.
    if let Some(net) = netlist {
        emit_temporal_guards(&net.guards, &regs, &guard_map, &mut instrs)?;
    }

    // Step 5: Reflex emission (conditional assignments).
    for reflex in &module.reflexes {
        emit_reflex(reflex, &guard_map, &mut regs, &mut instrs)?;
    }

    // Step 6: Property assertion emission.
    emit_properties(&module.properties, &mut regs, &mut instrs)?;

    // Step 7: Store outputs (tick postamble).
    let mut out_port_idx: PortId = 0;
    for sig in &module.signals {
        if sig.kind == SignalKind::Output {
            let r = regs.reg(&sig.name);
            instrs.push(RspuInstruction::StoreOutput { src: r, port: out_port_idx });
            out_port_idx += 1;
        }
    }

    // Bounds check.
    if instrs.len() > MAX_INSTRUCTIONS {
        return Err(rspu_err(format!(
            "[E702] R-SPU instruction budget exceeded: {} instructions > {}.",
            instrs.len(),
            MAX_INSTRUCTIONS,
        )));
    }

    Ok(RspuProgram {
        registers_used: regs.total_used,
        guards_used: guard_map_vec.len(),
        register_map: regs.entries.clone(),
        guard_map: guard_map_vec,
        instructions: instrs,
    })
}

// ---------------------------------------------------------------------------
// Guard allocation
// ---------------------------------------------------------------------------

/// Guard allocation result: ordered entries for metadata + lookup map.
type GuardAllocResult = (Vec<(String, GuardId)>, HashMap<String, GuardId>);

/// Allocate guard hardware unit IDs from the temporal netlist.
///
/// Returns both the ordered vec (for program metadata) and the lookup map.
/// Bounded: at most `MAX_GUARDS` entries.
fn allocate_guards(netlist: Option<&TemporalNetlist>) -> Result<GuardAllocResult, MirrError> {
    let mut entries = Vec::new();
    let mut map = HashMap::new();
    let mut next_id: GuardId = 0;

    if let Some(net) = netlist {
        for guard in &net.guards {
            let name = guard_name(guard);
            if next_id as usize >= MAX_GUARDS {
                return Err(rspu_err(format!(
                    "[E703] R-SPU guard resource exhausted: {} guards > {}.",
                    next_id as usize + 1,
                    MAX_GUARDS,
                )));
            }
            map.insert(name.clone(), next_id);
            entries.push((name, next_id));
            next_id = next_id.saturating_add(1);
        }
    }

    Ok((entries, map))
}

fn guard_name(guard: &CompiledGuard) -> String {
    match guard {
        CompiledGuard::ShiftRegister(sr) => sr.name.clone(),
        CompiledGuard::Counter(cg) => cg.name.clone(),
        CompiledGuard::Complex(cx) => cx.name.clone(),
    }
}

// ---------------------------------------------------------------------------
// Temporal guard emission
// ---------------------------------------------------------------------------

/// Emit instructions for all temporal guards.
///
/// For each guard: init, tick, query (deterministic three-instruction pattern).
fn emit_temporal_guards(
    guards: &[CompiledGuard],
    regs: &RegAllocResult,
    guard_map: &HashMap<String, GuardId>,
    instrs: &mut Vec<RspuInstruction>,
) -> Result<(), MirrError> {
    for guard in guards {
        match guard {
            CompiledGuard::ShiftRegister(sr) => {
                let gid = guard_map[&sr.name];
                let cond_reg = condition_to_reg(&sr.condition_kind, regs);
                instrs.push(RspuInstruction::SrInit {
                    guard: gid,
                    length: sr.delay_cycles as u32,
                    cond: cond_reg,
                });
                instrs.push(RspuInstruction::SrTick { guard: gid });
                instrs.push(RspuInstruction::SrQuery { dst: cond_reg, guard: gid });
            }
            CompiledGuard::Counter(cg) => {
                let gid = guard_map[&cg.name];
                let cond_reg = condition_to_reg(&cg.condition_kind, regs);
                instrs.push(RspuInstruction::CtrInit {
                    guard: gid,
                    target: cg.target_count,
                    cond: cond_reg,
                });
                instrs.push(RspuInstruction::CtrTick { guard: gid });
                instrs.push(RspuInstruction::CtrQuery { dst: cond_reg, guard: gid });
            }
            CompiledGuard::Complex(cx) => {
                // Complex guards have sub-guards; emit them, then combine.
                emit_temporal_guards(&cx.sub_guards, regs, guard_map, instrs)?;
                // The complex guard itself is derived from combination logic.
                // For now, emit a GUARD_AND if it has exactly 2 sub-guards.
                let gid = guard_map[&cx.name];
                if cx.sub_guards.len() == 2 {
                    let a_gid = guard_map[&guard_name(&cx.sub_guards[0])];
                    let b_gid = guard_map[&guard_name(&cx.sub_guards[1])];
                    instrs.push(RspuInstruction::GuardAnd { dst: gid, a: a_gid, b: b_gid });
                }
            }
        }
    }
    Ok(())
}

/// Map a condition kind to the register holding its primary signal.
fn condition_to_reg(cond: &ConditionKind, regs: &RegAllocResult) -> RegId {
    let sig = cond.primary_signal();
    regs.map.get(sig).copied().unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Reflex emission
// ---------------------------------------------------------------------------

/// Emit instructions for a single reflex.
///
/// For each assignment:
/// 1. Evaluate RHS expression into a temporary register.
/// 2. Resolve the guard(s).
/// 3. Emit `REFLEX_IF` conditional move.
fn emit_reflex(
    reflex: &crate::ast::program::Reflex,
    guard_map: &HashMap<String, GuardId>,
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
) -> Result<(), MirrError> {
    // Resolve the first guard name to a GuardId.
    // If the guard is a direct output signal from temporal compilation,
    // its name matches an entry in guard_map. Otherwise, use guard 0.
    let gid = reflex
        .guard_names
        .first()
        .and_then(|name| guard_map.get(name.as_str()))
        .copied()
        .unwrap_or(0);

    for assignment in &reflex.assignments {
        let dst_reg = regs.map.get(&assignment.target).copied().unwrap_or(0);

        // Evaluate RHS expression into a temp register.
        let src_reg = emit_expr(&assignment.value, regs, instrs)?;

        // Conditional move.
        instrs.push(RspuInstruction::ReflexIf { guard: gid, dst: dst_reg, src: src_reg });
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Expression emission
// ---------------------------------------------------------------------------

/// Emit instructions to evaluate an expression, returning the register
/// holding the result.
///
/// Bounded: at most `MAX_EXPR_NODES` recursive visits via explicit stack.
fn emit_expr(
    expr: &Expr,
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
) -> Result<RegId, MirrError> {
    // Use explicit stack to avoid recursion (NASA P10).
    // We collect a postorder sequence, then emit instructions bottom-up.
    let mut result_stack: Vec<RegId> = Vec::with_capacity(32);
    let mut work: Vec<ExprWork> = Vec::with_capacity(32);
    work.push(ExprWork::Eval(expr));

    let mut visited = 0usize;

    while let Some(item) = work.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            return Err(rspu_err(
                "[E702] R-SPU expression exceeds maximum node count.".to_string(),
            ));
        }

        match item {
            ExprWork::Eval(e) => match e {
                Expr::Signal(name) => {
                    let r = regs.map.get(name.as_str()).copied().unwrap_or(0);
                    result_stack.push(r);
                }
                Expr::Literal(lit) => {
                    let tmp = regs
                        .alloc_temp()
                        .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
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
                    let tmp = regs
                        .alloc_temp()
                        .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                    instrs.push(RspuInstruction::Prev {
                        dst: tmp,
                        signal: sig_reg,
                        delay: *delay as u32,
                    });
                    result_stack.push(tmp);
                }
                Expr::Unary { op, operand } => {
                    // Push emit-unary marker, then evaluate operand.
                    work.push(ExprWork::EmitUnary(*op));
                    work.push(ExprWork::Eval(operand));
                }
                Expr::Binary { op, left, right } => {
                    // Push emit-binary marker, then evaluate both sides.
                    // Right first (stack reversal for left-first evaluation).
                    work.push(ExprWork::EmitBinary(*op));
                    work.push(ExprWork::Eval(right));
                    work.push(ExprWork::Eval(left));
                }
            },
            ExprWork::EmitUnary(op) => {
                let src = result_stack.pop().unwrap_or(0);
                let tmp = regs
                    .alloc_temp()
                    .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                let alu_op = match op {
                    UnaryOp::Not => AluUnaryOp::Not,
                    UnaryOp::Negate => AluUnaryOp::Negate,
                };
                instrs.push(RspuInstruction::AluUnary { op: alu_op, dst: tmp, src });
                result_stack.push(tmp);
            }
            ExprWork::EmitBinary(op) => {
                let a = result_stack.pop().unwrap_or(0);
                let b = result_stack.pop().unwrap_or(0);
                let tmp = regs
                    .alloc_temp()
                    .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                let alu_op = binary_to_alu(op);
                instrs.push(RspuInstruction::Alu { op: alu_op, dst: tmp, a, b });
                result_stack.push(tmp);
            }
        }
    }

    Ok(result_stack.pop().unwrap_or(0))
}

/// Explicit work-stack item for bounded expression traversal.
enum ExprWork<'a> {
    Eval(&'a Expr),
    EmitUnary(UnaryOp),
    EmitBinary(BinaryOp),
}

fn binary_to_alu(op: BinaryOp) -> AluOp {
    match op {
        BinaryOp::Add => AluOp::Add,
        BinaryOp::Sub => AluOp::Sub,
        BinaryOp::Mul => AluOp::Mul,
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

// ---------------------------------------------------------------------------
// Property emission
// ---------------------------------------------------------------------------

/// Emit LTL assertion tier instructions for properties.
///
/// Verification-only: these do not affect the hardware datapath.
fn emit_properties(
    properties: &[PropertyDecl],
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
) -> Result<(), MirrError> {
    for (idx, prop) in properties.iter().enumerate() {
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
                // P -> Q ≡ !P | Q : emit both, combine with OR.
                let p = emit_expr(antecedent, regs, instrs)?;
                let q = emit_expr(consequent, regs, instrs)?;
                // !P
                let not_p = regs
                    .alloc_temp()
                    .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                instrs.push(RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: not_p, src: p });
                // !P | Q
                let implies = regs
                    .alloc_temp()
                    .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                instrs.push(RspuInstruction::Alu { op: AluOp::Or, dst: implies, a: not_p, b: q });
                instrs.push(RspuInstruction::AssertAlways { cond: implies, property_id });
            }
            PropertyFormula::NeverImplies { antecedent, consequent } => {
                // never (P -> Q) ≡ assert_never(!P | Q)
                let p = emit_expr(antecedent, regs, instrs)?;
                let q = emit_expr(consequent, regs, instrs)?;
                let not_p = regs
                    .alloc_temp()
                    .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                instrs.push(RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: not_p, src: p });
                let implies = regs
                    .alloc_temp()
                    .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                instrs.push(RspuInstruction::Alu { op: AluOp::Or, dst: implies, a: not_p, b: q });
                instrs.push(RspuInstruction::AssertNever { cond: implies, property_id });
            }
            PropertyFormula::EventuallyWithin { expr, .. } => {
                // Simplified: evaluate expr, assert always (monitoring layer).
                let cond = emit_expr(expr, regs, instrs)?;
                instrs.push(RspuInstruction::AssertAlways { cond, property_id });
            }
            PropertyFormula::AlwaysFollowedBy { trigger, response, .. } => {
                // Simplified: both exprs evaluated, assert trigger -> response.
                let p = emit_expr(trigger, regs, instrs)?;
                let q = emit_expr(response, regs, instrs)?;
                let not_p = regs
                    .alloc_temp()
                    .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                instrs.push(RspuInstruction::AluUnary { op: AluUnaryOp::Not, dst: not_p, src: p });
                let implies = regs
                    .alloc_temp()
                    .ok_or_else(|| rspu_err("[E701] R-SPU temporary registers exhausted."))?;
                instrs.push(RspuInstruction::Alu { op: AluOp::Or, dst: implies, a: not_p, b: q });
                instrs.push(RspuInstruction::AssertAlways { cond: implies, property_id });
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Produce the `SignalType` bit width for LOAD_IMM annotations.
#[allow(dead_code)]
fn signal_width(ty: &SignalType) -> u32 {
    match ty {
        SignalType::Bool => 1,
        SignalType::Unsigned(w) | SignalType::Signed(w) => *w,
    }
}

fn rspu_err(msg: impl Into<String>) -> MirrError {
    MirrError::RspuError { message: msg.into() }
}
