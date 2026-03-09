//! SystemVerilog RTL emitter for MIRR IR.
//!
//! Produces structural `.sv` output: module declarations, port lists,
//! `always_ff` blocks for temporal guards (shift-register and counter),
//! and `always_comb` blocks for reflex assignments.
//!
//! Width annotations from Phase 4 drive port declarations.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::program::Module;
use crate::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
use crate::ast::types::{BinaryOp, SignalKind, SignalType, UnaryOp};
use crate::emit::fpga_target::FpgaTarget;
use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::{CompiledGuard, TemporalNetlist};

use crate::ast::MAX_EXPR_NODES;

/// Maximum shift-register stages to emit inline before truncating.
pub(crate) const MAX_SR_STAGES_INLINE: u64 = 1024;

/// Emit SystemVerilog RTL from pipeline results.
pub fn emit_sv(result: &PipelineResult) -> String {
    emit_sv_with_options(result, None, crate::emit::dsp::DEFAULT_DSP_THRESHOLD)
}

pub fn emit_sv_with_options(
    result: &PipelineResult,
    target: Option<FpgaTarget>,
    dsp_threshold: u32,
) -> String {
    emit_sv_full(result, target, dsp_threshold, false)
}

/// Emit synthesis-clean SystemVerilog (no SVA properties).
///
/// Use this when targeting Yosys or other synthesis tools that
/// cannot parse SVA `assert property` / `assume property` blocks.
pub fn emit_sv_synthesis(
    result: &PipelineResult,
    target: Option<FpgaTarget>,
    dsp_threshold: u32,
) -> String {
    emit_sv_full(result, target, dsp_threshold, true)
}

fn emit_sv_full(
    result: &PipelineResult,
    target: Option<FpgaTarget>,
    dsp_threshold: u32,
    strip_sva: bool,
) -> String {
    let module = &result.program.module;
    let mut out = String::with_capacity(4096);

    // Determine which reflexes contain DSP-eligible multiplies.
    // threshold=0 means DSP inference is disabled.
    let dsp_reflexes = if target.is_some() && dsp_threshold > 0 {
        let analysis = crate::emit::dsp::analyze_dsp(module, dsp_threshold);
        analysis.candidates.into_iter().map(|c| c.reflex_name).collect()
    } else {
        std::collections::HashSet::new()
    };
    let dsp_attr = if dsp_threshold > 0 { target.map(|t| t.dsp_attribute()) } else { None };

    emit_header(&mut out);
    emit_pattern_annotations(module, &mut out);
    emit_module_decl(module, &mut out);
    emit_internal_signals(module, &mut out);

    if let Some(netlist) = &result.temporal_netlist {
        emit_temporal_logic(netlist, &mut out);
    }

    emit_reflex_logic(module, &dsp_reflexes, dsp_attr, &mut out);

    if !strip_sva {
        let has_rst_n = module_has_rst_n(module);
        emit_property_assertions(module, has_rst_n, &mut out);
    }

    emit_module_end(&mut out);

    out
}

/// Emit a SystemVerilog bind file containing only SVA properties.
///
/// This produces a standalone module with the same port list as the
/// original design, containing only SVA assertions. A `bind` statement
/// connects it to the DUT for formal verification while keeping RTL
/// synthesis-clean.
pub fn emit_sva_bind_file(result: &PipelineResult) -> String {
    let module = &result.program.module;

    if module.properties.is_empty() {
        return String::new();
    }

    let mut out = String::with_capacity(2048);
    let has_rst_n = module_has_rst_n(module);

    out.push_str("// Auto-generated SVA bind file from MIRR compiler\n");
    out.push_str(&format!("// Bind target: {}\n", module.name));
    out.push_str("// Use with: read_verilog -sv <this_file> (formal verification only)\n\n");

    // Emit the SVA wrapper module with the same ports.
    let sva_mod_name = format!("{}_sva", module.name);
    out.push_str(&format!("module {sva_mod_name} (\n"));

    let needs_temporal = !module.guards.is_empty();
    let has_clk = module.signals.iter().any(|s| s.name == "clk");
    let has_rst = module.signals.iter().any(|s| s.name == "rst_n");

    let mut ports: Vec<String> = Vec::new();
    if needs_temporal && !has_clk {
        ports.push("  input  logic        clk".to_string());
    }
    if needs_temporal && !has_rst {
        ports.push("  input  logic        rst_n".to_string());
    }
    for s in &module.signals {
        if s.kind == SignalKind::Input || s.kind == SignalKind::Output {
            let dir = "input ";
            let type_str = sv_type(&s.ty);
            ports.push(format!("  {dir} {type_str} {}", s.name));
        }
    }
    let port_count = ports.len();
    for (i, port) in ports.iter().enumerate() {
        let comma = if i + 1 < port_count { "," } else { "" };
        out.push_str(&format!("{port}{comma}\n"));
    }
    out.push_str(");\n\n");

    // Emit all SVA properties.
    for prop in &module.properties {
        emit_single_property(prop, has_rst_n, &mut out);
    }

    out.push_str("endmodule\n\n");

    // Emit the bind statement.
    out.push_str(&format!("bind {} {sva_mod_name} u_sva (.*);\n", module.name));

    out
}

// -----------------------------------------------------------------------
// Internal helpers
// -----------------------------------------------------------------------

fn emit_header(out: &mut String) {
    out.push_str("// Auto-generated by MIRR compiler (Phase 6)\n");
    out.push_str("// Do not edit — regenerate from .mirr source.\n");
    out.push_str("// Target: SystemVerilog (.sv)\n\n");
}

/// Emit pattern expansion annotations as SV comments.
fn emit_pattern_annotations(module: &Module, out: &mut String) {
    if module.pattern_origins.is_empty() {
        return;
    }
    out.push_str("// ── Pattern Expansions ──\n");
    for origin in &module.pattern_origins {
        out.push_str(&format!(
            "// Pattern: {}({})\n",
            origin.pattern_name, origin.call_args_summary
        ));
    }
    out.push('\n');
}

fn emit_module_decl(module: &Module, out: &mut String) {
    out.push_str(&format!("module {} (\n", module.name));

    let has_clk = module.signals.iter().any(|s| s.name == "clk");
    let has_rst_n = module.signals.iter().any(|s| s.name == "rst_n");
    let needs_temporal = !module.guards.is_empty();

    let mut ports: Vec<String> = Vec::new();

    // Auto-inject clk and rst_n when temporal guards exist.
    if needs_temporal && !has_clk {
        ports.push("  input  logic        clk".to_string());
    }
    if needs_temporal && !has_rst_n {
        ports.push("  input  logic        rst_n".to_string());
    }

    for s in &module.signals {
        if s.kind == SignalKind::Input || s.kind == SignalKind::Output {
            let dir = match s.kind {
                SignalKind::Input => "input ",
                SignalKind::Output => "output",
                SignalKind::Internal => "internal",
            };
            let type_str = sv_type(&s.ty);
            ports.push(format!("  {dir} {type_str} {}", s.name));
        }
    }

    let port_count = ports.len();
    for (i, port) in ports.iter().enumerate() {
        let comma = if i + 1 < port_count { "," } else { "" };
        out.push_str(&format!("{port}{comma}\n"));
    }

    out.push_str(");\n\n");
}

fn emit_internal_signals(module: &Module, out: &mut String) {
    let internals: Vec<_> =
        module.signals.iter().filter(|s| s.kind == SignalKind::Internal).collect();

    if !internals.is_empty() {
        out.push_str("  // Internal signals\n");
        for s in &internals {
            if let Some(ref origin) = s.origin {
                out.push_str(&format!("  // Pattern: {origin}\n"));
            }
            let type_str = sv_type(&s.ty);
            out.push_str(&format!("  {type_str} {};\n", s.name));
        }
        out.push('\n');
    }
}

fn emit_temporal_logic(netlist: &TemporalNetlist, out: &mut String) {
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
                    emit_expr_inline(&cx.combination_logic),
                ));
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

fn emit_reflex_logic(
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
                emit_expr_inline(&a.value),
            ));
        }
        out.push_str("  end\n\n");
    }
}

fn emit_module_end(out: &mut String) {
    out.push_str("endmodule\n");
}

/// Map MIRR SignalType to SystemVerilog type string.
fn sv_type(ty: &SignalType) -> String {
    super::sv_type(ty)
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

/// Emit an Expr as an inline SystemVerilog expression.
///
/// Bounded by 512 nodes.
fn emit_expr_inline(expr: &Expr) -> String {
    let mut iterations = 0usize;
    emit_expr_str(expr, &mut iterations)
}

/// Bounded expression-to-string conversion.
fn emit_expr_str(expr: &Expr, iterations: &mut usize) -> String {
    *iterations += 1;
    if *iterations > MAX_EXPR_NODES {
        return "/* truncated */".to_string();
    }
    match expr {
        Expr::Literal(crate::ast::types::LiteralValue::Bool(true)) => "1'b1".to_string(),
        Expr::Literal(crate::ast::types::LiteralValue::Bool(false)) => "1'b0".to_string(),
        Expr::Literal(crate::ast::types::LiteralValue::Integer(n)) => format!("{n}"),
        Expr::Signal(name) => name.clone(),
        Expr::Prev { signal, delay } => {
            // Prev maps to a registered delayed version of the signal.
            format!("{signal}_d{delay}")
        }
        Expr::Unary { op, operand } => {
            let inner = emit_expr_str(operand, iterations);
            match op {
                UnaryOp::Not => format!("(!{inner})"),
                UnaryOp::Negate => format!("(-{inner})"),
            }
        }
        Expr::Binary { op, left, right } => {
            let l = emit_expr_str(left, iterations);
            let r = emit_expr_str(right, iterations);
            let op_str = match op {
                BinaryOp::And => "&",
                BinaryOp::Or => "|",
                BinaryOp::Xor => "^",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                BinaryOp::Eq => "==",
                BinaryOp::Ne => "!=",
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Shl => "<<",
                BinaryOp::Shr => ">>",
            };
            format!("({l} {op_str} {r})")
        }
    }
}

// -----------------------------------------------------------------------
// Safety Property Assertions (SVA)
// -----------------------------------------------------------------------

/// Check if the module declares an input signal named `rst_n`.
fn module_has_rst_n(module: &Module) -> bool {
    module.signals.iter().any(|s| s.name == "rst_n" && s.kind == SignalKind::Input)
}

/// Emit SVA assertion blocks for all property declarations.
fn emit_property_assertions(module: &Module, has_rst_n: bool, out: &mut String) {
    if module.properties.is_empty() {
        return;
    }

    out.push_str("  // ── Safety Properties (SVA) ──\n\n");

    for prop in &module.properties {
        emit_single_property(prop, has_rst_n, out);
    }
}

/// Emit a single SVA `assert property` statement.
fn emit_single_property(prop: &PropertyDecl, has_rst_n: bool, out: &mut String) {
    if let Some(ref origin) = prop.origin {
        out.push_str(&format!("  // Pattern: {origin}\n"));
    }
    out.push_str(&format!("  // property: {}\n", prop.name));

    let sva_keyword = match prop.directive {
        PropertyDirective::Assert => "assert",
        PropertyDirective::Cover => "cover",
        PropertyDirective::Assume => "assume",
    };

    let disable_clause = if has_rst_n { " disable iff (!rst_n)" } else { "" };

    match &prop.formula {
        PropertyFormula::Always(expr) => {
            let sv_expr = emit_expr_inline(expr);
            out.push_str(&format!(
                "  {sva_keyword} property (@(posedge clk){disable_clause}\n    {sv_expr});\n\n"
            ));
        }
        PropertyFormula::Never(expr) => {
            let sv_expr = emit_expr_inline(expr);
            out.push_str(&format!(
                "  {sva_keyword} property (@(posedge clk){disable_clause}\n    !({sv_expr}));\n\n"
            ));
        }
        PropertyFormula::AlwaysImplies { antecedent, consequent } => {
            let ante_sv = emit_expr_inline(antecedent);
            let cons_sv = emit_expr_inline(consequent);
            out.push_str(&format!(
                "  {sva_keyword} property (@(posedge clk){disable_clause}\n    {ante_sv} |-> {cons_sv});\n\n"
            ));
        }
        PropertyFormula::NeverImplies { antecedent, consequent } => {
            let ante_sv = emit_expr_inline(antecedent);
            let cons_sv = emit_expr_inline(consequent);
            out.push_str(&format!(
                "  {sva_keyword} property (@(posedge clk){disable_clause}\n    !({ante_sv} |-> {cons_sv}));\n\n"
            ));
        }
        PropertyFormula::EventuallyWithin { expr, cycles } => {
            let sv_expr = emit_expr_inline(expr);
            out.push_str(&format!(
                "  {sva_keyword} property (@(posedge clk){disable_clause}\n    ##[1:{cycles}] {sv_expr});\n\n"
            ));
        }
        PropertyFormula::AlwaysFollowedBy { trigger, response, delay_cycles } => {
            let trig_sv = emit_expr_inline(trigger);
            let resp_sv = emit_expr_inline(response);
            out.push_str(&format!(
                "  {sva_keyword} property (@(posedge clk){disable_clause}\n    {trig_sv} |-> ##{delay_cycles} {resp_sv});\n\n"
            ));
        }
    }
}

// -----------------------------------------------------------------------
// Input Synchronizer Chains
// -----------------------------------------------------------------------

use crate::emit::fpga_target::MAX_SYNC_STAGES;

/// Emit 2-stage (or N-stage) synchronizer chains for all input signals
/// except `clk` and `rst_n`. Returns a mapping of original signal names
/// to their synchronized versions (_s suffix).
pub fn emit_synchronizer_chains(
    module: &Module,
    sync_stages: u32,
    out: &mut String,
) -> Vec<(String, String)> {
    if sync_stages == 0 {
        return Vec::new();
    }
    let stages = sync_stages.min(MAX_SYNC_STAGES);

    let mut mappings = Vec::new();
    out.push_str("  // ── Input Synchronizer Chains ──\n\n");

    for s in &module.signals {
        if s.kind != SignalKind::Input {
            continue;
        }
        if s.name == "clk" || s.name == "rst_n" {
            continue;
        }

        let width = match &s.ty {
            SignalType::Bool => 1u32,
            SignalType::Unsigned(w) | SignalType::Signed(w) => *w,
        };
        let sync_name = format!("{}_s", s.name);
        let sync_reg = format!("{}_sync", s.name);

        // Declare synchronizer register chain.
        let total_bits = width * stages;
        out.push_str(&format!("  // {}-stage synchronizer for {}\n", stages, s.name));
        out.push_str(&format!("  logic [{}:0] {};\n", total_bits.saturating_sub(1), sync_reg,));

        // Sequential synchronizer logic.
        out.push_str("  always_ff @(posedge clk or negedge rst_n) begin\n");
        out.push_str(&format!("    if (!rst_n)\n      {} <= '0;\n", sync_reg));
        if stages == 1 {
            out.push_str(&format!("    else\n      {} <= {};\n", sync_reg, s.name));
        } else {
            // Shift chain: {input, sync[high:width]}
            out.push_str(&format!(
                "    else\n      {} <= {{{}, {}[{}:{}]}};\n",
                sync_reg,
                s.name,
                sync_reg,
                total_bits.saturating_sub(1),
                width,
            ));
        }
        out.push_str("  end\n");

        // Output: synchronized signal is the last stage.
        let type_str = sv_type(&s.ty);
        out.push_str(&format!(
            "  {} {} = {}[{}:0];\n\n",
            type_str,
            sync_name,
            sync_reg,
            width.saturating_sub(1),
        ));

        mappings.push((s.name.clone(), sync_name));
    }

    mappings
}

/// Emit only the SVA assertion block (no module wrapper).
///
/// Used by `--emit sva` standalone mode.
pub fn emit_sva_only(result: &PipelineResult) -> String {
    let module = &result.program.module;
    let mut out = String::with_capacity(1024);

    out.push_str("// Auto-generated SVA assertions from MIRR compiler\n");
    out.push_str(&format!("// Module: {}\n\n", module.name));

    let has_rst_n = module_has_rst_n(module);

    for prop in &module.properties {
        emit_single_property(prop, has_rst_n, &mut out);
    }

    out
}
