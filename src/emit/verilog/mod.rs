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

use crate::ast::types::{BinaryOp, UnaryOp};
use crate::emit::fpga_target::FpgaTarget;
use crate::pipeline::PipelineResult;
use crate::span::{FileTable, Span};

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
    let mut out = String::with_capacity(4096);

    let ft = &result.file_table;
    let registry = match &result.ecs_registry {
        Some(r) => r,
        None => return "// No ECS registry available\n".to_string(),
    };
    let module_name = registry.get_module_name().unwrap_or_else(|| "unnamed".to_string());
    let module_span = registry.get_module_span();

    // Determine which reflexes contain DSP-eligible multiplies.
    // threshold=0 means DSP inference is disabled.
    let dsp_reflexes = if target.is_some() && dsp_threshold > 0 {
        let analysis = crate::emit::dsp::analyze_dsp_ecs(registry, dsp_threshold);
        analysis.candidates.into_iter().map(|c| c.reflex_name).collect()
    } else {
        std::collections::HashSet::new()
    };
    let dsp_attr = if dsp_threshold > 0 { target.map(|t| t.dsp_attribute()) } else { None };

    sva::emit_header(&mut out);
    sva::emit_pattern_annotations_ecs(registry, &mut out);
    sva::emit_module_decl(&module_name, registry, ft, &mut out, module_span.as_ref());
    sva::emit_internal_signals(registry, ft, &mut out);

    if let Some(netlist) = &result.temporal_netlist {
        if !netlist.signals.is_empty() {
            out.push_str("  // Temporal signals\n");
            for s in &netlist.signals {
                let type_str = super::sv_type(&s.ty);
                out.push_str(&format!("  {type_str} {};\n", s.name));
            }
            out.push('\n');
        }
        temporal::emit_temporal_logic_ecs(registry, netlist, ft, &mut out);
    }

    temporal::emit_reflex_logic_ecs(registry, &dsp_reflexes, dsp_attr, ft, &mut out);

    if !registry.extern_instantiations.is_empty() {
        out.push_str("  // ── Structural Module Instantiations ──\n\n");
        let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
            if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
                Some(crate::ecs::EntityId(i as u32))
            } else {
                None
            }
        });

        for call_id in &registry.extern_instantiations {
            if let Some(top_id) = top_module_id {
                if let Some(crate::ecs::components::ModuleComponent(parent_id)) = &registry.modules[call_id.0 as usize] {
                    if *parent_id != top_id {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if let Some(call_comp) = &registry.pattern_calls[call_id.0 as usize] {
                let call = &call_comp.0;
                let pattern_name = &call.pattern_name;

                let mut param_names = Vec::new();
                if let Some(def_id) = registry.get_entity_by_name(pattern_name) {
                    if let Some(def_comp) = &registry.pattern_defs[def_id.0 as usize] {
                        param_names = def_comp.0.params.iter().map(|p| p.name.clone()).collect();
                    }
                }

                let instance_name =
                    format!("{}_inst_{}", pattern_name.replace("::", "_"), call_id.0);

                let module_name = if let Some(idx) = pattern_name.rfind("::") {
                    &pattern_name[idx + 2..]
                } else {
                    pattern_name
                };

                out.push_str(&format!("  {} {} (\n", module_name, instance_name));

                for (i, arg) in call.arguments.iter().enumerate() {
                    let port_name =
                        if i < param_names.len() { &param_names[i] } else { "UNKNOWN_PORT" };

                    let arg_str = match arg {
                        crate::ast::pattern::PatternArg::SignalRef(s) => s.clone(),
                        crate::ast::pattern::PatternArg::ConstInt(v) => format!("{}", v),
                        crate::ast::pattern::PatternArg::ConstBool(b) => {
                            if *b {
                                "1'b1".to_string()
                            } else {
                                "1'b0".to_string()
                            }
                        }
                        crate::ast::pattern::PatternArg::PatternRef(s) => s.clone(),
                    };

                    let comma = if i == call.arguments.len() - 1 { "" } else { "," };
                    out.push_str(&format!("    .{}({}){}\n", port_name, arg_str, comma));
                }
                out.push_str("  );\n\n");
            }
        }
    }

    if !strip_sva {
        let has_rst_n = sva::module_has_rst_n(registry);
        sva::emit_property_assertions(registry, has_rst_n, ft, &mut out);
    }

    sva::emit_module_end(&mut out);

    out
}

pub fn emit_sv_standalone(result: &PipelineResult) -> String {
    let mut out = String::with_capacity(1024);
    let ft = &result.file_table;
    let registry = match &result.ecs_registry {
        Some(r) => r,
        None => return "// No ECS registry available\n".to_string(),
    };
    let module_name = registry.get_module_name().unwrap_or_else(|| "unnamed".to_string());

    sva::emit_header(&mut out);
    sva::emit_pattern_annotations_ecs(registry, &mut out);
    sva::emit_module_decl(&module_name, registry, ft, &mut out, None);
    sva::emit_internal_signals(registry, ft, &mut out);

    if let Some(netlist) = &result.temporal_netlist {
        temporal::emit_temporal_logic_standalone(netlist, ft, &mut out);
    }

    temporal::emit_reflex_logic_ecs(
        registry,
        &std::collections::HashSet::new(),
        None,
        ft,
        &mut out,
    );

    out.push_str("endmodule\n");
    out
}

use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};

/// Emit a SystemVerilog bind file containing only SVA properties.
///
/// This produces a standalone module with the same port list as the
/// original design, containing only SVA assertions. A `bind` statement
/// connects it to the DUT for formal verification while keeping RTL
/// synthesis-clean.
pub fn emit_sva_bind_file(result: &PipelineResult) -> Result<String, MirrError> {
    let registry = result.ecs_registry.as_ref().ok_or_else(|| {
        mirrcode(ErrorCode::RspuFallback, "ECS registry required for SVA bind emission")
    })?;
    let module_name = registry.get_module_name().unwrap_or_else(|| "unnamed".to_string());

    let property_count = registry.property_comps.iter().flatten().count();
    if property_count == 0 {
        return Ok(String::new());
    }

    let has_rst_n = sva::module_has_rst_n(registry);

    let mut out = String::with_capacity(2048);
    out.push_str("// Auto-generated SVA bind file from MIRR compiler\n");
    out.push_str(&format!("// Bind target: {}\n", module_name));
    out.push_str("// Use with: read_verilog -sv <this_file> (formal verification only)\n\n");

    // Emit the SVA wrapper module with the same ports.
    let sva_mod_name = format!("{}_sva", module_name);
    out.push_str(&format!("module {sva_mod_name} (\n"));

    let needs_temporal =
        registry.kinds.iter().flatten().any(|k| k.0 == crate::ecs::EntityKind::GUARD)
            || property_count > 0;

    let top_module_id = registry.kinds.iter().enumerate().rev().find_map(|(i, k)| {
        if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
            Some(crate::ecs::EntityId(i as u32))
        } else {
            None
        }
    });

    let mut declared_ports: Vec<String> = Vec::new();
    let mut has_clk = false;
    let mut has_rst = false;

    for i in 0..registry.names.len() {
        if let Some(top_id) = top_module_id {
            if let Some(crate::ecs::components::ModuleComponent(parent_id)) = &registry.modules[i] {
                if *parent_id != top_id {
                    continue;
                }
            } else {
                continue;
            }
        }

        if let (Some(name), Some(kind), Some(ty)) =
            (&registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            if let crate::ecs::EntityKind::SIGNAL(_) = kind.0 {
                let sname = registry.resolve_name(name.0);
                if sname == "clk" {
                    has_clk = true;
                }
                if sname == "rst_n" {
                    has_rst = true;
                }
                let dir = "input ";
                let type_str = super::sv_type(&ty.0.signal_type());
                declared_ports.push(format!("  {dir} {type_str} {}", sname));
            }
        }
    }

    let mut ports: Vec<String> = Vec::new();
    if needs_temporal && !has_clk {
        ports.push("  input  logic        clk".to_string());
    }
    if needs_temporal && !has_rst {
        ports.push("  input  logic        rst_n".to_string());
    }
    ports.extend(declared_ports);

    let port_count = ports.len();
    for (i, port) in ports.iter().enumerate() {
        let comma = if i + 1 < port_count { "," } else { "" };
        out.push_str(&format!("{port}{comma}\n"));
    }
    out.push_str(");\n\n");

    // Emit all SVA properties.
    let ft = &result.file_table;
    for i in 0..registry.names.len() {
        if let (Some(name_comp), Some(kind_comp), Some(prop_comp)) =
            (&registry.names[i], &registry.kinds[i], &registry.property_comps[i])
        {
            if let crate::ecs::EntityKind::PROPERTY = kind_comp.0 {
                let prop_name = registry.resolve_name(name_comp.0);
                let mut clock_domain = "clk";
                if let Some(tc) = &registry.types[i] {
                    if let Some(cd) = tc.0.annotations.clock_domain.as_deref() {
                        clock_domain = cd;
                    }
                }
                let span = registry.spans[i].as_ref().map(|s| &s.0);
                sva::emit_single_property(
                    prop_name, prop_comp, clock_domain, has_rst_n, registry, ft, &mut out, span,
                );
            }
        }
    }

    out.push_str("endmodule\n\n");

    // Emit the bind statement.
    out.push_str(&format!("bind {} {sva_mod_name} u_sva (.*);\n", module_name));

    Ok(out)
}

// -----------------------------------------------------------------------
// Shared helpers (used by both temporal.rs and sva.rs submodules)
// -----------------------------------------------------------------------

/// Emit a source location comment if span data is available.
pub(crate) fn emit_source_comment(span: Option<&Span>, table: &FileTable, out: &mut String) {
    if let Some(s) = span {
        let loc = s.display_location(table);
        out.push_str(&format!("  // source: {loc}\n"));
    }
}

/// Emit an ExpressionComponent graph as an inline SystemVerilog expression.
///
/// Bounded by 512 nodes.
pub(super) fn emit_expr_inline(
    expr_id: crate::ecs::EntityId,
    registry: &crate::ecs::Registry,
) -> String {
    let mut result_stack: Vec<String> = Vec::with_capacity(32);
    let mut work: Vec<ExprWork> = Vec::with_capacity(32);
    work.push(ExprWork::Eval(expr_id));

    let mut visited = 0usize;

    while let Some(item) = work.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            return "/* truncated */".to_string();
        }

        match item {
            ExprWork::Eval(id) => {
                let idx = id.0 as usize;

                if let Some(crate::ecs::components::LiteralComponent(lit)) = &registry.literals[idx]
                {
                    match lit {
                        crate::ast::types::LiteralValue::Bool(true) => {
                            result_stack.push("1'b1".to_string())
                        }
                        crate::ast::types::LiteralValue::Bool(false) => {
                            result_stack.push("1'b0".to_string())
                        }
                        crate::ast::types::LiteralValue::Integer(n) => {
                            result_stack.push(format!("{n}"))
                        }
                    }
                } else if let Some(crate::ecs::components::SignalRefComponent(sig_ent)) =
                    registry.signal_refs[idx]
                {
                    let sig_name = registry.names[sig_ent.0 as usize]
                        .map(|n| registry.resolve_name(n.0).to_string())
                        .unwrap_or_default();
                    result_stack.push(sig_name);
                } else if let Some(crate::ecs::components::PendingSignalRef(name)) =
                    &registry.pending_signal_refs[idx]
                {
                    result_stack.push(name.clone());
                } else if let Some(crate::ecs::components::PrevComponent { signal, delay }) =
                    &registry.prev_ops[idx]
                {
                    let sig_name = if let Some(crate::ecs::components::SignalRefComponent(decl)) =
                        registry.signal_refs[signal.0 as usize]
                    {
                        registry.names[decl.0 as usize]
                            .map(|n| registry.resolve_name(n.0).to_string())
                            .unwrap_or_default()
                    } else if let Some(crate::ecs::components::PendingSignalRef(n)) =
                        &registry.pending_signal_refs[signal.0 as usize]
                    {
                        n.clone()
                    } else {
                        String::new()
                    };
                    result_stack.push(format!("{}_d{}", sig_name, delay));
                } else if let Some(crate::ecs::components::BinaryComponent { op, left, right }) =
                    &registry.binary_ops[idx]
                {
                    work.push(ExprWork::EmitBinary(*op));
                    work.push(ExprWork::Eval(*right));
                    work.push(ExprWork::Eval(*left));
                } else if let Some(crate::ecs::components::UnaryComponent { op, operand }) =
                    &registry.unary_ops[idx]
                {
                    work.push(ExprWork::EmitUnary(*op));
                    work.push(ExprWork::Eval(*operand));
                } else if let Some(crate::ecs::components::ArrayIndexComponent { array, index }) =
                    &registry.array_indices[idx]
                {
                    work.push(ExprWork::EmitArrayIndex);
                    work.push(ExprWork::Eval(*index));
                    work.push(ExprWork::Eval(*array));
                } else if let Some(crate::ecs::components::FieldAccessComponent { object, field }) =
                    &registry.field_accesses[idx]
                {
                    work.push(ExprWork::EmitFieldAccess(field.clone()));
                    work.push(ExprWork::Eval(*object));
                } else if let Some(crate::ecs::components::ArrayLiteralComponent(elems)) =
                    &registry.array_literals[idx]
                {
                    work.push(ExprWork::EmitArrayLiteral(elems.len()));
                    for elem in elems.iter().rev() {
                        work.push(ExprWork::Eval(*elem));
                    }
                } else if let Some(crate::ecs::components::StructLiteralComponent {
                    name,
                    fields,
                }) = &registry.struct_literals[idx]
                {
                    work.push(ExprWork::EmitStructLiteral(
                        name.clone(),
                        fields.iter().map(|(f, _)| f.clone()).collect(),
                    ));
                    for (_, val) in fields.iter().rev() {
                        work.push(ExprWork::Eval(*val));
                    }
                } else if let Some(crate::ecs::components::UnfoldIndexComponent(name)) =
                    &registry.unfold_indices[idx]
                {
                    unreachable!("UnfoldIndex '{}' reached Verilog emitter", name);
                } else if let Some(crate::ecs::components::MuxComponent {
                    select,
                    true_val,
                    false_val,
                }) = &registry.muxes[idx]
                {
                    work.push(ExprWork::EmitMux);
                    work.push(ExprWork::Eval(*false_val));
                    work.push(ExprWork::Eval(*true_val));
                    work.push(ExprWork::Eval(*select));
                } else {
                    result_stack.push("/* unknown_expr */".to_string());
                }
            }
            ExprWork::EmitUnary(op) => {
                let inner = result_stack.pop().unwrap_or_default();
                let res = match op {
                    UnaryOp::Not => format!("(!{inner})"),
                    UnaryOp::Negate => format!("(-{inner})"),
                    UnaryOp::ReductionOr => format!("(|{inner})"),
                };
                result_stack.push(res);
            }
            ExprWork::EmitBinary(op) => {
                let rhs = result_stack.pop().unwrap_or_default();
                let lhs = result_stack.pop().unwrap_or_default();
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
                result_stack.push(format!("({lhs} {op_str} {rhs})"));
            }
            ExprWork::EmitArrayIndex => {
                let index = result_stack.pop().unwrap_or_default();
                let array = result_stack.pop().unwrap_or_default();
                result_stack.push(format!("{array}[{index}]"));
            }
            ExprWork::EmitFieldAccess(field) => {
                let object = result_stack.pop().unwrap_or_default();
                result_stack.push(format!("{object}.{field}"));
            }
            ExprWork::EmitArrayLiteral(count) => {
                let mut parts = Vec::with_capacity(count);
                for _ in 0..count {
                    parts.push(result_stack.pop().unwrap_or_default());
                }
                result_stack.push(format!("'{{{}}}", parts.join(", ")));
            }
            ExprWork::EmitStructLiteral(name, fields) => {
                let mut parts = Vec::with_capacity(fields.len());
                for f in fields {
                    let val = result_stack.pop().unwrap_or_default();
                    parts.push(format!("{f}: {val}"));
                }
                result_stack.push(format!("{name}'{{{}}}", parts.join(", ")));
            }
            ExprWork::EmitMux => {
                let false_val = result_stack.pop().unwrap_or_default();
                let true_val = result_stack.pop().unwrap_or_default();
                let cond = result_stack.pop().unwrap_or_default();
                result_stack.push(format!("({cond} ? {true_val} : {false_val})"));
            }
        }
    }

    result_stack.pop().unwrap_or_default()
}

enum ExprWork {
    Eval(crate::ecs::EntityId),
    EmitUnary(UnaryOp),
    EmitBinary(BinaryOp),
    EmitArrayIndex,
    EmitFieldAccess(String),
    EmitArrayLiteral(usize),
    EmitStructLiteral(String, Vec<String>),
    EmitMux,
}
