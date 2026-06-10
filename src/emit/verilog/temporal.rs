//! Temporal guard emission: shift-register, counter, dynamic-counter,
//! reflex assignments, and condition expressions.

#![forbid(unsafe_code)]

use crate::ast::program::Module;
use crate::ast::types::BinaryOp;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};

use super::MAX_SR_STAGES_INLINE;

pub(super) fn emit_temporal_logic(module: &Module, netlist: &TemporalNetlist, out: &mut String) {
    out.push_str("  // ── Temporal Guards ──\n\n");

    // Declare registers for ALL prev() back-references found in the module
    let mut seen_prevs = std::collections::HashSet::new();
    for reflex in &module.reflexes {
        for assignment in &reflex.assignments {
            // Helper to find Prev nodes in expressions
            fn collect_prevs(
                expr: &crate::ast::Expr,
                seen: &mut std::collections::HashSet<(String, u64)>,
            ) {
                match expr {
                    crate::ast::Expr::Prev { signal, delay } => {
                        seen.insert((signal.clone(), *delay));
                    }
                    crate::ast::Expr::Unary { operand, .. } => collect_prevs(operand, seen),
                    crate::ast::Expr::Binary { left, right, .. } => {
                        collect_prevs(left, seen);
                        collect_prevs(right, seen);
                    }
                    crate::ast::Expr::ArrayIndex { array, index } => {
                        collect_prevs(array, seen);
                        collect_prevs(index, seen);
                    }
                    _ => {}
                }
            }
            collect_prevs(&assignment.value, &mut seen_prevs);
        }
    }

    for (sig_name, delay) in seen_prevs {
        // Find the original signal to get its type
        if let Some(sig) = module.signals.iter().find(|s| s.name == sig_name) {
            let type_str = crate::emit::sv_type(&sig.ty.signal_type());
            out.push_str(&format!("  {} {}_d{};\n", type_str, sig_name, delay));
        }
    }
    out.push('\n');

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
    hls_result: Option<&crate::hls::HlsResult>,
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

    // Group reflexes by their target signals to prevent multiple-driver conflicts.
    let mut signal_to_reflexes: std::collections::HashMap<
        String,
        Vec<&crate::ast::program::Reflex>,
    > = std::collections::HashMap::new();

    for r in &module.reflexes {
        for a in &r.assignments {
            signal_to_reflexes.entry(a.target.clone()).or_default().push(r);
        }
    }

    if let Some(hls) = hls_result {
        out.push_str("  // ── HLS Finite State Machine ──\n");
        out.push_str("  logic [31:0] hls_state;\n");
        out.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
        out.push_str("    if (!rst_n) begin\n");
        out.push_str("      hls_state <= 0;\n");

        let mut signals: Vec<String> = signal_to_reflexes.keys().cloned().collect();
        signals.sort();
        for sig in &signals {
            out.push_str(&format!("      {} <= '0;\n", sig));
        }

        out.push_str("    end else begin\n");
        out.push_str("      case (hls_state)\n");

        let mut target_to_cycle: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();
        let mut max_cycle = 0;
        for op in &hls.schedule {
            if let Some(target) = hls.target_signals.get(&op.op_id) {
                target_to_cycle.insert(target.clone(), op.earliest);
                if op.earliest > max_cycle {
                    max_cycle = op.earliest;
                }
            }
        }

        // We want to group by cycle for the FSM states
        let mut cycle_to_signals: std::collections::HashMap<u32, Vec<String>> =
            std::collections::HashMap::new();
        for sig in &signals {
            let cycle = target_to_cycle.get(sig).copied().unwrap_or(0);
            cycle_to_signals.entry(cycle).or_default().push(sig.clone());
        }

        for cycle in 0..=max_cycle {
            out.push_str(&format!("        {}: begin\n", cycle));
            if let Some(sigs) = cycle_to_signals.get(&cycle) {
                for sig in sigs {
                    let refs = &signal_to_reflexes[sig];
                    for r in refs {
                        let guard_cond = if r.guard_names.len() == 1 {
                            format!("{}_out", r.guard_names[0])
                        } else {
                            let parts: Vec<String> =
                                r.guard_names.iter().map(|g| format!("{g}_out")).collect();
                            parts.join(" && ")
                        };
                        for a in &r.assignments {
                            if a.target == *sig {
                                out.push_str(&format!(
                                    "          if ({}) {} <= {};\n",
                                    guard_cond,
                                    sig,
                                    super::emit_expr_inline(&a.value),
                                ));
                            }
                        }
                    }
                }
            }
            if cycle == max_cycle {
                out.push_str("          hls_state <= 0;\n");
            } else {
                out.push_str(&format!("          hls_state <= {};\n", cycle + 1));
            }
            out.push_str("        end\n");
        }
        out.push_str("      endcase\n");
        out.push_str("    end\n");
        out.push_str("  end\n\n");

        // Emit FIFOs if there are any
        if !hls.fifos.is_empty() {
            out.push_str("  // ── HLS FIFOs ──\n");
            for fifo in &hls.fifos {
                out.push_str(&format!(
                    "  // FIFO: {} (depth: {}, width: {})\n",
                    fifo.name, fifo.depth, fifo.elem_width
                ));
                out.push_str(&format!(
                    "  logic [{}:0] {}_buffer [0:{}];\n",
                    fifo.elem_width.saturating_sub(1),
                    fifo.name,
                    fifo.depth.saturating_sub(1)
                ));
                out.push_str(&format!("  logic [31:0] {}_head, {}_tail;\n", fifo.name, fifo.name));
            }
            out.push('\n');
        }
    } else {
        // Sort signals by name for deterministic emission.
        let mut signals: Vec<String> = signal_to_reflexes.keys().cloned().collect();
        signals.sort();

        for sig in signals {
            let refs = &signal_to_reflexes[&sig];

            // Emit DSP synthesis attribute if ANY reflex for this signal contains a multiply.
            if let Some(attr) = dsp_attr {
                let has_dsp = refs.iter().any(|r| dsp_reflexes.contains(&r.name));
                if has_dsp {
                    out.push_str(&format!("  {attr}\n"));
                }
            }

            out.push_str(&format!("  // Unified Reflex Block for: {sig}\n"));
            out.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
            out.push_str("    if (!rst_n) begin\n");
            out.push_str(&format!("      {} <= '0;\n", sig));
            out.push_str("    end else begin\n");

            // Priority-ordered assignments: later reflexes in the module override earlier ones.
            for r in refs {
                let guard_cond = if r.guard_names.len() == 1 {
                    format!("{}_out", r.guard_names[0])
                } else {
                    let parts: Vec<String> =
                        r.guard_names.iter().map(|g| format!("{g}_out")).collect();
                    parts.join(" && ")
                };

                // Find the assignment to THIS signal in this reflex.
                for a in &r.assignments {
                    if a.target == sig {
                        out.push_str(&format!(
                            "      if ({}) {} <= {};\n",
                            guard_cond,
                            sig,
                            super::emit_expr_inline(&a.value),
                        ));
                    }
                }
            }
            out.push_str("    end\n");
            out.push_str("  end\n\n");
        }
    }
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
