//! Semantic validation for MIRR modules and pattern definitions.
//!
//! Checks for duplicate names, undeclared signal references, guard-reflex
//! consistency, property formula validity, and pattern definition constraints.
//!
//! All validation functions accumulate errors into `PipelineErrors` instead of
//! returning on first failure (ERR-002).

#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::expr::Expr;
use crate::ast::pattern::PatternDef;
use crate::ast::program::Module;

use crate::ast::types::SignalKind;
use crate::ast::MAX_EXPR_NODES;
use crate::error::MirrError;
use crate::error::PipelineErrors;
use crate::parser::pattern_parser::{MAX_PARAMS, MAX_REFLECT_LINES};
use crate::span::Span;

/// Validate a parsed module for semantic correctness: duplicate names,
/// undeclared signal/guard references, assignment targets, and composite types.
/// Errors are accumulated up to [`crate::error::MAX_ACCUMULATED_ERRORS`].
pub fn validate_module(module: &Module) -> Result<(), PipelineErrors> {
    let mut errors = PipelineErrors::new();
    let mut reported_undeclared: HashSet<String> = HashSet::with_capacity(16);
    let signal_capacity = module.signals.len();
    let guard_capacity = module.guards.len();
    let reflex_capacity = module.reflexes.len();

    // Collect signal names and check for duplicates.
    let mut signal_names: HashSet<&str> = HashSet::with_capacity(signal_capacity);
    let mut signal_first_span: HashMap<&str, Option<Span>> =
        HashMap::with_capacity(signal_capacity);
    for sig in &module.signals {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        if let Some(first_span) = signal_first_span.get(sig.name.as_str()) {
            let mut msg = format!("[E201] Duplicate signal name: '{}'.", sig.name);
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: sig.span });
            continue;
        } else {
            signal_names.insert(&sig.name);
            signal_first_span.insert(&sig.name, sig.span);
        }
    }

    // Collect guard names and check for duplicates.
    let mut guard_names: HashSet<&str> = HashSet::with_capacity(guard_capacity);
    let mut guard_first_span: HashMap<&str, Option<Span>> = HashMap::with_capacity(guard_capacity);
    for guard in &module.guards {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        if let Some(first_span) = guard_first_span.get(guard.name.as_str()) {
            let mut msg = format!("[E202] Duplicate guard name: '{}'.", guard.name);
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: guard.span });
            continue;
        } else {
            guard_names.insert(&guard.name);
            guard_first_span.insert(&guard.name, guard.span);
        }
    }

    // Collect reflex names and check for duplicates.
    let mut reflex_names: HashSet<&str> = HashSet::with_capacity(reflex_capacity);
    let mut reflex_first_span: HashMap<&str, Option<Span>> =
        HashMap::with_capacity(reflex_capacity);
    for reflex in &module.reflexes {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        if let Some(first_span) = reflex_first_span.get(reflex.name.as_str()) {
            let mut msg = format!("[E203] Duplicate reflex name: '{}'.", reflex.name);
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: reflex.span });
            continue;
        } else {
            reflex_names.insert(&reflex.name);
            reflex_first_span.insert(&reflex.name, reflex.span);
        }
    }

    // Build set of output/internal signals (valid assignment targets).
    let writable_capacity = module
        .signals
        .iter()
        .filter(|s| s.kind == SignalKind::Output || s.kind == SignalKind::Internal)
        .count();
    let writable_signals: HashSet<&str> = {
        let mut set = HashSet::with_capacity(writable_capacity);
        for s in &module.signals {
            if s.kind == SignalKind::Output || s.kind == SignalKind::Internal {
                set.insert(s.name.as_str());
            }
        }
        set
    };

    // Build candidate vectors ONCE for "did you mean?" suggestions.
    let signal_name_candidates: Vec<&str> =
        module.signals.iter().map(|s| s.name.as_str()).collect();
    let guard_name_candidates: Vec<&str> = module.guards.iter().map(|g| g.name.as_str()).collect();

    // Validate guard conditions reference declared signals.
    for guard in &module.guards {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        if let Err(e) = validate_prev_delays(&guard.condition, &guard.name, guard.span) {
            errors.push(e);
            continue;
        }
        let refs = collect_signal_refs(&guard.condition);
        for sig_ref in &refs {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                break;
            }
            if !signal_names.contains(sig_ref.as_str()) {
                if !reported_undeclared.insert(sig_ref.clone()) {
                    continue;
                }
                let suggestion = crate::suggest::closest_match(sig_ref, &signal_name_candidates);
                let mut msg = format!(
                    "[E204] Guard '{}' references undeclared signal '{}'.",
                    guard.name, sig_ref
                );
                if let Some(s) = suggestion {
                    msg.push_str(&format!(" Did you mean '{}'?", s));
                }
                errors.push(MirrError::SemanticError { message: msg, span: guard.span });
                continue;
            }
        }
    }

    // Validate reflexes.
    for reflex in &module.reflexes {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        // Check guard references.
        for gname in &reflex.guard_names {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                break;
            }
            if !guard_names.contains(gname.as_str()) {
                let suggestion = crate::suggest::closest_match(gname, &guard_name_candidates);
                let mut msg = format!(
                    "[E205] Reflex '{}' references undeclared guard '{}'.",
                    reflex.name, gname
                );
                if let Some(s) = suggestion {
                    msg.push_str(&format!(" Did you mean '{}'?", s));
                }
                errors.push(MirrError::SemanticError { message: msg, span: reflex.span });
                continue;
            }
        }

        // Check assignments.
        for assignment in &reflex.assignments {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                break;
            }
            // Target must be a writable signal.
            if !writable_signals.contains(assignment.target.as_str()) {
                if signal_names.contains(assignment.target.as_str()) {
                    errors.push(MirrError::SemanticError {
                        message: format!(
                            "[E206] Reflex '{}' assigns to input signal '{}', which is not writable.",
                            reflex.name, assignment.target
                        ),
                        span: reflex.span,
                    });
                    continue;
                }
                let suggestion =
                    crate::suggest::closest_match(&assignment.target, &signal_name_candidates);
                let mut msg = format!(
                    "[E207] Reflex '{}' assigns to undeclared signal '{}'.",
                    reflex.name, assignment.target
                );
                if let Some(s) = suggestion {
                    msg.push_str(&format!(" Did you mean '{}'?", s));
                }
                errors.push(MirrError::SemanticError { message: msg, span: reflex.span });
                continue;
            }

            // Validate Prev delays in RHS expressions.
            if let Err(e) = validate_prev_delays(&assignment.value, &reflex.name, reflex.span) {
                errors.push(e);
            }

            // RHS expression signals must be declared.
            let refs = collect_signal_refs(&assignment.value);
            for sig_ref in &refs {
                if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                    break;
                }
                if !signal_names.contains(sig_ref.as_str()) {
                    if !reported_undeclared.insert(sig_ref.clone()) {
                        continue;
                    }
                    let suggestion =
                        crate::suggest::closest_match(sig_ref, &signal_name_candidates);
                    let mut msg = format!(
                        "[E208] Reflex '{}' assignment references undeclared signal '{}'.",
                        reflex.name, sig_ref
                    );
                    if let Some(s) = suggestion {
                        msg.push_str(&format!(" Did you mean '{}'?", s));
                    }
                    errors.push(MirrError::SemanticError { message: msg, span: reflex.span });
                    continue;
                }
            }
        }
    }

    // Validate composite expression usage (FieldAccess/ArrayIndex) against signal types.
    validate_composite_exprs(module, &mut errors);

    // Validate single-writer ownership: each writable signal must have
    // at most one reflex that assigns to it.
    let ownership_errors = validate_signal_ownership(module);
    for e in ownership_errors {
        errors.push(e);
    }

    // Validate property declarations.
    validate_properties(&module.properties, &signal_names, &signal_name_candidates, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate composite expression usage: `FieldAccess` must target a struct
/// signal with the named field, and `ArrayIndex` must target an array signal.
/// Bounded: iterative expression walk capped by MAX_EXPR_NODES.
fn validate_composite_exprs(module: &Module, errors: &mut PipelineErrors) {
    use crate::ast::types::SignalType;
    let mut sig_types: HashMap<&str, &SignalType> = HashMap::with_capacity(module.signals.len());
    for sig in &module.signals {
        sig_types.insert(&sig.name, &sig.ty.core);
    }
    let mut exprs: Vec<&Expr> = Vec::with_capacity(64);
    for guard in &module.guards {
        exprs.push(&guard.condition);
    }
    for reflex in &module.reflexes {
        for assignment in &reflex.assignments {
            exprs.push(&assignment.value);
        }
    }
    let mut iterations: usize = 0;
    let max_total = MAX_EXPR_NODES.saturating_mul(exprs.len().min(256));
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    for root in &exprs {
        stack.clear();
        stack.push(root);
        while let Some(node) = stack.pop() {
            iterations = iterations.saturating_add(1);
            if iterations > max_total {
                return;
            }
            match node {
                Expr::FieldAccess { object, field } => {
                    if let Expr::Signal(name) = object.as_ref() {
                        if let Some(SignalType::Struct { fields, .. }) =
                            sig_types.get(name.as_str())
                        {
                            if !fields.iter().any(|(f, _)| f == field) {
                                errors.push(MirrError::SemanticError {
                                    message: format!(
                                        "[E229] No field '{}' on struct signal '{}'.",
                                        field, name
                                    ),
                                    span: None,
                                });
                            }
                        }
                    }
                    stack.push(object);
                }
                Expr::ArrayIndex { array, index } => {
                    if let Expr::Signal(name) = array.as_ref() {
                        if let Some(ty) = sig_types.get(name.as_str()) {
                            if !matches!(ty, SignalType::Array { .. }) {
                                errors.push(MirrError::SemanticError {
                                    message: format!(
                                        "[E230] Signal '{}' is not an array type but is indexed.",
                                        name
                                    ),
                                    span: None,
                                });
                            }
                        }
                    }
                    stack.push(array);
                    stack.push(index);
                }
                Expr::Unary { operand, .. } => stack.push(operand),
                Expr::Binary { left, right, .. } => {
                    stack.push(left);
                    stack.push(right);
                }
                Expr::ArrayLiteral(elems) => {
                    for e in elems.iter().take(MAX_EXPR_NODES) {
                        stack.push(e);
                    }
                }
                Expr::StructLiteral { fields, .. } => {
                    for (_, v) in fields.iter().take(MAX_EXPR_NODES) {
                        stack.push(v);
                    }
                }
                Expr::Signal(_) | Expr::Prev { .. } | Expr::Literal(_) => {}
            }
        }
    }
}

/// Validate single-writer: each writable signal has at most one reflex writer.
/// Bounded: single pass over all reflexes and their assignments.
fn validate_signal_ownership(module: &Module) -> Vec<MirrError> {
    let mut ownership_errors: Vec<MirrError> = Vec::new();
    // Map: signal_name -> (first_writer_reflex_name, first_writer_origin, first_writer_span)
    let mut writers: HashMap<&str, (&str, Option<&str>, Option<Span>)> = HashMap::new();

    for reflex in &module.reflexes {
        for assignment in &reflex.assignments {
            let target = assignment.target.as_str();
            let origin = reflex.origin.as_deref();
            match writers.get(target) {
                Some((first_reflex, first_origin, first_span)) => {
                    // Conflict: two different reflexes write to the same signal.
                    if *first_reflex != reflex.name {
                        let mut msg = match (first_origin, origin) {
                            (Some(p1), Some(p2)) => format!(
                                "[E216] Signal '{}' has multiple writers: \
                                 reflex '{}' (from pattern '{}') and reflex '{}' (from pattern '{}').",
                                target, first_reflex, p1, reflex.name, p2
                            ),
                            _ => format!(
                                "[E216] Signal '{}' has multiple writers: \
                                 reflex '{}' and reflex '{}'.",
                                target, first_reflex, reflex.name
                            ),
                        };
                        if let Some(fs) = first_span {
                            msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
                        }
                        ownership_errors
                            .push(MirrError::SemanticError { message: msg, span: reflex.span });
                    }
                    // Same reflex writing again -- allowed (intra-reflex sequential).
                }
                None => {
                    writers.insert(target, (&reflex.name, origin, reflex.span));
                }
            }
        }
    }

    ownership_errors
}

/// Validate that all `Prev` nodes in an expression have delay >= 1.
/// Uses an explicit stack to avoid recursion.
/// Bounded: at most MAX_EXPR_NODES iterations.
fn validate_prev_delays(
    expr: &Expr,
    context_name: &str,
    context_span: Option<crate::span::Span>,
) -> Result<(), MirrError> {
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        match node {
            Expr::Prev { signal, delay } => {
                if *delay == 0 {
                    return Err(MirrError::SemanticError {
                        message: format!(
                            "[E209] '{}' contains prev('{}') with delay 0; delay must be >= 1.",
                            context_name, signal
                        ),
                        span: context_span,
                    });
                }
            }
            Expr::Literal(_) | Expr::Signal(_) => {}
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            Expr::ArrayIndex { array, index } => {
                stack.push(array);
                stack.push(index);
            }
            Expr::FieldAccess { object, .. } => stack.push(object),
            Expr::ArrayLiteral(elems) => {
                for e in elems {
                    stack.push(e);
                }
            }
            Expr::StructLiteral { fields, .. } => {
                for (_, v) in fields {
                    stack.push(v);
                }
            }
        }
    }
    Ok(())
}

/// Collect all signal references from an expression tree (iterative, bounded).
pub fn collect_signal_refs(expr: &Expr) -> Vec<String> {
    let mut iterations = 0usize;
    let mut refs = Vec::with_capacity(16);
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);

    while let Some(node) = stack.pop() {
        iterations += 1;
        if iterations > MAX_EXPR_NODES {
            break;
        }
        match node {
            Expr::Signal(name) => refs.push(name.clone()),
            Expr::Prev { signal, .. } => refs.push(signal.clone()),
            Expr::Literal(_) => {}
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            Expr::ArrayIndex { array, index } => {
                stack.push(array);
                stack.push(index);
            }
            Expr::FieldAccess { object, .. } => stack.push(object),
            Expr::ArrayLiteral(elems) => {
                for e in elems {
                    stack.push(e);
                }
            }
            Expr::StructLiteral { fields, .. } => {
                for (_, v) in fields {
                    stack.push(v);
                }
            }
        }
    }

    refs
}

/// Validate property declarations: no duplicate names, all signal refs declared.
/// Errors are accumulated into the caller's `PipelineErrors`.
fn validate_properties(
    properties: &[crate::ast::property::PropertyDecl],
    signal_names: &HashSet<&str>,
    signal_name_candidates: &[&str],
    errors: &mut PipelineErrors,
) {
    let mut property_first_span: HashMap<&str, Option<Span>> =
        HashMap::with_capacity(properties.len());
    for prop in properties {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        if let Some(first_span) = property_first_span.get(prop.name.as_str()) {
            let mut msg = format!("[E210] Duplicate property name: '{}'.", prop.name);
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: prop.span });
            continue;
        } else {
            property_first_span.insert(&prop.name, prop.span);
        }
        validate_property_signals(prop, signal_names, signal_name_candidates, errors);
        validate_property_prev_delays(prop, errors);
    }
}

/// Check that all signal references in a property formula are declared.
/// Errors are accumulated into the caller's `PipelineErrors`.
fn validate_property_signals(
    prop: &crate::ast::property::PropertyDecl,
    signal_names: &HashSet<&str>,
    signal_name_candidates: &[&str],
    errors: &mut PipelineErrors,
) {
    for expr in prop.formula.exprs() {
        let refs = collect_signal_refs(expr);
        for sig_ref in &refs {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            if !signal_names.contains(sig_ref.as_str()) {
                let suggestion = crate::suggest::closest_match(sig_ref, signal_name_candidates);
                let mut msg = format!(
                    "[E211] Property '{}' references undeclared signal '{}'.",
                    prop.name, sig_ref
                );
                if let Some(s) = suggestion {
                    msg.push_str(&format!(" Did you mean '{}'?", s));
                }
                errors.push(MirrError::SemanticError { message: msg, span: prop.span });
            }
        }
    }
}

/// Validate that all Prev nodes in a property formula have delay >= 1.
/// Errors are accumulated into the caller's `PipelineErrors`.
fn validate_property_prev_delays(
    prop: &crate::ast::property::PropertyDecl,
    errors: &mut PipelineErrors,
) {
    for expr in prop.formula.exprs() {
        if let Err(e) = validate_prev_delays(expr, &prop.name, prop.span) {
            errors.push(e);
        }
    }
}

// ---------------------------------------------------------------------------
// Pattern definition validation (Phase 7b)
// ---------------------------------------------------------------------------

/// Validate pattern definitions for structural correctness.
///
/// Called BEFORE pattern expansion. Checks:
/// - No duplicate pattern names.
/// - No duplicate parameter names within a pattern.
/// - Reflect body is non-empty.
/// - Parameter count <= MAX_PARAMS.
/// - Body line count <= MAX_REFLECT_LINES.
///
/// Errors are accumulated (up to [`crate::error::MAX_ACCUMULATED_ERRORS`])
/// and returned together in a `PipelineErrors`.
///
/// Post-expansion, the standard `validate_module` catches all signal/guard/
/// reflex/property reference issues in the expanded result.
///
/// Bounded: iterates over patterns (max 64) and params (max 32).
pub fn validate_pattern_defs(patterns: &[PatternDef]) -> Result<(), PipelineErrors> {
    let mut errors = PipelineErrors::new();
    let mut names: HashSet<&str> = HashSet::with_capacity(patterns.len());
    for pat in patterns {
        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
            break;
        }
        if !names.insert(&pat.name) {
            errors.push(MirrError::PatternError {
                message: format!("[E417] Duplicate pattern definition: '{}'.", pat.name),
                span: pat.span,
            });
            continue;
        }

        // Check for duplicate parameter names within this pattern.
        let mut param_names: HashSet<&str> = HashSet::with_capacity(pat.params.len());
        for p in &pat.params {
            if !param_names.insert(&p.name) {
                errors.push(MirrError::PatternError {
                    message: format!(
                        "[E418] Pattern '{}' has duplicate parameter name: '{}'.",
                        pat.name, p.name
                    ),
                    span: pat.span,
                });
            }
        }

        if pat.params.len() > MAX_PARAMS {
            errors.push(MirrError::PatternError {
                message: format!(
                    "[E419] Pattern '{}' has {} parameters (max {MAX_PARAMS}).",
                    pat.name,
                    pat.params.len()
                ),
                span: pat.span,
            });
        }

        if pat.body.raw_lines.is_empty() {
            errors.push(MirrError::PatternError {
                message: format!("[E420] Pattern '{}' has empty reflect body.", pat.name),
                span: pat.span,
            });
        }

        if pat.body.raw_lines.len() > MAX_REFLECT_LINES {
            errors.push(MirrError::PatternError {
                message: format!(
                    "[E421] Pattern '{}' reflect body has {} lines (max {MAX_REFLECT_LINES}).",
                    pat.name,
                    pat.body.raw_lines.len()
                ),
                span: pat.span,
            });
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
