//! JSON netlist emitter for MIRR IR.
//!
//! Serializes the compiled IR (post-simplify, post-width) as a
//! machine-readable JSON artifact for downstream tools.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use crate::ast::MirrAstJson;
use crate::pipeline::PipelineResult;
use crate::temporal::low_level_ir::TemporalNetlistJson;

/// Top-level JSON netlist structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonNetlist {
    /// Bump when JSON netlist schema changes. See CHANGELOG.md.
    pub schema_version: String,
    /// IR version for contract tracking.
    pub ir_version: String,
    /// The compiled program AST (post-simplification).
    pub program: MirrAstJson,
    /// Simplification statistics (null if skipped).
    pub simplify_stats: Option<SimplifyStatsJson>,
    /// Width inference statistics (null if skipped).
    pub width_stats: Option<WidthStatsJson>,
    /// Temporal netlist (null if skipped).
    pub temporal: Option<TemporalNetlistJson>,
    /// Safety property declarations.
    pub properties: Vec<PropertyJson>,
    /// Pattern expansion provenance (Phase 7b).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pattern_origins: Vec<PatternOriginJson>,
}

/// Serializable wrapper for SimplifyStats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifyStatsJson {
    /// Number of simplification rules applied.
    pub rules_applied: usize,
    /// Expression node count before simplification.
    pub nodes_before: usize,
    /// Expression node count after simplification.
    pub nodes_after: usize,
}

/// Serializable wrapper for WidthStats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthStatsJson {
    /// Total expression nodes analyzed.
    pub nodes_analyzed: usize,
    /// Constraint propagation iterations to fixpoint.
    pub propagation_rounds: usize,
    /// Number of width diagnostics emitted.
    pub diagnostics_count: usize,
    /// Number of strongly connected components detected.
    pub scc_count: usize,
    /// Number of expansive SCCs (contain Add/Mul/Shl).
    pub expansive_count: usize,
    /// Number of non-expansive SCCs (Prev-only or bitwise).
    pub nonexpansive_count: usize,
    /// True if any diagnostic is a hard error.
    pub has_errors: bool,
}

/// Serializable wrapper for a property declaration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyJson {
    /// Property name.
    pub name: String,
    /// Verification directive: "assert", "cover", or "assume".
    pub directive: String,
    /// Formula kind: "always", "never", "always_implies", etc.
    pub kind: String,
    /// Human-readable formula text.
    pub formula_text: String,
}

/// Serializable wrapper for pattern origin annotation (Phase 7b).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternOriginJson {
    /// Name of the pattern that was expanded.
    pub pattern_name: String,
    /// Human-readable argument summary.
    pub args_summary: String,
}

/// Emit a JSON netlist string from pipeline results.
pub fn emit_json(result: &PipelineResult) -> Result<String, serde_json::Error> {
    let netlist = build_netlist(result);
    serde_json::to_string_pretty(&netlist)
}

/// Build the JsonNetlist structure from pipeline results.
pub fn build_netlist(result: &PipelineResult) -> JsonNetlist {
    let program = MirrAstJson::from_program(&result.program);

    let simplify_stats = result.simplify_stats.as_ref().map(|s| SimplifyStatsJson {
        rules_applied: s.rules_applied,
        nodes_before: s.nodes_before,
        nodes_after: s.nodes_after,
    });

    let width_stats = result.width_result.as_ref().map(|w| WidthStatsJson {
        nodes_analyzed: w.stats.nodes_analyzed,
        propagation_rounds: w.stats.propagation_rounds,
        diagnostics_count: w.stats.diagnostics_count,
        scc_count: w.stats.scc_count,
        expansive_count: w.stats.expansive_count,
        nonexpansive_count: w.stats.nonexpansive_count,
        has_errors: w.has_errors(),
    });

    let temporal = result
        .temporal_netlist
        .as_ref()
        .map(crate::temporal::low_level_ir::TemporalNetlistJson::from_netlist);

    let properties: Vec<PropertyJson> =
        result.program.module.properties.iter().map(property_to_json).collect();

    let pattern_origins: Vec<PatternOriginJson> = result
        .program
        .module
        .pattern_origins
        .iter()
        .map(|o| PatternOriginJson {
            pattern_name: o.pattern_name.clone(),
            args_summary: o.call_args_summary.clone(),
        })
        .collect();

    JsonNetlist {
        schema_version: "0.2.0".to_string(),
        ir_version: "1.0".to_string(),
        program,
        simplify_stats,
        width_stats,
        temporal,
        properties,
        pattern_origins,
    }
}

/// Convert a PropertyDecl into its JSON representation.
fn property_to_json(prop: &crate::ast::property::PropertyDecl) -> PropertyJson {
    use crate::ast::property::{PropertyDirective, PropertyFormula};

    let directive_str = match prop.directive {
        PropertyDirective::Assert => "assert",
        PropertyDirective::Cover => "cover",
        PropertyDirective::Assume => "assume",
    };

    let (kind, formula_text) = match &prop.formula {
        PropertyFormula::Always(expr) => {
            ("always".to_string(), format!("always ({})", expr_text(expr)))
        }
        PropertyFormula::Never(expr) => {
            ("never".to_string(), format!("never ({})", expr_text(expr)))
        }
        PropertyFormula::AlwaysImplies { antecedent, consequent } => (
            "always_implies".to_string(),
            format!("always ({} -> {})", expr_text(antecedent), expr_text(consequent)),
        ),
        PropertyFormula::NeverImplies { antecedent, consequent } => (
            "never_implies".to_string(),
            format!("never ({} -> {})", expr_text(antecedent), expr_text(consequent)),
        ),
        PropertyFormula::EventuallyWithin { expr, cycles } => (
            "eventually_within".to_string(),
            format!("eventually within {} ({})", cycles, expr_text(expr)),
        ),
        PropertyFormula::AlwaysFollowedBy { trigger, response, delay_cycles } => (
            "always_followed_by".to_string(),
            format!(
                "always ({} followed_by {} {})",
                expr_text(trigger),
                delay_cycles,
                expr_text(response),
            ),
        ),
    };
    PropertyJson {
        name: prop.name.clone(),
        directive: directive_str.to_string(),
        kind,
        formula_text,
    }
}

/// Render an expression in MIRR-like text form for JSON output.
fn expr_text(expr: &crate::ast::Expr) -> String {
    let mut iters = 0usize;
    expr_text_bounded(expr, &mut iters)
}

fn expr_text_bounded(expr: &crate::ast::Expr, iters: &mut usize) -> String {
    use crate::ast::expr::Expr;
    use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
    const MAX: usize = 512;
    *iters += 1;
    if *iters > MAX {
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
        Expr::Binary { op, left, right } => {
            let l = expr_text_bounded(left, iters);
            let r = expr_text_bounded(right, iters);
            let op_str = match op {
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
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
