//! Temporal guard emission: shift-register, counter, dynamic-counter,
//! reflex assignments, and condition expressions.

#![forbid(unsafe_code)]

use crate::ast::program::Module;
use crate::ast::types::BinaryOp;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};

use super::MAX_SR_STAGES_INLINE;

pub(super) fn emit_temporal_logic(netlist: &TemporalNetlist, out: &mut String) {
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
                    super::emit_expr_inline(&cx.combination_logic),
                ));
            }
            CompiledGuard::DynamicCounter(dc) => {
                emit_dynamic_counter_guard(dc, out);
            }
        }
    }
}

fn emit_shift_register_guard(
    sr: &crate::temporal::low_level_ir::ShiftRegisterGuard,
    out: &mut String,
) {
    let cond_desc = sr.condition_kind.describe();
    // Special case: 1-cycle guard is purely combinational.
    if sr.delay_cycles == 1 {
        out.push_str(&format!(
            "  // Guard: {} — {} for 1 cycle (combinational)\n",
            sr.name, cond_desc
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
        super::emit_expr_inline(&dc.delay_expr),
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

pub(super) fn emit_reflex_logic(
    module: &Module,
    dsp_reflexes: &std::collections::HashSet<String>,
    dsp_attr: Option<&str>,
    out: &mut String,
) {
    if module.reflexes.is_empty() {
        return;
    }

    out.push_str("  // ── Reflex Assignments ──\n\n");

    // Declare guard _out wires used in reflex combinational logic.
    let mut declared_outs = std::collections::HashSet::new();
    for r in &module.reflexes {
        for g in &r.guard_names {
            let wire_name = format!("{g}_out");
            if declared_outs.insert(wire_name.clone()) {
                out.push_str(&format!("  logic {};\n", wire_name));
            }
        }
    }
    out.push('\n');

    for r in &module.reflexes {
        if let Some(ref origin) = r.origin {
            out.push_str(&format!("  // Pattern: {origin}\n"));
        }
        out.push_str(&format!("  // Reflex: {}\n", r.name));
        // Emit DSP synthesis attribute if this reflex contains a multiply.
        if let Some(attr) = dsp_attr {
            if dsp_reflexes.contains(&r.name) {
                out.push_str(&format!("  {attr}\n"));
            }
        }
        out.push_str("  always_comb begin\n");
        // Default assignments to prevent latch inference.
        for a in &r.assignments {
            out.push_str(&format!("    {} = '0;\n", a.target));
        }
        for a in &r.assignments {
            let guard_cond = if r.guard_names.len() == 1 {
                format!("{}_out", r.guard_names[0])
            } else {
                let parts: Vec<String> = r.guard_names.iter().map(|g| format!("{g}_out")).collect();
                parts.join(" && ")
            };
            out.push_str(&format!(
                "    if ({}) {} = {};\n",
                guard_cond,
                a.target,
                super::emit_expr_inline(&a.value),
            ));
        }
        out.push_str("  end\n\n");
    }
}

/// Emit a ConditionKind as an inline SystemVerilog expression.
fn emit_condition_expr(ck: &crate::temporal::low_level_ir::ConditionKind) -> String {
    use crate::temporal::low_level_ir::ConditionKind;
    match ck {
        ConditionKind::SimpleSignal(s) => s.clone(),
        ConditionKind::NegatedSignal(s) => format!("!{s}"),
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
    }
}
