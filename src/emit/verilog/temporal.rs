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
                    super::emit_expr_ast_legacy(&cx.combination_logic),
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
                    super::emit_expr_ast_legacy(&cx.combination_logic),
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

    if let Some(_hls) = hls {
        // HLS logic remains unchanged for now, but we'd need an ECS-native way to handle it later.
        out.push_str("  // HLS logic (AST-based, pending ECS migration)\n");
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
        super::emit_expr_ast_legacy(&dc.delay_expr),
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
