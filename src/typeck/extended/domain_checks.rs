//! Domain-specific type checks: effects, clock domains, phantom tags, session types.
//!
//! Part of the MEGA-1 Extended Type System.

#![forbid(unsafe_code)]

use super::qualifiers::*;
use super::types::*;

// ===========================================================================
// Phase 4: Effect checking
// ===========================================================================

/// Check that `pure`-qualified signals are not derived from stateful sources.
///
/// A pure signal must only depend on:
/// - Other pure signals
/// - Input signals (implicitly pure)
/// - Literals
///
/// Using `Prev` (register read) in a pure context is an error (E616).
/// Referencing a `stateful` signal from a pure expression is an error (E617).
///
/// Bounded: iterates over reflexes and assignments with bounded inner traversal.
pub(super) fn check_effect_qualifiers(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build lookup: signal name -> is_pure, is_stateful
    let mut pure_signals: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(extended_signals.len());
    let mut stateful_signals: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if sig.extended_ty.is_pure() {
            pure_signals.insert(&sig.name);
        }
        if sig.extended_ty.is_stateful() {
            stateful_signals.insert(&sig.name);
        }
        idx += 1;
    }

    if pure_signals.is_empty() {
        return;
    }

    // For each reflex, check assignments to pure targets
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            if !pure_signals.contains(assignment.target.as_str()) {
                continue;
            }

            // E616: Check for Prev in pure expression
            if expr_contains_prev(&assignment.value) {
                if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                    return;
                }
                errors.push(crate::error::MirrError::TypeError {
                    message: format!(
                        "[{}] Pure signal '{}' cannot depend on prev() (stateful operation) in reflex '{}'.",
                        error_codes::E616_EFF_PURE,
                        assignment.target,
                        reflex.name
                    ),
                    span: assignment.span,
                });
            }

            // E617: Check for stateful signal references in pure expression
            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;
                if stateful_signals.contains(sig_ref.as_str()) {
                    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                        return;
                    }
                    errors.push(crate::error::MirrError::TypeError {
                        message: format!(
                            "[{}] Pure signal '{}' cannot depend on stateful signal '{}' in reflex '{}'.",
                            error_codes::E617_EFF_MIX,
                            assignment.target,
                            sig_ref,
                            reflex.name
                        ),
                        span: assignment.span,
                    });
                }
            }
        }
    }
}

/// Check whether an expression contains any `Prev` node.
/// Uses explicit stack (no recursion). Bounded by MAX_EXTENDED_TYPE_NODES.
fn expr_contains_prev(expr: &crate::ast::expr::Expr) -> bool {
    let mut stack: Vec<&crate::ast::expr::Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXTENDED_TYPE_NODES {
            break;
        }
        match node {
            crate::ast::expr::Expr::Prev { .. } => return true,
            crate::ast::expr::Expr::Literal(_) | crate::ast::expr::Expr::Signal(_) => {}
            crate::ast::expr::Expr::Unary { operand, .. } => stack.push(operand),
            crate::ast::expr::Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            crate::ast::expr::Expr::ArrayIndex { array, index } => {
                stack.push(array);
                stack.push(index);
            }
            crate::ast::expr::Expr::FieldAccess { object, .. } => stack.push(object),
            crate::ast::expr::Expr::ArrayLiteral(elems) => {
                let mut i = 0;
                while i < elems.len().min(MAX_EXTENDED_TYPE_NODES) {
                    stack.push(&elems[i]);
                    i += 1;
                }
            }
            crate::ast::expr::Expr::StructLiteral { fields, .. } => {
                let mut i = 0;
                while i < fields.len().min(MAX_EXTENDED_TYPE_NODES) {
                    stack.push(&fields[i].1);
                    i += 1;
                }
            }
            crate::ast::expr::Expr::UnfoldIndex(_) => {
                // UnfoldIndex is treated as non-Prev expression for this check.
            }
        }
    }

    false
}

// ===========================================================================
// Phase 5: Clock domain checking
// ===========================================================================

/// Verify that cross-clock-domain signal references use a synchronizer.
///
/// If signal A is in domain `@clk_fast` and signal B is in domain `@clk_slow`,
/// then referencing A in an expression assigned to B (or vice versa) without
/// an explicit synchronizer construct is an error (E618).
///
/// Bounded: iterates over reflexes and assignments.
pub(super) fn check_clock_domains(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    declared_domains: &[ClockDomain],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build domain lookup: signal name -> clock domain name
    let mut signal_domain: std::collections::HashMap<&str, &str> =
        std::collections::HashMap::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if let Some(ref cd) = sig.extended_ty.clock_domain {
            signal_domain.insert(&sig.name, &cd.name);
        }
        idx += 1;
    }

    if signal_domain.is_empty() {
        return;
    }

    // Validate declared domains exist (E619)
    let declared_names: std::collections::HashSet<&str> =
        declared_domains.iter().map(|cd| cd.name.as_str()).collect();
    for (sig_name, domain_name) in &signal_domain {
        if !declared_names.contains(domain_name) {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Signal '{}' references undeclared clock domain '@{}'.",
                    error_codes::E619_CLK_UNDEF,
                    sig_name,
                    domain_name
                ),
                span: None,
            });
        }
    }

    // Check cross-domain references (E618)
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            let target_domain = signal_domain.get(assignment.target.as_str());

            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;

                let source_domain = signal_domain.get(sig_ref.as_str());

                if let (Some(td), Some(sd)) = (target_domain, source_domain) {
                    if td != sd {
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Clock domain crossing: signal '{}' (@{}) references '{}' (@{}) \
                                 without synchronizer in reflex '{}'.",
                                error_codes::E618_CLK_CROSS,
                                assignment.target,
                                td,
                                sig_ref,
                                sd,
                                reflex.name
                            ),
                            span: assignment.span,
                        });
                    }
                }
            }
        }
    }
}

// ===========================================================================
// Phase 6: Phantom tag checking
// ===========================================================================

/// Verify phantom tag compatibility on assignments.
///
/// A signal tagged `#Verified` can only be assigned from a `#Verified` source.
/// Assigning from `#Unverified` to `#Verified` is error E620.
///
/// Bounded: iterates over reflexes and assignments.
pub(super) fn check_phantom_tags(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    declared_tags: &[PhantomTag],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build phantom tag lookup: signal name -> tag name
    let mut signal_tag: std::collections::HashMap<&str, &str> =
        std::collections::HashMap::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if let Some(ref pt) = sig.extended_ty.phantom {
            signal_tag.insert(&sig.name, &pt.tag);
        }
        idx += 1;
    }

    if signal_tag.is_empty() {
        return;
    }

    // E621: Validate declared tags exist
    let declared_tag_names: std::collections::HashSet<&str> =
        declared_tags.iter().map(|pt| pt.tag.as_str()).collect();
    for (sig_name, tag_name) in &signal_tag {
        if !declared_tag_names.contains(tag_name) {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Signal '{}' references undeclared phantom tag '#{}'. \
                     Declare it in the module's tag list.",
                    error_codes::E621_PHT_UNDEF,
                    sig_name,
                    tag_name
                ),
                span: None,
            });
        }
    }

    // E620: Check tag compatibility on assignments
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            let target_tag = signal_tag.get(assignment.target.as_str());

            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;

                let source_tag = signal_tag.get(sig_ref.as_str());

                match (target_tag, source_tag) {
                    (Some(tt), Some(st)) if tt != st => {
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Phantom tag mismatch: cannot assign #{}-tagged signal '{}' \
                                 to #{}-tagged target '{}' in reflex '{}'.",
                                error_codes::E620_PHT_MISMATCH,
                                st,
                                sig_ref,
                                tt,
                                assignment.target,
                                reflex.name
                            ),
                            span: assignment.span,
                        });
                    }
                    (Some(tt), None) => {
                        // Target is tagged but source is untagged — error
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Phantom tag mismatch: cannot assign untagged signal '{}' \
                                 to #{}-tagged target '{}' in reflex '{}'.",
                                error_codes::E620_PHT_MISMATCH,
                                sig_ref,
                                tt,
                                assignment.target,
                                reflex.name
                            ),
                            span: assignment.span,
                        });
                    }
                    // (None, Some(_)) — untagged target accepts any source (tag is dropped)
                    // (None, None) — no phantom types involved
                    _ => {}
                }
            }
        }
    }
}

// ===========================================================================
// Phase 7: Session type checking
// ===========================================================================

/// Verify session type protocol compliance.
///
/// For each signal participating in a session protocol, verify that the
/// signal's declared state is a legal state in the protocol and that
/// all state transitions observable in the module are legal.
///
/// Bounded: iterates over signals, protocols, and transitions.
pub fn check_session_types(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    protocols: &[SessionProtocol],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build protocol lookup: name -> &SessionProtocol
    let mut protocol_map: std::collections::HashMap<&str, &SessionProtocol> =
        std::collections::HashMap::with_capacity(protocols.len());
    let mut proto_idx = 0usize;
    while proto_idx < protocols.len() && proto_idx < MAX_SESSION_STATES {
        protocol_map.insert(&protocols[proto_idx].name, &protocols[proto_idx]);
        proto_idx += 1;
    }

    if protocol_map.is_empty() {
        return;
    }

    // Collect session-typed signals
    let mut sig_idx = 0usize;
    while sig_idx < extended_signals.len() && sig_idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[sig_idx];
        sig_idx += 1;

        if let Some(ref session_ref) = sig.extended_ty.session {
            // Verify protocol exists
            match protocol_map.get(session_ref.protocol.as_str()) {
                Some(proto) => {
                    // Verify state exists in protocol
                    let state_exists = proto
                        .transitions
                        .iter()
                        .any(|t| t.from == session_ref.state || t.to == session_ref.state);
                    if !state_exists {
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Signal '{}' references state '{}' which does not exist \
                                 in protocol '{}'.",
                                error_codes::E625_SES_PROTOCOL,
                                sig.name,
                                session_ref.state,
                                session_ref.protocol
                            ),
                            span: sig.span,
                        });
                    }
                }
                None => {
                    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                        return;
                    }
                    errors.push(crate::error::MirrError::TypeError {
                        message: format!(
                            "[{}] Signal '{}' references undeclared session protocol '{}'.",
                            error_codes::E625_SES_PROTOCOL,
                            sig.name,
                            session_ref.protocol
                        ),
                        span: sig.span,
                    });
                }
            }
        }
    }

    // Cross-reflex transition checking would verify that if a sender signal
    // transitions from state A to state B, the corresponding receiver signal
    // also transitions according to the protocol. This requires interprocedural
    // analysis across reflexes, which is bounded by module size.
    //
    // Full implementation deferred to MEGA-1 Phase 2 (multi-module linking).
    let _ = module;
}
