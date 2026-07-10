//! Monitor and Analyze block emitters, plus shared SV helpers.
//!
//! `predicate_to_sv` and `bit_width` are `pub(crate)` so the sibling
//! `upper` module can use them without duplication.

#![forbid(unsafe_code)]

use super::{MAX_RTL_KNOWLEDGE_DEPTH, MAX_RTL_PROPERTIES, MAX_RTL_SIGNALS};
use crate::mape_k::ltl::{SignalPredicate, TemporalProperty};
use crate::mape_k::SimConfig;
use std::fmt::Write;

// ---------------------------------------------------------------------------
// Monitor block
// ---------------------------------------------------------------------------

/// Emit `mirr_monitor`: shadow registers, trace buffer, threshold storage.
pub(super) fn emit_monitor_block(config: &SimConfig, main_clock: &str) -> String {
    let n_sig = config.sensors.len();
    let depth = config.window_size.min(MAX_RTL_KNOWLEDGE_DEPTH);
    let addr_w = bit_width(depth);

    let mut sv = String::with_capacity(1024);
    sv.push_str("// --- Monitor (M) -------------------------------------------\n");
    sv.push_str("module mirr_monitor #(\n");
    writeln!(sv, "  parameter N_SIGNALS = {n_sig},").unwrap();
    writeln!(sv, "  parameter TRACE_DEPTH = {depth}").unwrap();
    sv.push_str(") (\n");
    writeln!(sv, "  input  logic {},", main_clock).unwrap();
    sv.push_str("  input  logic rst_n,\n");
    sv.push_str("  input  logic [N_SIGNALS-1:0][31:0] sensor_in,\n");
    sv.push_str("  output logic [N_SIGNALS-1:0][31:0] shadow,\n");
    sv.push_str("  output logic        sample_valid\n");
    sv.push_str(");\n\n");

    // Trace buffer (ring).
    writeln!(sv, "  logic [{}:0] wr_ptr;", addr_w.saturating_sub(1)).unwrap();
    sv.push_str("  logic [31:0] trace_buf [0:TRACE_DEPTH-1][0:N_SIGNALS-1];\n\n");

    // Shadow registers + trace write.
    writeln!(sv, "  always_ff @(posedge {} or negedge rst_n) begin", main_clock).unwrap();
    sv.push_str("    if (!rst_n) begin\n");
    writeln!(sv, "      wr_ptr <= {addr_w}'d0;").unwrap();
    sv.push_str("      sample_valid <= 1'b0;\n");

    for i in 0..n_sig.min(MAX_RTL_SIGNALS) {
        writeln!(sv, "      shadow[{i}] <= 32'd0;").unwrap();
    }

    sv.push_str("    end else begin\n");
    sv.push_str("      sample_valid <= 1'b1;\n");

    for i in 0..n_sig.min(MAX_RTL_SIGNALS) {
        writeln!(sv, "      shadow[{i}] <= sensor_in[{i}];").unwrap();
        writeln!(sv, "      trace_buf[wr_ptr][{i}] <= sensor_in[{i}];").unwrap();
    }

    writeln!(sv, "      wr_ptr <= (32'(wr_ptr) == TRACE_DEPTH-1) ? {addr_w}'d0 : wr_ptr + 1;")
        .unwrap();
    sv.push_str("    end\n");
    sv.push_str("  end\n\n");
    sv.push_str("endmodule\n\n");
    sv
}

// ---------------------------------------------------------------------------
// Analyze block
// ---------------------------------------------------------------------------

/// Emit `mirr_analyze`: per-property LTL checkers, priority encoder.
pub(super) fn emit_analyze_block(config: &SimConfig, main_clock: &str) -> String {
    let n_sig = config.sensors.len();
    let n_prop = config.properties.len();

    let mut sv = String::with_capacity(2048);
    sv.push_str("// --- Analyze (A) -------------------------------------------\n");
    sv.push_str("module mirr_analyze #(\n");
    writeln!(sv, "  parameter N_SIGNALS    = {n_sig},").unwrap();
    writeln!(sv, "  parameter N_PROPERTIES = {n_prop}").unwrap();
    sv.push_str(") (\n");
    writeln!(sv, "  input  logic {},", main_clock).unwrap();
    sv.push_str("  input  logic rst_n,\n");
    sv.push_str("  input  logic [N_SIGNALS-1:0][31:0] shadow,\n");
    sv.push_str("  input  logic        sample_valid,\n");
    writeln!(sv, "  output logic [{}:0] violation_vec,", n_prop.max(1).saturating_sub(1)).unwrap();
    writeln!(sv, "  output logic [{}:0] top_violation_idx", bit_width(n_prop).saturating_sub(1))
        .unwrap();
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
                writeln!(sv, "  // Property {pi}: Always").unwrap();
                sv.push_str("  always_comb begin\n");
                writeln!(sv, "    violation_vec[{pi}] = sample_valid && !({cond});").unwrap();
                sv.push_str("  end\n\n");
            }
            TemporalProperty::EventuallyWithin(pred, cycles) => {
                let idx = sig_idx(pred.signal_name());
                let cond = predicate_to_sv(pred, &format!("shadow[{idx}]"));
                let cw = bit_width(*cycles as usize);
                writeln!(sv, "  // Property {pi}: EventuallyWithin({cycles})").unwrap();
                writeln!(sv, "  logic [{cw}:0] ev_cnt_{pi};").unwrap();
                writeln!(sv, "  always_ff @(posedge {} or negedge rst_n) begin", main_clock)
                    .unwrap();
                sv.push_str("    if (!rst_n)\n");
                writeln!(sv, "      ev_cnt_{pi} <= {}'d0;", cw + 1).unwrap();
                writeln!(sv, "    else if ({cond})").unwrap();
                writeln!(sv, "      ev_cnt_{pi} <= {}'d0;", cw + 1).unwrap();
                writeln!(sv, "    else if (ev_cnt_{pi} < {}'d{cycles})", cw + 1).unwrap();
                writeln!(sv, "      ev_cnt_{pi} <= ev_cnt_{pi} + 1;").unwrap();
                sv.push_str("  end\n");
                sv.push_str("  always_comb begin\n");
                writeln!(sv, "    violation_vec[{pi}] = (ev_cnt_{pi} >= {}'d{cycles});", cw + 1)
                    .unwrap();
                sv.push_str("  end\n\n");
            }
            TemporalProperty::Persists(pred, cycles) => {
                let idx = sig_idx(pred.signal_name());
                let cond = predicate_to_sv(pred, &format!("shadow[{idx}]"));
                let cw = bit_width(*cycles as usize);
                writeln!(sv, "  // Property {pi}: Persists({cycles})").unwrap();
                writeln!(sv, "  logic [{cw}:0] ps_cnt_{pi};").unwrap();
                writeln!(sv, "  logic        ps_ok_{pi};").unwrap();
                writeln!(sv, "  always_ff @(posedge {} or negedge rst_n) begin", main_clock)
                    .unwrap();
                sv.push_str("    if (!rst_n) begin\n");
                writeln!(sv, "      ps_cnt_{pi} <= {}'d0;", cw + 1).unwrap();
                writeln!(sv, "      ps_ok_{pi}  <= 1'b0;").unwrap();
                writeln!(sv, "    end else if ({cond}) begin").unwrap();
                writeln!(sv, "      if (ps_cnt_{pi} < {}'d{cycles})", cw + 1).unwrap();
                writeln!(sv, "        ps_cnt_{pi} <= ps_cnt_{pi} + 1;").unwrap();
                writeln!(sv, "      if (ps_cnt_{pi} >= {}'d{}) ", cw + 1, cycles.saturating_sub(1))
                    .unwrap();
                writeln!(sv, "        ps_ok_{pi} <= 1'b1;").unwrap();
                sv.push_str("    end else begin\n");
                writeln!(sv, "      ps_cnt_{pi} <= {}'d0;", cw + 1).unwrap();
                sv.push_str("    end\n");
                sv.push_str("  end\n");
                sv.push_str("  always_comb begin\n");
                writeln!(sv, "    violation_vec[{pi}] = sample_valid && !ps_ok_{pi};").unwrap();
                sv.push_str("  end\n\n");
            }
            TemporalProperty::AlwaysImplies(a, b) => {
                let idx_a = sig_idx(a.signal_name());
                let idx_b = sig_idx(b.signal_name());
                let cond_a = predicate_to_sv(a, &format!("shadow[{idx_a}]"));
                let cond_b = predicate_to_sv(b, &format!("shadow[{idx_b}]"));
                writeln!(sv, "  // Property {pi}: AlwaysImplies").unwrap();
                sv.push_str("  always_comb begin\n");
                writeln!(
                    sv,
                    "    violation_vec[{pi}] = sample_valid && ({cond_a} && !({cond_b}));"
                )
                .unwrap();
                sv.push_str("  end\n\n");
            }
            TemporalProperty::NeverImplies(a, b) => {
                let idx_a = sig_idx(a.signal_name());
                let idx_b = sig_idx(b.signal_name());
                let cond_a = predicate_to_sv(a, &format!("shadow[{idx_a}]"));
                let cond_b = predicate_to_sv(b, &format!("shadow[{idx_b}]"));
                writeln!(sv, "  // Property {pi}: NeverImplies").unwrap();
                writeln!(sv, "  logic never_implies_seen_{pi};").unwrap();
                writeln!(sv, "  always_ff @(posedge {} or negedge rst_n) begin", main_clock)
                    .unwrap();
                sv.push_str("    if (!rst_n)\n");
                writeln!(sv, "      never_implies_seen_{pi} <= 1'b0;").unwrap();
                writeln!(sv, "    else if ({cond_a} && !({cond_b}))").unwrap();
                writeln!(sv, "      never_implies_seen_{pi} <= 1'b1;").unwrap();
                sv.push_str("  end\n");
                sv.push_str("  always_comb begin\n");
                writeln!(sv, "    violation_vec[{pi}] = sample_valid && !never_implies_seen_{pi};")
                    .unwrap();
                sv.push_str("  end\n\n");
            }
            TemporalProperty::AlwaysFollowedBy(trigger, delay, response) => {
                let idx_t = sig_idx(trigger.signal_name());
                let idx_r = sig_idx(response.signal_name());
                let cond_t = predicate_to_sv(trigger, &format!("shadow[{idx_t}]"));
                let cond_r = predicate_to_sv(response, &format!("shadow[{idx_r}]"));
                let delay_w = bit_width(*delay as usize);
                writeln!(sv, "  // Property {pi}: AlwaysFollowedBy({delay})").unwrap();
                writeln!(sv, "  logic [{delay_w}:0] follow_cnt_{pi};").unwrap();
                writeln!(sv, "  logic follow_ok_{pi};").unwrap();
                writeln!(sv, "  always_ff @(posedge {} or negedge rst_n) begin", main_clock)
                    .unwrap();
                sv.push_str("    if (!rst_n) begin\n");
                writeln!(sv, "      follow_cnt_{pi} <= {}'d0;", delay_w + 1).unwrap();
                writeln!(sv, "      follow_ok_{pi}  <= 1'b1;").unwrap();
                sv.push_str("    end else begin\n");
                writeln!(sv, "      if ({cond_t}) begin").unwrap();
                writeln!(sv, "        follow_cnt_{pi} <= {}'d{delay};", delay_w + 1).unwrap();
                sv.push_str("        follow_ok_{pi} <= 1'b0;\n");
                sv.push_str("      end else if (follow_cnt_{pi} > 0) begin\n");
                writeln!(sv, "        follow_cnt_{pi} <= follow_cnt_{pi} - 1;").unwrap();
                writeln!(
                    sv,
                    "        if (follow_cnt_{pi} == 1 && !({cond_r})) follow_ok_{pi} <= 1'b0;"
                )
                .unwrap();
                sv.push_str("        else if (follow_cnt_{pi} == 1) follow_ok_{pi} <= 1'b1;\n");
                sv.push_str("      end\n");
                sv.push_str("    end\n");
                sv.push_str("  end\n");
                sv.push_str("  always_comb begin\n");
                writeln!(sv, "    violation_vec[{pi}] = sample_valid && !follow_ok_{pi};").unwrap();
                sv.push_str("  end\n\n");
            }
        }
    }

    // Priority encoder: find lowest-index violation.
    sv.push_str("  // Priority encoder — lowest index wins\n");
    sv.push_str("  always_comb begin\n");
    writeln!(sv, "    top_violation_idx = {}'d0;", bit_width(n_prop)).unwrap();
    for pi in (0..n_prop.min(MAX_RTL_PROPERTIES)).rev() {
        writeln!(
            sv,
            "    if (violation_vec[{pi}]) top_violation_idx = {}'d{pi};",
            bit_width(n_prop)
        )
        .unwrap();
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
