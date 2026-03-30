//! Property extraction and formula lowering.

#![forbid(unsafe_code)]

use crate::ast::property::{PropertyDirective, PropertyFormula};
use crate::ast::Expr;
use crate::mape_k::ltl::{SignalPredicate, TemporalProperty};
use crate::pipeline::PipelineResult;

use super::{BridgeError, MAX_BRIDGE_PROPERTIES};

/// Maximum expression nodes to visit when extracting a signal name.
const MAX_EXPR_VISIT: usize = 64;

/// Walk the module's property declarations. For each `Assert` property,
/// attempt to lower the formula to a `TemporalProperty`. `Cover` and
/// `Assume` directives are skipped.
pub(super) fn extract_properties(
    result: &PipelineResult,
    errors: &mut Vec<BridgeError>,
) -> Vec<TemporalProperty> {
    let props = &result.program.module.properties;

    let assert_count = count_assert_properties(props);
    if assert_count > MAX_BRIDGE_PROPERTIES {
        errors.push(BridgeError::TooManyProperties { count: assert_count });
        return Vec::new();
    }

    let mut temporal_props = Vec::with_capacity(assert_count);

    for prop in props.iter().take(MAX_BRIDGE_PROPERTIES) {
        if prop.directive != PropertyDirective::Assert {
            continue;
        }

        match lower_formula(&prop.formula) {
            Ok(tp) => temporal_props.push(tp),
            Err(desc) => errors.push(BridgeError::UnsupportedFormula { description: desc }),
        }

        if temporal_props.len() >= MAX_BRIDGE_PROPERTIES {
            break;
        }
    }

    temporal_props
}

/// Count assert-properties (bounded scan).
pub(super) fn count_assert_properties(props: &[crate::ast::property::PropertyDecl]) -> usize {
    let mut count: usize = 0;
    for p in props.iter().take(MAX_BRIDGE_PROPERTIES.saturating_add(1)) {
        if p.directive == PropertyDirective::Assert {
            count = count.saturating_add(1);
        }
    }
    count
}

/// Attempt to lower a `PropertyFormula` into a `TemporalProperty`.
///
/// Supported lowerings:
/// - `Always(expr)` -> `TemporalProperty::Always(predicate)`
/// - `Never(expr)` -> `TemporalProperty::Always(LessThan(name, 1))`
/// - `AlwaysImplies { antecedent, consequent }` -> `TemporalProperty::AlwaysImplies(pred_a, pred_b)`
/// - `NeverImplies { antecedent, consequent }` -> `TemporalProperty::NeverImplies(pred_a, pred_b)`
/// - `AlwaysFollowedBy { trigger, delay_cycles, response }` -> `TemporalProperty::AlwaysFollowedBy(trigger, delay, response)`
/// - `EventuallyWithin { expr, cycles }` -> `TemporalProperty::EventuallyWithin(predicate, cycles)`
fn lower_formula(formula: &PropertyFormula) -> Result<TemporalProperty, String> {
    match formula {
        PropertyFormula::Always(expr) => {
            let pred = lower_expr_to_predicate(expr)?;
            Ok(TemporalProperty::Always(pred))
        }
        PropertyFormula::Never(expr) => {
            let signal = extract_signal_name(expr)?;
            let pred = SignalPredicate::LessThan(signal, 1);
            Ok(TemporalProperty::Always(pred))
        }
        PropertyFormula::EventuallyWithin { expr, cycles } => {
            let pred = lower_expr_to_predicate(expr)?;
            Ok(TemporalProperty::EventuallyWithin(pred, u64::from(*cycles)))
        }
        PropertyFormula::AlwaysImplies { antecedent, consequent } => {
            let a = lower_expr_to_predicate(antecedent)?;
            let b = lower_expr_to_predicate(consequent)?;
            Ok(TemporalProperty::AlwaysImplies(a, b))
        }
        PropertyFormula::NeverImplies { antecedent, consequent } => {
            let a = lower_expr_to_predicate(antecedent)?;
            let b = lower_expr_to_predicate(consequent)?;
            Ok(TemporalProperty::NeverImplies(a, b))
        }
        PropertyFormula::AlwaysFollowedBy { trigger, delay_cycles, response } => {
            let t = lower_expr_to_predicate(trigger)?;
            let r = lower_expr_to_predicate(response)?;
            Ok(TemporalProperty::AlwaysFollowedBy(t, u64::from(*delay_cycles), r))
        }
    }
}

/// Lower a simple expression to a `SignalPredicate`.
fn lower_expr_to_predicate(expr: &Expr) -> Result<SignalPredicate, String> {
    match expr {
        Expr::Signal(name) => Ok(SignalPredicate::IsTrue(name.clone())),
        Expr::Binary { op, left, right } => lower_binary_predicate(op, left, right),
        Expr::Unary { .. } => {
            let name = extract_signal_name(expr)?;
            Ok(SignalPredicate::IsTrue(name))
        }
        Expr::Literal(_) => Err("bare literal cannot be a signal predicate".to_string()),
        Expr::Prev { signal, .. } => Ok(SignalPredicate::IsTrue(signal.clone())),
        Expr::UnfoldIndex(_) => Err("E506: UnfoldIndex reached analysis stage unresolved".to_string()),
        Expr::ArrayIndex { .. }
        | Expr::FieldAccess { .. }
        | Expr::ArrayLiteral(_)
        | Expr::StructLiteral { .. } => {
            let name = extract_signal_name(expr).unwrap_or_else(|_| "__composite__".to_string());
            Ok(SignalPredicate::IsTrue(name))
        }
    }
}

/// Lower a binary expression to a `SignalPredicate`.
fn lower_binary_predicate(
    op: &crate::ast::types::BinaryOp,
    left: &Expr,
    right: &Expr,
) -> Result<SignalPredicate, String> {
    use crate::ast::types::BinaryOp;

    if let (Expr::Signal(name), Some(threshold)) = (left, literal_u64(right)) {
        return match op {
            BinaryOp::Lt => Ok(SignalPredicate::LessThan(name.clone(), threshold)),
            BinaryOp::Le => {
                Ok(SignalPredicate::LessThan(name.clone(), threshold.saturating_add(1)))
            }
            BinaryOp::Gt => Ok(SignalPredicate::GreaterThan(name.clone(), threshold)),
            BinaryOp::Ge => {
                Ok(SignalPredicate::GreaterThan(name.clone(), threshold.saturating_sub(1)))
            }
            _ => Ok(SignalPredicate::IsTrue(name.clone())),
        };
    }

    let name = extract_signal_name(left).or_else(|_| extract_signal_name(right))?;
    Ok(SignalPredicate::IsTrue(name))
}

/// Extract a `u64` from a literal expression, if it is one.
fn literal_u64(expr: &Expr) -> Option<u64> {
    match expr {
        Expr::Literal(crate::ast::types::LiteralValue::Integer(n)) => Some(*n),
        Expr::Literal(crate::ast::types::LiteralValue::Bool(b)) => Some(u64::from(*b)),
        _ => None,
    }
}

/// Walk an expression tree (bounded, iterative) to find the first `Signal` name.
pub(super) fn extract_signal_name(expr: &Expr) -> Result<String, String> {
    let mut stack: Vec<&Expr> = Vec::with_capacity(MAX_EXPR_VISIT);
    stack.push(expr);

    let mut visited: usize = 0;
    while let Some(current) = stack.pop() {
        visited = visited.saturating_add(1);
        if visited > MAX_EXPR_VISIT {
            break;
        }

        match current {
            Expr::Signal(name) => return Ok(name.clone()),
            Expr::Prev { signal, .. } => return Ok(signal.clone()),
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(right);
                stack.push(left);
            }
            Expr::Literal(_) => {}
            Expr::ArrayIndex { array, .. } => stack.push(array),
            Expr::FieldAccess { object, .. } => stack.push(object),
            Expr::ArrayLiteral(elems) => {
                if let Some(first) = elems.first() {
                    stack.push(first);
                }
            }
            Expr::StructLiteral { fields, .. } => {
                if let Some((_, first_val)) = fields.first() {
                    stack.push(first_val);
                }
            }
            Expr::UnfoldIndex(_) => {
                // Meta-stage artifact; not a signal reference.
            }
        }
    }

    Err("no signal reference found in expression".to_string())
}
