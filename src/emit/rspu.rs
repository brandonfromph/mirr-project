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

use crate::ast::types::SignalKind;
use crate::ecs::EntityKind;
use crate::error::MirrError;
use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};

use super::rspu_isa::*;
use std::collections::{HashMap, HashSet};
use super::rspu_regalloc::{allocate_registers, RegAllocResult};
use crate::emit::rspu_helpers::{condition_to_reg, emit_expr, emit_properties};
use crate::emit::rspu_opt::peephole_optimize;

/// Emit an R-SPU program from pipeline results.
pub fn emit_rspu(result: &PipelineResult) -> Result<RspuProgram, MirrError> {
    let registry = result.ecs_registry.as_ref().expect("ECS registry required for Phase 6 emission");
    let netlist = result.temporal_netlist.as_ref();
    let target_spec = TargetSpec::from_config(&result.program.target);

    // Step 1: Register allocation.
    let mut regs = allocate_registers(registry, &target_spec)?;

    // Step 2: Guard allocation.
    let (guard_map_vec, guard_map) = allocate_guards(netlist, registry, &target_spec)?;

    // Build a helper map to check compiled guards by name in emit_reflex.
    let mut compiled_guard_map = HashMap::new();
    if let Some(net) = netlist {
        let mut stack = Vec::new();
        for guard in net.guards.iter().rev() {
            stack.push(guard);
        }
        while let Some(guard) = stack.pop() {
            let name = guard_name(guard);
            compiled_guard_map.insert(name.clone(), guard);
            if let CompiledGuard::Complex(cx) = guard {
                for sub in cx.sub_guards.iter().rev() {
                    stack.push(sub);
                }
            }
        }
    }

    // Instruction accumulator.
    let mut instrs: Vec<RspuInstruction> = Vec::with_capacity(256);

    // Step 3: Load inputs (tick preamble).
    let mut port_idx: PortId = 0;
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(type_comp)) = (
            &registry.names[i],
            &registry.kinds[i],
            &registry.types[i],
        ) {
            if let EntityKind::SIGNAL(SignalKind::Input) = kind_comp.0 {
                let name = &name_comp.0;
                let size = match &type_comp.0.core {
                    crate::ast::types::SignalType::Array { length, .. } => *length as usize,
                    _ => 1,
                };
                if size == 1 {
                    let r = regs.reg(name);
                    instrs.push(RspuInstruction::LoadInput { dst: r, port: port_idx });
                    port_idx += 1;
                } else {
                    for idx in 0..size {
                        let r = regs.reg(&format!("{}[{}]", name, idx));
                        instrs.push(RspuInstruction::LoadInput { dst: r, port: port_idx });
                        port_idx += 1;
                    }
                }
            }
        }
    }

    // Step 3.5: Emit TAG_LOAD for each signal (type tag metadata).
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(type_comp)) = (
            &registry.names[i],
            &registry.kinds[i],
            &registry.types[i],
        ) {
            if let EntityKind::SIGNAL(_) = kind_comp.0 {
                let name = &name_comp.0;
                let (size, element_ty) = match &type_comp.0.core {
                    crate::ast::types::SignalType::Array { length, element } => {
                        (*length as usize, &**element)
                    }
                    other => (1, other),
                };

                let tag = crate::emit::rspu_tagged::tag_from_signal_type(element_ty);
                let tag_byte = match tag {
                    crate::emit::rspu_tagged::TypeTag::Bool => 1,
                    crate::emit::rspu_tagged::TypeTag::Unsigned { width } => {
                        if width == 64 {
                            64
                        } else if width == 32 {
                            32
                        } else if width == 16 {
                            16
                        } else {
                            0 // T0 placeholder for complex types
                        }
                    }
                    crate::emit::rspu_tagged::TypeTag::Signed { width: _ } => 128, // High bit set for signed
                    crate::emit::rspu_tagged::TypeTag::Uninitialized => 0,
                    crate::emit::rspu_tagged::TypeTag::Interval { .. } => 0,
                };

                if size == 1 {
                    let r = regs.reg(name);
                    instrs.push(RspuInstruction::TagLoad { dst: r, tag: tag_byte });
                } else {
                    for idx in 0..size {
                        let r = regs.reg(&format!("{}[{}]", name, idx));
                        instrs.push(RspuInstruction::TagLoad { dst: r, tag: tag_byte });
                    }
                }
            }
        }
    }

    // Step 3.6: Initialize constant registers.
    if let Some(true_reg) = regs.map.get("true") {
        instrs.push(RspuInstruction::LoadImm { dst: *true_reg, value: 1, width: 1 });
        instrs.push(RspuInstruction::TagLoad { dst: *true_reg, tag: 1 });
    }

    // Step 4: Temporal guard emission.
    if let Some(net) = netlist {
        emit_temporal_guards(&net.guards, &mut regs, &guard_map, &mut instrs, registry)?;
    }

    // Step 4.5: Initialize signal-based guards.
    for (name, &gid) in &guard_map {
        if name != "always" && regs.map.contains_key(name) {
            let cond_reg = regs.map[name];
            instrs.push(RspuInstruction::SrInit { guard: gid, length: 1, cond: cond_reg });
            instrs.push(RspuInstruction::SrTick { guard: gid });
        }
    }

    // Step 5: Reflex emission (conditional assignments).
    for reflex_comp_opt in &registry.reflex_comps {
        if let Some(reflex_comp) = reflex_comp_opt {
            emit_reflex(reflex_comp, &guard_map, &compiled_guard_map, &mut regs, &mut instrs, registry)?;
        }
    }

    // Step 6: Property assertion emission.
    let mut logical_prop_idx = 0;
    for (_idx, prop_comp_opt) in registry.property_comps.iter().enumerate() {
        if let Some(prop_comp) = prop_comp_opt {
            emit_properties(logical_prop_idx, prop_comp, &mut regs, &mut instrs, registry)?;
            logical_prop_idx += 1;
        }
    }

    // Step 7: Store outputs (tick postamble).
    let mut out_port_idx: PortId = 0;
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(type_comp)) = (
            &registry.names[i],
            &registry.kinds[i],
            &registry.types[i],
        ) {
            if let EntityKind::SIGNAL(SignalKind::Output) = kind_comp.0 {
                let name = &name_comp.0;
                let size = match &type_comp.0.core {
                    crate::ast::types::SignalType::Array { length, .. } => *length as usize,
                    _ => 1,
                };
                if size == 1 {
                    let r = regs.reg(name);
                    instrs.push(RspuInstruction::StoreOutput { src: r, port: out_port_idx });
                    out_port_idx += 1;
                } else {
                    for idx in 0..size {
                        let r = regs.reg(&format!("{}[{}]", name, idx));
                        instrs.push(RspuInstruction::StoreOutput { src: r, port: out_port_idx });
                        out_port_idx += 1;
                    }
                }
            }
        }
    }

    // Apply a bounded peephole pass.
    let instrs = peephole_optimize(&instrs);

    // Bounds check.
    if instrs.len() > MAX_INSTRUCTIONS {
        return Err(rspu_err(format!(
            "{} R-SPU instruction budget exceeded: {} instructions > {}.",
            crate::error_codes::ec(702),
            instrs.len(),
            MAX_INSTRUCTIONS,
        )));
    }

    Ok(RspuProgram {
        target: result.program.target.clone(),
        registers_used: regs.total_used,
        guards_used: guard_map_vec.len(),
        register_map: regs.entries.clone(),
        guard_map: guard_map_vec,
        instructions: instrs,
        certificate: None,
    })
}

// ---------------------------------------------------------------------------
// Guard allocation
// ---------------------------------------------------------------------------

type GuardAllocResult = (Vec<(String, GuardId)>, HashMap<String, GuardId>);

fn allocate_guards(
    netlist: Option<&TemporalNetlist>,
    registry: &crate::ecs::Registry,
    target: &TargetSpec,
) -> Result<GuardAllocResult, MirrError> {
    let mut entries = Vec::new();
    let mut map = HashMap::new();

    map.insert("always".to_string(), 0);
    entries.push(("always".to_string(), 0));
    let mut next_id: GuardId = 1;
    let max_guards = target.max_guards();

    if let Some(net) = netlist {
        let mut stack = Vec::new();
        for guard in net.guards.iter().rev() {
            stack.push(guard);
        }

        while let Some(guard) = stack.pop() {
            let name = guard_name(guard);
            if !map.contains_key(&name) {
                if next_id as usize >= max_guards {
                    println!("DEBUG GUARDS: {:?}", map.keys().collect::<Vec<_>>());
                    return Err(rspu_err(format!(
                        "{} R-SPU guard resource exhausted: {} guards > {}.",
                        crate::error_codes::ec(703),
                        next_id as usize + 1,
                        max_guards,
                    )));
                }
                map.insert(name.clone(), next_id);
                entries.push((name, next_id));
                next_id = next_id.saturating_add(1);
            }
            if let CompiledGuard::Complex(cx) = guard {
                for sub in cx.sub_guards.iter().rev() {
                    stack.push(sub);
                }
            }
        }
    }

    // Allocate hardware guards for any direct signal-based guards in reflexes
    for reflex_comp_opt in &registry.reflex_comps {
        if let Some(reflex_comp) = reflex_comp_opt {
            for &guard_id in &reflex_comp.guards {
                if let Some(name_comp) = &registry.names[guard_id.0 as usize] {
                    let gname = &name_comp.0;
                    if gname != "always" && !map.contains_key(gname) {
                        if next_id as usize >= max_guards {
                            return Err(rspu_err(format!(
                                "{} R-SPU guard resource exhausted: {} guards > {}.",
                                crate::error_codes::ec(703),
                                next_id as usize + 1,
                                max_guards,
                            )));
                        }
                        map.insert(gname.clone(), next_id);
                        entries.push((gname.clone(), next_id));
                        next_id = next_id.saturating_add(1);
                    }
                }
            }
        }
    }

    Ok((entries, map))
}

fn guard_name(guard: &CompiledGuard) -> String {
    match guard {
        CompiledGuard::ShiftRegister(sr) => sr.name.clone(),
        CompiledGuard::Counter(cg) => cg.name.clone(),
        CompiledGuard::Complex(cx) => cx.name.clone(),
        CompiledGuard::DynamicCounter(dc) => dc.name.clone(),
    }
}

// ---------------------------------------------------------------------------
// Temporal guard emission
// ---------------------------------------------------------------------------

const MAX_GUARD_DEPTH: usize = 64;

enum GuardWork<'a> {
    Process(&'a CompiledGuard),
    Combine { name: &'a str, sub_guards: &'a [CompiledGuard], is_or: bool },
}

fn emit_temporal_guards<'a>(
    guards: &'a [CompiledGuard],
    regs: &mut RegAllocResult,
    guard_map: &HashMap<String, GuardId>,
    instrs: &mut Vec<RspuInstruction>,
    registry: &crate::ecs::Registry,
) -> Result<(), MirrError> {
    let mut work: Vec<GuardWork<'_>> = Vec::with_capacity(MAX_GUARD_DEPTH);
    let mut emitted = HashSet::new();

    for guard in guards.iter().rev() {
        work.push(GuardWork::Process(guard));
    }

    let max_iterations = MAX_GUARD_DEPTH * 4;
    let mut visited = 0usize;
    let temp_start = regs.next_temp;

    while let Some(item) = work.pop() {
        visited += 1;
        if visited > max_iterations {
            return Err(rspu_err(format!(
                "{} R-SPU guard tree exceeds maximum iteration bound.",
                crate::error_codes::ec(706)
            )));
        }

        regs.next_temp = temp_start;

        match item {
            GuardWork::Process(guard) => {
                let name = guard_name(guard);
                if !emitted.insert(name) {
                    continue;
                }
                match guard {
                    CompiledGuard::ShiftRegister(sr) => {
                        let gid = guard_map[&sr.name];
                        let cond_reg = condition_to_reg(&sr.condition_kind, regs, instrs, registry)?;
                        instrs.push(RspuInstruction::SrInit {
                            guard: gid,
                            length: sr.delay_cycles as u32,
                            cond: cond_reg,
                        });
                        instrs.push(RspuInstruction::SrTick { guard: gid });
                        let dst_reg = regs
                            .map
                            .get(&sr.output_signal)
                            .copied()
                            .unwrap_or_else(|| regs.alloc_temp().unwrap_or(0));
                        instrs.push(RspuInstruction::SrQuery { dst: dst_reg, guard: gid });
                    }
                    CompiledGuard::Counter(cg) => {
                        let gid = guard_map[&cg.name];
                        let cond_reg = condition_to_reg(&cg.condition_kind, regs, instrs, registry)?;
                        instrs.push(RspuInstruction::CtrInit {
                            guard: gid,
                            target: cg.target_count,
                            cond: cond_reg,
                        });
                        instrs.push(RspuInstruction::CtrTick { guard: gid });
                        let dst_reg = regs
                            .map
                            .get(&cg.output_signal)
                            .copied()
                            .unwrap_or_else(|| regs.alloc_temp().unwrap_or(0));
                        instrs.push(RspuInstruction::CtrQuery { dst: dst_reg, guard: gid });
                    }
                    CompiledGuard::Complex(cx) => {
                        let is_or = matches!(
                            &cx.combination_logic,
                            crate::ast::expr::Expr::Binary {
                                op: crate::ast::types::BinaryOp::Or,
                                ..
                            }
                        );
                        work.push(GuardWork::Combine {
                            name: &cx.name,
                            sub_guards: &cx.sub_guards,
                            is_or,
                        });
                        for sub in cx.sub_guards.iter().rev() {
                            work.push(GuardWork::Process(sub));
                        }
                    }
                    CompiledGuard::DynamicCounter(dc) => {
                        let gid = guard_map[&dc.name];
                        let cond_reg = condition_to_reg(&dc.condition_kind, regs, instrs, registry)?;
                        instrs.push(RspuInstruction::CtrInit {
                            guard: gid,
                            target: dc.max_delay,
                            cond: cond_reg,
                        });
                        instrs.push(RspuInstruction::CtrTick { guard: gid });
                        let dst_reg = regs
                            .map
                            .get(&dc.output_signal)
                            .copied()
                            .unwrap_or_else(|| regs.alloc_temp().unwrap_or(0));
                        instrs.push(RspuInstruction::CtrQuery { dst: dst_reg, guard: gid });
                    }
                }
            }
            GuardWork::Combine { name, sub_guards, is_or } => {
                let gid = guard_map[name];
                if sub_guards.len() == 2 {
                    let a_gid = guard_map[&guard_name(&sub_guards[0])];
                    let b_gid = guard_map[&guard_name(&sub_guards[1])];
                    if is_or {
                        instrs.push(RspuInstruction::GuardOr { dst: gid, a: a_gid, b: b_gid });
                    } else {
                        instrs.push(RspuInstruction::GuardAnd { dst: gid, a: a_gid, b: b_gid });
                    }
                }
            }
        }
    }

    regs.next_temp = temp_start;
    Ok(())
}

// ---------------------------------------------------------------------------
// Reflex emission
// ---------------------------------------------------------------------------

fn emit_reflex(
    reflex: &crate::ecs::components::ReflexComponent,
    guard_map: &HashMap<String, GuardId>,
    compiled_guard_map: &HashMap<String, &CompiledGuard>,
    regs: &mut RegAllocResult,
    instrs: &mut Vec<RspuInstruction>,
    registry: &crate::ecs::Registry,
) -> Result<(), MirrError> {
    let guard_names: Vec<String> = reflex.guards.iter().filter_map(|&g| registry.names[g.0 as usize].as_ref().map(|n| n.0.clone())).collect();

    // 1. Resolve temporal_gid to the first guard (if any), otherwise always (0)
    let gid = if guard_names.is_empty() {
        0
    } else {
        guard_map.get(&guard_names[0]).copied().unwrap_or(0)
    };

    let temp_start = regs.next_temp;

    // 2. Conjunctor all subsequent guards in registers
    let mut cond_regs = Vec::new();
    if guard_names.len() > 1 {
        for gname in &guard_names[1..] {
            if let Some(guard) = compiled_guard_map.get(gname.as_str()) {
                let sig_name = guard.output_signal();
                let reg = regs.map.get(sig_name).copied().unwrap_or(0);
                cond_regs.push(reg);
            } else {
                let reg = regs.map.get(gname.as_str()).copied().unwrap_or(0);
                cond_regs.push(reg);
            }
        }
    }

    let mut acc_reg = None;
    for cond_reg in cond_regs {
        if let Some(acc) = acc_reg {
            let tmp = regs.alloc_temp().ok_or_else(|| {
                rspu_err("R-SPU temporary registers exhausted during reflex guard conjunction.")
            })?;
            instrs.push(RspuInstruction::Alu { op: AluOp::And, dst: tmp, a: acc, b: cond_reg });
            acc_reg = Some(tmp);
        } else {
            acc_reg = Some(cond_reg);
        }
    }

    let assignment_temp_start = regs.next_temp;

    // 3. Emit each assignment
    for &assignment_id in &reflex.assignments {
        if let Some(assignment) = &registry.assignment_comps[assignment_id.0 as usize] {
            regs.next_temp = assignment_temp_start;

            let target_name = registry.names[assignment.target.0 as usize].as_ref().map(|n| n.0.clone()).unwrap_or_default();
            let dst_reg = regs.map.get(&target_name).copied().unwrap_or(0);

            if let Some(acc) = acc_reg {
                // Check if the assigned value is a literal boolean `true`
                let is_bool_true = if let Some(lit) = &registry.literals[assignment.value.0 as usize] {
                    matches!(lit.0, crate::ast::types::LiteralValue::Bool(true))
                } else {
                    false
                };

                if is_bool_true {
                    // Highly optimized 1-instruction path: final = dst_reg | acc
                    let final_src_reg = regs.alloc_temp().ok_or_else(|| {
                        rspu_err(
                            "R-SPU temporary registers exhausted during reflex assignment evaluation.",
                        )
                    })?;
                    instrs.push(RspuInstruction::Alu {
                        op: AluOp::Or,
                        dst: final_src_reg,
                        a: dst_reg,
                        b: acc,
                    });
                    instrs.push(RspuInstruction::ReflexIf {
                        guard: gid,
                        dst: dst_reg,
                        src: final_src_reg,
                    });
                } else {
                    let src_reg = emit_expr(assignment.value, regs, instrs, registry)?;

                    // Cast acc to dst_reg's tag to satisfy strict ALU typing
                    let acc_cast = regs.alloc_temp().ok_or_else(|| {
                        rspu_err("R-SPU temporary registers exhausted during reflex assignment cast.")
                    })?;
                    let dst_tag =
                        crate::emit::rspu_helpers::get_signal_tag_byte(&target_name, registry);
                    instrs.push(RspuInstruction::Mov { dst: acc_cast, src: acc });
                    instrs.push(RspuInstruction::TagLoad { dst: acc_cast, tag: dst_tag });

                    // Highly optimized 3-instruction XOR-multiplexer path:
                    // tmp = dst_reg ^ src_reg
                    // tmp2 = tmp * acc_cast
                    // final = dst_reg ^ tmp2
                    let tmp = regs.alloc_temp().ok_or_else(|| {
                        rspu_err(
                            "R-SPU temporary registers exhausted during reflex assignment evaluation.",
                        )
                    })?;
                    instrs.push(RspuInstruction::Alu {
                        op: AluOp::Xor,
                        dst: tmp,
                        a: dst_reg,
                        b: src_reg,
                    });

                    let tmp2 = regs.alloc_temp().ok_or_else(|| {
                        rspu_err(
                            "R-SPU temporary registers exhausted during reflex assignment evaluation.",
                        )
                    })?;
                    instrs.push(RspuInstruction::Alu {
                        op: AluOp::Mul,
                        dst: tmp2,
                        a: tmp,
                        b: acc_cast,
                    });

                    let final_src_reg = regs.alloc_temp().ok_or_else(|| {
                        rspu_err(
                            "R-SPU temporary registers exhausted during reflex assignment evaluation.",
                        )
                    })?;
                    instrs.push(RspuInstruction::Alu {
                        op: AluOp::Xor,
                        dst: final_src_reg,
                        a: dst_reg,
                        b: tmp2,
                    });

                    instrs.push(RspuInstruction::ReflexIf {
                        guard: gid,
                        dst: dst_reg,
                        src: final_src_reg,
                    });
                }
            } else {
                // Normal assignment gated purely on temporal hardware guard
                let src_reg = emit_expr(assignment.value, regs, instrs, registry)?;
                instrs.push(RspuInstruction::ReflexIf { guard: gid, dst: dst_reg, src: src_reg });
            }
        }
    }

    regs.next_temp = temp_start;
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

pub(crate) fn rspu_err(msg: impl Into<String>) -> MirrError {
    MirrError::RspuError { message: msg.into(), span: None }
}

