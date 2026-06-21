//! Phase 6: Output backend module.
//!
//! Re-exports emitters for Graphviz DOT, FIRRTL, SystemVerilog RTL, JSON netlist,
//! and R-SPU assembly.

#![forbid(unsafe_code)]

pub mod arm;
pub mod cert;
pub mod dot;
pub mod dsp;
pub mod firrtl;
pub mod fpga_scaffold;
pub mod fpga_target;
pub mod json_netlist;
pub mod mape_k_rtl;
pub mod provenance;
pub mod riscv;
pub mod rspu;
pub mod rspu_encoding;
pub mod rspu_exceptions;
pub mod rspu_helpers;
pub mod rspu_isa;
pub mod rspu_opt;
pub mod rspu_regalloc;
pub mod rspu_sim;
pub mod rspu_tagged;
pub mod sexpr;
pub mod testbench;
pub mod verilog;

use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use crate::ast::MAX_EXPR_NODES;

/// Render an expression in MIRR-like text form for property/JSON output.
pub fn expr_text(expr: &Expr) -> String {
    let mut iters = 0usize;
    expr_text_bounded(expr, &mut iters)
}

pub(crate) fn expr_text_bounded(expr: &Expr, iters: &mut usize) -> String {
    *iters += 1;
    if *iters > MAX_EXPR_NODES {
        return "...".to_string();
    }
    match expr {
        Expr::Literal(LiteralValue::Bool(true)) => "true".to_string(),
        Expr::Literal(LiteralValue::Bool(false)) => "false".to_string(),
        Expr::Literal(LiteralValue::Integer(n)) => format!("{n}"),
        Expr::Signal(name) => name.clone(),
        Expr::Prev { signal, delay } => format!("prev({signal}, {delay})"),
        Expr::Unary { op: UnaryOp::Not, operand } => {
            format!("!{}", expr_text_bounded(operand, iters))
        }
        Expr::Unary { op: UnaryOp::Negate, operand } => {
            format!("-{}", expr_text_bounded(operand, iters))
        }
        Expr::Unary { op: UnaryOp::ReductionOr, operand } => {
            format!("|{}", expr_text_bounded(operand, iters))
        }
        Expr::Binary { op, left, right } => {
            let l = expr_text_bounded(left, iters);
            let r = expr_text_bounded(right, iters);
            let op_str = match op {
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
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
            let a = expr_text_bounded(array, iters);
            let i = expr_text_bounded(index, iters);
            format!("{a}[{i}]")
        }
        Expr::FieldAccess { object, field } => {
            let o = expr_text_bounded(object, iters);
            format!("{o}.{field}")
        }
        Expr::ArrayLiteral(elems) => {
            let parts: Vec<String> =
                elems.iter().take(MAX_EXPR_NODES).map(|e| expr_text_bounded(e, iters)).collect();
            format!("'{{{}}}", parts.join(", "))
        }
        Expr::StructLiteral { name, fields } => {
            let parts: Vec<String> = fields
                .iter()
                .take(MAX_EXPR_NODES)
                .map(|(f, v)| format!("{}: {}", f, expr_text_bounded(v, iters)))
                .collect();
            format!("{} {{ {} }}", name, parts.join(", "))
        }
        Expr::UnfoldIndex(name) => unreachable!("UnfoldIndex '{}' reached generic emit_text", name),
    }
}

use crate::ast::types::SignalType;

/// Map MIRR SignalType to SystemVerilog type string.
///
/// Shared by the verilog and testbench emitters.
pub(crate) fn sv_type(ty: &SignalType) -> String {
    match ty {
        SignalType::Bool => "logic       ".to_string(),
        SignalType::Unsigned(w) => {
            if *w == 1 {
                "logic       ".to_string()
            } else {
                format!("logic [{:>2}:0]", w - 1)
            }
        }
        SignalType::Signed(w) => {
            if *w == 1 {
                "logic signed".to_string()
            } else {
                format!("logic signed [{:>2}:0]", w - 1)
            }
        }
        SignalType::Array { element, length } => {
            let flat = element.width().checked_mul(*length as u32).unwrap_or(1).saturating_sub(1);
            format!("logic [{:>2}:0]", flat)
        }
        SignalType::Struct { .. } => {
            format!("logic [{:>2}:0]", ty.width().saturating_sub(1))
        }
        SignalType::FixedPoint { total_bits, .. } => {
            format!("logic [{:>2}:0]", total_bits.saturating_sub(1))
        }
        SignalType::Bundle(_) => "/* interface */ ".to_string(),
        SignalType::Fifo { element, depth } => {
            let flat = element.width().checked_mul(*depth as u32).unwrap_or(1).saturating_sub(1);
            format!("logic [{:>2}:0]", flat)
        }
    }
}
