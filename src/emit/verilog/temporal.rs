//! Temporal guard emission: shift-register, counter, dynamic-counter,
//! reflex assignments, and condition expressions.

#![forbid(unsafe_code)]

use crate::ast::types::BinaryOp;
use crate::emit::verilog::emit_source_comment;
use crate::span::FileTable;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};

use super::MAX_SR_STAGES_INLINE;

pub(super) fn emit_temporal_logic_standalone(
    netlist: &TemporalNetlist,
    _ft: &FileTable,
    out: &mut String,
) {
    out.push_str("  // ── Temporal Guards ──\n\n");

    for guard in &netlist.guards {
        match guard {
            CompiledGuard::ShiftRegister(sr) => {
                emit_shift_register_guard(sr, out);
            }
            CompiledGuard::Counter(cg) => {
                emit_counter_guard(cg, out);
            }
            CompiledGuard::Complex(cx) => {
                out.push_str(&format!("  // Complex guard: {} (sub-guards combined)\n", cx.name));
                out.push_str(&format!("  logic {};\n", cx.output_signal));
                out.push_str(&format!(
                    "  assign {} = {};\n\n",
                    cx.output_signal,
                    emit_logic_expr(&cx.combination_logic),
                ));
            }
            CompiledGuard::DynamicCounter(dc) => {
                emit_dynamic_counter_guard(dc, out);
            }
        }
    }
}

pub(super) fn emit_temporal_logic_ecs(
    registry: &crate::ecs::Registry,
    netlist: &TemporalNetlist,
    ft: &FileTable,
    out: &mut String,
) {
    out.push_str("  // ── Temporal Guards ──\n\n");

    // Declare registers for ALL prev() back-references found in the module
    let mut seen_prevs = std::collections::HashSet::new();
    for i in 0..registry.reflex_comps.len() {
        if let Some(reflex) = &registry.reflex_comps[i] {
            for asgn_ent in &reflex.assignments {
                if let Some(asgn) = &registry.assignment_comps[asgn_ent.0 as usize] {
                    collect_prevs_ecs(asgn.value, registry, &mut seen_prevs);
                }
            }
        }
    }

    let mut sorted_prevs: Vec<_> = seen_prevs.into_iter().collect();
    sorted_prevs.sort();

    for (sig_ent, delay) in sorted_prevs {
        if let Some(name_comp) = &registry.names[sig_ent.0 as usize] {
            if let Some(type_comp) = &registry.types[sig_ent.0 as usize] {
                let sig_name = &name_comp.0;
                let type_str = crate::emit::sv_type(&type_comp.0.signal_type());
                out.push_str(&format!("  {} {}_d{};\n", type_str, sig_name, delay));
            }
        }
    }
    out.push('\n');

    for guard in &netlist.guards {
        // Find the guard entity to get its span
        let mut span = None;
        for i in 0..registry.names.len() {
            if let (Some(name_comp), Some(kind_comp)) = (&registry.names[i], &registry.kinds[i]) {
                if name_comp.0 == guard.name() && kind_comp.0 == crate::ecs::EntityKind::GUARD {
                    span = registry.spans[i].as_ref().map(|s| &s.0);
                    break;
                }
            }
        }

        match guard {
            CompiledGuard::ShiftRegister(sr) => {
                emit_source_comment(span, ft, out);
                emit_shift_register_guard(sr, out);
            }
            CompiledGuard::Counter(cg) => {
                emit_source_comment(span, ft, out);
                emit_counter_guard(cg, out);
            }
            CompiledGuard::Complex(cx) => {
                emit_source_comment(span, ft, out);
                out.push_str(&format!("  // Complex guard: {} (sub-guards combined)\n", cx.name));
                out.push_str(&format!("  logic {};\n", cx.output_signal));
                out.push_str(&format!(
                    "  assign {} = {};\n\n",
                    cx.output_signal,
                    emit_logic_expr(&cx.combination_logic),
                ));
            }
            CompiledGuard::DynamicCounter(dc) => {
                emit_source_comment(span, ft, out);
                emit_dynamic_counter_guard(dc, out);
            }
        }
    }
}

fn collect_prevs_ecs(
    root: crate::ecs::EntityId,
    registry: &crate::ecs::Registry,
    seen: &mut std::collections::HashSet<(crate::ecs::EntityId, u64)>,
) {
    let mut stack = Vec::new();
    stack.push(root);
    let mut visited = 0;
    while let Some(id) = stack.pop() {
        visited += 1;
        if visited > 512 {
            break;
        }
        let idx = id.0 as usize;
        if let Some(p) = &registry.prev_ops[idx] {
            seen.insert((p.signal, p.delay));
            stack.push(p.signal);
        } else if let Some(b) = &registry.binary_ops[idx] {
            stack.push(b.left);
            stack.push(b.right);
        } else if let Some(u) = &registry.unary_ops[idx] {
            stack.push(u.operand);
        } else if let Some(m) = &registry.muxes[idx] {
            stack.push(m.select);
            stack.push(m.true_val);
            stack.push(m.false_val);
        }
    }
}

pub(super) fn emit_reflex_logic_ecs(
    registry: &crate::ecs::Registry,
    dsp_reflexes: &std::collections::HashSet<String>,
    dsp_attr: Option<&str>,
    hls: Option<&crate::hls::HlsResult>,
    ft: &FileTable,
    out: &mut String,
) {
    // Group assignments by target signal
    let mut signal_to_reflexes: std::collections::HashMap<String, Vec<usize>> =
        std::collections::HashMap::new();

    // Track guard names used in reflexes to declare their _out wires
    let mut guard_names_used: std::collections::HashSet<String> = std::collections::HashSet::new();

    for i in 0..registry.reflex_comps.len() {
        if let Some(reflex) = &registry.reflex_comps[i] {
            for g_ent in &reflex.guards {
                if let Some(name_comp) = &registry.names[g_ent.0 as usize] {
                    if name_comp.0 != "always" {
                        guard_names_used.insert(name_comp.0.clone());
                    }
                }
            }
            for asgn_ent in &reflex.assignments {
                if let Some(asgn) = &registry.assignment_comps[asgn_ent.0 as usize] {
                    if let Some(target_name) = &registry.names[asgn.target.0 as usize] {
                        signal_to_reflexes.entry(target_name.0.clone()).or_default().push(i);
                    }
                }
            }
        }
    }

    if signal_to_reflexes.is_empty() && guard_names_used.is_empty() {
        return;
    }

    out.push_str("  // ── Reflex Assignments ──\n\n");

    // Declare wires for guard outputs used in reflexes
    let mut sorted_guards: Vec<_> = guard_names_used.into_iter().collect();
    sorted_guards.sort();
    for gname in sorted_guards {
        out.push_str(&format!("  logic {}_out;\n", gname));
    }
    if !signal_to_reflexes.is_empty() {
        out.push('\n');
    }

    out.push_str("  // ── Reflex Signal Drivers ──\n\n");

    if let Some(hls_res) = hls {
        emit_hls_logic_ecs(registry, hls_res, ft, out);
    } else {
        // Sort signals by name for deterministic emission.
        let mut signals: Vec<String> = signal_to_reflexes.keys().cloned().collect();
        signals.sort();

        for sig_name in signals {
            let reflex_indices = &signal_to_reflexes[&sig_name];

            // Emit DSP synthesis attribute if ANY reflex for this signal contains a multiply.
            if let Some(attr) = dsp_attr {
                let mut has_dsp = false;
                for &ri in reflex_indices {
                    if let Some(name_comp) = &registry.names[ri] {
                        if dsp_reflexes.contains(&name_comp.0) {
                            has_dsp = true;
                            break;
                        }
                    }
                }
                if has_dsp {
                    out.push_str(&format!("  {attr}\n"));
                }
            }

            out.push_str(&format!("  // Unified Reflex Block for: {sig_name}\n"));
            out.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
            out.push_str("    if (!rst_n) begin\n");
            out.push_str(&format!("      {} <= '0;\n", sig_name));
            out.push_str("    end else begin\n");

            // Priority-ordered assignments
            for &ri in reflex_indices {
                if let Some(reflex) = &registry.reflex_comps[ri] {
                    let mut guard_parts = Vec::new();
                    for g_ent in &reflex.guards {
                        if let Some(g_name) = &registry.names[g_ent.0 as usize] {
                            guard_parts.push(format!("{}_out", g_name.0));
                        }
                    }
                    let guard_cond = if guard_parts.is_empty() {
                        "1'b1".to_string()
                    } else {
                        guard_parts.join(" && ")
                    };

                    for asgn_ent in &reflex.assignments {
                        if let Some(asgn) = &registry.assignment_comps[asgn_ent.0 as usize] {
                            if let Some(target_name) = &registry.names[asgn.target.0 as usize] {
                                if target_name.0 == sig_name {
                                    let span = registry.spans[ri].as_ref().map(|s| &s.0);
                                    emit_source_comment(span, ft, out);
                                    out.push_str(&format!(
                                        "      if ({}) {} <= {};\n",
                                        guard_cond,
                                        sig_name,
                                        super::emit_expr_inline(asgn.value, registry),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            out.push_str("    end\n");
            out.push_str("  end\n\n");
        }
    }
}

fn emit_hls_logic_ecs(
    _registry: &crate::ecs::Registry,
    hls: &crate::hls::HlsResult,
    _ft: &FileTable,
    out: &mut String,
) {
    out.push_str("  // ── HLS Finite State Machine (MEGA-12) ──\n\n");

    // 1. Declare hls_state register.
    out.push_str("  logic [31:0] hls_state;\n\n");

    // 2. Declare intermediate wires for every HLS operation.
    let mut sorted_ops = hls.ops.clone();
    sorted_ops.sort_by_key(|o| o.op_id);
    for op in &sorted_ops {
        out.push_str(&format!(
            "  logic [{}:0] op_{}_res;\n",
            op.width.saturating_sub(1),
            op.op_id
        ));
    }
    out.push('\n');

    // 3. Group operations by cycle.
    let mut cycle_to_ops: std::collections::HashMap<u32, Vec<usize>> =
        std::collections::HashMap::new();
    let mut max_cycle = 0;
    for (i, op_sched) in hls.schedule.iter().enumerate() {
        cycle_to_ops.entry(op_sched.earliest).or_default().push(i);
        if op_sched.earliest > max_cycle {
            max_cycle = op_sched.earliest;
        }
    }

    // Sort cycles for deterministic emission.
    let mut sorted_cycles: Vec<u32> = cycle_to_ops.keys().cloned().collect();
    sorted_cycles.sort();

    out.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    out.push_str("    if (!rst_n) begin\n");
    out.push_str("      hls_state <= 0;\n");
    // Initialize target signals.
    let mut sorted_targets: Vec<_> = hls.target_signals.values().collect();
    sorted_targets.sort();
    for target in sorted_targets {
        out.push_str(&format!("      {} <= '0;\n", target));
    }
    out.push_str("    end else begin\n");
    out.push_str("      case (hls_state)\n");

    for cycle in sorted_cycles {
        out.push_str(&format!("        {}: begin\n", cycle));
        let op_indices = &cycle_to_ops[&cycle];
        for &idx in op_indices {
            let op_sched = &hls.schedule[idx];
            let op = &hls.ops[op_sched.op_id as usize];

            let op_str = match op.kind {
                crate::hls::ResourceKind::Add => "+",
                crate::hls::ResourceKind::Sub => "-",
                crate::hls::ResourceKind::Mul => "*",
                crate::hls::ResourceKind::And => "&",
                crate::hls::ResourceKind::Or => "|",
                crate::hls::ResourceKind::Xor => "^",
                crate::hls::ResourceKind::Eq => "==",
                crate::hls::ResourceKind::Ne => "!=",
                crate::hls::ResourceKind::Lt => "<",
                crate::hls::ResourceKind::Le => "<=",
                crate::hls::ResourceKind::Gt => ">",
                crate::hls::ResourceKind::Ge => ">=",
                crate::hls::ResourceKind::Shl => "<<",
                crate::hls::ResourceKind::Shr => ">>",
                crate::hls::ResourceKind::Not => "!",
                crate::hls::ResourceKind::Negate => "-",
            };

            let mut operands_str = Vec::new();
            for operand in &op.operands {
                match operand {
                    crate::hls::HlsOperand::Op(id) => operands_str.push(format!("op_{}_res", id)),
                    crate::hls::HlsOperand::Signal(s) => operands_str.push(s.clone()),
                    crate::hls::HlsOperand::Literal(l) => operands_str.push(l.clone()),
                }
            }

            if op.kind == crate::hls::ResourceKind::Not
                || op.kind == crate::hls::ResourceKind::Negate
            {
                out.push_str(&format!(
                    "          op_{}_res <= {}({});\n",
                    op.op_id, op_str, operands_str[0]
                ));
            } else {
                out.push_str(&format!(
                    "          op_{}_res <= ({} {} {});\n",
                    op.op_id, operands_str[0], op_str, operands_str[1]
                ));
            }

            // If this operation is a final assignment to a target signal.
            if let Some(target_name) = hls.target_signals.get(&op.op_id) {
                out.push_str(&format!("          {} <= op_{}_res;\n", target_name, op.op_id));
            }
        }

        // State transition.
        if cycle < max_cycle {
            out.push_str(&format!("          hls_state <= {};\n", cycle + 1));
        } else {
            out.push_str("          hls_state <= 0;\n");
        }
        out.push_str("        end\n");
    }

    out.push_str("        default: hls_state <= 0;\n");
    out.push_str("      endcase\n");
    out.push_str("    end\n");
    out.push_str("  end\n\n");
}

fn emit_shift_register_guard(
    sr: &crate::temporal::low_level_ir::ShiftRegisterGuard,
    out: &mut String,
) {
    let cond_desc = sr.condition_kind.describe();
    // Special case: 0 or 1-cycle guard is purely combinational.
    if sr.delay_cycles <= 1 {
        out.push_str(&format!(
            "  // Guard: {} — {} for {} cycle (combinational)\n",
            sr.name, cond_desc, sr.delay_cycles
        ));
        out.push_str(&format!("  logic {}_cond;\n", sr.name));
        out.push_str(&format!(
            "  assign {}_cond = {};\n",
            sr.name,
            emit_condition_expr(&sr.condition_kind),
        ));
        out.push_str(&format!("  assign {} = {}_cond;\n\n", sr.output_signal, sr.name));
        return;
    }

    out.push_str(&format!(
        "  // Guard: {} — {} for {} cycles\n",
        sr.name, cond_desc, sr.delay_cycles
    ));

    let stage_count = sr.delay_cycles.min(MAX_SR_STAGES_INLINE);

    // Declare the shift register.
    out.push_str(&format!("  logic [{}:0] {}_sr;\n", stage_count.saturating_sub(1), sr.name,));

    // Condition wire.
    out.push_str(&format!("  logic {}_cond;\n", sr.name));
    out.push_str(&format!(
        "  assign {}_cond = {};\n",
        sr.name,
        emit_condition_expr(&sr.condition_kind),
    ));

    // Shift register always_ff block.
    out.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    out.push_str(&format!("    if (!rst_n)\n      {}_sr <= '0;\n", sr.name));
    out.push_str(&format!(
        "    else\n      {0}_sr <= {{{0}_cond, {0}_sr[{1}:1]}};\n",
        sr.name,
        stage_count.saturating_sub(1),
    ));
    out.push_str("  end\n");

    // Output: guard fires when all stages are 1.
    out.push_str(&format!("  assign {} = &{}_sr;\n\n", sr.output_signal, sr.name,));
}

fn emit_counter_guard(cg: &crate::temporal::low_level_ir::CounterGuard, out: &mut String) {
    let cond_desc = cg.condition_kind.describe();
    let width = cg.counter_width();
    out.push_str(&format!(
        "  // Guard: {} — {} for {} cycles (counter)\n",
        cg.name, cond_desc, cg.target_count
    ));

    // Counter register.
    out.push_str(&format!("  logic [{}:0] {};\n", width.saturating_sub(1), cg.counter_signal,));

    // Condition wire.
    out.push_str(&format!("  logic {}_cond;\n", cg.name));
    out.push_str(&format!(
        "  assign {}_cond = {};\n",
        cg.name,
        emit_condition_expr(&cg.condition_kind),
    ));

    // Counter always_ff block.
    out.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    out.push_str(&format!("    if (!rst_n)\n      {} <= '0;\n", cg.counter_signal));
    out.push_str(&format!("    else if (!{}_cond)\n      {} <= '0;\n", cg.name, cg.counter_signal));
    out.push_str(&format!(
        "    else if ({0} < {1})\n      {0} <= {0} + 1;\n",
        cg.counter_signal, cg.target_count,
    ));
    out.push_str("  end\n");

    // Output: guard fires when counter reaches target.
    out.push_str(&format!(
        "  assign {} = ({} >= {});\n\n",
        cg.output_signal, cg.counter_signal, cg.target_count,
    ));
}

fn emit_dynamic_counter_guard(
    dc: &crate::temporal::low_level_ir::DynamicCounterGuard,
    out: &mut String,
) {
    let cond_desc = dc.condition_kind.describe();
    let width = dc.counter_width();
    out.push_str(&format!(
        "  // Guard: {} — {} for dynamic delay (max {} cycles)\n",
        dc.name, cond_desc, dc.max_delay
    ));

    // Counter register.
    out.push_str(&format!("  logic [{}:0] {};\n", width.saturating_sub(1), dc.counter_signal));

    // Dynamic target wire.
    let target_signal = format!("{}_target", dc.name);
    out.push_str(&format!("  logic [{}:0] {};\n", width.saturating_sub(1), target_signal));
    out.push_str(&format!(
        "  assign {} = {};\n",
        target_signal,
        dc.delay_expr,
    ));

    // Condition wire.
    out.push_str(&format!("  logic {}_cond;\n", dc.name));
    out.push_str(&format!(
        "  assign {}_cond = {};\n",
        dc.name,
        emit_condition_expr(&dc.condition_kind),
    ));

    // Counter always_ff block.
    out.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    out.push_str(&format!("    if (!rst_n)\n      {} <= '0;\n", dc.counter_signal));
    out.push_str(&format!("    else if (!{}_cond)\n      {} <= '0;\n", dc.name, dc.counter_signal));
    out.push_str(&format!(
        "    else if ({0} < {1})\n      {0} <= {0} + 1;\n",
        dc.counter_signal, target_signal
    ));
    out.push_str("  end\n");

    // Output: guard fires when counter reaches dynamic target.
    out.push_str(&format!(
        "  assign {} = ({} >= {});\n\n",
        dc.output_signal, dc.counter_signal, target_signal
    ));
}

/// Emit a ConditionKind as an inline SystemVerilog expression.
fn emit_condition_expr(ck: &crate::temporal::low_level_ir::ConditionKind) -> String {
    use crate::temporal::low_level_ir::ConditionKind;
    match ck {
        ConditionKind::SimpleSignal(s) => s.clone(),
        ConditionKind::NegatedSignal(s) => format!("!{s}"),
        ConditionKind::PrevSignal { signal, .. } => {
            // For now, we return the base signal; the temporal compiler
            // handles the extra delay by increasing the SR/Counter depth.
            signal.clone()
        }
        ConditionKind::Comparison { signal, op, value } => {
            let op_str = match op {
                BinaryOp::Eq => "==",
                BinaryOp::Ne => "!=",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                _ => "??",
            };
            let val_str = match value {
                crate::ast::types::LiteralValue::Integer(n) => format!("{n}"),
                crate::ast::types::LiteralValue::Bool(b) => {
                    if *b {
                        "1'b1".to_string()
                    } else {
                        "1'b0".to_string()
                    }
                }
            };
            format!("({signal} {op_str} {val_str})")
        }
        ConditionKind::AlwaysTrue => "1'b1".to_string(),
    }
}

/// Emit a LogicExpr as an inline SystemVerilog expression.
fn emit_logic_expr(le: &crate::temporal::low_level_ir::LogicExpr) -> String {
    use crate::temporal::low_level_ir::LogicExpr;
    match le {
        LogicExpr::Signal(s) => s.clone(),
        LogicExpr::And(l, r) => format!("({} && {})", emit_logic_expr(l), emit_logic_expr(r)),
        LogicExpr::Or(l, r) => format!("({} || {})", emit_logic_expr(l), emit_logic_expr(r)),
    }
}
