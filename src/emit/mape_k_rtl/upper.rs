//! Plan, Execute, Knowledge, and Top-level block emitters.

#![forbid(unsafe_code)]

use super::{MAX_RTL_ACTIONS, MAX_RTL_KNOWLEDGE_DEPTH, MAX_RTL_SIGNALS};
use crate::emit::mape_k_rtl::lower::bit_width;
use crate::mape_k::planner::{AdaptationAction, TriggerCondition};
use crate::mape_k::SimConfig;

// ---------------------------------------------------------------------------
// Plan block
// ---------------------------------------------------------------------------

/// Emit `mirr_plan`: action lookup table, priority-based selection.
pub(super) fn emit_plan_block(config: &SimConfig) -> String {
    let n_prop = config.properties.len();
    let n_act = config.action_table.len();
    let act_w = bit_width(n_act);

    let mut sv = String::with_capacity(1024);
    sv.push_str("// --- Plan (P) ----------------------------------------------\n");
    sv.push_str("module mirr_plan #(\n");
    sv.push_str(&format!("  parameter N_PROPERTIES = {n_prop},\n"));
    sv.push_str(&format!("  parameter N_ACTIONS    = {n_act}\n"));
    sv.push_str(") (\n");
    sv.push_str("  input  logic clk,\n");
    sv.push_str("  input  logic rst_n,\n");
    sv.push_str(&format!(
        "  input  logic [{}:0] violation_vec,\n",
        n_prop.max(1).saturating_sub(1)
    ));
    sv.push_str(&format!(
        "  output logic [{}:0] selected_action_idx,\n",
        act_w.saturating_sub(1).max(0)
    ));
    sv.push_str("  output logic        action_valid\n");
    sv.push_str(");\n\n");

    // Encode action table as constants: trigger_idx, priority, trigger_on.
    sv.push_str("  logic [7:0] best_priority;\n");
    sv.push_str(&format!("  logic [{}:0] best_idx;\n", act_w.saturating_sub(1).max(0)));
    sv.push_str("  logic        found;\n\n");

    sv.push_str("  always_comb begin\n");
    sv.push_str("    best_priority = 8'd0;\n");
    sv.push_str(&format!("    best_idx      = {}'d0;\n", act_w.max(1)));
    sv.push_str("    found         = 1'b0;\n\n");

    for (ai, entry) in config.action_table.iter().enumerate().take(MAX_RTL_ACTIONS) {
        let ti = entry.trigger_property_idx;
        let pri = entry.priority;
        let trigger_cond = match entry.trigger_on {
            TriggerCondition::OnViolation => format!("violation_vec[{ti}]"),
            TriggerCondition::OnSatisfaction => format!("!violation_vec[{ti}]"),
        };
        sv.push_str(&format!("    // action[{ai}]: prop={ti} pri={pri}\n"));
        sv.push_str(&format!(
            "    if ({trigger_cond} && (8'd{pri} > best_priority || !found)) begin\n"
        ));
        sv.push_str(&format!("      best_priority = 8'd{pri};\n"));
        sv.push_str(&format!("      best_idx      = {}'d{ai};\n", act_w.max(1)));
        sv.push_str("      found         = 1'b1;\n");
        sv.push_str("    end\n");
    }

    sv.push_str("  end\n\n");

    // Register the selection.
    sv.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    sv.push_str("    if (!rst_n) begin\n");
    sv.push_str(&format!("      selected_action_idx <= {}'d0;\n", act_w.max(1)));
    sv.push_str("      action_valid        <= 1'b0;\n");
    sv.push_str("    end else begin\n");
    sv.push_str("      selected_action_idx <= best_idx;\n");
    sv.push_str("      action_valid        <= found;\n");
    sv.push_str("    end\n");
    sv.push_str("  end\n\n");
    sv.push_str("endmodule\n\n");
    sv
}

// ---------------------------------------------------------------------------
// Execute block
// ---------------------------------------------------------------------------

/// Emit `mirr_execute`: action dispatch, emergency latch.
pub(super) fn emit_execute_block(config: &SimConfig) -> String {
    let n_act = config.action_table.len();
    let n_sig = config.sensors.len();
    let act_w = bit_width(n_act);

    let mut sv = String::with_capacity(1024);
    sv.push_str("// --- Execute (E) -------------------------------------------\n");
    sv.push_str("module mirr_execute #(\n");
    sv.push_str(&format!("  parameter N_SIGNALS = {n_sig},\n"));
    sv.push_str(&format!("  parameter N_ACTIONS = {n_act}\n"));
    sv.push_str(") (\n");
    sv.push_str("  input  logic clk,\n");
    sv.push_str("  input  logic rst_n,\n");
    sv.push_str(&format!(
        "  input  logic [{}:0] selected_action_idx,\n",
        act_w.saturating_sub(1).max(0)
    ));
    sv.push_str("  input  logic        action_valid,\n");
    sv.push_str("  output logic [N_SIGNALS-1:0][31:0] signal_override,\n");
    sv.push_str("  output logic [N_SIGNALS-1:0]       override_en,\n");
    sv.push_str("  output logic        emergency_active\n");
    sv.push_str(");\n\n");

    // Emergency latch — sticky until reset.
    sv.push_str("  // Emergency latch (sticky)\n");
    sv.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    sv.push_str("    if (!rst_n)\n");
    sv.push_str("      emergency_active <= 1'b0;\n");
    sv.push_str("    else if (action_valid)\n");
    sv.push_str("      case (selected_action_idx)\n");

    for (ai, entry) in config.action_table.iter().enumerate().take(MAX_RTL_ACTIONS) {
        if matches!(entry.action, AdaptationAction::EmergencyStop) {
            sv.push_str(&format!("        {}'d{ai}: emergency_active <= 1'b1;\n", act_w.max(1)));
        }
    }

    sv.push_str("        default: ; // no change\n");
    sv.push_str("      endcase\n");
    sv.push_str("  end\n\n");

    // Action dispatch — signal overrides.
    sv.push_str("  // Action dispatch\n");
    sv.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    sv.push_str("    if (!rst_n) begin\n");

    for i in 0..n_sig.min(MAX_RTL_SIGNALS) {
        sv.push_str(&format!("      signal_override[{i}] <= 32'd0;\n"));
        sv.push_str(&format!("      override_en[{i}]     <= 1'b0;\n"));
    }

    sv.push_str("    end else if (action_valid) begin\n");
    sv.push_str("      case (selected_action_idx)\n");

    let sig_idx = |name: &str| -> usize {
        for (i, s) in config.sensors.iter().enumerate().take(MAX_RTL_SIGNALS) {
            if s.name == name {
                return i;
            }
        }
        0
    };

    for (ai, entry) in config.action_table.iter().enumerate().take(MAX_RTL_ACTIONS) {
        sv.push_str(&format!("        {}'d{ai}: begin\n", act_w.max(1)));
        match &entry.action {
            AdaptationAction::SetSignal { name, value } => {
                let si = sig_idx(name);
                sv.push_str(&format!("          signal_override[{si}] <= 32'd{value};\n"));
                sv.push_str(&format!("          override_en[{si}]     <= 1'b1;\n"));
            }
            AdaptationAction::SwitchMode { .. } => {
                sv.push_str("          ; // mode switch — handled by top-level FSM\n");
            }
            AdaptationAction::EmergencyStop => {
                for i in 0..n_sig.min(MAX_RTL_SIGNALS) {
                    sv.push_str(&format!("          signal_override[{i}] <= 32'd0;\n"));
                    sv.push_str(&format!("          override_en[{i}]     <= 1'b1;\n"));
                }
            }
            AdaptationAction::Throttle => {
                sv.push_str("          ; // throttle — rate reduction handled by top-level FSM\n");
            }
            AdaptationAction::Reduce | AdaptationAction::LogWarning => {
                sv.push_str("          ; // no action (logging / reduce semantics)\n");
            }
        }
        sv.push_str("        end\n");
    }

    sv.push_str("        default: ; // no action\n");
    sv.push_str("      endcase\n");
    sv.push_str("    end\n");
    sv.push_str("  end\n\n");
    sv.push_str("endmodule\n\n");
    sv
}

// ---------------------------------------------------------------------------
// Knowledge block
// ---------------------------------------------------------------------------

/// Emit `mirr_knowledge`: FIFO ring buffer for adaptation records.
pub(super) fn emit_knowledge_block(config: &SimConfig) -> String {
    let depth = config.knowledge_capacity.min(MAX_RTL_KNOWLEDGE_DEPTH);
    let addr_w = bit_width(depth);
    let n_act = config.action_table.len();
    let act_w = bit_width(n_act);

    let mut sv = String::with_capacity(1024);
    sv.push_str("// --- Knowledge (K) -----------------------------------------\n");
    sv.push_str("module mirr_knowledge #(\n");
    sv.push_str(&format!("  parameter DEPTH = {depth}\n"));
    sv.push_str(") (\n");
    sv.push_str("  input  logic clk,\n");
    sv.push_str("  input  logic rst_n,\n");
    sv.push_str("  input  logic        wr_en,\n");
    sv.push_str(&format!("  input  logic [{}:0] wr_action_idx,\n", act_w.saturating_sub(1).max(0)));
    sv.push_str("  input  logic [31:0] wr_tick,\n");
    sv.push_str(&format!("  output logic [{}:0] count,\n", addr_w));
    sv.push_str("  output logic        full\n");
    sv.push_str(");\n\n");

    let record_w = 32 + act_w.max(1);
    sv.push_str(&format!("  logic [{}:0] fifo [0:DEPTH-1];\n", record_w.saturating_sub(1)));
    sv.push_str(&format!("  logic [{}:0] wr_ptr;\n\n", addr_w.saturating_sub(1)));

    sv.push_str("  assign full = (count == DEPTH);\n\n");

    sv.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    sv.push_str("    if (!rst_n) begin\n");
    sv.push_str(&format!("      wr_ptr <= {}'d0;\n", addr_w));
    sv.push_str(&format!("      count  <= {}'d0;\n", addr_w + 1));
    sv.push_str("    end else if (wr_en && !full) begin\n");
    sv.push_str("      fifo[wr_ptr] <= {wr_tick, wr_action_idx};\n");
    sv.push_str(&format!(
        "      wr_ptr <= (32'(wr_ptr) == DEPTH-1) ? {}'d0 : wr_ptr + 1;\n",
        addr_w
    ));
    sv.push_str("      count  <= count + 1;\n");
    sv.push_str("    end\n");
    sv.push_str("  end\n\n");
    sv.push_str("endmodule\n\n");
    sv
}

// ---------------------------------------------------------------------------
// Top-level block
// ---------------------------------------------------------------------------

/// Emit `mirr_mape_k_top`: wires all five MAPE-K blocks together.
pub(super) fn emit_mape_k_top(config: &SimConfig) -> String {
    let n_sig = config.sensors.len();
    let n_prop = config.properties.len();
    let n_act = config.action_table.len();
    let depth = config.knowledge_capacity.min(MAX_RTL_KNOWLEDGE_DEPTH);
    let trace_depth = config.window_size.min(MAX_RTL_KNOWLEDGE_DEPTH);
    let act_w = bit_width(n_act);
    let prop_w = bit_width(n_prop);
    let addr_w = bit_width(depth);

    let mut sv = String::with_capacity(2048);
    sv.push_str("// --- MAPE-K Top -------------------------------------------\n");
    sv.push_str("module mirr_mape_k_top #(\n");
    sv.push_str(&format!("  parameter N_SIGNALS    = {n_sig},\n"));
    sv.push_str(&format!("  parameter N_PROPERTIES = {n_prop},\n"));
    sv.push_str(&format!("  parameter N_ACTIONS    = {n_act},\n"));
    sv.push_str(&format!("  parameter K_DEPTH      = {depth}\n"));
    sv.push_str(") (\n");
    sv.push_str("  input  logic clk,\n");
    sv.push_str("  input  logic rst_n,\n");
    sv.push_str("  input  logic [N_SIGNALS-1:0][31:0] sensor_in,\n");
    sv.push_str("  output logic        emergency_active,\n");
    sv.push_str("  output logic [N_SIGNALS-1:0][31:0] signal_override,\n");
    sv.push_str("  output logic [N_SIGNALS-1:0]       override_en\n");
    sv.push_str(");\n\n");

    sv.push_str("  // Monitor -> Analyze\n");
    sv.push_str("  logic [N_SIGNALS-1:0][31:0] shadow;\n");
    sv.push_str("  logic        sample_valid;\n\n");

    sv.push_str("  // Analyze -> Plan\n");
    sv.push_str(&format!("  logic [{}:0] violation_vec;\n", n_prop.max(1).saturating_sub(1)));
    sv.push_str(&format!("  logic [{}:0] top_violation_idx;\n\n", prop_w.saturating_sub(1).max(0)));

    sv.push_str("  // Plan -> Execute\n");
    sv.push_str(&format!("  logic [{}:0] selected_action_idx;\n", act_w.saturating_sub(1).max(0)));
    sv.push_str("  logic        action_valid;\n\n");

    sv.push_str("  // Knowledge write channel\n");
    sv.push_str(&format!("  logic [{}:0] k_count;\n", addr_w));
    sv.push_str("  logic        k_full;\n");
    sv.push_str("  logic [31:0] tick_counter;\n\n");

    sv.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    sv.push_str("    if (!rst_n)\n");
    sv.push_str("      tick_counter <= 32'd0;\n");
    sv.push_str("    else\n");
    sv.push_str("      tick_counter <= tick_counter + 1;\n");
    sv.push_str("  end\n\n");

    sv.push_str("  mirr_monitor #(\n");
    sv.push_str("    .N_SIGNALS   (N_SIGNALS),\n");
    sv.push_str(&format!("    .TRACE_DEPTH ({trace_depth})\n"));
    sv.push_str("  ) u_monitor (\n");
    sv.push_str("    .clk          (clk),\n");
    sv.push_str("    .rst_n        (rst_n),\n");
    sv.push_str("    .sensor_in    (sensor_in),\n");
    sv.push_str("    .shadow       (shadow),\n");
    sv.push_str("    .sample_valid (sample_valid)\n");
    sv.push_str("  );\n\n");

    sv.push_str("  mirr_analyze #(\n");
    sv.push_str("    .N_SIGNALS    (N_SIGNALS),\n");
    sv.push_str("    .N_PROPERTIES (N_PROPERTIES)\n");
    sv.push_str("  ) u_analyze (\n");
    sv.push_str("    .clk               (clk),\n");
    sv.push_str("    .rst_n             (rst_n),\n");
    sv.push_str("    .shadow            (shadow),\n");
    sv.push_str("    .sample_valid      (sample_valid),\n");
    sv.push_str("    .violation_vec     (violation_vec),\n");
    sv.push_str("    .top_violation_idx (top_violation_idx)\n");
    sv.push_str("  );\n\n");

    sv.push_str("  mirr_plan #(\n");
    sv.push_str("    .N_PROPERTIES (N_PROPERTIES),\n");
    sv.push_str("    .N_ACTIONS    (N_ACTIONS)\n");
    sv.push_str("  ) u_plan (\n");
    sv.push_str("    .clk                (clk),\n");
    sv.push_str("    .rst_n              (rst_n),\n");
    sv.push_str("    .violation_vec      (violation_vec),\n");
    sv.push_str("    .selected_action_idx(selected_action_idx),\n");
    sv.push_str("    .action_valid       (action_valid)\n");
    sv.push_str("  );\n\n");

    sv.push_str("  mirr_execute #(\n");
    sv.push_str("    .N_SIGNALS (N_SIGNALS),\n");
    sv.push_str("    .N_ACTIONS (N_ACTIONS)\n");
    sv.push_str("  ) u_execute (\n");
    sv.push_str("    .clk                (clk),\n");
    sv.push_str("    .rst_n              (rst_n),\n");
    sv.push_str("    .selected_action_idx(selected_action_idx),\n");
    sv.push_str("    .action_valid       (action_valid),\n");
    sv.push_str("    .signal_override    (signal_override),\n");
    sv.push_str("    .override_en        (override_en),\n");
    sv.push_str("    .emergency_active   (emergency_active)\n");
    sv.push_str("  );\n\n");

    sv.push_str("  mirr_knowledge #(\n");
    sv.push_str("    .DEPTH (K_DEPTH)\n");
    sv.push_str("  ) u_knowledge (\n");
    sv.push_str("    .clk           (clk),\n");
    sv.push_str("    .rst_n         (rst_n),\n");
    sv.push_str("    .wr_en         (action_valid),\n");
    sv.push_str("    .wr_action_idx (selected_action_idx),\n");
    sv.push_str("    .wr_tick       (tick_counter),\n");
    sv.push_str("    .count         (k_count),\n");
    sv.push_str("    .full          (k_full)\n");
    sv.push_str("  );\n\n");

    sv.push_str("endmodule\n");
    sv
}
