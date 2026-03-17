//! Monitor and Analyze block emitters, plus shared SV helpers.
//!
//! `predicate_to_sv` and `bit_width` are `pub(crate)` so the sibling
//! `upper` module can use them without duplication.

#![forbid(unsafe_code)]

use super::{MAX_RTL_KNOWLEDGE_DEPTH, MAX_RTL_PROPERTIES, MAX_RTL_SIGNALS};
use crate::mape_k::ltl::{SignalPredicate, TemporalProperty};
use crate::mape_k::SimConfig;

// ---------------------------------------------------------------------------
// Monitor block
// ---------------------------------------------------------------------------

/// Emit `mirr_monitor`: shadow registers, trace buffer, threshold storage.
pub(super) fn emit_monitor_block(config: &SimConfig) -> String {
    let n_sig = config.sensors.len();
    let depth = config.window_size.min(MAX_RTL_KNOWLEDGE_DEPTH);
    let addr_w = bit_width(depth);

    let mut sv = String::with_capacity(1024);
    sv.push_str("// --- Monitor (M) -------------------------------------------\n");
    sv.push_str("module mirr_monitor #(\n");
    sv.push_str(&format!("  parameter N_SIGNALS = {n_sig},\n"));
    sv.push_str(&format!("  parameter TRACE_DEPTH = {depth}\n"));
    sv.push_str(") (\n");
    sv.push_str("  input  logic clk,\n");
    sv.push_str("  input  logic rst_n,\n");
    sv.push_str("  input  logic [31:0] sensor_in [0:N_SIGNALS-1],\n");
    sv.push_str("  output logic [31:0] shadow    [0:N_SIGNALS-1],\n");
    sv.push_str("  output logic        sample_valid\n");
    sv.push_str(");\n\n");

    // Trace buffer (ring).
    sv.push_str(&format!("  logic [{}:0] wr_ptr;\n", addr_w.saturating_sub(1)));
    sv.push_str("  logic [31:0] trace_buf [0:TRACE_DEPTH-1][0:N_SIGNALS-1];\n\n");

    // Shadow registers + trace write.
    sv.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
    sv.push_str("    if (!rst_n) begin\n");
    sv.push_str(&format!("      wr_ptr <= {addr_w}'d0;\n"));
    sv.push_str("      sample_valid <= 1'b0;\n");

    for i in 0..n_sig.min(MAX_RTL_SIGNALS) {
        sv.push_str(&format!("      shadow[{i}] <= 32'd0;\n"));
    }

    sv.push_str("    end else begin\n");
    sv.push_str("      sample_valid <= 1'b1;\n");

    for i in 0..n_sig.min(MAX_RTL_SIGNALS) {
        sv.push_str(&format!("      shadow[{i}] <= sensor_in[{i}];\n"));
        sv.push_str(&format!("      trace_buf[wr_ptr][{i}] <= sensor_in[{i}];\n"));
    }

    sv.push_str(&format!(
        "      wr_ptr <= (wr_ptr == TRACE_DEPTH-1) ? {addr_w}'d0 : wr_ptr + 1;\n"
    ));
    sv.push_str("    end\n");
    sv.push_str("  end\n\n");
    sv.push_str("endmodule\n\n");
    sv
}

// ---------------------------------------------------------------------------
// Analyze block
// ---------------------------------------------------------------------------

/// Emit `mirr_analyze`: per-property LTL checkers, priority encoder.
pub(super) fn emit_analyze_block(config: &SimConfig) -> String {
    let n_sig = config.sensors.len();
    let n_prop = config.properties.len();

    let mut sv = String::with_capacity(2048);
    sv.push_str("// --- Analyze (A) -------------------------------------------\n");
    sv.push_str("module mirr_analyze #(\n");
    sv.push_str(&format!("  parameter N_SIGNALS    = {n_sig},\n"));
    sv.push_str(&format!("  parameter N_PROPERTIES = {n_prop}\n"));
    sv.push_str(") (\n");
    sv.push_str("  input  logic clk,\n");
    sv.push_str("  input  logic rst_n,\n");
    sv.push_str("  input  logic [31:0] shadow [0:N_SIGNALS-1],\n");
    sv.push_str("  input  logic        sample_valid,\n");
    sv.push_str(&format!(
        "  output logic [{}:0] violation_vec,\n",
        n_prop.max(1).saturating_sub(1)
    ));
    sv.push_str(&format!(
        "  output logic [{}:0] top_violation_idx\n",
        bit_width(n_prop).saturating_sub(1).max(0)
    ));
    sv.push_str(");\n\n");

    // Build a signal-name-to-index map for this config.
    let sig_idx = |name: &str| -> usize {
        for (i, s) in config.sensors.iter().enumerate().take(MAX_RTL_SIGNALS) {
            if s.name == name {
                return i;
            }
        }
        0
    };

    // Per-property checker logic.
    for (pi, prop) in config.properties.iter().enumerate().take(MAX_RTL_PROPERTIES) {
        match prop {
            TemporalProperty::Always(pred) => {
                let idx = sig_idx(pred.signal_name());
                let cond = predicate_to_sv(pred, &format!("shadow[{idx}]"));
                sv.push_str(&format!("  // Property {pi}: Always\n"));
                sv.push_str("  always_comb begin\n");
                sv.push_str(&format!("    violation_vec[{pi}] = sample_valid && !({cond});\n"));
                sv.push_str("  end\n\n");
            }
            TemporalProperty::EventuallyWithin(pred, cycles) => {
                let idx = sig_idx(pred.signal_name());
                let cond = predicate_to_sv(pred, &format!("shadow[{idx}]"));
                let cw = bit_width(*cycles as usize);
                sv.push_str(&format!("  // Property {pi}: EventuallyWithin({cycles})\n"));
                sv.push_str(&format!("  logic [{cw}:0] ev_cnt_{pi};\n"));
                sv.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
                sv.push_str("    if (!rst_n)\n");
                sv.push_str(&format!("      ev_cnt_{pi} <= {}'d0;\n", cw + 1));
                sv.push_str(&format!("    else if ({cond})\n"));
                sv.push_str(&format!("      ev_cnt_{pi} <= {}'d0;\n", cw + 1));
                sv.push_str(&format!("    else if (ev_cnt_{pi} < {}'d{cycles})\n", cw + 1));
                sv.push_str(&format!("      ev_cnt_{pi} <= ev_cnt_{pi} + 1;\n"));
                sv.push_str("  end\n");
                sv.push_str("  always_comb begin\n");
                sv.push_str(&format!(
                    "    violation_vec[{pi}] = (ev_cnt_{pi} >= {}'d{cycles});\n",
                    cw + 1
                ));
                sv.push_str("  end\n\n");
            }
            TemporalProperty::Persists(pred, cycles) => {
                let idx = sig_idx(pred.signal_name());
                let cond = predicate_to_sv(pred, &format!("shadow[{idx}]"));
                let cw = bit_width(*cycles as usize);
                sv.push_str(&format!("  // Property {pi}: Persists({cycles})\n"));
                sv.push_str(&format!("  logic [{cw}:0] ps_cnt_{pi};\n"));
                sv.push_str(&format!("  logic        ps_ok_{pi};\n"));
                sv.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
                sv.push_str("    if (!rst_n) begin\n");
                sv.push_str(&format!("      ps_cnt_{pi} <= {}'d0;\n", cw + 1));
                sv.push_str(&format!("      ps_ok_{pi}  <= 1'b0;\n"));
                sv.push_str(&format!("    end else if ({cond}) begin\n"));
                sv.push_str(&format!("      if (ps_cnt_{pi} < {}'d{cycles})\n", cw + 1));
                sv.push_str(&format!("        ps_cnt_{pi} <= ps_cnt_{pi} + 1;\n"));
                sv.push_str(&format!(
                    "      if (ps_cnt_{pi} >= {}'d{}) \n",
                    cw + 1,
                    cycles.saturating_sub(1)
                ));
                sv.push_str(&format!("        ps_ok_{pi} <= 1'b1;\n"));
                sv.push_str("    end else begin\n");
                sv.push_str(&format!("      ps_cnt_{pi} <= {}'d0;\n", cw + 1));
                sv.push_str("    end\n");
                sv.push_str("  end\n");
                sv.push_str("  always_comb begin\n");
                sv.push_str(&format!("    violation_vec[{pi}] = sample_valid && !ps_ok_{pi};\n"));
                sv.push_str("  end\n\n");
            }
        }
    }

    // Priority encoder: find lowest-index violation.
    sv.push_str("  // Priority encoder — lowest index wins\n");
    sv.push_str("  always_comb begin\n");
    sv.push_str(&format!("    top_violation_idx = {}'d0;\n", bit_width(n_prop)));
    for pi in (0..n_prop.min(MAX_RTL_PROPERTIES)).rev() {
        sv.push_str(&format!(
            "    if (violation_vec[{pi}]) top_violation_idx = {}'d{pi};\n",
            bit_width(n_prop)
        ));
    }
    sv.push_str("  end\n\n");
    sv.push_str("endmodule\n\n");
    sv
}

// ---------------------------------------------------------------------------
// Helpers (pub(crate) so the sibling `upper` module can use them)
// ---------------------------------------------------------------------------

/// Convert a `SignalPredicate` to a SystemVerilog boolean expression.
///
/// `expr` is the SV expression for the signal value (e.g., `"shadow[3]"`).
pub(crate) fn predicate_to_sv(pred: &SignalPredicate, expr: &str) -> String {
    match pred {
        SignalPredicate::IsTrue(_) => format!("({expr} != 32'd0)"),
        SignalPredicate::LessThan(_, thr) => format!("({expr} < 32'd{thr})"),
        SignalPredicate::GreaterThan(_, thr) => format!("({expr} > 32'd{thr})"),
        SignalPredicate::InRange(_, lo, hi) => {
            format!("({expr} >= 32'd{lo} && {expr} <= 32'd{hi})")
        }
    }
}

/// Compute number of bits needed to represent `n` values (ceil(log2(n))).
/// Returns at least 1.
pub(crate) fn bit_width(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }
    let mut w = 0usize;
    let mut v = n - 1;
    // Bounded: at most 64 iterations (u64 width).
    for _ in 0..64 {
        if v == 0 {
            break;
        }
        v >>= 1;
        w += 1;
    }
    w.max(1)
}
