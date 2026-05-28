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

    // Collect names and check for duplicates (Signals, Guards, Reflexes, Properties).
    // This fulfills the S03 Symbol Shadowing / Collision check.
    let mut seen_names: HashMap<&str, (Option<Span>, &str)> = HashMap::with_capacity(
        signal_capacity + guard_capacity + reflex_capacity + module.properties.len(),
    );

    for sig in &module.signals {
        if let Some((first_span, kind)) = seen_names.get(sig.name.as_str()) {
            let code = crate::error_codes::ec(201); // Both E201 for signal collisions
            let mut msg = if *kind == "signal" {
                format!("{} Duplicate signal name: '{}'.", code, sig.name)
            } else {
                format!(
                    "{} Name collision: '{}' is defined as both a signal and a {}.",
                    code, sig.name, kind
                )
            };
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: sig.span });
        } else {
            seen_names.insert(&sig.name, (sig.span, "signal"));
        }
    }

    for guard in &module.guards {
        if let Some((first_span, kind)) = seen_names.get(guard.name.as_str()) {
            let code = if *kind == "guard" {
                crate::error_codes::ec(213)
            } else {
                crate::error_codes::ec(201)
            };
            let mut msg = if *kind == "guard" {
                format!("{} Duplicate guard name: '{}'.", code, guard.name)
            } else {
                format!(
                    "{} Name collision: '{}' is defined as both a guard and a {}.",
                    code, guard.name, kind
                )
            };
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: guard.span });
        } else {
            seen_names.insert(&guard.name, (guard.span, "guard"));
        }
    }

    for reflex in &module.reflexes {
        if let Some((first_span, kind)) = seen_names.get(reflex.name.as_str()) {
            let code = if *kind == "reflex" {
                crate::error_codes::ec(212)
            } else {
                crate::error_codes::ec(201)
            };
            let mut msg = if *kind == "reflex" {
                format!("{} Duplicate reflex name: '{}'.", code, reflex.name)
            } else {
                format!(
                    "{} Name collision: '{}' is defined as both a reflex and a {}.",
                    code, reflex.name, kind
                )
            };
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: reflex.span });
        } else {
            seen_names.insert(&reflex.name, (reflex.span, "reflex"));
        }
    }

    for prop in &module.properties {
        if let Some((first_span, kind)) = seen_names.get(prop.name.as_str()) {
            let code = if *kind == "property" {
                crate::error_codes::ec(214)
            } else {
                crate::error_codes::ec(201)
            };
            let mut msg = if *kind == "property" {
                format!("{} Duplicate property name: '{}'.", code, prop.name)
            } else {
                format!(
                    "{} Name collision: '{}' is defined as both a property and a {}.",
                    code, prop.name, kind
                )
            };
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: prop.span });
        } else {
            seen_names.insert(&prop.name, (prop.span, "property"));
        }
    }

    // Re-collect separate sets for reference validation logic below.
    let signal_names: HashSet<&str> = module.signals.iter().map(|s| s.name.as_str()).collect();
    let guard_names: HashSet<&str> = module.guards.iter().map(|g| g.name.as_str()).collect();

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
            if !signal_names.contains(sig_ref.as_str()) && !guard_names.contains(sig_ref.as_str()) {
                if !reported_undeclared.insert(sig_ref.clone()) {
                    continue;
                }
                let suggestion = crate::suggest::closest_match(sig_ref, &signal_name_candidates);
                let mut msg = format!(
                    "{} Guard '{}' references undeclared signal '{}'.",
                    crate::error_codes::ec(204),
                    guard.name,
                    sig_ref
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
            let is_bool_signal = module
                .signals
                .iter()
                .any(|s| s.name == *gname && s.ty.core == crate::ast::types::SignalType::Bool);
            if gname != "always" && !guard_names.contains(gname.as_str()) && !is_bool_signal {
                let suggestion = crate::suggest::closest_match(gname, &guard_name_candidates);
                let mut msg = format!(
                    "{} Reflex '{}' references undeclared guard '{}'.",
                    crate::error_codes::ec(205),
                    reflex.name,
                    gname
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
            let has_bracket = assignment.target.contains('[');
            let clean_target = if let Some(pos) = assignment.target.find('[') {
                assignment.target[..pos].trim()
            } else {
                assignment.target.as_str()
            };

            if has_bracket {
                if let Some(sig) = module.signals.iter().find(|s| s.name == clean_target) {
                    if !matches!(sig.ty.core, crate::ast::types::SignalType::Array { .. }) {
                        errors.push(MirrError::SemanticError {
                            message: format!(
                                "{} Reflex '{}' assigns to non-array signal '{}' with indexing.",
                                crate::error_codes::ec(207),
                                reflex.name,
                                assignment.target
                            ),
                            span: reflex.span,
                        });
                        continue;
                    }
                }
            }

            if !writable_signals.contains(clean_target) {
                if signal_names.contains(clean_target) {
                    errors.push(MirrError::SemanticError {
                        message: format!(
                            "{} Reflex '{}' assigns to input signal '{}', which is not writable.",
                            crate::error_codes::ec(206),
                            reflex.name,
                            assignment.target
                        ),
                        span: reflex.span,
                    });
                    continue;
                }
                let suggestion =
                    crate::suggest::closest_match(clean_target, &signal_name_candidates);
                let mut msg = format!(
                    "{} Reflex '{}' assigns to undeclared signal '{}'.",
                    crate::error_codes::ec(207),
                    reflex.name,
                    assignment.target
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
                        "{} Reflex '{}' assignment references undeclared signal '{}'.",
                        crate::error_codes::ec(208),
                        reflex.name,
                        sig_ref
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
    validate_properties(
        &module.properties,
        &signal_names,
        &guard_names,
        &signal_name_candidates,
        &mut errors,
    );

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
    for root in &exprs {
        if !expr_contains_composite(root) {
            continue;
        }

        let mut iterations: usize = 0;
        let mut stack: Vec<&Expr> = Vec::with_capacity(32);
        stack.push(root);
        while let Some(node) = stack.pop() {
            iterations = iterations.saturating_add(1);
            if iterations > MAX_EXPR_NODES {
                errors.push(MirrError::SemanticError {
                    message: format!("{} Composite semantic validation traversal budget exhausted (limit: {MAX_EXPR_NODES} nodes).", crate::error_codes::ec(231)),
                    span: None,
                });
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
                                        "{} No field '{}' on struct signal '{}'.",
                                        crate::error_codes::ec(229),
                                        field,
                                        name
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
                            if !matches!(
                                ty,
                                SignalType::Array { .. }
                                    | SignalType::Unsigned(_)
                                    | SignalType::Signed(_)
                            ) {
                                errors.push(MirrError::SemanticError {
                                    message: format!(
                                        "{} Signal '{}' is not an indexable type (array, unsigned, or signed) but is indexed.",
                                        crate::error_codes::ec(230),
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
                    for e in elems {
                        stack.push(e);
                    }
                }
                Expr::StructLiteral { fields, .. } => {
                    for (_, v) in fields {
                        stack.push(v);
                    }
                }
                Expr::UnfoldIndex(_) => {
                    // Unresolved meta-stage index is not validated here.
                }
                Expr::Signal(_) | Expr::Prev { .. } | Expr::Literal(_) => {}
            }
        }
    }
}

fn expr_contains_composite(expr: &Expr) -> bool {
    const DETECTION_LIMIT: usize = MAX_EXPR_NODES.saturating_mul(8);

    let mut iterations = 0usize;
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);

    while let Some(node) = stack.pop() {
        iterations = iterations.saturating_add(1);
        if iterations > DETECTION_LIMIT {
            break;
        }
        match node {
            Expr::FieldAccess { .. } | Expr::ArrayIndex { .. } => return true,
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
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
            Expr::Signal(_) | Expr::Prev { .. } | Expr::Literal(_) | Expr::UnfoldIndex(_) => {}
        }
    }

    false
}

fn get_reflex_base_name(name: &str) -> &str {
    if let Some(idx) = name.find("_split_") {
        &name[..idx]
    } else {
        name
    }
}

/// Validate single-writer: each writable signal has at most one reflex writer.
/// Exception: "clear" or "tick" reflexes may write to signals also written by other reflexes.
/// This allows the common "clear before set" pattern.
/// Bounded: single pass over all reflexes and their assignments.
fn validate_signal_ownership(module: &Module) -> Vec<MirrError> {
    let mut ownership_errors: Vec<MirrError> = Vec::new();

    // Build reflex origin lookup and guard sets once.
    // reflex_name -> guard_names
    let mut reflex_guards: HashMap<&str, Vec<&str>> = HashMap::new();
    for reflex in &module.reflexes {
        reflex_guards
            .insert(reflex.name.as_str(), reflex.guard_names.iter().map(|g| g.as_str()).collect());
    }

    // Build reflex origin lookup: reflex_name -> origin
    let mut reflex_origins: HashMap<&str, &str> = HashMap::new();
    for reflex in &module.reflexes {
        if let Some(ref origin) = reflex.origin {
            reflex_origins.insert(reflex.name.as_str(), origin.as_str());
        }
    }

    // Track writers per signal by guard so we can detect conflicts in linear time.
    let mut signal_guard_writers: HashMap<&str, HashMap<&str, (&str, Option<&str>)>> =
        HashMap::new();
    let mut emitted_pairs: HashMap<&str, HashSet<(&str, &str)>> = HashMap::new();

    for reflex in &module.reflexes {
        let guards = reflex_guards.get(reflex.name.as_str()).cloned().unwrap_or_default();
        let current_origin = reflex_origins.get(reflex.name.as_str()).copied();

        for assignment in &reflex.assignments {
            let target = assignment.target.as_str();
            let writers = signal_guard_writers.entry(target).or_default();
            let emitted = emitted_pairs.entry(target).or_default();

            for guard in &guards {
                if let Some((existing_reflex, existing_origin)) = writers.get(guard).copied() {
                    let existing_base = get_reflex_base_name(existing_reflex);
                    let current_base = get_reflex_base_name(reflex.name.as_str());
                    if existing_base == current_base {
                        continue;
                    }

                    let pair = if existing_reflex <= reflex.name.as_str() {
                        (existing_reflex, reflex.name.as_str())
                    } else {
                        (reflex.name.as_str(), existing_reflex)
                    };
                    if !emitted.insert(pair) {
                        continue;
                    }

                    let msg = match (existing_origin, current_origin) {
                        (Some(p1), Some(p2)) => format!(
                            "{} Signal '{}' has multiple writers: \
                             reflex '{}' (from pattern '{}') and reflex '{}' (from pattern '{}').",
                            crate::error_codes::ec(216),
                            target,
                            existing_reflex,
                            p1,
                            reflex.name,
                            p2
                        ),
                        _ => format!(
                            "{} Signal '{}' has multiple writers: \
                             reflex '{}' and reflex '{}'.",
                            crate::error_codes::ec(216),
                            target,
                            existing_reflex,
                            reflex.name
                        ),
                    };
                    ownership_errors.push(MirrError::SemanticError { message: msg, span: None });
                } else {
                    writers.insert(guard, (reflex.name.as_str(), current_origin));
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
                            "{} '{}' contains prev('{}') with delay 0; delay must be >= 1.",
                            crate::error_codes::ec(209),
                            context_name,
                            signal
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
            Expr::UnfoldIndex(_) => {
                // Unresolved meta-stage index needs no Prev delay validation.
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
            Expr::UnfoldIndex(_) => {
                // Unresolved meta-stage index does not contribute signal refs.
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
    guard_names: &HashSet<&str>,
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
            let mut msg = format!(
                "{} Duplicate property name: '{}'.",
                crate::error_codes::ec(210),
                prop.name
            );
            if let Some(fs) = first_span {
                msg.push_str(&format!(" First defined at line {}.", fs.start_line + 1));
            }
            errors.push(MirrError::SemanticError { message: msg, span: prop.span });
            continue;
        } else {
            property_first_span.insert(&prop.name, prop.span);
        }
        validate_property_signals(prop, signal_names, guard_names, signal_name_candidates, errors);
        validate_property_prev_delays(prop, errors);
    }
}

/// Check that all signal references in a property formula are declared.
/// Errors are accumulated into the caller's `PipelineErrors`.
fn validate_property_signals(
    prop: &crate::ast::property::PropertyDecl,
    signal_names: &HashSet<&str>,
    guard_names: &HashSet<&str>,
    signal_name_candidates: &[&str],
    errors: &mut PipelineErrors,
) {
    for expr in prop.formula.exprs() {
        let refs = collect_signal_refs(expr);
        for sig_ref in &refs {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            if !signal_names.contains(sig_ref.as_str()) && !guard_names.contains(sig_ref.as_str()) {
                let suggestion = crate::suggest::closest_match(sig_ref, signal_name_candidates);
                let mut msg = format!(
                    "{} Property '{}' references undeclared signal '{}'.",
                    crate::error_codes::ec(211),
                    prop.name,
                    sig_ref
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
                message: format!(
                    "{} Duplicate pattern definition: '{}'.",
                    crate::error_codes::ec(417),
                    pat.name
                ),
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
                        "{} Pattern '{}' has duplicate parameter name: '{}'.",
                        crate::error_codes::ec(418),
                        pat.name,
                        p.name
                    ),
                    span: pat.span,
                });
            }
        }

        if pat.params.len() > MAX_PARAMS {
            errors.push(MirrError::PatternError {
                message: format!(
                    "{} Pattern '{}' has {} parameters (max {MAX_PARAMS}).",
                    crate::error_codes::ec(419),
                    pat.name,
                    pat.params.len()
                ),
                span: pat.span,
            });
        }

        if pat.body.raw_lines.is_empty() {
            errors.push(MirrError::PatternError {
                message: format!(
                    "{} Pattern '{}' has empty reflect body.",
                    crate::error_codes::ec(420),
                    pat.name
                ),
                span: pat.span,
            });
        }

        if pat.body.raw_lines.len() > MAX_REFLECT_LINES {
            errors.push(MirrError::PatternError {
                message: format!(
                    "{} Pattern '{}' reflect body has {} lines (max {MAX_REFLECT_LINES}).",
                    crate::error_codes::ec(421),
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
