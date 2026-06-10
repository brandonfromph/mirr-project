//! SystemVerilog RTL emitter for MIRR IR.
//!
//! Produces structural `.sv` output: module declarations, port lists,
//! `always_ff` blocks for temporal guards (shift-register and counter),
//! and `always_comb` blocks for reflex assignments.
//!
//! Width annotations from Phase 4 drive port declarations.

#![forbid(unsafe_code)]

mod sva;
mod temporal;

pub use sva::{emit_sva_only, emit_synchronizer_chains};

use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, UnaryOp};
use crate::emit::fpga_target::FpgaTarget;
use crate::pipeline::PipelineResult;

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

    sva::emit_header(&mut out);
    sva::emit_pattern_annotations(module, &mut out);
    sva::emit_module_decl(module, &mut out);
    sva::emit_internal_signals(module, &mut out);

    if let Some(netlist) = &result.temporal_netlist {
        if !netlist.signals.is_empty() {
            out.push_str("  // Temporal signals\n");
            for s in &netlist.signals {
                let type_str = super::sv_type(&s.ty);
                out.push_str(&format!("  {type_str} {};\n", s.name));
            }
            out.push('\n');
        }
        temporal::emit_temporal_logic(module, netlist, &mut out);
    }

    temporal::emit_reflex_logic(
        module,
        &dsp_reflexes,
        dsp_attr,
        result.hls_result.as_ref(),
        &mut out,
    );

    if !module.pattern_calls.is_empty() {
        out.push_str("  // ── Structural Module Instantiations ──\n\n");
        for (i, call) in module.pattern_calls.iter().enumerate() {
            // Use the unqualified name for the module instantiation (e.g. 'ram' from 'ram::ram')
            let mod_name = call.pattern_name.split("::").last().unwrap_or(&call.pattern_name);
            let inst_name = format!("{}_{}", mod_name, i);
            out.push_str(&format!("  {mod_name} {inst_name} (\n"));
            let arg_count = call.arguments.len();
            for (j, arg) in call.arguments.iter().enumerate() {
                let comma = if j + 1 < arg_count { "," } else { "" };
                let val = match arg {
                    crate::ast::pattern::PatternArg::SignalRef(name) => name.clone(),
                    crate::ast::pattern::PatternArg::ConstInt(n) => format!("{n}"),
                    crate::ast::pattern::PatternArg::ConstBool(b) => {
                        if *b {
                            "1'b1".to_string()
                        } else {
                            "1'b0".to_string()
                        }
                    }
                    crate::ast::pattern::PatternArg::PatternRef(p) => p.clone(),
                };
                out.push_str(&format!("    {}{}\n", val, comma));
            }
            out.push_str("  );\n\n");
        }
    }

    if !strip_sva {
        let has_rst_n = sva::module_has_rst_n(module);
        sva::emit_property_assertions(module, has_rst_n, &mut out);
    }

    sva::emit_module_end(&mut out);

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
    let has_rst_n = sva::module_has_rst_n(module);

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
        let dir = "input ";
        let type_str = super::sv_type(&s.ty.signal_type());
        ports.push(format!("  {dir} {type_str} {}", s.name));
    }
    let port_count = ports.len();
    for (i, port) in ports.iter().enumerate() {
        let comma = if i + 1 < port_count { "," } else { "" };
        out.push_str(&format!("{port}{comma}\n"));
    }
    out.push_str(");\n\n");

    // Emit all SVA properties.
    for prop in &module.properties {
        sva::emit_single_property(prop, has_rst_n, &mut out);
    }

    out.push_str("endmodule\n\n");

    // Emit the bind statement.
    out.push_str(&format!("bind {} {sva_mod_name} u_sva (.*);\n", module.name));

    out
}

// -----------------------------------------------------------------------
// Shared helpers (used by both temporal.rs and sva.rs submodules)
// -----------------------------------------------------------------------

/// Emit an Expr as an inline SystemVerilog expression.
///
/// Bounded by 512 nodes.
pub(super) fn emit_expr_inline(expr: &Expr) -> String {
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
                BinaryOp::BitwiseOr => "|",
                BinaryOp::BitwiseAnd => "&",
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
        Expr::ArrayIndex { array, index } => {
            let a = emit_expr_str(array, iterations);
            let i = emit_expr_str(index, iterations);
            format!("{a}[{i}]")
        }
        Expr::FieldAccess { object, field } => {
            let o = emit_expr_str(object, iterations);
            format!("{o}.{field}")
        }
        Expr::ArrayLiteral(elems) => {
            let parts: Vec<String> =
                elems.iter().take(MAX_EXPR_NODES).map(|e| emit_expr_str(e, iterations)).collect();
            format!("'{{{}}}", parts.join(", "))
        }
        Expr::StructLiteral { name, fields } => {
            let parts: Vec<String> = fields
                .iter()
                .take(MAX_EXPR_NODES)
                .map(|(f, v)| format!("{}: {}", f, emit_expr_str(v, iterations)))
                .collect();
            format!("{}'{{{}}}", name, parts.join(", "))
        }
        Expr::UnfoldIndex(name) => unreachable!("UnfoldIndex '{}' reached Verilog emitter", name),
    }
}
